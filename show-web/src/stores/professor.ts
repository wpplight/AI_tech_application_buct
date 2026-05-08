/**
 * 专家系统状态管理
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { professorService, type AlgorithmType, type Rule, type InferenceStep } from '../api/professor'

export const useProfessorStore = defineStore('professor', () => {
  // 状态
  const currentAlgorithm = ref<AlgorithmType>('incremental')
  const facts = ref<string[]>([])
  const rules = ref<Rule[]>([])
  const inferenceSteps = ref<InferenceStep[]>([])
  const persistedHistory = ref<any[]>([])
  const allFactsCount = ref(0)
  const storedForwardFacts = ref<string[]>([])
  const backwardGoal = ref('')
  const currentResult = ref<any>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const isConnected = ref(false)

  // 计算属性
  const factsCount = computed(() => facts.value.length)
  const rulesCount = computed(() => rules.value.length)
  const forwardSteps = computed(() => 
    inferenceSteps.value.filter(s => s.type === 'forward')
  )
  const backwardSteps = computed(() => 
    inferenceSteps.value.filter(s => s.type === 'backward')
  )

  const HISTORY_KEY = 'professor_inference_history'

  function loadHistoryFromStorage() {
    try {
      const stored = localStorage.getItem(HISTORY_KEY)
      return stored ? JSON.parse(stored) : []
    } catch {
      return []
    }
  }

  function saveHistoryToStorage(history: any[]) {
    try {
      localStorage.setItem(HISTORY_KEY, JSON.stringify(history))
    } catch (e) {
      console.error('保存推理历史失败:', e)
    }
  }

  function addToHistory(entry: { timestamp: number; type: 'forward' | 'backward'; facts: string[]; steps: InferenceStep[]; algorithm: AlgorithmType; result?: any; goal?: string }) {
    const history = loadHistoryFromStorage()
    history.unshift(entry)
    const trimmed = history.slice(0, 50)
    saveHistoryToStorage(trimmed)
    return trimmed
  }

  // 操作
  async function checkConnection() {
    try {
      const info = await professorService.getServerInfo()
      isConnected.value = !!info.name
    } catch {
      isConnected.value = false
    }
  }

  async function loadAllFactsCount() {
    try {
      const res = await professorService.getAllFacts(currentAlgorithm.value, 1, 1)
      allFactsCount.value = res.pagination?.total || res.total || 0
    } catch (e) {
      console.error('加载事实总数失败', e)
    }
  }

  function storeForwardFacts() {
    storedForwardFacts.value = [...facts.value]
  }

  async function switchToBackward() {
    storedForwardFacts.value = [...facts.value]
    facts.value = []
  }

  async function switchToForward() {
    facts.value = [...storedForwardFacts.value]
    if (facts.value.length > 0) {
      for (const f of facts.value) {
        await professorService.addFact(f, currentAlgorithm.value)
      }
    }
  }

  async function loadFacts() {
    try {
      loading.value = true
      error.value = null
      const response = await professorService.getFacts(currentAlgorithm.value)
      facts.value = response.facts || []
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载事实失败'
    } finally {
      loading.value = false
    }
  }

  async function loadRules() {
    try {
      loading.value = true
      error.value = null
      const response = await professorService.getRules(currentAlgorithm.value)
      rules.value = response.rules || []
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载规则失败'
    } finally {
      loading.value = false
    }
  }

  async function addFact(fact: string) {
    try {
      loading.value = true
      error.value = null
      const result = await professorService.addFact(fact, currentAlgorithm.value)
      if (result.success) {
        await loadFacts()
      }
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : '添加事实失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function removeFact(fact: string) {
    try {
      loading.value = true
      error.value = null
      const result = await professorService.deleteFact(fact, currentAlgorithm.value)
      if (result.success) {
        await loadFacts()
      }
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : '删除事实失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function clearFacts() {
    try {
      loading.value = true
      error.value = null
      const result = await professorService.clearFacts(currentAlgorithm.value)
      if (result.success) {
        facts.value = []
        inferenceSteps.value = []
      }
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : '清空事实失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function forwardInference(params?: { conditionSetId?: number; facts?: string[] }) {
    try {
      loading.value = true
      error.value = null
      let result: any
      if (params?.conditionSetId) {
        result = await professorService.forwardInferenceWithConditionSet(params.conditionSetId, currentAlgorithm.value)
        currentResult.value = result
        inferenceSteps.value = result.steps || []
        if (result.success) {
          addToHistory({
            timestamp: Date.now(),
            type: 'forward',
            facts: result.input_facts || [],
            steps: inferenceSteps.value,
            algorithm: currentAlgorithm.value,
            result
          })
        }
      } else if (params?.facts) {
        result = await professorService.forwardInferenceWithFacts(params.facts, currentAlgorithm.value)
        currentResult.value = result
        inferenceSteps.value = result.steps || []
        if (result.success) {
          addToHistory({
            timestamp: Date.now(),
            type: 'forward',
            facts: params.facts,
            steps: inferenceSteps.value,
            algorithm: currentAlgorithm.value,
            result
          })
        }
      } else {
        result = await professorService.forwardInferenceWithFacts(facts.value, currentAlgorithm.value)
        currentResult.value = result
        inferenceSteps.value = result.steps || []
        if (result.success) {
          await loadFacts()
          addToHistory({
            timestamp: Date.now(),
            type: 'forward',
            facts: [...facts.value],
            steps: inferenceSteps.value,
            algorithm: currentAlgorithm.value,
            result
          })
        }
      }
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : '推理失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function backwardInference(goal: string, conditionSetId?: number) {
    console.log('[backwardInference] called with goal:', goal, 'conditionSetId:', conditionSetId, 'currentAlgorithm:', currentAlgorithm.value)
    try {
      loading.value = true
      error.value = null
      const result = await professorService.backwardInference(goal, currentAlgorithm.value, conditionSetId)
      console.log('[backwardInference] result.input_facts:', result.input_facts, 'result.goal:', result.goal)
      currentResult.value = result
      console.log('[backwardInference] currentResult.value set:', currentResult.value)
      if (result.success) {
        inferenceSteps.value = result.steps || []
        addToHistory({
          timestamp: Date.now(),
          type: 'backward',
          facts: result.input_facts || [],
          steps: inferenceSteps.value,
          algorithm: currentAlgorithm.value,
          goal,
          result
        })
      } else {
        inferenceSteps.value = result.steps || []
      }
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : '推理失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  async function loadInferenceSteps() {
    try {
      const response = await professorService.getInferenceSteps(currentAlgorithm.value)
      inferenceSteps.value = response.steps || []
    } catch (e) {
      console.error('加载推理步骤失败:', e)
    }
  }

  async function reset() {
    try {
      loading.value = true
      error.value = null
      const result = await professorService.reset(currentAlgorithm.value)
      if (result.success) {
        facts.value = []
        inferenceSteps.value = []
      }
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : '重置失败'
      throw e
    } finally {
      loading.value = false
    }
  }

  function loadPersistedHistory() {
    persistedHistory.value = loadHistoryFromStorage()
  }

  function clearPersistedHistory() {
    persistedHistory.value = []
    localStorage.removeItem(HISTORY_KEY)
  }

  function setAlgorithm(algorithm: AlgorithmType) {
    currentAlgorithm.value = algorithm
    loadFacts()
    loadRules()
    inferenceSteps.value = []
  }

  function setFacts(factList: string[]) {
    facts.value = [...factList]
  }

  return {
    // 状态
    currentAlgorithm,
    facts,
    rules,
    inferenceSteps,
    persistedHistory,
    allFactsCount,
    storedForwardFacts,
    backwardGoal,
    currentResult,
    loading,
    error,
    isConnected,
    
    // 计算属性
    factsCount,
    rulesCount,
    forwardSteps,
    backwardSteps,
    
    // 操作
    checkConnection,
    loadFacts,
    loadRules,
    loadAllFactsCount,
    addFact,
    removeFact,
    clearFacts,
    forwardInference,
    backwardInference,
    loadInferenceSteps,
    reset,
    setAlgorithm,
    setFacts,
    loadPersistedHistory,
    clearPersistedHistory,
    switchToBackward,
    switchToForward
  }
})
