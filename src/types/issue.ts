import { Annotation } from './annotation'

export type IssueType = 'Step' | 'GuideStep' | 'Note' | 'Warning' | 'Tip' | 'Bug' | 'UI' | 'UX' | 'Suggestion' | 'Requirement' | 'Question' | string
export type Severity = 'Critical' | 'Major' | 'Minor' | 'Info' | string
export type IssueStatus = 'Draft' | 'Open' | 'In Progress' | 'Resolved' | 'Closed' | string

export interface Tag {
  id: string
  name: string
  color: string
}

export interface Issue {
  id: string
  captureId: string
  sessionId: string
  projectId: string
  markerNumber: number
  title: string
  description: string
  issueType: IssueType
  severity: Severity
  status: IssueStatus
  markerX: number
  markerY: number
  annotationData: Annotation // Parsed Annotation object on the frontend
  color: string
  strokeWidth: number
  cropPath: string | null
  createdAt: string
  updatedAt: string
  tags?: Tag[]
}

export interface CreateIssuePayload {
  markerNumber: number
  title: string
  description: string
  issueType: string
  severity: string
  status: string
  markerX: number
  markerY: number
  annotationData: string // JSON string
  color: string
  strokeWidth: number
  tags: string[]
}

export interface UpdateIssuePayload {
  title?: string
  description?: string
  issueType?: string
  severity?: string
  status?: string
  tags?: string[]
}
