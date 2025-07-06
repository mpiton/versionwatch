import React from 'react'
import { DashboardMetrics } from '../types'
import { MetricCard } from '../components/ui'
import { BarChart, DoughnutChart, LineChart } from '../components/charts'

interface OverviewProps {
  metrics: DashboardMetrics
}

export const Overview: React.FC<OverviewProps> = ({ metrics }) => {
  // Chart data with adaptation to real backend categories
  const collectorsData = [
    { name: 'Active', value: metrics?.active_collectors || 0, color: '#4CAF50' },
    { name: 'Failed', value: metrics?.failed_collectors || 0, color: '#f44336' },
    { name: 'Idle', value: Math.max(0, (metrics?.total_collectors || 0) - (metrics?.active_collectors || 0) - (metrics?.failed_collectors || 0)), color: '#FF9800' }
  ]

  // Map real backend categories to simple displays
  const performanceMapping = {
    'No Data': { display: 'No Data', color: '#9E9E9E' },
    'Low Volume': { display: 'Low', color: '#4CAF50' },
    'Medium Volume': { display: 'Medium', color: '#FF9800' },
    'High Volume': { display: 'High', color: '#2196F3' },
    'Very High Volume': { display: 'Very High', color: '#9C27B0' }
  }

  const performanceCounts = metrics?.collector_stats?.reduce((acc, collector) => {
    const category = collector.performance_category
    if (performanceMapping[category as keyof typeof performanceMapping]) {
      acc[category] = (acc[category] || 0) + 1
    }
    return acc
  }, {} as Record<string, number>) || {}

  const performanceData = Object.entries(performanceCounts).map(([category, count]) => ({
    name: performanceMapping[category as keyof typeof performanceMapping]?.display || category,
    value: count,
    color: performanceMapping[category as keyof typeof performanceMapping]?.color || '#9E9E9E'
  }))

  const responseTimeData = metrics?.collector_stats?.slice(0, 8).map(c => ({
    name: c.name.substring(0, 8),
    value: Math.round(c.response_time)
  })) || []

  return (
    <div>
      {/* Quick Stats */}
      <div style={{ 
        display: 'grid', 
        gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', 
        gap: '1rem', 
        marginBottom: '2rem' 
      }}>
        <MetricCard
          title="Total Collectors"
          value={metrics?.total_collectors || 0}
          icon="📊"
          color="#4CAF50"
          tooltip="Total number of collectors configured in the system to monitor software versions"
        />
        
        <MetricCard
          title="Active"
          value={metrics?.active_collectors || 0}
          icon="✅"
          color="#2196F3"
          tooltip="Number of collectors that are working correctly and collecting data"
        />
        
        <MetricCard
          title="Failed"
          value={metrics?.failed_collectors || 0}
          icon="❌"
          color="#f44336"
          tooltip="Number of collectors that failed during their last collection attempt"
        />
        
        <MetricCard
          title="Total Versions"
          value={metrics?.total_versions || 0}
          icon="🔢"
          color="#FF9800"
          tooltip="Total number of software versions discovered by all active collectors"
        />
      </div>

      {/* System Health */}
      <div style={{ 
        display: 'grid', 
        gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', 
        gap: '1rem', 
        marginBottom: '2rem' 
      }}>
        <MetricCard
          title="Overall Status"
          value={metrics?.system_health?.overall_status || 'Unknown'}
          icon="🏥"
          color={
            metrics?.system_health?.overall_status === 'Healthy' ? '#4CAF50' :
            metrics?.system_health?.overall_status === 'Warning' ? '#FF9800' : '#f44336'
          }
          tooltip="Status based on success rate: Healthy (≥90%), Warning (70-89%), Critical (<70%)"
        />
        
        <MetricCard
          title="Success Rate"
          value={`${metrics?.system_health?.success_rate?.toFixed(1) || 0}%`}
          icon="📊"
          color="#2196F3"
          tooltip="Percentage of collectors that succeeded in their last data collection"
        />
        
        <MetricCard
          title="Response Time"
          value={`${metrics?.system_health?.average_response_time?.toFixed(0) || 0}ms`}
          icon="⚡"
          color="#9C27B0"
          tooltip="Average response time of all collectors during the last collection"
        />
        
        <MetricCard
          title="Anomalies Detected"
          value={metrics?.system_health?.anomalies_detected || 0}
          icon="🚨"
          color="#FF5722"
          tooltip="Collectors with abnormally low version count (< 30% of average). May indicate a collection issue or limited data source."
        />
      </div>

      {/* Charts */}
      <div style={{ 
        display: 'grid', 
        gridTemplateColumns: 'repeat(auto-fit, minmax(400px, 1fr))', 
        gap: '2rem' 
      }}>
        <DoughnutChart
          data={collectorsData}
          title="Collector Status Distribution"
        />
        
        <BarChart
          data={performanceData}
          title="Performance Distribution"
        />
      </div>

      <LineChart
        data={responseTimeData}
        title="Response Times by Collector"
      />
    </div>
  )
} 