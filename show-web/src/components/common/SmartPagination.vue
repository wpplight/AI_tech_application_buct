<script setup lang="ts">
import { ref, computed } from 'vue'

const props = withDefaults(defineProps<{
  current: number
  total: number
}>(), {
  current: 1,
  total: 1
})

const emit = defineEmits<{
  change: [page: number]
}>()

const ellipsisLeft = ref(false)
const ellipsisRight = ref(false)

const pages = computed(() => {
  const total = props.total
  const cur = props.current
  if (total <= 7) {
    return Array.from({ length: total }, (_, i) => i + 1)
  }
  if (cur <= 4) {
    return [1, 2, 3, 4, 5, -1, total]
  }
  if (cur >= total - 3) {
    return [1, -1, total - 4, total - 3, total - 2, total - 1, total]
  }
  return [1, -1, cur - 1, cur, cur + 1, -1, total]
})
</script>

<template>
  <div class="pagination">
    <button
      class="page-btn"
      :disabled="current === 1"
      @click="emit('change', current - 1)"
    >上一页</button>
    <template v-for="(p, idx) in pages" :key="idx">
      <button
        v-if="p === -1"
        class="page-btn ellipsis"
        disabled
      >...</button>
      <button
        v-else
        class="page-btn"
        :class="{ active: p === current }"
        @click="emit('change', p)"
      >{{ p }}</button>
    </template>
    <button
      class="page-btn"
      :disabled="current === total"
      @click="emit('change', current + 1)"
    >下一页</button>
  </div>
</template>

<style scoped>
.pagination {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 4px;
  flex-wrap: wrap;
}

.page-btn {
  background: var(--page-bg, #18181b);
  border: 1px solid var(--page-border, #27272a);
  border-radius: 6px;
  color: var(--page-color, #a1a1aa);
  font-size: 13px;
  padding: 4px 10px;
  cursor: pointer;
  min-width: 36px;
  transition: all 0.15s;
}

.page-btn:hover:not(:disabled):not(.active) {
  background: var(--page-hover-bg, #27272a);
  color: var(--page-hover-color, #fafafa);
}

.page-btn.active {
  background: var(--page-active-bg, #3b82f6);
  border-color: var(--page-active-bg, #3b82f6);
  color: white;
}

.page-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.page-btn.ellipsis {
  border-color: transparent;
  cursor: default;
}

.page-btn.ellipsis:hover {
  background: var(--page-bg, #18181b);
  color: var(--page-color, #a1a1aa);
}
</style>
