/**
 * Utility functions for date formatting.
 */

/**
 * Format ISO string to "Month Day, Year" (e.g. "Jun 15, 2026")
 */
export function formatDate(isoString: string): string {
  try {
    const d = new Date(isoString)
    return d.toLocaleDateString(undefined, {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    })
  } catch (e) {
    return isoString
  }
}

/**
 * Format ISO string to relative time (e.g. "2 hours ago")
 */
export function formatRelativeTime(isoString: string): string {
  try {
    const d = new Date(isoString)
    const now = new Date()
    const diffMs = now.getTime() - d.getTime()
    const diffMins = Math.floor(diffMs / 60000)
    
    if (diffMins < 1) return 'Just now'
    if (diffMins < 60) return `${diffMins}m ago`
    
    const diffHours = Math.floor(diffMins / 60)
    if (diffHours < 24) return `${diffHours}h ago`
    
    const diffDays = Math.floor(diffHours / 24)
    if (diffDays === 1) return 'Yesterday'
    if (diffDays < 7) return `${diffDays}d ago`
    
    return formatDate(isoString)
  } catch (e) {
    return isoString
  }
}

/**
 * Format ISO string to "Month Day, Year H:MM AM/PM" (e.g. "Jun 15, 2026 10:30 AM")
 */
export function formatDateTime(isoString: string): string {
  try {
    const d = new Date(isoString)
    return d.toLocaleDateString(undefined, {
      month: 'short',
      day: 'numeric',
      year: 'numeric',
      hour: 'numeric',
      minute: '2-digit',
    })
  } catch (e) {
    return isoString
  }
}
