use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ============================================================================
// Configuration Constants
// ============================================================================

const TEXT_TO_DISPLAY: &str = "[d3-text-sphere2]";
const ORBIT_RADIUS: f64 = 200.0;
const ROTATION_SPEED: f64 = 0.3;
const LETTER_SIZE: f64 = 48.0;
const SPHERE_RADIUS: f64 = 80.0;
const PERSPECTIVE_DISTANCE: f64 = 400.0; // Increased for less extreme perspective
const SKEW_INTENSITY: f64 = 0.3; // Subtle skew to keep letters more upright

// ============================================================================
// d3.js JavaScript Bindings
// ============================================================================

#[wasm_bindgen(inline_js = r#"
    export function check_d3_available() {
        return typeof d3 !== 'undefined';
    }

    export function create_svg(container_id, width, height) {
        const svg = d3.select('#' + container_id)
            .append('svg')
            .attr('width', width)
            .attr('height', height)
            .attr('id', 'main-svg');

        // Add gradient for sphere
        const defs = svg.append('defs');
        const gradient = defs.append('radialGradient')
            .attr('id', 'sphere-gradient')
            .attr('cx', '35%')
            .attr('cy', '35%')
            .attr('r', '60%');

        gradient.append('stop')
            .attr('offset', '0%')
            .attr('stop-color', '#6699ff');
        gradient.append('stop')
            .attr('offset', '70%')
            .attr('stop-color', '#3366cc');
        gradient.append('stop')
            .attr('offset', '100%')
            .attr('stop-color', '#1a3366');

        return svg.node();
    }

    export function update_svg_size(width, height) {
        d3.select('#main-svg')
            .attr('width', width)
            .attr('height', height);
    }

    export function create_sphere(svg, cx, cy, radius) {
        return d3.select(svg)
            .append('circle')
            .attr('cx', cx)
            .attr('cy', cy)
            .attr('r', radius)
            .attr('fill', 'url(#sphere-gradient)')
            .node();
    }

    export function update_sphere_position(sphere, cx, cy) {
        d3.select(sphere)
            .attr('cx', cx)
            .attr('cy', cy);
    }

    export function create_text_element(svg, x, y, char, fill, font_size, skew_x) {
        return d3.select(svg)
            .append('text')
            .attr('x', x)
            .attr('y', y)
            .text(char)
            .attr('fill', fill)
            .attr('font-size', font_size + 'px')
            .attr('font-family', 'Arial, sans-serif')
            .attr('font-weight', 'bold')
            .attr('text-anchor', 'middle')
            .attr('dominant-baseline', 'central')
            .attr('opacity', 1)
            .attr('transform', `skewX(${skew_x})`)
            .node();
    }

    export function update_text_element(element, x, y, font_size, opacity, scale_x, skew_x) {
        // Transform around the text's position, not the SVG origin
        // Order: translate to origin → scale → skew → translate back
        d3.select(element)
            .attr('x', x)
            .attr('y', y)
            .attr('font-size', font_size + 'px')
            .attr('opacity', opacity)
            .attr('transform', `translate(${x}, ${y}) scale(${scale_x}, 1) skewX(${skew_x}) translate(${-x}, ${-y})`);
    }

    export function bring_to_front(element) {
        const node = d3.select(element).node();
        if (node && node.parentNode) {
            node.parentNode.appendChild(node);
        }
    }

    export function send_to_back(element) {
        const node = d3.select(element).node();
        if (node && node.parentNode) {
            node.parentNode.insertBefore(node, node.parentNode.firstChild);
        }
    }

    export function reorder_elements(elements) {
        // elements is an array sorted back-to-front (lowest z first)
        elements.forEach(el => {
            const node = d3.select(el).node();
            if (node && node.parentNode) {
                node.parentNode.appendChild(node);
            }
        });
    }

    export function create_debug_lines(svg, center_x, center_y, width, height) {
        const g = d3.select(svg).append('g').attr('id', 'debug-lines');

        // Vertical line through sphere center
        g.append('line')
            .attr('id', 'sphere-center-line')
            .attr('x1', center_x)
            .attr('y1', 0)
            .attr('x2', center_x)
            .attr('y2', height)
            .attr('stroke', '#ff0000')
            .attr('stroke-width', 2)
            .attr('stroke-dasharray', '5,5')
            .attr('opacity', 0.7);

        // Vertical line for orbit center (same for now)
        g.append('line')
            .attr('id', 'orbit-center-line')
            .attr('x1', center_x)
            .attr('y1', 0)
            .attr('x2', center_x)
            .attr('y2', height)
            .attr('stroke', '#00ff00')
            .attr('stroke-width', 2)
            .attr('stroke-dasharray', '10,10')
            .attr('opacity', 0.7);

        return g.node();
    }

    export function update_debug_info(svg, text) {
        let info = d3.select(svg).select('#debug-info');
        if (info.empty()) {
            info = d3.select(svg).append('text')
                .attr('id', 'debug-info')
                .attr('x', 10)
                .attr('y', 20)
                .attr('fill', '#ffffff')
                .attr('font-size', '12px')
                .attr('font-family', 'monospace');
        }
        info.text(text);
    }

    export function update_debug_lines(svg, center_x, center_y, width, height) {
        const g = d3.select(svg).select('#debug-lines');
        if (!g.empty()) {
            g.select('#sphere-center-line')
                .attr('x1', center_x)
                .attr('x2', center_x)
                .attr('y2', height);

            g.select('#orbit-center-line')
                .attr('x1', center_x)
                .attr('x2', center_x)
                .attr('y2', height);
        }
    }

    export function create_orbit_lines(svg, center_x, center_y, orbit_radius, sphere_offset) {
        const g = d3.select(svg).append('g').attr('id', 'orbit-lines');

        // 0 degrees (right) - Red: x=orbit_radius, z=0
        g.append('line')
            .attr('id', 'orbit-0')
            .attr('x1', center_x + orbit_radius + sphere_offset)
            .attr('y1', center_y - 100)
            .attr('x2', center_x + orbit_radius + sphere_offset)
            .attr('y2', center_y + 100)
            .attr('stroke', '#ff0000')
            .attr('stroke-width', 3)
            .attr('opacity', 0.7);

        // 90 degrees (front) - Green: x=0, z=orbit_radius (closest to viewer)
        g.append('line')
            .attr('id', 'orbit-90')
            .attr('x1', center_x + sphere_offset)
            .attr('y1', center_y - 100)
            .attr('x2', center_x + sphere_offset)
            .attr('y2', center_y + 100)
            .attr('stroke', '#00ff00')
            .attr('stroke-width', 3)
            .attr('opacity', 0.7);

        // 180 degrees (left) - Blue: x=-orbit_radius, z=0
        g.append('line')
            .attr('id', 'orbit-180')
            .attr('x1', center_x - orbit_radius + sphere_offset)
            .attr('y1', center_y - 100)
            .attr('x2', center_x - orbit_radius + sphere_offset)
            .attr('y2', center_y + 100)
            .attr('stroke', '#0000ff')
            .attr('stroke-width', 3)
            .attr('opacity', 0.7);

        // 270 degrees (back) - Yellow: x=0, z=-orbit_radius (furthest from viewer)
        g.append('line')
            .attr('id', 'orbit-270')
            .attr('x1', center_x + sphere_offset)
            .attr('y1', center_y - 100)
            .attr('x2', center_x + sphere_offset)
            .attr('y2', center_y + 100)
            .attr('stroke', '#ffff00')
            .attr('stroke-width', 3)
            .attr('opacity', 0.7);
    }

    export function update_orbit_lines(svg, center_x, center_y, orbit_radius, sphere_offset) {
        const g = d3.select(svg).select('#orbit-lines');
        if (!g.empty()) {
            g.select('#orbit-0')
                .attr('x1', center_x + orbit_radius + sphere_offset)
                .attr('x2', center_x + orbit_radius + sphere_offset);

            g.select('#orbit-90')
                .attr('x1', center_x + sphere_offset)
                .attr('x2', center_x + sphere_offset);

            g.select('#orbit-180')
                .attr('x1', center_x - orbit_radius + sphere_offset)
                .attr('x2', center_x - orbit_radius + sphere_offset);

            g.select('#orbit-270')
                .attr('x1', center_x + sphere_offset)
                .attr('x2', center_x + sphere_offset);
        }
    }
