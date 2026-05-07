<script setup lang="ts">
defineProps<{
  label: string
  value: string | number
  unit?: string
  icon?: string
  trend?: 'up' | 'down' | 'neutral'
  trendValue?: string
}>()
</script>

<template>
  <div class="stat-card">
    <div class="stat-header">
      <span class="stat-label">{{ label }}</span>
      <div v-if="trend && trendValue" class="stat-trend" :class="[`trend-${trend}`]">
        <svg v-if="trend === 'up'" class="trend-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/>
          <polyline points="17 6 23 6 23 12"/>
        </svg>
        <svg v-if="trend === 'down'" class="trend-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="23 18 13.5 8.5 8.5 13.5 1 6"/>
          <polyline points="17 18 23 18 23 12"/>
        </svg>
        <span>{{ trendValue }}</span>
      </div>
    </div>
    <div class="stat-body">
      <span class="stat-value">{{ value }}</span>
      <span v-if="unit" class="stat-unit">{{ unit }}</span>
    </div>
  </div>
</template>

<style scoped>
.stat-card {
  background: #27272a;
  border: 1px solid #3f3f46;
  border-radius: 16px;
  padding: 20px;
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}

.stat-card:hover {
  border-color: #52525b;
  transform: translateY(-2px);
}

.stat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.stat-label {
  font-size: 13px;
  color: #a1a1aa;
  font-weight: 500;
}

.stat-trend {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  font-weight: 500;
}

.trend-up {
  color: #10b981;
}

.trend-down {
  color: #f43f5e;
}

.trend-neutral {
  color: #a1a1aa;
}

.trend-icon {
  width: 14px;
  height: 14px;
}

.stat-body {
  display: flex;
  align-items: baseline;
  gap: 4px;
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  color: #fafafa;
  letter-spacing: -0.02em;
  font-feature-settings: 'tnum' 'lnum';
}

.stat-unit {
  font-size: 14px;
  color: #a1a1aa;
  font-weight: 500;
}
</style>
