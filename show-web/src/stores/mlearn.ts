import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  checkHealth,
  createTrain,
  trainStep,
  getTrainStatus,
  stopTrain,
  getInference,
  listTasks,
  getTrainingHistory,
  getRecall,
  type CreateTrainRequest,
  type TrainStatusResponse,
  type InferenceResponse,
  type RegressionInference,
  type Genetic1DInference,
  type Genetic2DInference,
  type AlgorithmType,
  type RegressionFunction,
  type GeneticFunction,
  type TrainingHistory,
  type RecallResponse,
  type Objective,
  type GeneticParams
} from '../api/mlearn'

export interface TaskItem {
  id: string
  algorithm: AlgorithmType
  fn: RegressionFunction | GeneticFunction
  label: string
  createdAt: number
}

const regressionLabels: Record<RegressionFunction, string> = {
  linear: '线性 y=2x+1',
  quadratic: '二次 y=x²',
  sinusoidal: '正弦 y=sin(x)'
}

const geneticLabels: Record<GeneticFunction, string> = {
  ackley: 'Ackley (1D)',
  rastrigin_variant: 'Rastrigin 变体 (2D)'
}

export const useMLearnStore = defineStore('mlearn', () => {
  const isConnected = ref(false)
  const tasks = ref<TaskItem[]>([])
  const currentTaskId = ref<string | null>(null)
  const taskStatus = ref<TrainStatusResponse | null>(null)
  const inferenceData = ref<InferenceResponse | null>(null)
  const isTraining = ref(false)
  const error = ref<string | null>(null)
  const lossHistory = ref<number[]>([])
  const trainingHistory = ref<TrainingHistory | null>(null)
  const recallData = ref<RecallResponse | null>(null)
  const epochsPerStep = ref(10)

  const currentTask = computed(() => tasks.value.find(t => t.id === currentTaskId.value) ?? null)
  const hasTask = computed(() => currentTaskId.value !== null)
  const totalEpochs = computed(() => taskStatus.value?.total_epochs ?? 0)
  const bestFitness = computed(() => taskStatus.value?.best_fitness ?? null)
  const isRegression = computed(() => taskStatus.value?.algorithm === 'regression')
  const isGenetic = computed(() => taskStatus.value?.algorithm === 'genetic')

  const regressionData = computed((): RegressionInference | null => {
    if (inferenceData.value?.type === 'Regression') return inferenceData.value
    return null
  })

  const genetic1DData = computed((): Genetic1DInference | null => {
    if (inferenceData.value?.type === 'Genetic1D') return inferenceData.value
    return null
  })

  const genetic2DData = computed((): Genetic2DInference | null => {
    if (inferenceData.value?.type === 'Genetic2D') return inferenceData.value
    return null
  })

  async function checkConnection() {
    isConnected.value = await checkHealth()
  }

  async function fetchTasks() {
    try {
      const resp = await listTasks()
      const backendTasks = resp.tasks.map(t => {
        const fn = t.algorithm === 'regression'
          ? (t.regression_fn ?? 'linear')
          : (t.genetic_fn ?? 'ackley')
        const label = t.algorithm === 'regression'
          ? regressionLabels[fn as RegressionFunction]
          : geneticLabels[fn as GeneticFunction]
        return {
          id: t.task_id,
          algorithm: t.algorithm,
          fn: fn as RegressionFunction & GeneticFunction,
          label,
          createdAt: t.created_at * 1000
        } as TaskItem
      })
      const localIds = new Set(tasks.value.map(t => t.id))
      for (const bt of backendTasks) {
        if (!localIds.has(bt.id)) {
          tasks.value.push(bt)
        }
      }
      const backendIds = new Set(backendTasks.map(t => t.id))
      tasks.value = tasks.value.filter(t => backendIds.has(t.id))
    } catch (e) {
      error.value = e instanceof Error ? e.message : '同步任务列表失败'
    }
  }

  async function createTask(opts: {
    algorithm: AlgorithmType
    regressionFn?: RegressionFunction
    geneticFn?: GeneticFunction
    learningRate?: number
    noise?: number
    xMin?: number
    xMax?: number
    minValue?: number
    maxValue?: number
    objective?: Objective
    geneticParams?: GeneticParams
  }): Promise<string | null> {
    try {
      error.value = null
      const req: CreateTrainRequest = {
        algorithm: opts.algorithm,
        learning_rate: opts.learningRate ?? 0.01,
        noise: opts.noise ?? 0.1
      }
      if (opts.xMin !== undefined) req.x_min = opts.xMin
      if (opts.xMax !== undefined) req.x_max = opts.xMax
      if (opts.minValue !== undefined) req.min_value = opts.minValue
      if (opts.maxValue !== undefined) req.max_value = opts.maxValue
      if (opts.objective !== undefined) req.objective = opts.objective
      if (opts.geneticParams !== undefined) req.genetic_params = opts.geneticParams
      if (opts.algorithm === 'regression') {
        req.regression_fn = opts.regressionFn ?? 'linear'
      } else {
        req.genetic_fn = opts.geneticFn ?? 'ackley'
      }
      const { task_id } = await createTrain(req)
      const fn = opts.algorithm === 'regression'
        ? (opts.regressionFn ?? 'linear')
        : (opts.geneticFn ?? 'ackley')
      const label = opts.algorithm === 'regression'
        ? regressionLabels[fn as RegressionFunction]
        : geneticLabels[fn as GeneticFunction]

      tasks.value.unshift({
        id: task_id,
        algorithm: opts.algorithm,
        fn: fn as RegressionFunction & GeneticFunction,
        label,
        createdAt: Date.now()
      })
      return task_id
    } catch (e) {
      error.value = e instanceof Error ? e.message : '创建任务失败'
      return null
    }
  }

  async function selectTask(taskId: string) {
    currentTaskId.value = taskId
    lossHistory.value = []
    trainingHistory.value = null
    recallData.value = null
    inferenceData.value = null
    taskStatus.value = null
    await fetchStatus()
  }

  async function fetchStatus() {
    if (!currentTaskId.value) return
    try {
      taskStatus.value = await getTrainStatus(currentTaskId.value)
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取状态失败'
    }
  }

  const MAX_LOSS_HISTORY = 500

  function appendLoss(val: number) {
    const next = lossHistory.value.length >= MAX_LOSS_HISTORY
      ? [...lossHistory.value.slice(-MAX_LOSS_HISTORY / 2), val]
      : [...lossHistory.value, val]
    lossHistory.value = next
  }

  async function doStep() {
    if (!currentTaskId.value) return
    try {
      isTraining.value = true
      error.value = null
      const status = await trainStep(currentTaskId.value, epochsPerStep.value)
      taskStatus.value = status
      if (status.best_fitness !== null) {
        appendLoss(status.best_fitness)
      }
      if (currentTask.value?.algorithm === 'regression') {
        await fetchHistory()
        await fetchRecall()
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '训练失败'
    } finally {
      isTraining.value = false
    }
  }

  async function doMultiStep(steps: number) {
    if (!currentTaskId.value) return
    const isReg = currentTask.value?.algorithm === 'regression'
    try {
      isTraining.value = true
      error.value = null
      for (let i = 0; i < steps; i++) {
        if (!currentTaskId.value) break
        try {
          const status = await trainStep(currentTaskId.value, epochsPerStep.value)
          taskStatus.value = status
          if (status.best_fitness !== null) {
            appendLoss(status.best_fitness)
          }
          if (isReg) {
            await fetchHistory()
            await fetchRecall()
          }
        } catch (innerErr) {
          error.value = innerErr instanceof Error ? innerErr.message : '训练中断'
          break
        }
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '训练失败'
    } finally {
      isTraining.value = false
    }
  }

  async function fetchInference() {
    if (!currentTaskId.value) return
    try {
      inferenceData.value = await getInference(currentTaskId.value)
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取推理数据失败'
    }
  }

  async function fetchHistory() {
    if (!currentTaskId.value) return
    try {
      trainingHistory.value = await getTrainingHistory(currentTaskId.value)
    } catch (e) {
      console.warn('fetchHistory failed:', e instanceof Error ? e.message : e)
    }
  }

  async function fetchRecall() {
    if (!currentTaskId.value) return
    try {
      recallData.value = await getRecall(currentTaskId.value)
    } catch (e) {
      console.warn('fetchRecall failed:', e instanceof Error ? e.message : e)
    }
  }

  async function removeTask(taskId?: string) {
    const id = taskId ?? currentTaskId.value
    if (!id) return
    try {
      await stopTrain(id)
    } catch {}
    tasks.value = tasks.value.filter(t => t.id !== id)
    if (currentTaskId.value === id) {
      currentTaskId.value = null
      taskStatus.value = null
      inferenceData.value = null
      lossHistory.value = []
      trainingHistory.value = null
      recallData.value = null
    }
    error.value = null
  }

  function clearError() {
    error.value = null
  }

  return {
    isConnected,
    tasks,
    currentTaskId,
    currentTask,
    taskStatus,
    inferenceData,
    isTraining,
    error,
    lossHistory,
    trainingHistory,
    recallData,
    epochsPerStep,
    hasTask,
    totalEpochs,
    bestFitness,
    isRegression,
    isGenetic,
    regressionData,
    genetic1DData,
    genetic2DData,
    checkConnection,
    fetchTasks,
    createTask,
    selectTask,
    fetchStatus,
    doStep,
    doMultiStep,
    fetchInference,
    fetchHistory,
    fetchRecall,
    removeTask,
    clearError
  }
})
