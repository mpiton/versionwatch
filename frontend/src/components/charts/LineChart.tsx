import React from 'react'
import { LineChartData } from '../../types'
import { commonStyles } from '../../styles/common'

interface LineChartProps {
  data: LineChartData[]
  title: string
}

export const LineChart: React.FC<LineChartProps> = ({ data, title }) => {
  if (data.length === 0) {
    return (
      <div style={commonStyles.card}>
        <h3 style={commonStyles.cardTitle}>
          📈 {title}
        </h3>
        <div style={commonStyles.emptyState}>
          🔄 Waiting for collection data...
          <br />
          <small>The first collection may take a few minutes</small>
        </div>
      </div>
    )
  }

  const maxValue = Math.max(...data.map(d => d.value))
  const minValue = Math.min(...data.map(d => d.value))
  const range = maxValue - minValue
  const width = 400
  const height = 150
  const padding = 40

  return (
    <div style={commonStyles.card}>
      <h3 style={commonStyles.cardTitle}>
        📈 {title}
      </h3>
      <div style={{ 
        display: 'flex', 
        justifyContent: 'center', 
        alignItems: 'center',
        overflowX: 'auto'
      }}>
        <svg 
          width={Math.max(width, data.length * 60)} 
          height={height + padding * 2} 
          viewBox={`0 0 ${Math.max(width, data.length * 60)} ${height + padding * 2}`}
          style={{ minWidth: '100%' }}
        >
          {/* Grid lines */}
          {[0, 1, 2, 3, 4].map(i => {
            const y = padding + (i * height / 4)
            return (
              <line
                key={i}
                x1={padding}
                y1={y}
                x2={Math.max(width, data.length * 60) - padding}
                y2={y}
                stroke="#f0f0f0"
                strokeWidth="1"
              />
            )
          })}
          
          {/* Data line */}
          {data.length > 1 && (
            <polyline
              fill="none"
              stroke="#667eea"
              strokeWidth="3"
              points={data.map((item, index) => {
                const x = padding + (index * (Math.max(width, data.length * 60) - padding * 2) / (data.length - 1))
                const y = padding + height - ((item.value - minValue) / (range || 1)) * height
                return `${x},${y}`
              }).join(' ')}
            />
          )}
          
          {/* Data points */}
          {data.map((item, index) => {
            const x = padding + (index * (Math.max(width, data.length * 60) - padding * 2) / (data.length - 1))
            const y = padding + height - ((item.value - minValue) / (range || 1)) * height
            
            return (
              <g key={index}>
                <circle
                  cx={x}
                  cy={y}
                  r="4"
                  fill="#667eea"
                  stroke="white"
                  strokeWidth="2"
                />
                <text
                  x={x}
                  y={y - 10}
                  textAnchor="middle"
                  fontSize="10"
                  fill="#333"
                  fontWeight="bold"
                >
                  {item.value}
                </text>
                <text
                  x={x}
                  y={height + padding + 20}
                  textAnchor="middle"
                  fontSize="10"
                  fill="#666"
                  transform={data.length > 10 ? `rotate(-45, ${x}, ${height + padding + 20})` : ''}
                >
                  {item.name.length > 8 ? item.name.substring(0, 8) + '...' : item.name}
                </text>
              </g>
            )
          })}
        </svg>
      </div>
    </div>
  )
} 