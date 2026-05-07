"""
Rete 推理执行器 — 完整的 Rete 正向推理 + 反向推理
"""
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from rete_network import ReteNetwork


class Rule:
    def __init__(self, id, conditions, conclusion):
        self.id = id
        self.conditions = conditions
        self.conclusion = conclusion
        self._id_set = set(conditions)


class ReteInferenceEngine:
    def __init__(self):
        self.rules = []
        self.network = ReteNetwork()
        self.fb = set()
        self.built = False
        self.steps = []

    def reset_steps(self):
        self.steps = []

    def get_steps(self):
        return self.steps.copy()

    def add_rule(self, rule):
        self.rules.append(rule)
        self.built = False

    def build_network(self):
        self.network = ReteNetwork()
        self.network.build(self.rules)
        self.built = True

    def clear_rules(self):
        self.rules.clear()
        self.network = ReteNetwork()
        self.built = False

    def add_fact(self, fact):
        self.fb.add(fact)
        if not self.built:
            self.build_network()
        self.network.add_fact(fact)

    def set_facts(self, facts):
        self.fb = set(facts)
        if not self.built:
            self.build_network()
        self.network.facts_set.clear()
        self.network.facts_set.update(facts)
        for f in facts:
            self.network.add_fact(f)

    def forward_chain(self):
        if not self.built:
            self.build_network()
        self.steps = []
        all_new = []
        iteration = 0
        while True:
            round_new = set(self.network.get_new_facts())
            round_new.difference_update(self.network.facts_set)
            if not round_new:
                break
            for f in round_new:
                self.network.add_fact(f)
                self.fb.add(f)
                all_new.append(f)
                self.steps.append({
                    'type': 'forward',
                    'new_fact': f,
                    'iteration': iteration
                })
            iteration += 1
        return list(dict.fromkeys(all_new))

    def backward_chain(self, goal, visited=None):
        """反向推理：递归 + 目标驱动"""
        if visited is None:
            visited = set()
        if goal in visited:
            self.steps.append({'type': 'backward', 'goal': goal, 'result': '循环依赖，跳过'})
            return False
        visited.add(goal)

        known = self.fb.__contains__(goal) if hasattr(self.fb, '__contains__') else goal in self.fb
        if known or goal in self.network.facts_set:
            self.steps.append({'type': 'backward', 'goal': goal, 'result': '已知事实'})
            return True

        candidate_rules = [r for r in self.rules if r.conclusion == goal]
        if not candidate_rules:
            self.steps.append({'type': 'backward', 'goal': goal, 'result': '无规则可推导', 'missing_facts': [goal]})
            return False

        proven_conditions = []
        unproven_conditions = []

        for rule in candidate_rules:
            rule_proven = []
            rule_unproven = []

            for cond in rule.conditions:
                known_cond = self.fb.__contains__(cond) if hasattr(self.fb, '__contains__') else cond in self.fb
                if known_cond or cond in self.network.facts_set:
                    rule_proven.append(cond)
                    self.steps.append({
                        'type': 'backward', 'goal': goal, 'rule': rule,
                        'sub_goal': cond, 'result': '已知事实'
                    })
                elif self.backward_chain(cond, visited):
                    rule_proven.append(cond)
                else:
                    rule_unproven.append(cond)

            if not rule_unproven:
                proven_conditions.extend(rule_proven)
                self.steps.append({
                    'type': 'backward', 'goal': goal,
                    'rule': rule,
                    'rule_id': rule.id,
                    'rule_conditions': rule.conditions,
                    'rule_conclusion': rule.conclusion,
                    'result': '通过规则推导成功',
                    'conditions': rule.conditions,
                    'proven_conditions': rule_proven,
                    'missing_facts': []
                })
                self.fb.add(goal)
                return True
            else:
                unproven_conditions.extend(rule_unproven)
                missing_for_rule = set()
                for c in rule_unproven:
                    self._collect_missing_leaves(c, missing_for_rule, set())
                self.steps.append({
                    'type': 'backward', 'goal': goal,
                    'rule': rule,
                    'rule_id': rule.id,
                    'rule_conditions': rule.conditions,
                    'rule_conclusion': rule.conclusion,
                    'result': '规则条件不满足',
                    'conditions': rule.conditions,
                    'proven_conditions': rule_proven,
                    'unproven_conditions': rule_unproven,
                    'missing_facts': list(missing_for_rule)
                })

        all_missing = set()
        for cond in unproven_conditions:
            self._collect_missing_leaves(cond, all_missing, set())
        self.steps.append({
            'type': 'backward', 'goal': goal,
            'result': '推理失败',
            'unproven_conditions': unproven_conditions,
            'missing_facts': list(all_missing)
        })
        return False

    def get_missing_facts_for_goal(self, goal):
        missing = set()
        self._collect_missing_leaves(goal, missing, set())
        return list(missing)

    def _collect_missing_leaves(self, goal, missing, visited):
        if goal in visited:
            return
        visited.add(goal)
        known = self.fb.__contains__(goal) if hasattr(self.fb, '__contains__') else goal in self.fb
        if known or goal in self.network.facts_set:
            return
        rules = [r for r in self.rules if r.conclusion == goal]
        if not rules:
            missing.add(goal)
            return
        for rule in rules:
            for cond in rule.conditions:
                self._collect_missing_leaves(cond, missing, visited)

    def get_network(self):
        return self.network
