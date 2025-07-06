# VersionWatch Frontend Architecture

This document describes the component structure of the VersionWatch React frontend.

## 📁 Project Structure

```
frontend/src/
├── components/           # Reusable UI components
│   ├── charts/          # Chart components
│   │   ├── BarChart.tsx
│   │   ├── DoughnutChart.tsx
│   │   ├── LineChart.tsx
│   │   └── index.ts
│   ├── layout/          # Layout components
│   │   ├── Header.tsx
│   │   ├── Navigation.tsx
│   │   └── index.ts
│   └── ui/              # Basic UI components
│       ├── Loading.tsx
│       ├── MetricCard.tsx
│       ├── Tooltip.tsx
│       └── index.ts
├── hooks/               # Custom React hooks
│   ├── useDashboardData.ts
│   ├── useTooltipStyles.ts
│   └── index.ts
├── styles/              # Shared styles
│   └── common.ts
├── types/               # TypeScript interfaces
│   └── index.ts
├── views/               # Page/view components
│   ├── Analytics.tsx
│   ├── Collectors.tsx
│   ├── Logs.tsx
│   ├── Overview.tsx
│   └── index.ts
└── App.tsx             # Main application component
```

## 🧩 Component Categories

### Charts (`components/charts/`)
Chart components for data visualization:
- **BarChart**: Displays bar charts with customizable data and colors
- **DoughnutChart**: Circular charts with center totals and legends
- **LineChart**: Line graphs with responsive SVG rendering

### Layout (`components/layout/`)
Application layout components:
- **Header**: Top navigation with logo, refresh controls, and system status
- **Navigation**: Tab-based navigation between different views

### UI (`components/ui/`)
Basic reusable UI components:
- **Loading**: Loading spinner with customizable message
- **MetricCard**: Metric display cards with tooltip support
- **Tooltip**: Hover tooltips with custom content

### Views (`views/`)
Main application pages:
- **Overview**: Dashboard with metrics cards and charts
- **Collectors**: Detailed collector status and information
- **Analytics**: Performance analytics and slowest collectors
- **Logs**: System activity logs and error messages

### Hooks (`hooks/`)
Custom React hooks for business logic:
- **useDashboardData**: Manages API calls, auto-refresh, and state
- **useTooltipStyles**: Injects CSS for tooltip hover effects

### Types (`types/`)
TypeScript interfaces and type definitions:
- **DashboardMetrics**: Main data structure from API
- **CollectorStat**: Individual collector information
- **SystemHealth**: System health metrics
- **ViewType**: Navigation view types

### Styles (`styles/`)
Shared styling objects:
- **commonStyles**: Centralized styles to replace inline CSS

## 🔄 Data Flow

1. **App.tsx** is the main entry point
2. **useDashboardData** hook manages API calls and state
3. **Views** receive metrics data as props
4. **Charts** and **UI components** are purely presentational
5. **Layout components** handle navigation and user interactions

## ✨ Key Features

### Component Reusability
- All components are pure functions with clear interfaces
- Props are strongly typed with TypeScript
- Styles are centralized and consistent

### Performance
- Auto-refresh with configurable intervals
- Efficient data fetching with custom hooks
- Responsive design with CSS-in-JS

### Maintainability
- Clear separation of concerns
- Easy to add new views or components
- Comprehensive error handling

### Accessibility
- Semantic HTML structure
- Keyboard navigation support
- Screen reader friendly tooltips

## 🚀 Adding New Components

### New Chart Type
```typescript
// components/charts/NewChart.tsx
import React from 'react'
import { ChartData } from '../../types'
import { commonStyles } from '../../styles/common'

interface NewChartProps {
  data: ChartData[]
  title: string
}

export const NewChart: React.FC<NewChartProps> = ({ data, title }) => {
  return (
    <div style={commonStyles.card}>
      <h3 style={commonStyles.cardTitle}>{title}</h3>
      {/* Chart implementation */}
    </div>
  )
}
```

### New View
```typescript
// views/NewView.tsx
import React from 'react'
import { DashboardMetrics } from '../types'

interface NewViewProps {
  metrics: DashboardMetrics
}

export const NewView: React.FC<NewViewProps> = ({ metrics }) => {
  return (
    <div>
      {/* View implementation */}
    </div>
  )
}
```

## 🎯 Best Practices

1. **Single Responsibility**: Each component has one clear purpose
2. **Props Interface**: Always define TypeScript interfaces for props
3. **Style Consistency**: Use commonStyles for consistent theming
4. **Error Handling**: Handle loading and error states gracefully
5. **Accessibility**: Include proper ARIA labels and semantic markup
6. **Performance**: Use React.memo() for expensive computations
7. **Testing**: Write unit tests for business logic in hooks 