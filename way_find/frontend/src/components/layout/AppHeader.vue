<template>
  <header class="header">
    <div class="header-content">
      <div class="logo-section">
        <div class="logo-icon">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none">
            <path d="M12 2L2 7L12 12L22 7L12 2Z" stroke="currentColor" stroke-width="2"/>
            <path d="M2 17L12 22L22 17" stroke="currentColor" stroke-width="2"/>
            <path d="M2 12L12 17L22 12" stroke="currentColor" stroke-width="2"/>
          </svg>
        </div>
        <div class="logo-text">
          <h1>WayFind</h1>
          <p>Pathfinding Visualizer</p>
        </div>
      </div>
      
      <div class="header-actions">
        <button class="action-btn" title="Help">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/>
            <path d="M9 9a3 3 0 115.12 2.12c-.73.73-1.12 1.1-1.12 1.88v1M12 17h.01" stroke="currentColor" stroke-width="2"/>
          </svg>
        </button>
        
        <button class="action-btn" title="Settings">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="2"/>
            <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" stroke="currentColor" stroke-width="2"/>
          </svg>
        </button>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
function handleHelp() {
  alert('WayFind Pathfinding Visualizer\n\nControls:\n- Click on grid to add walls\n- Click start/end to move them\n- Use controls to run algorithms')
}
</script>

<style scoped>
.header {
  backdrop-filter: blur(20px);
  background: rgba(255, 255, 255, 0.03);
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  position: sticky;
  top: 0;
  z-index: 50;
}

.header-content {
  max-width: 1600px;
  margin: 0 auto;
  padding: 1rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo-section {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.logo-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.2) 0%, rgba(6, 182, 212, 0.2) 100%);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #10b981;
  box-shadow: 0 8px 32px rgba(16, 185, 129, 0.2);
}

.logo-text h1 {
  font-size: 1.25rem;
  font-weight: 700;
  letter-spacing: -0.025em;
  color: white;
  margin: 0;
}

.logo-text p {
  font-size: 0.75rem;
  opacity: 0.6;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  width: 40px;
  height: 40px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: rgba(255, 255, 255, 0.6);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
  transform: scale(1.05);
}

.action-btn:active {
  transform: scale(0.95);
}
</style>
