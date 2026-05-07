import json
import sys
import os
import time
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from rete_runner import Rule, ReteInferenceEngine
from knowledge_base import KnowledgeBase


kb = KnowledgeBase()
engine = ReteInferenceEngine()


def load_rules_from_db():
    for r in kb.get_rules():
        engine.add_rule(Rule(r.id, r.conditions, r.conclusion))


load_rules_from_db()


HTML_PAGE = """<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Rete 专家系统实验</title>
<style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body { font-family: 'Segoe UI', 'Microsoft YaHei', sans-serif; background: #0f172a; color: #e2e8f0; }
    .header { background: linear-gradient(135deg, #451a03 0%, #78350f 50%, #92400e 100%); padding: 0; position: sticky; top: 0; z-index: 100; box-shadow: 0 4px 30px rgba(0,0,0,0.3); }
    .header-top { display: flex; justify-content: space-between; align-items: center; padding: 16px 32px; }
    .header-left { display: flex; align-items: center; gap: 16px; }
    .header h1 { font-size: 22px; font-weight: 700; letter-spacing: -0.5px; }
    .algo-badge { display: inline-flex; align-items: center; gap: 6px; padding: 6px 16px; border-radius: 20px; font-size: 12px; font-weight: 600; letter-spacing: 0.5px; text-transform: uppercase; }
    .algo-badge.rete { background: linear-gradient(135deg, #f59e0b, #fbbf24); box-shadow: 0 2px 10px rgba(245,158,11,0.4); color: #1c1917; }
    .header-nav { display: flex; gap: 6px; padding: 0 32px 12px; }
    .header-nav a { color: rgba(255,255,255,0.6); text-decoration: none; font-size: 12px; padding: 4px 12px; border-radius: 6px; transition: all 0.2s; }
    .header-nav a:hover { color: white; background: rgba(255,255,255,0.1); }
    .header-nav a.active { color: white; background: rgba(255,255,255,0.15); }
    .header-right { display: flex; align-items: center; gap: 12px; }
    .port-tag { padding: 4px 12px; border-radius: 6px; font-size: 11px; font-weight: 600; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.15); }
    .badge { background: rgba(255,255,255,0.2); padding: 2px 10px; border-radius: 10px; font-size: 11px; }
    .container { display: grid; grid-template-columns: 320px 1fr 320px; gap: 12px; padding: 12px; height: calc(100vh - 60px); }
    .panel { background: #1e293b; border-radius: 10px; padding: 14px; overflow-y: auto; border: 1px solid #334155; }
    .panel-title { font-size: 14px; font-weight: 700; margin-bottom: 10px; color: #fbbf24; border-bottom: 1px solid #334155; padding-bottom: 8px; }
    .section { margin-bottom: 14px; }
    .section-title { font-size: 12px; color: #94a3b8; margin-bottom: 6px; font-weight: 600; }
    .rule-item { padding: 8px 10px; margin: 2px 0; background: #0f172a; border-radius: 6px; font-size: 12px; cursor: pointer; border-left: 3px solid #f59e0b; transition: all 0.15s; }
    .rule-item:hover { background: #2d3a50; border-left-color: #fbbf24; }
    .fact-item { display: inline-block; padding: 4px 10px; margin: 2px; background: #1e3a5f; border-radius: 12px; font-size: 12px; color: #93c5fd; cursor: pointer; }
    .fact-item:hover { background: #dc2626; color: white; }
    .step-item { padding: 8px 10px; margin: 4px 0; border-radius: 6px; font-size: 12px; border-left: 3px solid; }
    .btn { border: none; border-radius: 6px; padding: 8px 14px; cursor: pointer; font-size: 12px; font-weight: 600; transition: all 0.15s; }
    .btn-primary { background: #f59e0b; color: #1c1917; font-weight: 700; }
    .btn-primary:hover { background: #d97706; }
    .btn-danger { background: #dc2626; color: white; }
    .btn-danger:hover { background: #b91c1c; }
    .btn-outline { background: transparent; color: #fbbf24; border: 1px solid rgba(245,158,11,0.3); }
    .btn-outline:hover { background: rgba(245,158,11,0.1); border-color: rgba(245,158,11,0.5); }
    .btn-warning { background: #d97706; color: white; }
    .btn-warning:hover { background: #b45309; }
    .btn-sm { padding: 5px 10px; font-size: 11px; }
    .btn-block { display: block; width: 100%; margin-top: 6px; }
    .btn-success { background: #16a34a; color: white; }
    .btn-success:hover { background: #15803d; }
    .input-group input, .input-group textarea { width: 100%; padding: 8px; border: 1px solid #334155; border-radius: 6px; font-size: 12px; background: #0f172a; color: #e2e8f0; margin-top: 4px; }
    .input-group label { font-size: 11px; color: #94a3b8; }
    .scroll-area { max-height: calc(100vh - 150px); overflow-y: auto; }
    .scroll-area-tall { max-height: calc(100vh - 200px); overflow-y: auto; }
    .header-info { color: #94a3b8; font-size: 12px; display: flex; gap: 16px; margin-bottom: 4px; }
    .header-info span { display: flex; align-items: center; gap: 4px; }
    .badge-count { background: #334155; color: #fbbf24; padding: 1px 7px; border-radius: 8px; font-size: 11px; }
    .empty-hint { color: #64748b; font-size: 12px; text-align: center; padding: 20px; }
    .toast { position: fixed; top: 16px; right: 16px; padding: 10px 18px; border-radius: 8px; color: white; font-size: 13px; z-index: 9999; animation: slideIn 0.3s ease; }
    .toast-success { background: #16a34a; }
    .toast-error { background: #dc2626; }
    @keyframes slideIn { from { transform: translateX(100px); opacity: 0; } to { transform: translateX(0); opacity: 1; } }
    .modal-overlay { display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.6); z-index: 1000; justify-content: center; align-items: center; }
    .modal-overlay.show { display: flex; }
    .modal { background: #1e293b; padding: 20px; border-radius: 12px; min-width: 400px; max-width: 500px; border: 1px solid #334155; }
    .modal h3 { color: #fbbf24; margin-bottom: 14px; }
    .status-bar { font-size: 11px; color: #94a3b8; padding: 4px 0; border-top: 1px solid #334155; margin-top: 6px; }
    .highlight { color: #fbbf24; font-weight: 700; }
    .rule-link { color: #fbbf24; cursor: pointer; text-decoration: underline; }
    .rule-link:hover { color: #fcd34d; }
    @keyframes rule-flash { 0%,100% { background: #1e293b; } 20% { background: rgba(120,53,15,0.5); box-shadow: 0 0 12px #fbbf24; } 50% { background: rgba(146,64,14,0.6); } 80% { background: rgba(120,53,15,0.5); } }
    .rule-highlight { animation: rule-flash 1.2s ease-in-out 2; }
    .badge-rete { background: linear-gradient(135deg, #f59e0b, #fbbf24); color: #1c1917; padding: 1px 6px; border-radius: 6px; font-size: 10px; margin-left: 4px; font-weight: 700; }
    select { background: #0f172a; color: #e2e8f0; border: 1px solid #334155; border-radius: 6px; padding: 6px; font-size: 12px; }
</style>
</head>
<body>

<div class="header">
    <div class="header-top">
        <div class="header-left">
            <h1>🦉 动物识别专家系统</h1>
            <span class="algo-badge rete">⚡ Rete</span>
        </div>
        <div class="header-right">
            <button class="btn btn-outline btn-sm" onclick="showNetworkStats()">📊 Rete 网络</button>
            <span class="port-tag">PORT 8082</span>
        </div>
    </div>
    <div class="header-nav">
        <a href="http://localhost:8080">全扫描</a>
        <a href="http://localhost:8081">增量触发</a>
        <a href="http://localhost:8082" class="active">Rete</a>
    </div>
</div>

<div class="container">
    <!-- 左面板：规则列表 -->
    <div class="panel">
        <div class="panel-title">📋 知识库（Rete 正向推理引擎）</div>
        <div class="header-info"><span>规则数: <span class="badge-count" id="ruleCount">0</span></span><span>网络状态: <span class="badge-count" id="networkStatus">未编译</span></span></div>
        <div style="display:flex;gap:4px;margin-bottom:8px;">
            <button class="btn btn-primary btn-sm" onclick="showAddRuleModal()">+ 添加</button>
            <button class="btn btn-outline btn-sm" onclick="showDeleteRuleModal()">删除</button>
            <button class="btn btn-outline btn-sm" onclick="showModifyRuleModal()">修改</button>
        </div>
        <div class="scroll-area-tall" id="ruleList"></div>
    </div>

    <!-- 中间面板：推理结果 -->
    <div class="panel">
        <div class="panel-title">🔬 推理结果与步骤</div>
        <div class="scroll-area" id="resultsContent"><div class="empty-hint">点击上方按钮加载示例，或手动添加事实推理</div></div>
        <div class="section-title" style="margin-top:10px;">推理步骤</div>
        <div class="scroll-area" id="stepsContent"><div class="empty-hint">暂无推理步骤</div></div>
        <div class="status-bar" id="statusBar">就绪 — Rete 引擎已加载</div>
    </div>

    <!-- 右面板：事实库 + 控制 -->
    <div class="panel">
        <div class="panel-title">🧠 推理控制</div>

        <div style="display:flex;gap:4px;margin-bottom:8px;">
            <button class="btn btn-outline btn-sm" onclick="clearFactsOnly()">🗑 清空事实</button>
            <button class="btn btn-warning btn-sm" onclick="resetSystem()">🔄 重置推理</button>
        </div>

        <div class="section">
            <div class="section-title">▶ 正向推理 (Rete 网络)</div>
            <p style="font-size:11px;color:#94a3b8;margin-bottom:4px;">① 输入已知事实，② 点击执行推理</p>
            <div class="input-group" style="display:flex;gap:4px;">
                <input type="text" id="forwardFactInput" placeholder="事实，逗号分隔，如：有羽毛,会飞" onkeydown="if(event.key==='Enter')addForwardFact()" style="flex:1;">
                <button class="btn btn-primary btn-sm" onclick="addForwardFact()">添加</button>
            </div>
            <button class="btn btn-primary btn-block" onclick="runForward()">▶ 执行正向推理（Rete）</button>
        </div>

        <div class="section">
            <div class="section-title">◀ 反向推理</div>
            <div class="input-group" style="display:flex;gap:4px;">
                <input type="text" id="goalInput" placeholder="推理目标，如：虎" onkeydown="if(event.key==='Enter')runBackward()" style="flex:1;">
                <button class="btn btn-primary btn-sm" onclick="runBackward()">执行</button>
            </div>
        </div>

        <div class="section-title">📌 当前事实</div>
        <div style="display:flex;gap:4px;margin-bottom:6px;">
            <button class="btn btn-primary btn-sm" onclick="showAddFactModal()">+ 添加事实</button>
            <button class="btn btn-danger btn-sm" onclick="showDeleteFactModal()">删除事实</button>
        </div>
        <div id="factsContent" style="font-size:12px;max-height:180px;overflow-y:auto;"></div>
    </div>
</div>

<!-- 模态框 -->
<div class="modal-overlay" id="addRuleModal"><div class="modal">
    <h3>添加规则</h3>
    <div class="input-group"><label>条件（用逗号分隔，支持中英文）</label><input type="text" id="ruleConditions" placeholder="条件，多个用逗号隔开（支持中英文），如：有羽毛,会飞"></div>
    <div class="input-group"><label>结论</label><input type="text" id="ruleConclusion" placeholder="如：是鸟"></div>
    <button class="btn btn-outline" onclick="hideModal('addRuleModal')" style="margin-top:10px;">取消</button>
    <button class="btn btn-primary" onclick="addRule()" style="margin-top:10px;">添加</button>
</div></div>

<div class="modal-overlay" id="modifyRuleModal"><div class="modal">
    <h3>修改规则</h3>
    <label style="font-size:11px;color:#94a3b8;">选择规则</label>
    <select id="modifyRuleSelect" onchange="onModifyRuleSelected()"><option value="">-- 选择要修改的规则 --</option></select>
    <div class="input-group"><label>条件</label><input type="text" id="modifyRuleConditions" placeholder="条件，多个用逗号隔开（支持中英文），如：有毛发,有爪,吃肉"></div>
    <div class="input-group"><label>结论</label><input type="text" id="modifyRuleConclusion" placeholder="如：是食肉动物"></div>
    <button class="btn btn-outline" onclick="hideModal('modifyRuleModal')">取消</button>
    <button class="btn btn-primary" onclick="modifyRule()">修改</button>
</div></div>

<div class="modal-overlay" id="deleteRuleModal"><div class="modal">
    <h3>删除规则</h3>
    <input type="text" id="deleteRuleSearch" placeholder="搜索规则..." oninput="filterDeleteRuleList()" style="width:100%;padding:8px;border:1px solid #334155;border-radius:6px;font-size:12px;background:#0f172a;color:#e2e8f0;margin-bottom:8px;">
    <div id="deleteRuleList" class="scroll-area" style="max-height:300px;"></div>
    <button class="btn btn-outline" onclick="hideModal('deleteRuleModal')">取消</button>
</div></div>

<div class="modal-overlay" id="addFactModal"><div class="modal">
    <h3>添加事实</h3>
    <div class="input-group"><label>事实名称</label><input type="text" id="factInput" placeholder="如：有羽毛"></div>
    <button class="btn btn-outline" onclick="hideModal('addFactModal')">取消</button>
    <button class="btn btn-primary" onclick="addFact()">添加</button>
</div></div>

<div class="modal-overlay" id="deleteFactModal"><div class="modal">
    <h3>选择要删除的事实</h3>
    <div id="deleteFactList" class="scroll-area" style="max-height:300px;"></div>
    <button class="btn btn-outline" onclick="hideModal('deleteFactModal')">取消</button>
    <button class="btn btn-danger" onclick="deleteSelectedFact()">确认删除</button>
</div></div>

<div class="modal-overlay" id="networkStatsModal"><div class="modal">
    <h3>Rete 网络统计</h3>
    <div id="networkStatsContent"></div>
    <button class="btn btn-outline" onclick="hideModal('networkStatsModal')">关闭</button>
</div></div>

<script>
let modifyRuleId = null;
let selectedFact = null;

async function apiGet(path) {
    const res = await fetch('/api' + path);
    return res.json();
}

async function apiPost(path, body) {
    const res = await fetch('/api' + path, { method: 'POST', headers: {'Content-Type':'application/json'}, body: JSON.stringify(body) });
    const data = await res.json();
    if (!res.ok) throw new Error(data.error || '请求失败');
    return data;
}

function showToast(msg, type) {
    const t = document.createElement('div');
    t.className = 'toast toast-' + type;
    t.textContent = msg;
    document.body.appendChild(t);
    setTimeout(() => t.remove(), 2500);
}

function showModal(id) { document.getElementById(id).classList.add('show'); }
function hideModal(id) { document.getElementById(id).classList.remove('show'); }
function setStatus(msg) { document.getElementById('statusBar').textContent = msg; }
function appendResult(msg) {
    const el = document.getElementById('resultsContent');
    el.innerHTML += '<div style="padding:4px 0;border-bottom:1px solid #1e293b;font-size:12px;">' + msg + '</div>';
    el.scrollTop = el.scrollHeight;
}

async function loadRules() {
    const data = await apiGet('/rules');
    const el = document.getElementById('ruleList');
    document.getElementById('ruleCount').textContent = data.rules.length;
    if (data.rules.length === 0) { el.innerHTML = '<div class="empty-hint">知识库为空</div>'; return; }
    el.innerHTML = data.rules.map(r =>
        `<div class="rule-item" data-rule-id="${r.id}" onclick="highlightRule(${r.id})">Rule ${r.id}: IF ${r.conditions.join(' <span class="highlight">AND</span> ')} <span class="highlight">THEN</span> ${r.conclusion}</div>`
    ).join('');
}

async function loadFacts() {
    const data = await apiGet('/facts');
    document.getElementById('factsContent').innerHTML = data.facts.length === 0
        ? '<span style="color:#64748b;">暂无事实</span>'
        : data.facts.map(f => `<span class="fact-item" onclick="quickDeleteFact('${f}')">${f} ✕</span>`).join('');
    document.getElementById('deleteFactList').innerHTML = data.facts.length === 0
        ? '<div class="empty-hint">没有可删除的事实</div>'
        : data.facts.map(f => `<div class="rule-item" data-fact="${f}" onclick="selectFact(this,'${f}')">${f}</div>`).join('');
}

async function quickDeleteFact(fact) {
    const data = await apiPost('/facts/delete', { fact });
    if (data.success) { loadFacts(); showToast(`事实 "${fact}" 已删除`, 'success'); }
}

function selectFact(el, fact) {
    document.querySelectorAll('#deleteFactList .rule-item').forEach(i => i.style.background = '#0f172a');
    el.style.background = '#dc2626'; el.style.borderLeftColor = '#ef4444';
    selectedFact = fact;
}

async function deleteSelectedFact() {
    if (!selectedFact) { showToast('请选择要删除的事实', 'error'); return; }
    const data = await apiPost('/facts/delete', { fact: selectedFact });
    if (data.success) { showToast(`事实 "${selectedFact}" 已删除`, 'success'); hideModal('deleteFactModal'); selectedFact = null; loadFacts(); }
}

async function addFact() {
    const fact = document.getElementById('factInput').value.trim();
    if (!fact) { showToast('事实不能为空', 'error'); return; }
    const data = await apiPost('/facts/add', { fact });
    if (data.success) { showToast(`事实 "${fact}" 添加成功`, 'success'); hideModal('addFactModal'); loadFacts(); }
}

async function addForwardFact() {
    const input = document.getElementById('forwardFactInput');
    const raw = input.value.trim();
    if (!raw) { showToast('请输入事实', 'error'); return; }
    const facts = raw.replace(/，/g, ',').split(',').map(s => s.trim()).filter(Boolean);
    if (facts.length === 0) return;
    for (const fact of facts) await apiPost('/facts/add', { fact });
    showToast(`成功添加 ${facts.length} 个事实`, 'success');
    input.value = ''; input.focus();
    loadFacts();
}

async function showAddRuleModal() { showModal('addRuleModal'); }
async function addRule() {
    const conditions = document.getElementById('ruleConditions').value.trim();
    const conclusion = document.getElementById('ruleConclusion').value.trim();
    if (!conditions || !conclusion) { showToast('条件和结论不能为空', 'error'); return; }
    const condList = conditions.replace(/，/g, ',').split(',').map(s=>s.trim()).filter(Boolean);
    const rules = (await apiGet('/rules')).rules;
    const dup = rules.find(r => r.conclusion === conclusion && r.conditions.length === condList.length && r.conditions.every((c,i) => c === condList[i]));
    if (dup) { showToast(`规则已存在 (Rule ${dup.id})`, 'error'); return; }
    const data = await apiPost('/rules/add', { conditions: condList, conclusion });
    showToast(`规则添加成功 (ID: ${data.rule_id})`, 'success');
    hideModal('addRuleModal'); loadRules();
    appendResult(`规则添加成功: IF ${conditions} THEN ${conclusion}`);
}

async function showDeleteRuleModal() {
    showModal('deleteRuleModal');
    const data = await apiGet('/rules');
    document.getElementById('deleteRuleList').innerHTML = data.rules.map(r =>
        `<div class="rule-item" onclick="deleteRule(${r.id})" style="display:flex;justify-content:space-between;">Rule ${r.id}: IF ${r.conditions.join(' AND ')} THEN ${r.conclusion} <span style="color:#dc2626;">✕</span></div>`
    ).join('');
}

function filterDeleteRuleList() {
    const q = document.getElementById('deleteRuleSearch').value.toLowerCase();
    document.querySelectorAll('#deleteRuleList .rule-item').forEach(el => {
        el.style.display = el.textContent.toLowerCase().includes(q) ? '' : 'none';
    });
}

async function deleteRule(id) {
    await apiPost('/rules/delete', { rule_id: id });
    showToast(`Rule ${id} 已删除`, 'success');
    hideModal('deleteRuleModal'); loadRules();
}

async function showModifyRuleModal() {
    showModal('modifyRuleModal');
    const data = await apiGet('/rules');
    const sel = document.getElementById('modifyRuleSelect');
    sel.innerHTML = '<option value="">-- 选择要修改的规则 --</option>' + data.rules.map(r =>
        `<option value="${r.id}">Rule ${r.id}: IF ${r.conditions.join(' AND ')} THEN ${r.conclusion}</option>`
    ).join('');
}

function onModifyRuleSelected() {
    const id = parseInt(document.getElementById('modifyRuleSelect').value);
    if (!id) { modifyRuleId = null; return; }
    modifyRuleId = id;
    apiGet('/rules').then(data => {
        const r = data.rules.find(x => x.id === id);
        if (r) {
            document.getElementById('modifyRuleConditions').value = r.conditions.join(', ');
            document.getElementById('modifyRuleConclusion').value = r.conclusion;
        }
    });
}

async function modifyRule() {
    if (!modifyRuleId) { showToast('请先选择要修改的规则', 'error'); return; }
    const conditions = document.getElementById('modifyRuleConditions').value.trim();
    const conclusion = document.getElementById('modifyRuleConclusion').value.trim();
    if (!conditions || !conclusion) { showToast('条件和结论不能为空', 'error'); return; }
    const condList = conditions.replace(/，/g, ',').split(',').map(s=>s.trim()).filter(Boolean);
    const data = await apiPost('/rules/modify', { rule_id: modifyRuleId, conditions: condList, conclusion });
    if (data.success) { showToast(`Rule ${modifyRuleId} 修改成功`, 'success'); hideModal('modifyRuleModal'); modifyRuleId = null; loadRules(); }
}

async function runForward() {
    document.getElementById('resultsContent').innerHTML = '';
    appendResult('=== 正向推理（Rete 网络引擎）===');
    const facts = (await apiGet('/facts')).facts;
    appendResult(`已知事实: ${facts.length > 0 ? facts.join(', ') : '(无)'}`);
    const t0 = performance.now();
    const data = await apiPost('/inference/forward');
    const elapsed = (performance.now() - t0).toFixed(2);
    appendResult(`推导出的新结论: ${data.new_facts.length > 0 ? data.new_facts.join(', ') : '(无)'}`);
    appendResult(`⏱ Rete 推理耗时: ${elapsed} ms | 推理步骤: ${data.steps} 步`);

    // Rete 特有：网络传播追踪
    if (data.rete_trace && data.rete_trace.length > 0) {
        appendResult('');
        appendResult('━━━ 🔬 Rete 网络传播过程 ━━━');
        let nodeCounts = {alpha: 0, beta: 0, terminal: 0};
        for (const t of data.rete_trace) {
            if (t.type === 'alpha_activate') {
                nodeCounts.alpha++;
                appendResult(`<span style="color:#93c5fd;">🔵 Alpha激活</span>: 事实 <b>"${t.fact}"</b> → 进入 α(${t.condition}) → 下游 ${t.children} 个子节点`);
            } else if (t.type === 'beta_match') {
                nodeCounts.beta++;
                const facts_str = t.matched_facts.join(', ');
                if (t.is_chain_head) {
                    appendResult(`<span style="color:#fcd34d;">🟡 Beta链首匹配</span>: Rule ${t.rule_id}, α(${t.condition}) 命中 → [${facts_str}]`);
                } else {
                    appendResult(`<span style="color:#fcd34d;">🟡 Beta联合匹配</span>: Rule ${t.rule_id}, 联合 α(${t.condition}) → 累积匹配 [${facts_str}] ×${t.combined_count}`);
                }
            } else if (t.type === 'terminal_fire') {
                nodeCounts.terminal++;
                appendResult(`<span style="color:#4ade80;">🟢 Terminal触发!</span> Rule ${t.rule_id}: IF ${t.conditions.join(' AND ')} → <b>THEN ${t.conclusion}</b> ✅ (基于 [${t.matched_facts.join(', ')}])`);
            }
        }
        appendResult('');
        appendResult(`📊 Rete 节点统计: ${nodeCounts.alpha}α + ${nodeCounts.beta}β + ${nodeCounts.terminal}γ`);
    }

    setStatus(`正向推理完成 (Rete) — ${elapsed}ms, ${data.steps} 步`);
    loadFacts();
    loadSteps();
}

async function runBackward() {
    const goal = document.getElementById('goalInput').value.trim();
    if (!goal) { showToast('请输入推理目标', 'error'); return; }
    document.getElementById('resultsContent').innerHTML = '';
    appendResult(`=== 反向推理 → 目标: "${goal}" ===`);
    const data = await apiPost('/inference/backward', { goal });
    if (data.success) {
        appendResult(`✅ 推导成功！"${goal}" 成立`);
    } else {
        appendResult(`❌ 推导失败，缺少事实: ${data.missing_facts ? data.missing_facts.join(', ') : '无'}`);
    }
    setStatus(`反向推理完成 — ${data.success ? '成功' : '失败'}`);
    loadSteps();
}

async function loadSteps() {
    const data = await apiGet('/inference/steps');
    const el = document.getElementById('stepsContent');
    if (data.steps.length === 0) { el.innerHTML = '<div class="empty-hint">暂无推理步骤</div>'; return; }
    el.innerHTML = data.steps.map((s, i) => {
        let html = '', bg = '#0f172a', border = '#f59e0b';
        if (s.type === 'forward') {
            html = `<strong>步骤 ${i+1}</strong>: 正向推理 [迭代${s.iteration}] → 推导出: <span style="color:#4ade80;">"${s.new_fact}"</span>`;
            bg = '#052e16'; border = '#4ade80';
        } else if (s.type === 'backward') {
            const goal = s.goal || '';
            if (s.result === '已知事实') {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → "${goal}" ✅ 已知事实`;
                bg = '#052e16'; border = '#4ade80';
            } else if (s.result === '循环依赖，跳过') {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → "${goal}" ⚠ 循环依赖`;
                bg = '#422006'; border = '#d97706';
            } else if (s.result === '无规则可推导') {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → "${goal}" ❌ 无规则`;
                bg = '#450a0a'; border = '#dc2626';
            } else if (s.attempt) {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → "${goal}"，尝试 <span class="rule-link" onclick="highlightRule(${s.rule_id})">Rule ${s.rule_id}</span>: IF ${s.rule_conditions.join(' AND ')} THEN ${s.rule_conclusion}`;
                bg = 'rgba(120,53,15,0.3)'; border = '#fbbf24';
            } else if (s.result === '通过规则推导成功') {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → "${goal}" ✅ 通过 <span class="rule-link" onclick="highlightRule(${s.rule_id})">Rule ${s.rule_id}</span> 推导成功`;
                bg = '#052e16'; border = '#4ade80';
            } else {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → "${goal}" ❌ 不满足`;
                bg = '#450a0a'; border = '#dc2626';
            }
        }
        return `<div class="step-item" style="background:${bg};border-left-color:${border};">${html}</div>`;
    }).join('');
}

function highlightRule(ruleId) {
    const el = document.querySelector(`.rule-item[data-rule-id="${ruleId}"]`);
    if (!el) return;
    el.classList.remove('rule-highlight');
    void el.offsetWidth;
    el.classList.add('rule-highlight');
    el.scrollIntoView({ behavior: 'smooth', block: 'center' });
}

async function loadExample(type) {
    await apiPost('/reset');
    const data = await apiPost('/example', { type });
    if (data.success) {
        loadRules(); loadFacts();
        document.getElementById('resultsContent').innerHTML = '';
        appendResult(`=== ${data.label} ===`);
        appendResult(`已添加事实: ${data.facts.join(', ')}`);
        appendResult('点击"执行正向推理"查看结果');
        setStatus(`${data.label}已加载`);
        showToast(`${data.label}已加载`, 'success');
    }
}

async function clearFactsOnly() {
    await apiPost('/facts/clear');
    loadFacts();
    document.getElementById('resultsContent').innerHTML = '';
    setStatus('事实已清空');
    showToast('事实已清空', 'success');
}

async function resetSystem() {
    await apiPost('/reset');
    loadFacts();
    document.getElementById('resultsContent').innerHTML = '';
    document.getElementById('stepsContent').innerHTML = '';
    setStatus('推理已重置，规则保持不变');
    showToast('推理已重置', 'success');
}

async function showNetworkStats() {
    const data = await apiGet('/network/stats');
    document.getElementById('networkStatsContent').innerHTML = `
        <table style="width:100%;border-collapse:collapse;font-size:12px;">
            <tr><td style="padding:6px;color:#94a3b8;">Alpha 节点</td><td>${data.alpha_nodes}</td></tr>
            <tr><td style="padding:6px;color:#94a3b8;">Beta 节点</td><td>${data.beta_nodes}</td></tr>
            <tr><td style="padding:6px;color:#94a3b8;">Terminal 节点</td><td>${data.terminals}</td></tr>
            <tr><td style="padding:6px;color:#94a3b8;">Alpha Memory</td><td>${data.alpha_memory_size} 条</td></tr>
            <tr><td style="padding:6px;color:#94a3b8;">Beta Memory</td><td>${data.beta_memory_size} 条</td></tr>
            <tr><td style="padding:6px;color:#94a3b8;">编译耗时</td><td>${data.build_time_us.toFixed(2)} μs</td></tr>
        </table>
    `;
    showModal('networkStatsModal');
}

function showAddFactModal() { showModal('addFactModal'); document.getElementById('factInput').focus(); }
function showDeleteFactModal() { showModal('deleteFactModal'); loadFacts(); }

loadRules();
loadFacts();
apiGet('/network/stats').then(d => {
    document.getElementById('networkStatus').textContent = d.build_time_us > 0 ? `已编译 (${d.build_time_us.toFixed(0)}μs)` : '未编译';
});
</script>
</body>
</html>"""


