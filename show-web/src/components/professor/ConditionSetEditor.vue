<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { professorService, type ConditionSet } from '../../api/professor'
import { useProfessorStore } from '../../stores/professor'

const props = defineProps<{
  conditionSet?: ConditionSet | null
  mode: 'create' | 'edit'
}>()

const emit = defineEmits<{
  saved: [ConditionSet]
  cancelled: []
}>()

const professorStore = useProfessorStore()

const name = ref(props.conditionSet?.name ?? '')
const facts = ref<string[]>([...(props.conditionSet?.facts ?? [])])
const saving = ref(false)
const error = ref<string | null>(null)

const allFacts = ref<string[]>([])
const searchQuery = ref('')
const isLoadingFacts = ref(false)
const isBlurring = ref(false)
const inputWrapRef = ref<HTMLElement>()

const filteredFacts = computed(() => {
  if (!searchQuery.value) return []
  const q = searchQuery.value.toLowerCase()
  return allFacts.value
    .filter(f => f.toLowerCase().includes(q) && !facts.value.includes(f))
    .slice(0, 10)
})

const showDropdown = computed(() =>
  !isBlurring.value && searchQuery.value.length > 0 && filteredFacts.value.length > 0
)

const dropdownStyle = computed(() => {
  if (!inputWrapRef.value) return {}
  const rect = inputWrapRef.value.getBoundingClientRect()
  return {
    position: 'fixed' as const,
    top: `${rect.bottom + 4}px`,
    left: `${rect.left}px`,
    width: `${rect.width + 80}px`,
    zIndex: 9999,
  }
})

watch(() => props.conditionSet, (cs) => {
  name.value = cs?.name ?? ''
  facts.value = [...(cs?.facts ?? [])]
})

onMounted(async () => {
  isLoadingFacts.value = true
  try {
    const res = await professorService.getAllFacts(professorStore.currentAlgorithm, 1, 1000)
    allFacts.value = res.facts || []
  } catch {
    console.error('加载事实库失败')
  } finally {
    isLoadingFacts.value = false
  }
})

function addFact(fact: string) {
  const trimmed = fact.trim()
  if (trimmed && !facts.value.includes(trimmed)) {
    facts.value.push(trimmed)
  }
  searchQuery.value = ''
}

function removeFact(fact: string) {
  facts.value = facts.value.filter(f => f !== fact)
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    e.preventDefault()
    if (searchQuery.value.trim()) {
      addFact(searchQuery.value)
    }
  }
  if (e.key === 'Escape') {
    searchQuery.value = ''
  }
}

function handleBlur() {
  isBlurring.value = true
  setTimeout(() => { isBlurring.value = false }, 200)
}

