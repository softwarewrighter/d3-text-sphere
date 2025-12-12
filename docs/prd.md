# Product Requirements Document (PRD)

## Project Overview

**d3-text-sphere** is a Rust/WebAssembly web application that renders text characters orbiting around a simulated 3D sphere. Unlike its sibling project [three-text-sphere](../../../three-text-sphere) which uses Three.js and WebGL for true 3D rendering, this project uses **d3.js with SVG** to simulate a 3D orbiting effect.

This is a browser-based implementation built using:
- **Rust** for core logic
- **WebAssembly (WASM)** for browser execution
- **d3.js** for SVG manipulation and animation
- **Yew** as the web framework

## Goals

### Primary Goals

1. **Simulate 3D orbiting text** - Display the text "[d3-text-sphere]" as individual characters orbiting around a central sphere, rendered entirely in SVG
2. **Demonstrate d3.js + WASM integration** - Show how to combine Rust/WASM with d3.js for 2D/simulated-3D visualizations
3. **Provide a companion to three-text-sphere** - Offer an alternative approach using SVG instead of WebGL

### Secondary Goals

1. **Educational value** - Demonstrate 3D projection techniques using 2D SVG
2. **Performance** - Achieve smooth 60 FPS animation using requestAnimationFrame
3. **Simplicity** - Single-file implementation with minimal dependencies

## Functional Requirements

### FR-1: Central Sphere Display
- Display a circular element representing the central sphere
- Apply gradient shading to simulate 3D appearance
- Sphere should be centered in the viewport

### FR-2: Orbiting Text Characters
- Parse the text string "[d3-text-sphere]" into individual characters
- Filter out spaces
- Position characters in a circular orbit around the central sphere
- Characters should appear to orbit in 3D space (depth simulation)

### FR-3: 3D Simulation via Projection
- Use orthographic or perspective projection to simulate depth
- Characters closer to the viewer appear larger and in front
- Characters further away appear smaller and behind the sphere
- Proper z-ordering so characters behind the sphere are occluded or dimmed

### FR-4: Continuous Animation
- Rotate the text orbit continuously around the Y-axis (vertical)
- Use frame-rate independent animation with delta time
- Target 60 FPS smooth animation

### FR-5: Color Assignment
- Assign distinct colors to each character using HSV color cycling
- Colors should be visually appealing with consistent saturation/value

### FR-6: Responsive Viewport
- Handle browser window resizing
- Maintain aspect ratio and centered positioning

## Non-Functional Requirements

### NFR-1: Performance
- Achieve 60 FPS animation on modern browsers
- WASM bundle size under 300KB
- Initial load time under 3 seconds

### NFR-2: Browser Compatibility
- Support Chrome 80+, Firefox 75+, Safari 14+, Edge 80+
- Graceful degradation if d3.js fails to load

### NFR-3: Code Quality
- Clean, well-documented Rust code
- Minimal JavaScript interop (inline bindings only)
- Follow Rust best practices and idioms

### NFR-4: Maintainability
- Single-file WASM implementation (lib.rs)
- Clear separation between d3.js bindings and application logic
- Configurable parameters as constants

## Configuration Parameters

| Parameter | Default Value | Description |
|-----------|---------------|-------------|
| `TEXT_TO_DISPLAY` | "[d3-text-sphere]" | Text string to orbit |
| `ORBIT_RADIUS` | 150.0 | Distance from center (SVG units) |
| `ROTATION_SPEED` | 0.4 | Angular velocity (rad/s) |
| `LETTER_SIZE` | 24.0 | Font size for characters |
| `SPHERE_RADIUS` | 80.0 | Central sphere radius (SVG units) |
| `SPHERE_COLOR` | "#3366CC" | Central sphere base color |

## Success Criteria

1. Text "[d3-text-sphere]" visibly orbits around a central sphere
2. 3D depth effect is convincing (size scaling, z-ordering, opacity)
3. Animation runs smoothly at 60 FPS
4. Application loads and runs without errors on target browsers
5. Code is clean, documented, and follows project conventions

## Out of Scope

- True 3D rendering (use three-text-sphere for that)
- User interaction (click, drag, zoom)
- Multiple orbit planes or complex orbital mechanics
- Server-side rendering or API integration
- Mobile-specific optimizations

## References

- [three-text-sphere](https://github.com/softwarewrighter/three-text-sphere) - Sibling project using Three.js
- [d3.js](https://d3js.org/) - Data visualization library
- [Yew](https://yew.rs/) - Rust web framework
