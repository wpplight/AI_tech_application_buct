"""
Rete 算法简化实现 — 用于动物识别专家系统

核心概念：
  Alpha Memory  — 按条件类型分类的事实集合
  Beta Memory   — 部分匹配的累积（AND 连接）
  Terminal Node — 规则完全满足，触发推导

网络结构示意（以识别鸵鸟为例）:

        [有羽毛] → Alpha(有羽毛) ─┐
                                   ├→ Beta(是鸟) → Terminal(Rule 1: 是鸟)
        [有羽毛] → Alpha(有羽毛) ─┐
                                   ├→ Beta(不会飞) ─┐
        [不会飞]→ Alpha(不会飞) ─┘                  │
                                                     ├→ Beta(有长腿) ──┐
        [有长腿]→ Alpha(有长腿) ─────────────────────┘                  │
                                                                         ├→ Beta(有长颈) ──┐
        [有长颈]→ Alpha(有长颈) ─────────────────────────────────────────┘                  │
                                                                                             ├→ Terminal(Rule 3: 是鸵鸟)
        [黑白二色]→ Alpha(黑白二色) ─────────────────────────────────────────────────────────┘
"""
import time
import random


class Fact:
    __slots__ = ('name',)

    def __init__(self, name):
        self.name = name

    def __repr__(self):
        return self.name

    def __hash__(self):
        return hash(self.name)

    def __eq__(self, other):
        return self.name == other.name if isinstance(other, Fact) else False


class WME:
    """Working Memory Element — 推理过程中的中间事实"""
    __slots__ = ('fact', 'timestamp')

    def __init__(self, fact):
        self.fact = fact
        self.timestamp = 0

    def __repr__(self):
        return f"WME({self.fact})"


class Token:
    """Beta Memory 中的部分匹配令牌 — 记录"已有哪些条件满足"的一组事实"""
    __slots__ = ('facts', 'wmes', 'parent')

    def __init__(self, facts=None, wmes=None, parent=None):
        self.facts = facts or []
        self.wmes = wmes or []
        self.parent = parent

    def __repr__(self):
        return f"Token({[f.name for f in self.facts]})"


class AlphaNode:
    """
    Alpha Node（类型节点）
    功能：按条件名分发事实。"有羽毛"的事实只会进入 Alpha(有羽毛)
    每个 Alpha Node 维护一个 Alpha Memory（满足该条件的所有 WME）
    """
    __slots__ = ('condition', 'memory', 'children', 'timestamp_counter', 'tracer')

    def __init__(self, condition, tracer=None):
        self.condition = condition
        self.memory = []      # Alpha Memory
        self.children = []    # 下游 Beta / Terminal 节点
        self.timestamp_counter = 0
        self.tracer = tracer

    def activate(self, wme):
        """事实进入 Alpha 节点 → 加入 Alpha Memory → 向下传播"""
        wme.timestamp = self.timestamp_counter
        self.timestamp_counter += 1
        self.memory.append(wme)
        if self.tracer is not None:
            self.tracer.append({
                'type': 'alpha_activate',
                'condition': self.condition,
                'fact': wme.fact.name,
                'children': len(self.children)
            })
        for child in self.children:
            child.right_activate(wme, tracer=self.tracer)

    def add_child(self, node):
        self.children.append(node)

    def __repr__(self):
        return f"Alpha({self.condition})"


