import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { Annotation } from '@/types/annotation'
import { saveCaptureAnnotations } from '@/services/tauriCommands'
import { useUiStore } from './uiStore'
import { useI18n } from '@/composables/useI18n'

export interface HistoryAction {
  type: 'add' | 'delete' | 'update'
  annotation: Annotation
  index?: number
  previousState?: Annotation
}

export interface CropBox {
  x: number
  y: number
  w: number
  h: number
}

export const useOverlayStore = defineStore('overlay', () => {
  const activeTool = ref<'marker' | 'rect' | 'arrow' | 'text' | 'blur' | 'freedraw' | 'highlight' | 'spotlight' | 'crop' | null>('marker')
  const annotations = ref<Annotation[]>([])
  const undoStack = ref<HistoryAction[]>([])
  const redoStack = ref<HistoryAction[]>([])
  const captureId = ref<string | null>(null)
  const screenshotPath = ref<string>('')
  const monitorInfo = ref<any>(null)
  const nextMarkerNumber = ref<number>(1)
  
  // Pending annotation state
  const pendingAnnotation = ref<Annotation | null>(null)
  const showStepNotes = ref<boolean>(false)
  const defaultShowBadge = ref<boolean>(true)
  const cropBox = ref<CropBox | null>(null)

  // Current active step note inputs
  const guideTitle = ref<string>('')
  const currentColor = ref<string>('#FF6B35')
  const currentStrokeWidth = ref<number>(2)

  /**
   * Reindexes numbers (1, 2, 3...) based on sequential step order (index + 1).
   * If an annotation has showBadge: false (e.g. number deleted/hidden), its number is removed
   * while keeping its position in the guide step sequence (e.g. rec 1, arrow, rec 3).
   */
  function reindexAnnotations() {
    annotations.value.forEach((ann, index) => {
      if (ann.showBadge) {
        ann.number = index + 1
      } else {
        delete ann.number
      }
    })
    nextMarkerNumber.value = annotations.value.length + 1
  }

  function init(id: string, path: string, info?: any) {
    captureId.value = id
    screenshotPath.value = path
    monitorInfo.value = info || null
    annotations.value = []
    undoStack.value = []
    redoStack.value = []
    nextMarkerNumber.value = 1
    activeTool.value = 'marker'
    pendingAnnotation.value = null
    showStepNotes.value = false
    defaultShowBadge.value = true
    cropBox.value = null
    guideTitle.value = ''
    currentColor.value = '#FF6B35'
    currentStrokeWidth.value = 2
  }

  function setTool(tool: 'marker' | 'rect' | 'arrow' | 'text' | 'blur' | 'freedraw' | 'highlight' | 'spotlight' | 'crop' | null) {
    activeTool.value = tool
  }

  function setColor(color: string) {
    currentColor.value = color
  }

  function setStrokeWidth(width: number) {
    currentStrokeWidth.value = width
  }

  function setCropBox(box: CropBox | null) {
    cropBox.value = box
  }

  function addAnnotation(annotation: Annotation) {
    // By default all shapes and marks have badges enabled
    if (annotation.showBadge === undefined) {
      annotation.showBadge = defaultShowBadge.value
    }

    annotations.value.push(annotation)
    reindexAnnotations()

    undoStack.value.push({ type: 'add', annotation })
    redoStack.value = []
  }

  function toggleAnnotationBadge(id: string) {
    const ann = annotations.value.find(a => a.id === id)
    if (!ann) return

    ann.showBadge = !ann.showBadge
    reindexAnnotations()
  }

  function removeAnnotation(id: string) {
    const idx = annotations.value.findIndex(a => a.id === id)
    if (idx !== -1) {
      const annotation = annotations.value[idx]
      annotations.value.splice(idx, 1)
      reindexAnnotations()

      undoStack.value.push({ type: 'delete', annotation, index: idx })
      redoStack.value = []
    }
  }

  function reset() {
    captureId.value = null
    screenshotPath.value = ''
    monitorInfo.value = null
    annotations.value = []
    undoStack.value = []
    redoStack.value = []
    nextMarkerNumber.value = 1
    activeTool.value = 'marker'
    pendingAnnotation.value = null
    showStepNotes.value = false
    cropBox.value = null
    guideTitle.value = ''
  }

  function undo() {
    if (undoStack.value.length === 0) return
    const action = undoStack.value.pop()
    if (!action) return

    if (action.type === 'add') {
      annotations.value = annotations.value.filter(a => a.id !== action.annotation.id)
    } else if (action.type === 'delete') {
      const insertIdx = action.index !== undefined ? action.index : annotations.value.length
      annotations.value.splice(insertIdx, 0, action.annotation)
    }
    reindexAnnotations()
    redoStack.value.push(action)
  }

  function redo() {
    if (redoStack.value.length === 0) return
    const action = redoStack.value.pop()
    if (!action) return

    if (action.type === 'add') {
      annotations.value.push(action.annotation)
    } else if (action.type === 'delete') {
      annotations.value = annotations.value.filter(a => a.id !== action.annotation.id)
    }
    reindexAnnotations()
    undoStack.value.push(action)
  }

  async function saveAndClose(annotatedBase64?: string) {
    if (!captureId.value) return

    const cropX = cropBox.value?.x ?? 0
    const cropY = cropBox.value?.y ?? 0

    const payloads = annotations.value.map(ann => {
      let markerX = 0
      let markerY = 0
      if (ann.type === 'marker') {
        markerX = ann.position.x
        markerY = ann.position.y
      } else if (ann.type === 'rect' || ann.type === 'blur' || ann.type === 'highlight' || ann.type === 'spotlight') {
        markerX = ann.topLeft.x + ann.width / 2
        markerY = ann.topLeft.y + ann.height / 2
      } else if (ann.type === 'arrow') {
        markerX = ann.end.x
        markerY = ann.end.y
      } else if (ann.type === 'text') {
        markerX = ann.position.x
        markerY = ann.position.y
      } else if (ann.type === 'freedraw') {
        if (ann.points && ann.points.length > 0) {
          let minX = ann.points[0].x
          let maxX = ann.points[0].x
          let minY = ann.points[0].y
          let maxY = ann.points[0].y
          for (const pt of ann.points) {
            if (pt.x < minX) minX = pt.x
            if (pt.x > maxX) maxX = pt.x
            if (pt.y < minY) minY = pt.y
            if (pt.y > maxY) maxY = pt.y
          }
          markerX = minX + (maxX - minX) / 2
          markerY = minY + (maxY - minY) / 2
        }
      }

      const dpr = monitorInfo.value?.scaleFactor || window.devicePixelRatio || 1
      const physicalX = Math.max(0, (markerX - cropX) * dpr)
      const physicalY = Math.max(0, (markerY - cropY) * dpr)

      return {
        markerNumber: ann.number ?? 0,
        title: ann.stepNote || (ann.showBadge && ann.number ? `Bước ${ann.number}` : `Ghi chú`),
        description: ann.stepNote || '',
        issueType: 'GuideStep',
        severity: 'Info',
        status: 'Open',
        markerX: Math.round(physicalX),
        markerY: Math.round(physicalY),
        annotationData: JSON.stringify(ann),
        color: ann.color || '#FF6B35',
        strokeWidth: ann.strokeWidth || 2,
        tags: []
      }
    })

    try {
      const uiStore = useUiStore()
      uiStore.setLoading(true)
      await saveCaptureAnnotations(captureId.value, payloads, annotatedBase64)
      reset()
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.annotationsSaved'), type: 'success' })
    } catch (e: any) {
      console.error('[OverlayStore] Failed to save annotations:', e)
      const uiStore = useUiStore()
      const { t } = useI18n()
      uiStore.showToast({ message: t('toasts.failedSaveAnnotations', { msg: e?.message || String(e) }), type: 'error' })
      throw e
    } finally {
      const uiStore = useUiStore()
      uiStore.setLoading(false)
    }
  }

  return {
    activeTool,
    annotations,
    undoStack,
    redoStack,
    captureId,
    screenshotPath,
    monitorInfo,
    nextMarkerNumber,
    pendingAnnotation,
    showStepNotes,
    defaultShowBadge,
    cropBox,
    guideTitle,
    currentColor,
    currentStrokeWidth,
    init,
    setTool,
    setColor,
    setStrokeWidth,
    setCropBox,
    addAnnotation,
    toggleAnnotationBadge,
    removeAnnotation,
    reindexAnnotations,
    reset,
    saveAndClose,
    undo,
    redo,
  }
})
