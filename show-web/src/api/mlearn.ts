/**
 * 机器学习 API 服务
 * 对接 m-learn REST API (axum, port 8082)
 */

import { API_CONFIG } from './index'

const BASE_URL = API_CONFIG.mlearn.baseUrl

// ==================== 类型定义 ====================

export type AlgorithmType = 'regression' | 'genetic'
export type RegressionFunction = 'linear' | 'quadratic' | 'sinusoidal'
export type GeneticFunction = 'rastrigin_variant' | 'ackley'

export interface CreateTrainRequest {
  algorithm: AlgorithmType
  regression_fn?: RegressionFunction
  genetic_fn?: GeneticFunction
  learning_rate?: number
  noise?: number
}

export interface StepRequest {
  epochs: number
}

export interface TrainStatusResponse {
  task_id: string
  algorithm: AlgorithmType
  total_epochs: number
  is_running: boolean
  best_fitness: number | null
}

export interface RegressionInference {
  type: 'Regression'
  x_data: number[]
  y_data: number[]
  x_curve: number[]
  y_curve: number[]
  loss: number
}

export interface Genetic1DInference {
  type: 'Genetic1D'
  x_range: number[]
  y_true: number[]
  best_gene: number
  best_fitness: number
}

export interface Genetic2DInference {
  type: 'Genetic2D'
  x_grid: number[]
  y_grid: number[]
  fitness_grid: number[]
  best_gene_x: number
  best_gene_y: number
  best_fitness: number
}

export type InferenceResponse = RegressionInference | Genetic1DInference | Genetic2DInference

// ==================== API 函数 ====================

export async function checkHealth(): Promise<boolean> {
  try {
    const resp = await fetch(`${BASE_URL}/health`, { signal: AbortSignal.timeout(2000) })
    return resp.ok
  } catch {
    return false
  }
}

export async function getAlgorithms(): Promise<{ regression: string[], genetic: string[] }> {
  const resp = await fetch(`${BASE_URL}/algorithms`)
  return resp.json()
}

export async function createTrain(req: CreateTrainRequest): Promise<{ task_id: string }> {
  const resp = await fetch(`${BASE_URL}/train`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(req)
  })
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function trainStep(taskId: string, epochs: number): Promise<TrainStatusResponse> {
  const resp = await fetch(`${BASE_URL}/train/${taskId}/step`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ epochs })
  })
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function getTrainStatus(taskId: string): Promise<TrainStatusResponse> {
  const resp = await fetch(`${BASE_URL}/train/${taskId}/status`)
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function stopTrain(taskId: string): Promise<void> {
  const resp = await fetch(`${BASE_URL}/train/${taskId}/stop`, { method: 'POST' })
  if (!resp.ok) throw new Error(await resp.text())
}

export async function getInference(taskId: string): Promise<InferenceResponse> {
  const resp = await fetch(`${BASE_URL}/inference/${taskId}`)
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}
