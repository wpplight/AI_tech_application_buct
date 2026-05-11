<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useProfessorStore } from '../../stores/professor'
import { useConditionSetStore } from '../../stores/conditionSet'

const props = defineProps<{
  mode?: 'forward' | 'backward'
}>()

const emit = defineEmits<{
  'execute': []
}>()

const professorStore = useProfessorStore()
const csStore = useConditionSetStore()

const isBackward = ref(props.mode === 'backward')
const inputMode = ref<'set' | 'direct'>('set')

const directFactsInput = ref('')
const directFacts = ref<string[]>([])

const goalInput = ref('')
const goalQuery = ref('')
const isBlurringGoal = ref(false)
const allFacts = ref<string[]>([])
const isLoadingFacts = ref(false)

const filteredGoalFacts = computed(() => {
  if (!goalQuery.value) return []
  const q = goalQuery.value.toLowerCase()
  return allFacts.value.filter(f => f.toLowerCase().includes(q)).slice(0, 8)
})

const showGoalDropdown = computed(() =>
  !isBlurringGoal.value && goalQuery.value.length > 0 && filteredGoalFacts.value.length > 0
)

const quickExamples = [
  { label: '绿海龟', facts: ['变温', '脊椎动物', '鳞片', '有鳞片', '龟甲', '前肢扁平如桨', '海生', '绿海龟'] },
  { label: '眼镜蛇', facts: ['变温', '脊椎动物', '鳞片', '体表光滑', '吐舌分叉', '有鳞片', '体表鳞片', '无四肢', '毒腺', '毒蛇', '管牙', '眼镜蛇'] },
  { label: '科莫多巨蜥', facts: ['变温', '脊椎动物', '鳞片', '体表光滑', '吐舌分叉', '有鳞片', '体表鳞片', '有四肢', '眼睑可动', '主动捕食', '分叉舌', '巨蜥类', '科莫多巨蜥'] }
]

watch(() => props.mode, (m) => {
  isBackward.value = m === 'backward'
})

watch(goalInput, (val) => {
  professorStore.backwardGoal = val
  goalQuery.value = val
})

function addDirectFacts() {
  const raw = directFactsInput.value.trim()
  if (!raw) return
  const newFacts = raw.split(/[,，\n]+/).map(f => f.trim()).filter(f => f && !directFacts.value.includes(f))
  directFacts.value.push(...newFacts)
  directFactsInput.value = ''
}

function removeDirectFact(idx: number) {
  directFacts.value.splice(idx, 1)
}

function clearDirectFacts() {
  directFacts.value = []
  directFactsInput.value = ''
}

function applyQuickExample(example: typeof quickExamples[0]) {
  directFacts.value = [...example.facts]
  inputMode.value = 'direct'
}

async function loadFacts() {
  if (allFacts.value.length > 0) return
  isLoadingFacts.value = true
  try {
    const { professorService } = await import('../../api/professor')
    const res = await professorService.getAllFacts(professorStore.currentAlgorithm, 1, 1000)
    allFacts.value = res.facts || []
  } catch (e) {
    console.error('加载事实库失败', e)
  } finally {
    isLoadingFacts.value = false
  }
}

watch(isBackward, (bw) => {
  if (bw) {
    loadFacts()
    if (csStore.selectedSetId) {
      goalInput.value = professorStore.backwardGoal
    }
  } else {
    goalInput.value = ''
    goalQuery.value = ''
  }
})

function selectGoal(fact: string) {
  goalInput.value = fact
  goalQuery.value = ''
  professorStore.backwardGoal = fact
}

function handleGoalBlur() {
  isBlurringGoal.value = true
  setTimeout(() => { isBlurringGoal.value = false }, 200)
}

function selectSet(csId: number) {
  csStore.selectSet(csId)
}

function handleExecute() {
  emit('execute')
}

defineExpose({ directFacts, inputMode })
</script>

