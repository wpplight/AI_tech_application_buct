<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useTheme } from '../../stores/theme'

const route = useRoute()
const router = useRouter()
const { currentTheme, toggleTheme } = useTheme()
const isMobileMenuOpen = ref(false)

const navItems = [
  { path: '/', label: '首页', icon: 'grid' },
  { path: '/wayfind', label: '寻路算法', icon: 'map' },
  { path: '/professor', label: '专家系统', icon: 'brain' },
  { path: '/mlearn', label: '机器学习', icon: 'cpu' }
]

const currentPath = computed(() => route.path)

function navigate(path: string) {
  router.push(path)
  isMobileMenuOpen.value = false
}

function toggleMobileMenu() {
  isMobileMenuOpen.value = !isMobileMenuOpen.value
}
</script>

<template>
  <header class="header">
    <div class="header-container">
      <div class="logo" @click="navigate('/')">
        <svg class="logo-icon" viewBox="0 0 40 40" fill="none">
          <defs>
            <linearGradient id="logoGradient" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" style="stop-color:#3b82f6"/>
              <stop offset="100%" style="stop-color:#10b981"/>
            </linearGradient>
          </defs>
          <circle cx="20" cy="20" r="16" stroke="url(#logoGradient)" stroke-width="1.5" fill="none"/>
          <circle cx="20" cy="20" r="3" fill="url(#logoGradient)"/>
          <circle cx="12" cy="12" r="1.5" fill="#3b82f6"/>
          <circle cx="28" cy="12" r="1.5" fill="#3b82f6"/>
          <circle cx="12" cy="28" r="1.5" fill="#10b981"/>
          <circle cx="28" cy="28" r="1.5" fill="#10b981"/>
          <line x1="12" y1="12" x2="20" y2="20" stroke="#3b82f6" stroke-width="1.2" opacity="0.5"/>
          <line x1="28" y1="12" x2="20" y2="20" stroke="#3b82f6" stroke-width="1.2" opacity="0.5"/>
          <line x1="12" y1="28" x2="20" y2="20" stroke="#10b981" stroke-width="1.2" opacity="0.5"/>
          <line x1="28" y1="28" x2="20" y2="20" stroke="#10b981" stroke-width="1.2" opacity="0.5"/>
        </svg>
        <span class="logo-text">AI Showcase</span>
      </div>

      <nav class="desktop-nav">
        <button
          v-for="item in navItems"
          :key="item.path"
          class="nav-item"
          :class="{ active: currentPath === item.path }"
          @click="navigate(item.path)"
        >
          <svg v-if="item.icon === 'grid'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="7"/>
            <rect x="14" y="3" width="7" height="7"/>
            <rect x="14" y="14" width="7" height="7"/>
            <rect x="3" y="14" width="7" height="7"/>
          </svg>
          <svg v-if="item.icon === 'map'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 7l6-3 6 3 6-3v13l-6 3-6-3-6 3V7z"/>
            <path d="M9 4v13"/>
            <path d="M15 7v13"/>
          </svg>
          <svg v-if="item.icon === 'brain'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 4.5a2.5 2.5 0 0 0-4.96-.46 2.5 2.5 0 0 0-1.98 3 2.5 2.5 0 0 0 1.32 4.24 3 3 0 0 0 .34 5.58 2.5 2.5 0 0 0 5.51.63A2.5 2.5 0 0 0 12 18.5"/>
            <path d="M12 4.5a2.5 2.5 0 0 1 4.96-.46 2.5 2.5 0 0 1 1.98 3 2.5 2.5 0 0 1-1.32 4.24 3 3 0 0 1-.34 5.58 2.5 2.5 0 0 1-5.51.63"/>
            <path d="M12 4.5V18"/>
          </svg>
          <svg v-if="item.icon === 'cpu'" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="4" y="4" width="16" height="16" rx="2"/>
            <rect x="9" y="9" width="6" height="6"/>
            <path d="M9 1v3"/>
            <path d="M15 1v3"/>
            <path d="M9 20v3"/>
            <path d="M15 20v3"/>
            <path d="M20 9h3"/>
            <path d="M20 14h3"/>
            <path d="M1 9h3"/>
            <path d="M1 14h3"/>
          </svg>
          <span class="nav-label">{{ item.label }}</span>
        </button>
      </nav>

      <button class="theme-toggle" @click="toggleTheme" :title="currentTheme === 'dark' ? '切换亮色主题' : '切换暗色主题'">
        <svg v-if="currentTheme === 'dark'" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="5"/>
          <line x1="12" y1="1" x2="12" y2="3"/>
          <line x1="12" y1="21" x2="12" y2="23"/>
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
          <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
          <line x1="1" y1="12" x2="3" y2="12"/>
          <line x1="21" y1="12" x2="23" y2="12"/>
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
          <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
        </svg>
        <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
        </svg>
      </button>

      <button class="mobile-menu-btn" @click="toggleMobileMenu">
        <svg v-if="!isMobileMenuOpen" class="menu-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="3" y1="6" x2="21" y2="6"/>
          <line x1="3" y1="12" x2="21" y2="12"/>
          <line x1="3" y1="18" x2="21" y2="18"/>
        </svg>
        <svg v-else class="menu-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6" y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </div>

    <Transition name="slide">
      <nav v-if="isMobileMenuOpen" class="mobile-nav">
        <button
          v-for="item in navItems"
          :key="item.path"
          class="mobile-nav-item"
          :class="{ active: currentPath === item.path }"
          @click="navigate(item.path)"
        >
          {{ item.label }}
        </button>
      </nav>
    </Transition>
  </header>
