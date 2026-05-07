<template>
  <div class="layout">
    <AppHeader />
    
    <main class="main-content">
      <div class="bento-container">
        <!-- Sidebar -->
        <aside class="sidebar">
          <div class="bento-card">
            <ControlPanel />
          </div>
          
          <div class="bento-card">
            <AlgorithmSelector />
          </div>
          
          <div class="bento-card">
            <StatsPanel />
          </div>
        </aside>
        
        <!-- Main Grid -->
        <section class="grid-section">
          <div class="bento-card grid-card">
            <GridDisplay />
          </div>
        </section>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import AppHeader from './AppHeader.vue'
import GridDisplay from '../visualization/GridDisplay.vue'
import AlgorithmSelector from '../controls/AlgorithmSelector.vue'
import ControlPanel from '../controls/ControlPanel.vue'
import StatsPanel from '../controls/StatsPanel.vue'
</script>

<style scoped>
.layout {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1;
  padding: 2rem;
  max-width: 1600px;
  margin: 0 auto;
  width: 100%;
}

.bento-container {
  display: grid;
  grid-template-columns: 320px 1fr;
  gap: 1.5rem;
  align-items: start;
}

.sidebar {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.bento-card {
  background: rgba(255, 255, 255, 0.03);
  backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 1.5rem;
  padding: 1.5rem;
  transition: all 0.3s ease;
}

.bento-card:hover {
  background: rgba(255, 255, 255, 0.05);
  border-color: rgba(255, 255, 255, 0.12);
}

.grid-card {
  min-height: 500px;
  display: flex;
  align-items: center;
  justify-content: center;
}

@media (max-width: 1024px) {
  .bento-container {
    grid-template-columns: 1fr;
  }
  
  .sidebar {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  }
}
</style>
