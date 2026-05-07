<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useProfessorStore } from '../../stores/professor'
import { professorService } from '../../api/professor'
import type { AlgorithmType } from '../../api/professor'

const store = useProfessorStore()

const searchQuery = ref('')
const searchResults = ref<string[]>([])
const isSearching = ref(false)
const showDropdown = ref(false)

watch(searchQuery, async (query) => {
  if (query.trim().length > 0) {
    isSearching.value = true
    showDropdown.value = true
    try {
      const response = await professorService.searchFacts(query, store.currentAlgorithm)
      searchResults.value = response.facts || []
    } catch (e) {
      searchResults.value = []
    } finally {
      isSearching.value = false
    }
  } else {
    searchResults.value = []
    showDropdown.value = false
  }
})

function selectFact(fact: string) {
  if (!store.facts.includes(fact)) {
    store.addFact(fact)
  }
  searchQuery.value = ''
  showDropdown.value = false
}

function clearSearch() {
  searchQuery.value = ''
  searchResults.value = []
  showDropdown.value = false
}

function handleFocus() {
  if (searchQuery.value.length > 0) {
    showDropdown.value = true
  }
}

function handleBlur() {
  setTimeout(() => {
    showDropdown.value = false
  }, 200)
}
</script>

<template>
  <div class="fact-search-panel">
    <div class="search-container">
      <div class="search-box">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8"/>
          <path d="m21 21-4.35-4.35"/>
        </svg>
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索事实..."
          class="search-input"
          @focus="handleFocus"
          @blur="handleBlur"
        />
        <button v-if="searchQuery" class="clear-btn" @click="clearSearch">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="18" y1="6" x2="6" y2="18"/>
            <line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div v-if="showDropdown && searchResults.length > 0" class="search-dropdown">
        <div class="dropdown-header">
            <span class="dropdown-title">搜索结果</span>
            <span class="dropdown-count">{{ searchResults.length }} 个匹配</span>
          </div>
        <div class="dropdown-list">
            <button
              v-for="(fact, index) in searchResults.slice(0, 10)"
              :key="index"
              class="dropdown-item"
              :class="{ selected: store.facts.includes(fact) }"
              @mousedown.prevent="selectFact(fact)"
            >
              <span class="fact-name">{{ fact }}</span>
              <span v-if="store.facts.includes(fact)" class="fact-badge">已添加</span>
            </button>
            <div v-if="searchResults.length > 10" class="dropdown-more">
              还有 {{ searchResults.length - 10 }} 个结果...
            </div>
          </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.fact-search-panel {
  width: 100%;
}

.search-container {
  position: relative;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  transition: all 0.2s;
}

.search-box:focus-within {
  border-color: var(--accent-blue);
}

.search-icon {
  width: 20px;
  height: 20px;
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

.clear-btn {
  padding: 4px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  transition: color 0.2s;
  display: flex;
  align-items: center;
}

.clear-btn:hover {
  color: var(--text-secondary);
}

.clear-btn svg {
  width: 16px;
  height: 16px;
}

.search-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 8px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  z-index: 100;
  overflow: hidden;
}

.dropdown-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-tertiary);
}

.dropdown-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.dropdown-count {
  font-size: 11px;
  color: var(--text-muted);
}

.dropdown-list {
  max-height: 300px;
  overflow-y: auto;
}

.dropdown-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: 12px 16px;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: background 0.2s;
  text-align: left;
}

.dropdown-item:hover {
  background: var(--bg-tertiary);
}

.dropdown-item.selected {
  background: rgba(16, 185, 129, 0.05);
  color: var(--accent-green);
}

.fact-name {
  flex: 1;
}

.fact-badge {
  font-size: 10px;
  padding: 2px 8px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 12px;
  color: var(--accent-green);
}

.dropdown-more {
  padding: 12px 16px;
  font-size: 12px;
  color: var(--text-muted);
  text-align: center;
  border-top: 1px solid var(--border-color);
}
</style>
