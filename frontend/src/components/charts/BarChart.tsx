import React from 'react'
import { ChartData } from '../../types'
import { commonStyles } from '../../styles/common'

interface BarChartProps {
  data: ChartData[]
  title: string
}

export const BarChart: React.FC<BarChartProps> = ({ data, title }) => {
  const isEmpty = data.every(item => item.value === 0)

  return (
    <div style={commonStyles.card}>
      <h3 style={commonStyles.cardTitle}>
        📊 {title}
      </h3>
      {isEmpty ? (
        <div style={commonStyles.emptyState}>
          🔄 Waiting for collection data...
          <br />
          <small>The first collection may take a few minutes</small>
        </div>
      ) : (
        <div style={{ 
          display: 'flex', 
          alignItems: 'end', 
          gap: '0.5rem', 
          height: '150px' 
        }}>
          {data.map((item, index) => {
            const maxValue = Math.max(...data.map(d => d.value))
            const height = Math.max((item.value / maxValue) * 120, 5)
            
            return (
              <div 
                key={index} 
                style={{ 
                  flex: 1, 
                  display: 'flex', 
                  flexDirection: 'column', 
                  alignItems: 'center' 
                }}
              >
                <div style={{
                  width: '100%',
                  height: `${height}px`,
                  backgroundColor: item.color,
                  borderRadius: '4px 4px 0 0',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  color: 'white',
                  fontSize: '0.8rem',
                  fontWeight: 'bold'
                }}>
                  {item.value}
                </div>
                <div style={{ 
                  fontSize: '0.8rem', 
                  marginTop: '0.5rem', 
                  textAlign: 'center' 
                }}>
                  {item.name}
                </div>
              </div>
            )
          })}
        </div>
      )}
    </div>
  )
} 