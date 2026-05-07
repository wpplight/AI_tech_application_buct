import json
import sys
import os
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
from knowledge.wikipedia_rules import WIKIPEDIA_RULES

from knowledge_base import KnowledgeBase
from fact_base import FactBase
from inference_engine import InferenceEngine


kb = KnowledgeBase()
fb = FactBase()
engine = InferenceEngine(kb, fb)


def load_wikipedia_rules():
    for conditions, conclusion in WIKIPEDIA_RULES:
        kb.add_rule(conditions, conclusion)


if kb.is_empty():
    load_wikipedia_rules()


HTML_PAGE = """<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>动物识别专家系统</title>
<style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body { font-family: 'Inter', 'Segoe UI', 'Microsoft YaHei', sans-serif; background: #0f172a; color: #e2e8f0; min-height: 100vh; }
    .header { background: linear-gradient(135deg, #1e1b4b 0%, #312e81 50%, #4c1d95 100%); padding: 0; position: sticky; top: 0; z-index: 100; box-shadow: 0 4px 30px rgba(0,0,0,0.3); }
    .header-top { display: flex; justify-content: space-between; align-items: center; padding: 16px 32px; }
    .header-left { display: flex; align-items: center; gap: 16px; }
    .header h1 { font-size: 22px; font-weight: 700; letter-spacing: -0.5px; }
    .algo-badge { display: inline-flex; align-items: center; gap: 6px; padding: 6px 16px; border-radius: 20px; font-size: 12px; font-weight: 600; letter-spacing: 0.5px; text-transform: uppercase; }
    .algo-badge.fullscan { background: linear-gradient(135deg, #7c3aed, #a78bfa); box-shadow: 0 2px 10px rgba(124,58,237,0.4); }
    .header-nav { display: flex; gap: 6px; padding: 0 32px 12px; }
    .header-nav a { color: rgba(255,255,255,0.6); text-decoration: none; font-size: 12px; padding: 4px 12px; border-radius: 6px; transition: all 0.2s; }
    .header-nav a:hover { color: white; background: rgba(255,255,255,0.1); }
    .header-nav a.active { color: white; background: rgba(255,255,255,0.15); }
    .header-right { display: flex; align-items: center; gap: 12px; }
    .port-tag { padding: 4px 12px; border-radius: 6px; font-size: 11px; font-weight: 600; background: rgba(255,255,255,0.1); border: 1px solid rgba(255,255,255,0.15); }
    .container { display: flex; gap: 16px; padding: 16px; max-width: 1600px; margin: 0 auto; }
    .panel { background: rgba(30,41,59,0.8); backdrop-filter: blur(20px); border: 1px solid rgba(255,255,255,0.08); border-radius: 16px; padding: 20px; box-shadow: 0 8px 32px rgba(0,0,0,0.2); }
    .panel-title { font-size: 15px; font-weight: 600; margin-bottom: 16px; padding-bottom: 12px; border-bottom: 1px solid rgba(255,255,255,0.08); display: flex; justify-content: space-between; align-items: center; color: #f1f5f9; }
    .left { flex: 1; min-width: 0; }
    .center { flex: 0 0 300px; }
    .right { flex: 1.2; min-width: 0; }
    .rule-item { padding: 10px 12px; margin-bottom: 6px; background: rgba(124,58,237,0.08); border-left: 3px solid #7c3aed; border-radius: 8px; font-size: 13px; cursor: pointer; transition: all 0.2s; color: #cbd5e1; }
    .rule-item:hover { background: rgba(124,58,237,0.15); transform: translateX(2px); }
    .fact-item { padding: 8px 12px; margin-bottom: 5px; background: rgba(34,197,94,0.08); border-left: 3px solid #22c55e; border-radius: 8px; font-size: 13px; color: #cbd5e1; }
    .step-item { padding: 10px 12px; margin-bottom: 6px; background: rgba(249,115,22,0.08); border-left: 3px solid #f97316; border-radius: 8px; font-size: 13px; color: #cbd5e1; }
    @keyframes rule-flash { 0%,100% { background: rgba(124,58,237,0.08); } 20% { background: rgba(250,204,21,0.15); box-shadow: 0 0 15px rgba(250,204,21,0.2); } 50% { background: rgba(250,204,21,0.2); box-shadow: 0 0 25px rgba(250,204,21,0.3); } 80% { background: rgba(250,204,21,0.15); } }
    .rule-highlight { animation: rule-flash 1.2s ease-in-out 2; }
    .rule-link { color: #a78bfa; cursor: pointer; text-decoration: underline; font-weight: 600; }
    .rule-link:hover { color: #c4b5fd; }
    .btn { padding: 8px 16px; border: none; border-radius: 8px; cursor: pointer; font-size: 13px; font-weight: 500; transition: all 0.2s; display: inline-flex; align-items: center; gap: 5px; }
    .btn:hover { transform: translateY(-1px); box-shadow: 0 4px 15px rgba(0,0,0,0.3); }
    .btn-primary { background: linear-gradient(135deg, #7c3aed, #6d28d9); color: white; }
    .btn-primary:hover { background: linear-gradient(135deg, #8b5cf6, #7c3aed); }
    .btn-success { background: linear-gradient(135deg, #22c55e, #16a34a); color: white; }
    .btn-success:hover { background: linear-gradient(135deg, #4ade80, #22c55e); }
    .btn-danger { background: linear-gradient(135deg, #ef4444, #dc2626); color: white; }
    .btn-danger:hover { background: linear-gradient(135deg, #f87171, #ef4444); }
    .btn-warning { background: linear-gradient(135deg, #f97316, #ea580c); color: white; }
    .btn-warning:hover { background: linear-gradient(135deg, #fb923c, #f97316); }
    .btn-outline { background: transparent; color: #a78bfa; border: 1px solid rgba(167,139,250,0.3); }
    .btn-outline:hover { background: rgba(124,58,237,0.1); border-color: rgba(167,139,250,0.5); }
    .btn-sm { padding: 5px 10px; font-size: 12px; }
    .btn-block { width: 100%; justify-content: center; margin-bottom: 8px; }
    .scroll-area { max-height: 350px; overflow-y: auto; margin-bottom: 10px; }
    .scroll-area-tall { max-height: 500px; overflow-y: auto; }
    .scroll-area::-webkit-scrollbar, .scroll-area-tall::-webkit-scrollbar { width: 6px; }
    .scroll-area::-webkit-scrollbar-track, .scroll-area-tall::-webkit-scrollbar-track { background: transparent; }
    .scroll-area::-webkit-scrollbar-thumb, .scroll-area-tall::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.1); border-radius: 3px; }
    .input-group { display: flex; gap: 8px; margin-bottom: 10px; }
    .input-group input { flex: 1; padding: 10px 14px; border: 1px solid rgba(255,255,255,0.1); border-radius: 8px; font-size: 13px; outline: none; background: rgba(15,23,42,0.6); color: #e2e8f0; transition: all 0.2s; }
    .input-group input:focus { border-color: #7c3aed; box-shadow: 0 0 0 3px rgba(124,58,237,0.15); }
    .input-group input::placeholder { color: #64748b; }
    .status-bar { background: rgba(15,23,42,0.8); padding: 10px 20px; font-size: 13px; color: #94a3b8; border-top: 1px solid rgba(255,255,255,0.05); }
    .empty-hint { color: #64748b; text-align: center; padding: 20px; font-size: 13px; }
    .badge { display: inline-block; padding: 3px 10px; border-radius: 10px; font-size: 11px; font-weight: 600; }
    .badge-blue { background: rgba(124,58,237,0.15); color: #a78bfa; }
    .badge-green { background: rgba(34,197,94,0.15); color: #4ade80; }
    .badge-orange { background: rgba(249,115,22,0.15); color: #fb923c; }
    .section { margin-bottom: 20px; }
    .section-title { font-size: 14px; font-weight: 600; margin-bottom: 10px; color: #94a3b8; text-transform: uppercase; letter-spacing: 0.5px; font-size: 12px; }
    .fact-count { font-size: 12px; color: #64748b; }
    .goal-input { display: flex; gap: 8px; margin: 8px 0; }
    .goal-input input { flex: 1; padding: 10px 14px; border: 1px solid rgba(255,255,255,0.1); border-radius: 8px; font-size: 13px; outline: none; background: rgba(15,23,42,0.6); color: #e2e8f0; }
    .goal-input input:focus { border-color: #7c3aed; }
    .goal-input input::placeholder { color: #64748b; }
    .modal-overlay { display: none; position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.6); backdrop-filter: blur(4px); z-index: 1000; justify-content: center; align-items: center; }
    .modal-overlay.active { display: flex; }
    .modal { background: #1e293b; border: 1px solid rgba(255,255,255,0.1); border-radius: 16px; padding: 28px; width: 440px; max-width: 90%; box-shadow: 0 25px 60px rgba(0,0,0,0.5); }
    .modal h3 { margin-bottom: 18px; color: #f1f5f9; font-size: 18px; }
    .modal .input-group { margin-bottom: 14px; flex-direction: column; }
    .modal .input-group label { font-size: 13px; font-weight: 500; margin-bottom: 6px; color: #94a3b8; }
    .modal .input-group input { width: 100%; }
    .modal .modal-buttons { display: flex; gap: 8px; justify-content: flex-end; margin-top: 18px; }
    .toast { position: fixed; bottom: 30px; right: 30px; background: #1e293b; color: white; padding: 14px 24px; border-radius: 10px; font-size: 13px; z-index: 2000; opacity: 0; transition: all 0.3s; pointer-events: none; border: 1px solid rgba(255,255,255,0.1); box-shadow: 0 10px 30px rgba(0,0,0,0.3); }
    .toast.show { opacity: 1; transform: translateY(-4px); }
    .toast.success { border-color: rgba(34,197,94,0.3); background: linear-gradient(135deg, #064e3b, #1e293b); }
    .toast.error { border-color: rgba(239,68,68,0.3); background: linear-gradient(135deg, #7f1d1d, #1e293b); }
    .tabs { display: flex; gap: 4px; margin-bottom: 14px; background: rgba(15,23,42,0.5); padding: 4px; border-radius: 10px; }
    .tab { padding: 8px 16px; border-radius: 8px; cursor: pointer; font-size: 13px; background: transparent; color: #64748b; border: none; font-weight: 500; transition: all 0.2s; }
    .tab.active { background: rgba(124,58,237,0.2); color: #a78bfa; }
    .tab:hover { color: #e2e8f0; }
    .tab-content { display: none; }
    .tab-content.active { display: block; }
    .highlight { color: #a78bfa; font-weight: 600; }
    @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
    .panel { animation: fadeIn 0.4s ease-out; }
    .center .section { background: rgba(15,23,42,0.4); border-radius: 12px; padding: 16px; border: 1px solid rgba(255,255,255,0.05); }
</style>
</head>
<body>

<div class="header">
    <div class="header-top">
        <div class="header-left">
            <h1>🦁 动物识别专家系统</h1>
            <span class="algo-badge fullscan">⚡ Full Scan</span>
        </div>
        <div class="header-right">
            <span class="port-tag">PORT 8080</span>
        </div>
    </div>
    <div class="header-nav">
        <a href="http://localhost:8080" class="active">全扫描</a>
        <a href="http://localhost:8081">增量触发</a>
        <a href="http://localhost:8082">Rete</a>
    </div>
</div>

<div class="container">
    <div class="panel left">
        <div class="panel-title">
            📚 知识库
            <div style="display:flex;gap:4px;flex-wrap:wrap;justify-content:flex-end;">
                <button class="btn btn-outline btn-sm" onclick="showDeleteRuleModal()">删除</button>
                <button class="btn btn-outline btn-sm" onclick="showModifyRuleModal()">修改</button>
                <button class="btn btn-primary btn-sm" onclick="showAddRuleModal()">+ 添加</button>
            </div>
        </div>
        <div class="scroll-area-tall" id="ruleList"></div>
    </div>

    <div class="panel center">
        <div class="panel-title">🧠 推理控制</div>

        <div style="display:flex;gap:4px;margin-bottom:8px;">
            <button class="btn btn-outline btn-sm" onclick="clearFactsOnly()">🗑 清空事实</button>
            <button class="btn btn-warning btn-sm" onclick="resetSystem()">🔄 重置推理</button>
        </div>

        <div class="section">
            <div class="section-title">▶ 正向推理</div>
            <p style="font-size:12px;color:#888;margin-bottom:6px;">① 输入已知事实，② 点击执行推理</p>
            <div class="input-group">
                <input type="text" id="forwardFactInput" placeholder="输入事实，多个用逗号隔开，如：有羽毛,会飞" onkeydown="if(event.key==='Enter')addForwardFact()">
                <button class="btn btn-primary btn-sm" onclick="addForwardFact()">添加</button>
            </div>
            <div style="font-size:12px;color:#888;margin-bottom:8px;">
                当前事实: <span id="inlineFactCount" class="fact-count">0 个</span>
                <a href="#" onclick="switchTabByName('facts');return false;" style="color:#667eea;">查看详情</a>
            </div>
            <button class="btn btn-primary btn-block" onclick="runForward()">▶ 执行正向推理</button>
        </div>

        <div class="section">
            <div class="section-title">反向推理</div>
            <p style="font-size:12px;color:#888;margin-bottom:8px;">验证特定目标是否成立</p>
            <div class="goal-input">
                <input type="text" id="goalInput" placeholder="输入推理目标，如：是老虎" onkeydown="if(event.key==='Enter')runBackward()">
                <button class="btn btn-success btn-sm" onclick="runBackward()">验证</button>
            </div>
        </div>

        <div class="section">
            <div class="section-title">快速示例</div>
            <button class="btn btn-outline btn-block btn-sm" onclick="loadExample('ostrich')">🦩 识别鸵鸟</button>
            <button class="btn btn-outline btn-block btn-sm" onclick="loadExample('tiger')">🐯 识别虎</button>
            <button class="btn btn-outline btn-block btn-sm" onclick="loadExample('leopard')">🐆 识别豹</button>
            <button class="btn btn-warning btn-block btn-sm" onclick="resetSystem()">🔄 重置推理</button>
        </div>
    </div>

    <div class="panel right">
        <div class="panel-title">
            📊 推理结果与中间步骤
            <div>
                <button class="btn btn-outline btn-sm" onclick="clearResults()">清空</button>
            </div>
        </div>

        <div class="tabs">
            <button class="tab active" onclick="switchTab('results',this)">📋 结果</button>
            <button class="tab" onclick="switchTab('steps',this)">🔍 推理步骤</button>
            <button class="tab" onclick="switchTab('facts',this)">💾 事实库</button>
        </div>

        <div class="tab-content active" id="tab-results">
            <div class="scroll-area-tall" id="resultsContent"></div>
        </div>
        <div class="tab-content" id="tab-steps">
            <div class="scroll-area-tall" id="stepsContent"></div>
        </div>
        <div class="tab-content" id="tab-facts">
            <div style="display:flex;gap:4px;margin-bottom:8px;">
                <button class="btn btn-primary btn-sm" onclick="showAddFactModal()">+ 添加事实</button>
                <button class="btn btn-danger btn-sm" onclick="showDeleteFactModal()">删除事实</button>
            </div>
            <div class="scroll-area-tall" id="factsContent"></div>
        </div>
    </div>
</div>

<div class="status-bar" id="statusBar">就绪</div>

<div class="toast" id="toast"></div>

<div class="modal-overlay" id="addRuleModal">
    <div class="modal">
        <h3>添加规则</h3>
        <div class="input-group">
            <label>条件（用逗号分隔）</label>
            <input type="text" id="ruleConditions" placeholder="条件，多个用逗号隔开（支持中英文），如：有羽毛,会飞">
        </div>
        <div class="input-group">
            <label>结论</label>
            <input type="text" id="ruleConclusion" placeholder="如：是食肉动物">
        </div>
        <div class="modal-buttons">
            <button class="btn btn-outline" onclick="hideModal('addRuleModal')">取消</button>
            <button class="btn btn-primary" onclick="addRule()">添加</button>
        </div>
    </div>
</div>

<div class="modal-overlay" id="deleteRuleModal">
    <div class="modal">
        <h3>删除规则</h3>
        <p style="font-size:13px;color:#666;margin-bottom:8px;">选择要删除的规则：</p>
        <input type="text" id="deleteRuleSearch" placeholder="搜索规则（输入条件或结论关键词）..." oninput="filterDeleteRuleList()" style="width:100%;padding:8px;border:1px solid #ddd;border-radius:6px;font-size:13px;margin-bottom:8px;outline:none;">
        <div class="scroll-area" id="deleteRuleList" style="max-height:280px;margin-bottom:12px;"></div>
        <div class="modal-buttons">
            <button class="btn btn-outline" onclick="hideModal('deleteRuleModal')">取消</button>
            <button class="btn btn-danger" onclick="deleteSelectedRule()">删除选中</button>
        </div>
    </div>
</div>

<div class="modal-overlay" id="modifyRuleModal">
    <div class="modal">
        <h3>修改规则</h3>
        <div class="input-group">
            <label>搜索并选择要修改的规则</label>
            <input type="text" id="modifyRuleSearch" placeholder="输入关键词搜索..." oninput="filterModifyRuleList()" style="width:100%;padding:8px;border:1px solid #ddd;border-radius:6px;font-size:13px;margin-bottom:6px;outline:none;">
            <div class="scroll-area" id="modifyRuleList" style="max-height:150px;border:1px solid #eee;border-radius:6px;padding:4px;"></div>
        </div>
        <div class="input-group">
            <label>条件（用逗号分隔）</label>
            <input type="text" id="modifyRuleConditions" placeholder="条件，多个用逗号隔开（支持中英文），如：有毛发,有爪,吃肉">
        </div>
        <div class="input-group">
            <label>结论</label>
            <input type="text" id="modifyRuleConclusion" placeholder="如：是食肉动物">
        </div>
        <div class="modal-buttons">
            <button class="btn btn-outline" onclick="hideModal('modifyRuleModal')">取消</button>
            <button class="btn btn-primary" onclick="modifyRule()">保存修改</button>
        </div>
    </div>
</div>

<div class="modal-overlay" id="addFactModal">
    <div class="modal">
        <h3>添加事实</h3>
        <div class="input-group">
            <label>事实内容</label>
            <input type="text" id="factInput" placeholder="如：有羽毛" onkeydown="if(event.key==='Enter')addFact()">
        </div>
        <div class="modal-buttons">
            <button class="btn btn-outline" onclick="hideModal('addFactModal')">取消</button>
            <button class="btn btn-primary" onclick="addFact()">添加</button>
        </div>
    </div>
</div>

<div class="modal-overlay" id="deleteFactModal">
    <div class="modal">
        <h3>删除事实</h3>
        <p style="font-size:13px;color:#666;margin-bottom:12px;">选择要删除的事实：</p>
        <div class="scroll-area" id="deleteFactList" style="max-height:300px;margin-bottom:12px;"></div>
        <div class="modal-buttons">
            <button class="btn btn-outline" onclick="hideModal('deleteFactModal')">取消</button>
            <button class="btn btn-danger" onclick="deleteSelectedFact()">删除选中</button>
        </div>
    </div>
</div>

<script>
const API_BASE = '/api';

function showToast(msg, type='') {
    const t = document.getElementById('toast');
    t.textContent = msg; t.className = 'toast ' + type;
    t.classList.add('show');
    setTimeout(() => t.classList.remove('show'), 2500);
}

function setStatus(msg) {
    document.getElementById('statusBar').textContent = msg;
}

async function apiGet(endpoint) {
    const r = await fetch(API_BASE + endpoint);
    return r.json();
}

async function apiPost(endpoint, data={}) {
    const r = await fetch(API_BASE + endpoint, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(data)
    });
    return r.json();
}

async function apiPostWithError(endpoint, data={}) {
    const r = await fetch(API_BASE + endpoint, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(data)
    });
    const result = await r.json();
    if (!r.ok) throw new Error(result.error || '请求失败');
    return result;
}

async function loadRules() {
    const data = await apiGet('/rules');
    const el = document.getElementById('ruleList');
    if (data.rules.length === 0) {
        el.innerHTML = '<div class="empty-hint">知识库为空</div>';
        return;
    }
    el.innerHTML = data.rules.map(r =>
        `<div class="rule-item" data-rule-id="${r.id}" onclick="highlightRule(${r.id})">Rule ${r.id}: IF ${r.conditions.join(' <span class="highlight">AND</span> ')} <span class="highlight">THEN</span> ${r.conclusion}</div>`
    ).join('');
}

async function loadFacts() {
    const data = await apiGet('/facts');
    const factsContent = document.getElementById('factsContent');
    const deleteList = document.getElementById('deleteFactList');
    if (factsContent) {
        if (data.facts.length === 0) {
            factsContent.innerHTML = '<div class="empty-hint">事实库为空</div>';
        } else {
            factsContent.innerHTML = data.facts.map(f =>
                `<div class="fact-item">${f}</div>`
            ).join('');
        }
    }
    if (deleteList) {
        if (data.facts.length === 0) {
            deleteList.innerHTML = '<div class="empty-hint">事实库为空</div>';
        } else {
            deleteList.innerHTML = data.facts.map(f =>
                `<div class="fact-item" data-fact="${f.replace(/"/g, '&quot;')}">${f}</div>`
            ).join('');
        }
    }
    document.querySelector('.fact-count') && (document.querySelector('.fact-count').textContent = `共 ${data.facts.length} 个事实`);
    const inlineCount = document.getElementById('inlineFactCount');
    if (inlineCount) inlineCount.textContent = `${data.facts.length} 个`;
}

async function addForwardFact() {
    const input = document.getElementById('forwardFactInput');
    const raw = input.value.trim();
    if (!raw) { showToast('请输入事实', 'error'); return; }
    const facts = raw.replace(/，/g, ',').split(',').map(s => s.trim()).filter(Boolean);
    if (facts.length === 0) { showToast('请输入有效的事实', 'error'); return; }
    if (facts.length === 1) {
        const data = await apiPost('/facts/add', { fact: facts[0] });
        if (data.success) {
            showToast(`事实 "${facts[0]}" 添加成功`, 'success');
            input.value = '';
            input.focus();
            loadFacts();
        }
    } else {
        let successCount = 0;
        for (const fact of facts) {
            const data = await apiPost('/facts/add', { fact });
            if (data.success) successCount++;
        }
        if (successCount > 0) {
            showToast(`成功添加 ${successCount} 个事实`, 'success');
            input.value = '';
            input.focus();
            loadFacts();
        }
    }
}

async function runForward() {
    setStatus('正在执行正向推理...');
    const data = await apiPost('/inference/forward');
    appendResult('=== 正向推理完成 ===');
    if (data.new_facts.length > 0) {
        appendResult(`推导出新事实 (${data.new_facts.length} 个):`);
        data.new_facts.forEach(f => appendResult('  ✓ ' + f));
    } else {
        appendResult('无新事实推导 — 基于当前事实库无法推导更多结论');
    }
    appendResult(`共执行 ${data.steps} 步推理`);
    const inputFactsCount = data.all_facts.length - data.new_facts.length;
    appendResult(`当前事实库: ${data.all_facts.length} 个 (用户输入 ${inputFactsCount} 个 + 推导出 ${data.new_facts.length} 个)`);
    loadSteps();
    loadFacts();
    setStatus(`正向推理完成，共 ${data.steps} 步`);
    showToast('正向推理完成', 'success');
}

async function runBackward() {
    const goal = document.getElementById('goalInput').value.trim();
    if (!goal) { showToast('请输入推理目标', 'error'); return; }
    setStatus(`正在反向推理: ${goal}...`);
    const data = await apiPost('/inference/backward', { goal });
    appendResult(`=== 反向推理: 目标 "${goal}" ===`);
    if (data.goal_already_known) {
        appendResult(`目标 "${goal}" 已在事实库中（基于已有知识可直接确认）`);
        appendResult(`共执行 ${data.steps} 步验证`);
    } else {
        appendResult(`推导结果: ${data.success ? '✓ 成立' : '✗ 不成立'}`);
        appendResult(`共执行 ${data.steps} 步推理`);
    }
    if (data.missing_facts && data.missing_facts.length > 0) {
        appendResult('');
        appendResult('需要补充以下事实才能推导:');
        const container = document.getElementById('resultsContent');
        const row = document.createElement('div');
        row.style.cssText = 'display:flex;flex-wrap:wrap;gap:6px;margin:6px 0;';
        data.missing_facts.forEach(f => {
            const btn = document.createElement('button');
            btn.className = 'btn btn-sm';
            btn.style.cssText = 'padding:4px 10px;font-size:12px;background:#eef0ff;color:#667eea;border:1px solid #667eea;border-radius:4px;cursor:pointer;';
            btn.textContent = '+ ' + f;
            btn.onclick = async () => {
                await apiPost('/facts/add', { fact: f });
                showToast(`事实 "${f}" 已添加`, 'success');
                loadFacts();
                btn.disabled = true;
                btn.style.background = '#f0fdf4';
                btn.style.borderColor = '#22c55e';
                btn.style.color = '#22c55e';
                btn.textContent = '✓ ' + f;
            };
            row.appendChild(btn);
        });
        container.appendChild(row);
        const addAllBtn = document.createElement('button');
        addAllBtn.className = 'btn btn-primary btn-sm';
        addAllBtn.textContent = '一键添加所有缺失事实并重新验证';
        addAllBtn.style.cssText = 'padding:8px 16px;font-size:13px;margin:8px 0;';
        addAllBtn.onclick = async () => {
            for (const f of data.missing_facts) {
                await apiPost('/facts/add', { fact: f });
            }
            showToast('所有缺失事实已添加，正在重新验证...', 'success');
            loadFacts();
            document.getElementById('goalInput').value = goal;
            await runBackward();
        };
        container.appendChild(addAllBtn);
        appendResult('');
    }
    loadSteps();
    loadFacts();
    if (data.success) {
        showToast(`目标 "${goal}" 推导成功`, 'success');
    } else {
        showToast(`目标 "${goal}" 无法推导，缺少 ${data.missing_facts ? data.missing_facts.length : 0} 个事实`, 'error');
    }
    setStatus(`反向推理完成: ${goal}`);
}

function highlightRule(ruleId) {
    const el = document.querySelector(`.rule-item[data-rule-id="${ruleId}"]`);
    if (!el) return;
    el.classList.remove('rule-highlight');
    void el.offsetWidth;
    el.classList.add('rule-highlight');
    el.scrollIntoView({ behavior: 'smooth', block: 'center' });
}

async function loadSteps() {
    const data = await apiGet('/inference/steps');
    const el = document.getElementById('stepsContent');
    if (data.steps.length === 0) {
        el.innerHTML = '<div class="empty-hint">暂无推理步骤，请先执行推理</div>';
        return;
    }
    el.innerHTML = data.steps.map((s, i) => {
        let html = '', bgColor = '#fff7ed', borderColor = '#f97316';
        if (s.type === 'forward') {
            const ruleStr = `IF ${s.rule_conditions.join(' AND ')} THEN ${s.rule_conclusion}`;
            html = `<div style="display:flex;justify-content:space-between;align-items:flex-start;">
                <div><strong>步骤 ${i+1}</strong>: 正向推理 [迭代${s.iteration}]</div>
                <span class="rule-link" onclick="highlightRule(${s.rule_id})">跳转 Rule ${s.rule_id}</span>
            </div>
            <div style="margin-top:4px;font-size:12px;">规则 ${s.rule_id}: ${ruleStr}</div>
            <div style="margin-top:2px;font-size:12px;color:#22c55e;">→ 推导出: "${s.new_fact}"</div>`;
            bgColor = '#f0fdf4'; borderColor = '#22c55e';
        } else if (s.type === 'backward') {
            if (s.result === '已知事实') {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → 目标"${s.goal}" <span style="color:#22c55e;">✓ 已在事实库</span>`;
                bgColor = '#f0fdf4'; borderColor = '#22c55e';
            } else if (s.result === '无规则可推导') {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → 目标"${s.goal}" <span style="color:#ef4444;">✗ 无匹配规则</span>`;
                bgColor = '#fef2f2'; borderColor = '#ef4444';
            } else if (s.attempt) {
                const ruleStr = `IF ${s.rule_conditions.join(' AND ')} THEN ${s.rule_conclusion}`;
                html = `<div style="display:flex;justify-content:space-between;align-items:flex-start;">
                    <div><strong>步骤 ${i+1}</strong>: 反向推理 → 尝试推导"${s.goal}"</div>
                    <span class="rule-link" onclick="highlightRule(${s.rule_id})">跳转 Rule ${s.rule_id}</span>
                </div>
                <div style="margin-top:4px;font-size:12px;">规则 ${s.rule_id}: ${ruleStr}</div>
                <div style="margin-top:2px;font-size:12px;color:#888;">需要条件: ${s.conditions.join(', ')}</div>`;
                bgColor = '#fff7ed'; borderColor = '#f97316';
            } else if (s.result === '通过规则推导成功') {
                const ruleStr = `IF ${s.rule_conditions.join(' AND ')} THEN ${s.rule_conclusion}`;
                html = `<div style="display:flex;justify-content:space-between;align-items:flex-start;">
                    <div><strong>步骤 ${i+1}</strong>: 反向推理 → 推导"${s.goal}" <span style="color:#22c55e;">✓ 成功</span></div>
                    <span class="rule-link" onclick="highlightRule(${s.rule_id})">跳转 Rule ${s.rule_id}</span>
                </div>
                <div style="margin-top:4px;font-size:12px;">规则 ${s.rule_id}: ${ruleStr}</div>`;
                bgColor = '#f0fdf4'; borderColor = '#22c55e';
            } else if (s.result === '规则条件不满足') {
                const ruleStr = `IF ${s.rule_conditions.join(' AND ')} THEN ${s.rule_conclusion}`;
                html = `<div style="display:flex;justify-content:space-between;align-items:flex-start;">
                    <div><strong>步骤 ${i+1}</strong>: 反向推理 → 推导"${s.goal}" <span style="color:#ef4444;">✗ 条件不满足</span></div>
                    <span class="rule-link" onclick="highlightRule(${s.rule_id})">跳转 Rule ${s.rule_id}</span>
                </div>
                <div style="margin-top:4px;font-size:12px;">规则 ${s.rule_id}: ${ruleStr}</div>`;
                bgColor = '#fef2f2'; borderColor = '#ef4444';
            } else if (s.result === '循环依赖，跳过') {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → 目标"${s.goal}" <span style="color:#f59e0b;">⚠ 循环依赖，跳过</span>`;
                bgColor = '#fef3c7'; borderColor = '#f59e0b';
            } else {
                html = `<strong>步骤 ${i+1}</strong>: 反向推理 → 目标"${s.goal}"${s.rule_id ? `，尝试 Rule ${s.rule_id}` : ''}`;
            }
        }
        return `<div class="step-item" style="background:${bgColor};border-left-color:${borderColor};">${html}</div>`;
    }).join('');
}

function appendResult(text) {
    const el = document.getElementById('resultsContent');
    const d = document.createElement('div');
    d.style.cssText = 'padding:4px 0;font-size:13px;line-height:1.6;';
    d.textContent = text;
    el.appendChild(d);
    el.scrollTop = el.scrollHeight;
}

function clearResults() {
    document.getElementById('resultsContent').innerHTML = '';
    document.getElementById('stepsContent').innerHTML = '';
    setStatus('结果已清空');
}

function switchTab(name, btn) {
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    btn.classList.add('active');
    document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
    document.getElementById('tab-' + name).classList.add('active');
    if (name === 'steps') loadSteps();
    if (name === 'facts') loadFacts();
}

function switchTabByName(name) {
    const tabs = document.querySelectorAll('.tab');
    const tabMap = { results: 0, steps: 1, facts: 2 };
    const idx = tabMap[name];
    if (idx !== undefined && tabs[idx]) {
        switchTab(name, tabs[idx]);
    }
}

function showModal(id) { document.getElementById(id).classList.add('active'); }
function hideModal(id) { document.getElementById(id).classList.remove('active'); }

function showAddRuleModal() { showModal('addRuleModal'); }

async function addRule() {
    const conditions = document.getElementById('ruleConditions').value.trim();
    const conclusion = document.getElementById('ruleConclusion').value.trim();
    if (!conditions || !conclusion) { showToast('条件和结论不能为空', 'error'); return; }
    const condList = conditions.replace(/，/g, ',').split(',').map(s=>s.trim()).filter(Boolean);
    const rulesData = await apiGet('/rules');
    const duplicate = rulesData.rules.find(r =>
        r.conclusion === conclusion &&
        r.conditions.length === condList.length &&
        r.conditions.every((c, i) => c === condList[i])
    );
    if (duplicate) {
        showToast(`规则已存在 (Rule ${duplicate.id})，请勿重复添加`, 'error');
        return;
    }
    try {
        const data = await apiPostWithError('/rules/add', { conditions: condList, conclusion });
        showToast(`规则添加成功 (ID: ${data.rule_id})`, 'success');
        hideModal('addRuleModal');
        document.getElementById('ruleConditions').value = '';
        document.getElementById('ruleConclusion').value = '';
        loadRules();
        appendResult(`规则添加成功: IF ${conditions} THEN ${conclusion}`);
        setStatus(`规则 ${data.rule_id} 添加成功`);
    } catch (e) {
        showToast(e.message, 'error');
    }
}

let selectedRuleId = null;
let deleteRulesCache = [];
function showDeleteRuleModal() {
    showModal('deleteRuleModal');
    document.getElementById('deleteRuleSearch').value = '';
    const el = document.getElementById('deleteRuleList');
    apiGet('/rules').then(data => {
        deleteRulesCache = data.rules;
        if (data.rules.length === 0) {
            el.innerHTML = '<div class="empty-hint">知识库为空</div>';
            return;
        }
        renderDeleteRuleList(data.rules);
    });
}

function renderDeleteRuleList(rules) {
    const el = document.getElementById('deleteRuleList');
    if (rules.length === 0) {
        el.innerHTML = '<div class="empty-hint">无匹配规则</div>';
        return;
    }
    el.innerHTML = rules.map(r =>
        `<div class="rule-item" onclick="selectDeleteRule(this,${r.id})">Rule ${r.id}: ${r.conditions.join(' AND ')} → ${r.conclusion}</div>`
    ).join('');
}

function filterDeleteRuleList() {
    const keyword = document.getElementById('deleteRuleSearch').value.trim().toLowerCase();
    if (!keyword) {
        renderDeleteRuleList(deleteRulesCache);
        return;
    }
    const filtered = deleteRulesCache.filter(r =>
        r.conditions.some(c => c.toLowerCase().includes(keyword)) ||
        r.conclusion.toLowerCase().includes(keyword) ||
        `rule ${r.id}`.includes(keyword)
    );
    renderDeleteRuleList(filtered);
}

function selectDeleteRule(el, id) {
    document.querySelectorAll('#deleteRuleList .rule-item').forEach(e => e.style.background = '#f8f9ff');
    el.style.background = '#fef2f2';
    el.style.borderLeftColor = '#ef4444';
    selectedRuleId = id;
}

async function deleteSelectedRule() {
    if (!selectedRuleId) { showToast('请选择要删除的规则', 'error'); return; }
    const data = await apiPost('/rules/delete', { rule_id: selectedRuleId });
    if (data.success) {
        showToast(`规则 ${selectedRuleId} 已删除`, 'success');
        hideModal('deleteRuleModal');
        selectedRuleId = null;
        loadRules();
        appendResult(`规则 ${selectedRuleId} 已删除`);
        setStatus(`规则 ${selectedRuleId} 已删除`);
    }
}

let modifyRuleId = null;
let modifyRulesCache = [];
function showModifyRuleModal() {
    showModal('modifyRuleModal');
    document.getElementById('modifyRuleSearch').value = '';
    document.getElementById('modifyRuleConditions').value = '';
    document.getElementById('modifyRuleConclusion').value = '';
    modifyRuleId = null;
    apiGet('/rules').then(data => {
        modifyRulesCache = data.rules;
        renderModifyRuleList(data.rules);
    });
}

function renderModifyRuleList(rules) {
    const el = document.getElementById('modifyRuleList');
    if (rules.length === 0) {
        el.innerHTML = '<div class="empty-hint">无匹配规则</div>';
        return;
    }
    el.innerHTML = rules.map(r =>
        `<div class="rule-item" data-id="${r.id}" onclick="selectModifyRule(this,${r.id})">Rule ${r.id}: ${r.conditions.join(' AND ')} → ${r.conclusion}</div>`
    ).join('');
}

function filterModifyRuleList() {
    const keyword = document.getElementById('modifyRuleSearch').value.trim().toLowerCase();
    if (!keyword) {
        renderModifyRuleList(modifyRulesCache);
        return;
    }
    const filtered = modifyRulesCache.filter(r =>
        r.conditions.some(c => c.toLowerCase().includes(keyword)) ||
        r.conclusion.toLowerCase().includes(keyword) ||
        `rule ${r.id}`.includes(keyword)
    );
    renderModifyRuleList(filtered);
}

function selectModifyRule(el, id) {
    document.querySelectorAll('#modifyRuleList .rule-item').forEach(e => {
        e.style.background = '#f8f9ff';
        e.style.borderLeftColor = '#667eea';
    });
    el.style.background = '#eef0ff';
    el.style.borderLeftColor = '#667eea';
    modifyRuleId = id;
    const rule = modifyRulesCache.find(r => r.id === id);
    if (rule) {
        document.getElementById('modifyRuleConditions').value = rule.conditions.join(', ');
        document.getElementById('modifyRuleConclusion').value = rule.conclusion;
    }
}

async function modifyRule() {
    if (!modifyRuleId) { showToast('请先选择要修改的规则', 'error'); return; }
    const conditions = document.getElementById('modifyRuleConditions').value.trim();
    const conclusion = document.getElementById('modifyRuleConclusion').value.trim();
    if (!conditions || !conclusion) { showToast('条件和结论不能为空', 'error'); return; }
    const condList = conditions.replace(/，/g, ',').split(',').map(s=>s.trim()).filter(Boolean);
    const data = await apiPost('/rules/modify', {
        rule_id: modifyRuleId,
        conditions: condList,
        conclusion
    });
    if (data.success) {
        showToast(`规则 ${modifyRuleId} 修改成功`, 'success');
        hideModal('modifyRuleModal');
        modifyRuleId = null;
        loadRules();
        appendResult(`规则修改成功: IF ${conditions} THEN ${conclusion}`);
        setStatus(`规则 ${modifyRuleId} 修改成功`);
    } else {
        showToast('规则修改失败', 'error');
    }
}

let selectedFact = null;
function showDeleteFactModal() {
    showModal('deleteFactModal');
    loadFacts();
}

document.getElementById('deleteFactList').addEventListener('click', function(e) {
    const item = e.target.closest('.fact-item');
    if (!item) return;
    document.querySelectorAll('#deleteFactList .fact-item').forEach(el => {
        el.style.background = '#f0fdf4';
        el.style.borderLeftColor = '#22c55e';
    });
    item.style.background = '#fef2f2';
    item.style.borderLeftColor = '#ef4444';
    selectedFact = item.dataset.fact;
});

async function deleteSelectedFact() {
    if (!selectedFact) { showToast('请选择要删除的事实', 'error'); return; }
    const data = await apiPost('/facts/delete', { fact: selectedFact });
    if (data.success) {
        showToast(`事实 "${selectedFact}" 已删除`, 'success');
        hideModal('deleteFactModal');
        selectedFact = null;
        loadFacts();
        setStatus(`事实已删除`);
    }
}

function showAddFactModal() {
    showModal('addFactModal');
    document.getElementById('factInput').value = '';
    document.getElementById('factInput').focus();
}

async function addFact() {
    const fact = document.getElementById('factInput').value.trim();
    if (!fact) { showToast('事实不能为空', 'error'); return; }
    const data = await apiPost('/facts/add', { fact });
    if (data.success) {
        showToast(`事实 "${fact}" 添加成功`, 'success');
        hideModal('addFactModal');
        loadFacts();
        appendResult(`事实添加成功: ${fact}`);
        setStatus(`事实 "${fact}" 添加成功`);
    }
}

async function loadExample(type) {
    await apiPost('/reset');
    const data = await apiPost('/example', { type });
    if (data.success) {
        loadRules();
        loadFacts();
        document.getElementById('resultsContent').innerHTML = '';
        appendResult(`=== ${data.label} ===`);
        appendResult(`已添加事实: ${data.facts.join(', ')}`);
        appendResult('点击"执行正向推理"查看结果');
        setStatus(`${data.label}已加载`);
        showToast(`${data.label}已加载`, 'success');
    }
}

async function clearFactsOnly() {
    const data = await apiPost('/facts/clear');
    if (data.success) {
        loadFacts();
        document.getElementById('resultsContent').innerHTML = '';
        setStatus('事实已清空');
        showToast('事实已清空', 'success');
    }
}

async function resetSystem() {
    const data = await apiPost('/reset');
    if (data.success) {
        loadRules();
        loadFacts();
        document.getElementById('resultsContent').innerHTML = '';
        document.getElementById('stepsContent').innerHTML = '';
        setStatus('推理状态已重置，规则保持不变');
        showToast('推理已重置，你添加的规则仍在', 'success');
    }
}

loadRules();
loadFacts();
</script>
</body>
</html>"""


