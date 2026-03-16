import React from 'react'
import { ChartData } from '../../types'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { BarChart3 } from "lucide-react"

interface BarChartProps {
  data: ChartData[]
  title: string
}

export const BarChart: React.FC<BarChartProps> = ({ data, title }) => {
  const isEmpty = data.every(item => item.value === 0)

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <BarChart3 className="h-5 w-5" />
          {title}
        </CardTitle>
      </CardHeader>
      <CardContent>
        {isEmpty ? (
          <div className="text-center py-8 text-muted-foreground text-sm">
            Waiting for collection data...
            <br />
            <small>The first collection may take a few minutes</small>
          </div>
        ) : (
          <div className="flex items-end gap-2 h-[150px]">
            {data.map((item, index) => {
              const maxValue = Math.max(...data.map(d => d.value))
              const height = Math.max((item.value / maxValue) * 120, 5)

              return (
                <div
                  key={index}
                  className="flex-1 flex flex-col items-center"
                >
                  <div
                    className="w-full rounded-t flex items-center justify-center text-white text-xs font-bold"
                    style={{ height: `${height}px`, backgroundColor: item.color }}
                  >
                    {item.value}
                  </div>
                  <div className="text-xs mt-2 text-center">
                    {item.name}
                  </div>
                </div>
              )
            })}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
