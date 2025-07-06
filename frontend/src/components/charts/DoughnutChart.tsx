import React from 'react'
import { ChartData } from '../../types'
import { commonStyles } from '../../styles/common'

interface DoughnutChartProps {
  data: ChartData[]
  title: string
}

export const DoughnutChart: React.FC<DoughnutChartProps> = ({ data, title }) => {
  const total = data.reduce((sum, item) => sum + item.value, 0)
  
  if (total === 0) {
    return (
      <div style={commonStyles.card}>
        <h3 style={commonStyles.cardTitle}>
          🎯 {title}
        </h3>
        <div style={commonStyles.emptyState}>
          🔄 Waiting for collection data...
          <br />
          <small>The first collection may take a few minutes</small>
        </div>
      </div>
    )
  }

  let currentAngle = 0
  
  return (
    <div style={commonStyles.card}>
      <h3 style={commonStyles.cardTitle}>
        🎯 {title}
      </h3>
      <div style={{ display: 'flex', alignItems: 'center', gap: '2rem' }}>
        <svg width="120" height="120" viewBox="0 0 120 120">
          <circle cx="60" cy="60" r="50" fill="none" stroke="#f0f0f0" strokeWidth="20" />
          {data.map((item, index) => {
            const angle = (item.value / total) * 360
            const x1 = 60 + 50 * Math.cos((currentAngle - 90) * Math.PI / 180)
            const y1 = 60 + 50 * Math.sin((currentAngle - 90) * Math.PI / 180)
            const x2 = 60 + 50 * Math.cos((currentAngle + angle - 90) * Math.PI / 180)
            const y2 = 60 + 50 * Math.sin((currentAngle + angle - 90) * Math.PI / 180)
            const largeArcFlag = angle > 180 ? 1 : 0
            
            const pathData = `
              M 60 60
              L ${x1} ${y1}
              A 50 50 0 ${largeArcFlag} 1 ${x2} ${y2}
              Z
            `
            
            currentAngle += angle
            
            return (
              <path
                key={index}
                d={pathData}
                fill={item.color}
                stroke="white"
                strokeWidth="2"
              />
            )
          })}
          <circle cx="60" cy="60" r="25" fill="white" />
          <text x="60" y="60" textAnchor="middle" dominantBaseline="middle" fontSize="14" fontWeight="bold">
            {total}
          </text>
        </svg>
        <div style={{ flex: 1 }}>
          {data.map((item, index) => (
            <div key={index} style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: '0.5rem', 
              marginBottom: '0.5rem' 
            }}>
              <div style={{ 
                width: '12px', 
                height: '12px', 
                backgroundColor: item.color, 
                borderRadius: '2px' 
              }} />
              <span style={{ fontSize: '0.9rem' }}>
                {item.name}: <strong>{item.value}</strong> ({((item.value / total) * 100).toFixed(1)}%)
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
} 