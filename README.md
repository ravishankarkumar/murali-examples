# Murali Examples

A collection of 74+ independent Rust examples demonstrating the [Murali](https://crates.io/crates/murali) animation library. Each example is a self-contained crate with its own dependencies.

## Structure

Examples are organized by category:

| Category | Examples | Description | Showcase Video |
|----------|----------|-------------|----------------|
| `ai_and_storytelling` | 11 | AI visualizations, neural networks, flow charts | ✓ |
| `animation` | 9 | Morphing, text effects, write/unwrite animations | Pending |
| `basics` | 8 | Shapes, primitives, arrows, styling | Pending |
| `branding_and_export` | 7 | Logos, templates, screenshot markers | Pending |
| `dynamics` | 7 | Particle systems, force fields, physics | Pending |
| `graphs_and_fields` | 8 | Graphs, vector fields, noise, streamlines | Pending |
| `text_and_math` | 16 | LaTeX, matrices, tables, formulas | Pending |
| `three_d` | 8 | Parametric surfaces, 3D axes, wireframes | Pending |

## Running Examples

Each example is an independent Cargo project:

```bash
cd animation/text_write_effect_showcase
cargo run
```

## Common Dependencies

All examples use:
- `murali = "0.1.4"` - Core animation library
- `glam = "0.27"` - Linear algebra/math
- `anyhow = "1"` - Error handling
- `tokio = "1"` - Async runtime

## Quick Start

```bash
# Clone and navigate to any example
cd basics/shapes
cargo run

# Or try an AI visualization
cd ai_and_storytelling2/neural_signal_flow
cargo run
```

## License

MIT - See LICENSE file