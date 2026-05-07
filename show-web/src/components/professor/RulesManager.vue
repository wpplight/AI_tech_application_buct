<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useProfessorStore } from '../../stores/professor'
import professorService from '../../api/professor'
import SmartPagination from '../common/SmartPagination.vue'

const store = useProfessorStore()

const rules = ref<any[]>([])
const totalRules = ref(0)
const currentPage = ref(1)
const pageSize = ref(20)
const searchQuery = ref('')
const isLoading = ref(false)

const newConditionInput = ref('')
const newConditions = ref<string[]>([])
const newConclusion = ref('')
const isAdding = ref(false)
const addError = ref('')
const deletingId = ref<number | null>(null)

const totalPages = computed(() => Math.ceil(totalRules.value / pageSize.value))
const pageNumbers = computed(() => {
  const pages: number[] = []
  for (let i = 1; i <= totalPages.value; i++) pages.push(i)
  return pages
})

async function loadRules() {
  isLoading.value = true
  try {
    const response = await professorService.getRulesPaginated(
      store.currentAlgorithm,
      currentPage.value,
      pageSize.value
    )
    rules.value = response.rules || []
    totalRules.value = response.pagination?.total || 0
  } catch (e) {
    console.error('加载规则失败', e)
  } finally {
    isLoading.value = false
  }
}

function addCondition() {
  const v = newConditionInput.value.trim()
  if (v) {
    newConditions.value.push(v)
    newConditionInput.value = ''
  }
}

function removeCondition(idx: number) {
  newConditions.value.splice(idx, 1)
}

async function submitAddRule() {
  addError.value = ''
  if (newConditions.value.length === 0) {
    addError.value = '请至少添加一个条件'
    return
  }
  if (!newConclusion.value.trim()) {
    addError.value = '请填写结论'
    return
  }
  isAdding.value = true
  try {
    const result = await professorService.addRule(
      newConditions.value,
      newConclusion.value.trim(),
      store.currentAlgorithm
    )
    if (result.success) {
      newConditions.value = []
      newConclusion.value = ''
      currentPage.value = 1
      await loadRules()
      store.loadRules()
    } else {
      addError.value = result.error || '添加失败'
    }
  } catch (e) {
    addError.value = '网络错误'
  } finally {
    isAdding.value = false
  }
}

async function handleDelete(ruleId: number) {
  deletingId.value = ruleId
  try {
    await professorService.deleteRule(ruleId, store.currentAlgorithm)
    rules.value = rules.value.filter(r => r.id !== ruleId)
    totalRules.value--
    if (rules.value.length === 0 && currentPage.value > 1) {
      currentPage.value--
      await loadRules()
    }
    store.loadRules()
  } catch (e) {
    console.error('删除失败', e)
  } finally {
    deletingId.value = null
  }
}

onMounted(loadRules)
</script>

<template>
  <div class="rules-manager">
    <div class="manager-toolbar">
      <div class="toolbar-left">
        <h3 class="section-title">规则库</h3>
        <span class="total-badge">共 {{ totalRules }} 条规则</span>
      </div>
      <div class="toolbar-right">
        <div class="search-box">
          <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8"/>
            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
          </svg>
          <input
            v-model="searchQuery"
            class="search-input"
            placeholder="搜索规则..."
          />
        </div>
      </div>
    </div>

    <div class="add-rule-card">
      <h4 class="add-title">添加新规则</h4>
      <div class="add-form">
        <div class="form-group">
          <label class="form-label">条件（可添加多个）</label>
          <div class="conditions-row">
            <div class="conditions-chips">
              <span
                v-for="(c, idx) in newConditions"
                :key="idx"
                class="chip"
              >
                {{ c }}
                <button class="chip-remove" @click="removeCondition(idx)">×</button>
              </span>
            </div>
            <div class="condition-add-row">
              <input
                v-model="newConditionInput"
                class="inline-input"
                placeholder="输入条件后回车或点击添加"
                @keydown.enter.prevent="addCondition"
              />
              <button class="btn-add-cond" @click="addCondition">+</button>
            </div>
          </div>
        </div>
        <div class="form-group">
          <label class="form-label">结论</label>
          <input
            v-model="newConclusion"
            class="form-input"
            placeholder="输入结论"
          />
        </div>
        <div class="form-error" v-if="addError">{{ addError }}</div>
        <button
          class="btn-submit"
          :disabled="isAdding"
          @click="submitAddRule"
        >
          {{ isAdding ? '添加中...' : '添加规则' }}
        </button>
      </div>
    </div>

    <div class="rules-list" v-if="!isLoading">
      <div
        v-for="rule in rules"
        :key="rule.id"
        class="rule-item"
      >
        <div class="rule-id">#{{ rule.id }}</div>
        <div class="rule-body">
          <div class="rule-conditions">
            <span
              v-for="(cond, idx) in rule.conditions"
              :key="idx"
              class="condition-tag"
            >{{ cond }}</span>
          </div>
          <svg class="rule-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="5" y1="12" x2="19" y2="12"/>
            <polyline points="12 5 19 12 12 19"/>
          </svg>
          <span class="rule-conclusion">{{ rule.conclusion }}</span>
        </div>
        <button
          class="btn-delete"
          @click="handleDelete(rule.id)"
          :disabled="deletingId === rule.id"
        >
          {{ deletingId === rule.id ? '删除中...' : '删除' }}
        </button>
      </div>
      <div v-if="rules.length === 0" class="empty-state">
        暂无规则
      </div>
    </div>
    <div v-else class="loading-state">加载中...</div>

    <SmartPagination
      v-if="totalPages > 1"
      :current="currentPage"
      :total="totalPages"
      @change="(p) => { currentPage = p; loadRules() }"
    />
  </div>
