import React from 'react'
import { ViewType } from '../../types'
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { LayoutDashboard, Wrench, BarChart3, ScrollText } from "lucide-react"

interface NavigationProps {
  currentView: ViewType
  onViewChange: (view: ViewType) => void
}

interface TabConfig {
  view: ViewType
  label: string
  icon: React.ReactNode
}

const tabs: TabConfig[] = [
  { view: 'overview', label: 'Overview', icon: <LayoutDashboard className="h-4 w-4" /> },
  { view: 'collectors', label: 'Collectors', icon: <Wrench className="h-4 w-4" /> },
  { view: 'analytics', label: 'Analytics', icon: <BarChart3 className="h-4 w-4" /> },
  { view: 'logs', label: 'Logs', icon: <ScrollText className="h-4 w-4" /> }
]

export const Navigation: React.FC<NavigationProps> = ({
  currentView,
  onViewChange
}) => {
  return (
    <nav className="bg-secondary px-8 py-2">
      <Tabs value={currentView} onValueChange={(val) => onViewChange(val as ViewType)}>
        <TabsList className="bg-secondary">
          {tabs.map((tab) => (
            <TabsTrigger key={tab.view} value={tab.view} className="gap-2 data-[state=active]:bg-accent text-muted-foreground data-[state=active]:text-foreground">
              {tab.icon} {tab.label}
            </TabsTrigger>
          ))}
        </TabsList>
      </Tabs>
    </nav>
  )
}
