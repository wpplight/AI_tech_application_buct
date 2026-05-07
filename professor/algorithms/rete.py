"""
Rete 算法 —— Alpha/Beta 节点网络匹配
薄封装，核心逻辑见 rete_py/
"""
import sys
import os

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'rete_py'))

from rete_runner import Rule, ReteInferenceEngine


class ReteEngine:
    def __init__(self, rules):
        self.engine = ReteInferenceEngine()
        for i, (conditions, conclusion) in enumerate(rules):
            self.engine.add_rule(Rule(i + 1, conditions, conclusion))
        self.engine.build_network()

    def forward(self, input_facts):
        net = self.engine.network
        net.facts_set.clear()
        for a in net.alpha_nodes.values():
            a.memory.clear()
        for b in net.beta_nodes:
            b.completed.clear()
            b.pending_left.clear()
        for t in net.terminals:
            t.results.clear()

        self.engine.set_facts(input_facts)
        return self.engine.forward_chain()
