
export interface CanvasCoords {
  x: number
  y: number
}

export function useCanvas() {
  /**
   * Initializes a canvas with High-DPI (Retina) scaling support.
   * Sets canvas backing store dimensions scaled by devicePixelRatio
   * while keeping CSS dimensions fixed.
   */
  const initCanvas = (
    canvas: HTMLCanvasElement,
    width: number,
    height: number
  ): CanvasRenderingContext2D | null => {
    const dpr = window.devicePixelRatio || 1
    
    // Set backing store dimensions
    canvas.width = width * dpr
    canvas.height = height * dpr
    
    // Set display size (logical size)
    canvas.style.width = `${width}px`
    canvas.style.height = `${height}px`
    
    const ctx = canvas.getContext('2d')
    if (ctx) {
      // Reset transform first, then scale
      ctx.setTransform(1, 0, 0, 1, 0, 0)
      ctx.scale(dpr, dpr)
    }
    
    return ctx
  }

  /**
   * Translates client coordinates from a mouse event to logical coordinates relative to the canvas.
   */
  const getCanvasCoords = (canvas: HTMLCanvasElement, event: MouseEvent): CanvasCoords => {
    const rect = canvas.getBoundingClientRect()
    return {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top,
    }
  }

  /**
   * Clears the entire canvas viewport.
   */
  const clearCanvas = (canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D) => {
    const rect = canvas.getBoundingClientRect()
    ctx.clearRect(0, 0, rect.width, rect.height)
  }

  /**
   * Draws a static image onto the screenshot canvas.
   */
  const drawScreenshot = (
    ctx: CanvasRenderingContext2D,
    img: HTMLImageElement,
    width: number,
    height: number
  ) => {
    ctx.drawImage(img, 0, 0, width, height)
  }

  /**
   * Renders a numbered marker circle.
   */
  const renderMarker = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    num: number,
    isPending: boolean,
    color = '#FF6B35'
  ) => {
    ctx.save()
    
    // Draw outer ring if pending
    if (isPending) {
      ctx.strokeStyle = '#FFFFFF'
      ctx.lineWidth = 2
      ctx.setLineDash([4, 2])
      ctx.beginPath()
      ctx.arc(x, y, 19, 0, Math.PI * 2)
      ctx.stroke()
    }
    
    // Draw solid inner circle
    ctx.fillStyle = color
    ctx.beginPath()
    ctx.arc(x, y, 15, 0, Math.PI * 2)
    ctx.fill()
    
    // Draw white text number centered
    ctx.fillStyle = '#FFFFFF'
    ctx.font = 'bold 14px sans-serif'
    ctx.textAlign = 'center'
    ctx.textBaseline = 'middle'
    ctx.fillText(num.toString(), x, y)
    ctx.restore()
  }

  /**
   * Renders a rectangle.
   */
  const renderRect = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    width: number,
    height: number,
    isPending: boolean,
    color = '#FF6B35',
    strokeWidth = 2
  ) => {
    ctx.save()
    ctx.strokeStyle = color
    ctx.lineWidth = strokeWidth
    
    if (isPending) {
      ctx.setLineDash([6, 3])
    } else {
      ctx.setLineDash([])
    }
    
    ctx.strokeRect(x, y, width, height)
    
    // Translucent fill
    ctx.fillStyle = 'rgba(255, 107, 53, 0.04)'
    ctx.fillRect(x, y, width, height)
    ctx.restore()
  }

  /**
   * Renders an arrow.
   */
  const renderArrow = (
    ctx: CanvasRenderingContext2D,
    startX: number,
    startY: number,
    endX: number,
    endY: number,
    isPending: boolean,
    color = '#FF6B35',
    strokeWidth = 3
  ) => {
    ctx.save()
    ctx.strokeStyle = color
    ctx.lineWidth = strokeWidth
    
    if (isPending) {
      ctx.setLineDash([6, 3])
    } else {
      ctx.setLineDash([])
    }
    
    // Draw line
    ctx.beginPath()
    ctx.moveTo(startX, startY)
    ctx.lineTo(endX, endY)
    ctx.stroke()
    
    // Calculate angle for arrowhead
    const angle = Math.atan2(endY - startY, endX - startX)
    const headLength = 15
    
    // Draw arrowhead
    ctx.setLineDash([])
    ctx.fillStyle = color
    ctx.beginPath()
    ctx.moveTo(endX, endY)
    ctx.lineTo(
      endX - headLength * Math.cos(angle - Math.PI / 6),
      endY - headLength * Math.sin(angle - Math.PI / 6)
    )
    ctx.lineTo(
      endX - headLength * Math.cos(angle + Math.PI / 6),
      endY - headLength * Math.sin(angle + Math.PI / 6)
    )
    ctx.closePath()
    ctx.fill()
    ctx.restore()
  }

  /**
   * Renders a text note bubble.
   */
  const renderText = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    text: string,
    color = '#FFFFFF',
    bgColor = 'rgba(15, 17, 23, 0.85)'
  ) => {
    if (!text) return
    ctx.save()
    ctx.font = '500 16px sans-serif'
    ctx.textBaseline = 'top'
    
    const metrics = ctx.measureText(text)
    const textWidth = metrics.width
    const textHeight = 20
    const paddingX = 8
    const paddingY = 4
    
    // Draw background bubble
    ctx.fillStyle = bgColor
    ctx.beginPath()
    const rx = x
    const ry = y
    const rw = textWidth + paddingX * 2
    const rh = textHeight + paddingY * 2
    const radius = 4
    
    if (ctx.roundRect) {
      ctx.roundRect(rx, ry, rw, rh, radius)
    } else {
      ctx.rect(rx, ry, rw, rh)
    }
    ctx.fill()
    
    // Draw border
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.15)'
    ctx.lineWidth = 1
    ctx.stroke()
    
    // Draw text inside bubble
    ctx.fillStyle = color
    ctx.fillText(text, x + paddingX, y + paddingY)
    ctx.restore()
  }

  /**
   * Renders a freehand drawing stroke.
   */
  const renderFreeDraw = (
    ctx: CanvasRenderingContext2D,
    points: { x: number; y: number }[],
    color = '#FF6B35',
    strokeWidth = 3
  ) => {
    if (points.length < 1) return
    ctx.save()
    ctx.strokeStyle = color
    ctx.lineWidth = strokeWidth
    ctx.lineCap = 'round'
    ctx.lineJoin = 'round'
    ctx.setLineDash([])

    ctx.beginPath()
    ctx.moveTo(points[0].x, points[0].y)
    for (let i = 1; i < points.length; i++) {
      ctx.lineTo(points[i].x, points[i].y)
    }
    ctx.stroke()
    ctx.restore()
  }

  /**
   * Renders a yellow highlight block.
   */
  const renderHighlight = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    width: number,
    height: number,
    isPending: boolean,
    color = 'rgba(255, 235, 59, 0.35)'
  ) => {
    ctx.save()
    ctx.fillStyle = color
    ctx.fillRect(x, y, width, height)

    // Draw a border for visual feedback
    ctx.strokeStyle = 'rgba(255, 235, 59, 0.6)'
    ctx.lineWidth = 1.5
    if (isPending) {
      ctx.setLineDash([6, 3])
    } else {
      ctx.setLineDash([])
    }
    ctx.strokeRect(x, y, width, height)
    ctx.restore()
  }

  /**
   * Renders a pixelated/mosaic blur effect over a region.
   */
  const renderBlur = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    width: number,
    height: number,
    img: HTMLImageElement | HTMLCanvasElement | null
  ) => {
    if (!img || width <= 0 || height <= 0) return
    ctx.save()

    // Create a temporary offscreen canvas for fast pixelation
    const offscreen = document.createElement('canvas')
    const blockSize = 8
    offscreen.width = Math.max(1, Math.round(width / blockSize))
    offscreen.height = Math.max(1, Math.round(height / blockSize))

    const offCtx = offscreen.getContext('2d')
    if (offCtx) {
      offCtx.imageSmoothingEnabled = false
      offCtx.drawImage(
        img,
        x, y, width, height,
        0, 0, offscreen.width, offscreen.height
      )

      ctx.imageSmoothingEnabled = false
      ctx.drawImage(
        offscreen,
        0, 0, offscreen.width, offscreen.height,
        x, y, width, height
      )
      ctx.imageSmoothingEnabled = true
    }

    // Draw dashed border to define the blurred boundary
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.25)'
    ctx.lineWidth = 1
    ctx.setLineDash([4, 4])
    ctx.strokeRect(x, y, width, height)
    ctx.restore()
  }

  /**
   * Renders a spotlight cutout with dimmed surrounding background.
   */
  const renderSpotlight = (
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    width: number,
    height: number,
    canvasWidth: number,
    canvasHeight: number,
    isPending: boolean,
    borderColor = '#FF6B35',
    borderWidth = 2
  ) => {
    if (width <= 0 || height <= 0) return
    ctx.save()

    // Dim surrounding area using evenodd cutout rule
    ctx.fillStyle = 'rgba(0, 0, 0, 0.55)'
    ctx.beginPath()
    ctx.rect(0, 0, canvasWidth, canvasHeight)
    ctx.rect(x + width, y, -width, height)
    ctx.fill()

    // Draw border around spotlight area
    ctx.strokeStyle = borderColor
    ctx.lineWidth = borderWidth
    if (isPending) {
      ctx.setLineDash([6, 3])
    } else {
      ctx.setLineDash([])
    }
    ctx.strokeRect(x, y, width, height)
    ctx.restore()
  }

  return {
    initCanvas,
    getCanvasCoords,
    clearCanvas,
    drawScreenshot,
    renderMarker,
    renderRect,
    renderArrow,
    renderText,
    renderFreeDraw,
    renderHighlight,
    renderBlur,
    renderSpotlight,
  }
}
