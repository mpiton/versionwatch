import React from 'react'
import { DashboardMetrics } from '../types'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Badge } from "@/components/ui/badge"
import { ClipboardList } from "lucide-react"

interface CollectorsProps {
  metrics: DashboardMetrics
}

import { STATUS_COLORS, getCategoryColor } from "@/lib/constants"

export const Collectors: React.FC<CollectorsProps> = ({ metrics }) => {
  const hasCollectors = metrics?.collector_stats?.length > 0

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <ClipboardList className="h-5 w-5" />
          Collectors Details
        </CardTitle>
      </CardHeader>
      <CardContent>
        {!hasCollectors ? (
          <div className="text-center py-8 text-muted-foreground text-sm">
            Waiting for collection data...
            <br />
            <small className="mt-2 block">
              The first collection may take a few minutes after server startup
            </small>
          </div>
        ) : (
          <div className="grid gap-4">
            {metrics.collector_stats.map((collector) => (
              <div
                key={collector.name}
                className="p-4 bg-muted/50 rounded-lg grid grid-cols-[auto_1fr_auto_auto] items-center gap-4 border-l-4"
                style={{ borderLeftColor: collector.status === 'Active' ? STATUS_COLORS.active : STATUS_COLORS.failed }}
              >
                {/* Status indicator */}
                <div
                  className="w-3 h-3 rounded-full"
                  style={{ backgroundColor: collector.status === 'Active' ? STATUS_COLORS.active : STATUS_COLORS.failed }}
                />

                {/* Collector info */}
                <div>
                  <div className="text-lg font-semibold">{collector.name}</div>
                  <div className="text-sm text-muted-foreground mt-1">
                    {collector.version_count} versions &bull;{' '}
                    <Badge variant="outline" style={{ borderColor: getCategoryColor(collector.performance_category), color: getCategoryColor(collector.performance_category) }}>
                      {collector.performance_category}
                    </Badge>
                  </div>
                  {collector.error_message && (
                    <div className="text-xs text-destructive mt-1">
                      {collector.error_message}
                    </div>
                  )}
                </div>

                {/* Response time */}
                <div className="text-right">
                  <div className="text-sm font-bold">
                    {Math.round(collector.response_time)}ms
                  </div>
                  <div className="text-xs text-muted-foreground">
                    Response time
                  </div>
                </div>

                {/* Last updated */}
                <div className="text-right">
                  <div className="text-xs text-muted-foreground">
                    Last updated
                  </div>
                  <div className="text-sm">
                    {new Date(collector.last_collection).toLocaleString()}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </CardContent>
    </Card>
  )
}
