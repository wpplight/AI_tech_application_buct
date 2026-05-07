import { ref, watch } from 'vue'

type Theme = 'dark' | 'light'

const currentTheme = ref<Theme>('dark')

const THEME_KEY = 'professor_theme'

function loadTheme() {
  try {
    const stored = localStorage.getItem(THEME_KEY)
    if (stored === 'light' || stored === 'dark') {
      currentTheme.value = stored
    }
  } catch {}
}

function applyTheme(theme: Theme) {
  document.documentElement.setAttribute('data-theme', theme)
}

function toggleTheme() {
  currentTheme.value = currentTheme.value === 'dark' ? 'light' : 'dark'
}

loadTheme()
applyTheme(currentTheme.value)

watch(currentTheme, (theme) => {
  localStorage.setItem(THEME_KEY, theme)
  applyTheme(theme)
})

export function useTheme() {
  return {
    currentTheme,
    toggleTheme
  }
}
