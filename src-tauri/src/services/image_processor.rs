use crate::error::{AppError, AppResult};
use image::{DynamicImage, Rgba, RgbaImage};
use std::path::Path;

const DIGITS: [[u8; 7]; 10] = [
    [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // 0
    [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110], // 1
    [0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111], // 2
    [0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110], // 3
    [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010], // 4
    [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110], // 5
    [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110], // 6
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000], // 7
    [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110], // 8
    [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100], // 9
];

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum AnnotationItem {
    Marker {
        position: Point,
        number: Option<i32>,
        show_badge: Option<bool>,
        color: String,
    },
    Rect {
        top_left: Point,
        width: f64,
        height: f64,
        number: Option<i32>,
        show_badge: Option<bool>,
        color: String,
    },
    Arrow {
        start: Point,
        end: Point,
        number: Option<i32>,
        show_badge: Option<bool>,
        color: String,
    },
    Text {
        position: Point,
        text: String,
        number: Option<i32>,
        show_badge: Option<bool>,
        color: String,
    },
    Blur {
        top_left: Point,
        width: f64,
        height: f64,
        number: Option<i32>,
        show_badge: Option<bool>,
        color: String,
    },
    Highlight {
        top_left: Point,
        width: f64,
        height: f64,
        number: Option<i32>,
        show_badge: Option<bool>,
        color: String,
    },
    Freedraw {
        points: Vec<Point>,
        number: Option<i32>,
        show_badge: Option<bool>,
        color: String,
    },
    Spotlight {
        top_left: Point,
        width: f64,
        height: f64,
        number: Option<i32>,
        show_badge: Option<bool>,
        color: String,
    },
}

#[derive(Debug, serde::Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

/// Parses a hex color string (e.g., "#FF6B35") to Rgba<u8>.
fn parse_hex_color(hex: &str) -> Rgba<u8> {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(107);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(53);
        Rgba([r, g, b, 255])
    } else {
        Rgba([255, 107, 53, 255])
    }
}

/// Renders a single digit inside the image.
fn draw_digit(img: &mut RgbaImage, cx: i32, cy: i32, digit: usize, color: Rgba<u8>, scale: i32) {
    if digit > 9 {
        return;
    }
    let bitmap = DIGITS[digit];
    let start_x = cx - (5 * scale) / 2;
    let start_y = cy - (7 * scale) / 2;

    for row in 0..7 {
        let bits = bitmap[row];
        for col in 0..5 {
            if (bits & (1 << (4 - col))) != 0 {
                for dy in 0..scale {
                    for dx in 0..scale {
                        let px = start_x + (col as i32) * scale + dx;
                        let py = start_y + (row as i32) * scale + dy;
                        if px >= 0 && px < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                            img.put_pixel(px as u32, py as u32, color);
                        }
                    }
                }
            }
        }
    }
}

/// Renders a number centered.
fn draw_number(img: &mut RgbaImage, cx: i32, cy: i32, number: i32, color: Rgba<u8>, scale: i32) {
    let s = number.to_string();
    let chars: Vec<char> = s.chars().collect();
    let char_w = 5 * scale;
    let spacing = 1 * scale;
    let total_w = chars.len() as i32 * char_w + (chars.len() as i32 - 1) * spacing;
    let mut start_x = cx - total_w / 2 + char_w / 2;

    for c in chars {
        if let Some(digit) = c.to_digit(10) {
            draw_digit(img, start_x, cy, digit as usize, color, scale);
        }
        start_x += char_w + spacing;
    }
}

/// Draw a solid circle at (cx, cy) with specified radius.
fn draw_circle(img: &mut RgbaImage, cx: i32, cy: i32, radius: i32, color: Rgba<u8>) {
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                let px = cx + dx;
                let py = cy + dy;
                if px >= 0 && px < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                    img.put_pixel(px as u32, py as u32, color);
                }
            }
        }
    }
}

