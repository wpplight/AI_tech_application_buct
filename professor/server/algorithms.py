"""
算法引擎管理模块
为三种推理算法提供统一的接口
"""

import sys
import os
import json
import sqlite3
from enum import Enum
from typing import List, Optional, Dict, Any, Callable

_server_dir = os.path.dirname(os.path.abspath(__file__))
_professor_dir = os.path.dirname(_server_dir)

sys.path.insert(0, _professor_dir)
sys.path.insert(0, os.path.join(_professor_dir, 'fullscan_py'))
sys.path.insert(0, os.path.join(_professor_dir, 'incremental_py'))
sys.path.insert(0, os.path.join(_professor_dir, 'rete_py'))

from fullscan_py.knowledge_base import KnowledgeBase as FullScanKB
from fullscan_py.fact_base import FactBase as FullScanFB
from fullscan_py.inference_engine import InferenceEngine as FullScanEngine

from incremental_py.knowledge_base import KnowledgeBase as IncrementalKB
from incremental_py.fact_base import FactBase as IncrementalFB
from incremental_py.inference_engine import InferenceEngine as IncrementalEngine

from rete_py.rete_network import ReteNetwork
from rete_py.rete_runner import Rule, ReteInferenceEngine

DB_PATH = os.path.join(_professor_dir, 'knowledge', 'rules.db')


class AlgorithmType(Enum):
    FULLSCAN = "fullscan"
    INCREMENTAL = "incremental"
    RETE = "rete"


