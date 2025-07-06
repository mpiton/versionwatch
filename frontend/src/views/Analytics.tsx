import React from 'react'
import { DashboardMetrics } from '../types'
import { commonStyles } from '../styles/common'

interface AnalyticsProps {
  metrics: DashboardMetrics
}

export const Analytics: React.FC<AnalyticsProps> = ({ metrics }) => {
  // Map real backend categories
  const performanceMapping = {
    'No Data': 'No Data',
    'Low Volume': 'Low',
    'Medium Volume': 'Medium', 
    'High Volume': 'High',
    'Very High Volume': 'Very High'
  }

  const performanceCategories = metrics?.collector_stats?.reduce((acc, collector) => {
    const category = collector.performance_category
    const displayName = performanceMapping[category as keyof typeof performanceMapping] || category
    acc[displayName] = (acc[displayName] || 0) + 1
    return acc
  }, {} as Record<string, number>) || {}

  const slowestCollectors = [...(metrics?.collector_stats || [])].sort((a, b) => b.response_time - a.response_time).slice(0, 5)

  const hasData = Object.keys(performanceCategories).length > 0

  return (
    <div>
      {/* Performance Analytics */}
      <div style={commonStyles.card}>
        <h2 style={commonStyles.cardTitle}>
          📊 Performance Analytics
        </h2>
        {!hasData ? (
          <div style={commonStyles.emptyState}>
            🔄 Waiting for collection data...
          </div>
        ) : (
          <div style={{ 
            display: 'grid', 
            gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', 
            gap: '1rem' 
          }}>
            {Object.entries(performanceCategories).map(([category, count]) => (
              <div key={category} style={{
                padding: '1rem',
                backgroundColor: category === 'Low' ? '#e8f5e8' : 
                                category === 'Medium' ? '#fff8f0' : 
                                category === 'High' ? '#f0f8ff' :
                                category === 'Very High' ? '#f8f0ff' : '#f5f5f5',
                borderRadius: '0.5rem',
                borderLeft: `4px solid ${category === 'Low' ? commonStyles.statusColors.active : 
                                        category === 'Medium' ? commonStyles.statusColors.warning : 
                                        category === 'High' ? commonStyles.statusColors.info :
                                        category === 'Very High' ? '#9C27B0' : '#9E9E9E'}`
              }}>
                <strong>{category} Collectors:</strong> 
                <span style={{ marginLeft: '0.5rem', fontSize: '1.2rem', fontWeight: 'bold' }}>
                  {count}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Slowest Collectors */}
      {slowestCollectors.length > 0 && (
        <div style={commonStyles.card}>
          <h3 style={commonStyles.cardTitle}>
            🐌 Slowest Collectors
          </h3>
          <div style={{ display: 'grid', gap: '0.5rem' }}>
            {slowestCollectors.map((collector, index) => (
              <div key={collector.name} style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                padding: '0.75rem',
                backgroundColor: '#f8f9fa',
                borderRadius: '0.5rem',
                border: '1px solid #e0e0e0'
              }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
                  <span style={{ 
                    backgroundColor: '#666', 
                    color: 'white', 
                    borderRadius: '50%', 
                    width: '20px', 
                    height: '20px', 
                    display: 'flex', 
                    alignItems: 'center', 
                    justifyContent: 'center', 
                    fontSize: '0.8rem' 
                  }}>
                    {index + 1}
                  </span>
                  <strong>{collector.name}</strong>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
                  <span style={{ fontSize: '0.9rem', color: '#666' }}>
                    {collector.version_count} versions
                  </span>
                  <strong style={{ 
                    color: collector.response_time > 3000 ? commonStyles.statusColors.failed : 
                          collector.response_time > 1000 ? commonStyles.statusColors.warning : commonStyles.statusColors.active 
                  }}>
                    {Math.round(collector.response_time)}ms
                  </strong>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
} 