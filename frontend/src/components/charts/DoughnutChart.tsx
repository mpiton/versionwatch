import React from 'react'
import { ChartData } from '../../types'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Target } from "lucide-react"

interface DoughnutChartProps {
  data: ChartData[]
  title: string
}

export const DoughnutChart: React.FC<DoughnutChartProps> = ({ data, title }) => {
  const total = data.reduce((sum, item) => sum + item.value, 0)

  if (total === 0) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Target className="h-5 w-5" />
            {title}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8 text-muted-foreground text-sm">
            Waiting for collection data...
            <br />
            <small>The first collection may take a few minutes</small>
          </div>
        </CardContent>
      </Card>
    )
  }

  let currentAngle = 0

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Target className="h-5 w-5" />
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="flex items-center gap-8">
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
          <div className="flex-1">
            {data.map((item, index) => (
              <div key={index} className="flex items-center gap-2 mb-2">
                <div
                  className="w-3 h-3 rounded-sm"
                  style={{ backgroundColor: item.color }}
                />
                <span className="text-sm">
                  {item.name}: <strong>{item.value}</strong> ({((item.value / total) * 100).toFixed(1)}%)
                </span>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
