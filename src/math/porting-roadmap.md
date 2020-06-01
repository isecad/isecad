# Layers

## F32 layer

### Core functionality

#### ~~_from example `&Layer → F32 layer`_~~

Creates new F32 layer of the same length as given layer, _and with the same
associated grid_.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Multiple times per iteration.
-   **Conclusion:** Probably useless. We will not use associated grids, so we
    don’t need grid-related functionality in layers; most probably it will be
    much simpler to create new layers explicitly.
-   **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Default`.

#### ~~_of length `usize → F32 layer`_~~

Creates new F32 layer of given length, _and with a given associated grid with_,
where given length may be unequal to length of a grid.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Twice per plate per iteration.
-   **Conclusion:** Probably useless. See previous conclusion.
-   **Status:** Implemented as `of_length :: usize → Box<Self>`.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Default`.

#### _from u8 layer `&U8 layer → &mut F32 layer → ()`_

Creates new F32 layer of the same length as given U8 layer, with the same data
as in given layer, _and with the same associated grid_.

-   **Usage**: Few usages in the visualization code, required to convert certain
    internal layers to a common visualizable data layers.
-   **Frequency of the hottest usages:** Once per frame.
-   **Conclusion:** The functionality is required, the function probably may be
    eliminated. A _u8 layer :: to f32 layer_ probably will be better.
-   **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.
-   **Other implementations:** _None._
-   **Possible implementations:** Any scalar layer.
-   **TODO:** Investigate whether U8 layers required to convert to F32 layers
    may be safely replaced with F32 layers internally.

#### _from buffer `&ArrayBuffer → usize → F32 layer`_

Creates new F32 layer from raw array buffer at the given offset, and of the same
length as given grid.

-   **Usage:** Used to reduce allocations in some places.
-   **Frequency of the hottest usages:** Few times per supercontinent cycle.
-   **Conclusion:** Reject. Allocation in Rust have much lower cost than in
    JavaScript. Also, code using this function isn’t hot and may be safely
    suboptimal even in JavaScript.
-   **Status:** Rejected.
-   **Other implementations:** U8 layer.
-   **Possible implementations:** _N/A._

#### _copy `&F32 layer → &mut F32 layer → ()`_

Copies data from one layer to another.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Once pre iteration.
-   **Conclusion:** Reject.
-   **Status:** Implemented in the Rust library.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy`.
-   **TODO:** Make sure whether the existing implementation uses WASM
    `memory.copy`; probably we will need to reimplement it on our side.

#### _fill `&mut F32 layer → f32 → ()`_

Fills a layer with given value.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Really fucking hot.
-   **Conclusion:** Reject.
-   **Status:** Implemented in the Rust library.
-   **Other implementations:** U8 layer.
-   **Possible implementations:** Any layer of type implementing `Copy`; most
    optimized version may be implemented for layers of single-byte types, or
    when filling a layer with all zeroes.
-   **TODO:** Investigate whether the existing implementation makes use of WASM
    `memory.fill`; probably we will need to reimplement it on our side.
-   **Note:** May be straightforwardly parallelized.

#### ~~_max index `&F32 layer → usize`_~~

Returns an index of an element with max value.

-   **Usage:** Single usage in an image segmentation algorithm.
-   **Frequency of the hottest usages:** Multiple times per supercontinent
    cycle.
-   **Conclusion:** Keep.
-   **Status:** Implemented as `min_max_indices :: &self → (usize, usize)`.
-   **Other implementations:** _None._
-   **Possible implementations:** ~~Any layer of type implementing `Ord` or
    `PartialOrd`.~~

#### ~~_swizzle `&F32 layer → &Usize layer → &mut F32 layer → ()`_~~

Copies data from one layer to another using indices mapping from a supplementary
Usize layer.

-   **Usage:** Lots of them across almost every simulation module.
-   **Frequency of the hottest usages:** Really fucking hot.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** U8 layer.
-   **Possible implementations:** Any layer of type implementing `Copy`.
-   **Possible useful versions:** A version that consumes source layer without
    copying its data.
-   **TODO:** Investigate average hit rate of this function.
-   **Note:** Probably, May be straightforwardly parallelized.

#### ~~_inverse swizzle add_~~

Add values from one layer to existing values in other layer using indices
mapping from a supplementary Usize layer.

-   **Usage:** One indirect usage in the lithosphere model.
-   **Frequency of the hottest usages:** Multiple times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy` and
    `AddAssign`.
