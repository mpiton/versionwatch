import React from 'react'
import { Tooltip } from './Tooltip'

interface MetricCardProps {
  title: string
  value: string | number
  icon: string
  tooltip?: string
  color?: string
}

export const MetricCard: React.FC<MetricCardProps> = ({ 
  title, 
  value, 
  icon, 
  tooltip,
  color = '#667eea'
}) => {
  const cardContent = (
    <div style={{
      backgroundColor: 'rgba(255,255,255,0.95)',
      padding: '1.5rem',
      borderRadius: '1rem',
      textAlign: 'center',
      boxShadow: '0 4px 6px rgba(0,0,0,0.1)',
      color: '#333',
      minHeight: '140px',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      border: `3px solid ${color}`,
      transition: 'transform 0.2s ease, box-shadow 0.2s ease'
    }}>
      <div style={{ 
        fontSize: '2.5rem', 
        marginBottom: '0.5rem',
        filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.1))'
      }}>
        {icon}
      </div>
      <div style={{ 
        fontSize: '2rem', 
        fontWeight: 'bold', 
        color,
        marginBottom: '0.5rem'
      }}>
        {value}
      </div>
      <div style={{ 
        fontSize: '0.9rem', 
        color: '#666',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        gap: '0.25rem'
      }}>
        {title}
        {tooltip && (
          <span style={{
            fontSize: '0.8rem',
            backgroundColor: '#f0f0f0',
            color: '#666',
            borderRadius: '50%',
            width: '16px',
            height: '16px',
            display: 'inline-flex',
            alignItems: 'center',
            justifyContent: 'center',
            cursor: 'help'
          }}>
            ?
          </span>
        )}
      </div>
    </div>
  )

  if (tooltip) {
    return <Tooltip content={tooltip}>{cardContent}</Tooltip>
  }

  return cardContent
} 