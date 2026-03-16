import React from 'react'
import { Button } from "@/components/ui/button"
import { RefreshCw, Pause, Play } from "lucide-react"

interface HeaderProps {
  lastRefresh: string
  isAutoRefresh: boolean
  onRefreshToggle: () => void
  onManualRefresh: () => void
}

export const Header: React.FC<HeaderProps> = ({
  lastRefresh,
  isAutoRefresh,
  onRefreshToggle,
  onManualRefresh
}) => {
  return (
    <header className="bg-background border-b-[3px] border-primary px-8 py-4 shadow-sm">
      <div className="flex justify-between items-center">
        <div className="flex items-center gap-4">
          <img
            src="/logo.png"
            alt="VersionWatch Logo"
            className="w-[60px] h-[60px] rounded-xl shadow-md"
          />
          <div>
            <h1 className="text-4xl font-bold text-primary">VersionWatch</h1>
            <p className="text-sm text-muted-foreground mt-1">
              Software Version Monitoring Dashboard
            </p>
          </div>
        </div>

        <div className="flex items-center gap-4 text-sm text-muted-foreground">
          <div className="text-right">
            <div>Last refresh: {lastRefresh}</div>
            <div className="flex items-center gap-2 mt-2">
              <Button
                variant={isAutoRefresh ? "default" : "outline"}
                size="sm"
                onClick={onRefreshToggle}
              >
                {isAutoRefresh ? <Pause className="h-4 w-4 mr-1" /> : <Play className="h-4 w-4 mr-1" />}
                {isAutoRefresh ? 'Auto-refresh ON' : 'Auto-refresh OFF'}
              </Button>
              <Button size="sm" onClick={onManualRefresh}>
                <RefreshCw className="h-4 w-4 mr-1" />
                Refresh Now
              </Button>
            </div>
          </div>
        </div>
      </div>
    </header>
  )
}