-   **Possible useful versions:** A version that consumes source layer without
    copying its data.
-   **TODO:** Investigate average hit rate of this function? Even with low hit
    rate, it probably will not impact performance significantly. However, if hit
    rate optimization will be possible for the previous function, most probably
    these changes will be straightforwardly applicable to this function as well.
-   **Note:** Probably, May be straightforwardly parallelized.

### Statistics

#### ~~_min max_~~

Returns a tuple of minimum and maximum values of an F32 layer.

-   **Usage:** Direct and indirect usages in atmosphere, climatology, hydrology,
    and thermodynamics simulation.
-   **Frequency of the hottest usages:** Few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Ord` or
    `PartialOrd`, `Copy`, and `Bounded`.

#### _sum_

Returns sum of an F32 layer.

-   **Usage:** Direct and indirect usages in thermodynamics, hydrology,
    hydrosphere, crust, spherical geometry, and atmosphere simulation.
-   **Frequency of the hottest usages:** Few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; requires `Sum`.
-   **Other implementations:** U8 layer.
-   **Possible implementations:** Any layer of type implementing `Sum`.

#### _average_

Returns an average value of an F32 layer.

-   **Usage:** Direct and indirect usages in thermodynamics, hydrology,
    hydrosphere, and atmosphere simulation.
-   **Frequency of the hottest usages:** Few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Sum`,
    `Div<f32>`, and `Copy`.

#### _normalize from to_

Normalizes an F32 layer from given old range to given new range.

-   **Usage:** Direct and indirect usages in climatology, thermodynamics,
    spherical geometry, and visualization.
-   **Frequency of the hottest usages:** Multiple times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Sub<f32>`,
    `Add<f32>`, `Mul<f32>`, and `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _normalize to_

Normalizes an F32 layer from inferred old range to given new range.

-   **Usage:** Direct and indirect usages in climatology, thermodynamics,
    spherical geometry, and visualization.
-   **Frequency of the hottest usages:** Multiple times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Sub<f32>`,
    `Add<f32>`, `Mul<f32>`, `Copy`, `Ord` or `PartialOrd`, and `Bounded`.
-   **Note:** Probably, may be entirely or partially parallelized.

#### _normalize_

Normalizes an F32 layer from inferred old range to the $\[0, 1\]$ range.

-   **Usage:** Direct and indirect usages in climatology, thermodynamics,
    spherical geometry, and visualization.
-   **Frequency of the hottest usages:** Multiple times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Sub<f32>`,
    `Add<f32>`, `Mul<f32>`, `Copy`, `Ord` or `PartialOrd`, and `Bounded`, and
    consisting of floats, i.e., F32 layer, V3 layer; also may be implemented for
    integer layers, but it probably will be useless.
-   **Note:** Probably, may be entirely or partially parallelized.

### Raster graphics

#### _copy into selection_

Copies data from one F32 layer to another using a selection Bool layer.

-   **Usage:** Few usages in crust and lithosphere simulation.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _fill into selection_

Fills an F32 layer area with given value.

-   **Usage:** Few usages in crust, hydrosphere, and lithosphere simulation.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** U8 layer.
-   **Possible implementations:** Any layer of type implementing `Copy`.
-   **Note:** May be straightforwardly parallelized.

### Field math

#### _min layer_

Pairwise selects minimum values from two F32 layers.

Writes result to source layer.

-   **Usage:** Few usages in crust.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy` and
    `PartialOrd`.
-   **Note:** May be straightforwardly parallelized.

#### _min scalar_

Pairwise selects minimum of each F32 layer value and given scalar.

Writes result to source layer.

-   **Usage:** Few usages in crust.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy` and
    `PartialOrd<f32>`.
-   **Note:** May be straightforwardly parallelized.

#### _max scalar_

Pairwise selects maximum of each F32 layer value and given scalar.

Writes result to source layer.

