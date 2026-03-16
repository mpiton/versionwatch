export const STATUS_COLORS = {
  active: '#4CAF50',
  failed: '#f44336',
  warning: '#FF9800',
  info: '#2196F3',
} as const

export const PERFORMANCE_MAPPING: Record<string, { display: string; color: string }> = {
  'No Data': { display: 'No Data', color: '#9E9E9E' },
  'Low Volume': { display: 'Low', color: STATUS_COLORS.active },
  'Medium Volume': { display: 'Medium', color: STATUS_COLORS.warning },
  'High Volume': { display: 'High', color: STATUS_COLORS.info },
  'Very High Volume': { display: 'Very High', color: '#9C27B0' },
}

/** Get color for a performance category (accepts raw backend name or display name) */
export const getCategoryColor = (category: string): string => {
  // Try raw backend name first
  const mapping = PERFORMANCE_MAPPING[category]
  if (mapping) return mapping.color

  // Try display name
  const byDisplay = Object.values(PERFORMANCE_MAPPING).find(m => m.display === category)
  if (byDisplay) return byDisplay.color

  return '#9E9E9E'
}
