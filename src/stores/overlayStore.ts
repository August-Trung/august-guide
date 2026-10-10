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
  const activeTool = ref<'select' | 'marker' | 'rect' | 'arrow' | 'text' | 'blur' | 'freedraw' | 'highlight' | 'spotlight' | 'crop' | null>('marker')
  const annotations = ref<Annotation[]>([])
  const selectedAnnotationId = ref<string | null>(null)
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

  const copiedAnnotation = ref<Annotation | null>(null)

  // Current active step note inputs
  const guideTitle = ref<string>('')
  const currentColor = ref<string>('#FF6B35')
  const currentStrokeWidth = ref<number>(2)

  /**
   * Reindexes numbers (1, 2, 3...) sequentially ONLY for annotations with showBadge: true.
   * If an annotation has showBadge: false (e.g. blur or number hidden/deleted), its number is removed.
   */
  function reindexAnnotations() {
    let currentNumber = 1
    annotations.value.forEach((ann) => {
      if (ann.showBadge) {
        ann.number = currentNumber++
      } else {
        delete ann.number
      }
    })
    nextMarkerNumber.value = currentNumber
  }

  function init(id: string, path: string, info?: any) {
    captureId.value = id
    screenshotPath.value = path
    monitorInfo.value = info || null
    annotations.value = []
    selectedAnnotationId.value = null
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
    copiedAnnotation.value = null
  }

  function setTool(tool: 'select' | 'marker' | 'rect' | 'arrow' | 'text' | 'blur' | 'freedraw' | 'highlight' | 'spotlight' | 'crop' | null) {
    activeTool.value = tool
    if (tool !== 'select') {
      selectedAnnotationId.value = null
    }
  }

  function selectAnnotation(id: string | null) {
    selectedAnnotationId.value = id
  }

  function updateAnnotation(updated: Annotation, trackHistory = true, previousState?: Annotation) {
    const idx = annotations.value.findIndex(a => a.id === updated.id)
    if (idx !== -1) {
      const prev = previousState || JSON.parse(JSON.stringify(annotations.value[idx]))
      annotations.value[idx] = updated

      if (trackHistory) {
        undoStack.value.push({
          type: 'update',
          annotation: JSON.parse(JSON.stringify(updated)),
          previousState: prev,
          index: idx
        })
        redoStack.value = []
      }
    }
  }

  function duplicateAnnotation(id: string) {
    const ann = annotations.value.find(a => a.id === id)
    if (!ann) return null

    const newId = crypto?.randomUUID?.() ?? Math.random().toString(36).substring(2, 11)

    const cloned: Annotation = JSON.parse(JSON.stringify(ann))
    cloned.id = newId

    const offset = 20
    if (cloned.type === 'marker') {
      cloned.position = { x: cloned.position.x + offset, y: cloned.position.y + offset }
    } else if (cloned.type === 'rect' || cloned.type === 'blur' || cloned.type === 'highlight' || cloned.type === 'spotlight') {
      cloned.topLeft = { x: cloned.topLeft.x + offset, y: cloned.topLeft.y + offset }
    } else if (cloned.type === 'arrow') {
      cloned.start = { x: cloned.start.x + offset, y: cloned.start.y + offset }
      cloned.end = { x: cloned.end.x + offset, y: cloned.end.y + offset }
    } else if (cloned.type === 'text') {
      cloned.position = { x: cloned.position.x + offset, y: cloned.position.y + offset }
    } else if (cloned.type === 'freedraw') {
      cloned.points = cloned.points.map(p => ({ x: p.x + offset, y: p.y + offset }))
    }

    addAnnotation(cloned)
    selectedAnnotationId.value = cloned.id
    return cloned
  }

  function copySelectedAnnotation(): boolean {
    if (!selectedAnnotationId.value) return false
    const ann = annotations.value.find(a => a.id === selectedAnnotationId.value)
    if (!ann) return false
    copiedAnnotation.value = JSON.parse(JSON.stringify(ann))
    return true
  }

  function pasteAnnotation(): Annotation | null {
    if (!copiedAnnotation.value) return null
    const cloned: Annotation = JSON.parse(JSON.stringify(copiedAnnotation.value))
    cloned.id = crypto?.randomUUID?.() ?? Math.random().toString(36).substring(2, 11)

    const offset = 20
    if (cloned.type === 'marker') {
      cloned.position = { x: cloned.position.x + offset, y: cloned.position.y + offset }
    } else if (cloned.type === 'rect' || cloned.type === 'blur' || cloned.type === 'highlight' || cloned.type === 'spotlight') {
      cloned.topLeft = { x: cloned.topLeft.x + offset, y: cloned.topLeft.y + offset }
    } else if (cloned.type === 'arrow') {
      cloned.start = { x: cloned.start.x + offset, y: cloned.start.y + offset }
      cloned.end = { x: cloned.end.x + offset, y: cloned.end.y + offset }
    } else if (cloned.type === 'text') {
      cloned.position = { x: cloned.position.x + offset, y: cloned.position.y + offset }
    } else if (cloned.type === 'freedraw') {
      cloned.points = cloned.points.map(p => ({ x: p.x + offset, y: p.y + offset }))
    }

    addAnnotation(cloned)
    selectedAnnotationId.value = cloned.id
    copiedAnnotation.value = JSON.parse(JSON.stringify(cloned))
    return cloned
  }

  function setColor(color: string) {
    currentColor.value = color
    if (selectedAnnotationId.value) {
      const ann = annotations.value.find(a => a.id === selectedAnnotationId.value)
      if (ann) {
        const prev = JSON.parse(JSON.stringify(ann))
        ann.color = color
        updateAnnotation(ann, true, prev)
      }
    }
  }

  function setStrokeWidth(width: number) {
    currentStrokeWidth.value = width
    if (selectedAnnotationId.value) {
      const ann = annotations.value.find(a => a.id === selectedAnnotationId.value)
      if (ann) {
        const prev = JSON.parse(JSON.stringify(ann))
        ann.strokeWidth = width
        updateAnnotation(ann, true, prev)
      }
    }
  }

  function setCropBox(box: CropBox | null) {
    cropBox.value = box
  }

  function addAnnotation(annotation: Annotation) {
    if (annotation.showBadge === undefined) {
      annotation.showBadge = annotation.type === 'blur' ? false : defaultShowBadge.value
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
      if (selectedAnnotationId.value === id) {
        selectedAnnotationId.value = null
      }
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
    selectedAnnotationId.value = null
    undoStack.value = []
    redoStack.value = []
    nextMarkerNumber.value = 1
    activeTool.value = 'marker'
    pendingAnnotation.value = null
    showStepNotes.value = false
    cropBox.value = null
    guideTitle.value = ''
    copiedAnnotation.value = null
  }

  function undo() {
    if (undoStack.value.length === 0) return
    const action = undoStack.value.pop()
    if (!action) return

    if (action.type === 'add') {
      annotations.value = annotations.value.filter(a => a.id !== action.annotation.id)
      if (selectedAnnotationId.value === action.annotation.id) {
        selectedAnnotationId.value = null
      }
    } else if (action.type === 'delete') {
      const insertIdx = action.index !== undefined ? action.index : annotations.value.length
      annotations.value.splice(insertIdx, 0, action.annotation)
      selectedAnnotationId.value = action.annotation.id
    } else if (action.type === 'update') {
      if (action.previousState) {
        const idx = annotations.value.findIndex(a => a.id === action.annotation.id)
        if (idx !== -1) {
          annotations.value[idx] = JSON.parse(JSON.stringify(action.previousState))
          selectedAnnotationId.value = action.annotation.id
        }
      }
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
      selectedAnnotationId.value = action.annotation.id
    } else if (action.type === 'delete') {
      annotations.value = annotations.value.filter(a => a.id !== action.annotation.id)
      if (selectedAnnotationId.value === action.annotation.id) {
        selectedAnnotationId.value = null
      }
    } else if (action.type === 'update') {
      const idx = annotations.value.findIndex(a => a.id === action.annotation.id)
      if (idx !== -1) {
        annotations.value[idx] = JSON.parse(JSON.stringify(action.annotation))
        selectedAnnotationId.value = action.annotation.id
      }
    }
    reindexAnnotations()
    undoStack.value.push(action)
  }

  async function saveAndClose(annotatedBase64?: string) {
    if (!captureId.value) return

    const cropX = cropBox.value?.x ?? 0
    const cropY = cropBox.value?.y ?? 0

    let payloads = annotations.value.map(ann => {
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

    if (payloads.length === 0) {
      payloads = [{
        markerNumber: 1,
        title: guideTitle.value || 'Ảnh chụp màn hình',
        description: '',
        issueType: 'GuideStep',
        severity: 'Info',
        status: 'Open',
        markerX: 0,
        markerY: 0,
        annotationData: JSON.stringify({ type: 'full_screenshot' }),
        color: '#FF6B35',
        strokeWidth: 2,
        tags: []
      }]
    }

    try {
      const uiStore = useUiStore()
      uiStore.setLoading(true)
      await saveCaptureAnnotations(captureId.value, payloads, annotatedBase64)

      if (annotatedBase64) {
        try {
          const { autoSaveScreenshotCopy } = await import('@/services/tauriCommands')
          const savedPath = await autoSaveScreenshotCopy(annotatedBase64, guideTitle.value || 'August_Guide')
          if (savedPath) {
            console.log('[OverlayStore] Auto-saved screenshot copy to:', savedPath)
          }
        } catch (copyErr) {
          console.error('[OverlayStore] Failed to auto-save screenshot copy:', copyErr)
        }
      }

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
    selectedAnnotationId,
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
    selectAnnotation,
    updateAnnotation,
    duplicateAnnotation,
    copiedAnnotation,
    copySelectedAnnotation,
    pasteAnnotation,
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
