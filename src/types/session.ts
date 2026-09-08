export interface Session {
  id: string
  projectId: string
  title: string
  description: string
  status: 'active' | 'completed' | 'archived'
  createdAt: string
  updatedAt: string
  completedAt: string | null
  // Computed fields from database JOINs
  issueCount?: number
  captureCount?: number
}

export interface CreateSessionPayload {
  projectId: string
  title: string
  description?: string
}

export interface UpdateSessionPayload {
  title?: string
  description?: string
  status?: 'active' | 'completed' | 'archived'
}
