export interface DashboardMetrics {
  total_collectors: number
  active_collectors: number
  failed_collectors: number
  total_versions: number
  last_updated: string
  collector_stats: CollectorStat[]
  system_health: SystemHealth
}

export interface CollectorStat {
  name: string
  version_count: number
  status: string
  last_collection: string
  performance_category: string
  response_time: number
  error_message?: string
}

export interface SystemHealth {
  overall_status: string
  anomalies_detected: number
  success_rate: number
  average_response_time: number
}

export type ViewType = 'overview' | 'collectors' | 'analytics' | 'logs'

export interface ChartData {
  name: string
  value: number
  color: string
}

export interface LineChartData {
  name: string
  value: number
} 