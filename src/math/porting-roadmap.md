# Layers

## F32 layer

### Field math

#### _laplacian_

Calculates laplacian of a surface.

Writes result values to other layer.

-   **Usage:** No usages currently.
-   **Frequency of the hottest usages:** No usages currently.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** IDK.
-   **Note:** May be entirely or partially parallelized.

#### _average difference_

Like _laplacian_, but doesn’t care about distance between vertices.

Writes result values to other layer.

-   **Usage:** One usage in crust.
-   **Frequency of the hottest usages:** At least once per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** IDK.
-   **Note:** May be entirely or partially parallelized.

#### _diffusion by constant_

Like _average difference_, but then multiplies each value by a constant and adds
corresponding source value.

Writes result values to other layer.

-   **Usage:** Few usages in fluid mechanics and atmosphere.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** IDK.
-   **Note:** May be entirely or partially parallelized.

#### _gradient_

Calculates gradient of an F32 layer.

Writes result values to other layer.

-   **Usage:** Few usages in fluid mechanics, climatology, and visualization.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** U8 layer.
-   **Possible implementations:** Any scalar layer.
-   **Note:** May be entirely or partially parallelized.

### Misc

#### _mix_

Applies a `mix` interpolation to each value of an F32 layer.

-   **Usage:** Few usages in crust and hydrosphere.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; may be implemented using generalized
    _interpolation_ function.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Add<f32>`,
    `Mul<f32>`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _mix fsf_

Like _f32 layer :: mix_, but accepts an F32 layer instead of number as the `x`
parameter.

-   **Usage:** One usage in climatology.
-   **Frequency of the hottest usages:** Once per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; may be implemented using generalized
    _interpolation_ function.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Add<f32>`,
    `Mul<f32>`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _clamp_

Clamps each value between min and max values.

-   **Usage:** One usage in optics.
-   **Frequency of the hottest usages:** Once per celestial cycle per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; may be implemented using generalized
    _interpolation_ function.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `PartialOrd`
    and `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _smoothstep_

Applies the `smoothstep` interpolation to each value of an F32 layer.

-   **Usage:** No usages at all; one potential usage in crust generator.
-   **Frequency of the hottest usages:** Once per simulation.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; may be implemented using generalized
    _interpolation_ function, but separate implementation may be slightly
    faster.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy`,
    `PartialOrd<f32>`, `Sub<f32>`, `Mul<f32>`, `Mul`.
-   **Note:** May be straightforwardly parallelized.

#### _linearstep_

Like the `smoothstep` interpolation, but returns a value right after clamping it
to a range.

-   **Usage:** Few usages in biosphere, crust, and hydrosphere simulations.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; may be implemented using generalized
    _interpolation_ function, but separate implementation may be slightly
    faster.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy`,
    `PartialOrd<f32>`, `Sub<f32>`, `Mul<f32>`.
-   **Note:** May be straightforwardly parallelized.

#### _smoothstep2_

IDK WTF is this, but original comments say this is like smoothstep, but slower,
so we probably don’t want to use it, but original uses it to preserve some
internal compatibility with certain legacy behavior or kinda.

-   **Usage:** One usage in crust generator.
-   **Frequency of the hottest usages:** Once per simulation.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; may be implemented using generalized
    _interpolation_ function.
-   **Other implementations:** _None._
-   **Possible implementations:** Ah, I think that’s possible to implement this
    method for any layer of type implementing certain traits, but I’m too lazy
    to write out all these fucking traits. The formula is
    $\frac{2}{(e^{-k x} + 1)} - 1$.
-   **Note:** May be straightforwardly parallelized.

#### _lerp_

Applies linear interpolation by control points to an F32 layer values.

-   **Usage:** Few usages in crust generator.
-   **Frequency of the hottest usages:** Few times per simulation.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; may be implemented using generalized
    _interpolation_ function.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer supporting _f32 layer :: linearstep_
    and _f32 layer :: mix_fsf_ functions.
-   **Note:** May be straightforwardly parallelized.

#### _fix conserved quantity delta_

IDK, looks like this is something about physics, so probably it would be moved
to physics module.

-   **Usage:** One usage in thermodynamics.
-   **Frequency of the hottest usages:** Once per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer supporting _f32 layer :: linearstep_
    and _f32 layer :: mix_fsf_ functions.
-   **Note:** Probably, May be straightforwardly parallelized.

## U8 layer

### Field math

#### _gradient_

Calculates gradient of a U8 layer.

Writes result values to other layer.

-   **Usage:** On usage in plate.
-   **Frequency of the hottest usages:** At least once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any scalar layer.
-   **Note:** May be entirely or partially parallelized.

### Misc

> ### Common to all these functions
>
> -   **TODO:** These functions should be defined at Bool layers.
> -   **Usage:** Multiple usages in tectonics and lithosphere.
> -   **Frequency of the hottest usages:** At least few times per iteration.
> -   **Conclusion:** Keep.
> -   **Possible implementations:** Any layer of bool-like type.
> -   **Note:** Some of them May be straightforwardly parallelized.

#### _union_

Applies binary union to two layers.

-   **Status:** Unimplemented.

#### _intersection_

Applies binary intersection to two layers.

-   **Status:** Unimplemented.

#### _difference_

Applies binary difference to two layers.

-   **Status:** Unimplemented.

#### _dilation_

Applies binary dilation to two layers.

-   **Status:** Unimplemented.

#### _erosion_

Applies binary erosion to two layers.

-   **Status:** Unimplemented.

#### _closing_

Applies binary closing to two layers.

-   **Status:** Unimplemented.

#### _margin_

Applies dilation, then subtracts original from result.

-   **Status:** Unimplemented.

#### _padding_

Applies erosion, then subtracts original from result.

-   **Status:** Unimplemented.

## V3 layer

### Raster graphics

#### _flood select_

Selects an area of a V3 layer using flood fill. Also, uses a neighbor lookup
table from given grid.

-   **Usage:** One indirect usage in tectonics.
-   **Frequency of the hottest usages:** Once per supercontinent cycle.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** We may introduce a trait, say, `Similarity`,
    and then implement this function for any layer implementing this trait.
-   **TODO:** Investigate hit rate of this function, or… Even with extremely low
    hit rate, this function will not significantly affect performance of a
    simulation.
-   **Note:** Probably, may be parallelized.

#### _image segmentation_

Splits image into segments using repeated flood fill.

-   **Usage:** One usage in tectonics.
-   **Frequency of the hottest usages:** Once per supercontinent cycle.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** For any layer with the _v3 layer :: flood
    select_ implemented.
-   **Note:** Probably, may be parallelized.

### Field math

#### _arrow differential_

IDK WTF is this, original says it probably should be moved right to the grid.

-   **Usage:** One usage in grid.
-   **Frequency of the hottest usages:** Once per simulation.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _N/A._
-   **Possible implementations:** IDK.
-   **Note:** May be straightforwardly parallelized.

### Misc

# _TBC_

[![To Be Continued](https://img.youtube.com/vi/TEYG1ZXU2Pc/0.jpg)](https://youtu.be/TEYG1ZXU2Pc)
