import React from 'react'
import { DashboardMetrics } from '../types'
import { commonStyles } from '../styles/common'

interface LogsProps {
  metrics: DashboardMetrics
  lastRefresh: string
  isAutoRefresh: boolean
  refreshInterval: number
}

export const Logs: React.FC<LogsProps> = ({ 
  metrics, 
  lastRefresh, 
  isAutoRefresh, 
  refreshInterval = 30 
}) => {
  const logEntries = [
    {
      time: new Date().toLocaleTimeString(),
      level: 'INFO',
      message: 'Dashboard loaded successfully',
      color: '#4CAF50'
    },
    {
      time: lastRefresh,
      level: 'INFO',
      message: `Metrics refreshed - ${metrics?.active_collectors}/${metrics?.total_collectors} collectors active`,
      color: '#4CAF50'
    },
    // Error logs from collectors
    ...(metrics?.collector_stats?.filter(c => c.error_message).map((collector) => ({
      time: new Date(collector.last_collection).toLocaleTimeString(),
      level: 'ERROR',
      message: `${collector.name} - ${collector.error_message}`,
      color: '#f44336'
    })) || []),
    {
      time: new Date().toLocaleTimeString(),
      level: 'INFO',
      message: `Auto-refresh: ${isAutoRefresh ? 'ON' : 'OFF'} (every ${refreshInterval}s)`,
      color: '#2196F3'
    }
  ]

  // Add warning if no collectors
  if (metrics?.collector_stats?.length === 0) {
    logEntries.push({
      time: new Date().toLocaleTimeString(),
      level: 'WARN',
      message: 'Waiting for first collection cycle (every 5 minutes)',
      color: '#FF9800'
    })
  }

  return (
    <div style={commonStyles.card}>
      <h2 style={commonStyles.cardTitle}>
        📜 System Activity & Logs
      </h2>
      <div style={{
        backgroundColor: '#f5f5f5',
        padding: '1rem',
        borderRadius: '0.5rem',
        fontFamily: 'monospace',
        fontSize: '0.9rem',
        maxHeight: '400px',
        overflowY: 'auto'
      }}>
        {logEntries.map((entry, index) => (
          <div key={index} style={{ marginBottom: '0.5rem' }}>
            <span style={{ color: entry.color }}>[{entry.time}]</span> 
            <span style={{ color: entry.level === 'ERROR' ? entry.color : '#666' }}> {entry.level}: </span>
            {entry.message}
          </div>
        ))}
      </div>
    </div>
  )
} 