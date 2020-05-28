# F32 layer

## Core functionality

### _from example_

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

### _of length_

Creates new F32 layer of given length, _and with a given associated grid with_,
where given length may be unequal to length of a grid.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Twice per plate per iteration.
-   **Conclusion:** Probably useless. See previous conclusion.
-   **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Default`.

### _from u8 layer_

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

### _from buffer_

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

### _copy_

Copies data from one layer to another.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Once pre iteration.
-   **Conclusion:** Reject.
-   **Status:** Implemented in the Rust library.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy`.
-   **TODO:** Make sure whether the existing implementation uses WASM
    `memory.copy`; probably we will need to reimplement it on our side.

### _fill_

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

### _max index_

Returns an index of an element with max value.

-   **Usage:** Single usage in an image segmentation algorithm.
-   **Frequency of the hottest usages:** Multiple times per supercontinent
    cycle.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Ord` or
    `PartialOrd`.

### _swizzle_

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

### _inverse swizzle add_

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

## Statistics

### _min max_

Returns a tuple of minimum and maximum values of an F32 layer.

-   **Usage:** Direct and indirect usages in atmosphere, climatology, hydrology,
    and thermodynamics simulation.
-   **Frequency of the hottest usages:** Few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Implemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Ord` or
    `PartialOrd`, `Copy`, and `Bounded`.

### _sum_

Returns sum of an F32 layer.

-   **Usage:** Direct and indirect usages in thermodynamics, hydrology,
    hydrosphere, crust, spherical geometry, and atmosphere simulation.
-   **Frequency of the hottest usages:** Few times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented; requires `Sum`.
-   **Other implementations:** U8 layer.
-   **Possible implementations:** Any layer of type implementing `Sum`.

### _average_

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

### _normalize from to_

Normalizes an F32 layer from given old range to given new range.

-   **Usage:** Direct and indirect usages in climatology, thermodynamics,
    spherical geometry, and visualization.
-   **Frequency of the hottest usages:** Multiple times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Sub<f32>`,
    `Add<f32>`, `Mul<f32>`, and `Copy`.

### _normalize to_

Normalizes an F32 layer from inferred old range to given new range.

-   **Usage:** Direct and indirect usages in climatology, thermodynamics,
    spherical geometry, and visualization.
-   **Frequency of the hottest usages:** Multiple times per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Sub<f32>`,
    `Add<f32>`, `Mul<f32>`, `Copy`, `Ord` or `PartialOrd`, and `Bounded`.

### _normalize_

Normalizes an F32 layer from inferred old range to the $[0, 1]$ range, or
$[0, 1)$, or $(0, 1]$, IDK.

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

## Raster graphics

### _copy into selection_

Copies data from one F32 layer to another using a selection Bool layer.

-   **Usage:** Few usages in crust and lithosphere simulation.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy`.

### _fill into selection_

Fills an F32 layer area with given value.

-   **Usage:** Few usages in crust, hydrosphere, and lithosphere simulation.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** U8 layer.
-   **Possible implementations:** Any layer of type implementing `Copy`.

# U8 layer

## Core functionality

### _from buffer_

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

### _fill_

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

### _swizzle_

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

## Statistics

### _sum_

Returns sum of an U8 layer.

-   **Usage:** One usage in image segmentation. Probably, U8 layers used here as
    Bool layers, so this function works as popcnt for a boolean array.
-   **Frequency of the hottest usages:** Once per supercontinent cycle.
-   **Conclusion:** Keep.
-   **Status:** Implemented in the Rust library; possible implementation for
    Bool layers should be implemented on our side.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Sum`.

### _unique_

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

## Raster graphics

### _fill into selection_

Fills a U8 layer area with given value.

-   **Usage:** Few usages in tectonics and lithosphere simulation.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Copy`.

# Usize layer

## Core functionality

## Statistics

## Raster graphics

# V3 layer

## Core functionality

### _of length_

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

### _from vectors_

Creates new SoA V3 layer from an AoS V3 layer.

-   **Usage:** Two usages in grid constructor and Voronoi sphere constructor.
-   **Frequency of the hottest usages:** Few times per simulation.
-   **Conclusion:** Probably useless. Before MVP, we will use AoS vector layers
    by default.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

### _to array_

Creates new AoS V3 layer from a SoA V3 layer.

-   **Usage:** Few usages in Voronoi sphere constructor.
-   **Frequency of the hottest usages:** Few times per simulation.
-   **Conclusion:** Probably useless. See previous conclusion.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

## Statistics

### _normalize to_

Normalizes magnitudes of vectors from inferred current range to the $[0, n]$
range, where $n$ is given value.

-   **Usage:** One usage in climatology.
-   **Frequency of the hottest usages:** Once per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** F32 layer.
-   **Possible implementations:** Any layer of type implementing `Sub<f32>`,
    `Add<f32>`, `Mul<f32>`, `Copy`, `Ord` or `PartialOrd`, and `Bounded`.

### _weighted average_

Calculates weighted average of a V3 layer. Most probably weights layer may be a
Bool layer, not an F32 layer or U8 layer.

-   **Usage:** Two usages in tectonics.
-   **Frequency of the hottest usages:** Few times per plate per iteration.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Sum`,
    `Div<f32>`, `Mul<f32>`, and `Copy`.

## Raster graphics

### _flood select_

Selects an area of a V3 layer using flood fill. Also, uses a neighbor lookup
table from given grid.

-   **Usage:** One usage in tectonics.
-   **Frequency of the hottest usages:** Once per supercontinent cycle.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** We may introduce a trait, say, `Similarity`,
    and then implement this function for any layer implementing this trait.
-   **TODO:** Investigate hit rate of this function, or… Even with extremely low
    hit rate, this function will not significantly affect performance of a
    simulation.

# _TBC_

[![To Be Continued](https://img.youtube.com/vi/TEYG1ZXU2Pc/0.jpg)](https://youtu.be/TEYG1ZXU2Pc)
