<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useConditionSetStore } from '../../stores/conditionSet'
import type { ConditionSet } from '../../api/professor'
import ConditionSetEditor from './ConditionSetEditor.vue'

const store = useConditionSetStore()

const editingSet = ref<ConditionSet | null>(null)
const creatingNew = ref(false)
const deletingId = ref<number | null>(null)

onMounted(() => {
  store.loadSets()
})

function startCreate() {
  creatingNew.value = true
  editingSet.value = null
}

function startEdit(cs: ConditionSet) {
  editingSet.value = { ...cs }
  creatingNew.value = false
}

function cancelEdit() {
  editingSet.value = null
  creatingNew.value = false
}

async function handleSaved(cs: ConditionSet) {
  if (creatingNew.value) {
    const result = await store.createSet(cs.name, cs.facts)
    if (result) {
      creatingNew.value = false
      editingSet.value = null
    }
  } else if (editingSet.value?.id) {
    const result = await store.updateSet(editingSet.value.id, { name: cs.name, facts: cs.facts })
    if (result) {
      editingSet.value = null
    }
  }
}

async function handleDelete(id: number) {
  deletingId.value = id
  await store.deleteSet(id)
  deletingId.value = null
}

function formatDate(dateStr: string) {
  if (!dateStr) return ''
  return dateStr.replace('T', ' ').slice(0, 16)
}
</script>

<template>
  <div class="cs-manager">
    <div class="manager-header">
      <div class="header-info">
        <h2 class="page-title">条件集管理</h2>
        <span class="cs-count">{{ store.total }} 个条件集</span>
      </div>
      <button class="btn-create" @click="startCreate">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="12" y1="5" x2="12" y2="19"/>
          <line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
        新建条件集
      </button>
    </div>

    <div v-if="store.loading && store.sets.length === 0" class="loading-state">
      加载中...
    </div>

    <div v-else-if="store.error" class="error-state">
      {{ store.error }}
    </div>

    <div v-else class="manager-body">
      <div v-if="creatingNew || editingSet" class="editor-area">
        <ConditionSetEditor
          :condition-set="editingSet"
          :mode="creatingNew ? 'create' : 'edit'"
          @saved="handleSaved"
          @cancelled="cancelEdit"
        />
      </div>

      <div v-if="store.sets.length === 0 && !creatingNew && !editingSet" class="empty-state">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <rect x="3" y="3" width="18" height="18" rx="3"/>
          <path d="M3 9h18"/>
          <path d="M9 21V9"/>
        </svg>
        <p>暂无条件集</p>
        <p class="empty-hint">点击上方按钮创建一个条件集</p>
      </div>

      <div v-else class="cs-grid">
        <div
          v-for="cs in store.sets"
          :key="cs.id"
          class="cs-card"
          :class="{ 'editing': editingSet?.id === cs.id }"
        >
          <div v-if="editingSet?.id === cs.id" class="editor-inline">
            <ConditionSetEditor
              :condition-set="editingSet"
              mode="edit"
              @saved="handleSaved"
              @cancelled="cancelEdit"
            />
          </div>

          <template v-else>
            <div class="card-header">
              <h3 class="cs-name">{{ cs.name }}</h3>
              <div class="card-actions">
                <button class="action-btn edit" @click="startEdit(cs)" title="编辑">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                    <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                  </svg>
                </button>
                <button
                  class="action-btn delete"
                  :disabled="deletingId === cs.id"
                  @click="handleDelete(cs.id)"
                  title="删除"
                >
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="3 6 5 6 21 6"/>
                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                  </svg>
                </button>
              </div>
            </div>

            <div class="card-body">
              <div class="facts-preview">
                <span
                  v-for="fact in cs.facts.slice(0, 6)"
                  :key="fact"
                  class="fact-chip"
                >
                  {{ fact }}
                </span>
                <span v-if="cs.facts.length > 6" class="fact-more">
                  +{{ cs.facts.length - 6 }}
                </span>
              </div>
              <div v-if="cs.facts.length === 0" class="no-facts">无条件</div>
            </div>

            <div class="card-footer">
              <span class="cs-date">更新于 {{ formatDate(cs.updated_at) }}</span>
              <span class="cs-facts-count">{{ cs.facts.length }} 个条件</span>
            </div>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.cs-manager {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 20px;
}

.manager-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 12px;
}

.header-info {
  display: flex;
  align-items: baseline;
  gap: 12px;
}

.page-title {
  font-size: 20px;
  font-weight: 700;
  margin: 0;
  color: var(--text-primary);
}

.cs-count {
  font-size: 13px;
  color: var(--text-muted);
  background: var(--bg-tertiary);
  padding: 2px 10px;
  border-radius: 20px;
}

.btn-create {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: var(--accent-blue);
  border: none;
  border-radius: 10px;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-create:hover {
  opacity: 0.9;
  transform: translateY(-1px);
}

.btn-create svg {
  width: 14px;
  height: 14px;
}

.manager-body {
  flex: 1;
  overflow-y: auto;
}

.editor-area {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 20px;
}

.loading-state,
.error-state {
  text-align: center;
  padding: 60px;
  color: var(--text-muted);
  font-size: 14px;
}

.error-state {
  color: var(--accent-red);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  gap: 12px;
  color: var(--text-muted);
}

.empty-state svg {
  width: 48px;
  height: 48px;
  opacity: 0.4;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

.empty-hint {
  font-size: 12px;
  opacity: 0.7;
}

.cs-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 16px;
}

.cs-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
  overflow: hidden;
  transition: all 0.2s;
}

.cs-card:hover {
  border-color: var(--text-muted);
}

.cs-card.editing {
  border-color: var(--accent-blue);
}

.card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 16px 16px 12px;
  gap: 8px;
}

.cs-name {
  font-size: 15px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
  word-break: break-word;
}

.card-actions {
  display: flex;
  gap: 4px;
  flex-shrink: 0;
}

.action-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  color: var(--text-muted);
}

.action-btn svg {
  width: 14px;
  height: 14px;
}

.action-btn.edit:hover {
  background: var(--bg-tertiary);
  color: var(--accent-blue);
  border-color: var(--accent-blue);
}

.action-btn.delete:hover:not(:disabled) {
  background: rgba(239, 68, 68, 0.1);
  color: var(--accent-red);
  border-color: var(--accent-red);
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.card-body {
  padding: 0 16px 12px;
}

.facts-preview {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.fact-chip {
  padding: 3px 10px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  font-size: 12px;
  color: var(--text-secondary);
}

.fact-more {
  padding: 3px 10px;
  background: transparent;
  font-size: 12px;
  color: var(--text-muted);
}

.no-facts {
  font-size: 12px;
  color: var(--text-muted);
  font-style: italic;
}

.card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 16px;
  border-top: 1px solid var(--border-color);
}

.cs-date {
  font-size: 11px;
  color: var(--text-muted);
}

.cs-facts-count {
  font-size: 11px;
  color: var(--accent-blue);
  background: rgba(59, 130, 246, 0.1);
  padding: 2px 8px;
  border-radius: 10px;
}

.editor-inline {
  padding: 16px;
}

.editor-inline :deep(.cs-editor) {
  gap: 0;
}

.editor-inline :deep(.editor-header) {
  margin-bottom: 12px;
  padding-bottom: 12px;
}

.editor-inline :deep(.editor-title) {
  font-size: 14px;
}
</style>
