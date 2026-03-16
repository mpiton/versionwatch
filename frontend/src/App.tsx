import { useState } from 'react'
import { ViewType } from './types'
import { useDashboardData } from './hooks'
import { Header, Navigation } from './components/layout'
import { Loading } from './components/ui'
import { Overview, Collectors, Analytics, Logs } from './views'
import { Button } from "@/components/ui/button"
import { RefreshCw } from "lucide-react"

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

  if (loading) {
    return <Loading message="Loading VersionWatch dashboard..." />
  }

  if (error) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-[#667eea] to-[#764ba2] flex items-center justify-center font-sans">
        <div className="text-center text-white p-8 bg-white/10 rounded-2xl">
          <div className="text-5xl mb-4">❌</div>
          <div className="text-xl mb-4">Error loading dashboard</div>
          <div className="text-sm text-red-300">{error}</div>
          <Button onClick={manualRefresh} className="mt-4">
            <RefreshCw className="h-4 w-4 mr-1" /> Retry
          </Button>
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
    <div className="min-h-screen bg-gradient-to-br from-[#667eea] to-[#764ba2] font-sans text-white">
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

      <main className="p-8">
        {renderView()}
      </main>
    </div>
  )
}

export default App
