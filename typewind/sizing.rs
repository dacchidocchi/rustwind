def_types!(
    /// Utilities for setting the width of an element.
    ///
    /// <https://tailwindcss.com/docs/width>
    Width {
        /// ```css
        /// {
        ///     width: 0px;
        /// }
        /// ```
        _0 => "w-0",
        /// ```css
        /// {
        ///     width: 1px;
        /// }
        /// ```
        _Px => "w-px",
        /// ```css
        /// {
        ///     width: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "w-0.5",
        /// ```css
        /// {
        ///     width: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "w-1",
        /// ```css
        /// {
        ///     width: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "w-1.5",
        /// ```css
        /// {
        ///     width: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "w-2",
        /// ```css
        /// {
        ///     width: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "w-2.5",
        /// ```css
        /// {
        ///     width: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "w-3",
        /// ```css
        /// {
        ///     width: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "w-3.5",
        /// ```css
        /// {
        ///     width: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "w-4",
        /// ```css
        /// {
        ///     width: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "w-5",
        /// ```css
        /// {
        ///     width: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "w-6",
        /// ```css
        /// {
        ///     width: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "w-7",
        /// ```css
        /// {
        ///     width: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "w-8",
        /// ```css
        /// {
        ///     width: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "w-9",
        /// ```css
        /// {
        ///     width: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "w-10",
        /// ```css
        /// {
        ///     width: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "w-11",
        /// ```css
        /// {
        ///     width: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "w-12",
        /// ```css
        /// {
        ///     width: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "w-14",
        /// ```css
        /// {
        ///     width: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "w-16",
        /// ```css
        /// {
        ///     width: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "w-20",
        /// ```css
        /// {
        ///     width: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "w-24",
        /// ```css
        /// {
        ///     width: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "w-28",
        /// ```css
        /// {
        ///     width: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "w-32",
        /// ```css
        /// {
        ///     width: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "w-36",
        /// ```css
        /// {
        ///     width: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "w-40",
        /// ```css
        /// {
        ///     width: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "w-44",
        /// ```css
        /// {
        ///     width: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "w-48",
        /// ```css
        /// {
        ///     width: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "w-52",
        /// ```css
        /// {
        ///     width: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "w-56",
        /// ```css
        /// {
        ///     width: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "w-60",
        /// ```css
        /// {
        ///     width: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "w-64",
        /// ```css
        /// {
        ///     width: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "w-72",
        /// ```css
        /// {
        ///     width: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "w-80",
        /// ```css
        /// {
        ///     width: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "w-96",
        /// ```css
        /// {
        ///     width: auto;
        /// }
        /// ```
        Auto => "w-auto",
        /// ```css
        /// {
        ///     width: 50%;
        /// }
        /// ```
        _1over2 => "w-1/2",
        /// ```css
        /// {
        ///     width: 33.333333%;
        /// }
        /// ```
        _1over3 => "w-1/3",
        /// ```css
        /// {
        ///     width: 66.666667%;
        /// }
        /// ```
        _2over3 => "w-2/3",
        /// ```css
        /// {
        ///     width: 25%;
        /// }
        /// ```
        _1over4 => "w-1/4",
        /// ```css
        /// {
        ///     width: 50%;
        /// }
        /// ```
        _2over4 => "w-2/4",
        /// ```css
        /// {
        ///     width: 75%;
        /// }
        /// ```
        _3over4 => "w-3/4",
        /// ```css
        /// {
        ///     width: 20%;
        /// }
        /// ```
        _1over5 => "w-1/5",
        /// ```css
        /// {
        ///     width: 40%;
        /// }
        /// ```
        _2over5 => "w-2/5",
        /// ```css
        /// {
        ///     width: 60%;
        /// }
        /// ```
        _3over5 => "w-3/5",
        /// ```css
        /// {
        ///     width: 80%;
        /// }
        /// ```
        _4over5 => "w-4/5",
        /// ```css
        /// {
        ///     width: 16.666667%;
        /// }
        /// ```
        _1over6 => "w-1/6",
        /// ```css
        /// {
        ///     width: 33.333333%;
        /// }
        /// ```
        _2over6 => "w-2/6",
        /// ```css
        /// {
        ///     width: 50%;
        /// }
        /// ```
        _3over6 => "w-3/6",
        /// ```css
        /// {
        ///     width: 66.666667%;
        /// }
        /// ```
        _4over6 => "w-4/6",
        /// ```css
        /// {
        ///     width: 83.333333%;
        /// }
        /// ```
        _5over6 => "w-5/6",
        /// ```css
        /// {
        ///     width: 8.333333%;
        /// }
        /// ```
        _1over12 => "w-1/12",
        /// ```css
        /// {
        ///     width: 16.666667%;
        /// }
        /// ```
        _2over12 => "w-2/12",
        /// ```css
        /// {
        ///     width: 25%;
        /// }
        /// ```
        _3over12 => "w-3/12",
        /// ```css
        /// {
        ///     width: 33.333333%;
        /// }
        /// ```
        _4over12 => "w-4/12",
        /// ```css
        /// {
        ///     width: 41.666667%;
        /// }
        /// ```
        _5over12 => "w-5/12",
        /// ```css
        /// {
        ///     width: 50%;
        /// }
        /// ```
        _6over12 => "w-6/12",
        /// ```css
        /// {
        ///     width: 58.333333%;
        /// }
        /// ```
        _7over12 => "w-7/12",
        /// ```css
        /// {
        ///     width: 66.666667%;
        /// }
        /// ```
        _8over12 => "w-8/12",
        /// ```css
        /// {
        ///     width: 75%;
        /// }
        /// ```
        _9over12 => "w-9/12",
        /// ```css
        /// {
        ///     width: 83.333333%;
        /// }
        /// ```
        _10over12 => "w-10/12",
        /// ```css
        /// {
        ///     width: 91.666667%;
        /// }
        /// ```
        _11over12 => "w-11/12",
        /// ```css
        /// {
        ///     width: 100%;
        /// }
        /// ```
        Full => "w-full",
        /// ```css
        /// {
        ///     width: 100vw;
        /// }
        /// ```
        Screen => "w-screen",
        /// ```css
        /// {
        ///     width: 100svw;
        /// }
        /// ```
        Svw => "w-svw",
        /// ```css
        /// {
        ///     width: 100lvw;
        /// }
        /// ```
        Lvw => "w-lvw",
        /// ```css
        /// {
        ///     width: 100dvw;
        /// }
        /// ```
        Dvw => "w-dvw",
        /// ```css
        /// {
        ///     width: min-content;
        /// }
        /// ```
        Min => "w-min",
        /// ```css
        /// {
        ///     width: max-content;
        /// }
        /// ```
        Max => "w-max",
        /// ```css
        /// {
        ///     width: fit-content;
        /// }
        /// ```
        Fit => "w-fit",
    }
    /// Utilities for setting the minimum width of an element.
    ///
    /// <https://tailwindcss.com/docs/min-width>
    MinWidth {
        /// ```css
        /// {
        ///     min-width: 0px;
        /// }
        /// ```
        _0 => "min-w-0",
        /// ```css
        /// {
        ///     min-width: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "min-w-1",
        /// ```css
        /// {
        ///     min-width: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "min-w-2",
        /// ```css
        /// {
        ///     min-width: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "min-w-3",
        /// ```css
        /// {
        ///     min-width: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "min-w-4",
        /// ```css
        /// {
        ///     min-width: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "min-w-5",
        /// ```css
        /// {
        ///     min-width: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "min-w-6",
        /// ```css
        /// {
        ///     min-width: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "min-w-7",
        /// ```css
        /// {
        ///     min-width: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "min-w-8",
        /// ```css
        /// {
        ///     min-width: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "min-w-9",
        /// ```css
        /// {
        ///     min-width: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "min-w-10",
        /// ```css
        /// {
        ///     min-width: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "min-w-11",
        /// ```css
        /// {
        ///     min-width: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "min-w-12",
        /// ```css
        /// {
        ///     min-width: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "min-w-14",
        /// ```css
        /// {
        ///     min-width: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "min-w-16",
        /// ```css
        /// {
        ///     min-width: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "min-w-20",
        /// ```css
        /// {
        ///     min-width: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "min-w-24",
        /// ```css
        /// {
        ///     min-width: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "min-w-28",
        /// ```css
        /// {
        ///     min-width: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "min-w-32",
        /// ```css
        /// {
        ///     min-width: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "min-w-36",
        /// ```css
        /// {
        ///     min-width: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "min-w-40",
        /// ```css
        /// {
        ///     min-width: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "min-w-44",
        /// ```css
        /// {
        ///     min-width: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "min-w-48",
        /// ```css
        /// {
        ///     min-width: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "min-w-52",
        /// ```css
        /// {
        ///     min-width: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "min-w-56",
        /// ```css
        /// {
        ///     min-width: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "min-w-60",
        /// ```css
        /// {
        ///     min-width: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "min-w-64",
        /// ```css
        /// {
        ///     min-width: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "min-w-72",
        /// ```css
        /// {
        ///     min-width: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "min-w-80",
        /// ```css
        /// {
        ///     min-width: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "min-w-96",
        /// ```css
        /// {
        ///     min-width: 1px;
        /// }
        /// ```
        Px => "min-w-px",
        /// ```css
        /// {
        ///     min-width: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "min-w-0.5",
        /// ```css
        /// {
        ///     min-width: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "min-w-1.5",
        /// ```css
        /// {
        ///     min-width: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "min-w-2.5",
        /// ```css
        /// {
        ///     min-width: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "min-w-3.5",
        /// ```css
        /// {
        ///     min-width: 100%;
        /// }
        /// ```
        Full => "min-w-full",
        /// ```css
        /// {
        ///     min-width: min-content;
        /// }
        /// ```
        Min => "min-w-min",
        /// ```css
        /// {
        ///     min-width: max-content;
        /// }
        /// ```
        Max => "min-w-max",
        /// ```css
        /// {
        ///     min-width: fit-content;
        /// }
        /// ```
        Fit => "min-w-fit",
    }
    /// Utilities for setting the maximum width of an element.
    ///
    /// <https://tailwindcss.com/docs/max-width>
    MaxWidth {
        /// ```css
        /// {
        ///     max-width: 0px;
        /// }
        /// ```
        _0 => "max-w-0",
        /// ```css
        /// {
        ///     max-width: 1px;
        /// }
        /// ```
        Px => "max-w-px",
        /// ```css
        /// {
        ///     max-width: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "max-w-0.5",
        /// ```css
        /// {
        ///     max-width: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "max-w-1",
        /// ```css
        /// {
        ///     max-width: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "max-w-1.5",
        /// ```css
        /// {
        ///     max-width: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "max-w-2",
        /// ```css
        /// {
        ///     max-width: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "max-w-2.5",
        /// ```css
        /// {
        ///     max-width: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "max-w-3",
        /// ```css
        /// {
        ///     max-width: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "max-w-3.5",
        /// ```css
        /// {
        ///     max-width: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "max-w-4",
        /// ```css
        /// {
        ///     max-width: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "max-w-5",
        /// ```css
        /// {
        ///     max-width: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "max-w-6",
        /// ```css
        /// {
        ///     max-width: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "max-w-7",
        /// ```css
        /// {
        ///     max-width: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "max-w-8",
        /// ```css
        /// {
        ///     max-width: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "max-w-9",
        /// ```css
        /// {
        ///     max-width: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "max-w-10",
        /// ```css
        /// {
        ///     max-width: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "max-w-11",
        /// ```css
        /// {
        ///     max-width: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "max-w-12",
        /// ```css
        /// {
        ///     max-width: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "max-w-14",
        /// ```css
        /// {
        ///     max-width: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "max-w-16",
        /// ```css
        /// {
        ///     max-width: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "max-w-20",
        /// ```css
        /// {
        ///     max-width: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "max-w-24",
        /// ```css
        /// {
        ///     max-width: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "max-w-28",
        /// ```css
        /// {
        ///     max-width: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "max-w-32",
        /// ```css
        /// {
        ///     max-width: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "max-w-36",
        /// ```css
        /// {
        ///     max-width: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "max-w-40",
        /// ```css
        /// {
        ///     max-width: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "max-w-44",
        /// ```css
        /// {
        ///     max-width: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "max-w-48",
        /// ```css
        /// {
        ///     max-width: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "max-w-52",
        /// ```css
        /// {
        ///     max-width: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "max-w-56",
        /// ```css
        /// {
        ///     max-width: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "max-w-60",
        /// ```css
        /// {
        ///     max-width: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "max-w-64",
        /// ```css
        /// {
        ///     max-width: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "max-w-72",
        /// ```css
        /// {
        ///     max-width: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "max-w-80",
        /// ```css
        /// {
        ///     max-width: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "max-w-96",
        /// ```css
        /// {
        ///     max-width: none;
        /// }
        /// ```
        None => "max-w-none",
        /// ```css
        /// {
        ///     max-width: 20rem; /* 320px */
        /// }
        /// ```
        Xs => "max-w-xs",
        /// ```css
        /// {
        ///     max-width: 24rem; /* 384px */
        /// }
        /// ```
        Sm => "max-w-sm",
        /// ```css
        /// {
        ///     max-width: 28rem; /* 448px */
        /// }
        /// ```
        Md => "max-w-md",
        /// ```css
        /// {
        ///     max-width: 32rem; /* 512px */
        /// }
        /// ```
        Lg => "max-w-lg",
        /// ```css
        /// {
        ///     max-width: 36rem; /* 576px */
        /// }
        /// ```
        Xl => "max-w-xl",
        /// ```css
        /// {
        ///     max-width: 42rem; /* 672px */
        /// }
        /// ```
        _2xl => "max-w-2xl",
        /// ```css
        /// {
        ///     max-width: 48rem; /* 768px */
        /// }
        /// ```
        _3xl => "max-w-3xl",
        /// ```css
        /// {
        ///     max-width: 56rem; /* 896px */
        /// }
        /// ```
        _4xl => "max-w-4xl",
        /// ```css
        /// {
        ///     max-width: 64rem; /* 1024px */
        /// }
        /// ```
        _5xl => "max-w-5xl",
        /// ```css
        /// {
        ///     max-width: 72rem; /* 1152px */
        /// }
        /// ```
        _6xl => "max-w-6xl",
        /// ```css
        /// {
        ///     max-width: 80rem; /* 1280px */
        /// }
        /// ```
        _7xl => "max-w-7xl",
        /// ```css
        /// {
        ///     max-width: 100%;
        /// }
        /// ```
        Full => "max-w-full",
        /// ```css
        /// {
        ///     max-width: min-content;
        /// }
        /// ```
        Min => "max-w-min",
        /// ```css
        /// {
        ///     max-width: max-content;
        /// }
        /// ```
        Max => "max-w-max",
        /// ```css
        /// {
        ///     max-width: fit-content;
        /// }
        /// ```
        Fit => "max-w-fit",
        /// ```css
        /// {
        ///     max-width: 65ch;
        /// }
        /// ```
        Prose => "max-w-prose",
        /// ```css
        /// {
        ///     max-width: 640px;
        /// }
        /// ```
        ScreenSm => "max-w-screen-sm",
        /// ```css
        /// {
        ///     max-width: 768px;
        /// }
        /// ```
        ScreenMd => "max-w-screen-md",
        /// ```css
        /// {
        ///     max-width: 1024px;
        /// }
        /// ```
        ScreenLg => "max-w-screen-lg",
        /// ```css
        /// {
        ///     max-width: 1280px;
        /// }
        /// ```
        ScreenXl => "max-w-screen-xl",
        /// ```css
        /// {
        ///     max-width: 1536px;
        /// }
        /// ```
        Screen2xl => "max-w-screen-2xl",
    }
    /// Utilities for setting the height of an element.
    ///
    /// <https://tailwindcss.com/docs/height>
    Height {
        /// ```css
        /// {
        ///     height: 0px;
        /// }
        /// ```
        _0 => "h-0",
        /// ```css
        /// {
        ///     height: 1px;
        /// }
        /// ```
        Px => "h-px",
        /// ```css
        /// {
        ///     height: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "h-0.5",
        /// ```css
        /// {
        ///     height: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "h-1",
        /// ```css
        /// {
        ///     height: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "h-1.5",
        /// ```css
        /// {
        ///     height: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "h-2",
        /// ```css
        /// {
        ///     height: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "h-2.5",
        /// ```css
        /// {
        ///     height: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "h-3",
        /// ```css
        /// {
        ///     height: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "h-3.5",
        /// ```css
        /// {
        ///     height: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "h-4",
        /// ```css
        /// {
        ///     height: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "h-5",
        /// ```css
        /// {
        ///     height: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "h-6",
        /// ```css
        /// {
        ///     height: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "h-7",
        /// ```css
        /// {
        ///     height: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "h-8",
        /// ```css
        /// {
        ///     height: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "h-9",
        /// ```css
        /// {
        ///     height: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "h-10",
        /// ```css
        /// {
        ///     height: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "h-11",
        /// ```css
        /// {
        ///     height: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "h-12",
        /// ```css
        /// {
        ///     height: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "h-14",
        /// ```css
        /// {
        ///     height: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "h-16",
        /// ```css
        /// {
        ///     height: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "h-20",
        /// ```css
        /// {
        ///     height: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "h-24",
        /// ```css
        /// {
        ///     height: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "h-28",
        /// ```css
        /// {
        ///     height: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "h-32",
        /// ```css
        /// {
        ///     height: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "h-36",
        /// ```css
        /// {
        ///     height: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "h-40",
        /// ```css
        /// {
        ///     height: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "h-44",
        /// ```css
        /// {
        ///     height: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "h-48",
        /// ```css
        /// {
        ///     height: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "h-52",
        /// ```css
        /// {
        ///     height: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "h-56",
        /// ```css
        /// {
        ///     height: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "h-60",
        /// ```css
        /// {
        ///     height: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "h-64",
        /// ```css
        /// {
        ///     height: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "h-72",
        /// ```css
        /// {
        ///     height: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "h-80",
        /// ```css
        /// {
        ///     height: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "h-96",
        /// ```css
        /// {
        ///     height: auto;
        /// }
        /// ```
        Auto => "h-auto",
        /// ```css
        /// {
        ///     height: 50%;
        /// }
        /// ```
        _1over2 => "h-1/2",
        /// ```css
        /// {
        ///     height: 33.333333%;
        /// }
        /// ```
        _1over3 => "h-1/3",
        /// ```css
        /// {
        ///     height: 66.666667%;
        /// }
        /// ```
        _2over3 => "h-2/3",
        /// ```css
        /// {
        ///     height: 25%;
        /// }
        /// ```
        _1over4 => "h-1/4",
        /// ```css
        /// {
        ///     height: 50%;
        /// }
        /// ```
        _2over4 => "h-2/4",
        /// ```css
        /// {
        ///     height: 75%;
        /// }
        /// ```
        _3over4 => "h-3/4",
        /// ```css
        /// {
        ///     height: 20%;
        /// }
        /// ```
        _1over5 => "h-1/5",
        /// ```css
        /// {
        ///     height: 40%;
        /// }
        /// ```
        _2over5 => "h-2/5",
        /// ```css
        /// {
        ///     height: 60%;
        /// }
        /// ```
        _3over5 => "h-3/5",
        /// ```css
        /// {
        ///     height: 80%;
        /// }
        /// ```
        _4over5 => "h-4/5",
        /// ```css
        /// {
        ///     height: 16.666667%;
        /// }
        /// ```
        _1over6 => "h-1/6",
        /// ```css
        /// {
        ///     height: 33.333333%;
        /// }
        /// ```
        _2over6 => "h-2/6",
        /// ```css
        /// {
        ///     height: 50%;
        /// }
        /// ```
        _3over6 => "h-3/6",
        /// ```css
        /// {
        ///     height: 66.666667%;
        /// }
        /// ```
        _4over6 => "h-4/6",
        /// ```css
        /// {
        ///     height: 83.333333%;
        /// }
        /// ```
        _5over6 => "h-5/6",
        /// ```css
        /// {
        ///     height: 100%;
        /// }
        /// ```
        Full => "h-full",
        /// ```css
        /// {
        ///     height: 100vh;
        /// }
        /// ```
        Screen => "h-screen",
        /// ```css
        /// {
        ///     height: 100svh;
        /// }
        /// ```
        Svh => "h-svh",
        /// ```css
        /// {
        ///     height: 100lvh;
        /// }
        /// ```
        Lvh => "h-lvh",
        /// ```css
        /// {
        ///     height: 100dvh;
        /// }
        /// ```
        Dvh => "h-dvh",
        /// ```css
        /// {
        ///     height: min-content;
        /// }
        /// ```
        Min => "h-min",
        /// ```css
        /// {
        ///     height: max-content;
        /// }
        /// ```
        Max => "h-max",
        /// ```css
        /// {
        ///     height: fit-content;
        /// }
        /// ```
        Fit => "h-fit",
    }
    /// Utilities for setting the minimum height of an element.
    ///
    /// <https://tailwindcss.com/docs/min-height>
    MinHeight {
        /// ```css
        /// {
        ///     min-height: 0px;
        /// }
        /// ```
        _0 => "min-h-0",
        /// ```css
        /// {
        ///     min-height: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "min-h-1",
        /// ```css
        /// {
        ///     min-height: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "min-h-2",
        /// ```css
        /// {
        ///     min-height: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "min-h-3",
        /// ```css
        /// {
        ///     min-height: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "min-h-4",
        /// ```css
        /// {
        ///     min-height: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "min-h-5",
        /// ```css
        /// {
        ///     min-height: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "min-h-6",
        /// ```css
        /// {
        ///     min-height: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "min-h-7",
        /// ```css
        /// {
        ///     min-height: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "min-h-8",
        /// ```css
        /// {
        ///     min-height: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "min-h-9",
        /// ```css
        /// {
        ///     min-height: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "min-h-10",
        /// ```css
        /// {
        ///     min-height: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "min-h-11",
        /// ```css
        /// {
        ///     min-height: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "min-h-12",
        /// ```css
        /// {
        ///     min-height: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "min-h-14",
        /// ```css
        /// {
        ///     min-height: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "min-h-16",
        /// ```css
        /// {
        ///     min-height: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "min-h-20",
        /// ```css
        /// {
        ///     min-height: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "min-h-24",
        /// ```css
        /// {
        ///     min-height: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "min-h-28",
        /// ```css
        /// {
        ///     min-height: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "min-h-32",
        /// ```css
        /// {
        ///     min-height: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "min-h-36",
        /// ```css
        /// {
        ///     min-height: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "min-h-40",
        /// ```css
        /// {
        ///     min-height: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "min-h-44",
        /// ```css
        /// {
        ///     min-height: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "min-h-48",
        /// ```css
        /// {
        ///     min-height: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "min-h-52",
        /// ```css
        /// {
        ///     min-height: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "min-h-56",
        /// ```css
        /// {
        ///     min-height: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "min-h-60",
        /// ```css
        /// {
        ///     min-height: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "min-h-64",
        /// ```css
        /// {
        ///     min-height: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "min-h-72",
        /// ```css
        /// {
        ///     min-height: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "min-h-80",
        /// ```css
        /// {
        ///     min-height: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "min-h-96",
        /// ```css
        /// {
        ///     min-height: 1px;
        /// }
        /// ```
        Px => "min-h-px",
        /// ```css
        /// {
        ///     min-height: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "min-h-0.5",
        /// ```css
        /// {
        ///     min-height: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "min-h-1.5",
        /// ```css
        /// {
        ///     min-height: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "min-h-2.5",
        /// ```css
        /// {
        ///     min-height: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "min-h-3.5",
        /// ```css
        /// {
        ///     min-height: 100%;
        /// }
        /// ```
        Full => "min-h-full",
        /// ```css
        /// {
        ///     min-height: 100vh;
        /// }
        /// ```
        Screen => "min-h-screen",
        /// ```css
        /// {
        ///     min-height: 100svh;
        /// }
        /// ```
        Svh => "min-h-svh",
        /// ```css
        /// {
        ///     min-height: 100lvh;
        /// }
        /// ```
        Lvh => "min-h-lvh",
        /// ```css
        /// {
        ///     min-height: 100dvh;
        /// }
        /// ```
        Dvh => "min-h-dvh",
        /// ```css
        /// {
        ///     min-height: min-content;
        /// }
        /// ```
        Min => "min-h-min",
        /// ```css
        /// {
        ///     min-height: max-content;
        /// }
        /// ```
        Max => "min-h-max",
        /// ```css
        /// {
        ///     min-height: fit-content;
        /// }
        /// ```
        Fit => "min-h-fit",
    }
    /// Utilities for setting the maximum height of an element.
    ///
    /// <https://tailwindcss.com/docs/max-height>
    MaxHeight {
        /// ```css
        /// {
        ///     max-height: 0px;
        /// }
        /// ```
        _0 => "max-h-0",
        /// ```css
        /// {
        ///     max-height: 1px;
        /// }
        /// ```
        Px => "max-h-px",
        /// ```css
        /// {
        ///     max-height: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "max-h-0.5",
        /// ```css
        /// {
        ///     max-height: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "max-h-1",
        /// ```css
        /// {
        ///     max-height: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "max-h-1.5",
        /// ```css
        /// {
        ///     max-height: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "max-h-2",
        /// ```css
        /// {
        ///     max-height: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "max-h-2.5",
        /// ```css
        /// {
        ///     max-height: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "max-h-3",
        /// ```css
        /// {
        ///     max-height: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "max-h-3.5",
        /// ```css
        /// {
        ///     max-height: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "max-h-4",
        /// ```css
        /// {
        ///     max-height: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "max-h-5",
        /// ```css
        /// {
        ///     max-height: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "max-h-6",
        /// ```css
        /// {
        ///     max-height: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "max-h-7",
        /// ```css
        /// {
        ///     max-height: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "max-h-8",
        /// ```css
        /// {
        ///     max-height: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "max-h-9",
        /// ```css
        /// {
        ///     max-height: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "max-h-10",
        /// ```css
        /// {
        ///     max-height: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "max-h-11",
        /// ```css
        /// {
        ///     max-height: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "max-h-12",
        /// ```css
        /// {
        ///     max-height: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "max-h-14",
        /// ```css
        /// {
        ///     max-height: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "max-h-16",
        /// ```css
        /// {
        ///     max-height: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "max-h-20",
        /// ```css
        /// {
        ///     max-height: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "max-h-24",
        /// ```css
        /// {
        ///     max-height: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "max-h-28",
        /// ```css
        /// {
        ///     max-height: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "max-h-32",
        /// ```css
        /// {
        ///     max-height: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "max-h-36",
        /// ```css
        /// {
        ///     max-height: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "max-h-40",
        /// ```css
        /// {
        ///     max-height: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "max-h-44",
        /// ```css
        /// {
        ///     max-height: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "max-h-48",
        /// ```css
        /// {
        ///     max-height: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "max-h-52",
        /// ```css
        /// {
        ///     max-height: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "max-h-56",
        /// ```css
        /// {
        ///     max-height: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "max-h-60",
        /// ```css
        /// {
        ///     max-height: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "max-h-64",
        /// ```css
        /// {
        ///     max-height: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "max-h-72",
        /// ```css
        /// {
        ///     max-height: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "max-h-80",
        /// ```css
        /// {
        ///     max-height: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "max-h-96",
        /// ```css
        /// {
        ///     max-height: none;
        /// }
        /// ```
        None => "max-h-none",
        /// ```css
        /// {
        ///     max-height: 100%;
        /// }
        /// ```
        Full => "max-h-full",
        /// ```css
        /// {
        ///     max-height: 100vh;
        /// }
        /// ```
        Screen => "max-h-screen",
        /// ```css
        /// {
        ///     max-height: 100svh;
        /// }
        /// ```
        Svh => "max-h-svh",
        /// ```css
        /// {
        ///     max-height: 100lvh;
        /// }
        /// ```
        Lvh => "max-h-lvh",
        /// ```css
        /// {
        ///     max-height: 100dvh;
        /// }
        /// ```
        Dvh => "max-h-dvh",
        /// ```css
        /// {
        ///     max-height: min-content;
        /// }
        /// ```
        Min => "max-h-min",
        /// ```css
        /// {
        ///     max-height: max-content;
        /// }
        /// ```
        Max => "max-h-max",
        /// ```css
        /// {
        ///     max-height: fit-content;
        /// }
        /// ```
        Fit => "max-h-fit",
    }
    /// Utilities for setting the width and height of an element at the same time.
    ///
    /// <https://tailwindcss.com/docs/size>
    Size {
        /// ```css
        /// {
        ///     width: 0px;
        ///     height: 0px;
        /// }
        /// ```
        _0 => "size-0",
        /// ```css
        /// {
        ///     width: 1px;
        ///     height: 1px;
        /// }
        /// ```
        Px => "size-px",
        /// ```css
        /// {
        ///     width: 0.125rem; /* 2px */
        ///     height: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "size-0.5",
        /// ```css
        /// {
        ///     width: 0.25rem; /* 4px */
        ///     height: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "size-1",
        /// ```css
        /// {
        ///     width: 0.375rem; /* 6px */
        ///     height: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "size-1.5",
        /// ```css
        /// {
        ///     width: 0.5rem; /* 8px */
        ///     height: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "size-2",
        /// ```css
        /// {
        ///     width: 0.625rem; /* 10px */
        ///     height: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "size-2.5",
        /// ```css
        /// {
        ///     width: 0.75rem; /* 12px */
        ///     height: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "size-3",
        /// ```css
        /// {
        ///     width: 0.875rem; /* 14px */
        ///     height: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "size-3.5",
        /// ```css
        /// {
        ///     width: 1rem; /* 16px */
        ///     height: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "size-4",
        /// ```css
        /// {
        ///     width: 1.25rem; /* 20px */
        ///     height: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "size-5",
        /// ```css
        /// {
        ///     width: 1.5rem; /* 24px */
        ///     height: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "size-6",
        /// ```css
        /// {
        ///     width: 1.75rem; /* 28px */
        ///     height: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "size-7",
        /// ```css
        /// {
        ///     width: 2rem; /* 32px */
        ///     height: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "size-8",
        /// ```css
        /// {
        ///     width: 2.25rem; /* 36px */
        ///     height: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "size-9",
        /// ```css
        /// {
        ///     width: 2.5rem; /* 40px */
        ///     height: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "size-10",
        /// ```css
        /// {
        ///     width: 2.75rem; /* 44px */
        ///     height: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "size-11",
        /// ```css
        /// {
        ///     width: 3rem; /* 48px */
        ///     height: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "size-12",
        /// ```css
        /// {
        ///     width: 3.5rem; /* 56px */
        ///     height: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "size-14",
        /// ```css
        /// {
        ///     width: 4rem; /* 64px */
        ///     height: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "size-16",
        /// ```css
        /// {
        ///     width: 5rem; /* 80px */
        ///     height: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "size-20",
        /// ```css
        /// {
        ///     width: 6rem; /* 96px */
        ///     height: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "size-24",
        /// ```css
        /// {
        ///     width: 7rem; /* 112px */
        ///     height: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "size-28",
        /// ```css
        /// {
        ///     width: 8rem; /* 128px */
        ///     height: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "size-32",
        /// ```css
        /// {
        ///     width: 9rem; /* 144px */
        ///     height: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "size-36",
        /// ```css
        /// {
        ///     width: 10rem; /* 160px */
        ///     height: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "size-40",
        /// ```css
        /// {
        ///     width: 11rem; /* 176px */
        ///     height: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "size-44",
        /// ```css
        /// {
        ///     width: 12rem; /* 192px */
        ///     height: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "size-48",
        /// ```css
        /// {
        ///     width: 13rem; /* 208px */
        ///     height: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "size-52",
        /// ```css
        /// {
        ///     width: 14rem; /* 224px */
        ///     height: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "size-56",
        /// ```css
        /// {
        ///     width: 15rem; /* 240px */
        ///     height: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "size-60",
        /// ```css
        /// {
        ///     width: 16rem; /* 256px */
        ///     height: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "size-64",
        /// ```css
        /// {
        ///     width: 18rem; /* 288px */
        ///     height: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "size-72",
        /// ```css
        /// {
        ///     width: 20rem; /* 320px */
        ///     height: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "size-80",
        /// ```css
        /// {
        ///     width: 24rem; /* 384px */
        ///     height: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "size-96",
        /// ```css
        /// {
        ///     width: auto;
        ///     height: auto;
        /// }
        /// ```
        Auto => "size-auto",
        /// ```css
        /// {
        ///     width: 50%;
        ///     height: 50%;
        /// }
        /// ```
        _1over2 => "size-1/2",
        /// ```css
        /// {
        ///     width: 33.333333%;
        ///     height: 33.333333%;
        /// }
        /// ```
        _1over3 => "size-1/3",
        /// ```css
        /// {
        ///     width: 66.666667%;
        ///     height: 66.666667%;
        /// }
        /// ```
        _2over3 => "size-2/3",
        /// ```css
        /// {
        ///     width: 25%;
        ///     height: 25%;
        /// }
        /// ```
        _1over4 => "size-1/4",
        /// ```css
        /// {
        ///     width: 50%;
        ///     height: 50%;
        /// }
        /// ```
        _2over4 => "size-2/4",
        /// ```css
        /// {
        ///     width: 75%;
        ///     height: 75%;
        /// }
        /// ```
        _3over4 => "size-3/4",
        /// ```css
        /// {
        ///     width: 20%;
        ///     height: 20%;
        /// }
        /// ```
        _1over5 => "size-1/5",
        /// ```css
        /// {
        ///     width: 40%;
        ///     height: 40%;
        /// }
        /// ```
        _2over5 => "size-2/5",
        /// ```css
        /// {
        ///     width: 60%;
        ///     height: 60%;
        /// }
        /// ```
        _3over5 => "size-3/5",
        /// ```css
        /// {
        ///     width: 80%;
        ///     height: 80%;
        /// }
        /// ```
        _4over5 => "size-4/5",
        /// ```css
        /// {
        ///     width: 16.666667%;
        ///     height: 16.666667%;
        /// }
        /// ```
        _1over6 => "size-1/6",
        /// ```css
        /// {
        ///     width: 33.333333%;
        ///     height: 33.333333%;
        /// }
        /// ```
        _2over6 => "size-2/6",
        /// ```css
        /// {
        ///     width: 50%;
        ///     height: 50%;
        /// }
        /// ```
        _3over6 => "size-3/6",
        /// ```css
        /// {
        ///     width: 66.666667%;
        ///     height: 66.666667%;
        /// }
        /// ```
        _4over6 => "size-4/6",
        /// ```css
        /// {
        ///     width: 83.333333%;
        ///     height: 83.333333%;
        /// }
        /// ```
        _5over6 => "size-5/6",
        /// ```css
        /// {
        ///     width: 8.333333%;
        ///     height: 8.333333%;
        /// }
        /// ```
        _1over12 => "size-1/12",
        /// ```css
        /// {
        ///     width: 16.666667%;
        ///     height: 16.666667%;
        /// }
        /// ```
        _2over12 => "size-2/12",
        /// ```css
        /// {
        ///     width: 25%;
        ///     height: 25%;
        /// }
        /// ```
        _3over12 => "size-3/12",
        /// ```css
        /// {
        ///     width: 33.333333%;
        ///     height: 33.333333%;
        /// }
        /// ```
        _4over12 => "size-4/12",
        /// ```css
        /// {
        ///     width: 41.666667%;
        ///     height: 41.666667%;
        /// }
        /// ```
        _5over12 => "size-5/12",
        /// ```css
        /// {
        ///     width: 50%;
        ///     height: 50%;
        /// }
        /// ```
        _6over12 => "size-6/12",
        /// ```css
        /// {
        ///     width: 58.333333%;
        ///     height: 58.333333%;
        /// }
        /// ```
        _7over12 => "size-7/12",
        /// ```css
        /// {
        ///     width: 66.666667%;
        ///     height: 66.666667%;
        /// }
        /// ```
        _8over12 => "size-8/12",
        /// ```css
        /// {
        ///     width: 75%;
        ///     height: 75%;
        /// }
        /// ```
        _9over12 => "size-9/12",
        /// ```css
        /// {
        ///     width: 83.333333%;
        ///     height: 83.333333%;
        /// }
        /// ```
        _10over12 => "size-10/12",
        /// ```css
        /// {
        ///     width: 91.666667%;
        ///     height: 91.666667%;
        /// }
        /// ```
        _11over12 => "size-11/12",
        /// ```css
        /// {
        ///     width: 100%;
        ///     height: 100%;
        /// }
        /// ```
        Full => "size-full",
        /// ```css
        /// {
        ///     width: min-content;
        ///     height: min-content;
        /// }
        /// ```
        Min => "size-min",
        /// ```css
        /// {
        ///     width: max-content;
        ///     height: max-content;
        /// }
        /// ```
        Max => "size-max",
        /// ```css
        /// {
        ///     width: fit-content;
        ///     height: fit-content;
        /// }
        /// ```
        Fit => "size-fit",
    }
);