"#)]
extern "C" {
    fn check_d3_available() -> bool;
    fn create_svg(container_id: &str, width: f64, height: f64) -> JsValue;
    fn update_svg_size(width: f64, height: f64);
    fn create_sphere(svg: &JsValue, cx: f64, cy: f64, radius: f64) -> JsValue;
    fn update_sphere_position(sphere: &JsValue, cx: f64, cy: f64);
    fn update_debug_lines(svg: &JsValue, center_x: f64, center_y: f64, width: f64, height: f64);
    fn create_orbit_lines(
        svg: &JsValue,
        center_x: f64,
        center_y: f64,
        orbit_radius: f64,
        sphere_offset: f64,
    );
    fn update_orbit_lines(
        svg: &JsValue,
        center_x: f64,
        center_y: f64,
        orbit_radius: f64,
        sphere_offset: f64,
    );
    fn create_text_element(
        svg: &JsValue,
        x: f64,
        y: f64,
        char: &str,
        fill: &str,
        font_size: f64,
        skew_x: f64,
    ) -> JsValue;
    fn update_text_element(
        element: &JsValue,
        x: f64,
        y: f64,
        font_size: f64,
        opacity: f64,
        scale_x: f64,
        skew_x: f64,
    );
    fn reorder_elements(elements: &js_sys::Array);
    fn get_window_size() -> JsValue;
    fn create_debug_lines(
        svg: &JsValue,
        center_x: f64,
        center_y: f64,
        width: f64,
        height: f64,
    ) -> JsValue;
    fn update_debug_info(svg: &JsValue, text: &str);
}

