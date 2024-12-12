def_types!(
    /// Utilities for controlling the font family of an element.
    ///
    /// <https://tailwindcss.com/docs/font-family>
    FontFamily {
        /// ```css
        /// {
        ///     font-family: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
        /// }
        /// ```
        Sans => "font-sans",
        /// ```css
        /// {
        ///     font-family: ui-serif, Georgia, Cambria, "Times New Roman", Times, serif;
        /// }
        /// ```
        Serif => "font-serif",
        /// ```css
        /// {
        ///     font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
        /// }
        /// ```
        Mono => "font-mono",
    }
    /// Utilities for controlling the font size of an element.
    ///
    /// <https://tailwindcss.com/docs/font-size>
    FontSize {
        /// ```css
        /// {
        ///     font-size: 0.75rem; /* 12px */
        ///     line-height: 1rem; /* 16px */
        /// }
        /// ```
        Xs => "text-xs",
        /// ```css
        /// {
        ///     font-size: 0.875rem; /* 14px */
        ///     line-height: 1.25rem; /* 20px */
        /// }
        /// ```
        Sm => "text-sm",
        /// ```css
        /// {
        ///     font-size: 1rem; /* 16px */
        ///     line-height: 1.5rem; /* 24px */
        /// }
        /// ```
        Base => "text-base",
        /// ```css
        /// {
        ///     font-size: 1.125rem; /* 18px */
        ///     line-height: 1.75rem; /* 28px */
        /// }
        /// ```
        Lg => "text-lg",
        /// ```css
        /// {
        ///     font-size: 1.25rem; /* 20px */
        ///     line-height: 1.75rem; /* 28px */
        /// }
        /// ```
        Xl => "text-xl",
        /// ```css
        /// {
        ///     font-size: 1.5rem; /* 24px */
        ///     line-height: 2rem; /* 32px */
        /// }
        /// ```
        _2xl => "text-2xl",
        /// ```css
        /// {
        ///     font-size: 1.875rem; /* 30px */
        ///     line-height: 2.25rem; /* 36px */
        /// }
        /// ```
        _3xl => "text-3xl",
        /// ```css
        /// {
        ///     font-size: 2.25rem; /* 36px */
        ///     line-height: 2.5rem; /* 40px */
        /// }
        /// ```
        _4xl => "text-4xl",
        /// ```css
        /// {
        ///     font-size: 3rem; /* 48px */
        ///     line-height: 1;
        /// }
        /// ```
        _5xl => "text-5xl",
        /// ```css
        /// {
        ///     font-size: 3.75rem; /* 60px */
        ///     line-height: 1;
        /// }
        /// ```
        _6xl => "text-6xl",
        /// ```css
        /// {
        ///     font-size: 4.5rem; /* 72px */
        ///     line-height: 1;
        /// }
        /// ```
        _7xl => "text-7xl",
        /// ```css
        /// {
        ///     font-size: 6rem; /* 96px */
        ///     line-height: 1;
        /// }
        /// ```
        _8xl => "text-8xl",
        /// ```css
        /// {
        ///     font-size: 8rem; /* 128px */
        ///     line-height: 1;
        /// }
        /// ```
        _9xl => "text-9xl",
    }
    /// Utilities for controlling the font smoothing of an element.
    ///
    /// <https://tailwindcss.com/docs/font-smoothing>
    FontSmoothing {
        /// ```css
        /// {
        ///     -webkit-font-smoothing: antialiased;
        ///     -moz-osx-font-smoothing: grayscale;
        /// }
        /// ```
        Antialiased => "antialiased",
        /// ```css
        /// {
        ///     -webkit-font-smoothing: auto;
        ///     -moz-osx-font-smoothing: auto;
        /// }
        /// ```
        SubpixelAntialiased => "subpixel-antialiased",
    }
    /// Utilities for controlling the style of text.
    ///
    /// <https://tailwindcss.com/docs/font-style>
    FontStyle {
        /// ```css
        /// {
        ///     font-style: italic;
        /// }
        /// ```
        Italic => "italic",
        /// ```css
        /// {
        ///     font-style: normal;
        /// }
        /// ```
        NotItalic => "not-italic",
    }
    /// Utilities for controlling the font weight of an element.
    ///
    /// <https://tailwindcss.com/docs/font-weight>
    FontWeight {
        /// ```css
        /// {
        ///     font-weight: 100;
        /// }
        /// ```
        Thin => "font-thin",
        /// ```css
        /// {
        ///     font-weight: 200;
        /// }
        /// ```
        Extralight => "font-extralight",
        /// ```css
        /// {
        ///     font-weight: 300;
        /// }
        /// ```
        Light => "font-light",
        /// ```css
        /// {
        ///     font-weight: 400;
        /// }
        /// ```
        Normal => "font-normal",
        /// ```css
        /// {
        ///     font-weight: 500;
        /// }
        /// ```
        Medium => "font-medium",
        /// ```css
        /// {
        ///     font-weight: 600;
        /// }
        /// ```
        Semibold => "font-semibold",
        /// ```css
        /// {
        ///     font-weight: 700;
        /// }
        /// ```
        Bold => "font-bold",
        /// ```css
        /// {
        ///     font-weight: 800;
        /// }
        /// ```
        Extrabold => "font-extrabold",
        /// ```css
        /// {
        ///     font-weight: 900;
        /// }
        /// ```
        Black => "font-black",
    }
    /// Utilities for controlling the variant of numbers.
    ///
    /// <https://tailwindcss.com/docs/font-variant-numeric>
    FontVariantNumeric {
        /// ```css
        /// {
        ///     font-variant-numeric: normal;
        /// }
        /// ```
        NormalNums => "normal-nums",
        /// ```css
        /// {
        ///     font-variant-numeric: ordinal;
        /// }
        /// ```
        Ordinal => "ordinal",
        /// ```css
        /// {
        ///     font-variant-numeric: slashed-zero;
        /// }
        /// ```
        SlashedZero => "slashed-zero",
        /// ```css
        /// {
        ///     font-variant-numeric: lining-nums;
        /// }
        /// ```
        LiningNums => "lining-nums",
        /// ```css
        /// {
        ///     font-variant-numeric: oldstyle-nums;
        /// }
        /// ```
        OldstyleNums => "oldstyle-nums",
        /// ```css
        /// {
        ///     font-variant-numeric: proportional-nums;
        /// }
        /// ```
        ProportionalNums => "proportional-nums",
        /// ```css
        /// {
        ///     font-variant-numeric: tabular-nums;
        /// }
        /// ```
        TabularNums => "tabular-nums",
        /// ```css
        /// {
        ///     font-variant-numeric: diagonal-fractions;
        /// }
        /// ```
        DiagonalFractions => "diagonal-fractions",
        /// ```css
        /// {
        ///     font-variant-numeric: stacked-fractions;
        /// }
        /// ```
        StackedFractions => "stacked-fractions",
    }
    /// Utilities for controlling the tracking (letter spacing) of an element.
    ///
    /// <https://tailwindcss.com/docs/letter-spacing>
    LetterSpacing {
        /// ```css
        /// {
        ///     letter-spacing: -0.05em;
        /// }
        /// ```
        Tighter => "tracking-tighter",
        /// ```css
        /// {
        ///     letter-spacing: -0.025em;
        /// }
        /// ```
        Tight => "tracking-tight",
        /// ```css
        /// {
        ///     letter-spacing: 0em;
        /// }
        /// ```
        Normal => "tracking-normal",
        /// ```css
        /// {
        ///     letter-spacing: 0.025em;
        /// }
        /// ```
        Wide => "tracking-wide",
        /// ```css
        /// {
        ///     letter-spacing: 0.05em;
        /// }
        /// ```
        Wider => "tracking-wider",
        /// ```css
        /// {
        ///     letter-spacing: 0.1em;
        /// }
        /// ```
        Widest => "tracking-widest",
    }
    /// Utilities for clamping text to a specific number of lines.
    ///
    /// <https://tailwindcss.com/docs/line-clamp>
    LineClamp {
        /// ```css
        /// {
        ///     overflow: hidden;
        ///     display: -webkit-box;
        ///     -webkit-box-orient: vertical;
        ///     -webkit-line-clamp: 1;
        /// }
        /// ```
        _1 => "line-clamp-1",
        /// ```css
        /// {
        ///     overflow: hidden;
        ///     display: -webkit-box;
        ///     -webkit-box-orient: vertical;
        ///     -webkit-line-clamp: 2;
        /// }
        /// ```
        _2 => "line-clamp-2",
        /// ```css
        /// {
        ///     overflow: hidden;
        ///     display: -webkit-box;
        ///     -webkit-box-orient: vertical;
        ///     -webkit-line-clamp: 3;
        /// }
        /// ```
        _3 => "line-clamp-3",
        /// ```css
        /// {
        ///     overflow: hidden;
        ///     display: -webkit-box;
        ///     -webkit-box-orient: vertical;
        ///     -webkit-line-clamp: 4;
        /// }
        /// ```
        _4 => "line-clamp-4",
        /// ```css
        /// {
        ///     overflow: hidden;
        ///     display: -webkit-box;
        ///     -webkit-box-orient: vertical;
        ///     -webkit-line-clamp: 5;
        /// }
        /// ```
        _5 => "line-clamp-5",
        /// ```css
        /// {
        ///     overflow: hidden;
        ///     display: -webkit-box;
        ///     -webkit-box-orient: vertical;
        ///     -webkit-line-clamp: 6;
        /// }
        /// ```
        _6 => "line-clamp-6",
        /// ```css
        /// {
        ///     overflow: visible;
        ///     display: block;
        ///     -webkit-box-orient: horizontal;
        ///     -webkit-line-clamp: none;
        /// }
        /// ```
        None => "line-clamp-none",
    }
    /// Utilities for controlling the leading (line height) of an element.
    ///
    /// <https://tailwindcss.com/docs/line-height>
    LineHeight {
        /// ```css
        /// {
        ///     line-height: .75rem; /* 12px */
        /// }
        /// ```
        _3 => "leading-3",
        /// ```css
        /// {
        ///     line-height: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "leading-4",
        /// ```css
        /// {
        ///     line-height: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "leading-5",
        /// ```css
        /// {
        ///     line-height: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "leading-6",
        /// ```css
        /// {
        ///     line-height: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "leading-7",
        /// ```css
        /// {
        ///     line-height: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "leading-8",
        /// ```css
        /// {
        ///     line-height: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "leading-9",
        /// ```css
        /// {
        ///     line-height: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "leading-10",
        /// ```css
        /// {
        ///     line-height: 1;
        /// }
        /// ```
        None => "leading-none",
        /// ```css
        /// {
        ///     line-height: 1.25;
        /// }
        /// ```
        Tight => "leading-tight",
        /// ```css
        /// {
        ///     line-height: 1.375;
        /// }
        /// ```
        Snug => "leading-snug",
        /// ```css
        /// {
        ///     line-height: 1.5;
        /// }
        /// ```
        Normal => "leading-normal",
        /// ```css
        /// {
        ///     line-height: 1.625;
        /// }
        /// ```
        Relaxed => "leading-relaxed",
        /// ```css
        /// {
        ///     line-height: 2;
        /// }
        /// ```
        Loose => "leading-loose",
    }
    /// Utilities for controlling the marker images for list items.
    ///
    /// <https://tailwindcss.com/docs/list-style-image>
    ListStyleImage {
        /// ```css
        /// {
        ///     list-style-image: none;
        /// }
        /// ```
        ListImageNone => "list-image-none",
    }
    /// Utilities for controlling the position of bullets/numbers in lists.
    ///
    /// <https://tailwindcss.com/docs/list-style-position>
    ListStylePosition {
        /// ```css
        /// {
        ///     list-style-position: inside;
        /// }
        /// ```
        Inside => "list-inside",
        /// ```css
        /// {
        ///     list-style-position: outside;
        /// }
        /// ```
        Outside => "list-outside",
    }
    /// Utilities for controlling the bullet/number style of a list.
    ///
    /// <https://tailwindcss.com/docs/list-style-type>
    ListStyleType {
        /// ```css
        /// {
        ///     list-style-type: none;
        /// }
        /// ```
        None => "list-none",
        /// ```css
        /// {
        ///     list-style-type: disc;
        /// }
        /// ```
        Disc => "list-disc",
        /// ```css
        /// {
        ///     list-style-type: decimal;
        /// }
        /// ```
        Decimal => "list-decimal",
    }
    /// Utilities for controlling the alignment of text.
    ///
    /// <https://tailwindcss.com/docs/text-align>
    TextAlign {
        /// ```css
        /// {
        ///     text-align: left;
        /// }
        /// ```
        Left => "text-left",
        /// ```css
        /// {
        ///     text-align: center;
        /// }
        /// ```
        Center => "text-center",
        /// ```css
        /// {
        ///     text-align: right;
        /// }
        /// ```
        Right => "text-right",
        /// ```css
        /// {
        ///     text-align: justify;
        /// }
        /// ```
        Justify => "text-justify",
        /// ```css
        /// {
        ///     text-align: start;
        /// }
        /// ```
        Start => "text-start",
        /// ```css
        /// {
        ///     text-align: end;
        /// }
        /// ```
        End => "text-end",
    }
    /// Utilities for controlling the text color of an element.
    ///
    /// <https://tailwindcss.com/docs/text-color>
    TextColor {
        /// ```css
        /// {
        ///     color: inherit;
        /// }
        /// ```
        Inherit => "text-inherit",
        /// ```css
        /// {
        ///     color: currentColor;
        /// }
        /// ```
        Current => "text-current",
        /// ```css
        /// {
        ///     color: transparent;
        /// }
        /// ```
        Transparent => "text-transparent",
        /// ```css
        /// {
        ///     color: rgb(0 0 0);
        /// }
        /// ```
        Black => "text-black",
        /// ```css
        /// {
        ///     color: rgb(255 255 255);
        /// }
        /// ```
        White => "text-white",
        /// ```css
        /// {
        ///     color: rgb(248 250 252);
        /// }
        /// ```
        Slate50 => "text-slate-50",
        /// ```css
        /// {
        ///     color: rgb(241 245 249);
        /// }
        /// ```
        Slate100 => "text-slate-100",
        /// ```css
        /// {
        ///     color: rgb(226 232 240);
        /// }
        /// ```
        Slate200 => "text-slate-200",
        /// ```css
        /// {
        ///     color: rgb(203 213 225);
        /// }
        /// ```
        Slate300 => "text-slate-300",
        /// ```css
        /// {
        ///     color: rgb(148 163 184);
        /// }
        /// ```
        Slate400 => "text-slate-400",
        /// ```css
        /// {
        ///     color: rgb(100 116 139);
        /// }
        /// ```
        Slate500 => "text-slate-500",
        /// ```css
        /// {
        ///     color: rgb(71 85 105);
        /// }
        /// ```
        Slate600 => "text-slate-600",
        /// ```css
        /// {
        ///     color: rgb(51 65 85);
        /// }
        /// ```
        Slate700 => "text-slate-700",
        /// ```css
        /// {
        ///     color: rgb(30 41 59);
        /// }
        /// ```
        Slate800 => "text-slate-800",
        /// ```css
        /// {
        ///     color: rgb(15 23 42);
        /// }
        /// ```
        Slate900 => "text-slate-900",
        /// ```css
        /// {
        ///     color: rgb(2 6 23);
        /// }
        /// ```
        Slate950 => "text-slate-950",
        /// ```css
        /// {
        ///     color: rgb(249 250 251);
        /// }
        /// ```
        Gray50 => "text-gray-50",
        /// ```css
        /// {
        ///     color: rgb(243 244 246);
        /// }
        /// ```
        Gray100 => "text-gray-100",
        /// ```css
        /// {
        ///     color: rgb(229 231 235);
        /// }
        /// ```
        Gray200 => "text-gray-200",
        /// ```css
        /// {
        ///     color: rgb(209 213 219);
        /// }
        /// ```
        Gray300 => "text-gray-300",
        /// ```css
        /// {
        ///     color: rgb(156 163 175);
        /// }
        /// ```
        Gray400 => "text-gray-400",
        /// ```css
        /// {
        ///     color: rgb(107 114 128);
        /// }
        /// ```
        Gray500 => "text-gray-500",
        /// ```css
        /// {
        ///     color: rgb(75 85 99);
        /// }
        /// ```
        Gray600 => "text-gray-600",
        /// ```css
        /// {
        ///     color: rgb(55 65 81);
        /// }
        /// ```
        Gray700 => "text-gray-700",
        /// ```css
        /// {
        ///     color: rgb(31 41 55);
        /// }
        /// ```
        Gray800 => "text-gray-800",
        /// ```css
        /// {
        ///     color: rgb(17 24 39);
        /// }
        /// ```
        Gray900 => "text-gray-900",
        /// ```css
        /// {
        ///     color: rgb(3 7 18);
        /// }
        /// ```
        Gray950 => "text-gray-950",
        /// ```css
        /// {
        ///     color: rgb(250 250 250);
        /// }
        /// ```
        Zinc50 => "text-zinc-50",
        /// ```css
        /// {
        ///     color: rgb(244 244 245);
        /// }
        /// ```
        Zinc100 => "text-zinc-100",
        /// ```css
        /// {
        ///     color: rgb(228 228 231);
        /// }
        /// ```
        Zinc200 => "text-zinc-200",
        /// ```css
        /// {
        ///     color: rgb(212 212 216);
        /// }
        /// ```
        Zinc300 => "text-zinc-300",
        /// ```css
        /// {
        ///     color: rgb(161 161 170);
        /// }
        /// ```
        Zinc400 => "text-zinc-400",
        /// ```css
        /// {
        ///     color: rgb(113 113 122);
        /// }
        /// ```
        Zinc500 => "text-zinc-500",
        /// ```css
        /// {
        ///     color: rgb(82 82 91);
        /// }
        /// ```
        Zinc600 => "text-zinc-600",
        /// ```css
        /// {
        ///     color: rgb(63 63 70);
        /// }
        /// ```
        Zinc700 => "text-zinc-700",
        /// ```css
        /// {
        ///     color: rgb(39 39 42);
        /// }
        /// ```
        Zinc800 => "text-zinc-800",
        /// ```css
        /// {
        ///     color: rgb(24 24 27);
        /// }
        /// ```
        Zinc900 => "text-zinc-900",
        /// ```css
        /// {
        ///     color: rgb(9 9 11);
        /// }
        /// ```
        Zinc950 => "text-zinc-950",
        /// ```css
        /// {
        ///     color: rgb(250 250 250);
        /// }
        /// ```
        Neutral50 => "text-neutral-50",
        /// ```css
        /// {
        ///     color: rgb(245 245 245);
        /// }
        /// ```
        Neutral100 => "text-neutral-100",
        /// ```css
        /// {
        ///     color: rgb(229 229 229);
        /// }
        /// ```
        Neutral200 => "text-neutral-200",
        /// ```css
        /// {
        ///     color: rgb(212 212 212);
        /// }
        /// ```
        Neutral300 => "text-neutral-300",
        /// ```css
        /// {
        ///     color: rgb(163 163 163);
        /// }
        /// ```
        Neutral400 => "text-neutral-400",
        /// ```css
        /// {
        ///     color: rgb(115 115 115);
        /// }
        /// ```
        Neutral500 => "text-neutral-500",
        /// ```css
        /// {
        ///     color: rgb(82 82 82);
        /// }
        /// ```
        Neutral600 => "text-neutral-600",
        /// ```css
        /// {
        ///     color: rgb(64 64 64);
        /// }
        /// ```
        Neutral700 => "text-neutral-700",
        /// ```css
        /// {
        ///     color: rgb(38 38 38);
        /// }
        /// ```
        Neutral800 => "text-neutral-800",
        /// ```css
        /// {
        ///     color: rgb(23 23 23);
        /// }
        /// ```
        Neutral900 => "text-neutral-900",
        /// ```css
        /// {
        ///     color: rgb(10 10 10);
        /// }
        /// ```
        Neutral950 => "text-neutral-950",
        /// ```css
        /// {
        ///     color: rgb(250 250 249);
        /// }
        /// ```
        Stone50 => "text-stone-50",
        /// ```css
        /// {
        ///     color: rgb(245 245 244);
        /// }
        /// ```
        Stone100 => "text-stone-100",
        /// ```css
        /// {
        ///     color: rgb(231 229 228);
        /// }
        /// ```
        Stone200 => "text-stone-200",
        /// ```css
        /// {
        ///     color: rgb(214 211 209);
        /// }
        /// ```
        Stone300 => "text-stone-300",
        /// ```css
        /// {
        ///     color: rgb(168 162 158);
        /// }
        /// ```
        Stone400 => "text-stone-400",
        /// ```css
        /// {
        ///     color: rgb(120 113 108);
        /// }
        /// ```
        Stone500 => "text-stone-500",
        /// ```css
        /// {
        ///     color: rgb(87 83 78);
        /// }
        /// ```
        Stone600 => "text-stone-600",
        /// ```css
        /// {
        ///     color: rgb(68 64 60);
        /// }
        /// ```
        Stone700 => "text-stone-700",
        /// ```css
        /// {
        ///     color: rgb(41 37 36);
        /// }
        /// ```
        Stone800 => "text-stone-800",
        /// ```css
        /// {
        ///     color: rgb(28 25 23);
        /// }
        /// ```
        Stone900 => "text-stone-900",
        /// ```css
        /// {
        ///     color: rgb(12 10 9);
        /// }
        /// ```
        Stone950 => "text-stone-950",
        /// ```css
        /// {
        ///     color: rgb(254 242 242);
        /// }
        /// ```
        Red50 => "text-red-50",
        /// ```css
        /// {
        ///     color: rgb(254 226 226);
        /// }
        /// ```
        Red100 => "text-red-100",
        /// ```css
        /// {
        ///     color: rgb(254 202 202);
        /// }
        /// ```
        Red200 => "text-red-200",
        /// ```css
        /// {
        ///     color: rgb(252 165 165);
        /// }
        /// ```
        Red300 => "text-red-300",
        /// ```css
        /// {
        ///     color: rgb(248 113 113);
        /// }
        /// ```
        Red400 => "text-red-400",
        /// ```css
        /// {
        ///     color: rgb(239 68 68);
        /// }
        /// ```
        Red500 => "text-red-500",
        /// ```css
        /// {
        ///     color: rgb(220 38 38);
        /// }
        /// ```
        Red600 => "text-red-600",
        /// ```css
        /// {
        ///     color: rgb(185 28 28);
        /// }
        /// ```
        Red700 => "text-red-700",
        /// ```css
        /// {
        ///     color: rgb(153 27 27);
        /// }
        /// ```
        Red800 => "text-red-800",
        /// ```css
        /// {
        ///     color: rgb(127 29 29);
        /// }
        /// ```
        Red900 => "text-red-900",
        /// ```css
        /// {
        ///     color: rgb(69 10 10);
        /// }
        /// ```
        Red950 => "text-red-950",
        /// ```css
        /// {
        ///     color: rgb(255 247 237);
        /// }
        /// ```
        Orange50 => "text-orange-50",
        /// ```css
        /// {
        ///     color: rgb(255 237 213);
        /// }
        /// ```
        Orange100 => "text-orange-100",
        /// ```css
        /// {
        ///     color: rgb(254 215 170);
        /// }
        /// ```
        Orange200 => "text-orange-200",
        /// ```css
        /// {
        ///     color: rgb(253 186 116);
        /// }
        /// ```
        Orange300 => "text-orange-300",
        /// ```css
        /// {
        ///     color: rgb(251 146 60);
        /// }
        /// ```
        Orange400 => "text-orange-400",
        /// ```css
        /// {
        ///     color: rgb(249 115 22);
        /// }
        /// ```
        Orange500 => "text-orange-500",
        /// ```css
        /// {
        ///     color: rgb(234 88 12);
        /// }
        /// ```
        Orange600 => "text-orange-600",
        /// ```css
        /// {
        ///     color: rgb(194 65 12);
        /// }
        /// ```
        Orange700 => "text-orange-700",
        /// ```css
        /// {
        ///     color: rgb(154 52 18);
        /// }
        /// ```
        Orange800 => "text-orange-800",
        /// ```css
        /// {
        ///     color: rgb(124 45 18);
        /// }
        /// ```
        Orange900 => "text-orange-900",
        /// ```css
        /// {
        ///     color: rgb(67 20 7);
        /// }
        /// ```
        Orange950 => "text-orange-950",
        /// ```css
        /// {
        ///     color: rgb(255 251 235);
        /// }
        /// ```
        Amber50 => "text-amber-50",
        /// ```css
        /// {
        ///     color: rgb(254 243 199);
        /// }
        /// ```
        Amber100 => "text-amber-100",
        /// ```css
        /// {
        ///     color: rgb(253 230 138);
        /// }
        /// ```
        Amber200 => "text-amber-200",
        /// ```css
        /// {
        ///     color: rgb(252 211 77);
        /// }
        /// ```
        Amber300 => "text-amber-300",
        /// ```css
        /// {
        ///     color: rgb(251 191 36);
        /// }
        /// ```
        Amber400 => "text-amber-400",
        /// ```css
        /// {
        ///     color: rgb(245 158 11);
        /// }
        /// ```
        Amber500 => "text-amber-500",
        /// ```css
        /// {
        ///     color: rgb(217 119 6);
        /// }
        /// ```
        Amber600 => "text-amber-600",
        /// ```css
        /// {
        ///     color: rgb(180 83 9);
        /// }
        /// ```
        Amber700 => "text-amber-700",
        /// ```css
        /// {
        ///     color: rgb(146 64 14);
        /// }
        /// ```
        Amber800 => "text-amber-800",
        /// ```css
        /// {
        ///     color: rgb(120 53 15);
        /// }
        /// ```
        Amber900 => "text-amber-900",
        /// ```css
        /// {
        ///     color: rgb(69 26 3);
        /// }
        /// ```
        Amber950 => "text-amber-950",
        /// ```css
        /// {
        ///     color: rgb(254 252 232);
        /// }
        /// ```
        Yellow50 => "text-yellow-50",
        /// ```css
        /// {
        ///     color: rgb(254 249 195);
        /// }
        /// ```
        Yellow100 => "text-yellow-100",
        /// ```css
        /// {
        ///     color: rgb(254 240 138);
        /// }
        /// ```
        Yellow200 => "text-yellow-200",
        /// ```css
        /// {
        ///     color: rgb(253 224 71);
        /// }
        /// ```
        Yellow300 => "text-yellow-300",
        /// ```css
        /// {
        ///     color: rgb(250 204 21);
        /// }
        /// ```
        Yellow400 => "text-yellow-400",
        /// ```css
        /// {
        ///     color: rgb(234 179 8);
        /// }
        /// ```
        Yellow500 => "text-yellow-500",
        /// ```css
        /// {
        ///     color: rgb(202 138 4);
        /// }
        /// ```
        Yellow600 => "text-yellow-600",
        /// ```css
        /// {
        ///     color: rgb(161 98 7);
        /// }
        /// ```
        Yellow700 => "text-yellow-700",
        /// ```css
        /// {
        ///     color: rgb(133 77 14);
        /// }
        /// ```
        Yellow800 => "text-yellow-800",
        /// ```css
        /// {
        ///     color: rgb(113 63 18);
        /// }
        /// ```
        Yellow900 => "text-yellow-900",
        /// ```css
        /// {
        ///     color: rgb(66 32 6);
        /// }
        /// ```
        Yellow950 => "text-yellow-950",
        /// ```css
        /// {
        ///     color: rgb(247 254 231);
        /// }
        /// ```
        Lime50 => "text-lime-50",
        /// ```css
        /// {
        ///     color: rgb(236 252 203);
        /// }
        /// ```
        Lime100 => "text-lime-100",
        /// ```css
        /// {
        ///     color: rgb(217 249 157);
        /// }
        /// ```
        Lime200 => "text-lime-200",
        /// ```css
        /// {
        ///     color: rgb(190 242 100);
        /// }
        /// ```
        Lime300 => "text-lime-300",
        /// ```css
        /// {
        ///     color: rgb(163 230 53);
        /// }
        /// ```
        Lime400 => "text-lime-400",
        /// ```css
        /// {
        ///     color: rgb(132 204 22);
        /// }
        /// ```
        Lime500 => "text-lime-500",
        /// ```css
        /// {
        ///     color: rgb(101 163 13);
        /// }
        /// ```
        Lime600 => "text-lime-600",
        /// ```css
        /// {
        ///     color: rgb(77 124 15);
        /// }
        /// ```
        Lime700 => "text-lime-700",
        /// ```css
        /// {
        ///     color: rgb(63 98 18);
        /// }
        /// ```
        Lime800 => "text-lime-800",
        /// ```css
        /// {
        ///     color: rgb(54 83 20);
        /// }
        /// ```
        Lime900 => "text-lime-900",
        /// ```css
        /// {
        ///     color: rgb(26 46 5);
        /// }
        /// ```
        Lime950 => "text-lime-950",
        /// ```css
        /// {
        ///     color: rgb(240 253 244);
        /// }
        /// ```
        Green50 => "text-green-50",
        /// ```css
        /// {
        ///     color: rgb(220 252 231);
        /// }
        /// ```
        Green100 => "text-green-100",
        /// ```css
        /// {
        ///     color: rgb(187 247 208);
        /// }
        /// ```
        Green200 => "text-green-200",
        /// ```css
        /// {
        ///     color: rgb(134 239 172);
        /// }
        /// ```
        Green300 => "text-green-300",
        /// ```css
        /// {
        ///     color: rgb(74 222 128);
        /// }
        /// ```
        Green400 => "text-green-400",
        /// ```css
        /// {
        ///     color: rgb(34 197 94);
        /// }
        /// ```
        Green500 => "text-green-500",
        /// ```css
        /// {
        ///     color: rgb(22 163 74);
        /// }
        /// ```
        Green600 => "text-green-600",
        /// ```css
        /// {
        ///     color: rgb(21 128 61);
        /// }
        /// ```
        Green700 => "text-green-700",
        /// ```css
        /// {
        ///     color: rgb(22 101 52);
        /// }
        /// ```
        Green800 => "text-green-800",
        /// ```css
        /// {
        ///     color: rgb(20 83 45);
        /// }
        /// ```
        Green900 => "text-green-900",
        /// ```css
        /// {
        ///     color: rgb(5 46 22);
        /// }
        /// ```
        Green950 => "text-green-950",
        /// ```css
        /// {
        ///     color: rgb(236 253 245);
        /// }
        /// ```
        Emerald50 => "text-emerald-50",
        /// ```css
        /// {
        ///     color: rgb(209 250 229);
        /// }
        /// ```
        Emerald100 => "text-emerald-100",
        /// ```css
        /// {
        ///     color: rgb(167 243 208);
        /// }
        /// ```
        Emerald200 => "text-emerald-200",
        /// ```css
        /// {
        ///     color: rgb(110 231 183);
        /// }
        /// ```
        Emerald300 => "text-emerald-300",
        /// ```css
        /// {
        ///     color: rgb(52 211 153);
        /// }
        /// ```
        Emerald400 => "text-emerald-400",
        /// ```css
        /// {
        ///     color: rgb(16 185 129);
        /// }
        /// ```
        Emerald500 => "text-emerald-500",
        /// ```css
        /// {
        ///     color: rgb(5 150 105);
        /// }
        /// ```
        Emerald600 => "text-emerald-600",
        /// ```css
        /// {
        ///     color: rgb(4 120 87);
        /// }
        /// ```
        Emerald700 => "text-emerald-700",
        /// ```css
        /// {
        ///     color: rgb(6 95 70);
        /// }
        /// ```
        Emerald800 => "text-emerald-800",
        /// ```css
        /// {
        ///     color: rgb(6 78 59);
        /// }
        /// ```
        Emerald900 => "text-emerald-900",
        /// ```css
        /// {
        ///     color: rgb(2 44 34);
        /// }
        /// ```
        Emerald950 => "text-emerald-950",
        /// ```css
        /// {
        ///     color: rgb(240 253 250);
        /// }
        /// ```
        Teal50 => "text-teal-50",
        /// ```css
        /// {
        ///     color: rgb(204 251 241);
        /// }
        /// ```
        Teal100 => "text-teal-100",
        /// ```css
        /// {
        ///     color: rgb(153 246 228);
        /// }
        /// ```
        Teal200 => "text-teal-200",
        /// ```css
        /// {
        ///     color: rgb(94 234 212);
        /// }
        /// ```
        Teal300 => "text-teal-300",
        /// ```css
        /// {
        ///     color: rgb(45 212 191);
        /// }
        /// ```
        Teal400 => "text-teal-400",
        /// ```css
        /// {
        ///     color: rgb(20 184 166);
        /// }
        /// ```
        Teal500 => "text-teal-500",
        /// ```css
        /// {
        ///     color: rgb(13 148 136);
        /// }
        /// ```
        Teal600 => "text-teal-600",
        /// ```css
        /// {
        ///     color: rgb(15 118 110);
        /// }
        /// ```
        Teal700 => "text-teal-700",
        /// ```css
        /// {
        ///     color: rgb(17 94 89);
        /// }
        /// ```
        Teal800 => "text-teal-800",
        /// ```css
        /// {
        ///     color: rgb(19 78 74);
        /// }
        /// ```
        Teal900 => "text-teal-900",
        /// ```css
        /// {
        ///     color: rgb(4 47 46);
        /// }
        /// ```
        Teal950 => "text-teal-950",
        /// ```css
        /// {
        ///     color: rgb(236 254 255);
        /// }
        /// ```
        Cyan50 => "text-cyan-50",
        /// ```css
        /// {
        ///     color: rgb(207 250 254);
        /// }
        /// ```
        Cyan100 => "text-cyan-100",
        /// ```css
        /// {
        ///     color: rgb(165 243 252);
        /// }
        /// ```
        Cyan200 => "text-cyan-200",
        /// ```css
        /// {
        ///     color: rgb(103 232 249);
        /// }
        /// ```
        Cyan300 => "text-cyan-300",
        /// ```css
        /// {
        ///     color: rgb(34 211 238);
        /// }
        /// ```
        Cyan400 => "text-cyan-400",
        /// ```css
        /// {
        ///     color: rgb(6 182 212);
        /// }
        /// ```
        Cyan500 => "text-cyan-500",
        /// ```css
        /// {
        ///     color: rgb(8 145 178);
        /// }
        /// ```
        Cyan600 => "text-cyan-600",
        /// ```css
        /// {
        ///     color: rgb(14 116 144);
        /// }
        /// ```
        Cyan700 => "text-cyan-700",
        /// ```css
        /// {
        ///     color: rgb(21 94 117);
        /// }
        /// ```
        Cyan800 => "text-cyan-800",
        /// ```css
        /// {
        ///     color: rgb(22 78 99);
        /// }
        /// ```
        Cyan900 => "text-cyan-900",
        /// ```css
        /// {
        ///     color: rgb(8 51 68);
        /// }
        /// ```
        Cyan950 => "text-cyan-950",
        /// ```css
        /// {
        ///     color: rgb(240 249 255);
        /// }
        /// ```
        Sky50 => "text-sky-50",
        /// ```css
        /// {
        ///     color: rgb(224 242 254);
        /// }
        /// ```
        Sky100 => "text-sky-100",
        /// ```css
        /// {
        ///     color: rgb(186 230 253);
        /// }
        /// ```
        Sky200 => "text-sky-200",
        /// ```css
        /// {
        ///     color: rgb(125 211 252);
        /// }
        /// ```
        Sky300 => "text-sky-300",
        /// ```css
        /// {
        ///     color: rgb(56 189 248);
        /// }
        /// ```
        Sky400 => "text-sky-400",
        /// ```css
        /// {
        ///     color: rgb(14 165 233);
        /// }
        /// ```
        Sky500 => "text-sky-500",
        /// ```css
        /// {
        ///     color: rgb(2 132 199);
        /// }
        /// ```
        Sky600 => "text-sky-600",
        /// ```css
        /// {
        ///     color: rgb(3 105 161);
        /// }
        /// ```
        Sky700 => "text-sky-700",
        /// ```css
        /// {
        ///     color: rgb(7 89 133);
        /// }
        /// ```
        Sky800 => "text-sky-800",
        /// ```css
        /// {
        ///     color: rgb(12 74 110);
        /// }
        /// ```
        Sky900 => "text-sky-900",
        /// ```css
        /// {
        ///     color: rgb(8 47 73);
        /// }
        /// ```
        Sky950 => "text-sky-950",
        /// ```css
        /// {
        ///     color: rgb(239 246 255);
        /// }
        /// ```
        Blue50 => "text-blue-50",
        /// ```css
        /// {
        ///     color: rgb(219 234 254);
        /// }
        /// ```
        Blue100 => "text-blue-100",
        /// ```css
        /// {
        ///     color: rgb(191 219 254);
        /// }
        /// ```
        Blue200 => "text-blue-200",
        /// ```css
        /// {
        ///     color: rgb(147 197 253);
        /// }
        /// ```
        Blue300 => "text-blue-300",
        /// ```css
        /// {
        ///     color: rgb(96 165 250);
        /// }
        /// ```
        Blue400 => "text-blue-400",
        /// ```css
        /// {
        ///     color: rgb(59 130 246);
        /// }
        /// ```
        Blue500 => "text-blue-500",
        /// ```css
        /// {
        ///     color: rgb(37 99 235);
        /// }
        /// ```
        Blue600 => "text-blue-600",
        /// ```css
        /// {
        ///     color: rgb(29 78 216);
        /// }
        /// ```
        Blue700 => "text-blue-700",
        /// ```css
        /// {
        ///     color: rgb(30 64 175);
        /// }
        /// ```
        Blue800 => "text-blue-800",
        /// ```css
        /// {
        ///     color: rgb(30 58 138);
        /// }
        /// ```
        Blue900 => "text-blue-900",
        /// ```css
        /// {
        ///     color: rgb(23 37 84);
        /// }
        /// ```
        Blue950 => "text-blue-950",
        /// ```css
        /// {
        ///     color: rgb(238 242 255);
        /// }
        /// ```
        Indigo50 => "text-indigo-50",
        /// ```css
        /// {
        ///     color: rgb(224 231 255);
        /// }
        /// ```
        Indigo100 => "text-indigo-100",
        /// ```css
        /// {
        ///     color: rgb(199 210 254);
        /// }
        /// ```
        Indigo200 => "text-indigo-200",
        /// ```css
        /// {
        ///     color: rgb(165 180 252);
        /// }
        /// ```
        Indigo300 => "text-indigo-300",
        /// ```css
        /// {
        ///     color: rgb(129 140 248);
        /// }
        /// ```
        Indigo400 => "text-indigo-400",
        /// ```css
        /// {
        ///     color: rgb(99 102 241);
        /// }
        /// ```
        Indigo500 => "text-indigo-500",
        /// ```css
        /// {
        ///     color: rgb(79 70 229);
        /// }
        /// ```
        Indigo600 => "text-indigo-600",
        /// ```css
        /// {
        ///     color: rgb(67 56 202);
        /// }
        /// ```
        Indigo700 => "text-indigo-700",
        /// ```css
        /// {
        ///     color: rgb(55 48 163);
        /// }
        /// ```
        Indigo800 => "text-indigo-800",
        /// ```css
        /// {
        ///     color: rgb(49 46 129);
        /// }
        /// ```
        Indigo900 => "text-indigo-900",
        /// ```css
        /// {
        ///     color: rgb(30 27 75);
        /// }
        /// ```
        Indigo950 => "text-indigo-950",
        /// ```css
        /// {
        ///     color: rgb(245 243 255);
        /// }
        /// ```
        Violet50 => "text-violet-50",
        /// ```css
        /// {
        ///     color: rgb(237 233 254);
        /// }
        /// ```
        Violet100 => "text-violet-100",
        /// ```css
        /// {
        ///     color: rgb(221 214 254);
        /// }
        /// ```
        Violet200 => "text-violet-200",
        /// ```css
        /// {
        ///     color: rgb(196 181 253);
        /// }
        /// ```
        Violet300 => "text-violet-300",
        /// ```css
        /// {
        ///     color: rgb(167 139 250);
        /// }
        /// ```
        Violet400 => "text-violet-400",
        /// ```css
        /// {
        ///     color: rgb(139 92 246);
        /// }
        /// ```
        Violet500 => "text-violet-500",
        /// ```css
        /// {
        ///     color: rgb(124 58 237);
        /// }
        /// ```
        Violet600 => "text-violet-600",
        /// ```css
        /// {
        ///     color: rgb(109 40 217);
        /// }
        /// ```
        Violet700 => "text-violet-700",
        /// ```css
        /// {
        ///     color: rgb(91 33 182);
        /// }
        /// ```
        Violet800 => "text-violet-800",
        /// ```css
        /// {
        ///     color: rgb(76 29 149);
        /// }
        /// ```
        Violet900 => "text-violet-900",
        /// ```css
        /// {
        ///     color: rgb(46 16 101);
        /// }
        /// ```
        Violet950 => "text-violet-950",
        /// ```css
        /// {
        ///     color: rgb(250 245 255);
        /// }
        /// ```
        Purple50 => "text-purple-50",
        /// ```css
        /// {
        ///     color: rgb(243 232 255);
        /// }
        /// ```
        Purple100 => "text-purple-100",
        /// ```css
        /// {
        ///     color: rgb(233 213 255);
        /// }
        /// ```
        Purple200 => "text-purple-200",
        /// ```css
        /// {
        ///     color: rgb(216 180 254);
        /// }
        /// ```
        Purple300 => "text-purple-300",
        /// ```css
        /// {
        ///     color: rgb(192 132 252);
        /// }
        /// ```
        Purple400 => "text-purple-400",
        /// ```css
        /// {
        ///     color: rgb(168 85 247);
        /// }
        /// ```
        Purple500 => "text-purple-500",
        /// ```css
        /// {
        ///     color: rgb(147 51 234);
        /// }
        /// ```
        Purple600 => "text-purple-600",
        /// ```css
        /// {
        ///     color: rgb(126 34 206);
        /// }
        /// ```
        Purple700 => "text-purple-700",
        /// ```css
        /// {
        ///     color: rgb(107 33 168);
        /// }
        /// ```
        Purple800 => "text-purple-800",
        /// ```css
        /// {
        ///     color: rgb(88 28 135);
        /// }
        /// ```
        Purple900 => "text-purple-900",
        /// ```css
        /// {
        ///     color: rgb(59 7 100);
        /// }
        /// ```
        Purple950 => "text-purple-950",
        /// ```css
        /// {
        ///     color: rgb(253 244 255);
        /// }
        /// ```
        Fuchsia50 => "text-fuchsia-50",
        /// ```css
        /// {
        ///     color: rgb(250 232 255);
        /// }
        /// ```
        Fuchsia100 => "text-fuchsia-100",
        /// ```css
        /// {
        ///     color: rgb(245 208 254);
        /// }
        /// ```
        Fuchsia200 => "text-fuchsia-200",
        /// ```css
        /// {
        ///     color: rgb(240 171 252);
        /// }
        /// ```
        Fuchsia300 => "text-fuchsia-300",
        /// ```css
        /// {
        ///     color: rgb(232 121 249);
        /// }
        /// ```
        Fuchsia400 => "text-fuchsia-400",
        /// ```css
        /// {
        ///     color: rgb(217 70 239);
        /// }
        /// ```
        Fuchsia500 => "text-fuchsia-500",
        /// ```css
        /// {
        ///     color: rgb(192 38 211);
        /// }
        /// ```
        Fuchsia600 => "text-fuchsia-600",
        /// ```css
        /// {
        ///     color: rgb(162 28 175);
        /// }
        /// ```
        Fuchsia700 => "text-fuchsia-700",
        /// ```css
        /// {
        ///     color: rgb(134 25 143);
        /// }
        /// ```
        Fuchsia800 => "text-fuchsia-800",
        /// ```css
        /// {
        ///     color: rgb(112 26 117);
        /// }
        /// ```
        Fuchsia900 => "text-fuchsia-900",
        /// ```css
        /// {
        ///     color: rgb(74 4 78);
        /// }
        /// ```
        Fuchsia950 => "text-fuchsia-950",
        /// ```css
        /// {
        ///     color: rgb(253 242 248);
        /// }
        /// ```
        Pink50 => "text-pink-50",
        /// ```css
        /// {
        ///     color: rgb(252 231 243);
        /// }
        /// ```
        Pink100 => "text-pink-100",
        /// ```css
        /// {
        ///     color: rgb(251 207 232);
        /// }
        /// ```
        Pink200 => "text-pink-200",
        /// ```css
        /// {
        ///     color: rgb(249 168 212);
        /// }
        /// ```
        Pink300 => "text-pink-300",
        /// ```css
        /// {
        ///     color: rgb(244 114 182);
        /// }
        /// ```
        Pink400 => "text-pink-400",
        /// ```css
        /// {
        ///     color: rgb(236 72 153);
        /// }
        /// ```
        Pink500 => "text-pink-500",
        /// ```css
        /// {
        ///     color: rgb(219 39 119);
        /// }
        /// ```
        Pink600 => "text-pink-600",
        /// ```css
        /// {
        ///     color: rgb(190 24 93);
        /// }
        /// ```
        Pink700 => "text-pink-700",
        /// ```css
        /// {
        ///     color: rgb(157 23 77);
        /// }
        /// ```
        Pink800 => "text-pink-800",
        /// ```css
        /// {
        ///     color: rgb(131 24 67);
        /// }
        /// ```
        Pink900 => "text-pink-900",
        /// ```css
        /// {
        ///     color: rgb(80 7 36);
        /// }
        /// ```
        Pink950 => "text-pink-950",
        /// ```css
        /// {
        ///     color: rgb(255 241 242);
        /// }
        /// ```
        Rose50 => "text-rose-50",
        /// ```css
        /// {
        ///     color: rgb(255 228 230);
        /// }
        /// ```
        Rose100 => "text-rose-100",
        /// ```css
        /// {
        ///     color: rgb(254 205 211);
        /// }
        /// ```
        Rose200 => "text-rose-200",
        /// ```css
        /// {
        ///     color: rgb(253 164 175);
        /// }
        /// ```
        Rose300 => "text-rose-300",
        /// ```css
        /// {
        ///     color: rgb(251 113 133);
        /// }
        /// ```
        Rose400 => "text-rose-400",
        /// ```css
        /// {
        ///     color: rgb(244 63 94);
        /// }
        /// ```
        Rose500 => "text-rose-500",
        /// ```css
        /// {
        ///     color: rgb(225 29 72);
        /// }
        /// ```
        Rose600 => "text-rose-600",
        /// ```css
        /// {
        ///     color: rgb(190 18 60);
        /// }
        /// ```
        Rose700 => "text-rose-700",
        /// ```css
        /// {
        ///     color: rgb(159 18 57);
        /// }
        /// ```
        Rose800 => "text-rose-800",
        /// ```css
        /// {
        ///     color: rgb(136 19 55);
        /// }
        /// ```
        Rose900 => "text-rose-900",
        /// ```css
        /// {
        ///     color: rgb(76 5 25);
        /// }
        /// ```
        Rose950 => "text-rose-950",
    }
    /// Utilities for controlling the decoration of text.
    ///
    /// <https://tailwindcss.com/docs/text-decoration>
    TextDecoration {
        /// ```css
        /// {
        ///     text-decoration-line: underline;
        /// }
        /// ```
        Underline => "underline",
        /// ```css
        /// {
        ///     text-decoration-line: overline;
        /// }
        /// ```
        Overline => "overline",
        /// ```css
        /// {
        ///     text-decoration-line: line-through;
        /// }
        /// ```
        LineThrough => "line-through",
        /// ```css
        /// {
        ///     text-decoration-line: none;
        /// }
        /// ```
        NoUnderline => "no-underline",
    }
    /// Utilities for controlling the color of text decorations.
    ///
    /// <https://tailwindcss.com/docs/text-decoration-color>
    TextDecorationColor {
        /// ```css
        /// {
        ///     text-decoration-color: inherit;
        /// }
        /// ```
        Inherit => "decoration-inherit",
        /// ```css
        /// {
        ///     text-decoration-color: currentColor;
        /// }
        /// ```
        Current => "decoration-current",
        /// ```css
        /// {
        ///     text-decoration-color: transparent;
        /// }
        /// ```
        Transparent => "decoration-transparent",
        /// ```css
        /// {
        ///     text-decoration-color: #000;
        /// }
        /// ```
        Black => "decoration-black",
        /// ```css
        /// {
        ///     text-decoration-color: #fff;
        /// }
        /// ```
        White => "decoration-white",
        /// ```css
        /// {
        ///     text-decoration-color: #f8fafc;
        /// }
        /// ```
        Slate50 => "decoration-slate-50",
        /// ```css
        /// {
        ///     text-decoration-color: #f1f5f9;
        /// }
        /// ```
        Slate100 => "decoration-slate-100",
        /// ```css
        /// {
        ///     text-decoration-color: #e2e8f0;
        /// }
        /// ```
        Slate200 => "decoration-slate-200",
        /// ```css
        /// {
        ///     text-decoration-color: #cbd5e1;
        /// }
        /// ```
        Slate300 => "decoration-slate-300",
        /// ```css
        /// {
        ///     text-decoration-color: #94a3b8;
        /// }
        /// ```
        Slate400 => "decoration-slate-400",
        /// ```css
        /// {
        ///     text-decoration-color: #64748b;
        /// }
        /// ```
        Slate500 => "decoration-slate-500",
        /// ```css
        /// {
        ///     text-decoration-color: #475569;
        /// }
        /// ```
        Slate600 => "decoration-slate-600",
        /// ```css
        /// {
        ///     text-decoration-color: #334155;
        /// }
        /// ```
        Slate700 => "decoration-slate-700",
        /// ```css
        /// {
        ///     text-decoration-color: #1e293b;
        /// }
        /// ```
        Slate800 => "decoration-slate-800",
        /// ```css
        /// {
        ///     text-decoration-color: #0f172a;
        /// }
        /// ```
        Slate900 => "decoration-slate-900",
        /// ```css
        /// {
        ///     text-decoration-color: #020617;
        /// }
        /// ```
        Slate950 => "decoration-slate-950",
        /// ```css
        /// {
        ///     text-decoration-color: #f9fafb;
        /// }
        /// ```
        Gray50 => "decoration-gray-50",
        /// ```css
        /// {
        ///     text-decoration-color: #f3f4f6;
        /// }
        /// ```
        Gray100 => "decoration-gray-100",
        /// ```css
        /// {
        ///     text-decoration-color: #e5e7eb;
        /// }
        /// ```
        Gray200 => "decoration-gray-200",
        /// ```css
        /// {
        ///     text-decoration-color: #d1d5db;
        /// }
        /// ```
        Gray300 => "decoration-gray-300",
        /// ```css
        /// {
        ///     text-decoration-color: #9ca3af;
        /// }
        /// ```
        Gray400 => "decoration-gray-400",
        /// ```css
        /// {
        ///     text-decoration-color: #6b7280;
        /// }
        /// ```
        Gray500 => "decoration-gray-500",
        /// ```css
        /// {
        ///     text-decoration-color: #4b5563;
        /// }
        /// ```
        Gray600 => "decoration-gray-600",
        /// ```css
        /// {
        ///     text-decoration-color: #374151;
        /// }
        /// ```
        Gray700 => "decoration-gray-700",
        /// ```css
        /// {
        ///     text-decoration-color: #1f2937;
        /// }
        /// ```
        Gray800 => "decoration-gray-800",
        /// ```css
        /// {
        ///     text-decoration-color: #111827;
        /// }
        /// ```
        Gray900 => "decoration-gray-900",
        /// ```css
        /// {
        ///     text-decoration-color: #030712;
        /// }
        /// ```
        Gray950 => "decoration-gray-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fafafa;
        /// }
        /// ```
        Zinc50 => "decoration-zinc-50",
        /// ```css
        /// {
        ///     text-decoration-color: #f4f4f5;
        /// }
        /// ```
        Zinc100 => "decoration-zinc-100",
        /// ```css
        /// {
        ///     text-decoration-color: #e4e4e7;
        /// }
        /// ```
        Zinc200 => "decoration-zinc-200",
        /// ```css
        /// {
        ///     text-decoration-color: #d4d4d8;
        /// }
        /// ```
        Zinc300 => "decoration-zinc-300",
        /// ```css
        /// {
        ///     text-decoration-color: #a1a1aa;
        /// }
        /// ```
        Zinc400 => "decoration-zinc-400",
        /// ```css
        /// {
        ///     text-decoration-color: #71717a;
        /// }
        /// ```
        Zinc500 => "decoration-zinc-500",
        /// ```css
        /// {
        ///     text-decoration-color: #52525b;
        /// }
        /// ```
        Zinc600 => "decoration-zinc-600",
        /// ```css
        /// {
        ///     text-decoration-color: #3f3f46;
        /// }
        /// ```
        Zinc700 => "decoration-zinc-700",
        /// ```css
        /// {
        ///     text-decoration-color: #27272a;
        /// }
        /// ```
        Zinc800 => "decoration-zinc-800",
        /// ```css
        /// {
        ///     text-decoration-color: #18181b;
        /// }
        /// ```
        Zinc900 => "decoration-zinc-900",
        /// ```css
        /// {
        ///     text-decoration-color: #09090b;
        /// }
        /// ```
        Zinc950 => "decoration-zinc-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fafafa;
        /// }
        /// ```
        Neutral50 => "decoration-neutral-50",
        /// ```css
        /// {
        ///     text-decoration-color: #f5f5f5;
        /// }
        /// ```
        Neutral100 => "decoration-neutral-100",
        /// ```css
        /// {
        ///     text-decoration-color: #e5e5e5;
        /// }
        /// ```
        Neutral200 => "decoration-neutral-200",
        /// ```css
        /// {
        ///     text-decoration-color: #d4d4d4;
        /// }
        /// ```
        Neutral300 => "decoration-neutral-300",
        /// ```css
        /// {
        ///     text-decoration-color: #a3a3a3;
        /// }
        /// ```
        Neutral400 => "decoration-neutral-400",
        /// ```css
        /// {
        ///     text-decoration-color: #737373;
        /// }
        /// ```
        Neutral500 => "decoration-neutral-500",
        /// ```css
        /// {
        ///     text-decoration-color: #525252;
        /// }
        /// ```
        Neutral600 => "decoration-neutral-600",
        /// ```css
        /// {
        ///     text-decoration-color: #404040;
        /// }
        /// ```
        Neutral700 => "decoration-neutral-700",
        /// ```css
        /// {
        ///     text-decoration-color: #262626;
        /// }
        /// ```
        Neutral800 => "decoration-neutral-800",
        /// ```css
        /// {
        ///     text-decoration-color: #171717;
        /// }
        /// ```
        Neutral900 => "decoration-neutral-900",
        /// ```css
        /// {
        ///     text-decoration-color: #0a0a0a;
        /// }
        /// ```
        Neutral950 => "decoration-neutral-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fafaf9;
        /// }
        /// ```
        Stone50 => "decoration-stone-50",
        /// ```css
        /// {
        ///     text-decoration-color: #f5f5f4;
        /// }
        /// ```
        Stone100 => "decoration-stone-100",
        /// ```css
        /// {
        ///     text-decoration-color: #e7e5e4;
        /// }
        /// ```
        Stone200 => "decoration-stone-200",
        /// ```css
        /// {
        ///     text-decoration-color: #d6d3d1;
        /// }
        /// ```
        Stone300 => "decoration-stone-300",
        /// ```css
        /// {
        ///     text-decoration-color: #a8a29e;
        /// }
        /// ```
        Stone400 => "decoration-stone-400",
        /// ```css
        /// {
        ///     text-decoration-color: #78716c;
        /// }
        /// ```
        Stone500 => "decoration-stone-500",
        /// ```css
        /// {
        ///     text-decoration-color: #57534e;
        /// }
        /// ```
        Stone600 => "decoration-stone-600",
        /// ```css
        /// {
        ///     text-decoration-color: #44403c;
        /// }
        /// ```
        Stone700 => "decoration-stone-700",
        /// ```css
        /// {
        ///     text-decoration-color: #292524;
        /// }
        /// ```
        Stone800 => "decoration-stone-800",
        /// ```css
        /// {
        ///     text-decoration-color: #1c1917;
        /// }
        /// ```
        Stone900 => "decoration-stone-900",
        /// ```css
        /// {
        ///     text-decoration-color: #0c0a09;
        /// }
        /// ```
        Stone950 => "decoration-stone-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fef2f2;
        /// }
        /// ```
        Red50 => "decoration-red-50",
        /// ```css
        /// {
        ///     text-decoration-color: #fee2e2;
        /// }
        /// ```
        Red100 => "decoration-red-100",
        /// ```css
        /// {
        ///     text-decoration-color: #fecaca;
        /// }
        /// ```
        Red200 => "decoration-red-200",
        /// ```css
        /// {
        ///     text-decoration-color: #fca5a5;
        /// }
        /// ```
        Red300 => "decoration-red-300",
        /// ```css
        /// {
        ///     text-decoration-color: #f87171;
        /// }
        /// ```
        Red400 => "decoration-red-400",
        /// ```css
        /// {
        ///     text-decoration-color: #ef4444;
        /// }
        /// ```
        Red500 => "decoration-red-500",
        /// ```css
        /// {
        ///     text-decoration-color: #dc2626;
        /// }
        /// ```
        Red600 => "decoration-red-600",
        /// ```css
        /// {
        ///     text-decoration-color: #b91c1c;
        /// }
        /// ```
        Red700 => "decoration-red-700",
        /// ```css
        /// {
        ///     text-decoration-color: #991b1b;
        /// }
        /// ```
        Red800 => "decoration-red-800",
        /// ```css
        /// {
        ///     text-decoration-color: #7f1d1d;
        /// }
        /// ```
        Red900 => "decoration-red-900",
        /// ```css
        /// {
        ///     text-decoration-color: #450a0a;
        /// }
        /// ```
        Red950 => "decoration-red-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fff7ed;
        /// }
        /// ```
        Orange50 => "decoration-orange-50",
        /// ```css
        /// {
        ///     text-decoration-color: #ffedd5;
        /// }
        /// ```
        Orange100 => "decoration-orange-100",
        /// ```css
        /// {
        ///     text-decoration-color: #fed7aa;
        /// }
        /// ```
        Orange200 => "decoration-orange-200",
        /// ```css
        /// {
        ///     text-decoration-color: #fdba74;
        /// }
        /// ```
        Orange300 => "decoration-orange-300",
        /// ```css
        /// {
        ///     text-decoration-color: #fb923c;
        /// }
        /// ```
        Orange400 => "decoration-orange-400",
        /// ```css
        /// {
        ///     text-decoration-color: #f97316;
        /// }
        /// ```
        Orange500 => "decoration-orange-500",
        /// ```css
        /// {
        ///     text-decoration-color: #ea580c;
        /// }
        /// ```
        Orange600 => "decoration-orange-600",
        /// ```css
        /// {
        ///     text-decoration-color: #c2410c;
        /// }
        /// ```
        Orange700 => "decoration-orange-700",
        /// ```css
        /// {
        ///     text-decoration-color: #9a3412;
        /// }
        /// ```
        Orange800 => "decoration-orange-800",
        /// ```css
        /// {
        ///     text-decoration-color: #7c2d12;
        /// }
        /// ```
        Orange900 => "decoration-orange-900",
        /// ```css
        /// {
        ///     text-decoration-color: #431407;
        /// }
        /// ```
        Orange950 => "decoration-orange-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fffbeb;
        /// }
        /// ```
        Amber50 => "decoration-amber-50",
        /// ```css
        /// {
        ///     text-decoration-color: #fef3c7;
        /// }
        /// ```
        Amber100 => "decoration-amber-100",
        /// ```css
        /// {
        ///     text-decoration-color: #fde68a;
        /// }
        /// ```
        Amber200 => "decoration-amber-200",
        /// ```css
        /// {
        ///     text-decoration-color: #fcd34d;
        /// }
        /// ```
        Amber300 => "decoration-amber-300",
        /// ```css
        /// {
        ///     text-decoration-color: #fbbf24;
        /// }
        /// ```
        Amber400 => "decoration-amber-400",
        /// ```css
        /// {
        ///     text-decoration-color: #f59e0b;
        /// }
        /// ```
        Amber500 => "decoration-amber-500",
        /// ```css
        /// {
        ///     text-decoration-color: #d97706;
        /// }
        /// ```
        Amber600 => "decoration-amber-600",
        /// ```css
        /// {
        ///     text-decoration-color: #b45309;
        /// }
        /// ```
        Amber700 => "decoration-amber-700",
        /// ```css
        /// {
        ///     text-decoration-color: #92400e;
        /// }
        /// ```
        Amber800 => "decoration-amber-800",
        /// ```css
        /// {
        ///     text-decoration-color: #78350f;
        /// }
        /// ```
        Amber900 => "decoration-amber-900",
        /// ```css
        /// {
        ///     text-decoration-color: #451a03;
        /// }
        /// ```
        Amber950 => "decoration-amber-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fefce8;
        /// }
        /// ```
        Yellow50 => "decoration-yellow-50",
        /// ```css
        /// {
        ///     text-decoration-color: #fef9c3;
        /// }
        /// ```
        Yellow100 => "decoration-yellow-100",
        /// ```css
        /// {
        ///     text-decoration-color: #fef08a;
        /// }
        /// ```
        Yellow200 => "decoration-yellow-200",
        /// ```css
        /// {
        ///     text-decoration-color: #fde047;
        /// }
        /// ```
        Yellow300 => "decoration-yellow-300",
        /// ```css
        /// {
        ///     text-decoration-color: #facc15;
        /// }
        /// ```
        Yellow400 => "decoration-yellow-400",
        /// ```css
        /// {
        ///     text-decoration-color: #eab308;
        /// }
        /// ```
        Yellow500 => "decoration-yellow-500",
        /// ```css
        /// {
        ///     text-decoration-color: #ca8a04;
        /// }
        /// ```
        Yellow600 => "decoration-yellow-600",
        /// ```css
        /// {
        ///     text-decoration-color: #a16207;
        /// }
        /// ```
        Yellow700 => "decoration-yellow-700",
        /// ```css
        /// {
        ///     text-decoration-color: #854d0e;
        /// }
        /// ```
        Yellow800 => "decoration-yellow-800",
        /// ```css
        /// {
        ///     text-decoration-color: #713f12;
        /// }
        /// ```
        Yellow900 => "decoration-yellow-900",
        /// ```css
        /// {
        ///     text-decoration-color: #422006;
        /// }
        /// ```
        Yellow950 => "decoration-yellow-950",
        /// ```css
        /// {
        ///     text-decoration-color: #f7fee7;
        /// }
        /// ```
        Lime50 => "decoration-lime-50",
        /// ```css
        /// {
        ///     text-decoration-color: #ecfccb;
        /// }
        /// ```
        Lime100 => "decoration-lime-100",
        /// ```css
        /// {
        ///     text-decoration-color: #d9f99d;
        /// }
        /// ```
        Lime200 => "decoration-lime-200",
        /// ```css
        /// {
        ///     text-decoration-color: #bef264;
        /// }
        /// ```
        Lime300 => "decoration-lime-300",
        /// ```css
        /// {
        ///     text-decoration-color: #a3e635;
        /// }
        /// ```
        Lime400 => "decoration-lime-400",
        /// ```css
        /// {
        ///     text-decoration-color: #84cc16;
        /// }
        /// ```
        Lime500 => "decoration-lime-500",
        /// ```css
        /// {
        ///     text-decoration-color: #65a30d;
        /// }
        /// ```
        Lime600 => "decoration-lime-600",
        /// ```css
        /// {
        ///     text-decoration-color: #4d7c0f;
        /// }
        /// ```
        Lime700 => "decoration-lime-700",
        /// ```css
        /// {
        ///     text-decoration-color: #3f6212;
        /// }
        /// ```
        Lime800 => "decoration-lime-800",
        /// ```css
        /// {
        ///     text-decoration-color: #365314;
        /// }
        /// ```
        Lime900 => "decoration-lime-900",
        /// ```css
        /// {
        ///     text-decoration-color: #1a2e05;
        /// }
        /// ```
        Lime950 => "decoration-lime-950",
        /// ```css
        /// {
        ///     text-decoration-color: #f0fdf4;
        /// }
        /// ```
        Green50 => "decoration-green-50",
        /// ```css
        /// {
        ///     text-decoration-color: #dcfce7;
        /// }
        /// ```
        Green100 => "decoration-green-100",
        /// ```css
        /// {
        ///     text-decoration-color: #bbf7d0;
        /// }
        /// ```
        Green200 => "decoration-green-200",
        /// ```css
        /// {
        ///     text-decoration-color: #86efac;
        /// }
        /// ```
        Green300 => "decoration-green-300",
        /// ```css
        /// {
        ///     text-decoration-color: #4ade80;
        /// }
        /// ```
        Green400 => "decoration-green-400",
        /// ```css
        /// {
        ///     text-decoration-color: #22c55e;
        /// }
        /// ```
        Green500 => "decoration-green-500",
        /// ```css
        /// {
        ///     text-decoration-color: #16a34a;
        /// }
        /// ```
        Green600 => "decoration-green-600",
        /// ```css
        /// {
        ///     text-decoration-color: #15803d;
        /// }
        /// ```
        Green700 => "decoration-green-700",
        /// ```css
        /// {
        ///     text-decoration-color: #166534;
        /// }
        /// ```
        Green800 => "decoration-green-800",
        /// ```css
        /// {
        ///     text-decoration-color: #14532d;
        /// }
        /// ```
        Green900 => "decoration-green-900",
        /// ```css
        /// {
        ///     text-decoration-color: #052e16;
        /// }
        /// ```
        Green950 => "decoration-green-950",
        /// ```css
        /// {
        ///     text-decoration-color: #ecfdf5;
        /// }
        /// ```
        Emerald50 => "decoration-emerald-50",
        /// ```css
        /// {
        ///     text-decoration-color: #d1fae5;
        /// }
        /// ```
        Emerald100 => "decoration-emerald-100",
        /// ```css
        /// {
        ///     text-decoration-color: #a7f3d0;
        /// }
        /// ```
        Emerald200 => "decoration-emerald-200",
        /// ```css
        /// {
        ///     text-decoration-color: #6ee7b7;
        /// }
        /// ```
        Emerald300 => "decoration-emerald-300",
        /// ```css
        /// {
        ///     text-decoration-color: #34d399;
        /// }
        /// ```
        Emerald400 => "decoration-emerald-400",
        /// ```css
        /// {
        ///     text-decoration-color: #10b981;
        /// }
        /// ```
        Emerald500 => "decoration-emerald-500",
        /// ```css
        /// {
        ///     text-decoration-color: #059669;
        /// }
        /// ```
        Emerald600 => "decoration-emerald-600",
        /// ```css
        /// {
        ///     text-decoration-color: #047857;
        /// }
        /// ```
        Emerald700 => "decoration-emerald-700",
        /// ```css
        /// {
        ///     text-decoration-color: #065f46;
        /// }
        /// ```
        Emerald800 => "decoration-emerald-800",
        /// ```css
        /// {
        ///     text-decoration-color: #064e3b;
        /// }
        /// ```
        Emerald900 => "decoration-emerald-900",
        /// ```css
        /// {
        ///     text-decoration-color: #022c22;
        /// }
        /// ```
        Emerald950 => "decoration-emerald-950",
        /// ```css
        /// {
        ///     text-decoration-color: #f0fdfa;
        /// }
        /// ```
        Teal50 => "decoration-teal-50",
        /// ```css
        /// {
        ///     text-decoration-color: #ccfbf1;
        /// }
        /// ```
        Teal100 => "decoration-teal-100",
        /// ```css
        /// {
        ///     text-decoration-color: #99f6e4;
        /// }
        /// ```
        Teal200 => "decoration-teal-200",
        /// ```css
        /// {
        ///     text-decoration-color: #5eead4;
        /// }
        /// ```
        Teal300 => "decoration-teal-300",
        /// ```css
        /// {
        ///     text-decoration-color: #2dd4bf;
        /// }
        /// ```
        Teal400 => "decoration-teal-400",
        /// ```css
        /// {
        ///     text-decoration-color: #14b8a6;
        /// }
        /// ```
        Teal500 => "decoration-teal-500",
        /// ```css
        /// {
        ///     text-decoration-color: #0d9488;
        /// }
        /// ```
        Teal600 => "decoration-teal-600",
        /// ```css
        /// {
        ///     text-decoration-color: #0f766e;
        /// }
        /// ```
        Teal700 => "decoration-teal-700",
        /// ```css
        /// {
        ///     text-decoration-color: #115e59;
        /// }
        /// ```
        Teal800 => "decoration-teal-800",
        /// ```css
        /// {
        ///     text-decoration-color: #134e4a;
        /// }
        /// ```
        Teal900 => "decoration-teal-900",
        /// ```css
        /// {
        ///     text-decoration-color: #042f2e;
        /// }
        /// ```
        Teal950 => "decoration-teal-950",
        /// ```css
        /// {
        ///     text-decoration-color: #ecfeff;
        /// }
        /// ```
        Cyan50 => "decoration-cyan-50",
        /// ```css
        /// {
        ///     text-decoration-color: #cffafe;
        /// }
        /// ```
        Cyan100 => "decoration-cyan-100",
        /// ```css
        /// {
        ///     text-decoration-color: #a5f3fc;
        /// }
        /// ```
        Cyan200 => "decoration-cyan-200",
        /// ```css
        /// {
        ///     text-decoration-color: #67e8f9;
        /// }
        /// ```
        Cyan300 => "decoration-cyan-300",
        /// ```css
        /// {
        ///     text-decoration-color: #22d3ee;
        /// }
        /// ```
        Cyan400 => "decoration-cyan-400",
        /// ```css
        /// {
        ///     text-decoration-color: #06b6d4;
        /// }
        /// ```
        Cyan500 => "decoration-cyan-500",
        /// ```css
        /// {
        ///     text-decoration-color: #0891b2;
        /// }
        /// ```
        Cyan600 => "decoration-cyan-600",
        /// ```css
        /// {
        ///     text-decoration-color: #0e7490;
        /// }
        /// ```
        Cyan700 => "decoration-cyan-700",
        /// ```css
        /// {
        ///     text-decoration-color: #155e75;
        /// }
        /// ```
        Cyan800 => "decoration-cyan-800",
        /// ```css
        /// {
        ///     text-decoration-color: #164e63;
        /// }
        /// ```
        Cyan900 => "decoration-cyan-900",
        /// ```css
        /// {
        ///     text-decoration-color: #083344;
        /// }
        /// ```
        Cyan950 => "decoration-cyan-950",
        /// ```css
        /// {
        ///     text-decoration-color: #f0f9ff;
        /// }
        /// ```
        Sky50 => "decoration-sky-50",
        /// ```css
        /// {
        ///     text-decoration-color: #e0f2fe;
        /// }
        /// ```
        Sky100 => "decoration-sky-100",
        /// ```css
        /// {
        ///     text-decoration-color: #bae6fd;
        /// }
        /// ```
        Sky200 => "decoration-sky-200",
        /// ```css
        /// {
        ///     text-decoration-color: #7dd3fc;
        /// }
        /// ```
        Sky300 => "decoration-sky-300",
        /// ```css
        /// {
        ///     text-decoration-color: #38bdf8;
        /// }
        /// ```
        Sky400 => "decoration-sky-400",
        /// ```css
        /// {
        ///     text-decoration-color: #0ea5e9;
        /// }
        /// ```
        Sky500 => "decoration-sky-500",
        /// ```css
        /// {
        ///     text-decoration-color: #0284c7;
        /// }
        /// ```
        Sky600 => "decoration-sky-600",
        /// ```css
        /// {
        ///     text-decoration-color: #0369a1;
        /// }
        /// ```
        Sky700 => "decoration-sky-700",
        /// ```css
        /// {
        ///     text-decoration-color: #075985;
        /// }
        /// ```
        Sky800 => "decoration-sky-800",
        /// ```css
        /// {
        ///     text-decoration-color: #0c4a6e;
        /// }
        /// ```
        Sky900 => "decoration-sky-900",
        /// ```css
        /// {
        ///     text-decoration-color: #082f49;
        /// }
        /// ```
        Sky950 => "decoration-sky-950",
        /// ```css
        /// {
        ///     text-decoration-color: #eff6ff;
        /// }
        /// ```
        Blue50 => "decoration-blue-50",
        /// ```css
        /// {
        ///     text-decoration-color: #dbeafe;
        /// }
        /// ```
        Blue100 => "decoration-blue-100",
        /// ```css
        /// {
        ///     text-decoration-color: #bfdbfe;
        /// }
        /// ```
        Blue200 => "decoration-blue-200",
        /// ```css
        /// {
        ///     text-decoration-color: #93c5fd;
        /// }
        /// ```
        Blue300 => "decoration-blue-300",
        /// ```css
        /// {
        ///     text-decoration-color: #60a5fa;
        /// }
        /// ```
        Blue400 => "decoration-blue-400",
        /// ```css
        /// {
        ///     text-decoration-color: #3b82f6;
        /// }
        /// ```
        Blue500 => "decoration-blue-500",
        /// ```css
        /// {
        ///     text-decoration-color: #2563eb;
        /// }
        /// ```
        Blue600 => "decoration-blue-600",
        /// ```css
        /// {
        ///     text-decoration-color: #1d4ed8;
        /// }
        /// ```
        Blue700 => "decoration-blue-700",
        /// ```css
        /// {
        ///     text-decoration-color: #1e40af;
        /// }
        /// ```
        Blue800 => "decoration-blue-800",
        /// ```css
        /// {
        ///     text-decoration-color: #1e3a8a;
        /// }
        /// ```
        Blue900 => "decoration-blue-900",
        /// ```css
        /// {
        ///     text-decoration-color: #172554;
        /// }
        /// ```
        Blue950 => "decoration-blue-950",
        /// ```css
        /// {
        ///     text-decoration-color: #eef2ff;
        /// }
        /// ```
        Indigo50 => "decoration-indigo-50",
        /// ```css
        /// {
        ///     text-decoration-color: #e0e7ff;
        /// }
        /// ```
        Indigo100 => "decoration-indigo-100",
        /// ```css
        /// {
        ///     text-decoration-color: #c7d2fe;
        /// }
        /// ```
        Indigo200 => "decoration-indigo-200",
        /// ```css
        /// {
        ///     text-decoration-color: #a5b4fc;
        /// }
        /// ```
        Indigo300 => "decoration-indigo-300",
        /// ```css
        /// {
        ///     text-decoration-color: #818cf8;
        /// }
        /// ```
        Indigo400 => "decoration-indigo-400",
        /// ```css
        /// {
        ///     text-decoration-color: #6366f1;
        /// }
        /// ```
        Indigo500 => "decoration-indigo-500",
        /// ```css
        /// {
        ///     text-decoration-color: #4f46e5;
        /// }
        /// ```
        Indigo600 => "decoration-indigo-600",
        /// ```css
        /// {
        ///     text-decoration-color: #4338ca;
        /// }
        /// ```
        Indigo700 => "decoration-indigo-700",
        /// ```css
        /// {
        ///     text-decoration-color: #3730a3;
        /// }
        /// ```
        Indigo800 => "decoration-indigo-800",
        /// ```css
        /// {
        ///     text-decoration-color: #312e81;
        /// }
        /// ```
        Indigo900 => "decoration-indigo-900",
        /// ```css
        /// {
        ///     text-decoration-color: #1e1b4b;
        /// }
        /// ```
        Indigo950 => "decoration-indigo-950",
        /// ```css
        /// {
        ///     text-decoration-color: #f5f3ff;
        /// }
        /// ```
        Violet50 => "decoration-violet-50",
        /// ```css
        /// {
        ///     text-decoration-color: #ede9fe;
        /// }
        /// ```
        Violet100 => "decoration-violet-100",
        /// ```css
        /// {
        ///     text-decoration-color: #ddd6fe;
        /// }
        /// ```
        Violet200 => "decoration-violet-200",
        /// ```css
        /// {
        ///     text-decoration-color: #c4b5fd;
        /// }
        /// ```
        Violet300 => "decoration-violet-300",
        /// ```css
        /// {
        ///     text-decoration-color: #a78bfa;
        /// }
        /// ```
        Violet400 => "decoration-violet-400",
        /// ```css
        /// {
        ///     text-decoration-color: #8b5cf6;
        /// }
        /// ```
        Violet500 => "decoration-violet-500",
        /// ```css
        /// {
        ///     text-decoration-color: #7c3aed;
        /// }
        /// ```
        Violet600 => "decoration-violet-600",
        /// ```css
        /// {
        ///     text-decoration-color: #6d28d9;
        /// }
        /// ```
        Violet700 => "decoration-violet-700",
        /// ```css
        /// {
        ///     text-decoration-color: #5b21b6;
        /// }
        /// ```
        Violet800 => "decoration-violet-800",
        /// ```css
        /// {
        ///     text-decoration-color: #4c1d95;
        /// }
        /// ```
        Violet900 => "decoration-violet-900",
        /// ```css
        /// {
        ///     text-decoration-color: #2e1065;
        /// }
        /// ```
        Violet950 => "decoration-violet-950",
        /// ```css
        /// {
        ///     text-decoration-color: #faf5ff;
        /// }
        /// ```
        Purple50 => "decoration-purple-50",
        /// ```css
        /// {
        ///     text-decoration-color: #f3e8ff;
        /// }
        /// ```
        Purple100 => "decoration-purple-100",
        /// ```css
        /// {
        ///     text-decoration-color: #e9d5ff;
        /// }
        /// ```
        Purple200 => "decoration-purple-200",
        /// ```css
        /// {
        ///     text-decoration-color: #d8b4fe;
        /// }
        /// ```
        Purple300 => "decoration-purple-300",
        /// ```css
        /// {
        ///     text-decoration-color: #c084fc;
        /// }
        /// ```
        Purple400 => "decoration-purple-400",
        /// ```css
        /// {
        ///     text-decoration-color: #a855f7;
        /// }
        /// ```
        Purple500 => "decoration-purple-500",
        /// ```css
        /// {
        ///     text-decoration-color: #9333ea;
        /// }
        /// ```
        Purple600 => "decoration-purple-600",
        /// ```css
        /// {
        ///     text-decoration-color: #7e22ce;
        /// }
        /// ```
        Purple700 => "decoration-purple-700",
        /// ```css
        /// {
        ///     text-decoration-color: #6b21a8;
        /// }
        /// ```
        Purple800 => "decoration-purple-800",
        /// ```css
        /// {
        ///     text-decoration-color: #581c87;
        /// }
        /// ```
        Purple900 => "decoration-purple-900",
        /// ```css
        /// {
        ///     text-decoration-color: #3b0764;
        /// }
        /// ```
        Purple950 => "decoration-purple-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fdf4ff;
        /// }
        /// ```
        Fuchsia50 => "decoration-fuchsia-50",
        /// ```css
        /// {
        ///     text-decoration-color: #fae8ff;
        /// }
        /// ```
        Fuchsia100 => "decoration-fuchsia-100",
        /// ```css
        /// {
        ///     text-decoration-color: #f5d0fe;
        /// }
        /// ```
        Fuchsia200 => "decoration-fuchsia-200",
        /// ```css
        /// {
        ///     text-decoration-color: #f0abfc;
        /// }
        /// ```
        Fuchsia300 => "decoration-fuchsia-300",
        /// ```css
        /// {
        ///     text-decoration-color: #e879f9;
        /// }
        /// ```
        Fuchsia400 => "decoration-fuchsia-400",
        /// ```css
        /// {
        ///     text-decoration-color: #d946ef;
        /// }
        /// ```
        Fuchsia500 => "decoration-fuchsia-500",
        /// ```css
        /// {
        ///     text-decoration-color: #c026d3;
        /// }
        /// ```
        Fuchsia600 => "decoration-fuchsia-600",
        /// ```css
        /// {
        ///     text-decoration-color: #a21caf;
        /// }
        /// ```
        Fuchsia700 => "decoration-fuchsia-700",
        /// ```css
        /// {
        ///     text-decoration-color: #86198f;
        /// }
        /// ```
        Fuchsia800 => "decoration-fuchsia-800",
        /// ```css
        /// {
        ///     text-decoration-color: #701a75;
        /// }
        /// ```
        Fuchsia900 => "decoration-fuchsia-900",
        /// ```css
        /// {
        ///     text-decoration-color: #4a044e;
        /// }
        /// ```
        Fuchsia950 => "decoration-fuchsia-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fdf2f8;
        /// }
        /// ```
        Pink50 => "decoration-pink-50",
        /// ```css
        /// {
        ///     text-decoration-color: #fce7f3;
        /// }
        /// ```
        Pink100 => "decoration-pink-100",
        /// ```css
        /// {
        ///     text-decoration-color: #fbcfe8;
        /// }
        /// ```
        Pink200 => "decoration-pink-200",
        /// ```css
        /// {
        ///     text-decoration-color: #f9a8d4;
        /// }
        /// ```
        Pink300 => "decoration-pink-300",
        /// ```css
        /// {
        ///     text-decoration-color: #f472b6;
        /// }
        /// ```
        Pink400 => "decoration-pink-400",
        /// ```css
        /// {
        ///     text-decoration-color: #ec4899;
        /// }
        /// ```
        Pink500 => "decoration-pink-500",
        /// ```css
        /// {
        ///     text-decoration-color: #db2777;
        /// }
        /// ```
        Pink600 => "decoration-pink-600",
        /// ```css
        /// {
        ///     text-decoration-color: #be185d;
        /// }
        /// ```
        Pink700 => "decoration-pink-700",
        /// ```css
        /// {
        ///     text-decoration-color: #9d174d;
        /// }
        /// ```
        Pink800 => "decoration-pink-800",
        /// ```css
        /// {
        ///     text-decoration-color: #831843;
        /// }
        /// ```
        Pink900 => "decoration-pink-900",
        /// ```css
        /// {
        ///     text-decoration-color: #500724;
        /// }
        /// ```
        Pink950 => "decoration-pink-950",
        /// ```css
        /// {
        ///     text-decoration-color: #fff1f2;
        /// }
        /// ```
        Rose50 => "decoration-rose-50",
        /// ```css
        /// {
        ///     text-decoration-color: #ffe4e6;
        /// }
        /// ```
        Rose100 => "decoration-rose-100",
        /// ```css
        /// {
        ///     text-decoration-color: #fecdd3;
        /// }
        /// ```
        Rose200 => "decoration-rose-200",
        /// ```css
        /// {
        ///     text-decoration-color: #fda4af;
        /// }
        /// ```
        Rose300 => "decoration-rose-300",
        /// ```css
        /// {
        ///     text-decoration-color: #fb7185;
        /// }
        /// ```
        Rose400 => "decoration-rose-400",
        /// ```css
        /// {
        ///     text-decoration-color: #f43f5e;
        /// }
        /// ```
        Rose500 => "decoration-rose-500",
        /// ```css
        /// {
        ///     text-decoration-color: #e11d48;
        /// }
        /// ```
        Rose600 => "decoration-rose-600",
        /// ```css
        /// {
        ///     text-decoration-color: #be123c;
        /// }
        /// ```
        Rose700 => "decoration-rose-700",
        /// ```css
        /// {
        ///     text-decoration-color: #9f1239;
        /// }
        /// ```
        Rose800 => "decoration-rose-800",
        /// ```css
        /// {
        ///     text-decoration-color: #881337;
        /// }
        /// ```
        Rose900 => "decoration-rose-900",
        /// ```css
        /// {
        ///     text-decoration-color: #4c0519;
        /// }
        /// ```
        Rose950 => "decoration-rose-950",
    }
    /// Utilities for controlling the style of text decorations.
    ///
    /// <https://tailwindcss.com/docs/text-decoration-style>
    TextDecorationStyle {
        /// ```css
        /// {
        ///     text-decoration-style: solid;
        /// }
        /// ```
        Solid => "decoration-solid",
        /// ```css
        /// {
        ///     text-decoration-style: double;
        /// }
        /// ```
        Double => "decoration-double",
        /// ```css
        /// {
        ///     text-decoration-style: dotted;
        /// }
        /// ```
        Dotted => "decoration-dotted",
        /// ```css
        /// {
        ///     text-decoration-style: dashed;
        /// }
        /// ```
        Dashed => "decoration-dashed",
        /// ```css
        /// {
        ///     text-decoration-style: wavy;
        /// }
        /// ```
        Wavy => "decoration-wavy",
    }
    /// Utilities for controlling the thickness of text decorations.
    ///
    /// <https://tailwindcss.com/docs/text-decoration-thickness>
    TextDecorationThickness {
        /// ```css
        /// {
        ///     text-decoration-thickness: auto;
        /// }
        /// ```
        Auto => "decoration-auto",
        /// ```css
        /// {
        ///     text-decoration-thickness: from-font;
        /// }
        /// ```
        FromFont => "decoration-from-font",
        /// ```css
        /// {
        ///     text-decoration-thickness: 0px;
        /// }
        /// ```
        _0 => "decoration-0",
        /// ```css
        /// {
        ///     text-decoration-thickness: 1px;
        /// }
        /// ```
        _1 => "decoration-1",
        /// ```css
        /// {
        ///     text-decoration-thickness: 2px;
        /// }
        /// ```
        _2 => "decoration-2",
        /// ```css
        /// {
        ///     text-decoration-thickness: 4px;
        /// }
        /// ```
        _4 => "decoration-4",
        /// ```css
        /// {
        ///     text-decoration-thickness: 8px;
        /// }
        /// ```
        _8 => "decoration-8",
    }
    /// Utilities for controlling the offset of a text underline.
    ///
    /// <https://tailwindcss.com/docs/text-underline-offset>
    TextUnderlineOffset {
        /// ```css
        /// {
        ///     text-underline-offset: auto;
        /// }
        /// ```
        Auto => "underline-offset-auto",
        /// ```css
        /// {
        ///     text-underline-offset: 0px;
        /// }
        /// ```
        _0 => "underline-offset-0",
        /// ```css
        /// {
        ///     text-underline-offset: 1px;
        /// }
        /// ```
        _1 => "underline-offset-1",
        /// ```css
        /// {
        ///     text-underline-offset: 2px;
        /// }
        /// ```
        _2 => "underline-offset-2",
        /// ```css
        /// {
        ///     text-underline-offset: 4px;
        /// }
        /// ```
        _4 => "underline-offset-4",
        /// ```css
        /// {
        ///     text-underline-offset: 8px;
        /// }
        /// ```
        _8 => "underline-offset-8",
    }
    /// Utilities for controlling the transformation of text.
    ///
    /// <https://tailwindcss.com/docs/text-transform>
    TextTransform {
        /// ```css
        /// {
        ///     text-transform: uppercase;
        /// }
        /// ```
        Uppercase => "uppercase",
        /// ```css
        /// {
        ///     text-transform: lowercase;
        /// }
        /// ```
        Lowercase => "lowercase",
        /// ```css
        /// {
        ///     text-transform: capitalize;
        /// }
        /// ```
        Capitalize => "capitalize",
        /// ```css
        /// {
        ///     text-transform: none;
        /// }
        /// ```
        NormalCase => "normal-case",
    }
    /// Utilities for controlling text overflow in an element.
    ///
    /// <https://tailwindcss.com/docs/text-overflow>
    TextOverflow {
        /// ```css
        /// {
        ///     overflow: hidden;
        ///     text-overflow: ellipsis;
        ///     white-space: nowrap;
        /// }
        /// ```
        Truncate => "truncate",
        /// ```css
        /// {
        ///     text-overflow: ellipsis;
        /// }
        /// ```
        TextEllipsis => "text-ellipsis",
        /// ```css
        /// {
        ///     text-overflow: clip;
        /// }
        /// ```
        TextClip => "text-clip",
    }
    /// Utilities for controlling how text wraps within an element.
    ///
    /// <https://tailwindcss.com/docs/text-wrap>
    TextWrap {
        /// ```css
        /// {
        ///     text-wrap: wrap;
        /// }
        /// ```
        Wrap => "text-wrap",
        /// ```css
        /// {
        ///     text-wrap: nowrap;
        /// }
        /// ```
        Nowrap => "text-nowrap",
        /// ```css
        /// {
        ///     text-wrap: balance;
        /// }
        /// ```
        Balance => "text-balance",
        /// ```css
        /// {
        ///     text-wrap: pretty;
        /// }
        /// ```
        Pretty => "text-pretty",
    }
    /// Utilities for controlling the amount of empty space shown before text in a block.
    ///
    /// <https://tailwindcss.com/docs/text-indent>
    TextIndent {
        /// ```css
        /// {
        ///     text-indent: 0px;
        /// }
        /// ```
        _0 => "indent-0",
        /// ```css
        /// {
        ///     text-indent: 1px;
        /// }
        /// ```
        Px => "indent-px",
        /// ```css
        /// {
        ///     text-indent: 0.125rem; /* 2px */
        /// }
        /// ```
        _0_5 => "indent-0.5",
        /// ```css
        /// {
        ///     text-indent: 0.25rem; /* 4px */
        /// }
        /// ```
        _1 => "indent-1",
        /// ```css
        /// {
        ///     text-indent: 0.375rem; /* 6px */
        /// }
        /// ```
        _1_5 => "indent-1.5",
        /// ```css
        /// {
        ///     text-indent: 0.5rem; /* 8px */
        /// }
        /// ```
        _2 => "indent-2",
        /// ```css
        /// {
        ///     text-indent: 0.625rem; /* 10px */
        /// }
        /// ```
        _2_5 => "indent-2.5",
        /// ```css
        /// {
        ///     text-indent: 0.75rem; /* 12px */
        /// }
        /// ```
        _3 => "indent-3",
        /// ```css
        /// {
        ///     text-indent: 0.875rem; /* 14px */
        /// }
        /// ```
        _3_5 => "indent-3.5",
        /// ```css
        /// {
        ///     text-indent: 1rem; /* 16px */
        /// }
        /// ```
        _4 => "indent-4",
        /// ```css
        /// {
        ///     text-indent: 1.25rem; /* 20px */
        /// }
        /// ```
        _5 => "indent-5",
        /// ```css
        /// {
        ///     text-indent: 1.5rem; /* 24px */
        /// }
        /// ```
        _6 => "indent-6",
        /// ```css
        /// {
        ///     text-indent: 1.75rem; /* 28px */
        /// }
        /// ```
        _7 => "indent-7",
        /// ```css
        /// {
        ///     text-indent: 2rem; /* 32px */
        /// }
        /// ```
        _8 => "indent-8",
        /// ```css
        /// {
        ///     text-indent: 2.25rem; /* 36px */
        /// }
        /// ```
        _9 => "indent-9",
        /// ```css
        /// {
        ///     text-indent: 2.5rem; /* 40px */
        /// }
        /// ```
        _10 => "indent-10",
        /// ```css
        /// {
        ///     text-indent: 2.75rem; /* 44px */
        /// }
        /// ```
        _11 => "indent-11",
        /// ```css
        /// {
        ///     text-indent: 3rem; /* 48px */
        /// }
        /// ```
        _12 => "indent-12",
        /// ```css
        /// {
        ///     text-indent: 3.5rem; /* 56px */
        /// }
        /// ```
        _14 => "indent-14",
        /// ```css
        /// {
        ///     text-indent: 4rem; /* 64px */
        /// }
        /// ```
        _16 => "indent-16",
        /// ```css
        /// {
        ///     text-indent: 5rem; /* 80px */
        /// }
        /// ```
        _20 => "indent-20",
        /// ```css
        /// {
        ///     text-indent: 6rem; /* 96px */
        /// }
        /// ```
        _24 => "indent-24",
        /// ```css
        /// {
        ///     text-indent: 7rem; /* 112px */
        /// }
        /// ```
        _28 => "indent-28",
        /// ```css
        /// {
        ///     text-indent: 8rem; /* 128px */
        /// }
        /// ```
        _32 => "indent-32",
        /// ```css
        /// {
        ///     text-indent: 9rem; /* 144px */
        /// }
        /// ```
        _36 => "indent-36",
        /// ```css
        /// {
        ///     text-indent: 10rem; /* 160px */
        /// }
        /// ```
        _40 => "indent-40",
        /// ```css
        /// {
        ///     text-indent: 11rem; /* 176px */
        /// }
        /// ```
        _44 => "indent-44",
        /// ```css
        /// {
        ///     text-indent: 12rem; /* 192px */
        /// }
        /// ```
        _48 => "indent-48",
        /// ```css
        /// {
        ///     text-indent: 13rem; /* 208px */
        /// }
        /// ```
        _52 => "indent-52",
        /// ```css
        /// {
        ///     text-indent: 14rem; /* 224px */
        /// }
        /// ```
        _56 => "indent-56",
        /// ```css
        /// {
        ///     text-indent: 15rem; /* 240px */
        /// }
        /// ```
        _60 => "indent-60",
        /// ```css
        /// {
        ///     text-indent: 16rem; /* 256px */
        /// }
        /// ```
        _64 => "indent-64",
        /// ```css
        /// {
        ///     text-indent: 18rem; /* 288px */
        /// }
        /// ```
        _72 => "indent-72",
        /// ```css
        /// {
        ///     text-indent: 20rem; /* 320px */
        /// }
        /// ```
        _80 => "indent-80",
        /// ```css
        /// {
        ///     text-indent: 24rem; /* 384px */
        /// }
        /// ```
        _96 => "indent-96",
    }
    /// Utilities for controlling the vertical alignment of an inline or table-cell box.
    ///
    /// <https://tailwindcss.com/docs/vertical-align>
    VerticalAlign {
        /// ```css
        /// {
        ///     vertical-align: baseline;
        /// }
        /// ```
        Baseline => "align-baseline",
        /// ```css
        /// {
        ///     vertical-align: top;
        /// }
        /// ```
        Top => "align-top",
        /// ```css
        /// {
        ///     vertical-align: middle;
        /// }
        /// ```
        Middle => "align-middle",
        /// ```css
        /// {
        ///     vertical-align: bottom;
        /// }
        /// ```
        Bottom => "align-bottom",
        /// ```css
        /// {
        ///     vertical-align: text-top;
        /// }
        /// ```
        TextTop => "align-text-top",
        /// ```css
        /// {
        ///     vertical-align: text-bottom;
        /// }
        /// ```
        TextBottom => "align-text-bottom",
        /// ```css
        /// {
        ///     vertical-align: sub;
        /// }
        /// ```
        Sub => "align-sub",
        /// ```css
        /// {
        ///     vertical-align: super;
        /// }
        /// ```
        Super => "align-super",
    }
    /// Utilities for controlling an element's white-space property.
    ///
    /// <https://tailwindcss.com/docs/whitespace>
    Whitespace {
        /// ```css
        /// {
        ///     white-space: normal;
        /// }
        /// ```
        Normal => "whitespace-normal",
        /// ```css
        /// {
        ///     white-space: nowrap;
        /// }
        /// ```
        Nowrap => "whitespace-nowrap",
        /// ```css
        /// {
        ///     white-space: pre;
        /// }
        /// ```
        Pre => "whitespace-pre",
        /// ```css
        /// {
        ///     white-space: pre-line;
        /// }
        /// ```
        PreLine => "whitespace-pre-line",
        /// ```css
        /// {
        ///     white-space: pre-wrap;
        /// }
        /// ```
        PreWrap => "whitespace-pre-wrap",
        /// ```css
        /// {
        ///     white-space: break-spaces;
        /// }
        /// ```
        BreakSpaces => "whitespace-break-spaces",
    }
    /// Utilities for controlling word breaks in an element.
    ///
    /// <https://tailwindcss.com/docs/word-break>
    WordBreak {
        /// ```css
        /// {
        ///     overflow-wrap: normal;
        ///     word-break: normal;
        /// }
        /// ```
        Normal => "break-normal",
        /// ```css
        /// {
        ///     overflow-wrap: break-word;
        /// }
        /// ```
        Words => "break-words",
        /// ```css
        /// {
        ///     word-break: break-all;
        /// }
        /// ```
        All => "break-all",
        /// ```css
        /// {
        ///     word-break: keep-all;
        /// }
        /// ```
        Keep => "break-keep",
    }
    /// Utilities for controlling how words should be hyphenated.
    ///
    /// <https://tailwindcss.com/docs/hyphens>
    Hyphens {
        /// ```css
        /// {
        ///     hyphens: none;
        /// }
        /// ```
        None => "hyphens-none",
        /// ```css
        /// {
        ///     hyphens: manual;
        /// }
        /// ```
        Manual => "hyphens-manual",
        /// ```css
        /// {
        ///     hyphens: auto;
        /// }
        /// ```
        Auto => "hyphens-auto",
    }
    /// Utilities for controlling the content of the before and after pseudo-elements.
    ///
    /// <https://tailwindcss.com/docs/content>
    Content {
        /// ```css
        /// {
        ///     content: none;
        /// }
        /// ```
        None => "content-none",
    }
);
