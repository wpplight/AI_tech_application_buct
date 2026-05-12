/**
 * 机器学习状态管理
 * 对接 m-learn REST API (step-based training)
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  checkHealth,
  createTrain,
  trainStep,
  getTrainStatus,
  stopTrain,
  getInference,
  type CreateTrainRequest,
  type TrainStatusResponse,
  type InferenceResponse,
  type RegressionInference,
  type Genetic1DInference,
  type Genetic2DInference,
  type AlgorithmType,
  type RegressionFunction,
  type GeneticFunction
} from '../api/mlearn'

export const useMLearnStore = defineStore('mlearn', () => {
  const isConnected = ref(false)
  const currentTaskId = ref<string | null>(null)
  const taskStatus = ref<TrainStatusResponse | null>(null)
  const inferenceData = ref<InferenceResponse | null>(null)
  const isTraining = ref(false)
  const error = ref<string | null>(null)
  const lossHistory = ref<number[]>([])

  const algorithm = ref<AlgorithmType>('regression')
  const regressionFn = ref<RegressionFunction>('linear')
  const geneticFn = ref<GeneticFunction>('ackley')
  const learningRate = ref(0.01)
  const noise = ref(0.1)
  const epochsPerStep = ref(10)

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

  async function createTask() {
    try {
      error.value = null
      const req: CreateTrainRequest = {
        algorithm: algorithm.value,
        learning_rate: learningRate.value,
        noise: noise.value
      }
      if (algorithm.value === 'regression') {
        req.regression_fn = regressionFn.value
      } else {
        req.genetic_fn = geneticFn.value
      }
      const { task_id } = await createTrain(req)
      currentTaskId.value = task_id
      lossHistory.value = []
      inferenceData.value = null
      await fetchStatus()
    } catch (e) {
      error.value = e instanceof Error ? e.message : '创建任务失败'
    }
  }

  async function fetchStatus() {
    if (!currentTaskId.value) return
    try {
      taskStatus.value = await getTrainStatus(currentTaskId.value)
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取状态失败'
    }
  }

  async function doStep() {
    if (!currentTaskId.value) return
    try {
      isTraining.value = true
      error.value = null
      const status = await trainStep(currentTaskId.value, epochsPerStep.value)
      taskStatus.value = status
      if (status.best_fitness !== null) {
        lossHistory.value.push(status.best_fitness)
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '训练失败'
    } finally {
      isTraining.value = false
    }
  }

  async function doMultiStep(steps: number) {
    if (!currentTaskId.value) return
    try {
      isTraining.value = true
      error.value = null
      for (let i = 0; i < steps; i++) {
        const status = await trainStep(currentTaskId.value, epochsPerStep.value)
        taskStatus.value = status
        if (status.best_fitness !== null) {
          lossHistory.value.push(status.best_fitness)
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

  async function removeTask() {
    if (!currentTaskId.value) return
    try {
      await stopTrain(currentTaskId.value)
    } catch {}
    currentTaskId.value = null
    taskStatus.value = null
    inferenceData.value = null
    lossHistory.value = []
    error.value = null
  }

  function clearError() {
    error.value = null
  }

  return {
    isConnected,
    currentTaskId,
    taskStatus,
    inferenceData,
    isTraining,
    error,
    lossHistory,
    algorithm,
    regressionFn,
    geneticFn,
    learningRate,
    noise,
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
    createTask,
    fetchStatus,
    doStep,
    doMultiStep,
    fetchInference,
    removeTask,
    clearError
  }
})
