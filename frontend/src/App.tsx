import { useState } from 'react'
import { ViewType } from './types'
import { useDashboardData, useTooltipStyles } from './hooks'
import { Header, Navigation } from './components/layout'
import { Loading } from './components/ui'
import { Overview, Collectors, Analytics, Logs } from './views'
import { commonStyles } from './styles/common'

function App() {
  const [currentView, setCurrentView] = useState<ViewType>('overview')
  
  const {
    metrics,
    loading,
    error,
    lastRefresh,
    isAutoRefresh,
    toggleAutoRefresh,
    manualRefresh
  } = useDashboardData()

  // Initialize tooltip styles
  useTooltipStyles()

  if (loading) {
    return <Loading message="Loading VersionWatch dashboard..." />
  }

  if (error) {
    return (
      <div style={commonStyles.loading}>
        <div style={commonStyles.loadingCard}>
          <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>❌</div>
          <div style={{ fontSize: '1.2rem', marginBottom: '1rem' }}>Error loading dashboard</div>
          <div style={{ fontSize: '0.9rem', color: '#f44336' }}>{error}</div>
          <button
            onClick={manualRefresh}
            style={{
              marginTop: '1rem',
              padding: '0.5rem 1rem',
              backgroundColor: '#667eea',
              color: 'white',
              border: 'none',
              borderRadius: '0.5rem',
              cursor: 'pointer'
            }}
          >
            Retry
          </button>
        </div>
      </div>
    )
  }

  if (!metrics) {
    return <Loading message="No data available" />
  }

  const renderView = () => {
    switch (currentView) {
      case 'overview':
        return <Overview metrics={metrics} />
      case 'collectors':
        return <Collectors metrics={metrics} />
      case 'analytics':
        return <Analytics metrics={metrics} />
      case 'logs':
        return (
          <Logs 
            metrics={metrics}
            lastRefresh={lastRefresh}
            isAutoRefresh={isAutoRefresh}
            refreshInterval={30}
          />
        )
      default:
        return <Overview metrics={metrics} />
    }
  }

  return (
    <div style={commonStyles.container}>
      <Header
        lastRefresh={lastRefresh}
        isAutoRefresh={isAutoRefresh}
        onRefreshToggle={toggleAutoRefresh}
        onManualRefresh={manualRefresh}
      />
      
      <Navigation
        currentView={currentView}
        onViewChange={setCurrentView}
      />
      
      <main style={commonStyles.content}>
        {renderView()}
      </main>
    </div>
  )
}

export default App
