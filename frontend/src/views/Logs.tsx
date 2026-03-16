import React from 'react'
import { DashboardMetrics } from '../types'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { ScrollText } from "lucide-react"

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
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <ScrollText className="h-5 w-5" />
          System Activity &amp; Logs
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="bg-muted p-4 rounded-lg font-mono text-sm max-h-[400px] overflow-y-auto">
          {logEntries.map((entry, index) => (
            <div key={index} className="mb-2">
              <span style={{ color: entry.color }}>[{entry.time}]</span>
              <span style={{ color: entry.level === 'ERROR' ? entry.color : '#666' }}> {entry.level}: </span>
              {entry.message}
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}
