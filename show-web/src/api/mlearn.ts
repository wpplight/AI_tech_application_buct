/**
 * 机器学习 API 服务
 * 对接 m-learn REST API (axum, port 8082)
 */

import { API_CONFIG } from './index'

const BASE_URL = API_CONFIG.mlearn.baseUrl
const TIMEOUT = 15000

function withTimeout(init?: RequestInit): RequestInit {
  return { ...init, signal: AbortSignal.timeout(TIMEOUT) }
}

// ==================== 类型定义 ====================

export type AlgorithmType = 'regression' | 'genetic'
export type RegressionFunction = 'linear' | 'quadratic' | 'sinusoidal'
export type GeneticFunction = 'rastrigin_variant' | 'ackley'
export type Objective = 'minimize' | 'maximize'

export interface GeneticParams {
  population_size?: number
  tournament_size?: number
  elite_count?: number
  elite_protect?: boolean
  mutation_rate?: number
  sbx_eta?: number
}

export interface CreateTrainRequest {
  algorithm: AlgorithmType
  regression_fn?: RegressionFunction
  genetic_fn?: GeneticFunction
  learning_rate?: number
  noise?: number
  x_min?: number
  x_max?: number
  min_value?: number
  max_value?: number
  objective?: Objective
  genetic_params?: GeneticParams
}

export interface EpochRecord {
  epoch: number
  train_loss: number
  val_loss: number
}

export interface TrainingHistory {
  task_id: string
  records: EpochRecord[]
}

export interface RecallResponse {
  y_true: number[]
  y_pred: number[]
  x_val: number[]
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
  x_curve: number[]
  y_curve: number[]
  x_min: number
  x_max: number
}

export interface Genetic1DInference {
  type: 'Genetic1D'
  x_range: number[]
  y_true: number[]
  best_gene: number
  best_fitness: number
  population_x: number[]
}

export interface Genetic2DInference {
  type: 'Genetic2D'
  x_grid: number[]
  y_grid: number[]
  fitness_grid: number[]
  best_gene_x: number
  best_gene_y: number
  best_fitness: number
  population_x: number[]
  population_y: number[]
  population_fitness: number[]
}

export type InferenceResponse = RegressionInference | Genetic1DInference | Genetic2DInference

export interface TaskListItem {
  task_id: string
  algorithm: AlgorithmType
  regression_fn: RegressionFunction | null
  genetic_fn: GeneticFunction | null
  total_epochs: number
  best_fitness: number | null
  created_at: number
}

export interface TaskListResponse {
  tasks: TaskListItem[]
}

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
  const resp = await fetch(`${BASE_URL}/algorithms`, withTimeout())
  return resp.json()
}

export async function listTasks(): Promise<TaskListResponse> {
  const resp = await fetch(`${BASE_URL}/train`, withTimeout())
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function createTrain(req: CreateTrainRequest): Promise<{ task_id: string }> {
  const resp = await fetch(`${BASE_URL}/train`, withTimeout({
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(req)
  }))
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function trainStep(taskId: string, epochs: number): Promise<TrainStatusResponse> {
  const resp = await fetch(`${BASE_URL}/train/${taskId}/step`, withTimeout({
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ epochs })
  }))
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function getTrainStatus(taskId: string): Promise<TrainStatusResponse> {
  const resp = await fetch(`${BASE_URL}/train/${taskId}/status`, withTimeout())
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function stopTrain(taskId: string): Promise<void> {
  const resp = await fetch(`${BASE_URL}/train/${taskId}/stop`, withTimeout({ method: 'POST' }))
  if (!resp.ok) throw new Error(await resp.text())
}

export async function getInference(taskId: string): Promise<InferenceResponse> {
  const resp = await fetch(`${BASE_URL}/inference/${taskId}`, withTimeout())
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function getTrainingHistory(taskId: string): Promise<TrainingHistory> {
  const resp = await fetch(`${BASE_URL}/train/${taskId}/history`, withTimeout())
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}

export async function getRecall(taskId: string): Promise<RecallResponse> {
  const resp = await fetch(`${BASE_URL}/train/${taskId}/recall`, withTimeout())
  if (!resp.ok) throw new Error(await resp.text())
  return resp.json()
}