class Handler(BaseHTTPRequestHandler):
    def log_message(self, format, *args): pass

    def _send_json(self, data, status=200):
        self.send_response(status)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()
        self.wfile.write(json.dumps(data, ensure_ascii=False).encode('utf-8'))

    def _send_html(self, html, head_only=False):
        encoded = html.encode('utf-8')
        self.send_response(200)
        self.send_header('Content-Type', 'text/html; charset=utf-8')
        self.send_header('Content-Length', str(len(encoded)))
        self.end_headers()
        if not head_only:
            self.wfile.write(encoded)

    def do_HEAD(self):
        parsed = urlparse(self.path)
        if parsed.path == '/':
            self._send_html(HTML_PAGE, head_only=True)
        else:
            self.send_response(200)
            self.send_header('Content-Type', 'application/json; charset=utf-8')
            self.end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()

    def _read_body(self):
        length = int(self.headers.get('Content-Length', 0))
        if length > 0:
            return json.loads(self.rfile.read(length).decode('utf-8'))
        return {}

    def do_GET(self):
        parsed = urlparse(self.path)
        path = parsed.path

        if path == '/':
            self._send_html(HTML_PAGE)
        elif path == '/api/rules':
            rules = []
            for r in kb.get_rules():
                rules.append({'id': r.id, 'conditions': r.conditions, 'conclusion': r.conclusion})
            self._send_json({'rules': rules})
        elif path == '/api/facts':
            facts = list(engine.fb) + list(engine.network.facts_set)
            self._send_json({'facts': list(dict.fromkeys(facts))})
        elif path == '/api/inference/steps':
            step_list = []
            for s in engine.get_steps():
                item = {'type': s['type']}
                if s['type'] == 'forward':
                    item['new_fact'] = s.get('new_fact', '')
                    item['iteration'] = s.get('iteration', 0)
                elif s['type'] == 'backward':
                    for k in ['goal', 'result', 'attempt', 'rule_id', 'rule_conditions', 'rule_conclusion', 'conditions']:
                        if k in s:
                            item[k] = s[k]
                step_list.append(item)
            self._send_json({'steps': step_list})
        elif path == '/api/network/stats':
            if engine.built:
                stats = engine.network.get_network_stats()
            else:
                engine.build_network()
                stats = engine.network.get_network_stats()
            self._send_json(stats)
        elif path == '/api/network/trace':
            trace_data = []
            for t in engine.network.trace:
                entry = dict(t)
                if 'matched_facts' in entry:
                    entry['matched_facts'] = [str(f) for f in entry['matched_facts']]
                trace_data.append(entry)
            self._send_json({'trace': trace_data})
        else:
            self._send_json({'error': 'Not Found'}, 404)

    def do_POST(self):
        parsed = urlparse(self.path)
        path = parsed.path
        body = self._read_body()

        if path == '/api/rules/add':
            conditions = body.get('conditions', [])
            conclusion = body.get('conclusion', '')
            if conditions and conclusion:
                duplicate_id = kb.find_duplicate(conditions, conclusion)
                if duplicate_id:
                    self._send_json({'success': False, 'error': f'规则已存在 (Rule {duplicate_id})'}, 400)
                else:
                    rid = kb.add_rule(conditions, conclusion)
                    engine.rules.append(Rule(rid, conditions, conclusion))
                    engine.built = False
                    self._send_json({'success': True, 'rule_id': rid})

        elif path == '/api/rules/delete':
            rule_id = body.get('rule_id')
            if rule_id:
                kb.delete_rule(rule_id)
                engine.rules = [r for r in engine.rules if r.id != rule_id]
                engine.built = False
                self._send_json({'success': True})

        elif path == '/api/rules/modify':
            rule_id = body.get('rule_id')
            conditions = body.get('conditions')
            conclusion = body.get('conclusion')
            if rule_id and (conditions or conclusion):
                kb.modify_rule(rule_id, conditions, conclusion)
                for r in engine.rules:
                    if r.id == rule_id:
                        if conditions is not None:
                            r.conditions = conditions
                        if conclusion is not None:
                            r.conclusion = conclusion
                        break
                engine.built = False
                self._send_json({'success': True})

        elif path == '/api/facts/add':
            fact = body.get('fact', '')
            if fact:
                engine.add_fact(fact)
                self._send_json({'success': True})

        elif path == '/api/facts/delete':
            fact = body.get('fact', '')
            if fact:
                engine.fb.discard(fact)
                engine.network.facts_set.discard(fact)
                self._send_json({'success': True})

        elif path == '/api/facts/clear':
            engine.fb.clear()
            engine.network.facts_set.clear()
            if engine.built:
                for alpha in engine.network.alpha_nodes.values():
                    alpha.memory.clear()
                for beta in engine.network.beta_nodes:
                    beta.completed.clear()
                    beta.pending_left.clear()
                for t in engine.network.terminals:
                    t.results.clear()
            self._send_json({'success': True})

        elif path == '/api/inference/forward':
            engine.reset_steps()
            engine.build_network()
            # 先建空网络，再注入事实
            engine.network.facts_set.clear()
            for alpha in engine.network.alpha_nodes.values():
                alpha.memory.clear()
            for beta in engine.network.beta_nodes:
                beta.completed.clear()
                beta.pending_left.clear()
            for t in engine.network.terminals:
                t.results.clear()
            facts_list = list(engine.fb)
            engine.network.facts_set.update(facts_list)
            for f in facts_list:
                engine.network.add_fact(f)
            new_facts = engine.forward_chain()
            steps = engine.get_steps()
            trace = []
            for t in engine.network.trace:
                entry = dict(t)
                if 'matched_facts' in entry:
                    entry['matched_facts'] = [str(f) for f in entry['matched_facts']]
                trace.append(entry)
            self._send_json({
                'success': True,
                'new_facts': new_facts,
                'steps': len(steps),
                'rete_trace': trace
            })

        elif path == '/api/inference/backward':
            goal = body.get('goal', '')
            if goal:
                engine.reset_steps()
                engine.build_network()
                success = engine.backward_chain(goal)
                steps = engine.get_steps()
                missing = engine.get_missing_facts_for_goal(goal)
                self._send_json({
                    'success': success,
                    'goal': goal,
                    'steps': len(steps),
                    'missing_facts': missing
                })

        elif path == '/api/reset':
            engine.fb.clear()
            engine.network.facts_set.clear()
            if engine.built:
                for alpha in engine.network.alpha_nodes.values():
                    alpha.memory.clear()
                for beta in engine.network.beta_nodes:
                    beta.completed.clear()
                    beta.pending_left.clear()
                for t in engine.network.terminals:
                    t.results.clear()
            engine.reset_steps()
            self._send_json({'success': True})

        elif path == '/api/example':
            ex = body.get('type', '')
            examples = {
                'ostrich': (['有脊椎', '有羽毛', '无龙骨突', '体型大', '长颈', '黑白羽色'], '鸵鸟识别示例'),
                'tiger': (['有脊椎', '有乳腺', '犬齿发达', '食肉', '独居', '趾行性', '黄褐色', '有黑色横纹'], '虎识别示例'),
                'leopard': (['有脊椎', '有乳腺', '犬齿发达', '食肉', '独居', '趾行性', '黄褐色', '有暗色斑纹'], '豹识别示例'),
            }
            if ex in examples:
                facts, label = examples[ex]
                engine.fb.clear()
                engine.network.facts_set.clear()
                for f in facts:
                    engine.add_fact(f)
                self._send_json({'success': True, 'label': label, 'facts': facts})

        else:
            self._send_json({'error': 'Not Found'}, 404)


def main():
    port = 8082
    server = HTTPServer(('0.0.0.0', port), Handler)
    print(f"Rete 专家系统实验")
    print(f"请在浏览器中打开: http://localhost:{port}")
    print(f"按 Ctrl+C 停止服务器")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n服务器已停止")
        server.server_close()


if __name__ == '__main__':
    main()