</template>

<style scoped>
.rules-manager {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.manager-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.total-badge {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  padding: 2px 10px;
  border-radius: 20px;
  font-size: 12px;
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 10px;
  width: 16px;
  height: 16px;
  color: var(--text-muted);
  pointer-events: none;
}

.search-input {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 6px 12px 6px 34px;
  font-size: 14px;
  color: var(--text-primary);
  width: 200px;
  outline: none;
  transition: border-color 0.2s;
}

.search-input:focus {
  border-color: var(--text-muted);
}

.add-rule-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  padding: 16px;
}

.add-title {
  font-size: 14px;
  font-weight: 600;
  margin: 0 0 12px;
  color: var(--text-primary);
}

.add-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.conditions-row {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.conditions-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  min-height: 28px;
}

.chip {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  padding: 3px 8px;
  border-radius: 6px;
  font-size: 13px;
  display: flex;
  align-items: center;
  gap: 4px;
}

.chip-remove {
  background: none;
  border: none;
  color: var(--text-muted);
  font-size: 14px;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.chip-remove:hover {
  color: var(--text-primary);
}

.condition-add-row {
  display: flex;
  gap: 8px;
}

.inline-input {
  flex: 1;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 6px 10px;
  font-size: 14px;
  color: var(--text-primary);
  outline: none;
}

.inline-input:focus {
  border-color: var(--text-muted);
}

.btn-add-cond {
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 16px;
  width: 32px;
  cursor: pointer;
  flex-shrink: 0;
}

.btn-add-cond:hover {
  background: var(--border-color);
  color: var(--text-primary);
}

.form-input {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 6px 10px;
  font-size: 14px;
  color: var(--text-primary);
  outline: none;
}

.form-input:focus {
  border-color: var(--text-muted);
}

.form-error {
  color: var(--accent-red);
  font-size: 13px;
}

.btn-submit {
  background: var(--border-color);
  border: none;
  border-radius: 8px;
  color: var(--text-primary);
  padding: 8px 16px;
  font-size: 14px;
  cursor: pointer;
  align-self: flex-start;
}

.btn-submit:hover {
  background: var(--text-muted);
}

.btn-submit:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.rules-list {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  overflow: hidden;
}

.rule-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
}

.rule-item:last-child {
  border-bottom: none;
}

.rule-id {
  color: var(--text-muted);
  font-size: 12px;
  font-weight: 600;
  min-width: 36px;
}

.rule-body {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.rule-conditions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.condition-tag {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
}

.rule-arrow {
  width: 16px;
  height: 16px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.rule-conclusion {
  color: var(--accent-lime);
  font-weight: 500;
  font-size: 13px;
}

.btn-delete {
  background: none;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-muted);
  font-size: 12px;
  padding: 4px 10px;
  cursor: pointer;
  flex-shrink: 0;
}

.btn-delete:hover {
  border-color: var(--accent-red);
  color: var(--accent-red);
}

.btn-delete:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.empty-state {
  padding: 40px;
  text-align: center;
  color: var(--text-muted);
  font-size: 14px;
}

.loading-state {
  padding: 40px;
  text-align: center;
  color: var(--text-muted);
  font-size: 14px;
}

.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 4px;
  flex-wrap: wrap;
}

.page-btn {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-secondary);
  font-size: 13px;
  padding: 4px 10px;
  cursor: pointer;
  min-width: 36px;
}

.page-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.page-btn.active {
  background: var(--border-color);
  border-color: var(--text-muted);
  color: var(--text-primary);
}

.page-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
</style>
