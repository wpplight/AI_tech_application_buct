/**
 * 机器学习状态管理
 * 支持神经网络和遗传算法
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { mlearnService, type NNTrainResult, type GAOptimizeResult, type ModelInfo } from '../api/mlearn'

export const useMLearnStore = defineStore('mlearn', () => {
  // ==================== 神经网络状态 ====================
  const nnLoading = ref(false)
  const nnError = ref<string | null>(null)
  const currentModel = ref<NNTrainResult | null>(null)
  const models = ref<ModelInfo[]>([])
  const lossHistory = ref<number[]>([])

  // NN 配置
  const nnConfig = ref({
    inputDim: 1,
    hiddenDim: 64,
    outputDim: 1,
    epochs: 1000,
    learningRate: 0.01
  })

  // 计算属性
  const nnTrainingProgress = computed(() => {
    if (!currentModel.value?.metrics?.loss_history) return 0
    return lossHistory.value.length
  })

  const finalLoss = computed(() => {
    if (!currentModel.value) return 0
    return currentModel.value.final_loss
  })

  const trainingTime = computed(() => {
    if (!currentModel.value) return 0
    return currentModel.value.training_time_ms
  })

  // ==================== 遗传算法状态 ====================
  const gaLoading = ref(false)
  const gaError = ref<string | null>(null)
  const gaResult = ref<GAOptimizeResult | null>(null)
  const fitnessHistory = ref<number[]>([])
  const isOptimizing = ref(false)

  // GA 配置
  const gaConfig = ref({
    problemType: 'regression' as const,
    benchmarkFunction: 'sphere' as const,
    dimensions: 2,
    bounds: [-50, 50] as [number, number],
    populationSize: 200,
    generations: 500,
    crossoverType: 'sbx' as const,
    mutationType: 'polynomial' as const,
    eliteProtect: true
  })

  // 计算属性
  const gaProgress = computed(() => {
    if (!gaResult.value?.convergence?.fitness_history) return 0
    return fitnessHistory.value.length
  })

  const bestFitness = computed(() => {
    if (!gaResult.value) return Infinity
    return gaResult.value.best_fitness
  })

  const bestSolution = computed(() => {
    if (!gaResult.value) return []
    return gaResult.value.best_solution
  })

  // ==================== 连接状态 ====================
  const isConnected = ref(false)

  // ==================== 神经网络操作 ====================
  async function checkConnection() {
    try {
      await mlearnService.trainNN({
        input_dim: 1,
        hidden_dim: 4,
        output_dim: 1,
        layers: ['Linear'],
        epochs: 1,
        learning_rate: 0.01,
        train_data: [{ input: [0], target: [0] }]
      })
      isConnected.value = true
    } catch {
      isConnected.value = false
    }
  }

  async function trainNN() {
    try {
      nnLoading.value = true
      nnError.value = null
      lossHistory.value = []

      // 生成训练数据 (sin 函数)
      const trainData = []
      for (let i = 0; i < 100; i++) {
        const x = (i / 100) * 2 * Math.PI
        trainData.push({
          input: [x],
          target: [Math.sin(x)]
        })
      }

      const result = await mlearnService.trainNN({
        input_dim: nnConfig.value.inputDim,
        hidden_dim: nnConfig.value.hiddenDim,
        output_dim: nnConfig.value.outputDim,
        layers: ['Linear', 'ReLU', 'Linear'],
        epochs: nnConfig.value.epochs,
        learning_rate: nnConfig.value.learningRate,
        train_data: trainData
      })

      if (result.success) {
        currentModel.value = result
        lossHistory.value = result.metrics?.loss_history || []
      }

      return result
    } catch (e) {
      nnError.value = e instanceof Error ? e.message : '训练失败'
      throw e
    } finally {
      nnLoading.value = false
    }
  }

  async function loadModels() {
    try {
      const result = await mlearnService.getModels()
      models.value = result.models || []
    } catch (e) {
      console.error('加载模型失败:', e)
    }
  }

  function setNNConfig(config: Partial<typeof nnConfig.value>) {
    nnConfig.value = { ...nnConfig.value, ...config }
  }

  // ==================== 遗传算法操作 ====================
  async function optimizeGA() {
    try {
      gaLoading.value = true
      gaError.value = null
      fitnessHistory.value = []
      isOptimizing.value = true

      const result = await mlearnService.optimizeGA({
        problem_type: gaConfig.value.problemType,
        objective_function: gaConfig.value.benchmarkFunction,
        dimension: gaConfig.value.dimensions,
        bounds: gaConfig.value.bounds,
        population_size: gaConfig.value.populationSize,
        generations: gaConfig.value.generations,
        crossover_type: gaConfig.value.crossoverType,
        mutation_type: gaConfig.value.mutationType,
        elite_protect: gaConfig.value.eliteProtect
      })

      if (result.success) {
        gaResult.value = result
        fitnessHistory.value = result.convergence?.fitness_history || []
      }

      return result
    } catch (e) {
      gaError.value = e instanceof Error ? e.message : '优化失败'
      throw e
    } finally {
      gaLoading.value = false
      isOptimizing.value = false
    }
  }

  function setGAConfig(config: Partial<typeof gaConfig.value>) {
    gaConfig.value = { ...gaConfig.value, ...config }
  }

  function resetGA() {
    gaResult.value = null
    fitnessHistory.value = []
  }

  function resetNN() {
    currentModel.value = null
    lossHistory.value = []
  }

  return {
    // 神经网络状态
    nnLoading,
    nnError,
    currentModel,
    models,
    lossHistory,
    nnConfig,
    nnTrainingProgress,
    finalLoss,
    trainingTime,
    trainNN,
    loadModels,
    setNNConfig,
    resetNN,

    // 遗传算法状态
    gaLoading,
    gaError,
    gaResult,
    fitnessHistory,
    isOptimizing,
    gaConfig,
    gaProgress,
    bestFitness,
    bestSolution,
    optimizeGA,
    setGAConfig,
    resetGA,

    // 连接状态
    isConnected,
    checkConnection
  }
})
