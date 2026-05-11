/**
 * 专家系统 API 服务
 * 提供动物识别专家系统的 API 调用封装
 */

import { API_CONFIG, buildUrl, type ApiResponse } from './index'

// 算法类型
export type AlgorithmType = 'fullscan' | 'incremental' | 'rete'

// 规则类型
export interface Rule {
  id: number
  conditions: string[]
  conclusion: string
}

// 条件集类型
export interface ConditionSet {
  id: number
  name: string
  facts: string[]
  created_at: string
  updated_at: string
}

// 推理步骤类型
export interface InferenceStep {
  type: 'forward' | 'backward'
  rule_id?: number
  new_fact?: string
  iteration?: number
  rule_conditions?: string[]
  rule_conclusion?: string
  goal?: string
  result?: string
  attempt?: string
  conditions?: string[]
  step_index?: number
  accumulated_facts?: string[]
}

// Rete 网络追踪事件
export interface ReteTraceEvent {
  type: 'alpha_activate' | 'beta_match' | 'terminal_fire'
  condition?: string
  fact?: string
  children?: number
  rule_id?: number
  matched_facts?: string[]
  is_chain_head?: boolean
  combined_count?: number
  conclusion?: string
  conditions?: string[]
}

// Rete 网络拓扑
export interface ReteTopology {
  alpha_nodes: Array<{
    condition: string
    memory: string[]
    children_count: number
  }>
  beta_nodes: Array<{
    rule_id: number
    condition: string
    is_chain_head: boolean
    completed_count: number
    pending_count: number
    children_count: number
  }>
  terminals: Array<{
    rule_id: number
    conclusion: string
    conditions: string[]
    fired: boolean
  }>
  stats: NetworkStats
}

// 正向推理结果
export interface ForwardInferenceResult {
  success: boolean
  new_facts: string[]
  all_facts: string[]
  steps: InferenceStep[]
  algorithm: AlgorithmType
  rete_trace?: ReteTraceEvent[]
  network_topology?: ReteTopology
  condition_set_id?: number
  cache_hit?: boolean
  input_facts?: string[]
}

// 反向推理结果
export interface BackwardInferenceResult {
  success: boolean
  goal: string
  condition_set_id: number | null
  input_facts: string[]
  steps: any[]
  missing_facts: string[]
  goal_already_known: boolean
  algorithm: AlgorithmType
}

// Rete 网络统计
export interface NetworkStats {
  alpha_nodes: number
  beta_nodes: number
  terminal_nodes?: number
  terminals?: number
  alpha_memory_size?: number
  beta_memory_size?: number
  build_time_us?: number
  total_rules?: number
}

// 算法状态
export interface AlgorithmStatus {
  rules_count: number
  facts_count: number
}

/**
 * 专家系统 API 服务类
 */
export class ProfessorService {
  private baseUrl: string

  constructor(baseUrl?: string) {
    this.baseUrl = baseUrl || API_CONFIG.professor.baseUrl
  }

