import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import { professorService, type ConditionSet } from '../api/professor'

export const useConditionSetStore = defineStore('conditionSet', () => {
  const sets = ref<ConditionSet[]>([])
  const selectedSetId = ref<number | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const total = ref(0)
  const currentPage = ref(1)
  const totalPages = ref(1)

  const selectedSet = computed(() =>
    sets.value.find(s => s.id === selectedSetId.value) ?? null
  )

  async function loadSets(page = 1) {
    loading.value = true
    error.value = null
    try {
      const res = await professorService.getConditionSets('fullscan', page, 20)
      sets.value = res.condition_sets || []
      total.value = res.total
      currentPage.value = res.page
      totalPages.value = res.total_pages
    } catch (e: any) {
      error.value = e.message || '加载条件集失败'
    } finally {
      loading.value = false
    }
  }

  async function createSet(name: string, facts: string[] = []) {
    loading.value = true
    error.value = null
    try {
      const res = await professorService.createConditionSet(name, facts)
      if (res.success && res.condition_set) {
        sets.value.unshift(res.condition_set)
        total.value++
        return res.condition_set
      }
      throw new Error(res.error || '创建失败')
    } catch (e: any) {
      error.value = e.message || '创建条件集失败'
      return null
    } finally {
      loading.value = false
    }
  }

  async function updateSet(id: number, data: { name?: string; facts?: string[] }) {
    loading.value = true
    error.value = null
    try {
      const res = await professorService.updateConditionSet(id, data)
      if (res.success && res.condition_set) {
        const idx = sets.value.findIndex(s => s.id === id)
        if (idx !== -1) {
          sets.value[idx] = res.condition_set
        }
        return res.condition_set
      }
      throw new Error(res.error || '更新失败')
    } catch (e: any) {
      error.value = e.message || '更新条件集失败'
      return null
    } finally {
      loading.value = false
    }
  }

  async function deleteSet(id: number) {
    loading.value = true
    error.value = null
    try {
      const res = await professorService.deleteConditionSet(id)
      if (res.success) {
        sets.value = sets.value.filter(s => s.id !== id)
        total.value--
        if (selectedSetId.value === id) {
          selectedSetId.value = null
        }
        return true
      }
      return false
    } catch (e: any) {
      error.value = e.message || '删除条件集失败'
      return false
    } finally {
      loading.value = false
    }
  }

  function selectSet(id: number | null) {
    selectedSetId.value = id
  }

  return {
    sets,
    selectedSetId,
    selectedSet,
    loading,
    error,
    total,
    currentPage,
    totalPages,
    loadSets,
    createSet,
    updateSet,
    deleteSet,
    selectSet
  }
})