</template>

<style scoped>
.header {
  position: sticky;
  top: 0;
  z-index: 50;
  background: var(--bg-secondary);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--border-color);
  transition: background 0.2s, border-color 0.2s;
}

.header-container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 64px;
}

@media (min-width: 768px) {
  .header-container {
    padding: 0 24px;
  }
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.logo:hover {
  opacity: 0.8;
}

.logo-icon {
  width: 36px;
  height: 36px;
  color: var(--accent-blue);
}

.logo-text {
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: -0.02em;
}

.desktop-nav {
  display: none;
  gap: 8px;
}

@media (min-width: 768px) {
  .desktop-nav {
    display: flex;
  }
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border-radius: 12px;
  background: transparent;
  border: 1px solid transparent;
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.nav-item:hover {
  color: var(--text-primary);
  background: var(--bg-tertiary);
}

.nav-item.active {
  color: var(--text-primary);
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.3);
}

.nav-icon {
  width: 18px;
  height: 18px;
}

.theme-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.theme-toggle:hover {
  color: var(--text-primary);
  border-color: var(--text-secondary);
}

.theme-toggle svg {
  width: 18px;
  height: 18px;
}

@media (max-width: 767px) {
  .theme-toggle {
    display: none;
  }
}

.mobile-menu-btn {
  display: flex;
  padding: 8px;
  border-radius: 8px;
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.mobile-menu-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

@media (min-width: 768px) {
  .mobile-menu-btn {
    display: none;
  }
}

.menu-icon {
  width: 24px;
  height: 24px;
}

.mobile-nav {
  display: flex;
  flex-direction: column;
  padding: 8px;
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

@media (min-width: 768px) {
  .mobile-nav {
    display: none;
  }
}

.mobile-nav-item {
  padding: 12px 16px;
  border-radius: 8px;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  font-size: 16px;
  font-weight: 500;
  text-align: left;
  cursor: pointer;
  transition: all 0.2s;
}

.mobile-nav-item:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.mobile-nav-item.active {
  background: rgba(59, 130, 246, 0.1);
  color: var(--accent-blue);
}

.slide-enter-active,
.slide-leave-active {
  transition: all 0.3s ease;
}

.slide-enter-from,
.slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>
