"""
全扫描算法 —— 每轮遍历所有规则直到不再产生新事实
"""


class FullScan:
    def __init__(self, rules):
        self.rules = [(conds, concl) for conds, concl in rules]

    def forward(self, input_facts):
        known = set(input_facts)
        new_facts = []

        while True:
            added = False
            for conditions, conclusion in self.rules:
                if all(c in known for c in conditions) and conclusion not in known:
                    known.add(conclusion)
                    new_facts.append(conclusion)
                    added = True
            if not added:
                break

        return new_facts