// ============================================================================
// Color Utilities
// ============================================================================

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn get_color_for_index(index: usize, total: usize) -> String {
    let hue = (index as f64 / total as f64) * 360.0;
    let (r, g, b) = hsv_to_rgb(hue, 0.8, 0.95);
    format!("rgb({},{},{})", r, g, b)
}

// ============================================================================
// Type Aliases
// ============================================================================

/// Character render data: (index, screen_x, screen_y, font_size, opacity, z, scale_x, skew)
type CharRenderData = (usize, f64, f64, f64, f64, f64, f64, f64);

// ============================================================================
// Character Data Structure
// ============================================================================

struct Character {
    element: JsValue,
    base_angle: f64, // Position along orbit (0-2π)
}

// ============================================================================
// TextSphere - Main Application Logic
// ============================================================================

struct TextSphere {
    #[allow(dead_code)] // Kept for potential future use
    svg: JsValue,
    sphere: JsValue,
    characters: Vec<Character>,
    current_angle: f64,
    center_x: f64,
    center_y: f64,
}

impl TextSphere {
    fn new() -> Option<Self> {
        if !check_d3_available() {
            log::error!("d3.js is not available");
            return None;
        }

        let window = web_sys::window()?;
        let width = window.inner_width().ok()?.as_f64()?;
        let height = window.inner_height().ok()?.as_f64()?;

        let center_x = width / 2.0;
        let center_y = height / 2.0;

        // Create SVG (returns node reference used for creating child elements)
        let svg = create_svg("app", width, height);

        // Create central sphere (appended to svg)
        let sphere = create_sphere(&svg, center_x, center_y, SPHERE_RADIUS);

        // Create characters
        let chars: Vec<char> = TEXT_TO_DISPLAY.chars().filter(|c| *c != ' ').collect();
        let char_count = chars.len();
        let mut characters = Vec::with_capacity(char_count);

        for (i, ch) in chars.iter().enumerate() {
            // Position along the orbit (0 to 2π)
            let base_angle = (PI / 2.0) - (i as f64 / char_count as f64) * 2.0 * PI;
            let color = get_color_for_index(i, char_count);

            // Calculate initial position (at base_angle)
            let x = ORBIT_RADIUS * base_angle.cos();
            let z = ORBIT_RADIUS * base_angle.sin();

            // Project to 2D
            let scale = PERSPECTIVE_DISTANCE / (PERSPECTIVE_DISTANCE + z);
            let screen_x = center_x + x;
            let screen_y = center_y;
            let font_size = LETTER_SIZE * scale;

            let element = create_text_element(
                &svg,
                screen_x,
                screen_y,
                &ch.to_string(),
                &color,
                font_size,
                0.0,
            );

            characters.push(Character {
                element,
                base_angle,
            });
        }

        Some(TextSphere {
            svg,
            sphere,
            characters,
            current_angle: 0.0,
            center_x,
            center_y,
        })
    }

