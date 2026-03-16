import React from 'react'
import { DashboardMetrics } from '../types'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { BarChart3, Timer } from "lucide-react"
import { getCategoryColor, PERFORMANCE_MAPPING } from "@/lib/constants"

interface AnalyticsProps {
  metrics: DashboardMetrics
}

export const Analytics: React.FC<AnalyticsProps> = ({ metrics }) => {
  const performanceCategories = metrics?.collector_stats?.reduce((acc, collector) => {
    const category = collector.performance_category
    const displayName = PERFORMANCE_MAPPING[category]?.display || category
    acc[displayName] = (acc[displayName] || 0) + 1
    return acc
  }, {} as Record<string, number>) || {}

  const slowestCollectors = [...(metrics?.collector_stats || [])].sort((a, b) => b.response_time - a.response_time).slice(0, 5)

  const hasData = Object.keys(performanceCategories).length > 0

  return (
    <div>
      {/* Performance Analytics */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <BarChart3 className="h-5 w-5" />
            Performance Analytics
          </CardTitle>
        </CardHeader>
        <CardContent>
          {!hasData ? (
            <div className="text-center py-8 text-muted-foreground text-sm">
              Waiting for collection data...
            </div>
          ) : (
            <div className="grid grid-cols-[repeat(auto-fit,minmax(200px,1fr))] gap-4">
              {Object.entries(performanceCategories).map(([category, count]) => (
                <div
                  key={category}
                  className="p-4 rounded-lg border-l-4"
                  style={{
                    backgroundColor: category === 'Low' ? '#e8f5e8' : category === 'Medium' ? '#fff8f0' : category === 'High' ? '#f0f8ff' : category === 'Very High' ? '#f8f0ff' : '#f5f5f5',
                    borderLeftColor: getCategoryColor(category)
                  }}
                >
                  <strong>{category} Collectors:</strong>
                  <span className="ml-2 text-xl font-bold">
                    {count}
                  </span>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Slowest Collectors */}
      {slowestCollectors.length > 0 && (
        <Card className="mt-6">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Timer className="h-5 w-5" />
              Slowest Collectors
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid gap-2">
              {slowestCollectors.map((collector, index) => (
                <div
                  key={collector.name}
                  className="flex justify-between items-center p-3 bg-muted/50 rounded-lg border"
                >
                  <div className="flex items-center gap-2">
                    <span className="bg-muted-foreground text-white rounded-full w-5 h-5 flex items-center justify-center text-xs">
                      {index + 1}
                    </span>
                    <strong>{collector.name}</strong>
                  </div>
                  <div className="flex items-center gap-4">
                    <span className="text-sm text-muted-foreground">
                      {collector.version_count} versions
                    </span>
                    <strong style={{
                      color: collector.response_time > 3000 ? '#f44336' :
                             collector.response_time > 1000 ? '#FF9800' : '#4CAF50'
                    }}>
                      {Math.round(collector.response_time)}ms
                    </strong>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
}
