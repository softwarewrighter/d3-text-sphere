# Grid-Based 3D Projection Approach

## Overview

An alternative approach to simulating 3D text orbiting a sphere using a **property grid** system. Instead of calculating 3D projections mathematically in real-time, we pre-define a 2D grid where each cell contains transformation properties that get applied to letters as they pass through.

## Concept

The screen space around the sphere is divided into a grid. Each grid cell has predefined properties:

```
┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐
│     │     │     │     │     │     │     │
│ -Z  │ -Z  │ -Z  │ -Z  │ -Z  │ -Z  │ -Z  │  ← Behind sphere
│     │     │     │     │     │     │     │
├─────┼─────┼─────┼─────┼─────┼─────┼─────┤
│skew │skew │     │     │     │skew │skew │
│left │left │ +Z  │ +Z  │ +Z  │right│right│  ← In front of sphere
│small│     │large│large│large│     │small│
├─────┼─────┼─────┼─────┼─────┼─────┼─────┤
│     │     │     │     │     │     │     │
│ -Z  │ -Z  │ -Z  │ -Z  │ -Z  │ -Z  │ -Z  │  ← Behind sphere
│     │     │     │     │     │     │     │
└─────┴─────┴─────┴─────┴─────┴─────┴─────┘
  ←───── LEFT                RIGHT ─────→
```

## Grid Cell Properties

Each cell in the grid would define:

| Property | Description | Values |
|----------|-------------|--------|
| `z_index` | Render order (front/back of sphere) | positive = in front, negative = behind |
| `scale` | Size multiplier for perspective | 0.5 (far) to 1.5 (near) |
| `skew_x` | Horizontal skew angle in degrees | -45 to +45 |
| `width_scale` | Horizontal compression (foreshortening) | 0.0 (edge-on) to 1.0 (facing) |
| `opacity` | Fade for depth cue | 0.0 to 1.0 |
| `flip` | Mirror text horizontally | true/false |

## Example Grid Definition

```rust
struct GridCell {
    z_index: i32,
    scale: f64,
    skew_x: f64,
    width_scale: f64,
    opacity: f64,
    flip: bool,
}

// Grid organized by (column, row) from top-left
const GRID: [[GridCell; 7]; 3] = [
    // Row 0: Top (behind sphere)
    [
        GridCell { z_index: -1, scale: 0.6, skew_x: 30.0, width_scale: 0.3, opacity: 0.5, flip: true },
        GridCell { z_index: -1, scale: 0.7, skew_x: 15.0, width_scale: 0.6, opacity: 0.6, flip: true },
        GridCell { z_index: -1, scale: 0.8, skew_x: 0.0,  width_scale: 0.9, opacity: 0.7, flip: true },
        // ... center behind sphere
    ],
    // Row 1: Middle (passes in front)
    [
        GridCell { z_index: -1, scale: 0.7, skew_x: -35.0, width_scale: 0.2, opacity: 0.6, flip: false },
        GridCell { z_index: 1,  scale: 0.9, skew_x: -20.0, width_scale: 0.5, opacity: 0.9, flip: false },
        GridCell { z_index: 1,  scale: 1.2, skew_x: -10.0, width_scale: 0.8, opacity: 1.0, flip: false },
        GridCell { z_index: 1,  scale: 1.3, skew_x: 0.0,   width_scale: 1.0, opacity: 1.0, flip: false }, // Center front
        GridCell { z_index: 1,  scale: 1.2, skew_x: 10.0,  width_scale: 0.8, opacity: 1.0, flip: false },
        GridCell { z_index: 1,  scale: 0.9, skew_x: 20.0,  width_scale: 0.5, opacity: 0.9, flip: false },
        GridCell { z_index: -1, scale: 0.7, skew_x: 35.0,  width_scale: 0.2, opacity: 0.6, flip: false },
    ],
    // Row 2: Bottom (behind sphere)
    // ... similar to Row 0
];
```

## How It Works

1. **Letter Position**: As a letter orbits, calculate its screen (x, y) position
2. **Grid Lookup**: Determine which grid cell the letter is in
3. **Apply Properties**: Use the cell's properties for transforms
4. **Interpolation** (optional): Blend between adjacent cells for smooth transitions

```rust
fn get_cell_for_position(x: f64, y: f64) -> &GridCell {
    let col = ((x - grid_left) / cell_width) as usize;
    let row = ((y - grid_top) / cell_height) as usize;
    &GRID[row.min(ROWS-1)][col.min(COLS-1)]
}

fn apply_cell_properties(letter: &mut Letter, cell: &GridCell) {
    letter.transform = format!(
        "scale({}, 1) skewX({})",
        cell.width_scale * if cell.flip { -1.0 } else { 1.0 },
        cell.skew_x
    );
    letter.font_size = BASE_SIZE * cell.scale;
    letter.opacity = cell.opacity;
    letter.z_index = cell.z_index;
}
```

## Advantages

1. **Intuitive Tuning**: Adjust 3D appearance by editing grid values directly
2. **No Complex Math**: Avoids trigonometry, atan, perspective formulas
3. **Predictable Results**: Each screen region has consistent behavior
4. **Easy Debugging**: Can visualize the grid overlay to see property zones
5. **Artistic Control**: Can create non-realistic but visually pleasing effects
6. **Performance**: Simple lookups instead of per-frame trig calculations

## Disadvantages

1. **Discrete Steps**: May appear choppy without interpolation
2. **Resolution Trade-off**: More cells = smoother but more to configure
3. **Less Flexible**: Changing orbit path requires redesigning grid
4. **Manual Effort**: Properties must be hand-tuned for good results

## Interpolation for Smooth Transitions

To avoid discrete jumps between cells, use bilinear interpolation:

```rust
fn interpolate_properties(x: f64, y: f64) -> GridCell {
    // Get fractional position within cell
    let fx = ((x - grid_left) / cell_width).fract();
    let fy = ((y - grid_top) / cell_height).fract();

    // Get four surrounding cells
    let tl = get_cell(col, row);
    let tr = get_cell(col + 1, row);
    let bl = get_cell(col, row + 1);
    let br = get_cell(col + 1, row + 1);

    // Bilinear interpolation for each property
    GridCell {
        scale: lerp2d(tl.scale, tr.scale, bl.scale, br.scale, fx, fy),
        skew_x: lerp2d(tl.skew_x, tr.skew_x, bl.skew_x, br.skew_x, fx, fy),
        // ... etc
    }
}
```

## Hybrid Approach

Could combine both methods:
- Use mathematical projection for position and base scale
- Use grid for fine-tuning skew, opacity, and artistic adjustments
- Grid acts as a "correction layer" on top of math-based projection

## Visual Debug Mode

Add a debug overlay showing the grid and current letter positions:

```
┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐
│     │     │  U  │     │     │     │     │
│     │     │  ↓  │     │     │     │     │
├─────┼─────┼─────┼─────┼─────┼─────┼─────┤
│ L → │     │  N  │  I  │  V  │     │ ← A │
│     │  E  │     │     │     │  S  │     │
├─────┼─────┼─────┼─────┼─────┼─────┼─────┤
│     │     │  R  │     │     │     │     │
│     │     │  ↑  │     │     │     │     │
└─────┴─────┴─────┴─────┴─────┴─────┴─────┘
```

## Future Considerations

- **Bezier Paths**: Define orbit as a bezier curve with property keyframes
- **JSON Configuration**: Load grid from external file for easy editing
- **Visual Editor**: Tool to paint grid properties interactively
- **Multiple Grids**: Different grids for different orbit shapes/sizes