-   **Usage:** Few usages in hydrology and crust.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy` and
    `PartialOrd<f32>`.
-   **Note:** May be straightforwardly parallelized.

#### _get gt layer mask_

Pairwise compares values of F32 layers, then writes results to a Bool layer,
where `true` represents values of the source layer greater than values of the
other layer.

-   **Usage:** Few usages in hydrosphere and lithosphere.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `PartialOrd`.
-   **Note:** May be straightforwardly parallelized.

#### _get gt scalar mask_

Compares each value of an F32 layer against given scalar, then writes results to
a Bool layer, where `true` represents source values greater than a value.

-   **Usage:** Few usages in tectonics and lithosphere.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing
    `PartialOrd<f32>`.
-   **Note:** May be straightforwardly parallelized.

#### _get lt scalar mask_

Compares each value of an F32 layer against given scalar, then writes results to
a Bool layer, where `true` represents source values less than a value.

-   **Usage:** Few usages in crust.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing
    `PartialOrd<f32>`.
-   **Note:** May be straightforwardly parallelized.

#### _fma layer_

Triple-wise applies FMA to values of three layers; values from multipliers
layers may be read repeatedly.

Writes result to the source layer.

-   **Usage:** Few usages in crust and lithosphere.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Mul`, `Add`,
    `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _fma scalar_

Multiplies other layer values by a scalar, then adds it to values of source
layer.

Writes result to the source layer.

-   **Usage:** Few usages in crust, lithosphere, and climatology.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Mul`, `Add`,
    `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _add layer_

Pairwise adds values of two F32 layers.

Has two versions, one writes result to a third layer, second writes result to a
source layer.

-   **Usage:** Few usages in atmosphere, crust, and universe.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** U8 layer, V3 layer.
-   **Possible implementations:** Any layer of type implementing `Add`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _sub layer_

Pairwise subtracts values of two F32 layers.

Writes result values to other layer.

-   **Usage:** One usage in atmosphere.
-   **Frequency of the hottest usages:** Once per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Sub`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _mul layer_

Pairwise multiplies values of two F32 layers, or an F32 layer and U8 or Bool
layer.

Has two versions, one writes result to a third layer, second writes result to a
source layer.

-   **Usage:** Few usages in crust, tectonics, atmosphere, and thermodynamics.
-   **Frequency of the hottest usages:** At least once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer has a _\[v3\] → \[f32\] → \[v3\]_
    implementation.
-   **Possible implementations:** Any layer of type implementing `Mul`,
    `Mul<u8>`, `Mul<bool>`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _div layer_

Pairwise divides values of two F32 layers.

Writes result values to other layer.

-   **Usage:** Few usages in crust and atmosphere.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer has a _\[v3\] → \[f32\] → \[v3\]_
    implementation.
-   **Possible implementations:** Any layer of type implementing `Div`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _div scalar_

Divides an F32 layer values by a given scalar.

Has two versions, one writes result to a third layer, second writes result to a
source layer.

-   **Usage:** Few usages in crust, atmosphere, and visualization.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Div<f32>`,
    `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _inv layer_

Inverts an F32 layer values.

Writes result to the source layer.

-   **Usage:** One usage in crust.
-   **Frequency of the hottest usages:** Once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** We may introduce the `Inv` trait and implement
    it for numbers and matrices, then, this function may be implemented for
    layer of any type implementing `Inv`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _add value_

Adds given value to each item of an F32 layer.

Writes result to the source layer.

-   **Usage:** Few usages in crust, atmosphere, lithosphere.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Add`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _sub value_

Subtracts given value from each item of an F32 layer.

Has two versions, one writes result to a third layer, second writes result to a
source layer.

-   **Usage:** Few usages in hydrology, crust, hydrosphere, visualization,
    thermodynamics.
-   **Frequency of the hottest usages:** At least few times per plate per
    iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Sub`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _mul scalar_

Multiplies an F32 layer values by a given scalar.

Has two versions, one writes result to a third layer, second writes result to a
source layer.

-   **Usage:** Few usages in climatology, fluid mechanics, tectonics,
    thermodynamics, atmosphere, crust, visualization, hydrology, optics.
