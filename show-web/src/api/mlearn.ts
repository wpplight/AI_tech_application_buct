/**
 * 机器学习 API 服务
 * 提供神经网络和遗传算法的 API 调用封装
 */

import { API_CONFIG, buildUrl } from './index'

// ==================== 神经网络类型 ====================

// 训练数据点
export interface TrainingDataPoint {
  input: number[]
  target: number[]
}

// 神经网络训练请求
export interface NNTrainRequest {
  input_dim: number
  hidden_dim: number
  output_dim: number
  layers: string[]
  epochs: number
  learning_rate: number
  train_data: TrainingDataPoint[]
}

// 神经网络训练结果
export interface NNTrainResult {
  success: boolean
  model_id: string
  final_loss: number
  training_time_ms: number
  metrics: {
    loss_history: number[]
  }
}

// 神经网络预测请求
export interface NNPredictRequest {
  model_id: string
  inputs: number[][]
}

// 神经网络预测结果
export interface NNPredictResult {
  success: boolean
  predictions: number[][]
}

// 模型信息
export interface ModelInfo {
  id: string
  architecture: string
  created_at: string
  final_loss: number
}

// ==================== 遗传算法类型 ====================

// 优化问题类型
export type ProblemType = 'regression' | 'classification' | 'custom'

// 基准测试函数
export type BenchmarkFunction = 'sphere' | 'rastrigin' | 'ackley' | 'rosenbrock'

// 交叉算法类型
export type CrossoverType = 'sbx' | 'single_point' | 'two_point' | 'arithmetic'

// 变异算法类型
export type MutationType = 'polynomial' | 'uniform' | 'gaussian'

// 遗传算法优化请求
export interface GAOptimizeRequest {
  problem_type: ProblemType
  objective_function: BenchmarkFunction
  dimension: number
  bounds: [number, number]
  population_size: number
  generations: number
  crossover_type: CrossoverType
  mutation_type: MutationType
  elite_protect: boolean
}

// 遗传算法优化结果
export interface GAOptimizeResult {
  success: boolean
  job_id: string
  best_solution: number[]
  best_fitness: number
  convergence: {
    generations: number
    fitness_history: number[]
    execution_time_ms: number
  }
}

// 优化任务状态
export interface GAJobStatus {
  job_id: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  progress: number
  current_best: number[]
  current_fitness: number
}

// ==================== 可视化类型 ====================

// 图表类型
export type ChartType = 'line' | 'scatter' | 'heatmap' | 'surface'

// 图表数据
export interface ChartData {
  x?: number[]
  y?: number[]
  labels?: string[]
}

// 图表选项
export interface ChartOptions {
  title?: string
  xlabel?: string
  ylabel?: string
  export_format?: 'svg' | 'png'
}

// 可视化请求
export interface VizPlotRequest {
  chart_type: ChartType
  title: string
  data: ChartData
  options?: ChartOptions
}

// 可视化结果
export interface VizPlotResult {
  success: boolean
  chart_id: string
  image_url?: string
  base64_data?: string
}

// ==================== 服务类 ====================

/**
 * 机器学习 API 服务类
 */
export class MLearnService {
  private baseUrl: string

  constructor(baseUrl?: string) {
    this.baseUrl = baseUrl || API_CONFIG.mlearn.baseUrl
  }

  // ==================== 神经网络 API ====================

  /**
   * 训练神经网络
   */
  async trainNN(request: NNTrainRequest): Promise<NNTrainResult> {
    const url = buildUrl(this.baseUrl, API_CONFIG.mlearn.endpoints.nn.train)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    })
    return response.json()
  }

  /**
   * 使用模型预测
   */
  async predictNN(request: NNPredictRequest): Promise<NNPredictResult> {
    const url = buildUrl(this.baseUrl, API_CONFIG.mlearn.endpoints.nn.predict)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    })
    return response.json()
  }

  /**
   * 获取模型列表
   */
  async getModels(): Promise<{ models: ModelInfo[] }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.mlearn.endpoints.nn.models)
    const response = await fetch(url)
    return response.json()
  }

  // ==================== 遗传算法 API ====================

  /**
   * 运行遗传算法优化
   */
  async optimizeGA(request: GAOptimizeRequest): Promise<GAOptimizeResult> {
    const url = buildUrl(this.baseUrl, API_CONFIG.mlearn.endpoints.ga.optimize)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    })
    return response.json()
  }

  /**
   * 查询优化任务状态
   */
  async getGAStatus(jobId: string): Promise<GAJobStatus> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.mlearn.endpoints.ga.status}/${jobId}`)
    const response = await fetch(url)
    return response.json()
  }

  // ==================== 可视化 API ====================

  /**
   * 生成图表
   */
  async plot(request: VizPlotRequest): Promise<VizPlotResult> {
    const url = buildUrl(this.baseUrl, API_CONFIG.mlearn.endpoints.viz.plot)
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    })
    return response.json()
  }

  /**
   * 获取图表图片
   */
  async getChartImage(chartId: string): Promise<Blob> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.mlearn.endpoints.viz.image}/${chartId}`)
    const response = await fetch(url)
    return response.blob()
  }

  // ==================== 便捷方法 ====================

  /**
   * 训练简单的 sin 函数拟合模型
   */
  async trainSinFitting(
    epochs: number = 1000,
    hiddenDim: number = 64,
    learningRate: number = 0.01
  ): Promise<NNTrainResult> {
    // 生成 sin 函数训练数据
    const trainData: TrainingDataPoint[] = []
    for (let i = 0; i < 100; i++) {
      const x = Math.random() * 2 * Math.PI
      trainData.push({
        input: [x],
        target: [Math.sin(x)]
      })
    }

    return this.trainNN({
      input_dim: 1,
      hidden_dim: hiddenDim,
      output_dim: 1,
      layers: ['Linear', 'ReLU', 'Linear'],
      epochs,
      learning_rate: learningRate,
      train_data: trainData
    })
  }

  /**
   * 运行 Sphere 函数优化
   */
  async optimizeSphere(
    dimensions: number = 2,
    generations: number = 500,
    populationSize: number = 200
  ): Promise<GAOptimizeResult> {
    return this.optimizeGA({
      problem_type: 'regression',
      objective_function: 'sphere',
      dimension: dimensions,
      bounds: [-50, 50],
      population_size: populationSize,
      generations,
      crossover_type: 'sbx',
      mutation_type: 'polynomial',
      elite_protect: true
    })
  }
}

// 导出单例
export const mlearnService = new MLearnService()

export default mlearnService
