/**
 * API 基础配置
 * 集中管理所有后端服务的 URL 配置
 */

// 后端服务地址配置
export const API_CONFIG = {
  // 专家系统 - 统一服务器 (支持 fullscan/incremental/rete)
  professor: {
    baseUrl: 'http://localhost:8080',
    endpoints: {
      info: '/',
      algorithms: '/api/algorithms',
      rules: '/api/rules',
      facts: '/api/facts',
      inference: '/api/inference',
      network: '/api/network',
      reset: '/api/reset',
      conditionSets: '/api/condition-sets'
    }
  },

  // 寻路算法服务 (Gin REST API)
  wayfind: {
    baseUrl: 'http://localhost:8081',
    endpoints: {
      health: '/api/v1/health',
      algorithms: '/api/v1/algorithms',
      tasks: '/api/v1/tasks',
      map: '/api/v1/map',
      search: '/api/v1/search',
      maps: '/api/v1/maps'
    }
  },

  // 机器学习服务 (神经网络 + 遗传算法)
  mlearn: {
    baseUrl: 'http://localhost:8081',
    endpoints: {
      nn: {
        train: '/api/ml/nn/train',
        predict: '/api/ml/nn/predict',
        models: '/api/ml/nn/models'
      },
      ga: {
        optimize: '/api/ml/ga/optimize',
        status: '/api/ml/ga/status'
      },
      viz: {
        plot: '/api/ml/viz/plot',
        image: '/api/ml/viz/image'
      }
    }
  }
} as const

// API 响应类型定义
export interface ApiResponse<T = any> {
  success: boolean
  data?: T
  error?: string
  [key: string]: any
}

// HTTP 方法类型
export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'

// 请求配置类型
export interface RequestConfig {
  method?: HttpMethod
  headers?: Record<string, string>
  body?: any
  params?: Record<string, string | number>
}

/**
 * 构建完整的 API URL
 */
export function buildUrl(baseUrl: string, endpoint: string, params?: Record<string, string | number>): string {
  const url = new URL(endpoint, baseUrl)
  
  if (params) {
    Object.entries(params).forEach(([key, value]) => {
      url.searchParams.append(key, String(value))
    })
  }
  
  return url.toString()
}

/**
 * 检查服务是否可用
 */
export async function checkServiceHealth(baseUrl: string): Promise<boolean> {
  try {
    const response = await fetch(baseUrl, {
      method: 'HEAD',
      signal: AbortSignal.timeout(2000)
    })
    return response.ok
  } catch {
    return false
  }
}

export default API_CONFIG
