"""
全扫 vs 增量触发 vs Rete 性能对比
使用 Wikipedia 生物分类知识库（610条真实规则），控制变量法
"""
import sys
import os
import time
import random
import gc

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from algorithms.fullscan import FullScan
from algorithms.incremental import IncrementalEngine
from algorithms.rete import ReteEngine
from knowledge.wikipedia_rules import WIKIPEDIA_RULES


def bench(engine_cls, rules, facts, warm=5, runs=30):
    for _ in range(warm):
        eng = engine_cls(rules)
        eng.forward(facts)

    times = []
    for _ in range(runs):
        eng = engine_cls(rules)
        t0 = time.perf_counter_ns()
        eng.forward(facts)
        t1 = time.perf_counter_ns()
        times.append((t1 - t0) / 1000)

    times.sort()
    trimmed = times[3:-3]
    return sum(trimmed) / len(trimmed)


random.seed(42)
all_facts = []
for c, _ in WIKIPEDIA_RULES:
    all_facts.extend(c)
all_facts = list(set(all_facts))

sizes = [20, 50, 100, 200, 300, 400, 500, 610]

print("=" * 70)
print("全扫 vs 增量触发 vs Rete  性能对比")
print(f"数据: {len(WIKIPEDIA_RULES)} 条 Wikipedia 生物分类规则")
print("控制变量: 同一规则集, 同一输入事实")
print("=" * 70)
print()

results = []

for n in sizes:
    rules = WIKIPEDIA_RULES[:n]
    facts = random.sample(all_facts, min(20, len(all_facts)))

    print(f"--- {n} 条规则 ---")

    gc.collect()
    t_fs = bench(FullScan, rules, facts)
    print(f"  全扫:      {t_fs:8.2f} μs")

    gc.collect()
    t_inc = bench(IncrementalEngine, rules, facts)
    print(f"  增量触发:  {t_inc:8.2f} μs")

    gc.collect()
    t_re = bench(ReteEngine, rules, facts)
    print(f"  Rete:     {t_re:8.2f} μs")

    fastest = min(t_fs, t_inc, t_re)
    name = "全扫" if t_fs == fastest else ("增量触发" if t_inc == fastest else "Rete")
    print(f"  最快: {name} ({(max(t_fs,t_inc,t_re)/fastest):.1f}x)")
    print()

    results.append((n, round(t_fs, 2), round(t_inc, 2), round(t_re, 2)))

print("=" * 70)
print("汇总")
print("=" * 70)
print(f"{'规则数':>6} | {'全扫(μs)':>10} | {'增量(μs)':>10} | {'Rete(μs)':>10} | {'最快':>8}")
print("-" * 65)
for n, fs, inc, re in results:
    fastest = min(fs, inc, re)
    name = "全扫" if fs == fastest else ("增量" if inc == fastest else "Rete")
    print(f"{n:>6} | {fs:>10.2f} | {inc:>10.2f} | {re:>10.2f} | {name:>8}")

print()
print("REAL_DATA = [")
for n, fs, inc, re in results:
    print(f"    ({n}, {fs}, {inc}, {re}),")
print("]")

print()
print("结论:")
fs_wins = sum(1 for _, fs, inc, re in results if fs < inc and fs < re)
inc_wins = sum(1 for _, fs, inc, re in results if inc < fs and inc < re)
re_wins = sum(1 for _, fs, inc, re in results if re < fs and re < inc)
print(f"  全扫胜: {fs_wins}")
print(f"  增量胜: {inc_wins}")
print(f"  Rete胜: {re_wins}")
