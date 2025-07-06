import React from 'react'
import { commonStyles } from '../../styles/common'

interface HeaderProps {
  lastRefresh: string
  isAutoRefresh: boolean
  onRefreshToggle: () => void
  onManualRefresh: () => void
}

export const Header: React.FC<HeaderProps> = ({
  lastRefresh,
  isAutoRefresh,
  onRefreshToggle,
  onManualRefresh
}) => {
  return (
    <header style={commonStyles.header}>
      <div style={commonStyles.headerContent}>
        <div style={commonStyles.logoContainer}>
          <img 
            src="/logo.png" 
            alt="VersionWatch Logo" 
            style={commonStyles.logo}
          />
          <div>
            <h1 style={commonStyles.title}>VersionWatch</h1>
            <p style={commonStyles.subtitle}>
              Software Version Monitoring Dashboard
            </p>
          </div>
        </div>
        
        <div style={{ 
          display: 'flex', 
          alignItems: 'center', 
          gap: '1rem',
          fontSize: '0.9rem',
          color: '#666'
        }}>
          <div style={{ textAlign: 'right' }}>
            <div>Last refresh: {lastRefresh}</div>
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '0.5rem', 
              marginTop: '0.5rem' 
            }}>
              <button
                onClick={onRefreshToggle}
                style={{
                  padding: '0.5rem 1rem',
                  border: 'none',
                  borderRadius: '0.5rem',
                  cursor: 'pointer',
                  backgroundColor: isAutoRefresh ? '#4CAF50' : '#f0f0f0',
                  color: isAutoRefresh ? 'white' : '#333',
                  fontSize: '0.8rem',
                  transition: 'all 0.2s ease'
                }}
              >
                {isAutoRefresh ? '🔄 Auto-refresh ON' : '⏸️ Auto-refresh OFF'}
              </button>
              <button
                onClick={onManualRefresh}
                style={{
                  padding: '0.5rem 1rem',
                  border: 'none',
                  borderRadius: '0.5rem',
                  cursor: 'pointer',
                  backgroundColor: '#667eea',
                  color: 'white',
                  fontSize: '0.8rem',
                  transition: 'all 0.2s ease'
                }}
              >
                🔄 Refresh Now
              </button>
            </div>
          </div>
        </div>
      </div>
    </header>
  )
} 