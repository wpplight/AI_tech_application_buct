class FactBase:
    def __init__(self):
        self.facts = set()  # set of strings

    def add_fact(self, fact):
        self.facts.add(fact)

    def remove_fact(self, fact):
        if fact in self.facts:
            self.facts.remove(fact)
            return True
        return False

    def contains(self, fact):
        return fact in self.facts

    def get_facts(self):
        return self.facts.copy()

    def add_facts(self, facts):
        for f in facts:
            self.facts.add(f)

    def clear(self):
        self.facts.clear()

    def __repr__(self):
        return f"FactBase({self.facts})"