-   **Frequency of the hottest usages:** Really fucking hot.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Mul`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _pow scalar_

Exponentiates an F32 layer values to a given scalar power.

Writes result to the source layer.

-   **Usage:** One usage in thermodynamics.
-   **Frequency of the hottest usages:** Once per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** We may introduce a `Pow` trait, then implement
    this function for any layer of type implementing `Pow`.
-   **Note:** May be straightforwardly parallelized.

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

Applies a `mix`-like interpolation to each value of an F32 layer. While the
`mix` interpolation uses the $x \times (1 - a) + y \times a$ formula, this one
uses the $x + a \times (y - x)$ formula.

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

-   **Usage:** Mo usages at all; one potential usage in crust generator.
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

### Core functionality

#### _from buffer_

Creates new U8 layer from raw array buffer at the given offset, and of the same
length as given grid.

-   **Usage:** Single usage in plate constructor; probably may be replaced with
    Bool layer.
-   **Frequency of the hottest usages:** Once per plate per supercontinent
    cycle.
-   **Conclusion:** Reject. Allocation in Rust have much lower cost than in
    JavaScript. Also, code using this function isn’t hot and may be safely
    suboptimal even in JavaScript.
-   **Status:** Rejected.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** _N/A._

#### _fill_

Fills a layer with given value.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Reject.
-   **Status:** Implemented in the Rust library.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Copy`; most
    optimized version may be implemented for layers of single-byte types, or
    when filling a layer with all zeroes.
-   **TODO:** Investigate whether the existing implementation makes use of WASM
    `memory.fill`; probably we will need to reimplement it on our side.
-   **Note:** May be straightforwardly parallelized.

#### _swizzle_

Copies data from one layer to another using indices mapping from a supplementary
Usize layer.

-   **Usage:** Few usages in lithosphere simulation.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Copy`.
-   **Possible useful versions:** A version that consumes source layer without
    copying its data.
-   **TODO:** Investigate average hit rate of this function.
-   **Note:** Probably, May be straightforwardly parallelized.

### Statistics

#### _sum_

Returns sum of an U8 layer.

-   **Usage:** One usage in image segmentation. Probably, U8 layers used here as
    Bool layers, so this function works as popcnt for a boolean array.
-   **Frequency of the hottest usages:** Once per supercontinent cycle.
-   **Conclusion:** Keep.
-   **Status:** Implemented in the Rust library; possible implementation for
    Bool layers should be implemented on our side.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Sum`.

#### _unique_

Returns an array of unique values of a U8 layer.

-   **Usage:** Two usages in tectonics and lithosphere; probably may be reduced
    to one.
-   **Frequency of the hottest usages:** Once per supercontinent cycle.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy` and
    `Hash`. `Hash`? `Copy`? Investigate how to implement this in Rust most
    efficiently.

### Raster graphics

#### _fill into selection_

Fills a U8 layer area with given value.

-   **Usage:** Few usages in tectonics and lithosphere simulation.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Copy`.
-   **Note:** May be straightforwardly parallelized.

### Field math

#### _get eq scalar mask_

Compares each value of a U8 layer against given scalar, then writes results to a
Bool layer, where `true` represents source values equal to value.

-   **Usage:** Few usages in tectonics and lithosphere.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `PartialEq`.
-   **Note:** May be straightforwardly parallelized.

#### _get ne scalar mask_

Compares each value of a U8 layer against given scalar, then writes results to a
Bool layer, where `true` represents source values unequal to value.

-   **Usage:** Few usages in tectonics and lithosphere.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `PartialEq`.
-   **Note:** May be straightforwardly parallelized.

#### _add layer_

Pairwise adds values of two U8 layers.

Writes result to other layer.

-   **Usage:** Few usages in atmosphere, crust, and universe.
-   **Frequency of the hottest usages:** At least once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer, V3 layer.
-   **Possible implementations:** Any layer of type implementing `Add`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

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

## Usize layer

### Core functionality

### Statistics

### Raster graphics

### Misc

## V3 layer

### Core functionality

#### _of length_

Creates new V3 layer of given length, _and with a given associated grid with_,
where given length may be unequal to length of a grid.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Unknown.
-   **Conclusion:** Probably useless. We will not use associated grids, so we
    don’t need grid-related functionality in layers; most probably it will be
    much simpler to create new layers explicitly.
