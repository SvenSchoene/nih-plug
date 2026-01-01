//! A resize handle for uniformly scaling a plugin GUI.

use vizia::prelude::*;
use vizia::vg;

/// A resize handle placed at the bottom right of the window that lets you resize the window.
///
/// Needs to be the last element in the GUI because of how event targetting in Vizia works right
/// now.
///
/// NOTE: In vizia 0.3.0, user scale factor APIs have changed. This widget may need to be updated
/// to use a different approach for resizing.
pub struct ResizeHandle {
    /// Will be set to `true` if we're dragging the parameter. Resetting the parameter or entering a
    /// text value should not initiate a drag.
    drag_active: bool,

    /// The scale factor when we started dragging. This is kept track of separately to avoid
    /// accumulating rounding errors.
    start_scale_factor: f64,
    /// The DPI factor when we started dragging, includes both the HiDPI scaling and the user
    /// scaling factor. This is kept track of separately to avoid accumulating rounding errors.
    start_dpi_factor: f32,
    /// The cursor position in physical screen pixels when the drag started.
    start_physical_coordinates: (f32, f32),
}

impl ResizeHandle {
    /// Create a resize handle at the bottom right of the window. This should be created at the top
    /// level. Dragging this handle around will cause the window to be resized.
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        // Styling is done in the style sheet
        ResizeHandle {
            drag_active: false,
            start_scale_factor: 1.0,
            start_dpi_factor: 1.0,
            start_physical_coordinates: (0.0, 0.0),
        }
        .build(cx, |_| {})
    }
}

impl View for ResizeHandle {
    fn element(&self) -> Option<&'static str> {
        Some("resize-handle")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match *window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                // The handle is a triangle, so we should also interact with it as if it was a
                // triangle
                if intersects_triangle(
                    cx.cache.get_bounds(cx.current()),
                    (cx.mouse().cursor_x, cx.mouse().cursor_y),
                ) {
                    cx.capture();
                    cx.set_active(true);

                    self.drag_active = true;
                    // In vizia 0.3.0, user_scale_factor is not available on EventContext.
                    // Using scale_factor() as a substitute for now.
                    self.start_scale_factor = cx.scale_factor() as f64;
                    self.start_dpi_factor = cx.scale_factor();
                    self.start_physical_coordinates = (
                        cx.mouse().cursor_x * self.start_dpi_factor,
                        cx.mouse().cursor_y * self.start_dpi_factor,
                    );

                    meta.consume();
                } else {
                    // TODO: The click should be forwarded to the element behind the triangle
                }
            }
            WindowEvent::MouseUp(MouseButton::Left) => {
                if self.drag_active {
                    cx.release();
                    cx.set_active(false);

                    self.drag_active = false;
                }
            }
            WindowEvent::MouseMove(x, y) => {
                cx.set_hover(intersects_triangle(
                    cx.cache.get_bounds(cx.current()),
                    (x, y),
                ));

                if self.drag_active {
                    // We need to convert our measurements into physical pixels relative to the
                    // initial drag to be able to keep a consistent ratio. This 'relative to the
                    // start' bit is important because otherwise we would be comparing the position
                    // to the same absoltue screen spotion.
                    // TODO: This may start doing fun things when the window grows so large that it
                    //       gets pushed upwards or leftwards
                    let (compensated_physical_x, compensated_physical_y) =
                        (x * self.start_dpi_factor, y * self.start_dpi_factor);
                    let (start_physical_x, start_physical_y) = self.start_physical_coordinates;
                    let _new_scale_factor = (self.start_scale_factor
                        * (compensated_physical_x / start_physical_x)
                            .max(compensated_physical_y / start_physical_y)
                            as f64)
                        // Vizia rounds borders to integer pixels, and at <0.5 scaling one pixel
                        // borders will simply disappear
                        .max(0.5);

                    // TODO: In vizia 0.3.0, set_user_scale_factor is not available.
                    // This needs to be reimplemented using a different approach,
                    // possibly by emitting an event to resize the window.
                    // cx.set_user_scale_factor(new_scale_factor);
                }
            }
            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        // We'll draw the handle directly as styling elements for this is going to be a bit tricky

        // These basics are taken directly from the default implementation of this function
        let bounds = cx.bounds();
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let background_color = cx.background_color();
        let border_color = cx.border_color();
        let opacity = cx.opacity();
        let border_width = cx.border_width();

        let mut path = vg::Path::new();
        let x = bounds.x + border_width / 2.0;
        let y = bounds.y + border_width / 2.0;
        let w = bounds.w - border_width;
        let h = bounds.h - border_width;
        path.move_to((x, y));
        path.line_to((x, y + h));
        path.line_to((x + w, y + h));
        path.line_to((x + w, y));
        path.line_to((x, y));
        path.close();

        // Fill with background color
        let mut bg_paint = vg::Paint::default();
        let bg_rgba = background_color.get_rgba();
        let bg_alpha = (bg_rgba.alpha as f32 * opacity) as u8;
        bg_paint.set_color(vg::Color::from_argb(bg_alpha, bg_rgba.red, bg_rgba.green, bg_rgba.blue));
        bg_paint.set_style(vg::PaintStyle::Fill);
        bg_paint.set_anti_alias(true);
        canvas.draw_path(&path, &bg_paint);

        // Borders are only supported to make debugging easier
        let mut border_paint = vg::Paint::default();
        let border_rgba = border_color.get_rgba();
        let border_alpha = (border_rgba.alpha as f32 * opacity) as u8;
        border_paint.set_color(vg::Color::from_argb(border_alpha, border_rgba.red, border_rgba.green, border_rgba.blue));
        border_paint.set_stroke_width(border_width);
        border_paint.set_style(vg::PaintStyle::Stroke);
        border_paint.set_anti_alias(true);
        canvas.draw_path(&path, &border_paint);

        // We'll draw a simple triangle, since we're going flat everywhere anyways and that style
        // tends to not look too tacky
        let mut triangle_path = vg::Path::new();
        let x = bounds.x + border_width / 2.0;
        let y = bounds.y + border_width / 2.0;
        let w = bounds.w - border_width;
        let h = bounds.h - border_width;
        triangle_path.move_to((x, y + h));
        triangle_path.line_to((x + w, y + h));
        triangle_path.line_to((x + w, y));
        triangle_path.close();

        let font_color = cx.font_color();
        let mut triangle_paint = vg::Paint::default();
        let font_rgba = font_color.get_rgba();
        let font_alpha = (font_rgba.alpha as f32 * opacity) as u8;
        triangle_paint.set_color(vg::Color::from_argb(font_alpha, font_rgba.red, font_rgba.green, font_rgba.blue));
        triangle_paint.set_style(vg::PaintStyle::Fill);
        triangle_paint.set_anti_alias(true);
        canvas.draw_path(&triangle_path, &triangle_paint);
    }
}

