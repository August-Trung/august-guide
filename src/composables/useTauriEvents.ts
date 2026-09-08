import { onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { listenToEvent } from '@/services/tauriEvents'
import { useProjectStore } from '@/stores/projectStore'
import { useSessionStore } from '@/stores/sessionStore'
import { triggerCapture, openOverlay } from '@/services/tauriCommands'

// Guard: prevent concurrent capture workflows
let isCapturing = false

export async function startCaptureWorkflow(projectStore: any, sessionStore: any) {
  if (isCapturing) {
    console.log('[Capture] Already capturing, ignoring duplicate trigger')
    return
  }
  isCapturing = true

  try {
    console.log('[Capture] Trigger received — starting workflow')

    // 1. Ensure project list is loaded
    if (!projectStore.projects || projectStore.projects.length === 0) {
      await projectStore.fetchProjects()
    }

    // 2. Resolve a valid project ID
    let projectId: string | null = null

    const realProjects = projectStore.projects
    if (realProjects?.length) {
      // Use the currently active project if it exists in the list, otherwise first
      const active = realProjects.find((p: any) => p.id === projectStore.activeProjectId)
      projectId = active ? active.id : realProjects[0].id
    } else {
      // No projects — create one automatically
      console.log('[Capture] No projects found, creating Default project...')
      const newProject = await projectStore.createProject({
        name: 'Default',
        description: 'Automatically created via global shortcut',
      })
      projectId = newProject.id
    }

    console.log('[Capture] Using project:', projectId)

    // 3. Find or create an active session
    await sessionStore.fetchSessionsByProject(projectId)
    let activeSession = sessionStore.sessions?.find((s: any) => s.status === 'active')

    if (!activeSession) {
      const dateStr = new Date().toLocaleDateString('en-GB').replace(/\//g, '-')
      const title = `Guide ${dateStr}`
      console.log('[Capture] Creating session:', title)
      activeSession = await sessionStore.createSession({
        projectId,
        title,
        description: 'Automatically created via global shortcut',
      })
    }

    console.log('[Capture] Session ready:', activeSession.id)

    // 4. Take screenshot
    console.log('[Capture] Taking screenshot...')
    const result = await triggerCapture(activeSession.id)
    console.log('[Capture] Screenshot saved, captureId:', result.captureId)

    // 5. Open the overlay window
    console.log('[Capture] Opening overlay...')
    await openOverlay(result.captureId, result.screenshotPath)
    console.log('[Capture] Overlay opened successfully')
  } catch (error) {
    console.error('[Capture] Workflow failed:', error)
  } finally {
    isCapturing = false
  }
}

/**
 * Composable to listen for Tauri events in the frontend.
 * Catches the global hotkey capture trigger and starts the capture workflow.
 */
export function useTauriEvents() {
  const router = useRouter()
  const projectStore = useProjectStore()
  const sessionStore = useSessionStore()
  let unlistenTrigger: (() => void) | null = null
  let unlistenNavigate: (() => void) | null = null

  const handleCaptureTrigger = async () => {
    await startCaptureWorkflow(projectStore, sessionStore)
  }

  onMounted(async () => {
    try {
      unlistenTrigger = await listenToEvent('capture:trigger', handleCaptureTrigger)
      console.log('[Events] capture:trigger listener registered')
    } catch (e) {
      console.error('[Events] Failed to register capture:trigger listener:', e)
    }

    try {
      unlistenNavigate = await listenToEvent<string>('navigate', (path) => {
        console.log('[Events] Received navigate event:', path)
        router.push(path)
      })
      console.log('[Events] navigate listener registered')
    } catch (e) {
      console.error('[Events] Failed to register navigate listener:', e)
    }
  })

  onUnmounted(() => {
    if (unlistenTrigger) {
      unlistenTrigger()
      console.log('[Events] capture:trigger listener removed')
    }
    if (unlistenNavigate) {
      unlistenNavigate()
      console.log('[Events] navigate listener removed')
    }
  })
}
