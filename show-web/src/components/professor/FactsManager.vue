<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { professorService, type Rule } from '../../api/professor'
import { useProfessorStore } from '../../stores/professor'
import SmartPagination from '../common/SmartPagination.vue'

const store = useProfessorStore()

const allFacts = ref<string[]>([])
const isLoading = ref(false)

const searchQuery = ref('')
const showDropdown = ref(false)
const selectedFact = ref<string | null>(null)

const relatedRules = ref<Rule[]>([])
const isSearchingRules = ref(false)

const currentPage = ref(1)
const pageSize = 50

const filteredFacts = computed(() => {
  if (!searchQuery.value) return allFacts.value
  const q = searchQuery.value.toLowerCase()
  return allFacts.value.filter(f => f.toLowerCase().includes(q))
})

const paginatedFacts = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  return filteredFacts.value.slice(start, start + pageSize)
})

const totalFacts = computed(() => filteredFacts.value.length)

onMounted(async () => {
  isLoading.value = true
  try {
    const res = await professorService.getAllFacts(store.currentAlgorithm, 1, 1000)
    allFacts.value = res.facts || []
  } catch (e) {
    console.error('加载事实库失败', e)
  } finally {
    isLoading.value = false
  }
})

async function selectFact(fact: string) {
  selectedFact.value = fact
  searchQuery.value = fact
  showDropdown.value = false
  isSearchingRules.value = true
  try {
    const res = await professorService.searchRulesByFact(fact, store.currentAlgorithm)
    relatedRules.value = res.rules || []
  } catch {
    relatedRules.value = []
  } finally {
    isSearchingRules.value = false
  }
}

function clearSelection() {
  selectedFact.value = null
  relatedRules.value = []
}

function handlePageChange(page: number) {
  currentPage.value = page
}

function handleSearchInput() {
  showDropdown.value = filteredFacts.value.length > 0 && filteredFacts.value.length < allFacts.value.length
}

function handleBlur() {
  setTimeout(() => {
    showDropdown.value = false
  }, 150)
}
</script>

<template>
  <div class="facts-manager">
    <div class="facts-list-panel">
      <div class="search-box">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input
          v-model="searchQuery"
          type="text"
          class="search-input"
          placeholder="搜索事实..."
          @input="handleSearchInput"
          @focus="showDropdown = filteredFacts.length > 0 && filteredFacts.length < allFacts.length"
          @blur="handleBlur"
        />
      </div>

      <div v-if="showDropdown && filteredFacts.length > 0 && filteredFacts.length < allFacts.length" class="search-dropdown">
        <button
          v-for="fact in filteredFacts.slice(0, 8)"
          :key="fact"
          class="dropdown-item"
          @mousedown.prevent="selectFact(fact)"
        >
          {{ fact }}
        </button>
      </div>

      <div v-if="isLoading" class="loading">加载中...</div>

      <div v-else class="facts-list">
        <button
          v-for="fact in paginatedFacts"
          :key="fact"
          class="fact-item"
          :class="{ selected: selectedFact === fact }"
          @click="selectFact(fact)"
        >
          {{ fact }}
        </button>
      </div>

      <div v-if="!isLoading && totalFacts > pageSize" class="pagination-wrap">
        <SmartPagination
          :current="currentPage"
          :total="totalFacts"
          :page-size="pageSize"
          @change="handlePageChange"
        />
      </div>
    </div>

    <div class="rules-panel">
      <div v-if="!selectedFact" class="rules-empty">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/>
          <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/>
        </svg>
        <p>点击左侧事实查看关联规则</p>
      </div>

      <template v-else>
        <div class="rules-header">
          <span class="rules-label">关联规则</span>
          <span class="rules-count">{{ relatedRules.length }} 条</span>
          <button class="btn-clear" @click="clearSelection">清空</button>
        </div>

        <div v-if="isSearchingRules" class="loading">搜索中...</div>

        <div v-else-if="relatedRules.length === 0" class="no-rules">
          没有找到包含「{{ selectedFact }}」的规则
        </div>

        <div v-else class="rules-list">
          <div
            v-for="rule in relatedRules"
            :key="rule.id"
            class="rule-item"
          >
            <span class="rule-id">R{{ rule.id }}</span>
            <div class="rule-body">
              <div class="rule-conditions">
                <span
                  v-for="(cond, idx) in rule.conditions"
                  :key="idx"
                  class="cond-tag"
                  :class="{ highlight: cond === selectedFact }"
                >{{ cond }}</span>
                <span v-if="rule.conditions.length === 0" class="cond-empty">（无前提条件）</span>
              </div>
              <svg class="rule-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="5" y1="12" x2="19" y2="12"/>
                <polyline points="12 5 19 12 12 19"/>
              </svg>
              <span class="rule-conclusion">{{ rule.conclusion }}</span>
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.facts-manager {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  height: 100%;
}

