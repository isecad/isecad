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
