# Technical Design

## Overview

This document details the technical design for d3-text-sphere, focusing on how to simulate 3D orbiting text using d3.js and SVG.

## Visual Design

### Scene Layout

```
┌─────────────────────────────────────────┐
│              SVG Viewport               │
│                                         │
│     [d]                                 │
│           [3]                           │
│                ╭─────────╮              │
│     [-]        │ SPHERE  │      [s]     │
│                │(gradient)│             │
│                ╰─────────╯              │
│           [e]                           │
│     [r]                                 │
│                                [h]      │
│                         [p]             │
│                    [e]                  │
│              [-]                        │
│         [t]                             │
│    [x]                                  │
│ [e]                                     │
│                                         │
└─────────────────────────────────────────┘
```

### 3D Sphere Illusion

The central "sphere" is a 2D circle with a radial gradient to simulate 3D lighting:

```svg
<defs>
  <radialGradient id="sphere-gradient" cx="35%" cy="35%">
    <stop offset="0%" stop-color="#6699ff"/>
    <stop offset="70%" stop-color="#3366cc"/>
    <stop offset="100%" stop-color="#1a3366"/>
  </radialGradient>
</defs>
<circle cx="400" cy="300" r="80" fill="url(#sphere-gradient)"/>
```

The offset center (35%, 35%) creates a highlight in the upper-left, simulating a light source.

## 3D Math

### Coordinate System

```
        Y (up)
        │
        │
        │
        └───────── X (right)
       /
      /
     Z (toward viewer)
```

### Character Positioning

Characters are positioned on a circle in the XZ plane (horizontal orbit):

```rust
// Initial position for character i of n total
let angle = (i as f64 / n as f64) * 2.0 * PI;
let x = ORBIT_RADIUS * angle.cos();
let y = 0.0;  // Horizontal orbit
let z = ORBIT_RADIUS * angle.sin();
```

### Rotation Animation

Each frame, rotate all characters around the Y-axis:

```rust
// Update angle based on time
current_angle += ROTATION_SPEED * delta_time;

// For each character with initial angle:
let world_angle = initial_angle + current_angle;
let x = ORBIT_RADIUS * world_angle.cos();
let z = ORBIT_RADIUS * world_angle.sin();
```

### 3D to 2D Projection

**Orthographic Projection** (simpler, no perspective distortion):
```rust
let screen_x = center_x + x;
let screen_y = center_y - y;  // SVG Y is inverted
// z is used for depth effects only
```

**Perspective Projection** (more realistic):
```rust
let fov_factor = 400.0;  // Controls perspective strength
let scale = fov_factor / (fov_factor + z);
let screen_x = center_x + x * scale;
let screen_y = center_y - y * scale;
```

### Depth Effects

Characters further away (negative z) appear:

1. **Smaller** - Scale by depth factor
   ```rust
   let depth_scale = 0.5 + 0.5 * ((z + ORBIT_RADIUS) / (2.0 * ORBIT_RADIUS));
   let font_size = BASE_FONT_SIZE * depth_scale;
   ```

2. **Behind the sphere** - Z-ordering
   ```rust
   // Sort elements by z, render back-to-front
   elements.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
   ```

3. **Dimmer** (optional) - Opacity by depth
   ```rust
   let opacity = 0.3 + 0.7 * depth_scale;
   ```

## d3.js Binding Strategy

### JavaScript Interop

Use inline JavaScript via `#[wasm_bindgen]`:

```rust
#[wasm_bindgen(inline_js = r#"
    export function create_svg(container_id, width, height) {
        return d3.select('#' + container_id)
            .append('svg')
            .attr('width', width)
            .attr('height', height);
    }

    export function create_circle(svg, cx, cy, r, fill) {
        return svg.append('circle')
            .attr('cx', cx)
            .attr('cy', cy)
            .attr('r', r)
            .attr('fill', fill);
    }

    export function create_text(svg, x, y, text, fill, font_size) {
        return svg.append('text')
            .attr('x', x)
            .attr('y', y)
            .text(text)
            .attr('fill', fill)
            .attr('font-size', font_size + 'px')
            .attr('text-anchor', 'middle')
            .attr('dominant-baseline', 'central');
    }

    export function update_text_position(element, x, y, font_size, opacity) {
        element
            .attr('x', x)
            .attr('y', y)
            .attr('font-size', font_size + 'px')
            .attr('opacity', opacity);
    }

    export function reorder_elements(svg, elements_by_z) {
        // Re-append elements in z-order (back to front)
        elements_by_z.forEach(el => svg.node().appendChild(el.node()));
    }
"#)]
extern "C" {
    fn create_svg(container_id: &str, width: f64, height: f64) -> JsValue;
    fn create_circle(svg: &JsValue, cx: f64, cy: f64, r: f64, fill: &str) -> JsValue;
    fn create_text(svg: &JsValue, x: f64, y: f64, text: &str, fill: &str, font_size: f64) -> JsValue;
    fn update_text_position(element: &JsValue, x: f64, y: f64, font_size: f64, opacity: f64);
    fn reorder_elements(svg: &JsValue, elements: &JsValue);
}
```

