def_types!(
    /// Utilities for controlling the aspect ratio of an element.
    ///
    /// <https://tailwindcss.com/docs/aspect-ratio>
    AspectRatio {
        /// ```css
        /// {
        ///     aspect-ratio: auto;
        /// }
        /// ```
        Auto => "aspect-auto",
        /// ```css
        /// {
        ///     aspect-ratio: 1 / 1;
        /// }
        /// ```
        Square => "aspect-square",
        /// ```css
        /// {
        ///     aspect-ratio: 16 / 9;
        /// }
        /// ```
        Video => "aspect-video",
    }
    /// A component for fixing an element's width to the current breakpoint.
    ///
    /// <https://tailwindcss.com/docs/container>
    Container {
        /// | Breakpoint     | Properties             |
        /// |----------------|------------------------|
        /// | `None`         | `width: 100%;`         |
        /// | `sm (640px)`   | `max-width: 640px;`    |
        /// | `md (768px)`   | `max-width: 768px;`    |
        /// | `lg (1024px)`  | `max-width: 1024px;`   |
        /// | `xl (1280px)`  | `max-width: 1280px;`   |
        /// | `2xl (1536px)` | `max-width: 1536px;`   |
        Container => "container",
    }
    /// Utilities for controlling the number of columns within an element.
    ///
    /// <https://tailwindcss.com/docs/columns>
    Columns {
        /// ```css
        /// {
        ///     columns: 1;
        /// }
        /// ```
        _1 => "columns-1",
        /// ```css
        /// {
        ///     columns: 2;
        /// }
        /// ```
        _2 => "columns-2",
        /// ```css
        /// {
        ///     columns: 3;
        /// }
        /// ```
        _3 => "columns-3",
        /// ```css
        /// {
        ///     columns: 4;
        /// }
        /// ```
        _4 => "columns-4",
        /// ```css
        /// {
        ///     columns: 5;
        /// }
        /// ```
        _5 => "columns-5",
        /// ```css
        /// {
        ///     columns: 6;
        /// }
        /// ```
        _6 => "columns-6",
        /// ```css
        /// {
        ///     columns: 7;
        /// }
        /// ```
        _7 => "columns-7",
        /// ```css
        /// {
        ///     columns: 8;
        /// }
        /// ```
        _8 => "columns-8",
        /// ```css
        /// {
        ///     columns: 9;
        /// }
        /// ```
        _9 => "columns-9",
        /// ```css
        /// {
        ///     columns: 10;
        /// }
        /// ```
        _10 => "columns-10",
        /// ```css
        /// {
        ///     columns: 11;
        /// }
        /// ```
        _11 => "columns-11",
        /// ```css
        /// {
        ///     columns: 12;
        /// }
        /// ```
        _12 => "columns-12",
        /// ```css
        /// {
        ///     columns: auto;
        /// }
        /// ```
        Auto => "columns-auto",
        /// ```css
        /// {
        ///     columns: 16rem; /* 256px */
        /// }
        /// ```
        _3xs => "columns-3xs",
        /// ```css
        /// {
        ///     columns: 18rem; /* 288px */
        /// }
        /// ```
        _2xs => "columns-2xs",
        /// ```css
        /// {
        ///     columns: 20rem; /* 320px */
        /// }
        /// ```
        Xs => "columns-xs",
        /// ```css
        /// {
        ///     columns: 24rem; /* 384px */
        /// }
        /// ```
        Sm => "columns-sm",
        /// ```css
        /// {
        ///     columns: 28rem; /* 448px */
        /// }
        /// ```
        Md => "columns-md",
        /// ```css
        /// {
        ///     columns: 32rem; /* 512px */
        /// }
        /// ```
        Lg => "columns-lg",
        /// ```css
        /// {
        ///     columns: 36rem; /* 576px */
        /// }
        /// ```
        Xl => "columns-xl",
        /// ```css
        /// {
        ///     columns: 42rem; /* 672px */
        /// }
        /// ```
        _2xl => "columns-2xl",
        /// ```css
        /// {
        ///     columns: 48rem; /* 768px */
        /// }
        /// ```
        _3xl => "columns-3xl",
        /// ```css
        /// {
        ///     columns: 56rem; /* 896px */
        /// }
        /// ```
        _4xl => "columns-4xl",
        /// ```css
        /// {
        ///     columns: 64rem; /* 1024px */
        /// }
        /// ```
        _5xl => "columns-5xl",
        /// ```css
        /// {
        ///     columns: 72rem; /* 1152px */
        /// }
        /// ```
        _6xl => "columns-6xl",
        /// ```css
        /// {
        ///     columns: 80rem; /* 1280px */
        /// }
        /// ```
        _7xl => "columns-7xl",
    }
    /// Utilities for controlling how a column or page should break after an element.
    ///
    /// <https://tailwindcss.com/docs/break-after>
    BreakAfter {
        /// ```css
        /// {
        ///     break-after: auto;
        /// }
        /// ```
        Auto => "break-after-auto",
        /// ```css
        /// {
        ///     break-after: avoid;
        /// }
        /// ```
        Avoid => "break-after-avoid",
        /// ```css
        /// {
        ///     break-after: all;
        /// }
        /// ```
        All => "break-after-all",
        /// ```css
        /// {
        ///     break-after: avoid-page;
        /// }
        /// ```
        AvoidPage => "break-after-avoid-page",
        /// ```css
        /// {
        ///     break-after: page;
        /// }
        /// ```
        Page => "break-after-page",
        /// ```css
        /// {
        ///     break-after: left;
        /// }
        /// ```
        Left => "break-after-left",
        /// ```css
        /// {
        ///     break-after: right;
        /// }
        /// ```
        Right => "break-after-right",
        /// ```css
        /// {
        ///     break-after: column;
        /// }
        /// ```
        Column => "break-after-column",
    }
    /// Utilities for controlling how a column or page should break before an element.
    ///
    /// <https://tailwindcss.com/docs/break-before>
    BreakBefore {
        /// ```css
        /// {
        ///     break-before: auto;
        /// }
        /// ```
        Auto => "break-before-auto",
        /// ```css
        /// {
        ///     break-before: avoid;
        /// }
        /// ```
        Avoid => "break-before-avoid",
        /// ```css
        /// {
        ///     break-before: all;
        /// }
        /// ```
        All => "break-before-all",
        /// ```css
        /// {
        ///     break-before: avoid-page;
        /// }
        /// ```
        AvoidPage => "break-before-avoid-page",
        /// ```css
        /// {
        ///     break-before: page;
        /// }
        /// ```
        Page => "break-before-page",
        /// ```css
        /// {
        ///     break-before: left;
        /// }
        /// ```
        Left => "break-before-left",
        /// ```css
        /// {
        ///     break-before: right;
        /// }
        /// ```
        Right => "break-before-right",
        /// ```css
        /// {
        ///     break-before: column;
        /// }
        /// ```
        Column => "break-before-column",
    }
    /// Utilities for controlling how a column or page should break within an element.
    ///
    /// <https://tailwindcss.com/docs/break-inside>
    BreakInside {
        /// ```css
        /// {
        ///     break-inside: auto;
        /// }
        /// ```
        Auto => "break-inside-auto",
        /// ```css
        /// {
        ///     break-inside: avoid;
        /// }
        /// ```
        Avoid => "break-inside-avoid",
        /// ```css
        /// {
        ///     break-inside: avoid-page;
        /// }
        /// ```
        AvoidPage => "break-inside-avoid-page",
        /// ```css
        /// {
        ///     break-inside: avoid-column;
        /// }
        /// ```
        AvoidColumn => "break-inside-avoid-column",
    }
    /// Utilities for controlling how element fragments should be rendered across multiple lines, columns, or pages.
    ///
    /// <https://tailwindcss.com/docs/box-decoration-break>
    BoxDecorationBreak {
        /// ```css
        /// {
        ///     box-decoration-break: clone;
        /// }
        /// ```
        Clone => "box-decoration-clone",
        /// ```css
        /// {
        ///     box-decoration-break: slice;
        /// }
        /// ```
        Slice => "box-decoration-slice",
    }
    /// Utilities for controlling how the browser should calculate an element's total size.
    ///
    /// <https://tailwindcss.com/docs/box-sizing>
    BoxSizing {
        /// ```css
        /// {
        ///     box-sizing: border-box;
        /// }
        /// ```
        Border => "box-border",
        /// ```css
        /// {
        ///     box-sizing: content-box;
        /// }
        /// ```
        Content => "box-content",
    }
    /// Utilities for controlling the display box type of an element.
    ///
    /// <https://tailwindcss.com/docs/display>
    Display {
        /// ```css
        /// {
        ///     display: block;
        /// }
        /// ```
        Block => "block",
        /// ```css
        /// {
        ///     display: inline-block;
        /// }
        /// ```
        InlineBlock => "inline-block",
        /// ```css
        /// {
        ///     display: inline;
        /// }
        /// ```
        Inline => "inline",
        /// ```css
        /// {
        ///     display: flex;
        /// }
        /// ```
        Flex => "flex",
        /// ```css
        /// {
        ///     display: inline-flex;
        /// }
        /// ```
        InlineFlex => "inline-flex",
        /// ```css
        /// {
        ///     display: table;
        /// }
        /// ```
        Table => "table",
        /// ```css
        /// {
        ///     display: inline-table;
        /// }
        /// ```
        InlineTable => "inline-table",
        /// ```css
        /// {
        ///     display: table-caption;
        /// }
        /// ```
        TableCaption => "table-caption",
        /// ```css
        /// {
        ///     display: table-cell;
        /// }
        /// ```
        TableCell => "table-cell",
        /// ```css
        /// {
        ///     display: table-column;
        /// }
        /// ```
        TableColumn => "table-column",
        /// ```css
        /// {
        ///     display: table-column-group;
        /// }
        /// ```
        TableColumnGroup => "table-column-group",
        /// ```css
        /// {
        ///     display: table-footer-group;
        /// }
        /// ```
        TableFooterGroup => "table-footer-group",
        /// ```css
        /// {
        ///     display: table-header-group;
        /// }
        /// ```
        TableHeaderGroup => "table-header-group",
        /// ```css
        /// {
        ///     display: table-row-group;
        /// }
        /// ```
        TableRowGroup => "table-row-group",
        /// ```css
        /// {
        ///     display: table-row;
        /// }
        /// ```
        TableRow => "table-row",
        /// ```css
        /// {
        ///     display: flow-root;
        /// }
        /// ```
        FlowRoot => "flow-root",
        /// ```css
        /// {
        ///     display: grid;
        /// }
        /// ```
        Grid => "grid",
        /// ```css
        /// {
        ///     display: inline-grid;
        /// }
        /// ```
        InlineGrid => "inline-grid",
        /// ```css
        /// {
        ///     display: contents;
        /// }
        /// ```
        Contents => "contents",
        /// ```css
        /// {
        ///     display: list-item;
        /// }
        /// ```
        ListItem => "list-item",
        /// ```css
        /// {
        ///     display: none;
        /// }
        /// ```
        Hidden => "hidden",
    }
    /// Utilities for controlling the wrapping of content around an element.
    ///
    /// <https://tailwindcss.com/docs/float>
    Floats {
        /// ```css
        /// {
        ///     float: inline-start;
        /// }
        /// ```
        Start => "float-start",
        /// ```css
        /// {
        ///     float: inline-end;
        /// }
        /// ```
        End => "float-end",
        /// ```css
        /// {
        ///     float: right;
        /// }
        /// ```
        Right => "float-right",
        /// ```css
        /// {
        ///     float: left;
        /// }
        /// ```
        Left => "float-left",
        /// ```css
        /// {
        ///     float: none;
        /// }
        /// ```
        None => "float-none",
    }
    /// Utilities for controlling the wrapping of content around an element.
    ///
    /// <https://tailwindcss.com/docs/clear>
    Clear {
        /// ```css
        /// {
        ///     clear: inline-start;
        /// }
        /// ```
        Start => "clear-start",
        /// ```css
        /// {
        ///     clear: inline-end;
        /// }
        /// ```
        End => "clear-end",
        /// ```css
        /// {
        ///     clear: left;
        /// }
        /// ```
        Left => "clear-left",
        /// ```css
        /// {
        ///     clear: right;
        /// }
        /// ```
        Right => "clear-right",
        /// ```css
        /// {
        ///     clear: both;
        /// }
        /// ```
        Both => "clear-both",
        /// ```css
        /// {
        ///     clear: none;
        /// }
        /// ```
        None => "clear-none",
    }
    /// Utilities for controlling whether an element should explicitly create a new stacking context.
    ///
    /// <https://tailwindcss.com/docs/isolation>
    Isolation {
        /// ```css
        /// {
        ///     isolation: isolate;
        /// }
        /// ```
        Isolate => "isolate",
        /// ```css
        /// {
        ///     isolation: auto;
        /// }
        /// ```
        Auto => "isolation-auto",
    }
    /// Utilities for controlling how a replaced element's content should be resized.
    ///
    /// <https://tailwindcss.com/docs/object-fit>
    ObjectFit {
        /// ```css
        /// {
        ///     object-fit: contain;
        /// }
        /// ```
        Contain => "object-contain",
        /// ```css
        /// {
        ///     object-fit: cover;
        /// }
        /// ```
        Cover => "object-cover",
        /// ```css
        /// {
        ///     object-fit: fill;
        /// }
        /// ```
        Fill => "object-fill",
        /// ```css
        /// {
        ///     object-fit: none;
        /// }
        /// ```
        None => "object-none",
        /// ```css
        /// {
        ///     object-fit: scale-down;
        /// }
        /// ```
        ScaleDown => "object-scale-down",
    }
    /// Utilities for controlling how a replaced element's content should be positioned within its container.
    ///
    /// <https://tailwindcss.com/docs/object-position>
    ObjectPosition {
        /// ```css
        /// {
        ///     object-position: bottom;
        /// }
        /// ```
        Bottom => "object-bottom",
        /// ```css
        /// {
        ///     object-position: center;
        /// }
        /// ```
        Center => "object-center",
        /// ```css
        /// {
        ///     object-position: left;
        /// }
        /// ```
        Left => "object-left",
        /// ```css
        /// {
        ///     object-position: left bottom;
        /// }
        /// ```
        LeftBottom => "object-left-bottom",
        /// ```css
        /// {
        ///     object-position: left top;
        /// }
        /// ```
        LeftTop => "object-left-top",
        /// ```css
        /// {
        ///     object-position: right;
        /// }
        /// ```
        Right => "object-right",
        /// ```css
        /// {
        ///     object-position: right bottom;
        /// }
        /// ```
        RightBottom => "object-right-bottom",
        /// ```css
        /// {
        ///     object-position: right top;
        /// }
        /// ```
        RightTop => "object-right-top",
        /// ```css
        /// {
        ///     object-position: top;
        /// }
        /// ```
        Top => "object-top",
    }
    /// Utilities for controlling how an element handles content that is too large for the container.
    ///
    /// <https://tailwindcss.com/docs/overflow>
    Overflow {
        /// ```css
        /// {
        ///     overflow: auto;
        /// }
        /// ```
        Auto => "overflow-auto",
        /// ```css
        /// {
        ///     overflow: hidden;
        /// }
        /// ```
        Hidden => "overflow-hidden",
        /// ```css
        /// {
        ///     overflow: clip;
        /// }
        /// ```
        Clip => "overflow-clip",
        /// ```css
        /// {
        ///     overflow: visible;
        /// }
        /// ```
        Visible => "overflow-visible",
        /// ```css
        /// {
        ///     overflow: scroll;
        /// }
        /// ```
        Scroll => "overflow-scroll",
        /// ```css
        /// {
        ///     overflow-x: auto;
        /// }
        /// ```
        XAuto => "overflow-x-auto",
        /// ```css
        /// {
        ///     overflow-y: auto;
        /// }
        /// ```
        YAuto => "overflow-y-auto",
        /// ```css
        /// {
        ///     overflow-x: hidden;
        /// }
        /// ```
        XHidden => "overflow-x-hidden",
        /// ```css
        /// {
        ///     overflow-y: hidden;
        /// }
        /// ```
        YHidden => "overflow-y-hidden",
        /// ```css
        /// {
        ///     overflow-x: clip;
        /// }
        /// ```
        XClip => "overflow-x-clip",
        /// ```css
        /// {
        ///     overflow-y: clip;
        /// }
        /// ```
        YClip => "overflow-y-clip",
        /// ```css
        /// {
        ///     overflow-x: visible;
        /// }
        /// ```
        XVisible => "overflow-x-visible",
        /// ```css
        /// {
        ///     overflow-y: visible;
        /// }
        /// ```
        YVisible => "overflow-y-visible",
        /// ```css
        /// {
        ///     overflow-x: scroll;
        /// }
        /// ```
        XScroll => "overflow-x-scroll",
        /// ```css
        /// {
        ///     overflow-y: scroll;
        /// }
        /// ```
        YScroll => "overflow-y-scroll",
    }
    /// Utilities for controlling how the browser behaves when reaching the boundary of a scrolling area.
    ///
    /// <https://tailwindcss.com/docs/overscroll-behavior>
    OverscrollBehavior {
        /// ```css
        /// {
        ///     overscroll-behavior: auto;
        /// }
        /// ```
        Auto => "overscroll-auto",
        /// ```css
        /// {
        ///     overscroll-behavior: contain;
        /// }
        /// ```
        Contain => "overscroll-contain",
        /// ```css
        /// {
        ///     overscroll-behavior: none;
        /// }
        /// ```
        None => "overscroll-none",
        /// ```css
        /// {
        ///     overscroll-behavior-y: auto;
        /// }
        /// ```
        YAuto => "overscroll-y-auto",
        /// ```css
        /// {
        ///     overscroll-behavior-y: contain;
        /// }
        /// ```
        YContain => "overscroll-y-contain",
        /// ```css
        /// {
        ///     overscroll-behavior-y: none;
        /// }
        /// ```
        YNone => "overscroll-y-none",
        /// ```css
        /// {
        ///     overscroll-behavior-x: auto;
        /// }
        /// ```
        XAuto => "overscroll-x-auto",
        /// ```css
        /// {
        ///     overscroll-behavior-x: contain;
        /// }
        /// ```
        XContain => "overscroll-x-contain",
        /// ```css
        /// {
        ///     overscroll-behavior-x: none;
        /// }
        /// ```
        XNone => "overscroll-x-none",
    }
    /// Utilities for controlling how an element is positioned in the DOM.
    ///
    /// <https://tailwindcss.com/docs/position>
    Position {
        /// ```css
        /// {
        ///     position: static;
        /// }
        /// ```
        Static => "static",
        /// ```css
        /// {
        ///     position: fixed;
        /// }
        /// ```
        Fixed => "fixed",
        /// ```css
        /// {
        ///     position: absolute;
        /// }
        /// ```
        Absolute => "absolute",
        /// ```css
        /// {
        ///     position: relative;
        /// }
        /// ```
        Relative => "relative",
        /// ```css
        /// {
        ///     position: sticky;
        /// }
        /// ```
        Sticky => "sticky",
    }
    /// Utilities for controlling the placement of positioned elements.
    ///
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    TopRightBottomLeft {
        /// ```css
        /// {
        ///     inset: 0px;
        /// }
        /// ```
        Inset0 => "inset-0",
        /// ```css
        /// {
        ///     left: 0px;
        ///     right: 0px;
        /// }
        /// ```
        InsetX0 => "inset-x-0",
        /// ```css
        /// {
        ///     top: 0px;
        ///     bottom: 0px;
        /// }
        /// ```
        InsetY0 => "inset-y-0",
        /// ```css
        /// {
        ///     inset-inline-start: 0px;
        /// }
        /// ```
        Start0 => "start-0",
        /// ```css
        /// {
        ///     inset-inline-end: 0px;
        /// }
        /// ```
        End0 => "end-0",
        /// ```css
        /// {
        ///     top: 0px;
        /// }
        /// ```
        Top0 => "top-0",
        /// ```css
        /// {
        ///     right: 0px;
        /// }
        /// ```
        Right0 => "right-0",
        /// ```css
        /// {
        ///     bottom: 0px;
        /// }
        /// ```
        Bottom0 => "bottom-0",
        /// ```css
        /// {
        ///     left: 0px;
        /// }
        /// ```
        Left0 => "left-0",
        /// ```css
        /// {
        ///     inset: 1px;
        /// }
        /// ```
        InsetPx => "inset-px",
        /// ```css
        /// {
        ///     left: 1px;
        ///     right: 1px;
        /// }
        /// ```
        InsetXPx => "inset-x-px",
        /// ```css
        /// {
        ///     top: 1px;
        ///     bottom: 1px;
        /// }
        /// ```
        InsetYPx => "inset-y-px",
        /// ```css
        /// {
        ///     inset-inline-start: 1px;
        /// }
        /// ```
        StartPx => "start-px",
        /// ```css
        /// {
        ///     inset-inline-end: 1px;
        /// }
        /// ```
        EndPx => "end-px",
        /// ```css
        /// {
        ///     top: 1px;
        /// }
        /// ```
        TopPx => "top-px",
        /// ```css
        /// {
        ///     right: 1px;
        /// }
        /// ```
        RightPx => "right-px",
        /// ```css
        /// {
        ///     bottom: 1px;
        /// }
        /// ```
        BottomPx => "bottom-px",
        /// ```css
        /// {
        ///     left: 1px;
        /// }
        /// ```
        LeftPx => "left-px",
        /// ```css
        /// {
        ///     inset: 0.125rem; /* 2px */
        /// }
        /// ```
        Inset0_5 => "inset-0.5",
        /// ```css
        /// {
        ///     left: 0.125rem; /* 2px */
        ///     right: 0.125rem; /* 2px */
        /// }
        /// ```
        InsetX0_5 => "inset-x-0.5",
        /// ```css
        /// {
        ///     top: 0.125rem; /* 2px */
        ///     bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        InsetY0_5 => "inset-y-0.5",
        /// ```css
        /// {
        ///     inset-inline-start: 0.125rem; /* 2px */
        /// }
        /// ```
        Start0_5 => "start-0.5",
        /// ```css
        /// {
        ///     inset-inline-end: 0.125rem; /* 2px */
        /// }
        /// ```
        End0_5 => "end-0.5",
        /// ```css
        /// {
        ///     top: 0.125rem; /* 2px */
        /// }
        /// ```
        Top0_5 => "top-0.5",
        /// ```css
        /// {
        ///     right: 0.125rem; /* 2px */
        /// }
        /// ```
        Right0_5 => "right-0.5",
        /// ```css
        /// {
        ///     bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        Bottom0_5 => "bottom-0.5",
        /// ```css
        /// {
        ///     left: 0.125rem; /* 2px */
        /// }
        /// ```
        Left0_5 => "left-0.5",
        /// ```css
        /// {
        ///     inset: 0.25rem; /* 4px */
        /// }
        /// ```
        Inset1 => "inset-1",
        /// ```css
        /// {
        ///     left: 0.25rem; /* 4px */
        ///     right: 0.25rem; /* 4px */
        /// }
        /// ```
        InsetX1 => "inset-x-1",
        /// ```css
        /// {
        ///     top: 0.25rem; /* 4px */
        ///     bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        InsetY1 => "inset-y-1",
        /// ```css
        /// {
        ///     inset-inline-start: 0.25rem; /* 4px */
        /// }
        /// ```
        Start1 => "start-1",
        /// ```css
        /// {
        ///     inset-inline-end: 0.25rem; /* 4px */
        /// }
        /// ```
        End1 => "end-1",
        /// ```css
        /// {
        ///     top: 0.25rem; /* 4px */
        /// }
        /// ```
        Top1 => "top-1",
        /// ```css
        /// {
        ///     right: 0.25rem; /* 4px */
        /// }
        /// ```
        Right1 => "right-1",
        /// ```css
        /// {
        ///     bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        Bottom1 => "bottom-1",
        /// ```css
        /// {
        ///     left: 0.25rem; /* 4px */
        /// }
        /// ```
        Left1 => "left-1",
        /// ```css
        /// {
        ///     inset: 0.375rem; /* 6px */
        /// }
        /// ```
        Inset1_5 => "inset-1.5",
        /// ```css
        /// {
        ///     left: 0.375rem; /* 6px */
        ///     right: 0.375rem; /* 6px */
        /// }
        /// ```
        InsetX1_5 => "inset-x-1.5",
        /// ```css
        /// {
        ///     top: 0.375rem; /* 6px */
        ///     bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        InsetY1_5 => "inset-y-1.5",
        /// ```css
        /// {
        ///     inset-inline-start: 0.375rem; /* 6px */
        /// }
        /// ```
        Start1_5 => "start-1.5",
        /// ```css
        /// {
        ///     inset-inline-end: 0.375rem; /* 6px */
        /// }
        /// ```
        End1_5 => "end-1.5",
        /// ```css
        /// {
        ///     top: 0.375rem; /* 6px */
        /// }
        /// ```
        Top1_5 => "top-1.5",
        /// ```css
        /// {
        ///     right: 0.375rem; /* 6px */
        /// }
        /// ```
        Right1_5 => "right-1.5",
        /// ```css
        /// {
        ///     bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        Bottom1_5 => "bottom-1.5",
        /// ```css
        /// {
        ///     left: 0.375rem; /* 6px */
        /// }
        /// ```
        Left1_5 => "left-1.5",
        /// ```css
        /// {
        ///     inset: 0.5rem; /* 8px */
        /// }
        /// ```
        Inset2 => "inset-2",
        /// ```css
        /// {
        ///     left: 0.5rem; /* 8px */
        ///     right: 0.5rem; /* 8px */
        /// }
        /// ```
        InsetX2 => "inset-x-2",
        /// ```css
        /// {
        ///     top: 0.5rem; /* 8px */
        ///     bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        InsetY2 => "inset-y-2",
        /// ```css
        /// {
        ///     inset-inline-start: 0.5rem; /* 8px */
        /// }
        /// ```
        Start2 => "start-2",
        /// ```css
        /// {
        ///     inset-inline-end: 0.5rem; /* 8px */
        /// }
        /// ```
        End2 => "end-2",
        /// ```css
        /// {
        ///     top: 0.5rem; /* 8px */
        /// }
        /// ```
        Top2 => "top-2",
        /// ```css
        /// {
        ///     right: 0.5rem; /* 8px */
        /// }
        /// ```
        Right2 => "right-2",
        /// ```css
        /// {
        ///     bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        Bottom2 => "bottom-2",
        /// ```css
        /// {
        ///     left: 0.5rem; /* 8px */
        /// }
        /// ```
        Left2 => "left-2",
        /// ```css
        /// {
        ///     inset: 0.625rem; /* 10px */
        /// }
        /// ```
        Inset2_5 => "inset-2.5",
        /// ```css
        /// {
        ///     left: 0.625rem; /* 10px */
        ///     right: 0.625rem; /* 10px */
        /// }
        /// ```
        InsetX2_5 => "inset-x-2.5",
        /// ```css
        /// {
        ///     top: 0.625rem; /* 10px */
        ///     bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        InsetY2_5 => "inset-y-2.5",
        /// ```css
        /// {
        ///     inset-inline-start: 0.625rem; /* 10px */
        /// }
        /// ```
        Start2_5 => "start-2.5",
        /// ```css
        /// {
        ///     inset-inline-end: 0.625rem; /* 10px */
        /// }
        /// ```
        End2_5 => "end-2.5",
        /// ```css
        /// {
        ///     top: 0.625rem; /* 10px */
        /// }
        /// ```
        Top2_5 => "top-2.5",
        /// ```css
        /// {
        ///     right: 0.625rem; /* 10px */
        /// }
        /// ```
        Right2_5 => "right-2.5",
        /// ```css
        /// {
        ///     bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        Bottom2_5 => "bottom-2.5",
        /// ```css
        /// {
        ///     left: 0.625rem; /* 10px */
        /// }
        /// ```
        Left2_5 => "left-2.5",
        /// ```css
        /// {
        ///     inset: 0.75rem; /* 12px */
        /// }
        /// ```
        Inset3 => "inset-3",
        /// ```css
        /// {
        ///     left: 0.75rem; /* 12px */
        ///     right: 0.75rem; /* 12px */
        /// }
        /// ```
        InsetX3 => "inset-x-3",
        /// ```css
        /// {
        ///     top: 0.75rem; /* 12px */
        ///     bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        InsetY3 => "inset-y-3",
        /// ```css
        /// {
        ///     inset-inline-start: 0.75rem; /* 12px */
        /// }
        /// ```
        Start3 => "start-3",
        /// ```css
        /// {
        ///     inset-inline-end: 0.75rem; /* 12px */
        /// }
        /// ```
        End3 => "end-3",
        /// ```css
        /// {
        ///     top: 0.75rem; /* 12px */
        /// }
        /// ```
        Top3 => "top-3",
        /// ```css
        /// {
        ///     right: 0.75rem; /* 12px */
        /// }
        /// ```
        Right3 => "right-3",
        /// ```css
        /// {
        ///     bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        Bottom3 => "bottom-3",
        /// ```css
        /// {
        ///     left: 0.75rem; /* 12px */
        /// }
        /// ```
        Left3 => "left-3",
        /// ```css
        /// {
        ///     inset: 0.875rem; /* 14px */
        /// }
        /// ```
        Inset3_5 => "inset-3.5",
        /// ```css
        /// {
        ///     left: 0.875rem; /* 14px */
        ///     right: 0.875rem; /* 14px */
        /// }
        /// ```
        InsetX3_5 => "inset-x-3.5",
        /// ```css
        /// {
        ///     top: 0.875rem; /* 14px */
        ///     bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        InsetY3_5 => "inset-y-3.5",
        /// ```css
        /// {
        ///     inset-inline-start: 0.875rem; /* 14px */
        /// }
        /// ```
        Start3_5 => "start-3.5",
        /// ```css
        /// {
        ///     inset-inline-end: 0.875rem; /* 14px */
        /// }
        /// ```
        End3_5 => "end-3.5",
        /// ```css
        /// {
        ///     top: 0.875rem; /* 14px */
        /// }
        /// ```
        Top3_5 => "top-3.5",
        /// ```css
        /// {
        ///     right: 0.875rem; /* 14px */
        /// }
        /// ```
        Right3_5 => "right-3.5",
        /// ```css
        /// {
        ///     bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        Bottom3_5 => "bottom-3.5",
        /// ```css
        /// {
        ///     left: 0.875rem; /* 14px */
        /// }
        /// ```
        Left3_5 => "left-3.5",
        /// ```css
        /// {
        ///     inset: 1rem; /* 16px */
        /// }
        /// ```
        Inset4 => "inset-4",
        /// ```css
        /// {
        ///     left: 1rem; /* 16px */
        ///     right: 1rem; /* 16px */
        /// }
        /// ```
        InsetX4 => "inset-x-4",
        /// ```css
        /// {
        ///     top: 1rem; /* 16px */
        ///     bottom: 1rem; /* 16px */
        /// }
        /// ```
        InsetY4 => "inset-y-4",
        /// ```css
        /// {
        ///     inset-inline-start: 1rem; /* 16px */
        /// }
        /// ```
        Start4 => "start-4",
        /// ```css
        /// {
        ///     inset-inline-end: 1rem; /* 16px */
        /// }
        /// ```
        End4 => "end-4",
        /// ```css
        /// {
        ///     top: 1rem; /* 16px */
        /// }
        /// ```
        Top4 => "top-4",
        /// ```css
        /// {
        ///     right: 1rem; /* 16px */
        /// }
        /// ```
        Right4 => "right-4",
        /// ```css
        /// {
        ///     bottom: 1rem; /* 16px */
        /// }
        /// ```
        Bottom4 => "bottom-4",
        /// ```css
        /// {
        ///     left: 1rem; /* 16px */
        /// }
        /// ```
        Left4 => "left-4",
        /// ```css
        /// {
        ///     inset: 1.25rem; /* 20px */
        /// }
        /// ```
        Inset5 => "inset-5",
        /// ```css
        /// {
        ///     left: 1.25rem; /* 20px */
        ///     right: 1.25rem; /* 20px */
        /// }
        /// ```
        InsetX5 => "inset-x-5",
        /// ```css
        /// {
        ///     top: 1.25rem; /* 20px */
        ///     bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        InsetY5 => "inset-y-5",
        /// ```css
        /// {
        ///     inset-inline-start: 1.25rem; /* 20px */
        /// }
        /// ```
        Start5 => "start-5",
        /// ```css
        /// {
        ///     inset-inline-end: 1.25rem; /* 20px */
        /// }
        /// ```
        End5 => "end-5",
        /// ```css
        /// {
        ///     top: 1.25rem; /* 20px */
        /// }
        /// ```
        Top5 => "top-5",
        /// ```css
        /// {
        ///     right: 1.25rem; /* 20px */
        /// }
        /// ```
        Right5 => "right-5",
        /// ```css
        /// {
        ///     bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        Bottom5 => "bottom-5",
        /// ```css
        /// {
        ///     left: 1.25rem; /* 20px */
        /// }
        /// ```
        Left5 => "left-5",
        /// ```css
        /// {
        ///     inset: 1.5rem; /* 24px */
        /// }
        /// ```
        Inset6 => "inset-6",
        /// ```css
        /// {
        ///     left: 1.5rem; /* 24px */
        ///     right: 1.5rem; /* 24px */
        /// }
        /// ```
        InsetX6 => "inset-x-6",
        /// ```css
        /// {
        ///     top: 1.5rem; /* 24px */
        ///     bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        InsetY6 => "inset-y-6",
        /// ```css
        /// {
        ///     inset-inline-start: 1.5rem; /* 24px */
        /// }
        /// ```
        Start6 => "start-6",
        /// ```css
        /// {
        ///     inset-inline-end: 1.5rem; /* 24px */
        /// }
        /// ```
        End6 => "end-6",
        /// ```css
        /// {
        ///     top: 1.5rem; /* 24px */
        /// }
        /// ```
        Top6 => "top-6",
        /// ```css
        /// {
        ///     right: 1.5rem; /* 24px */
        /// }
        /// ```
        Right6 => "right-6",
        /// ```css
        /// {
        ///     bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        Bottom6 => "bottom-6",
        /// ```css
        /// {
        ///     left: 1.5rem; /* 24px */
        /// }
        /// ```
        Left6 => "left-6",
        /// ```css
        /// {
        ///     inset: 1.75rem; /* 28px */
        /// }
        /// ```
        Inset7 => "inset-7",
        /// ```css
        /// {
        ///     left: 1.75rem; /* 28px */
        ///     right: 1.75rem; /* 28px */
        /// }
        /// ```
        InsetX7 => "inset-x-7",
        /// ```css
        /// {
        ///     top: 1.75rem; /* 28px */
        ///     bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        InsetY7 => "inset-y-7",
        /// ```css
        /// {
        ///     inset-inline-start: 1.75rem; /* 28px */
        /// }
        /// ```
        Start7 => "start-7",
        /// ```css
        /// {
        ///     inset-inline-end: 1.75rem; /* 28px */
        /// }
        /// ```
        End7 => "end-7",
        /// ```css
        /// {
        ///     top: 1.75rem; /* 28px */
        /// }
        /// ```
        Top7 => "top-7",
        /// ```css
        /// {
        ///     right: 1.75rem; /* 28px */
        /// }
        /// ```
        Right7 => "right-7",
        /// ```css
        /// {
        ///     bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        Bottom7 => "bottom-7",
        /// ```css
        /// {
        ///     left: 1.75rem; /* 28px */
        /// }
        /// ```
        Left7 => "left-7",
        /// ```css
        /// {
        ///     inset: 2rem; /* 32px */
        /// }
        /// ```
        Inset8 => "inset-8",
        /// ```css
        /// {
        ///     left: 2rem; /* 32px */
        ///     right: 2rem; /* 32px */
        /// }
        /// ```
        InsetX8 => "inset-x-8",
        /// ```css
        /// {
        ///     top: 2rem; /* 32px */
        ///     bottom: 2rem; /* 32px */
        /// }
        /// ```
        InsetY8 => "inset-y-8",
        /// ```css
        /// {
        ///     inset-inline-start: 2rem; /* 32px */
        /// }
        /// ```
        Start8 => "start-8",
        /// ```css
        /// {
        ///     inset-inline-end: 2rem; /* 32px */
        /// }
        /// ```
        End8 => "end-8",
        /// ```css
        /// {
        ///     top: 2rem; /* 32px */
        /// }
        /// ```
        Top8 => "top-8",
        /// ```css
        /// {
        ///     right: 2rem; /* 32px */
        /// }
        /// ```
        Right8 => "right-8",
        /// ```css
        /// {
        ///     bottom: 2rem; /* 32px */
        /// }
        /// ```
        Bottom8 => "bottom-8",
        /// ```css
        /// {
        ///     left: 2rem; /* 32px */
        /// }
        /// ```
        Left8 => "left-8",
        /// ```css
        /// {
        ///     inset: 2.25rem; /* 36px */
        /// }
        /// ```
        Inset9 => "inset-9",
        /// ```css
        /// {
        ///     left: 2.25rem; /* 36px */
        ///     right: 2.25rem; /* 36px */
        /// }
        /// ```
        InsetX9 => "inset-x-9",
        /// ```css
        /// {
        ///     top: 2.25rem; /* 36px */
        ///     bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        InsetY9 => "inset-y-9",
        /// ```css
        /// {
        ///     inset-inline-start: 2.25rem; /* 36px */
        /// }
        /// ```
        Start9 => "start-9",
        /// ```css
        /// {
        ///     inset-inline-end: 2.25rem; /* 36px */
        /// }
        /// ```
        End9 => "end-9",
        /// ```css
        /// {
        ///     top: 2.25rem; /* 36px */
        /// }
        /// ```
        Top9 => "top-9",
        /// ```css
        /// {
        ///     right: 2.25rem; /* 36px */
        /// }
        /// ```
        Right9 => "right-9",
        /// ```css
        /// {
        ///     bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        Bottom9 => "bottom-9",
        /// ```css
        /// {
        ///     left: 2.25rem; /* 36px */
        /// }
        /// ```
        Left9 => "left-9",
        /// ```css
        /// {
        ///     inset: 2.5rem; /* 40px */
        /// }
        /// ```
        Inset10 => "inset-10",
        /// ```css
        /// {
        ///     left: 2.5rem; /* 40px */
        ///     right: 2.5rem; /* 40px */
        /// }
        /// ```
        InsetX10 => "inset-x-10",
        /// ```css
        /// {
        ///     top: 2.5rem; /* 40px */
        ///     bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        InsetY10 => "inset-y-10",
        /// ```css
        /// {
        ///     inset-inline-start: 2.5rem; /* 40px */
        /// }
        /// ```
        Start10 => "start-10",
        /// ```css
        /// {
        ///     inset-inline-end: 2.5rem; /* 40px */
        /// }
        /// ```
        End10 => "end-10",
        /// ```css
        /// {
        ///     top: 2.5rem; /* 40px */
        /// }
        /// ```
        Top10 => "top-10",
        /// ```css
        /// {
        ///     right: 2.5rem; /* 40px */
        /// }
        /// ```
        Right10 => "right-10",
        /// ```css
        /// {
        ///     bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        Bottom10 => "bottom-10",
        /// ```css
        /// {
        ///     left: 2.5rem; /* 40px */
        /// }
        /// ```
        Left10 => "left-10",
        /// ```css
        /// {
        ///     inset: 2.75rem; /* 44px */
        /// }
        /// ```
        Inset11 => "inset-11",
        /// ```css
        /// {
        ///     left: 2.75rem; /* 44px */
        ///     right: 2.75rem; /* 44px */
        /// }
        /// ```
        InsetX11 => "inset-x-11",
        /// ```css
        /// {
        ///     top: 2.75rem; /* 44px */
        ///     bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        InsetY11 => "inset-y-11",
        /// ```css
        /// {
        ///     inset-inline-start: 2.75rem; /* 44px */
        /// }
        /// ```
        Start11 => "start-11",
        /// ```css
        /// {
        ///     inset-inline-end: 2.75rem; /* 44px */
        /// }
        /// ```
        End11 => "end-11",
        /// ```css
        /// {
        ///     top: 2.75rem; /* 44px */
        /// }
        /// ```
        Top11 => "top-11",
        /// ```css
        /// {
        ///     right: 2.75rem; /* 44px */
        /// }
        /// ```
        Right11 => "right-11",
        /// ```css
        /// {
        ///     bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        Bottom11 => "bottom-11",
        /// ```css
        /// {
        ///     left: 2.75rem; /* 44px */
        /// }
        /// ```
        Left11 => "left-11",
        /// ```css
        /// {
        ///     inset: 3rem; /* 48px */
        /// }
        /// ```
        Inset12 => "inset-12",
        /// ```css
        /// {
        ///     left: 3rem; /* 48px */
        ///     right: 3rem; /* 48px */
        /// }
        /// ```
        InsetX12 => "inset-x-12",
        /// ```css
        /// {
        ///     top: 3rem; /* 48px */
        ///     bottom: 3rem; /* 48px */
        /// }
        /// ```
        InsetY12 => "inset-y-12",
        /// ```css
        /// {
        ///     inset-inline-start: 3rem; /* 48px */
        /// }
        /// ```
        Start12 => "start-12",
        /// ```css
        /// {
        ///     inset-inline-end: 3rem; /* 48px */
        /// }
        /// ```
        End12 => "end-12",
        /// ```css
        /// {
        ///     top: 3rem; /* 48px */
        /// }
        /// ```
        Top12 => "top-12",
        /// ```css
        /// {
        ///     right: 3rem; /* 48px */
        /// }
        /// ```
        Right12 => "right-12",
        /// ```css
        /// {
        ///     bottom: 3rem; /* 48px */
        /// }
        /// ```
        Bottom12 => "bottom-12",
        /// ```css
        /// {
        ///     left: 3rem; /* 48px */
        /// }
        /// ```
        Left12 => "left-12",
        /// ```css
        /// {
        ///     inset: 3.5rem; /* 56px */
        /// }
        /// ```
        Inset14 => "inset-14",
        /// ```css
        /// {
        ///     left: 3.5rem; /* 56px */
        ///     right: 3.5rem; /* 56px */
        /// }
        /// ```
        InsetX14 => "inset-x-14",
        /// ```css
        /// {
        ///     top: 3.5rem; /* 56px */
        ///     bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        InsetY14 => "inset-y-14",
        /// ```css
        /// {
        ///     inset-inline-start: 3.5rem; /* 56px */
        /// }
        /// ```
        Start14 => "start-14",
        /// ```css
        /// {
        ///     inset-inline-end: 3.5rem; /* 56px */
        /// }
        /// ```
        End14 => "end-14",
        /// ```css
        /// {
        ///     top: 3.5rem; /* 56px */
        /// }
        /// ```
        Top14 => "top-14",
        /// ```css
        /// {
        ///     right: 3.5rem; /* 56px */
        /// }
        /// ```
        Right14 => "right-14",
        /// ```css
        /// {
        ///     bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        Bottom14 => "bottom-14",
        /// ```css
        /// {
        ///     left: 3.5rem; /* 56px */
        /// }
        /// ```
        Left14 => "left-14",
        /// ```css
        /// {
        ///     inset: 4rem; /* 64px */
        /// }
        /// ```
        Inset16 => "inset-16",
        /// ```css
        /// {
        ///     left: 4rem; /* 64px */
        ///     right: 4rem; /* 64px */
        /// }
        /// ```
        InsetX16 => "inset-x-16",
        /// ```css
        /// {
        ///     top: 4rem; /* 64px */
        ///     bottom: 4rem; /* 64px */
        /// }
        /// ```
        InsetY16 => "inset-y-16",
        /// ```css
        /// {
        ///     inset-inline-start: 4rem; /* 64px */
        /// }
        /// ```
        Start16 => "start-16",
        /// ```css
        /// {
        ///     inset-inline-end: 4rem; /* 64px */
        /// }
        /// ```
        End16 => "end-16",
        /// ```css
        /// {
        ///     top: 4rem; /* 64px */
        /// }
        /// ```
        Top16 => "top-16",
        /// ```css
        /// {
        ///     right: 4rem; /* 64px */
        /// }
        /// ```
        Right16 => "right-16",
        /// ```css
        /// {
        ///     bottom: 4rem; /* 64px */
        /// }
        /// ```
        Bottom16 => "bottom-16",
        /// ```css
        /// {
        ///     left: 4rem; /* 64px */
        /// }
        /// ```
        Left16 => "left-16",
        /// ```css
        /// {
        ///     inset: 5rem; /* 80px */
        /// }
        /// ```
        Inset20 => "inset-20",
        /// ```css
        /// {
        ///     left: 5rem; /* 80px */
        ///     right: 5rem; /* 80px */
        /// }
        /// ```
        InsetX20 => "inset-x-20",
        /// ```css
        /// {
        ///     top: 5rem; /* 80px */
        ///     bottom: 5rem; /* 80px */
        /// }
        /// ```
        InsetY20 => "inset-y-20",
        /// ```css
        /// {
        ///     inset-inline-start: 5rem; /* 80px */
        /// }
        /// ```
        Start20 => "start-20",
        /// ```css
        /// {
        ///     inset-inline-end: 5rem; /* 80px */
        /// }
        /// ```
        End20 => "end-20",
        /// ```css
        /// {
        ///     top: 5rem; /* 80px */
        /// }
        /// ```
        Top20 => "top-20",
        /// ```css
        /// {
        ///     right: 5rem; /* 80px */
        /// }
        /// ```
        Right20 => "right-20",
        /// ```css
        /// {
        ///     bottom: 5rem; /* 80px */
        /// }
        /// ```
        Bottom20 => "bottom-20",
        /// ```css
        /// {
        ///     left: 5rem; /* 80px */
        /// }
        /// ```
        Left20 => "left-20",
        /// ```css
        /// {
        ///     inset: 6rem; /* 96px */
        /// }
        /// ```
        Inset24 => "inset-24",
        /// ```css
        /// {
        ///     left: 6rem; /* 96px */
        ///     right: 6rem; /* 96px */
        /// }
        /// ```
        InsetX24 => "inset-x-24",
        /// ```css
        /// {
        ///     top: 6rem; /* 96px */
        ///     bottom: 6rem; /* 96px */
        /// }
        /// ```
        InsetY24 => "inset-y-24",
        /// ```css
        /// {
        ///     inset-inline-start: 6rem; /* 96px */
        /// }
        /// ```
        Start24 => "start-24",
        /// ```css
        /// {
        ///     inset-inline-end: 6rem; /* 96px */
        /// }
        /// ```
        End24 => "end-24",
        /// ```css
        /// {
        ///     top: 6rem; /* 96px */
        /// }
        /// ```
        Top24 => "top-24",
        /// ```css
        /// {
        ///     right: 6rem; /* 96px */
        /// }
        /// ```
        Right24 => "right-24",
        /// ```css
        /// {
        ///     bottom: 6rem; /* 96px */
        /// }
        /// ```
        Bottom24 => "bottom-24",
        /// ```css
        /// {
        ///     left: 6rem; /* 96px */
        /// }
        /// ```
        Left24 => "left-24",
        /// ```css
        /// {
        ///     inset: 7rem; /* 112px */
        /// }
        /// ```
        Inset28 => "inset-28",
        /// ```css
        /// {
        ///     left: 7rem; /* 112px */
        ///     right: 7rem; /* 112px */
        /// }
        /// ```
        InsetX28 => "inset-x-28",
        /// ```css
        /// {
        ///     top: 7rem; /* 112px */
        ///     bottom: 7rem; /* 112px */
        /// }
        /// ```
        InsetY28 => "inset-y-28",
        /// ```css
        /// {
        ///     inset-inline-start: 7rem; /* 112px */
        /// }
        /// ```
        Start28 => "start-28",
        /// ```css
        /// {
        ///     inset-inline-end: 7rem; /* 112px */
        /// }
        /// ```
        End28 => "end-28",
        /// ```css
        /// {
        ///     top: 7rem; /* 112px */
        /// }
        /// ```
        Top28 => "top-28",
        /// ```css
        /// {
        ///     right: 7rem; /* 112px */
        /// }
        /// ```
        Right28 => "right-28",
        /// ```css
        /// {
        ///     bottom: 7rem; /* 112px */
        /// }
        /// ```
        Bottom28 => "bottom-28",
        /// ```css
        /// {
        ///     left: 7rem; /* 112px */
        /// }
        /// ```
        Left28 => "left-28",
        /// ```css
        /// {
        ///     inset: 8rem; /* 128px */
        /// }
        /// ```
        Inset32 => "inset-32",
        /// ```css
        /// {
        ///     left: 8rem; /* 128px */
        ///     right: 8rem; /* 128px */
        /// }
        /// ```
        InsetX32 => "inset-x-32",
        /// ```css
        /// {
        ///     top: 8rem; /* 128px */
        ///     bottom: 8rem; /* 128px */
        /// }
        /// ```
        InsetY32 => "inset-y-32",
        /// ```css
        /// {
        ///     inset-inline-start: 8rem; /* 128px */
        /// }
        /// ```
        Start32 => "start-32",
        /// ```css
        /// {
        ///     inset-inline-end: 8rem; /* 128px */
        /// }
        /// ```
        End32 => "end-32",
        /// ```css
        /// {
        ///     top: 8rem; /* 128px */
        /// }
        /// ```
        Top32 => "top-32",
        /// ```css
        /// {
        ///     right: 8rem; /* 128px */
        /// }
        /// ```
        Right32 => "right-32",
        /// ```css
        /// {
        ///     bottom: 8rem; /* 128px */
        /// }
        /// ```
        Bottom32 => "bottom-32",
        /// ```css
        /// {
        ///     left: 8rem; /* 128px */
        /// }
        /// ```
        Left32 => "left-32",
        /// ```css
        /// {
        ///     inset: 9rem; /* 144px */
        /// }
        /// ```
        Inset36 => "inset-36",
        /// ```css
        /// {
        ///     left: 9rem; /* 144px */
        ///     right: 9rem; /* 144px */
        /// }
        /// ```
        InsetX36 => "inset-x-36",
        /// ```css
        /// {
        ///     top: 9rem; /* 144px */
        ///     bottom: 9rem; /* 144px */
        /// }
        /// ```
        InsetY36 => "inset-y-36",
        /// ```css
        /// {
        ///     inset-inline-start: 9rem; /* 144px */
        /// }
        /// ```
        Start36 => "start-36",
        /// ```css
        /// {
        ///     inset-inline-end: 9rem; /* 144px */
        /// }
        /// ```
        End36 => "end-36",
        /// ```css
        /// {
        ///     top: 9rem; /* 144px */
        /// }
        /// ```
        Top36 => "top-36",
        /// ```css
        /// {
        ///     right: 9rem; /* 144px */
        /// }
        /// ```
        Right36 => "right-36",
        /// ```css
        /// {
        ///     bottom: 9rem; /* 144px */
        /// }
        /// ```
        Bottom36 => "bottom-36",
        /// ```css
        /// {
        ///     left: 9rem; /* 144px */
        /// }
        /// ```
        Left36 => "left-36",
        /// ```css
        /// {
        ///     inset: 10rem; /* 160px */
        /// }
        /// ```
        Inset40 => "inset-40",
        /// ```css
        /// {
        ///     left: 10rem; /* 160px */
        ///     right: 10rem; /* 160px */
        /// }
        /// ```
        InsetX40 => "inset-x-40",
        /// ```css
        /// {
        ///     top: 10rem; /* 160px */
        ///     bottom: 10rem; /* 160px */
        /// }
        /// ```
        InsetY40 => "inset-y-40",
        /// ```css
        /// {
        ///     inset-inline-start: 10rem; /* 160px */
        /// }
        /// ```
        Start40 => "start-40",
        /// ```css
        /// {
        ///     inset-inline-end: 10rem; /* 160px */
        /// }
        /// ```
        End40 => "end-40",
        /// ```css
        /// {
        ///     top: 10rem; /* 160px */
        /// }
        /// ```
        Top40 => "top-40",
        /// ```css
        /// {
        ///     right: 10rem; /* 160px */
        /// }
        /// ```
        Right40 => "right-40",
        /// ```css
        /// {
        ///     bottom: 10rem; /* 160px */
        /// }
        /// ```
        Bottom40 => "bottom-40",
        /// ```css
        /// {
        ///     left: 10rem; /* 160px */
        /// }
        /// ```
        Left40 => "left-40",
        /// ```css
        /// {
        ///     inset: 11rem; /* 176px */
        /// }
        /// ```
        Inset44 => "inset-44",
        /// ```css
        /// {
        ///     left: 11rem; /* 176px */
        ///     right: 11rem; /* 176px */
        /// }
        /// ```
        InsetX44 => "inset-x-44",
        /// ```css
        /// {
        ///     top: 11rem; /* 176px */
        ///     bottom: 11rem; /* 176px */
        /// }
        /// ```
        InsetY44 => "inset-y-44",
        /// ```css
        /// {
        ///     inset-inline-start: 11rem; /* 176px */
        /// }
        /// ```
        Start44 => "start-44",
        /// ```css
        /// {
        ///     inset-inline-end: 11rem; /* 176px */
        /// }
        /// ```
        End44 => "end-44",
        /// ```css
        /// {
        ///     top: 11rem; /* 176px */
        /// }
        /// ```
        Top44 => "top-44",
        /// ```css
        /// {
        ///     right: 11rem; /* 176px */
        /// }
        /// ```
        Right44 => "right-44",
        /// ```css
        /// {
        ///     bottom: 11rem; /* 176px */
        /// }
        /// ```
        Bottom44 => "bottom-44",
        /// ```css
        /// {
        ///     left: 11rem; /* 176px */
        /// }
        /// ```
        Left44 => "left-44",
        /// ```css
        /// {
        ///     inset: 12rem; /* 192px */
        /// }
        /// ```
        Inset48 => "inset-48",
        /// ```css
        /// {
        ///     left: 12rem; /* 192px */
        ///     right: 12rem; /* 192px */
        /// }
        /// ```
        InsetX48 => "inset-x-48",
        /// ```css
        /// {
        ///     top: 12rem; /* 192px */
        ///     bottom: 12rem; /* 192px */
        /// }
        /// ```
        InsetY48 => "inset-y-48",
        /// ```css
        /// {
        ///     inset-inline-start: 12rem; /* 192px */
        /// }
        /// ```
        Start48 => "start-48",
        /// ```css
        /// {
        ///     inset-inline-end: 12rem; /* 192px */
        /// }
        /// ```
        End48 => "end-48",
        /// ```css
        /// {
        ///     top: 12rem; /* 192px */
        /// }
        /// ```
        Top48 => "top-48",
        /// ```css
        /// {
        ///     right: 12rem; /* 192px */
        /// }
        /// ```
        Right48 => "right-48",
        /// ```css
        /// {
        ///     bottom: 12rem; /* 192px */
        /// }
        /// ```
        Bottom48 => "bottom-48",
        /// ```css
        /// {
        ///     left: 12rem; /* 192px */
        /// }
        /// ```
        Left48 => "left-48",
        /// ```css
        /// {
        ///     inset: 13rem; /* 208px */
        /// }
        /// ```
        Inset52 => "inset-52",
        /// ```css
        /// {
        ///     left: 13rem; /* 208px */
        ///     right: 13rem; /* 208px */
        /// }
        /// ```
        InsetX52 => "inset-x-52",
        /// ```css
        /// {
        ///     top: 13rem; /* 208px */
        ///     bottom: 13rem; /* 208px */
        /// }
        /// ```
        InsetY52 => "inset-y-52",
        /// ```css
        /// {
        ///     inset-inline-start: 13rem; /* 208px */
        /// }
        /// ```
        Start52 => "start-52",
        /// ```css
        /// {
        ///     inset-inline-end: 13rem; /* 208px */
        /// }
        /// ```
        End52 => "end-52",
        /// ```css
        /// {
        ///     top: 13rem; /* 208px */
        /// }
        /// ```
        Top52 => "top-52",
        /// ```css
        /// {
        ///     right: 13rem; /* 208px */
        /// }
        /// ```
        Right52 => "right-52",
        /// ```css
        /// {
        ///     bottom: 13rem; /* 208px */
        /// }
        /// ```
        Bottom52 => "bottom-52",
        /// ```css
        /// {
        ///     left: 13rem; /* 208px */
        /// }
        /// ```
        Left52 => "left-52",
        /// ```css
        /// {
        ///     inset: 14rem; /* 224px */
        /// }
        /// ```
        Inset56 => "inset-56",
        /// ```css
        /// {
        ///     left: 14rem; /* 224px */
        ///     right: 14rem; /* 224px */
        /// }
        /// ```
        InsetX56 => "inset-x-56",
        /// ```css
        /// {
        ///     top: 14rem; /* 224px */
        ///     bottom: 14rem; /* 224px */
        /// }
        /// ```
        InsetY56 => "inset-y-56",
        /// ```css
        /// {
        ///     inset-inline-start: 14rem; /* 224px */
        /// }
        /// ```
        Start56 => "start-56",
        /// ```css
        /// {
        ///     inset-inline-end: 14rem; /* 224px */
        /// }
        /// ```
        End56 => "end-56",
        /// ```css
        /// {
        ///     top: 14rem; /* 224px */
        /// }
        /// ```
        Top56 => "top-56",
        /// ```css
        /// {
        ///     right: 14rem; /* 224px */
        /// }
        /// ```
        Right56 => "right-56",
        /// ```css
        /// {
        ///     bottom: 14rem; /* 224px */
        /// }
        /// ```
        Bottom56 => "bottom-56",
        /// ```css
        /// {
        ///     left: 14rem; /* 224px */
        /// }
        /// ```
        Left56 => "left-56",
        /// ```css
        /// {
        ///     inset: 15rem; /* 240px */
        /// }
        /// ```
        Inset60 => "inset-60",
        /// ```css
        /// {
        ///     left: 15rem; /* 240px */
        ///     right: 15rem; /* 240px */
        /// }
        /// ```
        InsetX60 => "inset-x-60",
        /// ```css
        /// {
        ///     top: 15rem; /* 240px */
        ///     bottom: 15rem; /* 240px */
        /// }
        /// ```
        InsetY60 => "inset-y-60",
        /// ```css
        /// {
        ///     inset-inline-start: 15rem; /* 240px */
        /// }
        /// ```
        Start60 => "start-60",
        /// ```css
        /// {
        ///     inset-inline-end: 15rem; /* 240px */
        /// }
        /// ```
        End60 => "end-60",
        /// ```css
        /// {
        ///     top: 15rem; /* 240px */
        /// }
        /// ```
        Top60 => "top-60",
        /// ```css
        /// {
        ///     right: 15rem; /* 240px */
        /// }
        /// ```
        Right60 => "right-60",
        /// ```css
        /// {
        ///     bottom: 15rem; /* 240px */
        /// }
        /// ```
        Bottom60 => "bottom-60",
        /// ```css
        /// {
        ///     left: 15rem; /* 240px */
        /// }
        /// ```
        Left60 => "left-60",
        /// ```css
        /// {
        ///     inset: 16rem; /* 256px */
        /// }
        /// ```
        Inset64 => "inset-64",
        /// ```css
        /// {
        ///     left: 16rem; /* 256px */
        ///     right: 16rem; /* 256px */
        /// }
        /// ```
        InsetX64 => "inset-x-64",
        /// ```css
        /// {
        ///     top: 16rem; /* 256px */
        ///     bottom: 16rem; /* 256px */
        /// }
        /// ```
        InsetY64 => "inset-y-64",
        /// ```css
        /// {
        ///     inset-inline-start: 16rem; /* 256px */
        /// }
        /// ```
        Start64 => "start-64",
        /// ```css
        /// {
        ///     inset-inline-end: 16rem; /* 256px */
        /// }
        /// ```
        End64 => "end-64",
        /// ```css
        /// {
        ///     top: 16rem; /* 256px */
        /// }
        /// ```
        Top64 => "top-64",
        /// ```css
        /// {
        ///     right: 16rem; /* 256px */
        /// }
        /// ```
        Right64 => "right-64",
        /// ```css
        /// {
        ///     bottom: 16rem; /* 256px */
        /// }
        /// ```
        Bottom64 => "bottom-64",
        /// ```css
        /// {
        ///     left: 16rem; /* 256px */
        /// }
        /// ```
        Left64 => "left-64",
        /// ```css
        /// {
        ///     inset: 18rem; /* 288px */
        /// }
        /// ```
        Inset72 => "inset-72",
        /// ```css
        /// {
        ///     left: 18rem; /* 288px */
        ///     right: 18rem; /* 288px */
        /// }
        /// ```
        InsetX72 => "inset-x-72",
        /// ```css
        /// {
        ///     top: 18rem; /* 288px */
        ///     bottom: 18rem; /* 288px */
        /// }
        /// ```
        InsetY72 => "inset-y-72",
        /// ```css
        /// {
        ///     inset-inline-start: 18rem; /* 288px */
        /// }
        /// ```
        Start72 => "start-72",
        /// ```css
        /// {
        ///     inset-inline-end: 18rem; /* 288px */
        /// }
        /// ```
        End72 => "end-72",
        /// ```css
        /// {
        ///     top: 18rem; /* 288px */
        /// }
        /// ```
        Top72 => "top-72",
        /// ```css
        /// {
        ///     right: 18rem; /* 288px */
        /// }
        /// ```
        Right72 => "right-72",
        /// ```css
        /// {
        ///     bottom: 18rem; /* 288px */
        /// }
        /// ```
        Bottom72 => "bottom-72",
        /// ```css
        /// {
        ///     left: 18rem; /* 288px */
        /// }
        /// ```
        Left72 => "left-72",
        /// ```css
        /// {
        ///     inset: 20rem; /* 320px */
        /// }
        /// ```
        Inset80 => "inset-80",
        /// ```css
        /// {
        ///     left: 20rem; /* 320px */
        ///     right: 20rem; /* 320px */
        /// }
        /// ```
        InsetX80 => "inset-x-80",
        /// ```css
        /// {
        ///     top: 20rem; /* 320px */
        ///     bottom: 20rem; /* 320px */
        /// }
        /// ```
        InsetY80 => "inset-y-80",
        /// ```css
        /// {
        ///     inset-inline-start: 20rem; /* 320px */
        /// }
        /// ```
        Start80 => "start-80",
        /// ```css
        /// {
        ///     inset-inline-end: 20rem; /* 320px */
        /// }
        /// ```
        End80 => "end-80",
        /// ```css
        /// {
        ///     top: 20rem; /* 320px */
        /// }
        /// ```
        Top80 => "top-80",
        /// ```css
        /// {
        ///     right: 20rem; /* 320px */
        /// }
        /// ```
        Right80 => "right-80",
        /// ```css
        /// {
        ///     bottom: 20rem; /* 320px */
        /// }
        /// ```
        Bottom80 => "bottom-80",
        /// ```css
        /// {
        ///     left: 20rem; /* 320px */
        /// }
        /// ```
        Left80 => "left-80",
        /// ```css
        /// {
        ///     inset: 24rem; /* 384px */
        /// }
        /// ```
        Inset96 => "inset-96",
        /// ```css
        /// {
        ///     left: 24rem; /* 384px */
        ///     right: 24rem; /* 384px */
        /// }
        /// ```
        InsetX96 => "inset-x-96",
        /// ```css
        /// {
        ///     top: 24rem; /* 384px */
        ///     bottom: 24rem; /* 384px */
        /// }
        /// ```
        InsetY96 => "inset-y-96",
        /// ```css
        /// {
        ///     inset-inline-start: 24rem; /* 384px */
        /// }
        /// ```
        Start96 => "start-96",
        /// ```css
        /// {
        ///     inset-inline-end: 24rem; /* 384px */
        /// }
        /// ```
        End96 => "end-96",
        /// ```css
        /// {
        ///     top: 24rem; /* 384px */
        /// }
        /// ```
        Top96 => "top-96",
        /// ```css
        /// {
        ///     right: 24rem; /* 384px */
        /// }
        /// ```
        Right96 => "right-96",
        /// ```css
        /// {
        ///     bottom: 24rem; /* 384px */
        /// }
        /// ```
        Bottom96 => "bottom-96",
        /// ```css
        /// {
        ///     left: 24rem; /* 384px */
        /// }
        /// ```
        Left96 => "left-96",
        /// ```css
        /// {
        ///     inset: auto;
        /// }
        /// ```
        InsetAuto => "inset-auto",
        /// ```css
        /// {
        ///     inset: 50%;
        /// }
        /// ```
        Inset1over2 => "inset-1/2",
        /// ```css
        /// {
        ///     inset: 33.333333%;
        /// }
        /// ```
        Inset1over3 => "inset-1/3",
        /// ```css
        /// {
        ///     inset: 66.666667%;
        /// }
        /// ```
        Inset2over3 => "inset-2/3",
        /// ```css
        /// {
        ///     inset: 25%;
        /// }
        /// ```
        Inset1over4 => "inset-1/4",
        /// ```css
        /// {
        ///     inset: 50%;
        /// }
        /// ```
        Inset2over4 => "inset-2/4",
        /// ```css
        /// {
        ///     inset: 75%;
        /// }
        /// ```
        Inset3over4 => "inset-3/4",
        /// ```css
        /// {
        ///     inset: 100%;
        /// }
        /// ```
        InsetFull => "inset-full",
        /// ```css
        /// {
        ///     left: auto;
        ///     right: auto;
        /// }
        /// ```
        InsetXAuto => "inset-x-auto",
        /// ```css
        /// {
        ///     left: 50%;
        ///     right: 50%;
        /// }
        /// ```
        InsetX1over2 => "inset-x-1/2",
        /// ```css
        /// {
        ///     left: 33.333333%;
        ///     right: 33.333333%;
        /// }
        /// ```
        InsetX1over3 => "inset-x-1/3",
        /// ```css
        /// {
        ///     left: 66.666667%;
        ///     right: 66.666667%;
        /// }
        /// ```
        InsetX2over3 => "inset-x-2/3",
        /// ```css
        /// {
        ///     left: 25%;
        ///     right: 25%;
        /// }
        /// ```
        InsetX1over4 => "inset-x-1/4",
        /// ```css
        /// {
        ///     left: 50%;
        ///     right: 50%;
        /// }
        /// ```
        InsetX2over4 => "inset-x-2/4",
        /// ```css
        /// {
        ///     left: 75%;
        ///     right: 75%;
        /// }
        /// ```
        InsetX3over4 => "inset-x-3/4",
        /// ```css
        /// {
        ///     left: 100%;
        ///     right: 100%;
        /// }
        /// ```
        InsetXFull => "inset-x-full",
        /// ```css
        /// {
        ///     top: auto;
        ///     bottom: auto;
        /// }
        /// ```
        InsetYAuto => "inset-y-auto",
        /// ```css
        /// {
        ///     top: 50%;
        ///     bottom: 50%;
        /// }
        /// ```
        InsetY1over2 => "inset-y-1/2",
        /// ```css
        /// {
        ///     top: 33.333333%;
        ///     bottom: 33.333333%;
        /// }
        /// ```
        InsetY1over3 => "inset-y-1/3",
        /// ```css
        /// {
        ///     top: 66.666667%;
        ///     bottom: 66.666667%;
        /// }
        /// ```
        InsetY2over3 => "inset-y-2/3",
        /// ```css
        /// {
        ///     top: 25%;
        ///     bottom: 25%;
        /// }
        /// ```
        InsetY1over4 => "inset-y-1/4",
        /// ```css
        /// {
        ///     top: 50%;
        ///     bottom: 50%;
        /// }
        /// ```
        InsetY2over4 => "inset-y-2/4",
        /// ```css
        /// {
        ///     top: 75%;
        ///     bottom: 75%;
        /// }
        /// ```
        InsetY3over4 => "inset-y-3/4",
        /// ```css
        /// {
        ///     top: 100%;
        ///     bottom: 100%;
        /// }
        /// ```
        InsetYFull => "inset-y-full",
        /// ```css
        /// {
        ///     inset-inline-start: auto;
        /// }
        /// ```
        StartAuto => "start-auto",
        /// ```css
        /// {
        ///     inset-inline-start: 50%;
        /// }
        /// ```
        Start1over2 => "start-1/2",
        /// ```css
        /// {
        ///     inset-inline-start: 33.333333%;
        /// }
        /// ```
        Start1over3 => "start-1/3",
        /// ```css
        /// {
        ///     inset-inline-start: 66.666667%;
        /// }
        /// ```
        Start2over3 => "start-2/3",
        /// ```css
        /// {
        ///     inset-inline-start: 25%;
        /// }
        /// ```
        Start1over4 => "start-1/4",
        /// ```css
        /// {
        ///     inset-inline-start: 50%;
        /// }
        /// ```
        Start2over4 => "start-2/4",
        /// ```css
        /// {
        ///     inset-inline-start: 75%;
        /// }
        /// ```
        Start3over4 => "start-3/4",
        /// ```css
        /// {
        ///     inset-inline-start: 100%;
        /// }
        /// ```
        StartFull => "start-full",
        /// ```css
        /// {
        ///     inset-inline-end: auto;
        /// }
        /// ```
        EndAuto => "end-auto",
        /// ```css
        /// {
        ///     inset-inline-end: 50%;
        /// }
        /// ```
        End1over2 => "end-1/2",
        /// ```css
        /// {
        ///     inset-inline-end: 33.333333%;
        /// }
        /// ```
        End1over3 => "end-1/3",
        /// ```css
        /// {
        ///     inset-inline-end: 66.666667%;
        /// }
        /// ```
        End2over3 => "end-2/3",
        /// ```css
        /// {
        ///     inset-inline-end: 25%;
        /// }
        /// ```
        End1over4 => "end-1/4",
        /// ```css
        /// {
        ///     inset-inline-end: 50%;
        /// }
        /// ```
        End2over4 => "end-2/4",
        /// ```css
        /// {
        ///     inset-inline-end: 75%;
        /// }
        /// ```
        End3over4 => "end-3/4",
        /// ```css
        /// {
        ///     inset-inline-end: 100%;
        /// }
        /// ```
        EndFull => "end-full",
        /// ```css
        /// {
        ///     top: auto;
        /// }
        /// ```
        TopAuto => "top-auto",
        /// ```css
        /// {
        ///     top: 50%;
        /// }
        /// ```
        Top1over2 => "top-1/2",
        /// ```css
        /// {
        ///     top: 33.333333%;
        /// }
        /// ```
        Top1over3 => "top-1/3",
        /// ```css
        /// {
        ///     top: 66.666667%;
        /// }
        /// ```
        Top2over3 => "top-2/3",
        /// ```css
        /// {
        ///     top: 25%;
        /// }
        /// ```
        Top1over4 => "top-1/4",
        /// ```css
        /// {
        ///     top: 50%;
        /// }
        /// ```
        Top2over4 => "top-2/4",
        /// ```css
        /// {
        ///     top: 75%;
        /// }
        /// ```
        Top3over4 => "top-3/4",
        /// ```css
        /// {
        ///     top: 100%;
        /// }
        /// ```
        TopFull => "top-full",
        /// ```css
        /// {
        ///     right: auto;
        /// }
        /// ```
        RightAuto => "right-auto",
        /// ```css
        /// {
        ///     right: 50%;
        /// }
        /// ```
        Right1over2 => "right-1/2",
        /// ```css
        /// {
        ///     right: 33.333333%;
        /// }
        /// ```
        Right1over3 => "right-1/3",
        /// ```css
        /// {
        ///     right: 66.666667%;
        /// }
        /// ```
        Right2over3 => "right-2/3",
        /// ```css
        /// {
        ///     right: 25%;
        /// }
        /// ```
        Right1over4 => "right-1/4",
        /// ```css
        /// {
        ///     right: 50%;
        /// }
        /// ```
        Right2over4 => "right-2/4",
        /// ```css
        /// {
        ///     right: 75%;
        /// }
        /// ```
        Right3over4 => "right-3/4",
        /// ```css
        /// {
        ///     right: 100%;
        /// }
        /// ```
        RightFull => "right-full",
        /// ```css
        /// {
        ///     bottom: auto;
        /// }
        /// ```
        BottomAuto => "bottom-auto",
        /// ```css
        /// {
        ///     bottom: 50%;
        /// }
        /// ```
        Bottom1over2 => "bottom-1/2",
        /// ```css
        /// {
        ///     bottom: 33.333333%;
        /// }
        /// ```
        Bottom1over3 => "bottom-1/3",
        /// ```css
        /// {
        ///     bottom: 66.666667%;
        /// }
        /// ```
        Bottom2over3 => "bottom-2/3",
        /// ```css
        /// {
        ///     bottom: 25%;
        /// }
        /// ```
        Bottom1over4 => "bottom-1/4",
        /// ```css
        /// {
        ///     bottom: 50%;
        /// }
        /// ```
        Bottom2over4 => "bottom-2/4",
        /// ```css
        /// {
        ///     bottom: 75%;
        /// }
        /// ```
        Bottom3over4 => "bottom-3/4",
        /// ```css
        /// {
        ///     bottom: 100%;
        /// }
        /// ```
        BottomFull => "bottom-full",
        /// ```css
        /// {
        ///     left: auto;
        /// }
        /// ```
        LeftAuto => "left-auto",
        /// ```css
        /// {
        ///     left: 50%;
        /// }
        /// ```
        Left1over2 => "left-1/2",
        /// ```css
        /// {
        ///     left: 33.333333%;
        /// }
        /// ```
        Left1over3 => "left-1/3",
        /// ```css
        /// {
        ///     left: 66.666667%;
        /// }
        /// ```
        Left2over3 => "left-2/3",
        /// ```css
        /// {
        ///     left: 25%;
        /// }
        /// ```
        Left1over4 => "left-1/4",
        /// ```css
        /// {
        ///     left: 50%;
        /// }
        /// ```
        Left2over4 => "left-2/4",
        /// ```css
        /// {
        ///     left: 75%;
        /// }
        /// ```
        Left3over4 => "left-3/4",
        /// ```css
        /// {
        ///     left: 100%;
        /// }
        /// ```
        LeftFull => "left-full",
    }
    /// Utilities for controlling the visibility of an element.
    ///
    /// <https://tailwindcss.com/docs/visibility>
    Visibility {
        /// ```css
        /// {
        ///     visibility: visible;
        /// }
        /// ```
        Visible => "visible",
        /// ```css
        /// {
        ///     visibility: hidden;
        /// }
        /// ```
        Invisible => "invisible",
        /// ```css
        /// {
        ///     visibility: collapse;
        /// }
        /// ```
        Collapse => "collapse",
    }
    /// Utilities for controlling the stack order of an element.
    ///
    /// <https://tailwindcss.com/docs/z-index>
    ZIndex {
        /// ```css
        /// {
        ///     z-index: 0;
        /// }
        /// ```
        _0 => "z-0",
        /// ```css
        /// {
        ///     z-index: 10;
        /// }
        /// ```
        _10 => "z-10",
        /// ```css
        /// {
        ///     z-index: 20;
        /// }
        /// ```
        _20 => "z-20",
        /// ```css
        /// {
        ///     z-index: 30;
        /// }
        /// ```
        _30 => "z-30",
        /// ```css
        /// {
        ///     z-index: 40;
        /// }
        /// ```
        _40 => "z-40",
        /// ```css
        /// {
        ///     z-index: 50;
        /// }
        /// ```
        _50 => "z-50",
        /// ```css
        /// {
        ///     z-index: auto;
        /// }
        /// ```
        Auto => "z-auto",
    }
);
