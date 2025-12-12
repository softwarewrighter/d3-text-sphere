# Architecture

## Overview

d3-text-sphere is a Rust/WebAssembly application that uses d3.js to render SVG-based simulated 3D text orbiting a sphere. The architecture follows a similar pattern to three-text-sphere but adapts it for 2D SVG rendering with 3D projection simulation.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                       Browser                                │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────┐ │
│  │  index.html │───▶│   d3.js     │    │     WASM        │ │
│  │  (shell)    │    │  (ES Module)│◀──▶│   (lib.rs)      │ │
│  └─────────────┘    └─────────────┘    └─────────────────┘ │
│         │                  │                    │           │
│         ▼                  ▼                    ▼           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    SVG Container                        ││
│  │  ┌─────────────────────────────────────────────────┐   ││
│  │  │  Central Sphere (circle with gradient)          │   ││
│  │  │  ┌─────────────────────────────────────────┐    │   ││
│  │  │  │  Text Characters (text elements)        │    │   ││
│  │  │  │  - Positioned via 3D→2D projection      │    │   ││
│  │  │  │  - Scaled by depth                      │    │   ││
│  │  │  │  - Z-ordered for occlusion              │    │   ││
│  │  │  └─────────────────────────────────────────┘    │   ││
│  │  └─────────────────────────────────────────────────┘   ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

### Core Technologies

| Layer | Technology | Purpose |
|-------|------------|---------|
| Language | Rust | Core application logic |
| Runtime | WebAssembly | Browser execution |
| Framework | Yew 0.21 | Web application framework |
| Visualization | d3.js v7 | SVG creation and manipulation |
| Bindings | wasm-bindgen | Rust ↔ JavaScript FFI |

### Rust Dependencies

```toml
[dependencies]
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [...] }
js-sys = "0.3"
gloo = "0.11"
wasm-logger = "0.2"
log = "0.4"
```

### JavaScript Dependencies

- **d3.js v7** - Loaded via CDN as ES module
- No other JavaScript dependencies required

## Module Structure

### Single-File Architecture

```
src/
└── lib.rs              # Main WASM entry point
    ├── Constants       # Configuration parameters
    ├── D3 Bindings     # Inline JavaScript for d3.js interop
    ├── TextSphere      # Core 3D simulation logic
    │   ├── new()       # Initialize SVG scene
    │   ├── add_letters()   # Create text elements
    │   ├── animate()   # Update positions per frame
    │   └── resize()    # Handle viewport changes
    ├── App             # Yew component wrapper
    └── main()          # Entry point
```

## Data Flow

```
1. Browser loads index.html
         │
         ▼
2. d3.js loaded via ES module import
         │
         ▼
3. WASM module loaded and initialized
         │
         ▼
4. lib.rs::main() called
         │
         ▼
5. Check d3.js availability
         │
         ▼
6. Create SVG container with d3.js
         │
         ▼
7. Create central sphere (circle + gradient)
         │
         ▼
8. Create text elements for each character
         │
         ▼
9. Start animation loop (requestAnimationFrame)
         │
         ▼
10. Each frame:
    a. Calculate delta time
    b. Update rotation angle
    c. Project 3D positions to 2D
    d. Update SVG element positions/scales
    e. Reorder elements by z-depth
```

## 3D Simulation Approach

Unlike three-text-sphere which uses true 3D rendering via WebGL, this project simulates 3D using:

### Projection
- Characters exist in a virtual 3D space (x, y, z coordinates)
- 3D positions are projected to 2D screen coordinates
- Orthographic or simple perspective projection

### Depth Cues
1. **Size scaling** - Further objects appear smaller
2. **Z-ordering** - Proper layering via SVG element order
3. **Opacity** - Optional: dim objects further away
4. **Blur** - Optional: blur distant objects (performance cost)

### Animation
- Rotate characters around the Y-axis in 3D space
- Recalculate 2D projections each frame
- Update SVG transform attributes via d3.js

## Key Differences from three-text-sphere

| Aspect | three-text-sphere | d3-text-sphere |
|--------|-------------------|----------------|
| Rendering | WebGL (Three.js) | SVG (d3.js) |
| 3D | True 3D | Simulated via projection |
| Text | TextGeometry (3D mesh) | SVG text elements (2D) |
| Lighting | 3D lighting model | Gradient shading |
| Performance | GPU-accelerated | CPU + browser SVG |
| Complexity | Higher | Lower |
| Bundle Size | Larger (three.js) | Smaller |

## Build Pipeline

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Rust Code  │────▶│    Trunk    │────▶│    dist/    │
│  (src/)     │     │   (build)   │     │  (output)   │
└─────────────┘     └─────────────┘     └─────────────┘
                           │
                           ▼
              ┌────────────────────────┐
              │  - index.html          │
              │  - d3_text_sphere.js   │
              │  - d3_text_sphere.wasm │
              └────────────────────────┘
```

## Deployment

- Static file deployment (no server-side logic)
- Serve via any static file server
- CDN-compatible
- No build-time configuration required