@media (max-width: 1024px) {
  .facts-manager {
    grid-template-columns: 1fr;
  }
}

.facts-list-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
  position: relative;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  transition: border-color 0.2s;
}

.search-box:focus-within {
  border-color: var(--accent-blue);
}

.search-icon {
  width: 16px;
  height: 16px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.search-input {
  flex: 1;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
}

.search-input::placeholder {
  color: var(--text-muted);
}

.search-dropdown {
  position: absolute;
  top: 48px;
  left: 0;
  right: 0;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  z-index: 50;
  overflow: hidden;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
}

.dropdown-item {
  display: block;
  width: 100%;
  padding: 10px 14px;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 13px;
  text-align: left;
  cursor: pointer;
  transition: background 0.15s;
}

.dropdown-item:hover {
  background: var(--bg-tertiary);
}

.loading {
  text-align: center;
  padding: 40px;
  color: var(--text-muted);
  font-size: 14px;
}

.facts-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
  max-height: 500px;
  overflow-y: auto;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  padding: 8px;
}

.fact-item {
  padding: 8px 12px;
  background: transparent;
  border: none;
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 13px;
  text-align: left;
  cursor: pointer;
  transition: all 0.15s;
}

.fact-item:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.fact-item.selected {
  background: rgba(59, 130, 246, 0.1);
  color: var(--accent-blue);
}

.pagination-wrap {
  display: flex;
  justify-content: center;
}

.rules-panel {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.rules-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  min-height: 300px;
  gap: 12px;
  color: var(--text-muted);
  text-align: center;
  background: var(--bg-secondary);
  border: 1px dashed var(--border-color);
  border-radius: 12px;
}

.rules-empty svg {
  width: 40px;
  height: 40px;
  opacity: 0.4;
}

.rules-empty p {
  margin: 0;
  font-size: 14px;
}

.rules-header {
  display: flex;
  align-items: center;
  gap: 10px;
}

.rules-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.rules-count {
  font-size: 12px;
  color: var(--accent-blue);
  background: rgba(59, 130, 246, 0.1);
  padding: 2px 8px;
  border-radius: 10px;
}

.btn-clear {
  margin-left: auto;
  padding: 3px 10px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-muted);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-clear:hover {
  border-color: var(--accent-red);
  color: var(--accent-red);
}

.no-rules {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-muted);
  font-size: 14px;
  background: var(--bg-secondary);
  border: 1px dashed var(--border-color);
  border-radius: 12px;
}

.rules-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow-y: auto;
  max-height: 500px;
}

.rule-item {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 12px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
}

.rule-id {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  font-family: monospace;
  min-width: 26px;
  padding-top: 2px;
}

.rule-body {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  flex-wrap: wrap;
  flex: 1;
}

.rule-conditions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  align-items: center;
}

.cond-tag {
  padding: 2px 8px;
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.cond-tag.highlight {
  background: rgba(245, 158, 11, 0.15);
  border-color: rgba(245, 158, 11, 0.5);
  color: var(--accent-yellow);
}

.cond-empty {
  font-size: 12px;
  color: var(--text-muted);
  font-style: italic;
}

.rule-arrow {
  width: 12px;
  height: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
  margin-top: 4px;
}

.rule-conclusion {
  padding: 2px 8px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 8px;
  font-size: 12px;
  color: var(--accent-green);
  font-weight: 500;
}
</style>