class BetaNode:
    """
    Beta Node（联合节点）
    功能：实现 AND 连接。左侧输入 = 上游结果（Token），右侧输入 = Alpha Memory。

    左激活（left_activate）：上游 Token 到达
      → 先存为 pending_left → 再尝试与已有右 WME 组合

    右激活（right_activate）：新 WME 到达
      → 先加入 Alpha Memory → 再与已有 pending_left Token 组合
    """
    __slots__ = ('rule_id', 'condition', 'completed', 'pending_left',
                 'alpha_node', 'children', 'is_chain_head')

    def __init__(self, rule_id, condition):
        self.rule_id = rule_id
        self.condition = condition
        self.completed = []       # 已完整匹配的 Token
        self.pending_left = []    # 等右侧 WME 的左 Token
        self.alpha_node = None
        self.children = []
        self.is_chain_head = True

    def set_alpha(self, alpha_node):
        self.alpha_node = alpha_node

    def add_child(self, node):
        self.children.append(node)

    def left_activate(self, token, tracer=None):
        """左侧激活：上游 Token 到达"""
        self.pending_left.append(token)
        new_tokens = []
        for wme in self.alpha_node.memory:
            new_token = Token(
                facts=token.facts + [wme.fact],
                wmes=token.wmes + [wme],
                parent=token
            )
            self.completed.append(new_token)
            new_tokens.append(new_token)
        if tracer is not None and new_tokens:
            tracer.append({
                'type': 'beta_match',
                'rule_id': self.rule_id,
                'condition': self.condition,
                'matched_facts': [f.name for f in new_tokens[0].facts] if new_tokens else [],
                'is_chain_head': self.is_chain_head,
                'combined_count': len(new_tokens)
            })
        for child in self.children:
            for nt in new_tokens:
                child.left_activate(nt, tracer=tracer)

    def right_activate(self, wme, tracer=None):
        """右侧激活：新 WME 加入 Alpha Memory"""
        new_tokens = []

        if self.is_chain_head:
            new_token = Token(facts=[wme.fact], wmes=[wme])
            self.completed.append(new_token)
            self.pending_left.append(new_token)
            new_tokens.append(new_token)
            if tracer is not None:
                tracer.append({
                    'type': 'beta_match',
                    'rule_id': self.rule_id,
                    'condition': self.condition,
                    'matched_facts': [wme.fact.name],
                    'is_chain_head': True
                })
        else:
            for token in self.pending_left:
                new_token = Token(
                    facts=token.facts + [wme.fact],
                    wmes=token.wmes + [wme],
                    parent=token
                )
                self.completed.append(new_token)
                new_tokens.append(new_token)
            if tracer is not None and new_tokens:
                tracer.append({
                    'type': 'beta_match',
                    'rule_id': self.rule_id,
                    'condition': self.condition,
                    'matched_facts': new_tokens[0].facts if new_tokens else [],
                    'is_chain_head': False,
                    'combined_count': len(new_tokens)
                })

        for child in self.children:
            for nt in new_tokens:
                child.left_activate(nt, tracer=tracer)

    def __repr__(self):
        return f"Beta(R{self.rule_id}:{self.condition})"


class TerminalNode:
    """终端节点：规则完全匹配，触发推导"""
    __slots__ = ('rule_id', 'conclusion', 'conditions', 'results', 'tracer')

    def __init__(self, rule_id, conclusion, conditions, tracer=None):
        self.rule_id = rule_id
        self.conclusion = conclusion
        self.conditions = conditions
        self.results = []
        self.tracer = tracer

    def left_activate(self, token, tracer=None):
        facts_list = [f.name for f in token.facts]
        self.results.append((self.rule_id, self.conclusion, token.facts))
        t = tracer or self.tracer
        if t is not None:
            t.append({
                'type': 'terminal_fire',
                'rule_id': self.rule_id,
                'conclusion': self.conclusion,
                'conditions': self.conditions,
                'matched_facts': facts_list
            })

    def get_new_conclusions(self, known_facts_set):
        return [c for _, c, _ in self.results if c not in known_facts_set]

    def __repr__(self):
        return f"Terminal(R{self.rule_id}:{self.conclusion})"


