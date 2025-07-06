export const commonStyles = {
  // Card styles
  card: {
    backgroundColor: 'rgba(255,255,255,0.9)',
    color: '#333',
    padding: '1.5rem',
    borderRadius: '1rem',
    boxShadow: '0 4px 6px rgba(0,0,0,0.1)',
    marginBottom: '2rem'
  },

  cardTitle: {
    margin: '0 0 1rem 0',
    display: 'flex',
    alignItems: 'center',
    gap: '0.5rem'
  },

  // Layout styles
  container: {
    minHeight: '100vh',
    background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
    fontFamily: 'Arial, sans-serif',
    color: 'white'
  },

  header: {
    backgroundColor: 'rgba(255,255,255,0.98)',
    color: '#333',
    padding: '1rem 2rem',
    borderBottom: '3px solid #667eea',
    boxShadow: '0 2px 10px rgba(0,0,0,0.1)'
  },

  headerContent: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center'
  },

  logoContainer: {
    display: 'flex',
    alignItems: 'center',
    gap: '1rem'
  },

  logo: {
    width: '60px',
    height: '60px',
    borderRadius: '12px',
    boxShadow: '0 4px 8px rgba(0,0,0,0.2)'
  },

  title: {
    fontSize: '2.2rem',
    margin: 0,
    background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
    WebkitBackgroundClip: 'text',
    WebkitTextFillColor: 'transparent',
    backgroundClip: 'text'
  },

  subtitle: {
    fontSize: '1rem',
    color: '#666',
    margin: '0.5rem 0 0 0'
  },

  // Navigation styles
  navigation: {
    display: 'flex',
    gap: '0.5rem',
    padding: '1rem 2rem',
    backgroundColor: 'rgba(255,255,255,0.1)'
  },

  navButton: {
    padding: '0.75rem 1.5rem',
    border: 'none',
    borderRadius: '0.5rem',
    cursor: 'pointer',
    fontSize: '0.9rem',
    transition: 'all 0.2s ease',
    display: 'flex',
    alignItems: 'center',
    gap: '0.5rem'
  },

  navButtonActive: {
    backgroundColor: 'rgba(255,255,255,0.2)',
    color: 'white',
    fontWeight: 'bold'
  },

  navButtonInactive: {
    backgroundColor: 'transparent',
    color: 'white',
    fontWeight: 'normal'
  },

  // Content styles
  content: {
    padding: '2rem'
  },

  // Loading styles
  loading: {
    minHeight: '100vh',
    background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    fontFamily: 'Arial, sans-serif'
  },

  loadingCard: {
    textAlign: 'center',
    color: 'white',
    padding: '2rem',
    backgroundColor: 'rgba(255,255,255,0.1)',
    borderRadius: '1rem'
  },

  // Empty state styles
  emptyState: {
    textAlign: 'center',
    padding: '2rem',
    color: '#666',
    fontSize: '0.9rem'
  },

  // Tooltip styles
  tooltipContainer: {
    position: 'relative',
    display: 'inline-block'
  },

  tooltip: {
    position: 'absolute',
    bottom: '130%',
    left: '50%',
    transform: 'translateX(-50%)',
    backgroundColor: '#333',
    color: 'white',
    padding: '0.75rem 1rem',
    borderRadius: '8px',
    fontSize: '0.85rem',
    lineHeight: '1.4',
    maxWidth: '280px',
    width: 'max-content',
    textAlign: 'left',
    boxShadow: '0 4px 12px rgba(0,0,0,0.3)',
    opacity: '0',
    visibility: 'hidden',
    transition: 'opacity 0.3s ease, visibility 0.3s ease',
    zIndex: '1000'
  },

  tooltipArrow: {
    position: 'absolute',
    top: '100%',
    left: '50%',
    transform: 'translateX(-50%)',
    width: '0',
    height: '0',
    borderLeft: '6px solid transparent',
    borderRight: '6px solid transparent',
    borderTop: '6px solid #333'
  },

  // Status colors
  statusColors: {
    active: '#4CAF50',
    failed: '#f44336',
    warning: '#FF9800',
    info: '#2196F3'
  }
} as const 