class ConditionSetManager:
    """条件集管理器 — 所有算法引擎共享"""

    def __init__(self, db_path: str = DB_PATH):
        self.db_path = db_path
        self._ensure_table()
        self._cache_invalidators: List[Callable[[int], None]] = []

    def _ensure_table(self):
        conn = sqlite3.connect(self.db_path)
        conn.execute('''
            CREATE TABLE IF NOT EXISTS condition_sets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                facts TEXT NOT NULL DEFAULT '[]',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        conn.execute('CREATE INDEX IF NOT EXISTS idx_cs_name ON condition_sets(name)')
        conn.commit()
        conn.close()

    def _connect(self):
        conn = sqlite3.connect(self.db_path)
        conn.row_factory = sqlite3.Row
        return conn

    def register_invalidator(self, fn: Callable[[int], None]):
        """注册缓存失效回调，条件集变更时自动调用"""
        self._cache_invalidators.append(fn)

    def _notify_invalidate(self, cs_id: int):
        for fn in self._cache_invalidators:
            fn(cs_id)

    def create(self, name: str, facts: List[str]) -> Dict[str, Any]:
        conn = self._connect()
        cursor = conn.execute(
            'INSERT INTO condition_sets (name, facts) VALUES (?, ?)',
            (name, json.dumps(facts, ensure_ascii=False))
        )
        cs_id = cursor.lastrowid
        conn.commit()
        conn.close()
        return self.get_by_id(cs_id)

    def get_by_id(self, cs_id: int) -> Optional[Dict[str, Any]]:
        conn = self._connect()
        cursor = conn.execute('SELECT * FROM condition_sets WHERE id = ?', (cs_id,))
        row = cursor.fetchone()
        conn.close()
        if not row:
            return None
        return {
            'id': row['id'],
            'name': row['name'],
            'facts': json.loads(row['facts']),
            'created_at': row['created_at'],
            'updated_at': row['updated_at'],
        }

    def list_all(self, page: int = 1, limit: int = 20) -> Dict[str, Any]:
        conn = self._connect()
        total = conn.execute('SELECT COUNT(*) FROM condition_sets').fetchone()[0]
        offset = (page - 1) * limit
        cursor = conn.execute(
            'SELECT * FROM condition_sets ORDER BY updated_at DESC LIMIT ? OFFSET ?',
            (limit, offset)
        )
        rows = cursor.fetchall()
        conn.close()
        items = []
        for row in rows:
            items.append({
                'id': row['id'],
                'name': row['name'],
                'facts': json.loads(row['facts']),
                'created_at': row['created_at'],
                'updated_at': row['updated_at'],
            })
        return {
            'condition_sets': items,
            'total': total,
            'page': page,
            'limit': limit,
            'total_pages': (total + limit - 1) // limit,
        }

    def update(self, cs_id: int, name: Optional[str] = None,
               facts: Optional[List[str]] = None) -> Optional[Dict[str, Any]]:
        updates = []
        params = []
        if name is not None:
            updates.append('name = ?')
            params.append(name)
        if facts is not None:
            updates.append('facts = ?')
            params.append(json.dumps(facts, ensure_ascii=False))
        if not updates:
            return self.get_by_id(cs_id)
        updates.append('updated_at = CURRENT_TIMESTAMP')
        params.append(cs_id)
        conn = self._connect()
        cursor = conn.execute(
            f'UPDATE condition_sets SET {", ".join(updates)} WHERE id = ?',
            params
        )
        conn.commit()
        if cursor.rowcount == 0:
            conn.close()
            return None
        conn.close()
        self._notify_invalidate(cs_id)
        return self.get_by_id(cs_id)

    def delete(self, cs_id: int) -> bool:
        conn = self._connect()
        cursor = conn.execute('DELETE FROM condition_sets WHERE id = ?', (cs_id,))
        conn.commit()
        deleted = cursor.rowcount > 0
        conn.close()
        if deleted:
            self._notify_invalidate(cs_id)
        return deleted


class AlgorithmEngine:
    """统一算法引擎接口"""

    def __init__(self, algo_type: AlgorithmType, cs_manager: ConditionSetManager):
        self.algo_type = algo_type
        self.cs_manager = cs_manager
        self.kb = None
        self.fb = None
        self.engine = None
        self._rete_cache: Dict[int, Dict[str, Any]] = {}
        self._initialize()

    def _initialize(self):
        """根据算法类型初始化对应的引擎"""
        if self.algo_type == AlgorithmType.FULLSCAN:
            self._init_fullscan()
        elif self.algo_type == AlgorithmType.INCREMENTAL:
            self._init_incremental()
        elif self.algo_type == AlgorithmType.RETE:
            self._init_rete()
        self.cs_manager.register_invalidator(self._invalidate_rete_cache)

    def _invalidate_rete_cache(self, cs_id: int):
        """条件集变更时使 Rete 缓存失效"""
        self._rete_cache.pop(cs_id, None)

    def _init_fullscan(self):
        """初始化全扫描算法"""
        self.kb = FullScanKB()
        self.fb = FullScanFB()
        self.engine = FullScanEngine(self.kb, self.fb)

        from fullscan_py.knowledge_base import DB_PATH as _DB
        from fullscan_py.web_server import load_wikipedia_rules
        if self.kb.is_empty():
            load_wikipedia_rules()

    def _init_incremental(self):
        """初始化增量触发算法"""
        self.kb = IncrementalKB()
        self.fb = IncrementalFB()
        self.engine = IncrementalEngine(self.kb, self.fb)

        from incremental_py.web_server import load_wikipedia_rules
        if self.kb.is_empty():
            load_wikipedia_rules()

    def _init_rete(self):
        """初始化Rete网络算法"""
        from rete_py.knowledge_base import KnowledgeBase as ReteKB

        self.kb = ReteKB()
        self.fb = set()
        self.engine = ReteInferenceEngine()

        for r in self.kb.get_rules():
            self.engine.add_rule(Rule(r.id, r.conditions, r.conclusion))
        self.engine.build_network()

    def get_rules(self):
        """获取所有规则"""
        if self.algo_type == AlgorithmType.RETE:
            return [{'id': r.id, 'conditions': r.conditions, 'conclusion': r.conclusion}
                    for r in self.engine.rules]
        else:
            return [{'id': r.id, 'conditions': r.conditions, 'conclusion': r.conclusion}
                    for r in self.kb.get_rules()]

    def add_rule(self, conditions, conclusion):
        """添加规则"""
        if self.algo_type == AlgorithmType.RETE:
            from rete_py.knowledge_base import DB_PATH as _DB
            rid = len(self.engine.rules) + 1
            self.engine.add_rule(Rule(rid, conditions, conclusion))
            return rid
        else:
            return self.kb.add_rule(conditions, conclusion)

    def delete_rule(self, rule_id):
        """删除规则"""
        if self.algo_type == AlgorithmType.RETE:
            self.engine.rules = [r for r in self.engine.rules if r.id != rule_id]
            self.engine.built = False
        else:
            self.kb.delete_rule(rule_id)
        self._rete_cache.clear()

    def modify_rule(self, rule_id, conditions=None, conclusion=None):
        """修改规则"""
        if self.algo_type == AlgorithmType.RETE:
            for r in self.engine.rules:
                if r.id == rule_id:
                    if conditions is not None:
                        r.conditions = conditions
                    if conclusion is not None:
                        r.conclusion = conclusion
                    break
            self.engine.built = False
            return True
        else:
            return self.kb.modify_rule(rule_id, conditions, conclusion)

    def find_duplicate(self, conditions, conclusion):
        """检查重复规则"""
        if self.algo_type == AlgorithmType.RETE:
            for r in self.engine.rules:
                if r.conditions == conditions and r.conclusion == conclusion:
                    return r.id
            return None
        else:
            return self.kb.find_duplicate(conditions, conclusion)

    def get_facts(self):
        """获取当前工作内存中的事实"""
        if self.algo_type == AlgorithmType.RETE:
            return list(self.engine.fb)
        else:
            return list(self.fb.get_facts())

    def get_all_available_facts(self):
        """获取知识库中所有可用的条件事实"""
        all_facts = set()
        if self.algo_type == AlgorithmType.RETE:
            for r in self.engine.rules:
                all_facts.update(r.conditions)
        else:
            all_facts = set(self.kb.condition_index.keys())
        return sorted(list(all_facts))

    def add_fact(self, fact):
        """添加事实"""
        if self.algo_type == AlgorithmType.RETE:
            self.engine.add_fact(fact)
        else:
            self.fb.add_fact(fact)

    def remove_fact(self, fact):
        """删除事实"""
        if self.algo_type == AlgorithmType.RETE:
            self.engine.fb.discard(fact)
            if self.engine.built:
                self.engine.network.facts_set.discard(fact)
        else:
            return self.fb.remove_fact(fact)

    def clear_facts(self):
        """清空事实库"""
        if self.algo_type == AlgorithmType.RETE:
            self.engine.fb.clear()
            if self.engine.built:
                self.engine.network.facts_set.clear()
                for alpha in self.engine.network.alpha_nodes.values():
                    alpha.memory.clear()
                for beta in self.engine.network.beta_nodes:
                    beta.completed.clear()
                    beta.pending_left.clear()
                for t in self.engine.network.terminals:
                    t.results.clear()
        else:
            self.fb.clear()

    def _fb_save(self) -> List[str]:
        """保存当前 fb 状态"""
        if self.algo_type == AlgorithmType.RETE:
            return list(self.engine.fb)
        else:
            return list(self.fb.get_facts())

    def _fb_load(self, facts: List[str]):
        """清空并加载 facts 到 fb"""
        if self.algo_type == AlgorithmType.RETE:
            self.engine.fb.clear()
            for f in facts:
                self.engine.fb.add(f)
        else:
            self.fb.clear()
            for f in facts:
                self.fb.add_fact(f)

    def _fb_restore(self, facts: List[str]):
        """清空 fb 并恢复 facts"""
        if self.algo_type == AlgorithmType.RETE:
            self.engine.fb.clear()
            for f in facts:
                self.engine.fb.add(f)
        else:
            self.fb.clear()
            for f in facts:
                self.fb.add_fact(f)

    def forward_chain(self, facts: List[str] = None):
        """
        执行正向推理（无状态版本）
        - facts 为 None 时使用工作内存（兼容旧接口）
        - facts 有值时临时注入，推理后恢复原状态
        """
        if facts is None:
            facts = self.get_facts()

        if not facts:
            self.engine.reset_steps()
            return []

        original = self._fb_save()
        self._fb_load(facts)

        self.engine.reset_steps()
        if self.algo_type == AlgorithmType.RETE:
            self.engine.build_network()
            self.engine.network.facts_set.clear()
            for alpha in self.engine.network.alpha_nodes.values():
                alpha.memory.clear()
            for beta in self.engine.network.beta_nodes:
                beta.completed.clear()
                beta.pending_left.clear()
            for t in self.engine.network.terminals:
                t.results.clear()
            self.engine.network.facts_set.update(self.engine.fb)
            for f in self.engine.fb:
                self.engine.network.add_fact(f)
            new_facts = self.engine.forward_chain()
        else:
            new_facts = self.engine.forward_chain()

        self._fb_restore(original)
        return new_facts

    def forward_chain_with_cache(self, condition_set_id: int,
                                  facts: List[str]) -> Dict[str, Any]:
        """
        带缓存的前向推理（Rete 专用，其他算法直接执行）
        """
        cache_hit = False
        if self.algo_type == AlgorithmType.RETE:
            entry = self._rete_cache.get(condition_set_id)
            if entry and entry.get('facts_snapshot') == frozenset(facts):
                cache_hit = True
                new_facts = entry.get('deduced_facts', [])

        if not cache_hit:
            new_facts = self.forward_chain(facts)
            if self.algo_type == AlgorithmType.RETE:
                self._rete_cache[condition_set_id] = {
                    'facts_snapshot': frozenset(facts),
                    'deduced_facts': new_facts,
                }

        return {
            'new_facts': new_facts,
            'all_facts': list(set(facts) | set(new_facts)),
            'cache_hit': cache_hit,
        }

    def backward_chain(self, goal: str, facts: List[str] = None) -> bool:
        """
        执行反向推理（无状态版本）
        - facts 为 None 时使用工作内存
        - facts 有值时临时注入，推理后恢复原状态
        """
        original = self._fb_save()
        if facts:
            self._fb_load(facts)
        try:
            self.engine.reset_steps()
            result = self.engine.backward_chain(goal)
            return result
        finally:
            self._fb_restore(original)

    def get_steps(self):
        """获取推理步骤"""
        return self.engine.get_steps()

    def reset_steps(self):
        """重置推理步骤"""
        self.engine.reset_steps()

    def get_network_stats(self):
        """获取Rete网络统计（Rete特有）"""
        if self.algo_type == AlgorithmType.RETE:
            if not self.engine.built:
                self.engine.build_network()
            return self.engine.network.get_network_stats()
        return None

    def get_network_trace(self):
        """获取Rete网络追踪（Rete特有）"""
        if self.algo_type == AlgorithmType.RETE:
            trace = []
            for t in self.engine.network.trace:
                entry = dict(t)
                if 'matched_facts' in entry:
                    entry['matched_facts'] = [str(f) for f in entry['matched_facts']]
                trace.append(entry)
            return trace
        return None


class EngineManager:
    """算法引擎管理器"""

    def __init__(self):
        self.cs_manager = ConditionSetManager()
        self.engines = {
            AlgorithmType.FULLSCAN: AlgorithmEngine(AlgorithmType.FULLSCAN, self.cs_manager),
            AlgorithmType.INCREMENTAL: AlgorithmEngine(AlgorithmType.INCREMENTAL, self.cs_manager),
            AlgorithmType.RETE: AlgorithmEngine(AlgorithmType.RETE, self.cs_manager),
        }

    def get_engine(self, algo_type: str) -> AlgorithmEngine:
        """获取指定算法的引擎"""
        try:
            algo = AlgorithmType(algo_type.lower())
            return self.engines[algo]
        except (ValueError, KeyError):
            raise ValueError(f"不支持的算法类型: {algo_type}，支持的类型: fullscan, incremental, rete")

    def get_all_stats(self):
        """获取所有算法的统计信息"""
        stats = {}
        for algo_type, engine in self.engines.items():
            stats[algo_type.value] = {
                'rules_count': len(engine.get_rules()),
                'facts_count': len(engine.get_facts()),
            }
        return stats