### Element Storage

Store d3 selections in Rust for later updates:

```rust
struct Character {
    char: char,
    element: JsValue,       // d3 selection
    initial_angle: f64,     // Starting position on orbit
    color: String,          // HSV-derived color
}

struct TextSphere {
    svg: JsValue,
    sphere: JsValue,
    characters: Vec<Character>,
    current_angle: f64,
}
```

## Animation System

### Frame Loop

```rust
fn start_animation_loop(app: Rc<RefCell<App>>) {
    let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut last_time = 0.0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f64| {
        let delta = (time - last_time) / 1000.0;
        last_time = time;

        app.borrow_mut().animate(delta);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
```

### Per-Frame Update

```rust
fn animate(&mut self, delta: f64) {
    // Update global rotation
    self.current_angle += ROTATION_SPEED * delta;

    // Calculate new positions for all characters
    let mut char_positions: Vec<(usize, f64, f64, f64, f64)> = Vec::new();

    for (i, character) in self.characters.iter().enumerate() {
        let angle = character.initial_angle + self.current_angle;
        let x = ORBIT_RADIUS * angle.cos();
        let z = ORBIT_RADIUS * angle.sin();

        // Project to screen
        let (screen_x, screen_y, scale) = self.project(x, 0.0, z);
        let opacity = self.calculate_opacity(z);

        char_positions.push((i, screen_x, screen_y, scale, z));
    }

    // Sort by z (back to front)
    char_positions.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap());

    // Update DOM elements
    for (i, screen_x, screen_y, scale, _z) in char_positions {
        let font_size = LETTER_SIZE * scale;
        let opacity = 0.3 + 0.7 * scale;
        update_text_position(
            &self.characters[i].element,
            screen_x,
            screen_y,
            font_size,
            opacity
        );
    }

    // Reorder elements in SVG
    self.reorder_by_depth(&char_positions);
}
```

## Color System

### HSV to RGB Conversion

```rust
fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn assign_colors(chars: &[char]) -> Vec<String> {
    chars.iter().enumerate().map(|(i, _)| {
        let hue = (i as f64 / chars.len() as f64) * 360.0;
        let (r, g, b) = hsv_to_rgb(hue, 0.8, 0.9);
        format!("rgb({},{},{})", r, g, b)
    }).collect()
}
```

## Responsive Design

### Viewport Handling

```rust
fn on_resize(&mut self) {
    let window = web_sys::window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    // Update SVG size
    update_svg_size(&self.svg, width, height);

    // Update center point
    self.center_x = width / 2.0;
    self.center_y = height / 2.0;

    // Update sphere position
    update_circle_position(&self.sphere, self.center_x, self.center_y);
}
```

## Performance Considerations

### Optimization Strategies

1. **Minimize DOM operations** - Batch updates, avoid unnecessary reflows
2. **Use transform instead of position** - GPU-accelerated when possible
3. **Limit precision** - Round values to avoid sub-pixel rendering
4. **Avoid sorting every frame** - Only reorder when z-order changes significantly

### SVG Optimization

```rust
// Use transform for position updates (potentially GPU-accelerated)
fn update_text_transform(element: &JsValue, x: f64, y: f64, scale: f64) {
    element.attr("transform", format!(
        "translate({},{}) scale({})",
        x.round(), y.round(), scale
    ));
}
```

## Error Handling

```rust
fn check_d3_available() -> bool {
    let window = web_sys::window().unwrap();
    js_sys::Reflect::get(&window, &"d3".into())
        .map(|d3| !d3.is_undefined())
        .unwrap_or(false)
}

fn init() -> Result<TextSphere, String> {
    if !check_d3_available() {
        return Err("d3.js not loaded".to_string());
    }
    // ... initialization
}
```
