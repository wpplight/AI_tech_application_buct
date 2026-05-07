import sqlite3
import os

DB_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'knowledge', 'rules.db')


class Rule:
    def __init__(self, id, conditions, conclusion):
        self.id = id
        self.conditions = conditions
        self.conclusion = conclusion

    def to_dict(self):
        return {'id': self.id, 'conditions': self.conditions, 'conclusion': self.conclusion}

    @staticmethod
    def from_dict(d):
        return Rule(d['id'], d['conditions'], d['conclusion'])

    def __repr__(self):
        return f"Rule {self.id}: IF {' AND '.join(self.conditions)} THEN {self.conclusion}"


class KnowledgeBase:
    def __init__(self):
        self.conn = sqlite3.connect(DB_PATH)
        self.conn.execute('''
            CREATE TABLE IF NOT EXISTS rules (
                id INTEGER PRIMARY KEY,
                conclusion TEXT NOT NULL
            )
        ''')
        self.conn.execute('''
            CREATE TABLE IF NOT EXISTS rule_conditions (
                rule_id INTEGER NOT NULL,
                condition TEXT NOT NULL,
                PRIMARY KEY (rule_id, condition)
            )
        ''')
        self.conn.execute('''
            CREATE INDEX IF NOT EXISTS idx_rules_conclusion ON rules(conclusion)
        ''')
        self.conn.execute('''
            CREATE INDEX IF NOT EXISTS idx_rule_conditions_condition ON rule_conditions(condition)
        ''')
        self.conn.execute('''
            CREATE TABLE IF NOT EXISTS condition_sets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                facts TEXT NOT NULL DEFAULT '[]',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        self.conn.execute('''
            CREATE INDEX IF NOT EXISTS idx_cs_name ON condition_sets(name)
        ''')
        self.conn.execute('PRAGMA foreign_keys = ON')
        self.conn.commit()
        self.conn.row_factory = sqlite3.Row
        self._build_condition_index()

    def _build_condition_index(self):
        self.condition_index = {}
        cursor = self.conn.execute('SELECT rule_id, condition FROM rule_conditions')
        for row in cursor.fetchall():
            cond = row['condition']
            if cond not in self.condition_index:
                self.condition_index[cond] = set()
            self.condition_index[cond].add(row['rule_id'])

    def _rule_from_db(self, rule_id):
        cursor = self.conn.execute('SELECT * FROM rules WHERE id = ?', (rule_id,))
        rule_row = cursor.fetchone()
        if not rule_row:
            return None
        cursor = self.conn.execute(
            'SELECT condition FROM rule_conditions WHERE rule_id = ? ORDER BY condition',
            (rule_id,)
        )
        conditions = [row['condition'] for row in cursor.fetchall()]
        return Rule(rule_row['id'], conditions, rule_row['conclusion'])

    def add_rule(self, conditions, conclusion):
        cursor = self.conn.execute('SELECT id FROM rules ORDER BY id')
        used_ids = {row[0] for row in cursor.fetchall()}
        new_id = 1
        while new_id in used_ids:
            new_id += 1
        self.conn.execute(
            'INSERT INTO rules (id, conclusion) VALUES (?, ?)',
            (new_id, conclusion)
        )
        for cond in conditions:
            self.conn.execute(
                'INSERT INTO rule_conditions (rule_id, condition) VALUES (?, ?)',
                (new_id, cond)
            )
            if cond not in self.condition_index:
                self.condition_index[cond] = set()
            self.condition_index[cond].add(new_id)
        self.conn.commit()
        return new_id

    def delete_rule(self, rule_id):
        cursor = self.conn.execute(
            'SELECT condition FROM rule_conditions WHERE rule_id = ?', (rule_id,)
        )
        for row in cursor.fetchall():
            cond = row['condition']
            if cond in self.condition_index:
                self.condition_index[cond].discard(rule_id)
                if not self.condition_index[cond]:
                    del self.condition_index[cond]
        self.conn.execute('DELETE FROM rule_conditions WHERE rule_id = ?', (rule_id,))
        self.conn.execute('DELETE FROM rules WHERE id = ?', (rule_id,))
        self.conn.commit()

    def modify_rule(self, rule_id, conditions=None, conclusion=None):
        if conclusion is not None:
            self.conn.execute(
                'UPDATE rules SET conclusion = ? WHERE id = ?',
                (conclusion, rule_id)
            )
        if conditions is not None:
            cursor = self.conn.execute(
                'SELECT condition FROM rule_conditions WHERE rule_id = ?', (rule_id,)
            )
            for row in cursor.fetchall():
                cond = row['condition']
                if cond in self.condition_index:
                    self.condition_index[cond].discard(rule_id)
                    if not self.condition_index[cond]:
                        del self.condition_index[cond]
            self.conn.execute('DELETE FROM rule_conditions WHERE rule_id = ?', (rule_id,))
            for cond in conditions:
                self.conn.execute(
                    'INSERT INTO rule_conditions (rule_id, condition) VALUES (?, ?)',
                    (rule_id, cond)
                )
                if cond not in self.condition_index:
                    self.condition_index[cond] = set()
                self.condition_index[cond].add(rule_id)
        self.conn.commit()
        return True

    def get_rules(self):
        cursor = self.conn.execute('SELECT id FROM rules ORDER BY id')
        return [self._rule_from_db(row['id']) for row in cursor.fetchall()]

    def get_rules_triggered_by(self, fact):
        rule_ids = self.condition_index.get(fact, set())
        if not rule_ids:
            return []
        placeholders = ','.join('?' * len(rule_ids))
        cursor = self.conn.execute(
            f'SELECT id FROM rules WHERE id IN ({placeholders}) ORDER BY id',
            tuple(rule_ids)
        )
        return [self._rule_from_db(row['id']) for row in cursor.fetchall()]

    def find_rules_with_conclusion(self, conclusion):
        cursor = self.conn.execute(
            'SELECT id FROM rules WHERE conclusion = ? ORDER BY id',
            (conclusion,)
        )
        return [self._rule_from_db(row['id']) for row in cursor.fetchall()]

    def find_rules_with_condition(self, condition):
        rule_ids = self.condition_index.get(condition, set())
        if not rule_ids:
            return []
        placeholders = ','.join('?' * len(rule_ids))
        cursor = self.conn.execute(
            f'SELECT id FROM rules WHERE id IN ({placeholders}) ORDER BY id',
            tuple(rule_ids)
        )
        return [self._rule_from_db(row['id']) for row in cursor.fetchall()]

    def find_duplicate(self, conditions, conclusion):
        conditions_set = set(conditions)
        for rule in self.find_rules_with_conclusion(conclusion):
            if set(rule.conditions) == conditions_set:
                return rule.id
        return None

    def is_empty(self):
        cursor = self.conn.execute('SELECT COUNT(*) FROM rules')
        return cursor.fetchone()[0] == 0

    def clear_all_rules(self):
        self.conn.execute('DELETE FROM rule_conditions')
        self.conn.execute('DELETE FROM rules')
        self.conn.commit()
        self.condition_index.clear()
