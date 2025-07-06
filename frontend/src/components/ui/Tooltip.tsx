import React from 'react'
import { commonStyles } from '../../styles/common'

interface TooltipProps {
  content: string
  children: React.ReactNode
}

export const Tooltip: React.FC<TooltipProps> = ({ content, children }) => {
  return (
    <div style={commonStyles.tooltipContainer} className="tooltip-container">
      {children}
      <div style={commonStyles.tooltip} className="tooltip">
        {content}
        <div style={commonStyles.tooltipArrow}></div>
      </div>
    </div>
  )
} 