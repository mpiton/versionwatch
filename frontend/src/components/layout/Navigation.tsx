import React from 'react'
import { ViewType } from '../../types'
import { commonStyles } from '../../styles/common'

interface NavigationProps {
  currentView: ViewType
  onViewChange: (view: ViewType) => void
}

interface TabConfig {
  view: ViewType
  label: string
  icon: string
}

const tabs: TabConfig[] = [
  { view: 'overview', label: 'Overview', icon: '📊' },
  { view: 'collectors', label: 'Collectors', icon: '🔧' },
  { view: 'analytics', label: 'Analytics', icon: '📈' },
  { view: 'logs', label: 'Logs', icon: '📋' }
]

export const Navigation: React.FC<NavigationProps> = ({
  currentView,
  onViewChange
}) => {
  return (
    <nav style={commonStyles.navigation}>
      {tabs.map((tab) => (
        <button
          key={tab.view}
          onClick={() => onViewChange(tab.view)}
          style={{
            ...commonStyles.navButton,
            ...(currentView === tab.view 
              ? commonStyles.navButtonActive 
              : commonStyles.navButtonInactive
            )
          }}
        >
          {tab.icon} {tab.label}
        </button>
      ))}
    </nav>
  )
} 