/// Test whether a point intersects with the triangle of this resize handle.
fn intersects_triangle(bounds: BoundingBox, (x, y): (f32, f32)) -> bool {
    // We could also compute Barycentric coordinates, but this is simple and I like not having to
    // think. Just check if (going clockwise), the point is on the right of each of all of the
    // triangle's edges. We can compute this using the determinant of the 2x2 matrix formed by two
    // column vectors, aka the perp dot product, aka the wedge product.
    // NOTE: Since this element is positioned in the bottom right corner we would technically only
    //       have to calculate this for `v1`
    let (p1x, p1y) = bounds.bottom_left();
    let (p2x, p2y) = bounds.top_right();
    // let (p3x, p3y) = bounds.bottom_right();

    let (v1x, v1y) = (p2x - p1x, p2y - p1y);
    // let (v2x, v2y) = (p3x - p2x, p3y - p2y);
    // let (v3x, v3y) = (p1x - p3x, p1y - p3y);

    ((x - p1x) * v1y) - ((y - p1y) * v1x) >= 0.0
    // && ((x - p2x) * v2y) - ((y - p2y) * v2x) >= 0.0
    // && ((x - p3x) * v3y) - ((y - p3y) * v3x) >= 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_intersection() {
        let bbox = BoundingBox {
            x: 10.0,
            y: 10.0,
            w: 10.0,
            h: 10.0,
        };

        assert!(!intersects_triangle(bbox, (10.0, 10.0)));
        assert!(intersects_triangle(bbox, (20.0, 10.0)));
        assert!(intersects_triangle(bbox, (10.0, 20.0)));
        assert!(intersects_triangle(bbox, (20.0, 20.0)));

        assert!(intersects_triangle(bbox, (15.0, 15.0)));
        assert!(!intersects_triangle(bbox, (14.9, 15.0)));
        assert!(!intersects_triangle(bbox, (15.0, 14.9)));
    }
}