  /**
   * 获取服务器信息
   */
  async getServerInfo(): Promise<ApiResponse> {
    const response = await fetch(buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.info))
    return response.json()
  }

  /**
   * 获取所有算法状态
   */
  async getAlgorithmsStatus(): Promise<{ algorithms: Record<AlgorithmType, AlgorithmStatus> }> {
    const response = await fetch(buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.algorithms))
    return response.json()
  }

  /**
   * 获取知识库规则（支持分页）
   */
  async getRulesPaginated(
    algorithm: AlgorithmType = 'fullscan',
    page: number = 1,
    limit: number = 1000
  ): Promise<{ rules: Rule[]; algorithm: AlgorithmType; pagination?: { page: number; limit: number; total: number; total_pages: number } }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.rules, { algo: algorithm, page: page, limit: limit })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 获取知识库规则（默认获取全部）
   */
  async getRules(algorithm: AlgorithmType = 'fullscan'): Promise<{ rules: Rule[]; algorithm: AlgorithmType }> {
    const response = await this.getRulesPaginated(algorithm, 1, 1000)
    return response
  }

  async searchRulesByFact(
    fact: string,
    algorithm: AlgorithmType = 'fullscan'
  ): Promise<{ rules: Rule[]; algorithm: AlgorithmType; total: number }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.rules, { algo: algorithm, fact })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 添加规则
   */
  async addRule(
    conditions: string[],
    conclusion: string,
    algorithm: AlgorithmType = 'fullscan'
  ): Promise<{ success: boolean; rule_id?: number; error?: string }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.rules, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ conditions, conclusion })
    })
    return response.json()
  }

  /**
   * 删除规则
   */
  async deleteRule(ruleId: number, algorithm: AlgorithmType = 'fullscan'): Promise<{ success: boolean }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.rules}/${ruleId}`, { algo: algorithm })
    const response = await fetch(url, {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' }
    })
    return response.json()
  }

  /**
   * 获取当前工作内存中的事实
   */
  async getFacts(algorithm: AlgorithmType = 'fullscan'): Promise<{ facts: string[]; algorithm: AlgorithmType }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.facts, { algo: algorithm, working: 'true' })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 获取知识库可用事实（支持分页）
   */
  async getAllFacts(
    algorithm: AlgorithmType = 'fullscan',
    page: number = 1,
    limit: number = 1000
  ): Promise<{ facts: string[]; algorithm: AlgorithmType; total?: number; pagination?: { page: number; limit: number; total: number; total_pages: number } }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.facts, { algo: algorithm, page, limit })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 搜索事实（模糊搜索）
   */
  async searchFacts(query: string, algorithm: AlgorithmType = 'fullscan'): Promise<{ facts: string[]; algorithm: AlgorithmType; search?: string; total?: number }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.facts, { algo: algorithm, search: query })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 添加事实
   */
  async addFact(fact: string, algorithm: AlgorithmType = 'fullscan'): Promise<{ success: boolean }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.facts}/add`, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ fact })
    })
    return response.json()
  }

  /**
   * 删除事实
   */
  async deleteFact(fact: string, algorithm: AlgorithmType = 'fullscan'): Promise<{ success: boolean }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.facts}/delete`, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ fact })
    })
    return response.json()
  }

  /**
   * 清空事实库
   */
  async clearFacts(algorithm: AlgorithmType = 'fullscan'): Promise<{ success: boolean }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.facts}/clear`, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' }
    })
    return response.json()
  }

  /**
   * 执行正向推理（无状态，传入条件集 ID）
   */
  async forwardInferenceWithConditionSet(
    conditionSetId: number,
    algorithm: AlgorithmType = 'fullscan'
  ): Promise<ForwardInferenceResult & { condition_set_id: number; cache_hit: boolean }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.inference}/forward`, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ condition_set_id: conditionSetId })
    })
    return response.json()
  }

  /**
   * 执行正向推理（无状态，直接传入 facts 列表）
   */
  async forwardInferenceWithFacts(
    facts: string[],
    algorithm: AlgorithmType = 'fullscan'
  ): Promise<ForwardInferenceResult> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.inference}/forward`, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ facts })
    })
    return response.json()
  }

  /**
   * 执行反向推理
   */
  async backwardInference(
    goal: string,
    algorithm: AlgorithmType = 'fullscan',
    conditionSetId?: number
  ): Promise<BackwardInferenceResult> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.inference}/backward`, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ goal, condition_set_id: conditionSetId ?? null })
    })
    return response.json()
  }

  /**
   * 获取推理步骤
   */
  async getInferenceSteps(algorithm: AlgorithmType = 'fullscan'): Promise<{ steps: InferenceStep[]; algorithm: AlgorithmType }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.inference}/steps`, { algo: algorithm })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 重置系统
   */
  async reset(algorithm: AlgorithmType = 'fullscan'): Promise<{ success: boolean }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.reset, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' }
    })
    return response.json()
  }

  /**
   * 获取 Rete 网络统计
   */
  async getNetworkStats(): Promise<NetworkStats> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.network}/stats`, { algo: 'rete' })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 获取 Rete 执行追踪
   */
  async getNetworkTrace(): Promise<{ trace: ReteTraceEvent[] }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.network}/trace`, { algo: 'rete' })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 获取 Rete 网络拓扑
   */
  async getNetworkTopology(): Promise<ReteTopology> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.network}/topology`, { algo: 'rete' })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 获取条件集列表
   */
  async getConditionSets(
    algorithm: AlgorithmType = 'fullscan',
    page: number = 1,
    limit: number = 50
  ): Promise<{ condition_sets: ConditionSet[]; total: number; page: number; limit: number; total_pages: number }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.conditionSets, { algo: algorithm, page, limit })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 获取单个条件集
   */
  async getConditionSet(id: number, algorithm: AlgorithmType = 'fullscan'): Promise<{ condition_set: ConditionSet }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.conditionSets}/${id}`, { algo: algorithm })
    const response = await fetch(url)
    return response.json()
  }

  /**
   * 创建条件集
   */
  async createConditionSet(
    name: string,
    facts: string[] = [],
    algorithm: AlgorithmType = 'fullscan'
  ): Promise<{ success: boolean; condition_set: ConditionSet }> {
    const url = buildUrl(this.baseUrl, API_CONFIG.professor.endpoints.conditionSets, { algo: algorithm })
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, facts })
    })
    return response.json()
  }

  /**
   * 更新条件集
   */
  async updateConditionSet(
    id: number,
    data: { name?: string; facts?: string[] },
    algorithm: AlgorithmType = 'fullscan'
  ): Promise<{ success: boolean; condition_set: ConditionSet }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.conditionSets}/${id}`, { algo: algorithm })
    const response = await fetch(url, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data)
    })
    return response.json()
  }

  /**
   * 删除条件集
   */
  async deleteConditionSet(id: number, algorithm: AlgorithmType = 'fullscan'): Promise<{ success: boolean }> {
    const url = buildUrl(this.baseUrl, `${API_CONFIG.professor.endpoints.conditionSets}/${id}`, { algo: algorithm })
    const response = await fetch(url, { method: 'DELETE' })
    return response.json()
  }
}

// 导出单例
export const professorService = new ProfessorService()

// 导出默认服务实例
export default professorService
