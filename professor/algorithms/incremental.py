"""
增量触发算法 —— 条件索引，只检查被新事实触发的规则
纯内存实现，无SQLite开销
"""


class IncrementalEngine:
    def __init__(self, rules):
        self.rules = [(conds, concl) for conds, concl in rules]
        self.condition_index = {}
        for i, (conditions, _) in enumerate(self.rules):
            for c in conditions:
                if c not in self.condition_index:
                    self.condition_index[c] = set()
                self.condition_index[c].add(i)

    def forward(self, input_facts):
        known = set(input_facts)
        new_facts = []
        triggered = set()
        checked = set()

        for f in input_facts:
            if f in self.condition_index:
                triggered.update(self.condition_index[f])

        while triggered:
            newly_deduced = []
            for rid in list(triggered):
                if rid in checked:
                    continue
                checked.add(rid)
                conditions, conclusion = self.rules[rid]
                if all(c in known for c in conditions) and conclusion not in known:
                    known.add(conclusion)
                    new_facts.append(conclusion)
                    newly_deduced.append(conclusion)

            if not newly_deduced:
                break

            next_triggered = set()
            for f in newly_deduced:
                if f in self.condition_index:
                    next_triggered.update(self.condition_index[f])
            triggered = next_triggered - checked

        return new_facts
