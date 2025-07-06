import React from 'react'
import { DashboardMetrics } from '../types'
import { commonStyles } from '../styles/common'

interface CollectorsProps {
  metrics: DashboardMetrics
}

export const Collectors: React.FC<CollectorsProps> = ({ metrics }) => {
  const hasCollectors = metrics?.collector_stats?.length > 0

  return (
    <div style={commonStyles.card}>
      <h2 style={commonStyles.cardTitle}>
        📋 Collectors Details
      </h2>
      {!hasCollectors ? (
        <div style={commonStyles.emptyState}>
          🔄 Waiting for collection data...
          <br />
          <small style={{ marginTop: '0.5rem', display: 'block' }}>
            The first collection may take a few minutes after server startup
          </small>
        </div>
      ) : (
        <div style={{ display: 'grid', gap: '1rem' }}>
          {metrics.collector_stats.map((collector) => (
            <div key={collector.name} style={{
              padding: '1rem',
              backgroundColor: '#f8f9fa',
              borderRadius: '0.5rem',
              border: `2px solid ${collector.status === 'Active' ? commonStyles.statusColors.active : commonStyles.statusColors.failed}`,
              display: 'grid',
              gridTemplateColumns: 'auto 1fr auto auto',
              alignItems: 'center',
              gap: '1rem'
            }}>
              {/* Status indicator */}
              <div style={{
                width: '12px',
                height: '12px',
                borderRadius: '50%',
                backgroundColor: collector.status === 'Active' ? commonStyles.statusColors.active : commonStyles.statusColors.failed
              }} />
              
              {/* Collector info */}
              <div>
                <strong style={{ fontSize: '1.1rem' }}>{collector.name}</strong>
                <div style={{ fontSize: '0.9rem', color: '#666', marginTop: '0.25rem' }}>
                  {collector.version_count} versions • 
                  <span style={{ 
                    color: collector.performance_category === 'Low Volume' ? commonStyles.statusColors.active : 
                          collector.performance_category === 'Medium Volume' ? commonStyles.statusColors.warning : 
                          collector.performance_category === 'High Volume' ? commonStyles.statusColors.info :
                          collector.performance_category === 'Very High Volume' ? '#9C27B0' : '#9E9E9E',
                    marginLeft: '0.5rem'
                  }}>
                    {collector.performance_category}
                  </span>
                </div>
                {collector.error_message && (
                  <div style={{ fontSize: '0.8rem', color: commonStyles.statusColors.failed, marginTop: '0.25rem' }}>
                    ⚠️ {collector.error_message}
                  </div>
                )}
              </div>
              
              {/* Response time */}
              <div style={{ textAlign: 'right' }}>
                <div style={{ fontSize: '0.9rem', fontWeight: 'bold' }}>
                  {Math.round(collector.response_time)}ms
                </div>
                <div style={{ fontSize: '0.8rem', color: '#666' }}>
                  Response time
                </div>
              </div>
              
              {/* Last updated */}
              <div style={{ textAlign: 'right' }}>
                <div style={{ fontSize: '0.8rem', color: '#666' }}>
                  Last updated
                </div>
                <div style={{ fontSize: '0.9rem' }}>
                  {new Date(collector.last_collection).toLocaleString()}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
} 