class ReteNetwork:
    """
    Rete 网络编译器 + 执行引擎

    使用流程：
      1. 调用 build() 编译规则为 Rete 网络
      2. 调用 add_fact() 注入事实，网络自动传播
      3. 调用 get_new_facts() 获取新推导的结论
    """

    def __init__(self):
        self.alpha_nodes = {}      # condition → AlphaNode
        self.beta_nodes = []       # 链式 Beta 节点
        self.terminals = []        # TerminalNode
        self.facts_set = set()     # 已知事实名集合
        self.trace = []            # 推理追踪日志
        self._build_time_ns = 0

    def clear_trace(self):
        self.trace = []

    def build(self, rules):
        """编译规则集为 Rete 网络"""
        total_start = time.perf_counter_ns()
        self.alpha_nodes.clear()
        self.beta_nodes.clear()
        self.terminals.clear()
        self.trace = []
        for rule in rules:
            rule_id, conditions, conclusion = rule.id, rule.conditions, rule.conclusion
            # 创建 / 获取 Alpha 节点并串联 Beta 链
            prev_node = None
            beta_chain = []
            for cond in conditions:
                if cond not in self.alpha_nodes:
                    self.alpha_nodes[cond] = AlphaNode(cond, tracer=self.trace)
                alpha = self.alpha_nodes[cond]
                beta = BetaNode(rule_id, cond)
                beta.set_alpha(alpha)
                alpha.add_child(beta)
                self.beta_nodes.append(beta)
                if prev_node:
                    prev_node.add_child(beta)
                    beta.is_chain_head = False
                prev_node = beta
                beta_chain.append(beta)
            # 最后一个 Beta 节点连接到 Terminal
            terminal = TerminalNode(rule_id, conclusion, conditions, tracer=self.trace)
            if beta_chain:
                beta_chain[-1].add_child(terminal)
            self.terminals.append(terminal)
        self._build_time_ns = time.perf_counter_ns() - total_start
        return self._build_time_ns

    def add_fact(self, fact_name):
        fact = Fact(fact_name)
        wme = WME(fact)
        if fact_name in self.alpha_nodes:
            self.alpha_nodes[fact_name].activate(wme)
        self.facts_set.add(fact_name)

    def add_facts(self, fact_names):
        for f in fact_names:
            self.add_fact(f)

    def get_new_facts(self):
        new = set()
        for t in self.terminals:
            for c in t.get_new_conclusions(self.facts_set):
                new.add(c)
        return new

    def get_network_stats(self):
        total_alpha_memory = sum(len(n.memory) for n in self.alpha_nodes.values())
        total_beta_memory = sum(len(n.completed) + len(n.pending_left) for n in self.beta_nodes)
        return {
            'alpha_nodes': len(self.alpha_nodes),
            'beta_nodes': len(self.beta_nodes),
            'terminals': len(self.terminals),
            'alpha_memory_size': total_alpha_memory,
            'beta_memory_size': total_beta_memory,
            'build_time_us': self._build_time_ns / 1000
        }

    def visualize(self):
        """生成 Rete 网络的 DOT 图形描述（可用 Graphviz 渲染）"""
        lines = ['digraph Rete {']
        lines.append('  rankdir=LR;')
        lines.append('  node [shape=box, style=filled];')
        # Root
        lines.append('  root [label="新事实入口", shape=ellipse, fillcolor="#e0f2fe"];')
        for cond, alpha in self.alpha_nodes.items():
            lines.append(f'  alpha_{cond} [label="Alpha\\n{cond}", fillcolor="#dbeafe"];')
            lines.append(f'  root -> alpha_{cond} [style=dashed, color=gray];')
        for beta in self.beta_nodes:
            node_id = f"beta_{beta.rule_id}_{beta.condition}"
            lines.append(f'  {node_id} [label="β Rule{beta.rule_id}\\nAND {beta.condition}", fillcolor="#fef3c7"];')
            lines.append(f'  alpha_{beta.condition} -> {node_id};')
        for t in self.terminals:
            node_id = f"term_{t.rule_id}"
            conditions_str = '\\n'.join(t.conditions)
            lines.append(f'  {node_id} [label="Rule {t.rule_id}\\nTHEN {t.conclusion}\\n条件: {conditions_str}", fillcolor="#dcfce7"];')
            # 找最后一个 beta 链
            last_beta = None
            for beta in self.beta_nodes:
                if beta.rule_id == t.rule_id and t.conditions and beta.condition == t.conditions[-1]:
                    last_beta = beta
            if last_beta:
                lines.append(f'  beta_{last_beta.rule_id}_{last_beta.condition} -> {node_id};')
        lines.append('}')
        return '\n'.join(lines)

    def print_network(self):
        """控制台可视化 Rete 网络"""
        print("\n" + "=" * 70)
        print("  Rete 网络结构")
        print("=" * 70)
        print(f"\n  ┌─ Alpha Nodes ({len(self.alpha_nodes)} 个) ───────────────────┐")
        for cond, alpha in self.alpha_nodes.items():
            mem_size = len(alpha.memory)
            children_count = len(alpha.children)
            print(f"  │  Alpha({cond})  →  {children_count} 个下游节点, 内存: {mem_size} 条")
        print(f"  └{'─' * 55}┘")
        print(f"\n  ┌─ Beta Nodes ({len(self.beta_nodes)} 个) ────────────────────┐")
        for beta in self.beta_nodes:
            mem_size = len(beta.completed) + len(beta.pending_left)
            children_count = len(beta.children)
            print(f"  │  Beta(R{beta.rule_id}:{beta.condition})  →  {children_count} 个子节点, 内存: {mem_size} 条")
        print(f"  └{'─' * 55}┘")
        print(f"\n  ┌─ Terminal Nodes ({len(self.terminals)} 个) ─────────────────┐")
        for t in self.terminals:
            print(f"  │  Rule {t.rule_id}: IF {' AND '.join(t.conditions)} THEN {t.conclusion}")
        print(f"  └{'─' * 55}┘")
        stats = self.get_network_stats()
        print(f"\n  总计: {stats['alpha_nodes']} Alpha + {stats['beta_nodes']} Beta + {stats['terminals']} Terminal")
        print(f"  Alpha Memory: {stats['alpha_memory_size']} 条 | Beta Memory: {stats['beta_memory_size']} 条")
        print(f"  网络编译耗时: {stats['build_time_us']:.2f} μs")
