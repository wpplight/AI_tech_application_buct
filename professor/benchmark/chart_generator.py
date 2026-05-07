"""
全扫 vs 增量触发 vs Rete 图表生成器
"""
import os
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np

CHARTS_DIR = os.path.join(os.path.dirname(__file__), 'charts')
os.makedirs(CHARTS_DIR, exist_ok=True)

DATA = [
    (20, 9.01, 2.25, 24.09),
    (50, 23.27, 12.49, 67.50),
    (100, 47.34, 4.55, 68.89),
    (200, 54.58, 8.68, 76.30),
    (300, 138.05, 13.80, 142.48),
    (400, 73.83, 8.19, 123.16),
    (500, 77.48, 8.84, 122.44),
    (610, 92.29, 13.43, 149.66),
]

sizes = [d[0] for d in DATA]
fs_times = [d[1] for d in DATA]
inc_times = [d[2] for d in DATA]
rete_times = [d[3] for d in DATA]

plt.rcParams['font.family'] = ['DejaVu Sans']

def save(name):
    return os.path.join(CHARTS_DIR, name)

# --- 折线图 ---
fig, ax = plt.subplots(figsize=(10, 6))
ax.plot(sizes, fs_times, 'o-', color='#ef4444', linewidth=2, markersize=6, label='FullScan')
ax.plot(sizes, inc_times, 's-', color='#22c55e', linewidth=3, markersize=7, label='Incremental')
ax.plot(sizes, rete_times, 'D-', color='#7c3aed', linewidth=2, markersize=6, label='Rete')
ax.set_xlabel('Rules', fontsize=12)
ax.set_ylabel('Time (us)', fontsize=12)
ax.set_title('FullScan vs Incremental vs Rete (Python)', fontsize=14, fontweight='bold')
ax.legend(fontsize=11)
ax.grid(True, alpha=0.3)
ax.set_facecolor('#f8fafc')
fig.patch.set_facecolor('white')
fig.tight_layout()
fig.savefig(save('line.png'), dpi=150, bbox_inches='tight')
plt.close()

# --- 柱状图 ---
x = np.arange(len(sizes))
width = 0.25
fig, ax = plt.subplots(figsize=(11, 6))
ax.bar(x - width, fs_times, width, color='#ef4444', label='FullScan')
ax.bar(x, inc_times, width, color='#22c55e', label='Incremental')
ax.bar(x + width, rete_times, width, color='#7c3aed', label='Rete')
ax.set_xticks(x)
ax.set_xticklabels([str(s) for s in sizes])
ax.set_xlabel('Rules', fontsize=12)
ax.set_ylabel('Time (us)', fontsize=12)
ax.set_title('FullScan vs Incremental vs Rete (Python)', fontsize=14, fontweight='bold')
ax.legend(fontsize=11)
ax.grid(True, alpha=0.3, axis='y')
ax.set_facecolor('#f8fafc')
fig.patch.set_facecolor('white')
fig.tight_layout()
fig.savefig(save('bar.png'), dpi=150, bbox_inches='tight')
plt.close()

# --- 汇总大图 ---
fig = plt.figure(figsize=(12, 10))
gs = fig.add_gridspec(2, 1, height_ratios=[2, 1], hspace=0.35)

ax1 = fig.add_subplot(gs[0])
ax1.plot(sizes, fs_times, 'o-', color='#ef4444', linewidth=2, markersize=6, label='FullScan')
ax1.plot(sizes, inc_times, 's-', color='#22c55e', linewidth=3, markersize=7, label='Incremental')
ax1.plot(sizes, rete_times, 'D-', color='#7c3aed', linewidth=2, markersize=6, label='Rete')
ax1.set_xlabel('Rules', fontsize=12)
ax1.set_ylabel('Time (us)', fontsize=12)
ax1.set_title('FullScan vs Incremental vs Rete\n610 Wikipedia Taxonomy Rules', fontsize=14, fontweight='bold')
ax1.legend(fontsize=11, loc='upper left')
ax1.grid(True, alpha=0.3)
ax1.set_facecolor('#f8fafc')

ax2 = fig.add_subplot(gs[1])
ax2.axis('off')

cell_text = []
for n, fs, inc, re in DATA:
    fastest = min(fs, inc, re)
    name = 'FullScan' if fs == fastest else ('Incremental' if inc == fastest else 'Rete')
    cell_text.append([str(n), f'{fs:.1f}', f'{inc:.1f}', f'{re:.1f}', name])

cols = ['Rules', 'FullScan(us)', 'Incremental(us)', 'Rete(us)', 'Fastest']
table = ax2.table(cellText=cell_text, colLabels=cols, cellLoc='center', loc='center')
table.auto_set_font_size(False)
table.set_fontsize(10)
table.scale(1, 1.6)
for (row, col), cell in table.get_celld().items():
    if row == 0:
        cell.set_facecolor('#1e293b')
        cell.set_text_props(color='white', fontweight='bold')
    elif col == 0:
        cell.set_facecolor('#f1f5f9')
    elif col == 4:
        cell.set_facecolor('#dcfce7')
        cell.set_text_props(color='#166534', fontweight='bold')

fig.patch.set_facecolor('white')
fig.tight_layout()
fig.savefig(save('summary.png'), dpi=150, bbox_inches='tight')
plt.close()

print("Charts saved:")
for f in ['line.png', 'bar.png', 'summary.png']:
    print(f"  {save(f)}")
