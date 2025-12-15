# Implementation Plan

## Overview

This document outlines the step-by-step implementation plan for d3-text-sphere, organized into phases.

## Phase 1: Project Setup

### Tasks

1. **Configure Cargo.toml**
   - Add Yew with CSR feature
   - Add wasm-bindgen, web-sys, js-sys
   - Add gloo for browser utilities
   - Add wasm-logger and log
   - Configure required web-sys features

2. **Create index.html**
   - HTML shell with container div
   - Load d3.js v7 from CDN
   - ES module script setup
   - WASM module loading

3. **Create Trunk.toml**
   - Configure build settings
   - Set output directory

4. **Basic lib.rs skeleton**
   - Main entry point
   - Basic Yew app structure
   - Logging initialization

### Deliverables
- Project builds with `trunk build`
- WASM loads in browser
- Console shows "d3-text-sphere initialized"

## Phase 2: d3.js Bindings

### Tasks

1. **Create d3 availability check**
   - Check if `window.d3` exists
   - Log warning if not available

2. **Implement SVG creation bindings**
   - `create_svg(container, width, height)`
   - `update_svg_size(svg, width, height)`

3. **Implement circle bindings**
   - `create_circle(svg, cx, cy, r, fill)`
   - `create_gradient(svg, id, stops)`
   - `update_circle_position(circle, cx, cy)`

4. **Implement text bindings**
   - `create_text(svg, x, y, char, fill, size)`
   - `update_text_position(text, x, y)`
   - `update_text_style(text, size, opacity)`

5. **Implement element ordering**
   - `reorder_element(element, index)` or
   - `bring_to_front(element)` / `send_to_back(element)`

### Deliverables
- All d3.js operations accessible from Rust
- Bindings tested with simple static scene

## Phase 3: Core Data Structures

### Tasks

1. **Define configuration constants**
   ```rust
   const TEXT_TO_DISPLAY: &str = "[d3-text-sphere]";
   const ORBIT_RADIUS: f64 = 150.0;
   const ROTATION_SPEED: f64 = 0.4;
   const LETTER_SIZE: f64 = 24.0;
   const SPHERE_RADIUS: f64 = 80.0;
   const SPHERE_COLOR: &str = "#3366CC";
   ```

2. **Create Character struct**
   - char value
   - d3 element reference
   - initial orbit angle
   - color string

3. **Create TextSphere struct**
   - SVG reference
   - Sphere element reference
   - Vector of Characters
   - Current rotation angle
   - Center point (x, y)
   - Viewport size

4. **Implement color utilities**
   - `hsv_to_rgb(h, s, v) -> (u8, u8, u8)`
   - `assign_character_colors(chars) -> Vec<String>`

### Deliverables
- All data structures defined
- Color assignment produces rainbow spectrum

## Phase 4: Static Scene Rendering

### Tasks

1. **Implement TextSphere::new()**
   - Get container element
   - Create SVG with d3
   - Set initial size to window dimensions

2. **Implement sphere creation**
   - Create radial gradient definition
   - Create circle with gradient fill
   - Center in viewport

3. **Implement character creation**
   - Parse TEXT_TO_DISPLAY
   - Filter spaces
   - Calculate initial angles (evenly distributed)
   - Create text elements for each character
   - Position in a static circle (no animation yet)

4. **Add resize handling**
   - Listen for window resize events
   - Update SVG dimensions
   - Update center point
   - Reposition sphere

### Deliverables
- Static scene renders: sphere with text around it
- Characters positioned in a circle
- Scene resizes with window

## Phase 5: 3D Projection

### Tasks

1. **Implement 3D to 2D projection**
   - Simple orthographic projection first
   - Optional: perspective projection

2. **Implement depth-based scaling**
   - Characters further away (z < 0) appear smaller
   - Calculate scale factor based on z position

3. **Implement depth-based opacity**
   - Characters further away are more transparent
   - Range from ~0.3 (far) to 1.0 (near)

4. **Implement z-ordering**
   - Sort characters by z position
   - Reorder SVG elements back-to-front
   - Sphere renders at z=0

### Deliverables
- Characters show size variation based on "depth"
- Characters behind sphere are properly occluded
- 3D illusion is convincing

## Phase 6: Animation System

### Tasks

1. **Implement animation loop**
   - requestAnimationFrame via web-sys
   - Delta time calculation
   - Frame rate tracking (optional)

2. **Implement rotation update**
   - Increment current_angle by ROTATION_SPEED * delta
   - Handle angle wraparound (optional)

3. **Implement per-frame character update**
   - Calculate new 3D positions based on current_angle
   - Project to 2D
   - Update element positions
   - Update element sizes and opacity
   - Reorder elements by depth

4. **Optimize animation**
   - Minimize DOM operations
   - Use transform attributes
   - Profile and optimize if needed

### Deliverables
- Smooth 60 FPS animation
- Characters orbit the sphere
- 3D effect maintained during animation

## Phase 7: Polish and Finalization

### Tasks

1. **Error handling**
   - Graceful failure if d3 not loaded
   - Console warnings for issues
   - Recovery from resize errors

2. **Code cleanup**
   - Remove debug logging
   - Add documentation comments
   - Organize code sections

3. **Performance verification**
   - Test on multiple browsers
   - Verify 60 FPS
   - Check memory usage

4. **Final testing**
   - Test resize behavior
   - Test in Chrome, Firefox, Safari
   - Verify visual quality

### Deliverables
- Production-ready application
- Clean, documented code
- All browsers supported

## Implementation Order

```
Phase 1 ─────▶ Phase 2 ─────▶ Phase 3
   │              │              │
   ▼              ▼              ▼
 Setup        Bindings        Structs
                                 │
                                 ▼
Phase 7 ◀───── Phase 6 ◀───── Phase 5 ◀───── Phase 4
   │              │              │              │
   ▼              ▼              ▼              ▼
Polish        Animation     Projection    Static Scene
```

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| d3.js loading fails | Check availability, show error message |
| Performance issues | Profile early, optimize SVG operations |
| Z-ordering complexity | Test with simple cases first |
| WASM size too large | Monitor bundle size, tree-shake dependencies |

## Testing Strategy

1. **Manual testing** - Visual inspection at each phase
2. **Console logging** - Debug output during development
3. **Browser testing** - Test on Chrome, Firefox, Safari
4. **Performance testing** - Monitor FPS with devtools

## Success Metrics

- [ ] Text orbits sphere smoothly
- [ ] 60 FPS maintained
- [ ] 3D effect is convincing
- [ ] Works on all target browsers
- [ ] WASM size under 300KB
- [ ] Code is clean and documented
