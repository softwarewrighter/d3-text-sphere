# d3-text-sphere

A Rust/WebAssembly web application that displays the text "[d3-text-sphere]" orbiting around a simulated 3D sphere, rendered using **d3.js** and **SVG**.

![Screenshot](images/screenshot.png?ts=1734041970368)

This is a companion project to [three-text-sphere](https://github.com/softwarewrighter/three-text-sphere), which uses Three.js and WebGL for true 3D rendering. This project demonstrates an alternative approach using d3.js with 2D SVG to simulate a 3D effect.

## Features

- Text characters orbit around a central sphere
- Simulated 3D depth using projection, scaling, and z-ordering
- Smooth 60 FPS animation
- Pure SVG rendering (no WebGL required)
- Built with Rust, WebAssembly, and Yew

## Technology Stack

- **Rust** - Core application logic
- **WebAssembly** - Browser execution
- **Yew** - Web framework
- **d3.js** - SVG manipulation and animation
- **Trunk** - Build tool

## Documentation

- [Product Requirements (PRD)](docs/prd.md) - Goals, requirements, and success criteria
- [Architecture](docs/architecture.md) - System design and technology stack
- [Design](docs/design.md) - Technical design details and 3D math
- [Implementation Plan](docs/plan.md) - Step-by-step development phases
- [Status](docs/status.md) - Current project status

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Trunk](https://trunkrs.dev/) - `cargo install trunk`
- [wasm32-unknown-unknown target](https://rustwasm.github.io/docs/book/) - `rustup target add wasm32-unknown-unknown`

### Build and Run

```bash
# Development build with hot reload
trunk serve

# Production build
trunk build --release

# Serve the built files
cd dist && python3 -m http.server 8080
```

## Project Structure

```
d3-text-sphere/
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Build configuration
├── index.html          # HTML shell with d3.js
├── README.md           # This file
├── src/
│   └── lib.rs          # Main WASM entry point
└── docs/
    ├── prd.md          # Product requirements
    ├── architecture.md # System architecture
    ├── design.md       # Technical design
    ├── plan.md         # Implementation plan
    └── status.md       # Project status
```

## How It Works

Unlike three-text-sphere which uses WebGL for true 3D rendering, this project simulates 3D using:

1. **3D to 2D Projection** - Characters are positioned in virtual 3D space and projected to 2D screen coordinates
2. **Depth Scaling** - Characters further away appear smaller
3. **Z-Ordering** - SVG elements are reordered so characters behind the sphere are properly occluded
4. **Opacity** - Optional depth-based transparency for enhanced depth perception

## Related Projects

- [three-text-sphere](../three-text-sphere) - The Three.js WebGL version of this project (local)
- [three-text-sphere on GitHub](https://github.com/softwarewrighter/three-text-sphere) - GitHub repository

## License

MIT License - See LICENSE file for details.
