# bevy-earcutr

Runs earcutr on geometries and generates a Bevy `Mesh`.

## Examples

```rust
use bevy_earcutr::*;

let builder = PolygonMeshBuilder::new();

// Call `add_earcutr_input` or each polygon you want in the mesh.
builder.add_earcutr_input(EarcutrInput {
    vertices: vec![...],
    interior_indices: vec![...],
});

let mesh = builder.build().
```
