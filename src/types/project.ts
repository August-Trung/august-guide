export interface Project {
  id: string
  name: string
  description: string
  color: string
  isArchived: boolean
  createdAt: string
  updatedAt: string
}

export interface CreateProjectPayload {
  name: string
  description?: string
  color?: string
}

export interface UpdateProjectPayload {
  name?: string
  description?: string
  color?: string
  isArchived?: boolean
}
