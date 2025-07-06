import { useState, useEffect, useCallback } from 'react'
import { DashboardMetrics } from '../types'

export const useDashboardData = () => {
  const [metrics, setMetrics] = useState<DashboardMetrics | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [lastRefresh, setLastRefresh] = useState<string>('')
  const [isAutoRefresh, setIsAutoRefresh] = useState(true)

  const fetchMetrics = useCallback(async () => {
    try {
      const response = await fetch('/api/metrics')
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }
      const data = await response.json()
      setMetrics(data)
      setError(null)
      setLastRefresh(new Date().toLocaleTimeString())
    } catch (err) {
      console.error('Failed to fetch metrics:', err)
      setError(err instanceof Error ? err.message : 'Unknown error')
    } finally {
      setLoading(false)
    }
  }, [])

  const toggleAutoRefresh = useCallback(() => {
    setIsAutoRefresh(prev => !prev)
  }, [])

  const manualRefresh = useCallback(() => {
    setLoading(true)
    fetchMetrics()
  }, [fetchMetrics])

  // Initial fetch
  useEffect(() => {
    fetchMetrics()
  }, [fetchMetrics])

  // Auto-refresh logic
  useEffect(() => {
    if (!isAutoRefresh) return

    const interval = setInterval(fetchMetrics, 30000) // 30 seconds
    return () => clearInterval(interval)
  }, [isAutoRefresh, fetchMetrics])

  return {
    metrics,
    loading,
    error,
    lastRefresh,
    isAutoRefresh,
    toggleAutoRefresh,
    manualRefresh
  }
} 