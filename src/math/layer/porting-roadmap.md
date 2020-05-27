# F32 layer

-   _from example_ — creates new f32 layer of the same length as given layer,
    _and with the same associated grid_.

    **Usage:** Unknown.

    **Frequency of the hottest usages:** Multiple times per iteration.

    **Conclusion:** Probably useless. We will not use associated grids, so we
    don’t need grid-related functionality in layers; most probably it will be
    much simpler to create new layers explicitly.

    **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.

    **Other implementations:** _None._

-   _of length_ — creates new f32 layer of given length, _and with a given
    associated grid with_, where given length may be unequal to length of a
    grid.

    **Usage:** Unknown.

    **Frequency of the hottest usages:** Twice per plate per iteration.

    **Conclusion:** Probably useless. See previous conclusion.

    **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.

    **Other implementations:** V3 layer.

-   _from u8 layer_ — creates new f32 layer of the same length as given u8
    layer, with the same data as in given layer, _and with the same associated
    grid_.

    **Usage**: Few usages in the visualization code, required to convert certain
    internal layers to a common visualizable data layers.

    **Frequency of the hottest usages:** Once per frame.

    **Conclusion:** The functionality is required, the function probably may be
    eliminated. A _u8 layer :: to f32 layer_ probably will be better.

    **Status:** Unimplemented; probably implemented in the Rust library, or may
    be implemented as a one-liner.

    **Other implementations:** _None._

    **TODO:** Investigate whether u8 layers required to convert to f32 layers
    may be safely replaced with f32 layers internally.

-   _from buffer_ — creates new f32 layer from raw array buffer at the given
    offset, and of the same length a given grid.

    **Usage:** Used to reduce allocations in some places.

    **Frequency of the hottest usages:** Few times per supercontinent cycle.

    **Conclusion:** Reject. Allocation in Rust have much lower cost than in
    JavaScript. Also, code using this function isn’t hot and may be safely
    suboptimal even in JavaScript.

    **Status:** Rejected.

    **Other implementations:** U8 layer.

-   _copy_ — copies data from one layer to another.

    **Usage:** Unknown.

    **Frequency of the hottest usages:** Once pre iteration.

    **Conclusion:** Reject.

    **Status:** Implemented in the Rust library.

    **Other implementations:** _None._

    **TODO:** Investigate whether the existing implementation makes use of WASM
    `memory.copy`; probably we will need to reimplement it on our side.

-   _fill_ — fills a layer with given value.

    **Usage:** Unknown.

    **Frequency of the hottest usages:** Really fucking hot.

    **Conclusion:** Reject.

    **Status:** Implemented in the Rust library.

    **Other implementations:** U8 layer.

    **TODO:** Investigate whether the existing implementation makes use of WASM
    `memory.fill`; probably we will need to reimplement it on our side.

-   _max index_ — returns an index of an element with max value.

    **Usage:** Single usage in an image segmentation algorithm.

    **Frequency of the hottest usages:** Multiple times per supercontinent
    cycle.

    **Conclusion:** Keep.

    **Status:** Unimplemented.

    **Other implementations:** _None._

-   _swizzle_ — copies data from one layer to another using indices mapping from
    a supplementary usize layer.

    **Usage:** Lots of them across almost every simulation module.

    **Frequency of the hottest usages:** Really fucking hot.

    **Conclusion:** Keep.

    **Status:** Implemented.

    **Other implementations:** U8 layer.

    **TODO:** Investigate average hit rate of this function.

-   _inverse_swizzle_add_ — adds values from one layer to existing values in
    other layer using indices mapping from a supplementary usize layer.

    **Usage:** One indirect usage in the lithosphere model.

    **Frequency of the hottest usages:** Multiple times per plate per iteration.

    **Conclusion:** Keep.

    **Status:** Implemented.

    **Other implementations:** _None._

    **TODO:** Investigate average hit rate of this function. Or don’t. Even with
    low hit rate, it probably will not impact performance significantly.

[![To Be Continued](https://img.youtube.com/vi/TEYG1ZXU2Pc/0.jpg)](https://youtu.be/TEYG1ZXU2Pc)