/// Draw a line using Bresenham's algorithm with thick brush stroke support.
fn draw_thick_line(
    img: &mut RgbaImage,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: Rgba<u8>,
    thickness: i32,
) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let r = (thickness / 2).max(1);

    loop {
        // Draw a dot of radius r at the line step
        draw_circle(img, x0, y0, r, color);

        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Draw an outlined rectangle.
fn draw_rectangle(
    img: &mut RgbaImage,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Rgba<u8>,
    thickness: i32,
) {
    // Top and bottom borders
    for px in x..=(x + width) {
        draw_circle(img, px, y, thickness / 2, color);
        draw_circle(img, px, y + height, thickness / 2, color);
    }
    // Left and right borders
    for py in y..=(y + height) {
        draw_circle(img, x, py, thickness / 2, color);
        draw_circle(img, x + width, py, thickness / 2, color);
    }
}

/// Draw a translucent yellow highlight box with alpha blending.
fn draw_highlight(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32) {
    let alpha = 0.35;
    let hr = 255;
    let hg = 235;
    let hb = 59;
    
    let img_w = img.width() as i32;
    let img_h = img.height() as i32;
    
    for py in y..(y + h) {
        for px in x..(x + w) {
            if px >= 0 && px < img_w && py >= 0 && py < img_h {
                let current_pixel = img.get_pixel(px as u32, py as u32);
                let r = (current_pixel[0] as f64 * (1.0 - alpha) + hr as f64 * alpha) as u8;
                let g = (current_pixel[1] as f64 * (1.0 - alpha) + hg as f64 * alpha) as u8;
                let b = (current_pixel[2] as f64 * (1.0 - alpha) + hb as f64 * alpha) as u8;
                img.put_pixel(px as u32, py as u32, Rgba([r, g, b, 255]));
            }
        }
    }
    
    // Draw highlight outline border with 0.6 opacity
    let border_alpha = 0.6;
    let draw_border_pixel = |img: &mut RgbaImage, px: i32, py: i32| {
        if px >= 0 && px < img_w && py >= 0 && py < img_h {
            let current_pixel = img.get_pixel(px as u32, py as u32);
            let r = (current_pixel[0] as f64 * (1.0 - border_alpha) + hr as f64 * border_alpha) as u8;
            let g = (current_pixel[1] as f64 * (1.0 - border_alpha) + hg as f64 * border_alpha) as u8;
            let b = (current_pixel[2] as f64 * (1.0 - border_alpha) + hb as f64 * border_alpha) as u8;
            img.put_pixel(px as u32, py as u32, Rgba([r, g, b, 255]));
        }
    };
    
    for px in x..=(x + w) {
        draw_border_pixel(img, px, y);
        draw_border_pixel(img, px, y + h);
    }
    for py in y..=(y + h) {
        draw_border_pixel(img, x, py);
        draw_border_pixel(img, x + w, py);
    }
}

/// Draw a mosaic/pixelated region over a bounding box.
fn draw_pixelate(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32) {
    let block_size = 8;
    let img_w = img.width() as i32;
    let img_h = img.height() as i32;
    
    for by in (y..(y + h)).step_by(block_size as usize) {
        for bx in (x..(x + w)).step_by(block_size as usize) {
            let mut sum_r: u64 = 0;
            let mut sum_g: u64 = 0;
            let mut sum_b: u64 = 0;
            let mut count: u64 = 0;
            
            for dy in 0..block_size {
                for dx in 0..block_size {
                    let px = bx + dx;
                    let py = by + dy;
                    if px >= x && px < x + w && py >= y && py < y + h {
                        if px >= 0 && px < img_w && py >= 0 && py < img_h {
                            let p = img.get_pixel(px as u32, py as u32);
                            sum_r += p[0] as u64;
                            sum_g += p[1] as u64;
                            sum_b += p[2] as u64;
                            count += 1;
                        }
                    }
                }
            }
            
            if count > 0 {
                let avg_r = (sum_r / count) as u8;
                let avg_g = (sum_g / count) as u8;
                let avg_b = (sum_b / count) as u8;
                let avg_color = Rgba([avg_r, avg_g, avg_b, 255]);
                
                for dy in 0..block_size {
                    for dx in 0..block_size {
                        let px = bx + dx;
                        let py = by + dy;
                        if px >= x && px < x + w && py >= y && py < y + h {
                            if px >= 0 && px < img_w && py >= 0 && py < img_h {
                                img.put_pixel(px as u32, py as u32, avg_color);
                            }
                        }
                    }
                }
            }
        }
    }
}


/// Helper to render the standard orange marker badge.
fn render_marker_badge(img: &mut RgbaImage, cx: i32, cy: i32, number: i32, color: Rgba<u8>) {
    // 1. Draw solid orange circle (radius 15)
    draw_circle(img, cx, cy, 15, color);
    
    // 2. Draw white text number centered (using white color)
    draw_number(img, cx, cy, number, Rgba([255, 255, 255, 255]), 2);
}

/// Crops a 400x400 area centered at (marker_x, marker_y), clamped to image bounds.
pub fn crop_for_issue(image: &DynamicImage, marker_x: f64, marker_y: f64) -> DynamicImage {
    let img_w = image.width() as i32;
    let img_h = image.height() as i32;
    let crop_size = 400;

    let mut x = (marker_x as i32) - crop_size / 2;
    let mut y = (marker_y as i32) - crop_size / 2;

    if x < 0 {
        x = 0;
    }
    if y < 0 {
        y = 0;
    }
    if x + crop_size > img_w {
        x = img_w - crop_size;
    }
    if y + crop_size > img_h {
        y = img_h - crop_size;
    }

    let final_x = x.max(0) as u32;
    let final_y = y.max(0) as u32;
    let final_w = crop_size.min(img_w) as u32;
    let final_h = crop_size.min(img_h) as u32;

    image.crop_imm(final_x, final_y, final_w, final_h)
}

/// Render all annotations onto the base screenshot and return as RgbaImage.
pub fn draw_annotations_to_rgba(
    base_image: &DynamicImage,
    annotations_json: &str,
) -> AppResult<RgbaImage> {
    let annotations: Vec<AnnotationItem> = serde_json::from_str(annotations_json)
        .map_err(|e| AppError::Generic(format!("Failed to parse annotations JSON: {}", e)))?;

    let mut img = base_image.to_rgba8();

    for item in annotations {
        match item {
            AnnotationItem::Marker { position, number, show_badge, color } => {
                let color_rgba = parse_hex_color(&color);
                if show_badge.unwrap_or(true) {
                    let num = number.unwrap_or(0);
                    if num > 0 {
                        render_marker_badge(&mut img, position.x as i32, position.y as i32, num, color_rgba);
                    }
                }
            }
            AnnotationItem::Rect { top_left, width, height, number, show_badge, color } => {
                let color_rgba = parse_hex_color(&color);
                let x = top_left.x as i32;
                let y = top_left.y as i32;
                let w = width as i32;
                let h = height as i32;

                // Draw rectangle outline (thickness = 2)
                draw_rectangle(&mut img, x, y, w, h, color_rgba, 2);

                // Draw marker badge at top left (offset y by -20) only if show_badge is true
                if show_badge.unwrap_or(true) {
                    if let Some(num) = number {
                        if num > 0 {
                            render_marker_badge(&mut img, x, y - 20, num, color_rgba);
                        }
                    }
                }
            }
            AnnotationItem::Arrow { start, end, number, show_badge, color } => {
                let color_rgba = parse_hex_color(&color);
                let x0 = start.x as i32;
                let y0 = start.y as i32;
                let x1 = end.x as i32;
                let y1 = end.y as i32;

                // Draw thick arrow shaft (thickness = 3)
                draw_thick_line(&mut img, x0, y0, x1, y1, color_rgba, 3);

                // Calculate arrowhead lines
                let dx = (x1 - x0) as f64;
                let dy = (y1 - y0) as f64;
                let angle = dy.atan2(dx);
                let head_len = 15.0;

                let ear1_x = (x1 as f64 - head_len * (angle - std::f64::consts::PI / 6.0).cos()) as i32;
                let ear1_y = (y1 as f64 - head_len * (angle - std::f64::consts::PI / 6.0).sin()) as i32;
                let ear2_x = (x1 as f64 - head_len * (angle + std::f64::consts::PI / 6.0).cos()) as i32;
                let ear2_y = (y1 as f64 - head_len * (angle + std::f64::consts::PI / 6.0).sin()) as i32;

                // Draw arrowhead ears (thickness = 3)
                draw_thick_line(&mut img, x1, y1, ear1_x, ear1_y, color_rgba, 3);
                draw_thick_line(&mut img, x1, y1, ear2_x, ear2_y, color_rgba, 3);

                // Draw marker badge at start point only if show_badge is true
                if show_badge.unwrap_or(true) {
                    if let Some(num) = number {
                        if num > 0 {
                            render_marker_badge(&mut img, x0, y0, num, color_rgba);
                        }
                    }
                }
            }
            AnnotationItem::Text { position, number, show_badge, color, .. } => {
                let color_rgba = parse_hex_color(&color);
                
                // Draw text marker badge next to position (offset matching frontend: x-20, y+12)
                if show_badge.unwrap_or(true) {
                    if let Some(num) = number {
                        if num > 0 {
                            let badge_x = (position.x - 20.0) as i32;
                            let badge_y = (position.y + 12.0) as i32;
                            render_marker_badge(&mut img, badge_x, badge_y, num, color_rgba);
                        }
                    }
                }
            }
            AnnotationItem::Blur { top_left, width, height, number, show_badge, color } => {
                let color_rgba = parse_hex_color(&color);
                let x = top_left.x as i32;
                let y = top_left.y as i32;
                let w = width as i32;
                let h = height as i32;

                draw_pixelate(&mut img, x, y, w, h);
                if show_badge.unwrap_or(true) {
                    if let Some(num) = number {
                        if num > 0 {
                            render_marker_badge(&mut img, x, y - 20, num, color_rgba);
                        }
                    }
                }
            }
            AnnotationItem::Highlight { top_left, width, height, number, show_badge, color } => {
                let color_rgba = parse_hex_color(&color);
                let x = top_left.x as i32;
                let y = top_left.y as i32;
                let w = width as i32;
                let h = height as i32;

                draw_highlight(&mut img, x, y, w, h);
                if show_badge.unwrap_or(true) {
                    if let Some(num) = number {
                        if num > 0 {
                            render_marker_badge(&mut img, x, y - 20, num, color_rgba);
                        }
                    }
                }
            }
            AnnotationItem::Freedraw { points, number, show_badge, color } => {
                let color_rgba = parse_hex_color(&color);
                
                if points.len() > 1 {
                    for i in 0..(points.len() - 1) {
                        let pt0 = &points[i];
                        let pt1 = &points[i+1];
                        draw_thick_line(&mut img, pt0.x as i32, pt0.y as i32, pt1.x as i32, pt1.y as i32, color_rgba, 3);
                    }
                } else if points.len() == 1 {
                    draw_circle(&mut img, points[0].x as i32, points[0].y as i32, 1, color_rgba);
                }

                if !points.is_empty() && show_badge.unwrap_or(true) {
                    if let Some(num) = number {
                        if num > 0 {
                            render_marker_badge(&mut img, points[0].x as i32, points[0].y as i32, num, color_rgba);
                        }
                    }
                }
            }
            AnnotationItem::Spotlight { top_left, width, height, number, show_badge, color } => {
                let color_rgba = parse_hex_color(&color);
                let x = top_left.x as i32;
                let y = top_left.y as i32;
                let w = width as i32;
                let h = height as i32;

                draw_rectangle(&mut img, x, y, w, h, color_rgba, 2);
                if show_badge.unwrap_or(true) {
                    if let Some(num) = number {
                        if num > 0 {
                            render_marker_badge(&mut img, x, y - 20, num, color_rgba);
                        }
                    }
                }
            }
        }
    }

    Ok(img)
}

/// Render all annotations onto the base screenshot and save as annotated file.
pub fn save_annotated_screenshot(
    base_image: &DynamicImage,
    annotations_json: &str,
    output_path: &Path,
) -> AppResult<()> {
    let img = draw_annotations_to_rgba(base_image, annotations_json)?;
    img.save(output_path)
        .map_err(|e| AppError::FileIO(format!("Failed to save annotated screenshot: {}", e)))?;

    Ok(())
}
