import { useEffect } from 'react'

export const useTooltipStyles = () => {
  useEffect(() => {
    // Add styles for tooltips
    const style = document.createElement('style')
    style.textContent = `
      .tooltip-container:hover .tooltip {
        opacity: 1 !important;
        visibility: visible !important;
      }
    `
    document.head.appendChild(style)

    return () => {
      document.head.removeChild(style)
    }
  }, [])
} 