-   **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Default`.

#### _from vectors_

Creates new SoA V3 layer from an AoS V3 layer.

-   **Usage:** Two usages in grid constructor and Voronoi sphere constructor.
-   **Frequency of the hottest usages:** Few times per simulation.
-   **Conclusion:** Probably useless. Before MVP, we will use AoS vector layers
    by default.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

#### _to array_

Creates new AoS V3 layer from a SoA V3 layer.

-   **Usage:** Few usages in Voronoi sphere constructor.
-   **Frequency of the hottest usages:** Few times per simulation.
-   **Conclusion:** Probably useless. See previous conclusion.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

### Statistics

#### _normalize to_

Normalizes magnitudes of vectors from inferred current range to the $[0, n]$
range, where $n$ is given value.

-   **Usage:** One usage in climatology.
-   **Frequency of the hottest usages:** Once per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Sub<f32>`,
    `Add<f32>`, `Mul<f32>`, `Copy`, `Ord` or `PartialOrd`, and `Bounded`.
-   **Note:** May be entirely or partially parallelized.

#### _weighted average_

Calculates weighted average of a V3 layer. Most probably weights layer may be a
Bool layer, not an F32 layer or U8 layer.

-   **Usage:** Two usages in tectonics.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Sum`,
    `Div<f32>`, `Mul<f32>`, and `Copy`.
-   **Note:** May be entirely or partially parallelized.

#### _normalize values_

Normalizes each vector in a layer.

-   **Usage:** One usage in tectonics.
-   **Frequency of the hottest usages:** At least once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any vector/matrix/etc. layer.
-   **Note:** May be straightforwardly parallelized.

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

#### _add layer_

Pairwise adds values of two V3 layers.

Writes result to source layer.

-   **Usage:** One usage in climatology.
-   **Frequency of the hottest usages:** At least once per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** U8 layer, F32 layer.
-   **Possible implementations:** Any layer of type implementing `Add`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _sub value_

Subtracts given value from each item of a V3 layer.

Writes result to other layer.

-   **Usage:** One usage in tectonics.
-   **Frequency of the hottest usages:** At least once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Sub`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _div layer_

Pairwise divides values of a V3 layer by values of an F32 layer.

Writes result values to source layer.

-   **Usage:** One usage in tectonics.
-   **Frequency of the hottest usages:** At least once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Div`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _div scalar_

Divides a V3 layer values by a given scalar.

Writes result values to source layer.

-   **Usage:** Few usages in climatology and visualization.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Div<f32>`,
    `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _mul scalar_

Multiplies a V3 layer values by a given scalar.

Writes result values to other layer.

-   **Usage:** Few usages in climatology and visualization.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Mul`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _dot mul value_

Calculates dot product of values of a V3 layer and given vector.

Writes result values to other layer.

-   **Usage:** One usage in spherical geometry.
-   **Frequency of the hottest usages:** Once per simulation.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _N/A._
-   **Possible implementations:** IDK.
-   **Note:** May be straightforwardly parallelized.

#### _cross mul value_

Calculates cross product of values of a V3 layer and given vector.

Writes result to other layer.

-   **Usage:** One usage in Voronoi.
-   **Frequency of the hottest usages:** Once per simulation.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _N/A._
-   **Possible implementations:** IDK.
-   **Note:** May be straightforwardly parallelized.

#### _mul m4_

Multiplies a V3 layer values by a given M3.

Writes result to other layer.

-   **Usage:** Two usages in plate.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _N/A._
-   **Possible implementations:** IDK.
-   **Note:** May be straightforwardly parallelized.

#### _mul layer_

Pairwise multiplies values of a V3 layer by values of an F32 layer.

Has two versions, one writes result to a third layer, second writes result to a
source layer.

