<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useProfessorStore } from '../../stores/professor'
import { professorService } from '../../api/professor'
import SmartPagination from '../common/SmartPagination.vue'

const store = useProfessorStore()

const facts = ref<string[]>([])
const totalFacts = ref(0)
const currentPage = ref(1)
const pageSize = ref(100)
const loading = ref(false)
const error = ref<string | null>(null)

async function loadFacts() {
  loading.value = true
  error.value = null
  try {
    const response = await professorService.getAllFacts(store.currentAlgorithm, currentPage.value, pageSize.value)
    facts.value = response.facts || []
    totalFacts.value = response.pagination?.total || response.total || 0
  } catch (e) {
    error.value = e instanceof Error ? e.message : '加载事实失败'
  } finally {
    loading.value = false
  }
}

const totalPages = computed(() => Math.ceil(totalFacts.value / pageSize.value))

const factsByCategory = computed(() => {
  const categories: Record<string, string[]> = {}
  facts.value.forEach(fact => {
    const firstChar = fact.charAt(0)
    if (!categories[firstChar]) {
      categories[firstChar] = []
    }
    categories[firstChar].push(fact)
  })
  return categories
})

const sortedCategories = computed(() => Object.keys(factsByCategory.value).sort())

const visiblePages = computed(() => {
  const pages: number[] = []
  for (let i = 1; i <= totalPages.value; i++) pages.push(i)
  return pages
})

function goPage(p: number) {
  currentPage.value = p
  loadFacts()
}

onMounted(loadFacts)
</script>

<template>
  <div class="fact-list-panel">
    <div class="panel-header">
      <h3 class="panel-title">事实库</h3>
      <span class="fact-count">共 {{ totalFacts }} 个</span>
    </div>

    <div v-if="loading" class="loading">加载中...</div>
    <div v-else-if="error" class="error">{{ error }}</div>

    <template v-else>
      <div class="facts-container">
        <div v-for="char in sortedCategories" :key="char" class="fact-group">
          <div class="group-header">{{ char }}</div>
          <div class="group-items">
            <span
              v-for="fact in factsByCategory[char]"
              :key="fact"
              class="fact-item"
              :class="{ selected: store.facts.includes(fact) }"
            >
              {{ fact }}
            </span>
          </div>
        </div>
      </div>

      <SmartPagination
        v-if="totalPages > 1"
        :current="currentPage"
        :total="totalPages"
        @change="goPage"
      />
    </template>
  </div>
</template>

<style scoped>
.fact-list-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.panel-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.fact-count {
  font-size: 12px;
  color: var(--text-muted);
}

.loading,
.error {
  padding: 40px;
  text-align: center;
  color: var(--text-muted);
}

.error { color: var(--accent-red); }

.facts-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  max-height: 400px;
  overflow-y: auto;
}

.fact-group { display: flex; flex-direction: column; gap: 8px; }

.group-header {
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-blue);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.group-items { display: flex; flex-wrap: wrap; gap: 6px; }

.fact-item {
  padding: 6px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: default;
  transition: all 0.2s;
}

.fact-item.selected {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--accent-green);
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

.page-btn:hover { background: var(--bg-tertiary); color: var(--text-primary); }
.page-btn.active { background: var(--border-color); border-color: var(--text-muted); color: var(--text-primary); }
.page-btn:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
