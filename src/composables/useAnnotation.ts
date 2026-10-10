import { ref, Ref, watch } from 'vue'
import { useOverlayStore } from '@/stores/overlayStore'
import { useUiStore } from '@/stores/uiStore'
import { useCanvas } from './useCanvas'
import { useI18n } from './useI18n'
import type { 
  Point, 
  MarkerAnnotation, 
  RectAnnotation, 
  ArrowAnnotation, 
  TextAnnotation, 
  BlurAnnotation, 
  HighlightAnnotation, 
  SpotlightAnnotation,
  FreeDrawAnnotation,
  Annotation 
} from '@/types/annotation'

// Utility to generate unique IDs
const generateId = () => {
  return typeof crypto?.randomUUID === 'function' 
    ? crypto.randomUUID() 
    : Math.random().toString(36).substring(2, 11)
}

export function useAnnotation(
  markerCanvasRef: Ref<HTMLCanvasElement | null>,
  drawingCanvasRef: Ref<HTMLCanvasElement | null>,
  getMarkerCtx: () => CanvasRenderingContext2D | null,
  getDrawingCtx: () => CanvasRenderingContext2D | null
) {
  const overlayStore = useOverlayStore()
  const uiStore = useUiStore()
  const { t } = useI18n()

  const { 
    getCanvasCoords, 
    clearCanvas, 
    renderMarker, 
    renderRect, 
    renderArrow, 
    renderText,
    renderFreeDraw,
    renderHighlight,
    renderBlur,
    renderSpotlight
  } = useCanvas()

  const isDrawing = ref(false)
  const startPoint = ref<Point | null>(null)

  // Floating text input state
  const textInputState = ref({
    visible: false,
    x: 0,
    y: 0,
    text: ''
  })

  const hoveredAnnotationId = ref<string | null>(null)
  const interactionMode = ref<'move' | 'resize' | null>(null)
  const activeHandle = ref<string | null>(null)
  const initialAnnotationState = ref<Annotation | null>(null)
  const customCursor = ref<string | null>(null)
  
  const contextMenuState = ref({
    visible: false,
    x: 0,
    y: 0,
    annotationId: ''
  })

  /**
   * Helper to render a single annotation on a given 2D context.
   */
  const drawAnnotation = (
    ctx: CanvasRenderingContext2D,
    ann: Annotation,
    isPending = false
  ) => {
    const color = ann.color || overlayStore.currentColor || '#FF6B35'
    const strokeWidth = ann.strokeWidth || overlayStore.currentStrokeWidth || 2
    const width = window.innerWidth
    const height = window.innerHeight

    if (ann.type === 'marker') {
      renderMarker(ctx, ann.position.x, ann.position.y, ann.number || 0, isPending, color)
    } else if (ann.type === 'rect') {
      renderRect(ctx, ann.topLeft.x, ann.topLeft.y, ann.width, ann.height, isPending, color, strokeWidth)
      if (ann.showBadge && ann.number) {
        renderMarker(ctx, ann.topLeft.x, ann.topLeft.y - 20, ann.number, isPending, color)
      }
    } else if (ann.type === 'arrow') {
      const hasBadge = !!(ann.showBadge && ann.number)
      renderArrow(ctx, ann.start.x, ann.start.y, ann.end.x, ann.end.y, isPending, color, Math.max(3, strokeWidth + 1), hasBadge)
      if (hasBadge) {
        renderMarker(ctx, ann.start.x, ann.start.y, ann.number || 0, isPending, color)
      }
    } else if (ann.type === 'text') {
      renderText(ctx, ann.position.x, ann.position.y, ann.text, '#FFFFFF', 'rgba(15, 17, 23, 0.85)')
      if (ann.showBadge && ann.number) {
        renderMarker(ctx, ann.position.x - 20, ann.position.y + 12, ann.number, isPending, color)
      }
    } else if (ann.type === 'blur') {
      const screenshotCanvas = drawingCanvasRef.value?.parentElement?.querySelector('.z-screenshot') as HTMLCanvasElement | null
      renderBlur(ctx, ann.topLeft.x, ann.topLeft.y, ann.width, ann.height, screenshotCanvas)
      if (ann.showBadge && ann.number) {
        renderMarker(ctx, ann.topLeft.x, ann.topLeft.y - 20, ann.number, isPending, color)
      }
    } else if (ann.type === 'highlight') {
      renderHighlight(ctx, ann.topLeft.x, ann.topLeft.y, ann.width, ann.height, isPending)
      if (ann.showBadge && ann.number) {
        renderMarker(ctx, ann.topLeft.x, ann.topLeft.y - 20, ann.number, isPending, color)
      }
    } else if (ann.type === 'spotlight') {
      renderSpotlight(ctx, ann.topLeft.x, ann.topLeft.y, ann.width, ann.height, width, height, isPending, color, strokeWidth)
      if (ann.showBadge && ann.number) {
        renderMarker(ctx, ann.topLeft.x, ann.topLeft.y - 20, ann.number, isPending, color)
      }
    } else if (ann.type === 'freedraw') {
      renderFreeDraw(ctx, ann.points, color, strokeWidth || 3)
      if (ann.showBadge && ann.number && ann.points.length > 0) {
        renderMarker(ctx, ann.points[0].x, ann.points[0].y, ann.number, isPending, color)
      }
    }
  }

  /**
   * Clears and redraws the committed canvas layer with all confirmed annotations.
   */
  const redrawCommittedCanvas = () => {
    const ctx = getMarkerCtx()
    const canvas = markerCanvasRef.value
    if (!ctx || !canvas) return

    clearCanvas(canvas, ctx)

    // Render dimmed mask outside cropBox if cropped
    if (overlayStore.cropBox) {
      const { x, y, w, h } = overlayStore.cropBox
      const width = window.innerWidth
      const height = window.innerHeight
      ctx.save()
      ctx.fillStyle = 'rgba(0, 0, 0, 0.60)'
      ctx.fillRect(0, 0, width, height)
      ctx.clearRect(x, y, w, h)
      ctx.strokeStyle = '#38BDF8'
      ctx.lineWidth = 1.5
      ctx.setLineDash([4, 4])
      ctx.strokeRect(x, y, w, h)
      ctx.restore()
    }

    for (const ann of overlayStore.annotations) {
      drawAnnotation(ctx, ann, false)
    }
  }

  /**
   * Renders the preview of the pending annotation on the active drawing canvas layer.
   */
  const drawPendingPreview = () => {
    const ctx = getDrawingCtx()
    const canvas = drawingCanvasRef.value
    if (!ctx || !canvas) return

    clearCanvas(canvas, ctx)

    const ann = overlayStore.pendingAnnotation
    if (!ann) return

    drawAnnotation(ctx, ann, true)
  }

  // Handle mousedown events on top drawing canvas
  const handleMouseDown = (e: MouseEvent) => {
    // Left click only for drawing
    if (e.button !== 0) return

    // Close context menu if open
    if (contextMenuState.value.visible) {
      contextMenuState.value.visible = false
      if (!findAnnotationAt(getCanvasCoords(drawingCanvasRef.value!, e))) {
        clearHover()
      }
      return
    }

    // If text input is open, handle clicking outside
    if (textInputState.value.visible) {
      if (textInputState.value.text.trim()) {
        commitTextInput()
      } else {
        cancelTextInput()
      }
      return
    }

    if (!drawingCanvasRef.value || !overlayStore.activeTool) return
    const coords = getCanvasCoords(drawingCanvasRef.value, e)
    const color = overlayStore.currentColor || '#FF6B35'
    const strokeWidth = overlayStore.currentStrokeWidth || 2

    // 1. Check if clicking on handles of the currently selected annotation
    if (overlayStore.selectedAnnotationId) {
      const selectedAnn = overlayStore.annotations.find(a => a.id === overlayStore.selectedAnnotationId)
      if (selectedAnn) {
        const handle = findHandleAt(coords, selectedAnn)
        if (handle) {
          interactionMode.value = handle.type === 'center' ? 'move' : 'resize'
          activeHandle.value = handle.type
          startPoint.value = coords
          initialAnnotationState.value = JSON.parse(JSON.stringify(selectedAnn))
          return
        }
      }
    }

    // 2. Check if clicking on any existing annotation body
    const clickedAnn = findAnnotationAt(coords)
    if (clickedAnn) {
      if (overlayStore.activeTool === 'select' || overlayStore.selectedAnnotationId === clickedAnn.id || clickedAnn.type === 'marker') {
        overlayStore.selectAnnotation(clickedAnn.id)
        interactionMode.value = 'move'
        startPoint.value = coords
        initialAnnotationState.value = JSON.parse(JSON.stringify(clickedAnn))
        redrawCommittedCanvas()
        drawSelectionPreview()
        return
      }
    }

    // 3. If in select tool and clicked on empty space -> deselect
    if (overlayStore.activeTool === 'select') {
      overlayStore.selectAnnotation(null)
      redrawCommittedCanvas()
      const ctx = getDrawingCtx()
      if (ctx) clearCanvas(drawingCanvasRef.value, ctx)
      return
    }

    // 4. In other drawing tools: if an annotation was selected, clear selection before drawing
    if (overlayStore.selectedAnnotationId) {
      overlayStore.selectAnnotation(null)
      redrawCommittedCanvas()
      const ctx = getDrawingCtx()
      if (ctx) clearCanvas(drawingCanvasRef.value, ctx)
    }

    if (overlayStore.activeTool === 'marker') {
      // Place numbered marker directly
      const markerAnn: MarkerAnnotation = {
        id: generateId(),
        type: 'marker',
        position: coords,
        showBadge: true,
        color,
        strokeWidth
      }
      overlayStore.addAnnotation(markerAnn)
    } else if (
      overlayStore.activeTool === 'rect' || 
      overlayStore.activeTool === 'arrow' || 
      overlayStore.activeTool === 'blur' || 
      overlayStore.activeTool === 'highlight' ||
      overlayStore.activeTool === 'spotlight' ||
      overlayStore.activeTool === 'crop'
    ) {
      isDrawing.value = true
      startPoint.value = coords
    } else if (overlayStore.activeTool === 'text') {
      textInputState.value = {
        visible: true,
        x: coords.x,
        y: coords.y,
        text: ''
      }
    } else if (overlayStore.activeTool === 'freedraw') {
      isDrawing.value = true
      startPoint.value = coords
      
      const freeAnn: FreeDrawAnnotation = {
        id: generateId(),
        type: 'freedraw',
        points: [coords],
        showBadge: overlayStore.defaultShowBadge,
        color,
        strokeWidth: strokeWidth || 3
      }
      overlayStore.pendingAnnotation = freeAnn
    }
  }

  // Handle mousemove events on drawing canvas
  const handleMouseMove = (e: MouseEvent) => {
    if (!drawingCanvasRef.value || !getDrawingCtx()) return
    const coords = getCanvasCoords(drawingCanvasRef.value, e)
    const ctx = getDrawingCtx()!
    const canvas = drawingCanvasRef.value!
    const color = overlayStore.currentColor || '#FF6B35'
    const strokeWidth = overlayStore.currentStrokeWidth || 2
    const width = window.innerWidth
    const height = window.innerHeight

    // Hover hit-testing and cursor updating when not dragging
    if (!isDrawing.value && !interactionMode.value && !textInputState.value.visible) {
      if (overlayStore.selectedAnnotationId) {
        const selectedAnn = overlayStore.annotations.find(a => a.id === overlayStore.selectedAnnotationId)
        if (selectedAnn) {
          const handle = findHandleAt(coords, selectedAnn)
          if (handle) {
            customCursor.value = handle.cursor
            return
          }
        }
      }

      const ann = findAnnotationAt(coords)
      if (ann) {
        if (ann.id === overlayStore.selectedAnnotationId || overlayStore.activeTool === 'select' || ann.type === 'marker') {
          customCursor.value = 'move'
        } else {
          customCursor.value = 'pointer'
        }

        if (hoveredAnnotationId.value !== ann.id && !overlayStore.selectedAnnotationId) {
          hoveredAnnotationId.value = ann.id
          drawHoverHighlight()
        }
      } else {
        customCursor.value = null
        if (hoveredAnnotationId.value !== null) {
          hoveredAnnotationId.value = null
          if (!overlayStore.selectedAnnotationId) {
            clearCanvas(canvas, ctx)
          } else {
            drawSelectionPreview()
          }
        }
      }
      return
    }

    // Handle moving an annotation
    if (interactionMode.value === 'move' && startPoint.value && initialAnnotationState.value && overlayStore.selectedAnnotationId) {
      const dx = coords.x - startPoint.value.x
      const dy = coords.y - startPoint.value.y
      const initial = initialAnnotationState.value
      const current = overlayStore.annotations.find(a => a.id === overlayStore.selectedAnnotationId)
      if (!current) return

      if (current.type === 'marker' && initial.type === 'marker') {
        current.position = { x: initial.position.x + dx, y: initial.position.y + dy }
      } else if (
        (current.type === 'rect' || current.type === 'blur' || current.type === 'highlight' || current.type === 'spotlight') &&
        (initial.type === 'rect' || initial.type === 'blur' || initial.type === 'highlight' || initial.type === 'spotlight')
      ) {
        current.topLeft = { x: initial.topLeft.x + dx, y: initial.topLeft.y + dy }
      } else if (current.type === 'arrow' && initial.type === 'arrow') {
        current.start = { x: initial.start.x + dx, y: initial.start.y + dy }
        current.end = { x: initial.end.x + dx, y: initial.end.y + dy }
      } else if (current.type === 'text' && initial.type === 'text') {
        current.position = { x: initial.position.x + dx, y: initial.position.y + dy }
      } else if (current.type === 'freedraw' && initial.type === 'freedraw') {
        current.points = initial.points.map(p => ({ x: p.x + dx, y: p.y + dy }))
      }

      redrawCommittedCanvas()
      drawSelectionPreview()
      return
    }

    // Handle resizing an annotation
    if (interactionMode.value === 'resize' && startPoint.value && initialAnnotationState.value && overlayStore.selectedAnnotationId) {
      const dx = coords.x - startPoint.value.x
      const dy = coords.y - startPoint.value.y
      const initial = initialAnnotationState.value
      const current = overlayStore.annotations.find(a => a.id === overlayStore.selectedAnnotationId)
      if (!current) return

      if (
        (current.type === 'rect' || current.type === 'blur' || current.type === 'highlight' || current.type === 'spotlight') &&
        (initial.type === 'rect' || initial.type === 'blur' || initial.type === 'highlight' || initial.type === 'spotlight')
      ) {
        const handle = activeHandle.value
        let x = initial.topLeft.x
        let y = initial.topLeft.y
        let w = initial.width
        let h = initial.height

        if (handle === 'br') {
          w = Math.max(10, initial.width + dx)
          h = Math.max(10, initial.height + dy)
        } else if (handle === 'r') {
          w = Math.max(10, initial.width + dx)
        } else if (handle === 'b') {
          h = Math.max(10, initial.height + dy)
        } else if (handle === 'tl') {
          const newW = initial.width - dx
          const newH = initial.height - dy
          if (newW > 10) { x = initial.topLeft.x + dx; w = newW }
          if (newH > 10) { y = initial.topLeft.y + dy; h = newH }
        } else if (handle === 'tr') {
          const newH = initial.height - dy
          w = Math.max(10, initial.width + dx)
          if (newH > 10) { y = initial.topLeft.y + dy; h = newH }
        } else if (handle === 'bl') {
          const newW = initial.width - dx
          h = Math.max(10, initial.height + dy)
          if (newW > 10) { x = initial.topLeft.x + dx; w = newW }
        } else if (handle === 't') {
          const newH = initial.height - dy
          if (newH > 10) { y = initial.topLeft.y + dy; h = newH }
        } else if (handle === 'l') {
          const newW = initial.width - dx
          if (newW > 10) { x = initial.topLeft.x + dx; w = newW }
        }

        current.topLeft = { x, y }
        current.width = w
        current.height = h
      } else if (current.type === 'arrow' && initial.type === 'arrow') {
        if (activeHandle.value === 'start') {
          current.start = coords
        } else if (activeHandle.value === 'end') {
          current.end = coords
        }
      }

      redrawCommittedCanvas()
      drawSelectionPreview()
      return
    }

    if (!isDrawing.value || !startPoint.value) return

    clearCanvas(canvas, ctx)

    if (overlayStore.activeTool === 'rect') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const w = Math.abs(coords.x - startPoint.value.x)
      const h = Math.abs(coords.y - startPoint.value.y)

      renderRect(ctx, x, y, w, h, true, color, strokeWidth)
      if (overlayStore.defaultShowBadge) {
        renderMarker(ctx, x, y - 20, overlayStore.nextMarkerNumber, true, color)
      }
    } else if (overlayStore.activeTool === 'arrow') {
      const hasBadge = overlayStore.defaultShowBadge
      renderArrow(ctx, startPoint.value.x, startPoint.value.y, coords.x, coords.y, true, color, Math.max(3, strokeWidth + 1), hasBadge)
      if (hasBadge) {
        renderMarker(ctx, startPoint.value.x, startPoint.value.y, overlayStore.nextMarkerNumber, true, color)
      }
    } else if (overlayStore.activeTool === 'blur') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const w = Math.abs(coords.x - startPoint.value.x)
      const h = Math.abs(coords.y - startPoint.value.y)

      const screenshotCanvas = drawingCanvasRef.value?.parentElement?.querySelector('.z-screenshot') as HTMLCanvasElement | null
      renderBlur(ctx, x, y, w, h, screenshotCanvas)
    } else if (overlayStore.activeTool === 'highlight') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const w = Math.abs(coords.x - startPoint.value.x)
      const h = Math.abs(coords.y - startPoint.value.y)

      renderHighlight(ctx, x, y, w, h, true)
      if (overlayStore.defaultShowBadge) {
        renderMarker(ctx, x, y - 20, overlayStore.nextMarkerNumber, true, color)
      }
    } else if (overlayStore.activeTool === 'spotlight') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const w = Math.abs(coords.x - startPoint.value.x)
      const h = Math.abs(coords.y - startPoint.value.y)

      renderSpotlight(ctx, x, y, w, h, width, height, true, color, strokeWidth)
      if (overlayStore.defaultShowBadge) {
        renderMarker(ctx, x, y - 20, overlayStore.nextMarkerNumber, true, color)
      }
    } else if (overlayStore.activeTool === 'crop') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const w = Math.abs(coords.x - startPoint.value.x)
      const h = Math.abs(coords.y - startPoint.value.y)

      ctx.save()
      ctx.fillStyle = 'rgba(0, 0, 0, 0.55)'
      ctx.fillRect(0, 0, width, height)
      ctx.clearRect(x, y, w, h)

      ctx.strokeStyle = '#38BDF8'
      ctx.lineWidth = 2
      ctx.setLineDash([6, 4])
      ctx.strokeRect(x, y, w, h)

      const handleSize = 10
      ctx.fillStyle = '#38BDF8'
      ctx.fillRect(x - 2, y - 2, handleSize, 4)
      ctx.fillRect(x - 2, y - 2, 4, handleSize)
      ctx.fillRect(x + w - handleSize + 2, y - 2, handleSize, 4)
      ctx.fillRect(x + w - 2, y - 2, 4, handleSize)
      ctx.fillRect(x - 2, y + h - 2, handleSize, 4)
      ctx.fillRect(x - 2, y + h - handleSize + 2, 4, handleSize)
      ctx.fillRect(x + w - handleSize + 2, y + h - 2, handleSize, 4)
      ctx.fillRect(x + w - 2, y + h - handleSize + 2, 4, handleSize)

      if (w > 40 && h > 20) {
        const text = `${Math.round(w)} × ${Math.round(h)} px`
        ctx.font = '12px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif'
        ctx.fillStyle = 'rgba(15, 23, 42, 0.9)'
        const textW = ctx.measureText(text).width + 12
        const badgeY = y > 26 ? y - 26 : y + 6
        ctx.beginPath()
        if (ctx.roundRect) {
          ctx.roundRect(x + 4, badgeY, textW, 20, 4)
        } else {
          ctx.rect(x + 4, badgeY, textW, 20)
        }
        ctx.fill()
        ctx.fillStyle = '#38BDF8'
        ctx.fillText(text, x + 10, badgeY + 14)
      }
      ctx.restore()
    } else if (overlayStore.activeTool === 'freedraw') {
      const ann = overlayStore.pendingAnnotation
      if (ann && ann.type === 'freedraw') {
        ann.points.push(coords)
        renderFreeDraw(ctx, ann.points, color, strokeWidth || 3)
      }
    }
  }

  // Handle mouseup events on drawing canvas
  const handleMouseUp = (e: MouseEvent) => {
    if (interactionMode.value) {
      if (overlayStore.selectedAnnotationId && initialAnnotationState.value) {
        const current = overlayStore.annotations.find(a => a.id === overlayStore.selectedAnnotationId)
        if (current) {
          const hasChanged = JSON.stringify(current) !== JSON.stringify(initialAnnotationState.value)
          if (hasChanged) {
            overlayStore.updateAnnotation(current, true, initialAnnotationState.value)
          }
        }
      }
      interactionMode.value = null
      activeHandle.value = null
      initialAnnotationState.value = null
      startPoint.value = null
      drawSelectionPreview()
      return
    }

    if (!isDrawing.value || !startPoint.value || !drawingCanvasRef.value) return
    isDrawing.value = false

    const coords = getCanvasCoords(drawingCanvasRef.value, e)
    const color = overlayStore.currentColor || '#FF6B35'
    const strokeWidth = overlayStore.currentStrokeWidth || 2
    const ctx = getDrawingCtx()

    if (ctx) clearCanvas(drawingCanvasRef.value, ctx)

    if (overlayStore.activeTool === 'rect') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const width = Math.abs(coords.x - startPoint.value.x)
      const height = Math.abs(coords.y - startPoint.value.y)

      if (width > 5 && height > 5) {
        const rectAnn: RectAnnotation = {
          id: generateId(),
          type: 'rect',
          topLeft: { x, y },
          width,
          height,
          showBadge: overlayStore.defaultShowBadge,
          color,
          strokeWidth
        }
        overlayStore.addAnnotation(rectAnn)
      }
    } else if (overlayStore.activeTool === 'arrow') {
      const dx = coords.x - startPoint.value.x
      const dy = coords.y - startPoint.value.y
      const distance = Math.sqrt(dx * dx + dy * dy)

      if (distance > 5) {
        const arrowAnn: ArrowAnnotation = {
          id: generateId(),
          type: 'arrow',
          start: startPoint.value,
          end: coords,
          showBadge: overlayStore.defaultShowBadge,
          color,
          strokeWidth: Math.max(3, strokeWidth + 1)
        }
        overlayStore.addAnnotation(arrowAnn)
      }
    } else if (overlayStore.activeTool === 'blur') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const width = Math.abs(coords.x - startPoint.value.x)
      const height = Math.abs(coords.y - startPoint.value.y)

      if (width > 5 && height > 5) {
        const blurAnn: BlurAnnotation = {
          id: generateId(),
          type: 'blur',
          topLeft: { x, y },
          width,
          height,
          blurRadius: 8,
          showBadge: false,
          color,
          strokeWidth
        }
        overlayStore.addAnnotation(blurAnn)
      }
    } else if (overlayStore.activeTool === 'highlight') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const width = Math.abs(coords.x - startPoint.value.x)
      const height = Math.abs(coords.y - startPoint.value.y)

      if (width > 5 && height > 5) {
        const highlightAnn: HighlightAnnotation = {
          id: generateId(),
          type: 'highlight',
          topLeft: { x, y },
          width,
          height,
          opacity: 0.35,
          showBadge: overlayStore.defaultShowBadge,
          color,
          strokeWidth
        }
        overlayStore.addAnnotation(highlightAnn)
      }
    } else if (overlayStore.activeTool === 'spotlight') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const width = Math.abs(coords.x - startPoint.value.x)
      const height = Math.abs(coords.y - startPoint.value.y)

      if (width > 5 && height > 5) {
        const spotlightAnn: SpotlightAnnotation = {
          id: generateId(),
          type: 'spotlight',
          topLeft: { x, y },
          width,
          height,
          showBadge: overlayStore.defaultShowBadge,
          color,
          strokeWidth
        }
        overlayStore.addAnnotation(spotlightAnn)
      }
    } else if (overlayStore.activeTool === 'crop') {
      const x = Math.min(startPoint.value.x, coords.x)
      const y = Math.min(startPoint.value.y, coords.y)
      const width = Math.abs(coords.x - startPoint.value.x)
      const height = Math.abs(coords.y - startPoint.value.y)

      if (width > 20 && height > 20) {
        overlayStore.setCropBox({ x, y, w: width, h: height })
        redrawCommittedCanvas()
        uiStore.showToast({
          message: `Đã cắt vùng ảnh: ${Math.round(width)} × ${Math.round(height)} px`,
          type: 'success'
        })
        overlayStore.setTool('marker')
      }
    } else if (overlayStore.activeTool === 'freedraw') {
      const ann = overlayStore.pendingAnnotation
      if (ann && ann.type === 'freedraw' && ann.points.length > 2) {
        overlayStore.addAnnotation(ann)
      }
      overlayStore.pendingAnnotation = null
    }

    startPoint.value = null
  }

  /**
   * Commits the floating text input to a text annotation.
   */
  const commitTextInput = () => {
    if (!textInputState.value.text.trim()) {
      cancelTextInput()
      return
    }

    const textAnn: TextAnnotation = {
      id: generateId(),
      type: 'text',
      position: { x: textInputState.value.x, y: textInputState.value.y },
      text: textInputState.value.text,
      fontSize: 16,
      showBadge: overlayStore.defaultShowBadge,
      color: overlayStore.currentColor || '#FF6B35',
      strokeWidth: 2
    }

    textInputState.value.visible = false
    overlayStore.addAnnotation(textAnn)
  }

  const cancelTextInput = () => {
    textInputState.value.visible = false
    textInputState.value.text = ''
    const ctx = getDrawingCtx()
    if (ctx && drawingCanvasRef.value) {
      clearCanvas(drawingCanvasRef.value, ctx)
    }
  }

  const distanceSq = (p1: Point, p2: Point) => {
    const dx = p1.x - p2.x
    const dy = p1.y - p2.y
    return dx * dx + dy * dy
  }

  const distanceToSegmentSq = (p: Point, a: Point, b: Point) => {
    const dx = b.x - a.x
    const dy = b.y - a.y
    const lenSq = dx * dx + dy * dy
    if (lenSq === 0) return distanceSq(p, a)
    let t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / lenSq
    t = Math.max(0, Math.min(1, t))
    return distanceSq(p, { x: a.x + t * dx, y: a.y + t * dy })
  }

  const getAnnotationBoundingBox = (ann: Annotation) => {
    if (ann.type === 'marker') {
      return {
        x: ann.position.x - 15,
        y: ann.position.y - 15,
        w: 30,
        h: 30
      }
    } else if (ann.type === 'rect' || ann.type === 'blur' || ann.type === 'highlight' || ann.type === 'spotlight') {
      return {
        x: ann.topLeft.x,
        y: ann.topLeft.y,
        w: ann.width,
        h: ann.height
      }
    } else if (ann.type === 'arrow') {
      return {
        x: Math.min(ann.start.x, ann.end.x) - 10,
        y: Math.min(ann.start.y, ann.end.y) - 10,
        w: Math.abs(ann.end.x - ann.start.x) + 20,
        h: Math.abs(ann.end.y - ann.start.y) + 20
      }
    } else if (ann.type === 'text') {
      const w = ann.text.length * 9 + 16
      return {
        x: ann.position.x,
        y: ann.position.y,
        w,
        h: 28
      }
    } else if (ann.type === 'freedraw') {
      if (ann.points.length === 0) return { x: 0, y: 0, w: 0, h: 0 }
      let minX = ann.points[0].x, maxX = ann.points[0].x
      let minY = ann.points[0].y, maxY = ann.points[0].y
      for (const pt of ann.points) {
        if (pt.x < minX) minX = pt.x
        if (pt.x > maxX) maxX = pt.x
        if (pt.y < minY) minY = pt.y
        if (pt.y > maxY) maxY = pt.y
      }
      return {
        x: minX - 10,
        y: minY - 10,
        w: (maxX - minX) + 20,
        h: (maxY - minY) + 20
      }
    }
    return { x: 0, y: 0, w: 0, h: 0 }
  }

  interface Handle {
    type: string
    x: number
    y: number
    cursor: string
  }

  const getHandlesForAnnotation = (ann: Annotation): Handle[] => {
    if (ann.type === 'rect' || ann.type === 'blur' || ann.type === 'highlight' || ann.type === 'spotlight') {
      const x = ann.topLeft.x
      const y = ann.topLeft.y
      const w = ann.width
      const h = ann.height
      return [
        { type: 'tl', x, y, cursor: 'nwse-resize' },
        { type: 't', x: x + w / 2, y, cursor: 'ns-resize' },
        { type: 'tr', x: x + w, y, cursor: 'nesw-resize' },
        { type: 'r', x: x + w, y: y + h / 2, cursor: 'ew-resize' },
        { type: 'br', x: x + w, y: y + h, cursor: 'nwse-resize' },
        { type: 'b', x: x + w / 2, y: y + h, cursor: 'ns-resize' },
        { type: 'bl', x, y: y + h, cursor: 'nesw-resize' },
        { type: 'l', x, y: y + h / 2, cursor: 'ew-resize' },
      ]
    } else if (ann.type === 'arrow') {
      return [
        { type: 'start', x: ann.start.x, y: ann.start.y, cursor: 'crosshair' },
        { type: 'end', x: ann.end.x, y: ann.end.y, cursor: 'crosshair' },
      ]
    } else if (ann.type === 'marker') {
      return []
    } else if (ann.type === 'text') {
      const box = getAnnotationBoundingBox(ann)
      return [
        { type: 'tl', x: box.x, y: box.y, cursor: 'move' },
        { type: 'tr', x: box.x + box.w, y: box.y, cursor: 'move' },
        { type: 'br', x: box.x + box.w, y: box.y + box.h, cursor: 'move' },
        { type: 'bl', x: box.x, y: box.y + box.h, cursor: 'move' },
      ]
    }
    return []
  }

  const findHandleAt = (coords: Point, ann: Annotation): Handle | null => {
    const handles = getHandlesForAnnotation(ann)
    const radiusSq = 12 * 12
    for (const h of handles) {
      const dx = coords.x - h.x
      const dy = coords.y - h.y
      if (dx * dx + dy * dy <= radiusSq) {
        return h
      }
    }
    return null
  }

  const drawSelectionPreview = () => {
    const ctx = getDrawingCtx()
    const canvas = drawingCanvasRef.value
    if (!ctx || !canvas) return

    clearCanvas(canvas, ctx)

    if (!overlayStore.selectedAnnotationId) return

    const ann = overlayStore.annotations.find(a => a.id === overlayStore.selectedAnnotationId)
    if (!ann) return

    const box = getAnnotationBoundingBox(ann)
    ctx.save()

    // Selection outline
    ctx.strokeStyle = '#38BDF8'
    ctx.lineWidth = 1.5
    ctx.setLineDash([4, 4])

    if (ann.type === 'marker') {
      ctx.beginPath()
      ctx.arc(ann.position.x, ann.position.y, 18, 0, Math.PI * 2)
      ctx.stroke()
    } else {
      ctx.strokeRect(box.x - 3, box.y - 3, box.w + 6, box.h + 6)
    }

    // Draw handles
    const handles = getHandlesForAnnotation(ann)
    ctx.setLineDash([])
    for (const h of handles) {
      ctx.fillStyle = '#FFFFFF'
      ctx.strokeStyle = '#0284C7'
      ctx.lineWidth = 2
      ctx.beginPath()
      ctx.arc(h.x, h.y, 5, 0, Math.PI * 2)
      ctx.fill()
      ctx.stroke()
    }

    ctx.restore()
  }

  const findAnnotationAt = (coords: Point): Annotation | null => {
    for (let i = overlayStore.annotations.length - 1; i >= 0; i--) {
      const ann = overlayStore.annotations[i]
      if (ann.type === 'marker') {
        if (distanceSq(coords, ann.position) <= 22 * 22) {
          return ann
        }
      } else if (ann.type === 'rect' || ann.type === 'blur' || ann.type === 'highlight' || ann.type === 'spotlight') {
        if (ann.showBadge && ann.number) {
          if (distanceSq(coords, { x: ann.topLeft.x, y: ann.topLeft.y - 20 }) <= 20 * 20) {
            return ann
          }
        }
        const xMin = Math.min(ann.topLeft.x, ann.topLeft.x + ann.width)
        const xMax = Math.max(ann.topLeft.x, ann.topLeft.x + ann.width)
        const yMin = Math.min(ann.topLeft.y, ann.topLeft.y + ann.height)
        const yMax = Math.max(ann.topLeft.y, ann.topLeft.y + ann.height)
        if (coords.x >= xMin && coords.x <= xMax && coords.y >= yMin && coords.y <= yMax) {
          return ann
        }
      } else if (ann.type === 'arrow') {
        if (ann.showBadge && ann.number) {
          if (distanceSq(coords, ann.start) <= 20 * 20) {
            return ann
          }
        }
        if (distanceToSegmentSq(coords, ann.start, ann.end) <= 12 * 12) {
          return ann
        }
      } else if (ann.type === 'text') {
        const box = getAnnotationBoundingBox(ann)
        if (coords.x >= box.x && coords.x <= box.x + box.w && coords.y >= box.y && coords.y <= box.y + box.h) {
          return ann
        }
      } else if (ann.type === 'freedraw') {
        for (let j = 0; j < ann.points.length - 1; j++) {
          if (distanceToSegmentSq(coords, ann.points[j], ann.points[j+1]) <= 12 * 12) {
            return ann
          }
        }
      }
    }
    return null
  }

  const drawHoverHighlight = () => {
    const ctx = getDrawingCtx()
    const canvas = drawingCanvasRef.value
    if (!ctx || !canvas || !hoveredAnnotationId.value) return

    clearCanvas(canvas, ctx)

    const ann = overlayStore.annotations.find(a => a.id === hoveredAnnotationId.value)
    if (!ann) return

    const box = getAnnotationBoundingBox(ann)
    ctx.save()
    ctx.strokeStyle = '#2196F3'
    ctx.lineWidth = 1.5
    ctx.setLineDash([4, 4])
    ctx.strokeRect(box.x, box.y, box.w, box.h)
    ctx.restore()
  }

  const handleContextMenu = (e: MouseEvent) => {
    if (!drawingCanvasRef.value) return

    const coords = getCanvasCoords(drawingCanvasRef.value, e)
    const ann = findAnnotationAt(coords)
    if (ann) {
      e.preventDefault()
      hoveredAnnotationId.value = ann.id
      drawHoverHighlight()

      contextMenuState.value = {
        visible: true,
        x: e.clientX,
        y: e.clientY,
        annotationId: ann.id
      }
    } else {
      contextMenuState.value.visible = false
    }
  }

  const toggleSelectedBadge = () => {
    if (contextMenuState.value.annotationId) {
      overlayStore.toggleAnnotationBadge(contextMenuState.value.annotationId)
      contextMenuState.value.visible = false
    }
  }

  const deleteAnnotationById = (id: string) => {
    overlayStore.removeAnnotation(id)
    if (hoveredAnnotationId.value === id) {
      hoveredAnnotationId.value = null
      if (drawingCanvasRef.value && getDrawingCtx()) {
        clearCanvas(drawingCanvasRef.value, getDrawingCtx()!)
      }
    }
    if (contextMenuState.value.annotationId === id) {
      contextMenuState.value.visible = false
    }
  }

  const clearHover = () => {
    hoveredAnnotationId.value = null
    if (drawingCanvasRef.value && getDrawingCtx()) {
      clearCanvas(drawingCanvasRef.value, getDrawingCtx()!)
    }
  }

  /**
   * Merges all canvas layers into a clean standalone canvas.
   */
  const getMergedCanvas = (): HTMLCanvasElement | null => {
    const screenshotCanvas = drawingCanvasRef.value?.parentElement?.querySelector('.z-screenshot') as HTMLCanvasElement | null
    const markerCanvas = markerCanvasRef.value
    if (!screenshotCanvas || !markerCanvas) return null

    const dpr = overlayStore.monitorInfo?.scaleFactor || window.devicePixelRatio || 1

    if (overlayStore.cropBox) {
      const crop = overlayStore.cropBox
      const cropW = Math.max(1, Math.round(crop.w * dpr))
      const cropH = Math.max(1, Math.round(crop.h * dpr))
      const cropX = Math.round(crop.x * dpr)
      const cropY = Math.round(crop.y * dpr)

      const merged = document.createElement('canvas')
      merged.width = cropW
      merged.height = cropH

      const mCtx = merged.getContext('2d')
      if (!mCtx) return null

      // Draw cropped region from screenshotCanvas
      mCtx.drawImage(screenshotCanvas, cropX, cropY, cropW, cropH, 0, 0, cropW, cropH)

      // Draw annotations without crop mask on temp canvas then blit cropped window
      const tempAnnCanvas = document.createElement('canvas')
      tempAnnCanvas.width = screenshotCanvas.width
      tempAnnCanvas.height = screenshotCanvas.height
      const tempCtx = tempAnnCanvas.getContext('2d')
      if (tempCtx) {
        for (const ann of overlayStore.annotations) {
          drawAnnotation(tempCtx, ann, false)
        }
        mCtx.drawImage(tempAnnCanvas, cropX, cropY, cropW, cropH, 0, 0, cropW, cropH)
      }

      return merged
    }

    const merged = document.createElement('canvas')
    merged.width = screenshotCanvas.width
    merged.height = screenshotCanvas.height

    const mCtx = merged.getContext('2d')
    if (!mCtx) return null

    mCtx.drawImage(screenshotCanvas, 0, 0)
    mCtx.drawImage(markerCanvas, 0, 0)
    return merged
  }

  /**
   * 1-Click Fast Copy to System Clipboard (Ctrl + C)
   */
  const copyToClipboard = async (): Promise<boolean> => {
    const merged = getMergedCanvas()
    if (!merged) return false

    try {
      const blob = await new Promise<Blob | null>(resolve => merged.toBlob(resolve, 'image/png'))
      if (!blob) return false

      await navigator.clipboard.write([
        new ClipboardItem({ 'image/png': blob })
      ])

      uiStore.showToast({ message: t('toasts.copiedToClipboard') || 'Đã sao chép ảnh vào Clipboard!', type: 'success' })
      return true
    } catch (e: any) {
      console.error('[Annotation] Failed to copy to clipboard:', e)
      uiStore.showToast({ message: 'Không thể sao chép vào clipboard: ' + (e?.message || e), type: 'error' })
      return false
    }
  }

  /**
   * ESC Key State Machine:
   * Level 1: In-progress drawing or dragging -> cancel, revert changes, keep overlay open.
   * Level 2: In-progress moving/resizing -> revert to initial state, keep overlay open.
   * Level 3: Text input open -> cancel text input, keep overlay open.
   * Level 4: Selected annotation -> deselect, keep overlay open.
   * Returns true if an action was cancelled, false if completely idle.
   */
  const cancelCurrentDrawing = (): boolean => {
    // Level 1: In-progress drawing
    if (isDrawing.value) {
      isDrawing.value = false
      startPoint.value = null
      overlayStore.pendingAnnotation = null
      const ctx = getDrawingCtx()
      if (ctx && drawingCanvasRef.value) {
        clearCanvas(drawingCanvasRef.value, ctx)
      }
      if (overlayStore.selectedAnnotationId) {
        drawSelectionPreview()
      }
      return true
    }

    // Level 2: In-progress moving or resizing
    if (interactionMode.value) {
      if (overlayStore.selectedAnnotationId && initialAnnotationState.value) {
        const idx = overlayStore.annotations.findIndex(a => a.id === overlayStore.selectedAnnotationId)
        if (idx !== -1) {
          overlayStore.annotations[idx] = JSON.parse(JSON.stringify(initialAnnotationState.value))
        }
      }
      interactionMode.value = null
      activeHandle.value = null
      initialAnnotationState.value = null
      startPoint.value = null
      redrawCommittedCanvas()
      drawSelectionPreview()
      return true
    }

    // Level 3: Text input active
    if (textInputState.value.visible) {
      cancelTextInput()
      return true
    }

    // Level 4: Shape selected -> deselect
    if (overlayStore.selectedAnnotationId) {
      overlayStore.selectAnnotation(null)
      const ctx = getDrawingCtx()
      if (ctx && drawingCanvasRef.value) {
        clearCanvas(drawingCanvasRef.value, ctx)
      }
      return true
    }

    // Completely idle
    return false
  }

  // Watch for activeTool changes to close context menu and clear hover
  watch(() => overlayStore.activeTool, () => {
    contextMenuState.value.visible = false
    clearHover()
  })

  // Watch for pendingAnnotation changes to trigger drawing layer updates
  watch(() => overlayStore.pendingAnnotation, () => {
    drawPendingPreview()
  })

  // Watch for cropBox changes to redraw committed layer with mask
  watch(() => overlayStore.cropBox, () => {
    redrawCommittedCanvas()
  }, { deep: true })

  // Watch for annotations list updates to redraw committed layer
  watch(() => overlayStore.annotations, () => {
    redrawCommittedCanvas()
    if (overlayStore.selectedAnnotationId) {
      drawSelectionPreview()
    }
  }, { deep: true })

  // Watch for selectedAnnotationId changes to update selection preview
  watch(() => overlayStore.selectedAnnotationId, (newId) => {
    if (newId) {
      drawSelectionPreview()
    } else if (!isDrawing.value && !interactionMode.value) {
      const ctx = getDrawingCtx()
      if (ctx && drawingCanvasRef.value) {
        clearCanvas(drawingCanvasRef.value, ctx)
      }
    }
  })

  return {
    isDrawing,
    customCursor,
    textInputState,
    hoveredAnnotationId,
    contextMenuState,
    handleMouseDown,
    handleMouseMove,
    handleMouseUp,
    handleContextMenu,
    deleteAnnotationById,
    toggleSelectedBadge,
    clearHover,
    commitTextInput,
    cancelTextInput,
    cancelCurrentDrawing,
    drawSelectionPreview,
    redrawCommittedCanvas,
    getMergedCanvas,
    copyToClipboard,
  }
}

