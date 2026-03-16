import React from 'react'
import { TooltipLegacy } from './TooltipLegacy'
import { Card, CardContent } from "@/components/ui/card"

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
    <Card className="bg-white/95 text-center shadow-md min-h-[140px] flex flex-col justify-center border-[3px] transition-transform hover:scale-[1.02]" style={{ borderColor: color }}>
      <CardContent className="p-6 flex flex-col justify-center">
        <div className="text-[2.5rem] mb-2 drop-shadow">
          {icon}
        </div>
        <div className="text-2xl font-bold mb-2" style={{ color }}>
          {value}
        </div>
        <div className="text-sm text-muted-foreground flex items-center justify-center gap-1">
          {title}
          {tooltip && (
            <span className="text-xs bg-muted text-muted-foreground rounded-full w-4 h-4 inline-flex items-center justify-center cursor-help">
              ?
            </span>
          )}
        </div>
      </CardContent>
    </Card>
  )

  if (tooltip) {
    return <TooltipLegacy content={tooltip}>{cardContent}</TooltipLegacy>
  }

  return cardContent
}