    fn animate(&mut self, delta: f64) {
        // Update rotation angle
        self.current_angle += ROTATION_SPEED * delta;

        // Keep angle in reasonable range
        if self.current_angle > 2.0 * PI {
            self.current_angle -= 2.0 * PI;
        }

        // Calculate positions using base interpolation
        let mut char_data: Vec<CharRenderData> = Vec::new();

        for (i, character) in self.characters.iter().enumerate() {
            let angle = character.base_angle + self.current_angle;

            // 3D position (orbiting in XZ plane)
            let x = ORBIT_RADIUS * angle.cos();
            let z = ORBIT_RADIUS * angle.sin();

            // Perspective projection
            // z > 0 = in front of center (closer to viewer), z < 0 = behind
            let scale = PERSPECTIVE_DISTANCE / (PERSPECTIVE_DISTANCE - z);

            // Project position - centered at screen
            let screen_x = self.center_x + x;
            let screen_y = self.center_y;
            let font_size = LETTER_SIZE * scale;

            // Characters face outward from sphere center (radially)
            // Width scale = cos(angle from front) = z / R
            // This naturally gives:
            //   - Front (z = R): scale = 1.0 (full width, facing camera)
            //   - Sides (z = 0): scale = 0.0 (edge-on)
            //   - Back (z = -R): scale = -1.0 (full width, flipped/mirrored)
            let scale_x = z / ORBIT_RADIUS;

            // Calculate skew for "facing outward" effect
            // Letters are painted on the sphere surface, facing radially outward
            // The skew simulates viewing the letter's tangent plane at an angle
            //
            // For a letter at position (x, z):
            // - At front (z = max, x = 0): faces camera directly → no skew
            // - Moving right (z > 0, x > 0): left edge closer to camera → negative skew
            // - Moving left (z > 0, x < 0): right edge closer to camera → positive skew
            // - At back (z < 0): flipped, skew direction reverses
            let current_skew = if z.abs() > 1.0 {
                // atan(x/z) gives the angle between the letter normal and camera direction
                // Convert to degrees and scale by intensity
                let skew_radians = (x / z).atan();
                let skew_degrees = skew_radians * (180.0 / PI);
                // Negative because SVG skewX shifts top-right for positive angles
                // and we want the near edge (toward camera) to appear larger
                -skew_degrees * SKEW_INTENSITY
            } else {
                // Near edge-on (z ≈ 0), letter is nearly invisible anyway
                0.0
            };

            char_data.push((
                i,
                screen_x,
                screen_y,
                font_size,
                1.0,
                z,
                scale_x,
                current_skew,
            ));
        }

        // Sort by z (back to front - lowest z first, will be rendered first/behind)
        char_data.sort_by(|a, b| a.5.partial_cmp(&b.5).unwrap());

        // Update all character positions
        for (i, screen_x, screen_y, font_size, _opacity, _z, scale_x, current_skew) in &char_data {
            update_text_element(
                &self.characters[*i].element,
                *screen_x,
                *screen_y,
                *font_size,
                1.0, // Always fully visible
                *scale_x,
                *current_skew,
            );
        }

        // Reorder elements in DOM for proper z-ordering (back to front)
        // char_data is sorted by z ascending (most negative/furthest first)
        // Only include visible characters (opacity > 0) to avoid z-ordering issues
        let elements = js_sys::Array::new();

        let mut sphere_added = false;
        for (i, _screen_x, _screen_y, _font_size, _opacity, z, _scale_x, _current_skew) in
            &char_data
        {
            // Add sphere when transitioning from behind to in-front (z > 0)
            if !sphere_added && *z > 0.0 {
                elements.push(&self.sphere);
                sphere_added = true;
            }
            elements.push(&self.characters[*i].element);
        }

        // If all characters are behind sphere, add sphere last (on top)
        if !sphere_added {
            elements.push(&self.sphere);
        }

        reorder_elements(&elements);
    }

    fn resize(&mut self) {
        if let Some(window) = web_sys::window() {
            if let (Ok(width), Ok(height)) = (window.inner_width(), window.inner_height()) {
                if let (Some(w), Some(h)) = (width.as_f64(), height.as_f64()) {
                    self.center_x = w / 2.0;
                    self.center_y = h / 2.0;

                    update_svg_size(w, h);
                    update_sphere_position(&self.sphere, self.center_x, self.center_y);
                }
            }
        }
    }
}

// ============================================================================
// Animation Loop
// ============================================================================

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .expect("no window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame`");
}

#[allow(clippy::type_complexity)]
fn start_animation_loop(text_sphere: Rc<RefCell<TextSphere>>) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let last_time: Rc<RefCell<f64>> = Rc::new(RefCell::new(0.0));

    let text_sphere_clone = text_sphere.clone();
    let last_time_clone = last_time.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        let mut last = last_time_clone.borrow_mut();
        let delta = if *last == 0.0 {
            0.016 // Assume ~60fps for first frame
        } else {
            (time - *last) / 1000.0
        };
        *last = time;

        text_sphere_clone.borrow_mut().animate(delta);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

// ============================================================================
// Resize Handler
// ============================================================================

fn setup_resize_handler(text_sphere: Rc<RefCell<TextSphere>>) {
    let closure = Closure::wrap(Box::new(move || {
        text_sphere.borrow_mut().resize();
    }) as Box<dyn Fn()>);

    web_sys::window()
        .expect("no window")
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .expect("should add resize listener");

    closure.forget();
}

// ============================================================================
// Entry Point
// ============================================================================

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("d3-text-sphere starting...");

    if let Some(text_sphere) = TextSphere::new() {
        let text_sphere = Rc::new(RefCell::new(text_sphere));

        setup_resize_handler(text_sphere.clone());
        start_animation_loop(text_sphere);

        log::info!("d3-text-sphere running");
    } else {
        log::error!("Failed to initialize TextSphere");
    }
}