async function handleSave() {
  if (!name.value.trim()) {
    error.value = '请输入条件集名称'
    return
  }
  saving.value = true
  error.value = null
  try {
    emit('saved', { id: props.conditionSet?.id ?? 0, name: name.value.trim(), facts: facts.value, created_at: '', updated_at: '' } as ConditionSet)
  } catch (e: any) {
    error.value = e.message
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <div class="cs-editor">
    <div class="editor-header">
      <h3 class="editor-title">{{ mode === 'create' ? '新建条件集' : '编辑条件集' }}</h3>
    </div>

    <div class="editor-body">
      <div class="form-group">
        <label class="form-label">条件集名称</label>
        <input
          v-model="name"
          type="text"
          class="form-input"
          placeholder="例如：哺乳动物识别组合"
          maxlength="50"
        />
      </div>

      <div class="form-group">
        <label class="form-label">条件事实</label>

        <div class="selected-tags">
          <div v-if="facts.length === 0" class="empty-hint">尚未添加条件</div>
          <div
            v-for="fact in facts"
            :key="fact"
            class="fact-tag"
          >
            <span>{{ fact }}</span>
            <button @click="removeFact(fact)">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
              </svg>
            </button>
          </div>
        </div>

        <div class="search-section">
          <div ref="inputWrapRef" class="search-input-wrap">
            <input
              v-model="searchQuery"
              type="text"
              class="search-input"
              placeholder="搜索或输入条件名称..."
              @keydown="handleKeydown"
              @blur="handleBlur"
            />
          </div>
          <div
            v-if="showDropdown"
            class="search-dropdown"
            :style="dropdownStyle"
          >
            <template v-if="filteredFacts.length > 0">
              <button
                v-for="fact in filteredFacts"
                :key="fact"
                class="dropdown-item"
                @mousedown.prevent="addFact(fact)"
              >
                {{ fact }}
              </button>
            </template>
            <div v-else class="no-match-hint">没有匹配的事实</div>
          </div>
          <button
            class="btn-add"
            :disabled="!searchQuery.trim()"
            @click="addFact(searchQuery)"
          >
            添加
          </button>
        </div>

        <div v-if="isLoadingFacts" class="loading-hint">加载事实库中...</div>
        <div v-else-if="allFacts.length > 0 && !searchQuery" class="quick-facts">
          <span class="quick-label">快速添加：</span>
          <button
            v-for="fact in allFacts.slice(0, 12)"
            :key="fact"
            class="quick-btn"
            :disabled="facts.includes(fact)"
            @click="addFact(fact)"
          >
            {{ fact }}
          </button>
        </div>
      </div>

      <div v-if="error" class="error-msg">{{ error }}</div>
    </div>

    <div class="editor-footer">
      <button class="btn-cancel" @click="emit('cancelled')">取消</button>
      <button
        class="btn-save"
        :disabled="saving || !name.trim()"
        @click="handleSave"
      >
        {{ saving ? '保存中...' : '保存' }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.cs-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.editor-header {
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
  margin-bottom: 20px;
}

.editor-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.editor-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 20px;
  overflow-y: auto;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.form-input {
  padding: 10px 14px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
}

.form-input:focus {
  border-color: var(--accent-blue);
}

.form-input::placeholder {
  color: var(--text-muted);
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  min-height: 48px;
  padding: 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  align-items: center;
}

.empty-hint {
  color: var(--text-muted);
  font-size: 13px;
}

.fact-tag {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 5px 10px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 18px;
  font-size: 13px;
  color: var(--accent-green);
}

.fact-tag button {
  padding: 1px;
  background: transparent;
  border: none;
  color: currentColor;
  cursor: pointer;
  opacity: 0.6;
  transition: all 0.2s;
  display: flex;
  align-items: center;
}

.fact-tag button:hover {
  opacity: 1;
  transform: rotate(90deg);
}

.fact-tag button svg {
  width: 10px;
  height: 10px;
}

.search-section {
  display: flex;
  gap: 8px;
  position: relative;
}

.search-input-wrap {
  flex: 1;
  position: relative;
}

.search-input {
  width: 100%;
  padding: 10px 14px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
  box-sizing: border-box;
}

.search-input:focus {
  border-color: var(--accent-blue);
}

.search-input::placeholder {
  color: var(--text-muted);
}

.search-dropdown {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  overflow: hidden;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
}

.no-match-hint {
  padding: 12px 14px;
  color: var(--text-muted);
  font-size: 13px;
  text-align: center;
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

.btn-add {
  padding: 10px 16px;
  background: var(--accent-blue);
  border: none;
  border-radius: 10px;
  color: white;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.btn-add:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-add:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.loading-hint {
  font-size: 12px;
  color: var(--text-muted);
  text-align: center;
  padding: 8px;
}

.quick-facts {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  align-items: center;
  padding: 8px 0;
}

.quick-label {
  font-size: 12px;
  color: var(--text-muted);
  flex-shrink: 0;
}

.quick-btn {
  padding: 4px 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.quick-btn:hover:not(:disabled) {
  background: var(--border-color);
  color: var(--text-primary);
}

.quick-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.error-msg {
  color: var(--accent-red);
  font-size: 13px;
  padding: 8px 12px;
  background: rgba(239, 68, 68, 0.1);
  border-radius: 8px;
}

.editor-footer {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
  padding-top: 20px;
  border-top: 1px solid var(--border-color);
  margin-top: 16px;
}

.btn-cancel {
  padding: 10px 20px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 10px;
  color: var(--text-secondary);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-cancel:hover {
  border-color: var(--text-muted);
  color: var(--text-primary);
}

.btn-save {
  padding: 10px 24px;
  background: var(--accent-blue);
  border: none;
  border-radius: 10px;
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-save:hover:not(:disabled) {
  opacity: 0.9;
}

.btn-save:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
