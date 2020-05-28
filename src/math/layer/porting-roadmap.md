# F32 layer

## _from example_

Creates new fF32 layer of the same length as given layer, _and with the same
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

## _of length_

Creates new F32 layer of given length, _and with a given associated grid with_,
where given length may be unequal to length of a grid.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Twice per plate per iteration.
-   **Conclusion:** Probably useless. See previous conclusion.
-   **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.
-   **Other implementations:** V3 layer.
-   **Possible implementations:** Any layer of type implementing `Default`.

## _from u8 layer_

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

## _from buffer_

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

## _copy_

Copies data from one layer to another.

-   **Usage:** Unknown.
-   **Frequency of the hottest usages:** Once pre iteration.
-   **Conclusion:** Reject.
-   **Status:** Implemented in the Rust library.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Copy`.
-   **TODO:** Make sure whether the existing implementation uses WASM
    `memory.copy`; probably we will need to reimplement it on our side.

## _fill_

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

## _max index_

Returns an index of an element with max value.

-   **Usage:** Single usage in an image segmentation algorithm.
-   **Frequency of the hottest usages:** Multiple times per supercontinent
    cycle.
-   **Conclusion:** Keep.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** Any layer of type implementing `Ord` or
    `PartialOrd`.

## _swizzle_

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

## _inverse swizzle add_

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

# U8 layer

## _from buffer_

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

## _fill_

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

## _swizzle_

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

# Usize layer

# V3 layer

## _of length_

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

## _from vectors_

Creates new SoA V3 layer from an AoS V3 layer.

-   **Usage:** Two usages in grid constructor and Voronoi sphere constructor.
-   **Frequency of the hottest usages:** Few times per simulation.
-   **Conclusion:** Probably useless. Before MVP, we will use AoS vector layers
    by default.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

## _to array_

Creates new AoS V3 layer from a SoA V3 layer.

-   **Usage:** Few usages in Voronoi sphere constructor.
-   **Frequency of the hottest usages:** Few times per simulation.
-   **Conclusion:** Probably useless. See previous conclusion.
-   **Status:** Unimplemented.
-   **Other implementations:** _None._
-   **Possible implementations:** _N/A._

# _TBC_

[![To Be Continued](https://img.youtube.com/vi/TEYG1ZXU2Pc/0.jpg)](https://youtu.be/TEYG1ZXU2Pc)