class Handler(BaseHTTPRequestHandler):

    def log_message(self, format, *args):
        pass

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
                rules.append({
                    'id': r.id,
                    'conditions': r.conditions,
                    'conclusion': r.conclusion
                })
            self._send_json({'rules': rules})
        elif path == '/api/facts':
            self._send_json({'facts': list(fb.get_facts())})
        elif path == '/api/inference/steps':
            steps = engine.get_steps()
            step_list = []
            for s in steps:
                item = {'type': s['type']}
                if s['type'] == 'forward':
                    item['rule_id'] = s['rule'].id
                    item['new_fact'] = s['new_fact']
                    item['iteration'] = s['iteration']
                    item['rule_conditions'] = s['rule'].conditions
                    item['rule_conclusion'] = s['rule'].conclusion
                elif s['type'] == 'backward':
                    item['goal'] = s.get('goal', '')
                    if s.get('rule'):
                        item['rule_id'] = s['rule'].id
                        item['rule_conditions'] = s['rule'].conditions
                        item['rule_conclusion'] = s['rule'].conclusion
                    if s.get('result'):
                        item['result'] = s['result']
                    if s.get('attempt'):
                        item['attempt'] = s['attempt']
                    if s.get('conditions'):
                        item['conditions'] = s['conditions']
                step_list.append(item)
            self._send_json({'steps': step_list})
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
                    rule_id = kb.add_rule(conditions, conclusion)
                    self._send_json({'success': True, 'rule_id': rule_id})
            else:
                self._send_json({'success': False, 'error': '参数错误'}, 400)

        elif path == '/api/rules/delete':
            rule_id = body.get('rule_id')
            if rule_id:
                kb.delete_rule(rule_id)
                self._send_json({'success': True})
            else:
                self._send_json({'success': False, 'error': '缺少rule_id'}, 400)

        elif path == '/api/rules/modify':
            rule_id = body.get('rule_id')
            conditions = body.get('conditions')
            conclusion = body.get('conclusion')
            if rule_id and (conditions or conclusion):
                success = kb.modify_rule(rule_id, conditions, conclusion)
                self._send_json({'success': success})
            else:
                self._send_json({'success': False, 'error': '参数错误'}, 400)

        elif path == '/api/facts/add':
            fact = body.get('fact', '')
            if fact:
                fb.add_fact(fact)
                self._send_json({'success': True})
            else:
                self._send_json({'success': False, 'error': '缺少fact'}, 400)

        elif path == '/api/facts/delete':
            fact = body.get('fact', '')
            if fact:
                result = fb.remove_fact(fact)
                self._send_json({'success': result})
            else:
                self._send_json({'success': False, 'error': '缺少fact'}, 400)

        elif path == '/api/facts/clear':
            fb.clear()
            self._send_json({'success': True})

        elif path == '/api/inference/forward':
            new_facts = engine.forward_chain()
            steps = engine.get_steps()
            all_facts = list(fb.get_facts())
            self._send_json({
                'success': True,
                'new_facts': new_facts,
                'all_facts': all_facts,
                'steps': len(steps)
            })

        elif path == '/api/inference/backward':
            goal = body.get('goal', '')
            if goal:
                goal_already_known = fb.contains(goal)
                success = engine.backward_chain(goal)
                steps = engine.get_steps()
                missing_facts = engine.get_missing_facts_for_goal(goal)
                self._send_json({
                    'success': success,
                    'goal': goal,
                    'steps': len(steps),
                    'missing_facts': missing_facts,
                    'goal_already_known': goal_already_known
                })
            else:
                self._send_json({'success': False, 'error': '缺少goal'}, 400)

        elif path == '/api/reset':
            fb.clear()
            engine.reset_steps()
            self._send_json({'success': True})

        elif path == '/api/facts/clear':
            fb.clear()
            self._send_json({'success': True})

        elif path == '/api/example':
            ex_type = body.get('type', '')
            example_facts = {
                'ostrich': (['有脊椎', '有羽毛', '无龙骨突', '体型大', '长颈', '黑白羽色'], '鸵鸟识别示例'),
                'tiger': (['有脊椎', '有乳腺', '犬齿发达', '食肉', '独居', '趾行性', '黄褐色', '有黑色横纹'], '虎识别示例'),
                'leopard': (['有脊椎', '有乳腺', '犬齿发达', '食肉', '独居', '趾行性', '黄褐色', '有暗色斑纹'], '豹识别示例'),
            }
            if ex_type in example_facts:
                facts, label = example_facts[ex_type]
                for f in facts:
                    fb.add_fact(f)
                self._send_json({'success': True, 'label': label, 'facts': facts})
            else:
                self._send_json({'success': False, 'error': '未知示例'}, 400)

        else:
            self._send_json({'error': 'Not Found'}, 404)


def main():
    port = 8080
    server = HTTPServer(('0.0.0.0', port), Handler)
    print(f"动物识别专家系统 - 全扫描算法")
    print(f"请在浏览器中打开: http://localhost:{port}")
    print(f"按 Ctrl+C 停止服务器")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\n服务器已停止")
        server.server_close()


if __name__ == '__main__':
    main()
