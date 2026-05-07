from knowledge_base import KnowledgeBase
from fact_base import FactBase


class InferenceEngine:
    def __init__(self, kb, fb):
        self.kb = kb
        self.fb = fb
        self.steps = []

    def reset_steps(self):
        self.steps = []

    def get_steps(self):
        return self.steps.copy()

    def forward_chain(self, max_iterations=100):
        iteration = 0
        new_facts = []

        rules = self.kb.get_rules()

        while iteration < max_iterations:
            added = False
            for rule in rules:
                if all(self.fb.contains(cond) for cond in rule.conditions):
                    conclusion = rule.conclusion
                    if not self.fb.contains(conclusion):
                        self.fb.add_fact(conclusion)
                        new_facts.append(conclusion)
                        added = True
                        self.steps.append({
                            'type': 'forward',
                            'rule': rule,
                            'new_fact': conclusion,
                            'iteration': iteration
                        })

            if not added:
                break
            iteration += 1

        return new_facts

    def backward_chain(self, goal, visited=None):
        if visited is None:
            visited = set()
        if goal in visited:
            self.steps.append({
                'type': 'backward',
                'goal': goal,
                'result': '循环依赖，跳过'
            })
            return False
        visited.add(goal)

        if self.fb.contains(goal):
            self.steps.append({
                'type': 'backward',
                'goal': goal,
                'result': '已知事实'
            })
            return True

        candidate_rules = self.kb.find_rules_with_conclusion(goal)
        if not candidate_rules:
            self.steps.append({
                'type': 'backward',
                'goal': goal,
                'result': '无规则可推导',
                'missing_facts': [goal]
            })
            return False

        proven_conditions = []
        unproven_conditions = []

        for rule in candidate_rules:
            rule_proven = []
            rule_unproven = []

            for cond in rule.conditions:
                if self.fb.contains(cond):
                    rule_proven.append(cond)
                    self.steps.append({
                        'type': 'backward',
                        'goal': goal,
                        'rule': rule,
                        'sub_goal': cond,
                        'result': '已知事实'
                    })
                elif self.backward_chain(cond, visited):
                    rule_proven.append(cond)
                else:
                    rule_unproven.append(cond)

            if not rule_unproven:
                proven_conditions.extend(rule_proven)
                self.steps.append({
                    'type': 'backward',
                    'goal': goal,
                    'rule': rule,
                    'result': '通过规则推导成功',
                    'conditions': rule.conditions,
                    'proven_conditions': rule_proven,
                    'missing_facts': []
                })
                self.fb.add_fact(goal)
                return True
            else:
                unproven_conditions.extend(rule_unproven)
                missing_for_rule = set()
                for c in rule_unproven:
                    self._collect_missing_leaves(c, missing_for_rule, set())
                self.steps.append({
                    'type': 'backward',
                    'goal': goal,
                    'rule': rule,
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
            'type': 'backward',
            'goal': goal,
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
        if goal in visited or self.fb.contains(goal):
            return
        visited.add(goal)
        rules = self.kb.find_rules_with_conclusion(goal)
        if not rules:
            missing.add(goal)
            return
        for rule in rules:
            for cond in rule.conditions:
                self._collect_missing_leaves(cond, missing, visited)

    def explain(self):
        return self.steps
