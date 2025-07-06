import React from 'react'
import { commonStyles } from '../../styles/common'

interface LoadingProps {
  message?: string
}

export const Loading: React.FC<LoadingProps> = ({ 
  message = "Loading dashboard..." 
}) => {
  return (
    <div style={commonStyles.loading}>
      <div style={commonStyles.loadingCard}>
        <div style={{ fontSize: '3rem', marginBottom: '1rem' }}>⏳</div>
        <div style={{ fontSize: '1.2rem' }}>{message}</div>
      </div>
    </div>
  )
} 