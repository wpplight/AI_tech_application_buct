"""
专家系统统一服务器
将三种推理算法整合到单一端口提供服务
"""

import json
import sys
import os
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
from typing import Optional, Dict, Any, List

import algorithms
from algorithms import EngineManager, AlgorithmType


class UnifiedServer:
    """统一服务器"""

    def __init__(self, port: int = 8080):
        self.port = port
        self.manager = EngineManager()
        self.server = None

    def start(self):
        """启动服务器"""
        Handler = self._create_handler()
        self.server = HTTPServer(('0.0.0.0', self.port), Handler)
        print(f"动物识别专家系统 - 统一服务器")
        print(f"支持算法: fullscan, incremental, rete")
        print(f"请在浏览器中打开: http://localhost:{self.port}")
        print(f"API 基础路径: /api?algo=<algorithm>")
        print(f"按 Ctrl+C 停止服务器")
        try:
            self.server.serve_forever()
        except KeyboardInterrupt:
            print("\n服务器已停止")
            self.server.server_close()

    def _create_handler(self):
        """创建请求处理器"""
        manager = self.manager

        class UnifiedHandler(BaseHTTPRequestHandler):

            def log_message(self, format, *args):
                """自定义日志格式"""
                algo = self._get_algorithm()
                sys.stderr.write(f"[{algo}] {format % args}\n")

            def _get_algorithm(self) -> str:
                """从URL参数获取算法类型"""
                parsed = urlparse(self.path)
                params = parse_qs(parsed.query)
                algo = params.get('algo', ['fullscan'])[0]
                return algo

            def _get_engine(self):
                """获取当前请求对应的算法引擎"""
                algo = self._get_algorithm()
                try:
                    return manager.get_engine(algo)
                except ValueError as e:
                    self._send_error(str(e), 400)
                    return None

            def _send_json(self, data: Dict[str, Any], status: int = 200):
                """发送JSON响应"""
                self.send_response(status)
                self.send_header('Content-Type', 'application/json; charset=utf-8')
                self.send_header('Access-Control-Allow-Origin', '*')
                self.send_header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
                self.send_header('Access-Control-Allow-Headers', 'Content-Type')
                self.end_headers()
                self.wfile.write(json.dumps(data, ensure_ascii=False).encode('utf-8'))

            def _send_error(self, message: str, status: int = 400):
                """发送错误响应"""
                self._send_json({'success': False, 'error': message}, status)

            def _read_body(self) -> Dict[str, Any]:
                """读取请求体"""
                length = int(self.headers.get('Content-Length', 0))
                if length > 0:
                    return json.loads(self.rfile.read(length).decode('utf-8'))
                return {}

            def do_OPTIONS(self):
                """处理CORS预检请求"""
                self.send_response(200)
                self.send_header('Access-Control-Allow-Origin', '*')
                self.send_header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
                self.send_header('Access-Control-Allow-Headers', 'Content-Type')
                self.end_headers()

            def _search_rules(self, rules: List[Dict], query: str) -> List[Dict]:
                """模糊搜索规则"""
                query_lower = query.lower()
                filtered = []
                for rule in rules:
                    if (query_lower in rule['conclusion'].lower() or
                        any(query_lower in cond.lower() for cond in rule['conditions'])):
                        filtered.append(rule)
                return filtered

            def _get_rule_by_id(self, rules: List[Dict], rule_id: int) -> Optional[Dict]:
                """根据ID获取规则"""
                for rule in rules:
                    if rule['id'] == rule_id:
                        return rule
                return None

            def _get_rules_by_fact(self, rules: List[Dict], fact: str) -> List[Dict]:
                """获取包含指定事实的所有规则"""
                related = []
                for rule in rules:
                    if fact in rule['conditions']:
                        related.append(rule)
                return related

            def _parse_condition_set_id_from_path(self, path: str) -> Optional[int]:
                """从路径中解析条件集 ID，如 /api/condition-sets/5"""
                parts = path.rstrip('/').split('/')
                if len(parts) >= 3 and parts[-2] == 'condition-sets':
                    try:
                        return int(parts[-1])
                    except (ValueError, IndexError):
                        return None
                return None

            # ==================== GET ====================
            def do_GET(self):
                """处理GET请求"""
                parsed = urlparse(self.path)
                path = parsed.path
                query_params = parse_qs(parsed.query)
                algo = self._get_algorithm()

                if path == '/':
                    self._send_json({
                        'name': '动物识别专家系统 - 统一服务器',
                        'version': '2.0',
                        'algorithms': ['fullscan', 'incremental', 'rete'],
                        'usage': {
                            'base_url': f'/api?algo={algo}',
                            'rules_endpoint': f'/api/rules?algo={algo}',
                            'facts_endpoint': f'/api/facts?algo={algo}',
                            'condition_sets_endpoint': f'/api/condition-sets',
                        }
                    })

                # ---- 条件集 ----
                elif path == '/api/condition-sets':
                    page = int(query_params.get('page', [1])[0])
                    limit = int(query_params.get('limit', [20])[0])
                    result = manager.cs_manager.list_all(page=page, limit=limit)
                    result['algorithm'] = algo
                    self._send_json(result)

                elif path.startswith('/api/condition-sets/') and path != '/api/condition-sets':
                    cs_id = self._parse_condition_set_id_from_path(path)
                    if cs_id is None:
                        self._send_error('无效的条件集ID', 400)
                        return
                    cs = manager.cs_manager.get_by_id(cs_id)
                    if cs is None:
                        self._send_error(f'条件集 {cs_id} 不存在', 404)
                        return
                    self._send_json({'condition_set': cs, 'algorithm': algo})

                elif path == '/api/rules/search':
                    engine = self._get_engine()
                    if engine is None:
                        return
                    all_rules = engine.get_rules()
                    query = query_params.get('q', [''])[0]
                    if not query:
                        self._send_error('缺少搜索关键词参数 q')
                        return
                    filtered_rules = self._search_rules(all_rules, query)
                    self._send_json({
                        'rules': filtered_rules,
                        'algorithm': algo,
                        'total': len(filtered_rules),
                        'query': query
                    })

                elif path.startswith('/api/rules/') and not path.startswith('/api/rules?'):
                    parts = path.split('/')
                    if len(parts) >= 4:
                        try:
                            rule_id = int(parts[3])
                            engine = self._get_engine()
                            if engine is None:
                                return
                            all_rules = engine.get_rules()
                            rule = self._get_rule_by_id(all_rules, rule_id)
                            if rule:
                                related_rules = self._get_rules_by_fact(all_rules, rule['conclusion'])
                                self._send_json({
                                    'rule': rule,
                                    'algorithm': algo,
                                    'related_rules': related_rules
                                })
                            else:
                                self._send_error(f'规则 {rule_id} 不存在', 404)
                        except ValueError:
                            self._send_error('无效的规则ID', 400)

                elif path == '/api/rules':
                    engine = self._get_engine()
                    if engine is None:
                        return
                    all_rules = engine.get_rules()
                    page = int(query_params.get('page', [1])[0])
                    limit = int(query_params.get('limit', [50])[0])
                    search = query_params.get('search', [''])[0]
                    fact = query_params.get('fact', [''])[0]
                    filtered_rules = all_rules
                    if search:
                        filtered_rules = self._search_rules(all_rules, search)
                    if fact:
                        filtered_rules = self._get_rules_by_fact(all_rules, fact)
                    total = len(filtered_rules)
                    start = (page - 1) * limit
                    end = start + limit
                    paginated_rules = filtered_rules[start:end]
                    self._send_json({
                        'rules': paginated_rules,
                        'algorithm': algo,
                        'pagination': {
                            'page': page,
                            'limit': limit,
                            'total': total,
                            'total_pages': (total + limit - 1) // limit
                        }
                    })

                elif path == '/api/facts':
                    engine = self._get_engine()
                    if engine is None:
                        return
                    search = query_params.get('search', [''])[0]
                    working = query_params.get('working', ['false'])[0] == 'true'
                    page = int(query_params.get('page', [1])[0])
                    limit = int(query_params.get('limit', [1000])[0])
                    if working:
                        facts = engine.get_facts()
                    else:
                        facts = engine.get_all_available_facts()
                    if search:
                        facts = [f for f in facts if search.lower() in f.lower()]
                    total = len(facts)
                    start = (page - 1) * limit
                    end = start + limit
                    paginated_facts = facts[start:end]
                    self._send_json({
                        'facts': paginated_facts,
                        'algorithm': algo,
                        'total': total,
                        'search': search if search else None,
                        'pagination': {
                            'page': page,
                            'limit': limit,
                            'total': total,
                            'total_pages': (total + limit - 1) // limit
                        }
                    })

                elif path == '/api/inference/steps':
                    engine = self._get_engine()
                    if engine is None:
                        return
                    steps = engine.get_steps()
                    step_list = []
                    for s in steps:
                        item = {'type': s['type']}
                        if s['type'] == 'forward':
                            if algo == 'rete':
                                item['new_fact'] = s.get('new_fact', '')
                                item['iteration'] = s.get('iteration', 0)
                            else:
                                item['rule_id'] = s['rule'].id
                                item['new_fact'] = s['new_fact']
                                item['iteration'] = s['iteration']
                                item['rule_conditions'] = s['rule'].conditions
                                item['rule_conclusion'] = s['rule'].conclusion
                        elif s['type'] == 'backward':
                            item['goal'] = s.get('goal', '')
                            if s.get('rule'):
                                if algo == 'rete':
                                    item['rule_id'] = s['rule'].id if hasattr(s['rule'], 'id') else s['rule']
                                else:
                                    item['rule_id'] = s['rule'].id
                                    item['rule_conditions'] = s['rule'].conditions
                                    item['rule_conclusion'] = s['rule'].conclusion
                            if s.get('sub_goal'):
                                item['sub_goal'] = s['sub_goal']
                            if s.get('result'):
                                item['result'] = s['result']
                            if s.get('attempt'):
                                item['attempt'] = s['attempt']
                            if s.get('conditions'):
                                item['conditions'] = s['conditions']
                            if 'proven_conditions' in s:
                                item['proven_conditions'] = s['proven_conditions']
                            if 'unproven_conditions' in s:
                                item['unproven_conditions'] = s['unproven_conditions']
                            if 'missing_facts' in s:
                                item['missing_facts'] = s['missing_facts']
                        step_list.append(item)
                    self._send_json({'steps': step_list, 'algorithm': algo})

                elif path == '/api/network/stats':
                    engine = self._get_engine()
                    if engine is None:
                        return
                    if algo == 'rete':
                        stats = engine.get_network_stats()
                        self._send_json(stats)
                    else:
                        self._send_error('网络统计仅适用于 Rete 算法', 400)

                elif path == '/api/network/trace':
                    engine = self._get_engine()
                    if engine is None:
                        return
                    if algo == 'rete':
                        trace = engine.get_network_trace()
                        self._send_json({'trace': trace})
                    else:
                        self._send_error('网络追踪仅适用于 Rete 算法', 400)

                elif path == '/api/algorithms':
                    stats = manager.get_all_stats()
                    self._send_json({'algorithms': stats})

                else:
                    self._send_error('Not Found', 404)

            # ==================== POST ====================
            def do_POST(self):
                """处理POST请求"""
                body = self._read_body()
                parsed = urlparse(self.path)
                path = parsed.path
                engine = self._get_engine()
                algo = self._get_algorithm()

                if path == '/api/condition-sets':
                    name = body.get('name', '').strip()
                    facts = body.get('facts', [])
                    if not name:
                        self._send_error('缺少 name 参数')
                        return
                    cs = manager.cs_manager.create(name, facts)
                    self._send_json({'success': True, 'condition_set': cs, 'algorithm': algo})

                elif path == '/api/rules':
                    conditions = body.get('conditions', [])
                    conclusion = body.get('conclusion', '')
                    if conditions and conclusion:
                        duplicate_id = engine.find_duplicate(conditions, conclusion)
                        if duplicate_id:
                            self._send_json({
                                'success': False,
                                'error': f'规则已存在 (Rule {duplicate_id})'
                            }, 400)
                        else:
                            rule_id = engine.add_rule(conditions, conclusion)
                            self._send_json({
                                'success': True,
                                'rule_id': rule_id,
                                'algorithm': algo
                            })
                    else:
                        self._send_error('参数错误：需要 conditions 和 conclusion')

                elif path == '/api/facts/add':
                    fact = body.get('fact', '')
                    if fact:
                        engine.add_fact(fact)
                        self._send_json({'success': True, 'algorithm': algo})
                    else:
                        self._send_error('缺少 fact')

                elif path == '/api/facts/delete':
                    fact = body.get('fact', '')
                    if fact:
                        engine.remove_fact(fact)
                        self._send_json({'success': True, 'algorithm': algo})
                    else:
                        self._send_error('缺少 fact')

                elif path == '/api/facts/clear':
                    engine.clear_facts()
                    self._send_json({'success': True, 'algorithm': algo})

                elif path == '/api/inference/forward':
                    condition_set_id = body.get('condition_set_id')
                    facts_input = body.get('facts', [])
                    if condition_set_id:
                        cs = manager.cs_manager.get_by_id(condition_set_id)
                        if cs is None:
                            self._send_error(f'条件集 {condition_set_id} 不存在', 404)
                            return
                        input_facts = cs['facts']
                        result = engine.forward_chain_with_cache(condition_set_id, input_facts)
                        new_facts = result['new_facts']
                        all_facts = result['all_facts']
                        cache_hit = result['cache_hit']
                    elif facts_input:
                        input_facts = facts_input
                        cache_hit = False
                        if algo == 'rete':
                            entry = engine._rete_cache.get(condition_set_id)
                            new_facts = engine.forward_chain(input_facts)
                        else:
                            new_facts = engine.forward_chain(input_facts)
                        all_facts = list(set(input_facts) | set(new_facts))
                    else:
                        cache_hit = False
                        input_facts = engine.get_facts()
                        new_facts = engine.forward_chain()
                        all_facts = engine.get_facts()

                    steps = engine.get_steps()
                    steps_list = []
                    for s in steps:
                        item = {'type': s['type']}
                        if s['type'] == 'forward':
                            if algo == 'rete':
                                item['new_fact'] = s.get('new_fact', '')
                            else:
                                item['rule_id'] = s['rule'].id
                                item['new_fact'] = s['new_fact']
                                item['rule_conditions'] = s['rule'].conditions
                                item['rule_conclusion'] = s['rule'].conclusion
                        elif s['type'] == 'backward':
                            item['goal'] = s.get('goal', '')
                            if s.get('rule'):
                                item['rule_id'] = s['rule'].id if hasattr(s['rule'], 'id') else s['rule']
                                item['rule_conditions'] = s['rule'].conditions
                                item['rule_conclusion'] = s['rule'].conclusion
                        steps_list.append(item)
                    response = {
                        'success': True,
                        'condition_set_id': condition_set_id,
                        'input_facts': input_facts,
                        'new_facts': new_facts,
                        'all_facts': all_facts,
                        'steps': steps_list,
                        'cache_hit': cache_hit,
                        'algorithm': algo
                    }
                    if algo == 'rete':
                        response['rete_trace'] = engine.get_network_trace()
                    self._send_json(response)

                elif path == '/api/inference/backward':
                    goal = body.get('goal', '')
                    condition_set_id = body.get('condition_set_id')
                    input_facts = []
                    if goal:
                        if condition_set_id:
                            cs = manager.cs_manager.get_by_id(condition_set_id)
                            if cs is None:
                                self._send_error(f'条件集 {condition_set_id} 不存在', 404)
                                return
                            input_facts = cs['facts']
                            success = engine.backward_chain(goal, input_facts)
                        else:
                            success = engine.backward_chain(goal)
                        steps = engine.get_steps()
                        missing_facts = []
                        if hasattr(engine.engine, 'get_missing_facts_for_goal'):
                            missing_facts = engine.engine.get_missing_facts_for_goal(goal)
                        elif algo == 'rete':
                            if not success:
                                missing_facts = [goal]
                        if not missing_facts:
                            for s in reversed(steps):
                                if s.get('missing_facts'):
                                    missing_facts = s['missing_facts']
                                    break
                        goal_already_known = goal in input_facts
                        steps_list = []
                        for s in steps:
                            item = {'type': s['type']}
                            if s['type'] == 'forward':
                                if algo == 'rete':
                                    item['new_fact'] = s.get('new_fact', '')
                                else:
                                    item['rule_id'] = s['rule'].id
                                    item['new_fact'] = s['new_fact']
                                    item['rule_conditions'] = s['rule'].conditions
                                    item['rule_conclusion'] = s['rule'].conclusion
                            elif s['type'] == 'backward':
                                item['goal'] = s.get('goal', '')
                                if s.get('rule'):
                                    if algo == 'rete':
                                        item['rule_id'] = s['rule'].id if hasattr(s['rule'], 'id') else s['rule']
                                    else:
                                        item['rule_id'] = s['rule'].id
                                    item['rule_conditions'] = s['rule'].conditions
                                    item['rule_conclusion'] = s['rule'].conclusion
                                if s.get('result'):
                                    item['result'] = s['result']
                                if s.get('attempt'):
                                    item['attempt'] = s['attempt']
                                if 'proven_conditions' in s:
                                    item['proven_conditions'] = s['proven_conditions']
                                if 'unproven_conditions' in s:
                                    item['unproven_conditions'] = s['unproven_conditions']
                                if 'missing_facts' in s:
                                    item['missing_facts'] = s['missing_facts']
                                if 'sub_goal' in s:
                                    item['sub_goal'] = s['sub_goal']
                            steps_list.append(item)
                        self._send_json({
                            'success': success,
                            'goal': goal,
                            'condition_set_id': condition_set_id,
                            'input_facts': input_facts,
                            'steps': steps_list,
                            'missing_facts': missing_facts,
                            'goal_already_known': goal_already_known,
                            'algorithm': algo
                        })
                    else:
                        self._send_error('缺少 goal')

                elif path == '/api/reset':
                    engine.clear_facts()
                    engine.reset_steps()
                    self._send_json({'success': True, 'algorithm': algo})

                else:
                    self._send_error('Not Found', 404)

            # ==================== PUT ====================
            def do_PUT(self):
                """处理PUT请求"""
                body = self._read_body()
                parsed = urlparse(self.path)
                path = parsed.path

                if path.startswith('/api/condition-sets/') and path != '/api/condition-sets':
                    cs_id = self._parse_condition_set_id_from_path(path)
                    if cs_id is None:
                        self._send_error('无效的条件集ID', 400)
                        return
                    name = body.get('name')
                    facts = body.get('facts')
                    cs = manager.cs_manager.update(cs_id, name=name, facts=facts)
                    if cs is None:
                        self._send_error(f'条件集 {cs_id} 不存在', 404)
                        return
                    self._send_json({'success': True, 'condition_set': cs, 'algorithm': self._get_algorithm()})
                else:
                    self._send_error('Not Found', 404)

            # ==================== DELETE ====================
            def do_DELETE(self):
                """处理DELETE请求"""
                parsed = urlparse(self.path)
                path = parsed.path

                if path.startswith('/api/condition-sets/') and path != '/api/condition-sets':
                    cs_id = self._parse_condition_set_id_from_path(path)
                    if cs_id is None:
                        self._send_error('无效的条件集ID', 400)
                        return
                    deleted = manager.cs_manager.delete(cs_id)
                    if not deleted:
                        self._send_error(f'条件集 {cs_id} 不存在', 404)
                        return
                    self._send_json({'success': True, 'algorithm': self._get_algorithm()})

                elif path.startswith('/api/rules/'):
                    try:
                        rule_id = int(path.split('/')[-1])
                        engine = self._get_engine()
                        if engine is None:
                            return
                        engine.delete_rule(rule_id)
                        self._send_json({'success': True, 'algorithm': self._get_algorithm()})
                    except (ValueError, IndexError):
                        self._send_error('无效的规则ID', 400)
                else:
                    self._send_error('Not Found', 404)

        return UnifiedHandler


def main():
    import argparse
    parser = argparse.ArgumentParser(description='动物识别专家系统 - 统一服务器')
    parser.add_argument('-p', '--port', type=int, default=8080,
                        help='服务器端口 (默认: 8080)')
    args = parser.parse_args()

    server = UnifiedServer(port=args.port)
    server.start()


if __name__ == '__main__':
    main()