-   **Usage:** Few usages in tectonics and climatology.
-   **Frequency of the hottest usages:** At least once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Mul`,
    `Mul<u8>`, `Mul<bool>`, `Copy`.
-   **Note:** May be straightforwardly parallelized.

#### _dot mul layer_

Pairwise calculates dot product of values of two V3 layers.

Writes result values to other layer.

-   **Usage:** One usage in spherical geometry.
-   **Frequency of the hottest usages:** Once per simulation.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _N/A._
-   **Possible implementations:** IDK.
-   **Note:** May be straightforwardly parallelized.

#### _cross mul layer_

Pairwise calculates cross product of values of two V3 layers.

Writes result to other layer.

-   **Usage:** Usages in climatology, tectonics, lithosphere, Voronoi.
-   **Frequency of the hottest usages:** At least twice per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _N/A._
-   **Possible implementations:** IDK.
-   **Note:** May be straightforwardly parallelized.

#### _get magnitudes_

Calculates magnitudes of V3 layer values.

-   **Usage:** Usages in tectonics and maths.
-   **Frequency of the hottest usages:** At least once per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _N/A._
-   **Possible implementations:** IDK.
-   **Note:** May be straightforwardly parallelized.

#### _similarity_

Calculates V3 layer values similarity to a given V3.

-   **Usage:** One usage in optics.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _N/A._
-   **Possible implementations:** IDK.
-   **Note:** May be straightforwardly parallelized.

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

# Primitives

## V3

### _add v3_

Adds another v3 to the source v3.

Writes result to the source v3.

-   **Usage:** Few usages in Voronoi; inline usages in V3 layer field
    operations.
-   **Frequency of the hottest usages:** Few times per simulation.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure; as `Add`).
-   **Other implementations:** _None._
-   **Possible implementations:** ~~V4, M3, M4~~.

### _dot mul v3_

Calculates dot product of two v3s.

-   **Usage:** Few usages in Voronoi, inline usages in V3 layer field
    operations.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** _None._
-   **Possible implementations:** ~~V4.~~

### _mul m3_

Multiplies a v3 by an m3.

Writes result to the source v3.

-   **Usage:** No usages; inline usages in V3 layer field operations.
-   **Frequency of the hottest usages:** No usages.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure; as `Mul`).
-   **Other implementations:** _None._
-   **Possible implementations:** ~~M3.~~

### _div scalar_

Divides a v3 by a scalar.

Writes result to the source v3.

-   **Usage:** No usages; inline usage in _normalize_ and V3 layer field
    operations.
-   **Frequency of the hottest usages:** No usages.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure; as `Div`).
-   **Other implementations:** _None._
-   **Possible implementations:** ~~V4, M3, M4.~~

### _similarity_

Calculates similarity measure of two v3s.

-   **Usage:** One usage in V3 layer flood fill.
-   **Frequency of the hottest usages:** Multiple times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** _None._
-   **Possible implementations:** ~~V4. M3, M4?~~

### _magnitude_

Calculates magnitude of a v3.

-   **Usage:** Usages in optics and visualization, indirect usage in tectonics,
    multiple inline usages.
-   **Frequency of the hottest usages:** At least few times per plate per
    iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** _None._
-   **Possible implementations:** ~~V4.~~

### _normalize_

Normalizes a v3.

Writes result to other vector.

-   **Usage:** Usages in visualization, indirect usage in tectonics.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure).
-   **Other implementations:** _None._
-   **Possible implementations:** ~~V4.~~

### _to rotation m3_

Creates a rotation m3 from a rotation v3.

Writes result to other m3.

-   **Usage:** Usages in tectonics.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure).
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

### _to rotation m4_

Creates a rotation m4 from a rotation v3.

Writes result to other m4.

-   **Usage:** Usages in orbital mechanics.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure).
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

### _to translation m4_

Creates a translation m4 from a translation v3.

Writes result to other m4.

-   **Usage:** Usages in orbital mechanics.
-   **Frequency of the hottest usages:** At least few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure).
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

## V4

## M3

### _invert_

Inverts an m3.

Writes result to other m3.

-   **Usage:** Usages in plate.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure).
-   **Other implementations:** ~~M4.~~
-   **Possible implementations:** _N/A._

### _mul_

Multiplies one m3 by another.

Writes result to first m3.

-   **Usage:** Usages in tectonics and plate.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** ~~M4.~~
-   **Possible implementations:** _N/A._

## M4

### _invert_

Inverts an m4.

Writes result to other m4.

-   **Usage:** Usages in orbital mechanics.
-   **Frequency of the hottest usages:** Few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented (pure).
-   **Other implementations:** ~~M3.~~
-   **Possible implementations:** _N/A._

### _mul_

Multiplies one m4 by another.

Writes result to other m4.

-   **Usage:** Usages in orbital mechanics and celestial cycle.
-   **Frequency of the hottest usages:** Few times per celestial cycle per
    iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** ~~M3.~~
-   **Possible implementations:** _N/A._

# _TBC_

[![To Be Continued](https://img.youtube.com/vi/TEYG1ZXU2Pc/0.jpg)](https://youtu.be/TEYG1ZXU2Pc)