<template>
  <div class="condition-panel">
    <div class="panel-header">
      <h3 class="panel-title">{{ isBackward ? '反向推理' : '条件集事实' }}</h3>
      <button
        v-if="!isBackward && csStore.selectedSet"
        class="btn-switch"
        @click="isBackward = true; loadFacts()"
      >
        切换反向
      </button>
      <button
        v-if="isBackward"
        class="btn-switch"
        @click="isBackward = false"
      >
        切换正向
      </button>
    </div>

    <div v-if="isBackward" class="backward-area">
      <div class="form-group">
        <label class="form-label">目标结论</label>
        <div class="goal-input-wrap">
          <input
            v-model="goalInput"
            type="text"
            class="goal-input"
            placeholder="输入目标结论，模糊搜索..."
            @blur="handleGoalBlur"
            @focus="loadFacts(); showGoalDropdown"
          />
          <div v-if="showGoalDropdown" class="goal-dropdown">
            <button
              v-for="fact in filteredGoalFacts"
              :key="fact"
              class="dropdown-item"
              @mousedown.prevent="selectGoal(fact)"
            >
              {{ fact }}
            </button>
          </div>
        </div>
      </div>

      <div class="form-group">
        <label class="form-label">条件集（已知前提）</label>
        <div class="cs-cards">
          <button
            v-for="cs in csStore.sets"
            :key="cs.id"
            class="cs-card-btn"
            :class="{ selected: csStore.selectedSetId === cs.id }"
            @click="selectSet(cs.id)"
          >
            <span class="cs-card-name">{{ cs.name }}</span>
            <span class="cs-card-count">{{ cs.facts.length }} 个条件</span>
          </button>
        </div>
        <div v-if="csStore.sets.length === 0" class="no-sets-hint">暂无条件集，请先去条件集管理创建</div>
      </div>

      <div v-if="csStore.selectedSet && goalInput" class="preview-row">
        <span class="preview-label">条件集「{{ csStore.selectedSet.name }}」可推导出</span>
        <span class="preview-goal">{{ goalInput }}</span>
        <span class="preview-label">吗？</span>
      </div>
    </div>

    <div v-else class="forward-area">
      <div class="input-mode-switch">
        <button
          class="mode-btn"
          :class="{ active: inputMode === 'set' }"
          @click="inputMode = 'set'"
        >
          条件集
        </button>
        <button
          class="mode-btn"
          :class="{ active: inputMode === 'direct' }"
          @click="inputMode = 'direct'"
        >
          直接输入
        </button>
      </div>

      <div v-if="inputMode === 'set'" class="set-mode">
        <div v-if="!csStore.selectedSet" class="empty-state">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <polygon points="12 2 2 7 12 12 22 7 12 2"/>
            <polyline points="2 17 12 22 22 17"/>
            <polyline points="2 12 12 17 22 12"/>
          </svg>
          <p>从上方选择一个条件集</p>
          <p class="hint-sub">点击卡片自动填入条件</p>
        </div>
        <div v-else class="facts-display">
          <div class="selected-set-info">
            <span class="set-name">{{ csStore.selectedSet.name }}</span>
            <span class="set-count">{{ csStore.selectedSet.facts.length }} 个条件</span>
          </div>
          <div class="facts-list">
            <div
              v-for="fact in csStore.selectedSet.facts"
              :key="fact"
              class="fact-tag"
            >
              {{ fact }}
            </div>
          </div>
        </div>
      </div>

      <div v-else class="direct-mode">
        <div class="quick-examples">
          <span class="examples-label">快速示例：</span>
          <button
            v-for="ex in quickExamples"
            :key="ex.label"
            class="example-btn"
            @click="applyQuickExample(ex)"
          >
            {{ ex.label }}
          </button>
        </div>

        <div class="direct-input-area">
          <textarea
            v-model="directFactsInput"
            class="direct-textarea"
            placeholder="输入事实，用逗号或换行分隔&#10;例如：有毛发, 食肉, 黄褐色"
            rows="3"
            @keydown.ctrl.enter="addDirectFacts"
          ></textarea>
          <div class="direct-actions">
            <button class="btn-add-facts" @click="addDirectFacts">添加事实</button>
            <button class="btn-clear-facts" @click="clearDirectFacts">清空</button>
          </div>
        </div>

        <div v-if="directFacts.length > 0" class="direct-facts-display">
          <div class="direct-facts-header">
            <span class="direct-facts-title">已输入事实</span>
            <span class="direct-facts-count">{{ directFacts.length }} 个</span>
          </div>
          <div class="facts-list">
            <div
              v-for="(fact, idx) in directFacts"
              :key="idx"
              class="fact-tag removable"
              @click="removeDirectFact(idx)"
            >
              {{ fact }}
              <span class="fact-remove">×</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.condition-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.panel-title {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.btn-switch {
  padding: 4px 12px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-switch:hover {
  border-color: var(--accent-purple);
  color: var(--accent-purple);
}

.backward-area {
  display: flex;
  flex-direction: column;
  gap: 16px;
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

.goal-input-wrap {
  position: relative;
}

.goal-input {
  width: 100%;
  padding: 10px 14px;
  background: var(--bg-secondary);
  border: 1px solid rgba(168, 85, 247, 0.4);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  box-sizing: border-box;
  transition: border-color 0.2s;
}

.goal-input:focus {
  border-color: var(--accent-purple);
}

.goal-input::placeholder {
  color: var(--text-muted);
}

.goal-dropdown {
  position: absolute;
  top: calc(100% + 4px);
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

.cs-cards {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.cs-card-btn {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 3px;
  padding: 8px 14px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
  min-width: 120px;
  text-align: left;
}

.cs-card-btn:hover {
  border-color: var(--accent-purple);
  background: var(--bg-tertiary);
}

.cs-card-btn.selected {
  border-color: var(--accent-purple);
  background: rgba(168, 85, 247, 0.1);
}

.cs-card-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
}

.cs-card-count {
  font-size: 11px;
  color: var(--text-muted);
}

.cs-card-btn.selected .cs-card-name {
  color: var(--accent-purple);
}

.no-sets-hint {
  font-size: 13px;
  color: var(--text-muted);
  padding: 8px 0;
}

.preview-row {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 6px;
  padding: 12px 14px;
  background: rgba(168, 85, 247, 0.08);
  border: 1px solid rgba(168, 85, 247, 0.2);
  border-radius: 10px;
}

.preview-label {
  font-size: 13px;
  color: var(--text-muted);
}

.preview-goal {
  font-size: 14px;
  font-weight: 600;
  color: var(--accent-purple);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 10px;
  color: var(--text-muted);
  text-align: center;
  background: var(--bg-secondary);
  border: 1px dashed var(--border-color);
  border-radius: 12px;
}

.empty-state svg {
  width: 40px;
  height: 40px;
  opacity: 0.4;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

.hint-sub {
  font-size: 12px;
  opacity: 0.7;
}

.facts-display {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.selected-set-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.set-name {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.set-count {
  font-size: 12px;
  color: var(--accent-blue);
  background: rgba(59, 130, 246, 0.1);
  padding: 2px 8px;
  border-radius: 10px;
}

.facts-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding: 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
}

.fact-tag {
  padding: 6px 12px;
  background: rgba(16, 185, 129, 0.1);
  border: 1px solid rgba(16, 185, 129, 0.3);
  border-radius: 18px;
  font-size: 13px;
  color: var(--accent-green);
}

.fact-tag.removable {
  cursor: pointer;
  transition: all 0.2s;
}

.fact-tag.removable:hover {
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.3);
  color: #ef4444;
}

.fact-remove {
  margin-left: 4px;
  font-weight: 600;
  opacity: 0.6;
}

.forward-area {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.input-mode-switch {
  display: flex;
  gap: 4px;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 10px;
  padding: 4px;
}

.mode-btn {
  flex: 1;
  padding: 6px 12px;
  background: transparent;
  border: none;
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.mode-btn:hover {
  color: var(--text-primary);
}

.mode-btn.active {
  background: var(--accent-purple);
  color: white;
}

.direct-mode {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.quick-examples {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.examples-label {
  font-size: 12px;
  color: var(--text-muted);
}

.example-btn {
  padding: 4px 12px;
  background: rgba(168, 85, 247, 0.08);
  border: 1px solid rgba(168, 85, 247, 0.3);
  border-radius: 16px;
  color: var(--accent-purple);
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.example-btn:hover {
  background: rgba(168, 85, 247, 0.15);
}

.direct-input-area {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.direct-textarea {
  width: 100%;
  padding: 10px 14px;
  background: var(--bg-secondary);
  border: 1px solid rgba(168, 85, 247, 0.4);
  border-radius: 10px;
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  resize: vertical;
  min-height: 60px;
  box-sizing: border-box;
  font-family: inherit;
  transition: border-color 0.2s;
}

.direct-textarea:focus {
  border-color: var(--accent-purple);
}

.direct-textarea::placeholder {
  color: var(--text-muted);
}

.direct-actions {
  display: flex;
  gap: 8px;
}

.btn-add-facts {
  padding: 6px 16px;
  background: var(--accent-purple);
  border: none;
  border-radius: 8px;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-add-facts:hover {
  opacity: 0.9;
}

.btn-clear-facts {
  padding: 6px 16px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-clear-facts:hover {
  border-color: var(--text-muted);
  color: var(--text-primary);
}

.direct-facts-display {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.direct-facts-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.direct-facts-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.direct-facts-count {
  font-size: 11px;
  color: var(--accent-blue);
  background: rgba(59, 130, 246, 0.1);
  padding: 1px 8px;
  border-radius: 10px;
}

.set-mode {
  display: flex;
  flex-direction: column;
}
</style>
