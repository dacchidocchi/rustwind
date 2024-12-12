def_types!(
    /// Utilities for controlling the initial size of flex items.
    ///
    /// <https://tailwindcss.com/docs/flex-basis>
    FlexBasis {
        /// ```css
        /// {
        ///     flex-basis: 0px;
        /// }
        /// ```
        _0 => "basis-0",
        /// ```css
        /// {
        ///     flex-basis: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "basis-1",
        /// ```css
        /// {
        ///     flex-basis: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "basis-2",
        /// ```css
        /// {
        ///     flex-basis: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "basis-3",
        /// ```css
        /// {
        ///     flex-basis: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "basis-4",
        /// ```css
        /// {
        ///     flex-basis: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "basis-5",
        /// ```css
        /// {
        ///     flex-basis: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "basis-6",
        /// ```css
        /// {
        ///     flex-basis: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "basis-7",
        /// ```css
        /// {
        ///     flex-basis: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "basis-8",
        /// ```css
        /// {
        ///     flex-basis: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "basis-9",
        /// ```css
        /// {
        ///     flex-basis: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "basis-10",
        /// ```css
        /// {
        ///     flex-basis: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "basis-11",
        /// ```css
        /// {
        ///     flex-basis: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "basis-12",
        /// ```css
        /// {
        ///     flex-basis: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "basis-14",
        /// ```css
        /// {
        ///     flex-basis: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "basis-16",
        /// ```css
        /// {
        ///     flex-basis: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "basis-20",
        /// ```css
        /// {
        ///     flex-basis: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "basis-24",
        /// ```css
        /// {
        ///     flex-basis: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "basis-28",
        /// ```css
        /// {
        ///     flex-basis: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "basis-32",
        /// ```css
        /// {
        ///     flex-basis: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "basis-36",
        /// ```css
        /// {
        ///     flex-basis: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "basis-40",
        /// ```css
        /// {
        ///     flex-basis: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "basis-44",
        /// ```css
        /// {
        ///     flex-basis: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "basis-48",
        /// ```css
        /// {
        ///     flex-basis: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "basis-52",
        /// ```css
        /// {
        ///     flex-basis: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "basis-56",
        /// ```css
        /// {
        ///     flex-basis: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "basis-60",
        /// ```css
        /// {
        ///     flex-basis: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "basis-64",
        /// ```css
        /// {
        ///     flex-basis: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "basis-72",
        /// ```css
        /// {
        ///     flex-basis: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "basis-80",
        /// ```css
        /// {
        ///     flex-basis: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "basis-96",
        /// ```css
        /// {
        ///     flex-basis: auto;
        /// }
        /// ```
        Auto => "basis-auto",
        /// ```css
        /// {
        ///     flex-basis: 1px;
        /// }
        /// ```
        Px => "basis-px",
        /// ```css
        /// {
        ///     flex-basis: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "basis-0.5",
        /// ```css
        /// {
        ///     flex-basis: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "basis-1.5",
        /// ```css
        /// {
        ///     flex-basis: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "basis-2.5",
        /// ```css
        /// {
        ///     flex-basis: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "basis-3.5",
        /// ```css
        /// {
        ///     flex-basis: 50%;
        /// }
        /// ```
        _1over2 => "basis-1/2",
        /// ```css
        /// {
        ///     flex-basis: 33.333333%;
        /// }
        /// ```
        _1over3 => "basis-1/3",
        /// ```css
        /// {
        ///     flex-basis: 66.666667%;
        /// }
        /// ```
        _2over3 => "basis-2/3",
        /// ```css
        /// {
        ///     flex-basis: 25%;
        /// }
        /// ```
        _1over4 => "basis-1/4",
        /// ```css
        /// {
        ///     flex-basis: 50%;
        /// }
        /// ```
        _2over4 => "basis-2/4",
        /// ```css
        /// {
        ///     flex-basis: 75%;
        /// }
        /// ```
        _3over4 => "basis-3/4",
        /// ```css
        /// {
        ///     flex-basis: 20%;
        /// }
        /// ```
        _1over5 => "basis-1/5",
        /// ```css
        /// {
        ///     flex-basis: 40%;
        /// }
        /// ```
        _2over5 => "basis-2/5",
        /// ```css
        /// {
        ///     flex-basis: 60%;
        /// }
        /// ```
        _3over5 => "basis-3/5",
        /// ```css
        /// {
        ///     flex-basis: 80%;
        /// }
        /// ```
        _4over5 => "basis-4/5",
        /// ```css
        /// {
        ///     flex-basis: 16.666667%;
        /// }
        /// ```
        _1over6 => "basis-1/6",
        /// ```css
        /// {
        ///     flex-basis: 33.333333%;
        /// }
        /// ```
        _2over6 => "basis-2/6",
        /// ```css
        /// {
        ///     flex-basis: 50%;
        /// }
        /// ```
        _3over6 => "basis-3/6",
        /// ```css
        /// {
        ///     flex-basis: 66.666667%;
        /// }
        /// ```
        _4over6 => "basis-4/6",
        /// ```css
        /// {
        ///     flex-basis: 83.333333%;
        /// }
        /// ```
        _5over6 => "basis-5/6",
        /// ```css
        /// {
        ///     flex-basis: 8.333333%;
        /// }
        /// ```
        _1over12 => "basis-1/12",
        /// ```css
        /// {
        ///     flex-basis: 16.666667%;
        /// }
        /// ```
        _2over12 => "basis-2/12",
        /// ```css
        /// {
        ///     flex-basis: 25%;
        /// }
        /// ```
        _3over12 => "basis-3/12",
        /// ```css
        /// {
        ///     flex-basis: 33.333333%;
        /// }
        /// ```
        _4over12 => "basis-4/12",
        /// ```css
        /// {
        ///     flex-basis: 41.666667%;
        /// }
        /// ```
        _5over12 => "basis-5/12",
        /// ```css
        /// {
        ///     flex-basis: 50%;
        /// }
        /// ```
        _6over12 => "basis-6/12",
        /// ```css
        /// {
        ///     flex-basis: 58.333333%;
        /// }
        /// ```
        _7over12 => "basis-7/12",
        /// ```css
        /// {
        ///     flex-basis: 66.666667%;
        /// }
        /// ```
        _8over12 => "basis-8/12",
        /// ```css
        /// {
        ///     flex-basis: 75%;
        /// }
        /// ```
        _9over12 => "basis-9/12",
        /// ```css
        /// {
        ///     flex-basis: 83.333333%;
        /// }
        /// ```
        _10over12 => "basis-10/12",
        /// ```css
        /// {
        ///     flex-basis: 91.666667%;
        /// }
        /// ```
        _11over12 => "basis-11/12",
        /// ```css
        /// {
        ///     flex-basis: 100%;
        /// }
        /// ```
        Full => "basis-full",
    }
    /// Utilities for controlling the direction of flex items.
    ///
    /// <https://tailwindcss.com/docs/flex-direction>
    FlexDirection {
        /// ```css
        /// {
        ///     flex-direction: row;
        /// }
        /// ```
        Row => "flex-row",
        /// ```css
        /// {
        ///     flex-direction: row-reverse;
        /// }
        /// ```
        RowReverse => "flex-row-reverse",
        /// ```css
        /// {
        ///     flex-direction: column;
        /// }
        /// ```
        Col => "flex-col",
        /// ```css
        /// {
        ///     flex-direction: column-reverse;
        /// }
        /// ```
        ColReverse => "flex-col-reverse",
    }
    /// Utilities for controlling how flex items wrap.
    ///
    /// <https://tailwindcss.com/docs/flex-wrap>
    FlexWrap {
        /// ```css
        /// {
        ///     flex-wrap: wrap;
        /// }
        /// ```
        Wrap => "flex-wrap",
        /// ```css
        /// {
        ///     flex-wrap: wrap-reverse;
        /// }
        /// ```
        WrapReverse => "flex-wrap-reverse",
        /// ```css
        /// {
        ///     flex-wrap: nowrap;
        /// }
        /// ```
        Nowrap => "flex-nowrap",
    }
    /// Utilities for controlling how flex items both grow and shrink.
    ///
    /// <https://tailwindcss.com/docs/flex>
    Flex {
        /// ```css
        /// {
        ///     flex: 1 1 0%;
        /// }
        /// ```
        _1 => "flex-1",
        /// ```css
        /// {
        ///     flex: 1 1 auto;
        /// }
        /// ```
        Auto => "flex-auto",
        /// ```css
        /// {
        ///     flex: 0 1 auto;
        /// }
        /// ```
        Initial => "flex-initial",
        /// ```css
        /// {
        ///     flex: none;
        /// }
        /// ```
        None => "flex-none",
    }
    /// Utilities for controlling how flex items grow.
    ///
    /// <https://tailwindcss.com/docs/flex-grow>
    FlexGrow {
        /// ```css
        /// {
        ///     flex-grow: 1;
        /// }
        /// ```
        Grow => "grow",
        /// ```css
        /// {
        ///     flex-grow: 0;
        /// }
        /// ```
        _0 => "grow-0",
    }
    /// Utilities for controlling how flex items shrink.
    ///
    /// <https://tailwindcss.com/docs/flex-shrink>
    FlexShrink {
        /// ```css
        /// {
        ///     flex-shrink: 1;
        /// }
        /// ```
        Shrink => "shrink",
        /// ```css
        /// {
        ///     flex-shrink: 0;
        /// }
        /// ```
        _0 => "shrink-0",
    }
    /// Utilities for controlling the order of flex and grid items.
    ///
    /// <https://tailwindcss.com/docs/order>
    Order {
        /// ```css
        /// {
        ///     order: 1;
        /// }
        /// ```
        _1 => "order-1",
        /// ```css
        /// {
        ///     order: 2;
        /// }
        /// ```
        _2 => "order-2",
        /// ```css
        /// {
        ///     order: 3;
        /// }
        /// ```
        _3 => "order-3",
        /// ```css
        /// {
        ///     order: 4;
        /// }
        /// ```
        _4 => "order-4",
        /// ```css
        /// {
        ///     order: 5;
        /// }
        /// ```
        _5 => "order-5",
        /// ```css
        /// {
        ///     order: 6;
        /// }
        /// ```
        _6 => "order-6",
        /// ```css
        /// {
        ///     order: 7;
        /// }
        /// ```
        _7 => "order-7",
        /// ```css
        /// {
        ///     order: 8;
        /// }
        /// ```
        _8 => "order-8",
        /// ```css
        /// {
        ///     order: 9;
        /// }
        /// ```
        _9 => "order-9",
        /// ```css
        /// {
        ///     order: 10;
        /// }
        /// ```
        _10 => "order-10",
        /// ```css
        /// {
        ///     order: 11;
        /// }
        /// ```
        _11 => "order-11",
        /// ```css
        /// {
        ///     order: 12;
        /// }
        /// ```
        _12 => "order-12",
        /// ```css
        /// {
        ///     order: -9999;
        /// }
        /// ```
        First => "order-first",
        /// ```css
        /// {
        ///     order: 9999;
        /// }
        /// ```
        Last => "order-last",
        /// ```css
        /// {
        ///     order: 0;
        /// }
        /// ```
        None => "order-none",
    }
    /// Utilities for specifying the columns in a grid layout.
    ///
    /// <https://tailwindcss.com/docs/grid-template-columns>
    GridTemplateColumns {
        /// ```css
        /// {
        ///     grid-template-columns: repeat(1, minmax(0, 1fr));
        /// }
        /// ```
        _1 => "grid-cols-1",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(2, minmax(0, 1fr));
        /// }
        /// ```
        _2 => "grid-cols-2",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(3, minmax(0, 1fr));
        /// }
        /// ```
        _3 => "grid-cols-3",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(4, minmax(0, 1fr));
        /// }
        /// ```
        _4 => "grid-cols-4",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(5, minmax(0, 1fr));
        /// }
        /// ```
        _5 => "grid-cols-5",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(6, minmax(0, 1fr));
        /// }
        /// ```
        _6 => "grid-cols-6",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(7, minmax(0, 1fr));
        /// }
        /// ```
        _7 => "grid-cols-7",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(8, minmax(0, 1fr));
        /// }
        /// ```
        _8 => "grid-cols-8",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(9, minmax(0, 1fr));
        /// }
        /// ```
        _9 => "grid-cols-9",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(10, minmax(0, 1fr));
        /// }
        /// ```
        _10 => "grid-cols-10",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(11, minmax(0, 1fr));
        /// }
        /// ```
        _11 => "grid-cols-11",
        /// ```css
        /// {
        ///     grid-template-columns: repeat(12, minmax(0, 1fr));
        /// }
        /// ```
        _12 => "grid-cols-12",
        /// ```css
        /// {
        ///     grid-template-columns: none;
        /// }
        /// ```
        None => "grid-cols-none",
        /// ```css
        /// {
        ///     grid-template-columns: subgrid;
        /// }
        /// ```
        Subgrid => "grid-cols-subgrid",
    }
    /// Utilities for controlling how elements are sized and placed across grid columns.
    ///
    /// <https://tailwindcss.com/docs/grid-column>
    GridColumnStartEnd {
        /// ```css
        /// {
        ///     grid-column: auto;
        /// }
        /// ```
        Auto => "col-auto",
        /// ```css
        /// {
        ///     grid-column: span 1 / span 1;
        /// }
        /// ```
        Span1 => "col-span-1",
        /// ```css
        /// {
        ///     grid-column: span 2 / span 2;
        /// }
        /// ```
        Span2 => "col-span-2",
        /// ```css
        /// {
        ///     grid-column: span 3 / span 3;
        /// }
        /// ```
        Span3 => "col-span-3",
        /// ```css
        /// {
        ///     grid-column: span 4 / span 4;
        /// }
        /// ```
        Span4 => "col-span-4",
        /// ```css
        /// {
        ///     grid-column: span 5 / span 5;
        /// }
        /// ```
        Span5 => "col-span-5",
        /// ```css
        /// {
        ///     grid-column: span 6 / span 6;
        /// }
        /// ```
        Span6 => "col-span-6",
        /// ```css
        /// {
        ///     grid-column: span 7 / span 7;
        /// }
        /// ```
        Span7 => "col-span-7",
        /// ```css
        /// {
        ///     grid-column: span 8 / span 8;
        /// }
        /// ```
        Span8 => "col-span-8",
        /// ```css
        /// {
        ///     grid-column: span 9 / span 9;
        /// }
        /// ```
        Span9 => "col-span-9",
        /// ```css
        /// {
        ///     grid-column: span 10 / span 10;
        /// }
        /// ```
        Span10 => "col-span-10",
        /// ```css
        /// {
        ///     grid-column: span 11 / span 11;
        /// }
        /// ```
        Span11 => "col-span-11",
        /// ```css
        /// {
        ///     grid-column: span 12 / span 12;
        /// }
        /// ```
        Span12 => "col-span-12",
        /// ```css
        /// {
        ///     grid-column: 1 / -1;
        /// }
        /// ```
        SpanFull => "col-span-full",
        /// ```css
        /// {
        ///     grid-column-start: 1;
        /// }
        /// ```
        Start1 => "col-start-1",
        /// ```css
        /// {
        ///     grid-column-start: 2;
        /// }
        /// ```
        Start2 => "col-start-2",
        /// ```css
        /// {
        ///     grid-column-start: 3;
        /// }
        /// ```
        Start3 => "col-start-3",
        /// ```css
        /// {
        ///     grid-column-start: 4;
        /// }
        /// ```
        Start4 => "col-start-4",
        /// ```css
        /// {
        ///     grid-column-start: 5;
        /// }
        /// ```
        Start5 => "col-start-5",
        /// ```css
        /// {
        ///     grid-column-start: 6;
        /// }
        /// ```
        Start6 => "col-start-6",
        /// ```css
        /// {
        ///     grid-column-start: 7;
        /// }
        /// ```
        Start7 => "col-start-7",
        /// ```css
        /// {
        ///     grid-column-start: 8;
        /// }
        /// ```
        Start8 => "col-start-8",
        /// ```css
        /// {
        ///     grid-column-start: 9;
        /// }
        /// ```
        Start9 => "col-start-9",
        /// ```css
        /// {
        ///     grid-column-start: 10;
        /// }
        /// ```
        Start10 => "col-start-10",
        /// ```css
        /// {
        ///     grid-column-start: 11;
        /// }
        /// ```
        Start11 => "col-start-11",
        /// ```css
        /// {
        ///     grid-column-start: 12;
        /// }
        /// ```
        Start12 => "col-start-12",
        /// ```css
        /// {
        ///     grid-column-start: 13;
        /// }
        /// ```
        Start13 => "col-start-13",
        /// ```css
        /// {
        ///     grid-column-start: auto;
        /// }
        /// ```
        StartAuto => "col-start-auto",
        /// ```css
        /// {
        ///     grid-column-end: 1;
        /// }
        /// ```
        End1 => "col-end-1",
        /// ```css
        /// {
        ///     grid-column-end: 2;
        /// }
        /// ```
        End2 => "col-end-2",
        /// ```css
        /// {
        ///     grid-column-end: 3;
        /// }
        /// ```
        End3 => "col-end-3",
        /// ```css
        /// {
        ///     grid-column-end: 4;
        /// }
        /// ```
        End4 => "col-end-4",
        /// ```css
        /// {
        ///     grid-column-end: 5;
        /// }
        /// ```
        End5 => "col-end-5",
        /// ```css
        /// {
        ///     grid-column-end: 6;
        /// }
        /// ```
        End6 => "col-end-6",
        /// ```css
        /// {
        ///     grid-column-end: 7;
        /// }
        /// ```
        End7 => "col-end-7",
        /// ```css
        /// {
        ///     grid-column-end: 8;
        /// }
        /// ```
        End8 => "col-end-8",
        /// ```css
        /// {
        ///     grid-column-end: 9;
        /// }
        /// ```
        End9 => "col-end-9",
        /// ```css
        /// {
        ///     grid-column-end: 10;
        /// }
        /// ```
        End10 => "col-end-10",
        /// ```css
        /// {
        ///     grid-column-end: 11;
        /// }
        /// ```
        End11 => "col-end-11",
        /// ```css
        /// {
        ///     grid-column-end: 12;
        /// }
        /// ```
        End12 => "col-end-12",
        /// ```css
        /// {
        ///     grid-column-end: 13;
        /// }
        /// ```
        End13 => "col-end-13",
        /// ```css
        /// {
        ///     grid-column-end: auto;
        /// }
        /// ```
        EndAuto => "col-end-auto",
    }
    /// Utilities for specifying the rows in a grid layout.
    ///
    /// <https://tailwindcss.com/docs/grid-template-rows>
    GridTemplateRows {
        /// ```css
        /// {
        ///     grid-template-rows: repeat(1, minmax(0, 1fr));
        /// }
        /// ```
        _1 => "grid-rows-1",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(2, minmax(0, 1fr));
        /// }
        /// ```
        _2 => "grid-rows-2",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(3, minmax(0, 1fr));
        /// }
        /// ```
        _3 => "grid-rows-3",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(4, minmax(0, 1fr));
        /// }
        /// ```
        _4 => "grid-rows-4",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(5, minmax(0, 1fr));
        /// }
        /// ```
        _5 => "grid-rows-5",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(6, minmax(0, 1fr));
        /// }
        /// ```
        _6 => "grid-rows-6",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(7, minmax(0, 1fr));
        /// }
        /// ```
        _7 => "grid-rows-7",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(8, minmax(0, 1fr));
        /// }
        /// ```
        _8 => "grid-rows-8",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(9, minmax(0, 1fr));
        /// }
        /// ```
        _9 => "grid-rows-9",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(10, minmax(0, 1fr));
        /// }
        /// ```
        _10 => "grid-rows-10",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(11, minmax(0, 1fr));
        /// }
        /// ```
        _11 => "grid-rows-11",
        /// ```css
        /// {
        ///     grid-template-rows: repeat(12, minmax(0, 1fr));
        /// }
        /// ```
        _12 => "grid-rows-12",
        /// ```css
        /// {
        ///     grid-template-rows: none;
        /// }
        /// ```
        None => "grid-rows-none",
        /// ```css
        /// {
        ///     grid-template-rows: subgrid;
        /// }
        /// ```
        Subgrid => "grid-rows-subgrid",
    }
    /// Utilities for controlling how elements are sized and placed across grid rows.
    ///
    /// <https://tailwindcss.com/docs/grid-row>
    GridRowStartEnd {
        /// ```css
        /// {
        ///     grid-row: auto;
        /// }
        /// ```
        Auto => "row-auto",
        /// ```css
        /// {
        ///     grid-row: span 1 / span 1;
        /// }
        /// ```
        Span1 => "row-span-1",
        /// ```css
        /// {
        ///     grid-row: span 2 / span 2;
        /// }
        /// ```
        Span2 => "row-span-2",
        /// ```css
        /// {
        ///     grid-row: span 3 / span 3;
        /// }
        /// ```
        Span3 => "row-span-3",
        /// ```css
        /// {
        ///     grid-row: span 4 / span 4;
        /// }
        /// ```
        Span4 => "row-span-4",
        /// ```css
        /// {
        ///     grid-row: span 5 / span 5;
        /// }
        /// ```
        Span5 => "row-span-5",
        /// ```css
        /// {
        ///     grid-row: span 6 / span 6;
        /// }
        /// ```
        Span6 => "row-span-6",
        /// ```css
        /// {
        ///     grid-row: span 7 / span 7;
        /// }
        /// ```
        Span7 => "row-span-7",
        /// ```css
        /// {
        ///     grid-row: span 8 / span 8;
        /// }
        /// ```
        Span8 => "row-span-8",
        /// ```css
        /// {
        ///     grid-row: span 9 / span 9;
        /// }
        /// ```
        Span9 => "row-span-9",
        /// ```css
        /// {
        ///     grid-row: span 10 / span 10;
        /// }
        /// ```
        Span10 => "row-span-10",
        /// ```css
        /// {
        ///     grid-row: span 11 / span 11;
        /// }
        /// ```
        Span11 => "row-span-11",
        /// ```css
        /// {
        ///     grid-row: span 12 / span 12;
        /// }
        /// ```
        Span12 => "row-span-12",
        /// ```css
        /// {
        ///     grid-row: 1 / -1;
        /// }
        /// ```
        SpanFull => "row-span-full",
        /// ```css
        /// {
        ///     grid-row-start: 1;
        /// }
        /// ```
        Start1 => "row-start-1",
        /// ```css
        /// {
        ///     grid-row-start: 2;
        /// }
        /// ```
        Start2 => "row-start-2",
        /// ```css
        /// {
        ///     grid-row-start: 3;
        /// }
        /// ```
        Start3 => "row-start-3",
        /// ```css
        /// {
        ///     grid-row-start: 4;
        /// }
        /// ```
        Start4 => "row-start-4",
        /// ```css
        /// {
        ///     grid-row-start: 5;
        /// }
        /// ```
        Start5 => "row-start-5",
        /// ```css
        /// {
        ///     grid-row-start: 6;
        /// }
        /// ```
        Start6 => "row-start-6",
        /// ```css
        /// {
        ///     grid-row-start: 7;
        /// }
        /// ```
        Start7 => "row-start-7",
        /// ```css
        /// {
        ///     grid-row-start: 8;
        /// }
        /// ```
        Start8 => "row-start-8",
        /// ```css
        /// {
        ///     grid-row-start: 9;
        /// }
        /// ```
        Start9 => "row-start-9",
        /// ```css
        /// {
        ///     grid-row-start: 10;
        /// }
        /// ```
        Start10 => "row-start-10",
        /// ```css
        /// {
        ///     grid-row-start: 11;
        /// }
        /// ```
        Start11 => "row-start-11",
        /// ```css
        /// {
        ///     grid-row-start: 12;
        /// }
        /// ```
        Start12 => "row-start-12",
        /// ```css
        /// {
        ///     grid-row-start: 13;
        /// }
        /// ```
        Start13 => "row-start-13",
        /// ```css
        /// {
        ///     grid-row-start: auto;
        /// }
        /// ```
        StartAuto => "row-start-auto",
        /// ```css
        /// {
        ///     grid-row-end: 1;
        /// }
        /// ```
        End1 => "row-end-1",
        /// ```css
        /// {
        ///     grid-row-end: 2;
        /// }
        /// ```
        End2 => "row-end-2",
        /// ```css
        /// {
        ///     grid-row-end: 3;
        /// }
        /// ```
        End3 => "row-end-3",
        /// ```css
        /// {
        ///     grid-row-end: 4;
        /// }
        /// ```
        End4 => "row-end-4",
        /// ```css
        /// {
        ///     grid-row-end: 5;
        /// }
        /// ```
        End5 => "row-end-5",
        /// ```css
        /// {
        ///     grid-row-end: 6;
        /// }
        /// ```
        End6 => "row-end-6",
        /// ```css
        /// {
        ///     grid-row-end: 7;
        /// }
        /// ```
        End7 => "row-end-7",
        /// ```css
        /// {
        ///     grid-row-end: 8;
        /// }
        /// ```
        End8 => "row-end-8",
        /// ```css
        /// {
        ///     grid-row-end: 9;
        /// }
        /// ```
        End9 => "row-end-9",
        /// ```css
        /// {
        ///     grid-row-end: 10;
        /// }
        /// ```
        End10 => "row-end-10",
        /// ```css
        /// {
        ///     grid-row-end: 11;
        /// }
        /// ```
        End11 => "row-end-11",
        /// ```css
        /// {
        ///     grid-row-end: 12;
        /// }
        /// ```
        End12 => "row-end-12",
        /// ```css
        /// {
        ///     grid-row-end: 13;
        /// }
        /// ```
        End13 => "row-end-13",
        /// ```css
        /// {
        ///     grid-row-end: auto;
        /// }
        /// ```
        EndAuto => "row-end-auto",
    }
    /// Utilities for controlling how elements in a grid are auto-placed.
    ///
    /// <https://tailwindcss.com/docs/grid-auto-flow>
    GridAutoFlow {
        /// ```css
        /// {
        ///     grid-auto-flow: row;
        /// }
        /// ```
        Row => "grid-flow-row",
        /// ```css
        /// {
        ///     grid-auto-flow: column;
        /// }
        /// ```
        Col => "grid-flow-col",
        /// ```css
        /// {
        ///     grid-auto-flow: dense;
        /// }
        /// ```
        Dense => "grid-flow-dense",
        /// ```css
        /// {
        ///     grid-auto-flow: row dense;
        /// }
        /// ```
        RowDense => "grid-flow-row-dense",
        /// ```css
        /// {
        ///     grid-auto-flow: column dense;
        /// }
        /// ```
        ColDense => "grid-flow-col-dense",
    }
    /// Utilities for controlling the size of implicitly-created grid columns.
    ///
    /// <https://tailwindcss.com/docs/grid-auto-columns>
    GridAutoColumns {
        /// ```css
        /// {
        ///     grid-auto-columns: auto;
        /// }
        /// ```
        Auto => "auto-cols-auto",
        /// ```css
        /// {
        ///     grid-auto-columns: min-content;
        /// }
        /// ```
        Min => "auto-cols-min",
        /// ```css
        /// {
        ///     grid-auto-columns: max-content;
        /// }
        /// ```
        Max => "auto-cols-max",
        /// ```css
        /// {
        ///     grid-auto-columns: minmax(0, 1fr);
        /// }
        /// ```
        Fr => "auto-cols-fr",
    }
    /// Utilities for controlling the size of implicitly-created grid rows.
    ///
    /// <https://tailwindcss.com/docs/grid-auto-rows>
    GridAutoRows {
        /// ```css
        /// {
        ///     grid-auto-rows: auto;
        /// }
        /// ```
        Auto => "auto-rows-auto",
        /// ```css
        /// {
        ///     grid-auto-rows: min-content;
        /// }
        /// ```
        Min => "auto-rows-min",
        /// ```css
        /// {
        ///     grid-auto-rows: max-content;
        /// }
        /// ```
        Max => "auto-rows-max",
        /// ```css
        /// {
        ///     grid-auto-rows: minmax(0, 1fr);
        /// }
        /// ```
        Fr => "auto-rows-fr",
    }
    /// Utilities for controlling gutters between grid and flexbox items.
    ///
    /// <https://tailwindcss.com/docs/gap>
    Gap {
        /// ```css
        /// {
        ///     gap: 0px;
        /// }
        /// ```
        _0 => "gap-0",
        /// ```css
        /// {
        ///     column-gap: 0px;
        /// }
        /// ```
        X0 => "gap-x-0",
        /// ```css
        /// {
        ///     row-gap: 0px;
        /// }
        /// ```
        Y0 => "gap-y-0",
        /// ```css
        /// {
        ///     gap: 1px;
        /// }
        /// ```
        Px => "gap-px",
        /// ```css
        /// {
        ///     column-gap: 1px;
        /// }
        /// ```
        XPx => "gap-x-px",
        /// ```css
        /// {
        ///     row-gap: 1px;
        /// }
        /// ```
        YPx => "gap-y-px",
        /// ```css
        /// {
        ///     gap: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "gap-0.5",
        /// ```css
        /// {
        ///     column-gap: 0.125rem; /* 2px */
        /// }
        /// ```
        X0_5 => "gap-x-0.5",
        /// ```css
        /// {
        ///     row-gap: 0.125rem; /* 2px */
        /// }
        /// ```
        Y0_5 => "gap-y-0.5",
        /// ```css
        /// {
        ///     gap: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "gap-1",
        /// ```css
        /// {
        ///     column-gap: 0.25rem; /* 4px */
        /// }
        /// ```
        X1 => "gap-x-1",
        /// ```css
        /// {
        ///     row-gap: 0.25rem; /* 4px */
        /// }
        /// ```
        Y1 => "gap-y-1",
        /// ```css
        /// {
        ///     gap: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "gap-1.5",
        /// ```css
        /// {
        ///     column-gap: 0.375rem; /* 6px */
        /// }
        /// ```
        X1_5 => "gap-x-1.5",
        /// ```css
        /// {
        ///     row-gap: 0.375rem; /* 6px */
        /// }
        /// ```
        Y1_5 => "gap-y-1.5",
        /// ```css
        /// {
        ///     gap: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "gap-2",
        /// ```css
        /// {
        ///     column-gap: 0.5rem; /* 8px */
        /// }
        /// ```
        X2 => "gap-x-2",
        /// ```css
        /// {
        ///     row-gap: 0.5rem; /* 8px */
        /// }
        /// ```
        Y2 => "gap-y-2",
        /// ```css
        /// {
        ///     gap: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "gap-2.5",
        /// ```css
        /// {
        ///     column-gap: 0.625rem; /* 10px */
        /// }
        /// ```
        X2_5 => "gap-x-2.5",
        /// ```css
        /// {
        ///     row-gap: 0.625rem; /* 10px */
        /// }
        /// ```
        Y2_5 => "gap-y-2.5",
        /// ```css
        /// {
        ///     gap: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "gap-3",
        /// ```css
        /// {
        ///     column-gap: 0.75rem; /* 12px */
        /// }
        /// ```
        X3 => "gap-x-3",
        /// ```css
        /// {
        ///     row-gap: 0.75rem; /* 12px */
        /// }
        /// ```
        Y3 => "gap-y-3",
        /// ```css
        /// {
        ///     gap: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "gap-3.5",
        /// ```css
        /// {
        ///     column-gap: 0.875rem; /* 14px */
        /// }
        /// ```
        X3_5 => "gap-x-3.5",
        /// ```css
        /// {
        ///     row-gap: 0.875rem; /* 14px */
        /// }
        /// ```
        Y3_5 => "gap-y-3.5",
        /// ```css
        /// {
        ///     gap: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "gap-4",
        /// ```css
        /// {
        ///     column-gap: 1rem; /* 16px */
        /// }
        /// ```
        X4 => "gap-x-4",
        /// ```css
        /// {
        ///     row-gap: 1rem; /* 16px */
        /// }
        /// ```
        Y4 => "gap-y-4",
        /// ```css
        /// {
        ///     gap: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "gap-5",
        /// ```css
        /// {
        ///     column-gap: 1.25rem; /* 20px */
        /// }
        /// ```
        X5 => "gap-x-5",
        /// ```css
        /// {
        ///     row-gap: 1.25rem; /* 20px */
        /// }
        /// ```
        Y5 => "gap-y-5",
        /// ```css
        /// {
        ///     gap: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "gap-6",
        /// ```css
        /// {
        ///     column-gap: 1.5rem; /* 24px */
        /// }
        /// ```
        X6 => "gap-x-6",
        /// ```css
        /// {
        ///     row-gap: 1.5rem; /* 24px */
        /// }
        /// ```
        Y6 => "gap-y-6",
        /// ```css
        /// {
        ///     gap: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "gap-7",
        /// ```css
        /// {
        ///     column-gap: 1.75rem; /* 28px */
        /// }
        /// ```
        X7 => "gap-x-7",
        /// ```css
        /// {
        ///     row-gap: 1.75rem; /* 28px */
        /// }
        /// ```
        Y7 => "gap-y-7",
        /// ```css
        /// {
        ///     gap: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "gap-8",
        /// ```css
        /// {
        ///     column-gap: 2rem; /* 32px */
        /// }
        /// ```
        X8 => "gap-x-8",
        /// ```css
        /// {
        ///     row-gap: 2rem; /* 32px */
        /// }
        /// ```
        Y8 => "gap-y-8",
        /// ```css
        /// {
        ///     gap: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "gap-9",
        /// ```css
        /// {
        ///     column-gap: 2.25rem; /* 36px */
        /// }
        /// ```
        X9 => "gap-x-9",
        /// ```css
        /// {
        ///     row-gap: 2.25rem; /* 36px */
        /// }
        /// ```
        Y9 => "gap-y-9",
        /// ```css
        /// {
        ///     gap: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "gap-10",
        /// ```css
        /// {
        ///     column-gap: 2.5rem; /* 40px */
        /// }
        /// ```
        X10 => "gap-x-10",
        /// ```css
        /// {
        ///     row-gap: 2.5rem; /* 40px */
        /// }
        /// ```
        Y10 => "gap-y-10",
        /// ```css
        /// {
        ///     gap: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "gap-11",
        /// ```css
        /// {
        ///     column-gap: 2.75rem; /* 44px */
        /// }
        /// ```
        X11 => "gap-x-11",
        /// ```css
        /// {
        ///     row-gap: 2.75rem; /* 44px */
        /// }
        /// ```
        Y11 => "gap-y-11",
        /// ```css
        /// {
        ///     gap: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "gap-12",
        /// ```css
        /// {
        ///     column-gap: 3rem; /* 48px */
        /// }
        /// ```
        X12 => "gap-x-12",
        /// ```css
        /// {
        ///     row-gap: 3rem; /* 48px */
        /// }
        /// ```
        Y12 => "gap-y-12",
        /// ```css
        /// {
        ///     gap: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "gap-14",
        /// ```css
        /// {
        ///     column-gap: 3.5rem; /* 56px */
        /// }
        /// ```
        X14 => "gap-x-14",
        /// ```css
        /// {
        ///     row-gap: 3.5rem; /* 56px */
        /// }
        /// ```
        Y14 => "gap-y-14",
        /// ```css
        /// {
        ///     gap: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "gap-16",
        /// ```css
        /// {
        ///     column-gap: 4rem; /* 64px */
        /// }
        /// ```
        X16 => "gap-x-16",
        /// ```css
        /// {
        ///     row-gap: 4rem; /* 64px */
        /// }
        /// ```
        Y16 => "gap-y-16",
        /// ```css
        /// {
        ///     gap: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "gap-20",
        /// ```css
        /// {
        ///     column-gap: 5rem; /* 80px */
        /// }
        /// ```
        X20 => "gap-x-20",
        /// ```css
        /// {
        ///     row-gap: 5rem; /* 80px */
        /// }
        /// ```
        Y20 => "gap-y-20",
        /// ```css
        /// {
        ///     gap: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "gap-24",
        /// ```css
        /// {
        ///     column-gap: 6rem; /* 96px */
        /// }
        /// ```
        X24 => "gap-x-24",
        /// ```css
        /// {
        ///     row-gap: 6rem; /* 96px */
        /// }
        /// ```
        Y24 => "gap-y-24",
        /// ```css
        /// {
        ///     gap: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "gap-28",
        /// ```css
        /// {
        ///     column-gap: 7rem; /* 112px */
        /// }
        /// ```
        X28 => "gap-x-28",
        /// ```css
        /// {
        ///     row-gap: 7rem; /* 112px */
        /// }
        /// ```
        Y28 => "gap-y-28",
        /// ```css
        /// {
        ///     gap: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "gap-32",
        /// ```css
        /// {
        ///     column-gap: 8rem; /* 128px */
        /// }
        /// ```
        X32 => "gap-x-32",
        /// ```css
        /// {
        ///     row-gap: 8rem; /* 128px */
        /// }
        /// ```
        Y32 => "gap-y-32",
        /// ```css
        /// {
        ///     gap: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "gap-36",
        /// ```css
        /// {
        ///     column-gap: 9rem; /* 144px */
        /// }
        /// ```
        X36 => "gap-x-36",
        /// ```css
        /// {
        ///     row-gap: 9rem; /* 144px */
        /// }
        /// ```
        Y36 => "gap-y-36",
        /// ```css
        /// {
        ///     gap: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "gap-40",
        /// ```css
        /// {
        ///     column-gap: 10rem; /* 160px */
        /// }
        /// ```
        X40 => "gap-x-40",
        /// ```css
        /// {
        ///     row-gap: 10rem; /* 160px */
        /// }
        /// ```
        Y40 => "gap-y-40",
        /// ```css
        /// {
        ///     gap: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "gap-44",
        /// ```css
        /// {
        ///     column-gap: 11rem; /* 176px */
        /// }
        /// ```
        X44 => "gap-x-44",
        /// ```css
        /// {
        ///     row-gap: 11rem; /* 176px */
        /// }
        /// ```
        Y44 => "gap-y-44",
        /// ```css
        /// {
        ///     gap: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "gap-48",
        /// ```css
        /// {
        ///     column-gap: 12rem; /* 192px */
        /// }
        /// ```
        X48 => "gap-x-48",
        /// ```css
        /// {
        ///     row-gap: 12rem; /* 192px */
        /// }
        /// ```
        Y48 => "gap-y-48",
        /// ```css
        /// {
        ///     gap: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "gap-52",
        /// ```css
        /// {
        ///     column-gap: 13rem; /* 208px */
        /// }
        /// ```
        X52 => "gap-x-52",
        /// ```css
        /// {
        ///     row-gap: 13rem; /* 208px */
        /// }
        /// ```
        Y52 => "gap-y-52",
        /// ```css
        /// {
        ///     gap: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "gap-56",
        /// ```css
        /// {
        ///     column-gap: 14rem; /* 224px */
        /// }
        /// ```
        X56 => "gap-x-56",
        /// ```css
        /// {
        ///     row-gap: 14rem; /* 224px */
        /// }
        /// ```
        Y56 => "gap-y-56",
        /// ```css
        /// {
        ///     gap: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "gap-60",
        /// ```css
        /// {
        ///     column-gap: 15rem; /* 240px */
        /// }
        /// ```
        X60 => "gap-x-60",
        /// ```css
        /// {
        ///     row-gap: 15rem; /* 240px */
        /// }
        /// ```
        Y60 => "gap-y-60",
        /// ```css
        /// {
        ///     gap: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "gap-64",
        /// ```css
        /// {
        ///     column-gap: 16rem; /* 256px */
        /// }
        /// ```
        X64 => "gap-x-64",
        /// ```css
        /// {
        ///     row-gap: 16rem; /* 256px */
        /// }
        /// ```
        Y64 => "gap-y-64",
        /// ```css
        /// {
        ///     gap: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "gap-72",
        /// ```css
        /// {
        ///     column-gap: 18rem; /* 288px */
        /// }
        /// ```
        X72 => "gap-x-72",
        /// ```css
        /// {
        ///     row-gap: 18rem; /* 288px */
        /// }
        /// ```
        Y72 => "gap-y-72",
        /// ```css
        /// {
        ///     gap: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "gap-80",
        /// ```css
        /// {
        ///     column-gap: 20rem; /* 320px */
        /// }
        /// ```
        X80 => "gap-x-80",
        /// ```css
        /// {
        ///     row-gap: 20rem; /* 320px */
        /// }
        /// ```
        Y80 => "gap-y-80",
        /// ```css
        /// {
        ///     gap: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "gap-96",
        /// ```css
        /// {
        ///     column-gap: 24rem; /* 384px */
        /// }
        /// ```
        X96 => "gap-x-96",
        /// ```css
        /// {
        ///     row-gap: 24rem; /* 384px */
        /// }
        /// ```
        Y96 => "gap-y-96",
    }
    /// Utilities for controlling how flex and grid items are positioned along a container's main axis.
    ///
    /// <https://tailwindcss.com/docs/justify-content>
    JustifyContent {
        /// ```css
        /// {
        ///     justify-content: normal;
        /// }
        /// ```
        Normal => "justify-normal",
        /// ```css
        /// {
        ///     justify-content: flex-start;
        /// }
        /// ```
        Start => "justify-start",
        /// ```css
        /// {
        ///     justify-content: flex-end;
        /// }
        /// ```
        End => "justify-end",
        /// ```css
        /// {
        ///     justify-content: center;
        /// }
        /// ```
        Center => "justify-center",
        /// ```css
        /// {
        ///     justify-content: space-between;
        /// }
        /// ```
        Between => "justify-between",
        /// ```css
        /// {
        ///     justify-content: space-around;
        /// }
        /// ```
        Around => "justify-around",
        /// ```css
        /// {
        ///     justify-content: space-evenly;
        /// }
        /// ```
        Evenly => "justify-evenly",
        /// ```css
        /// {
        ///     justify-content: stretch;
        /// }
        /// ```
        Stretch => "justify-stretch",
    }
    /// Utilities for controlling how grid items are aligned along their inline axis.
    ///
    /// <https://tailwindcss.com/docs/justify-items>
    JustifyItems {
        /// ```css
        /// {
        ///     justify-items: start;
        /// }
        /// ```
        Start => "justify-items-start",
        /// ```css
        /// {
        ///     justify-items: end;
        /// }
        /// ```
        End => "justify-items-end",
        /// ```css
        /// {
        ///     justify-items: center;
        /// }
        /// ```
        Center => "justify-items-center",
        /// ```css
        /// {
        ///     justify-items: stretch;
        /// }
        /// ```
        Stretch => "justify-items-stretch",
    }
    /// Utilities for controlling how an individual grid item is aligned along its inline axis.
    ///
    /// <https://tailwindcss.com/docs/justify-self>
    JustifySelf {
        /// ```css
        /// {
        ///     justify-self: auto;
        /// }
        /// ```
        Auto => "justify-self-auto",
        /// ```css
        /// {
        ///     justify-self: start;
        /// }
        /// ```
        Start => "justify-self-start",
        /// ```css
        /// {
        ///     justify-self: end;
        /// }
        /// ```
        End => "justify-self-end",
        /// ```css
        /// {
        ///     justify-self: center;
        /// }
        /// ```
        Center => "justify-self-center",
        /// ```css
        /// {
        ///     justify-self: stretch;
        /// }
        /// ```
        Stretch => "justify-self-stretch",
    }
    /// Utilities for controlling how rows are positioned in multi-row flex and grid containers.
    ///
    /// <https://tailwindcss.com/docs/align-content>
    AlignContent {
        /// ```css
        /// {
        ///     align-content: normal;
        /// }
        /// ```
        Normal => "content-normal",
        /// ```css
        /// {
        ///     align-content: center;
        /// }
        /// ```
        Center => "content-center",
        /// ```css
        /// {
        ///     align-content: flex-start;
        /// }
        /// ```
        Start => "content-start",
        /// ```css
        /// {
        ///     align-content: flex-end;
        /// }
        /// ```
        End => "content-end",
        /// ```css
        /// {
        ///     align-content: space-between;
        /// }
        /// ```
        Between => "content-between",
        /// ```css
        /// {
        ///     align-content: space-around;
        /// }
        /// ```
        Around => "content-around",
        /// ```css
        /// {
        ///     align-content: space-evenly;
        /// }
        /// ```
        Evenly => "content-evenly",
        /// ```css
        /// {
        ///     align-content: baseline;
        /// }
        /// ```
        Baseline => "content-baseline",
        /// ```css
        /// {
        ///     align-content: stretch;
        /// }
        /// ```
        Stretch => "content-stretch",
    }
    /// Utilities for controlling how flex and grid items are positioned along a container's cross axis.
    ///
    /// <https://tailwindcss.com/docs/align-items>
    AlignItems {
        /// ```css
        /// {
        ///     align-items: flex-start;
        /// }
        /// ```
        Start => "items-start",
        /// ```css
        /// {
        ///     align-items: flex-end;
        /// }
        /// ```
        End => "items-end",
        /// ```css
        /// {
        ///     align-items: center;
        /// }
        /// ```
        Center => "items-center",
        /// ```css
        /// {
        ///     align-items: baseline;
        /// }
        /// ```
        Baseline => "items-baseline",
        /// ```css
        /// {
        ///     align-items: stretch;
        /// }
        /// ```
        Stretch => "items-stretch",
    }
    /// Utilities for controlling how an individual flex or grid item is positioned along its container's cross axis.
    ///
    /// <https://tailwindcss.com/docs/align-self>
    AlignSelf {
        /// ```css
        /// {
        ///     align-self: auto;
        /// }
        /// ```
        Auto => "self-auto",
        /// ```css
        /// {
        ///     align-self: flex-start;
        /// }
        /// ```
        Start => "self-start",
        /// ```css
        /// {
        ///     align-self: flex-end;
        /// }
        /// ```
        End => "self-end",
        /// ```css
        /// {
        ///     align-self: center;
        /// }
        /// ```
        Center => "self-center",
        /// ```css
        /// {
        ///     align-self: stretch;
        /// }
        /// ```
        Stretch => "self-stretch",
        /// ```css
        /// {
        ///     align-self: baseline;
        /// }
        /// ```
        Baseline => "self-baseline",
    }
    /// Utilities for controlling how content is justified and aligned at the same time.
    ///
    /// <https://tailwindcss.com/docs/place-content>
    PlaceContent {
        /// ```css
        /// {
        ///     place-content: center;
        /// }
        /// ```
        Center => "place-content-center",
        /// ```css
        /// {
        ///     place-content: start;
        /// }
        /// ```
        Start => "place-content-start",
        /// ```css
        /// {
        ///     place-content: end;
        /// }
        /// ```
        End => "place-content-end",
        /// ```css
        /// {
        ///     place-content: space-between;
        /// }
        /// ```
        Between => "place-content-between",
        /// ```css
        /// {
        ///     place-content: space-around;
        /// }
        /// ```
        Around => "place-content-around",
        /// ```css
        /// {
        ///     place-content: space-evenly;
        /// }
        /// ```
        Evenly => "place-content-evenly",
        /// ```css
        /// {
        ///     place-content: baseline;
        /// }
        /// ```
        Baseline => "place-content-baseline",
        /// ```css
        /// {
        ///     place-content: stretch;
        /// }
        /// ```
        Stretch => "place-content-stretch",
    }
    /// Utilities for controlling how items are justified and aligned at the same time.
    ///
    /// <https://tailwindcss.com/docs/place-items>
    PlaceItems {
        /// ```css
        /// {
        ///     place-items: start;
        /// }
        /// ```
        Start => "place-items-start",
        /// ```css
        /// {
        ///     place-items: end;
        /// }
        /// ```
        End => "place-items-end",
        /// ```css
        /// {
        ///     place-items: center;
        /// }
        /// ```
        Center => "place-items-center",
        /// ```css
        /// {
        ///     place-items: baseline;
        /// }
        /// ```
        Baseline => "place-items-baseline",
        /// ```css
        /// {
        ///     place-items: stretch;
        /// }
        /// ```
        Stretch => "place-items-stretch",
    }
    /// Utilities for controlling how an individual item is justified and aligned at the same time.
    ///
    /// <https://tailwindcss.com/docs/place-self>
    PlaceSelf {
        /// ```css
        /// {
        ///     place-self: auto;
        /// }
        /// ```
        Auto => "place-self-auto",
        /// ```css
        /// {
        ///     place-self: start;
        /// }
        /// ```
        Start => "place-self-start",
        /// ```css
        /// {
        ///     place-self: end;
        /// }
        /// ```
        End => "place-self-end",
        /// ```css
        /// {
        ///     place-self: center;
        /// }
        /// ```
        Center => "place-self-center",
        /// ```css
        /// {
        ///     place-self: stretch;
        /// }
        /// ```
        Stretch => "place-self-stretch",
    }
);
