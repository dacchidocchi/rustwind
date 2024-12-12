def_types!(
    /// Utilities for controlling the box shadow of an element.
    ///
    /// <https://tailwindcss.com/docs/box-shadow>
    BoxShadow {
        /// ```css
        /// {
        ///     box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
        /// }
        /// ```
        Sm => "shadow-sm",
        /// ```css
        /// {
        ///     box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);
        /// }
        /// ```
        Shadow => "shadow",
        /// ```css
        /// {
        ///     box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
        /// }
        /// ```
        Md => "shadow-md",
        /// ```css
        /// {
        ///     box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
        /// }
        /// ```
        Lg => "shadow-lg",
        /// ```css
        /// {
        ///     box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
        /// }
        /// ```
        Xl => "shadow-xl",
        /// ```css
        /// {
        ///     box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
        /// }
        /// ```
        _2xl => "shadow-2xl",
        /// ```css
        /// {
        ///     box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);
        /// }
        /// ```
        Inner => "shadow-inner",
        /// ```css
        /// {
        ///     box-shadow: 0 0 #0000;
        /// }
        /// ```
        None => "shadow-none",
    }
    /// Utilities for controlling the color of a box shadow.
    ///
    /// <https://tailwindcss.com/docs/box-shadow-color>
    BoxShadowColor {
        /// ```css
        /// {
        ///     --tw-shadow-color: inherit;
        /// }
        /// ```
        Inherit => "shadow-inherit",
        /// ```css
        /// {
        ///     --tw-shadow-color: currentColor;
        /// }
        /// ```
        Current => "shadow-current",
        /// ```css
        /// {
        ///     --tw-shadow-color: transparent;
        /// }
        /// ```
        Transparent => "shadow-transparent",
        /// ```css
        /// {
        ///     --tw-shadow-color: #000;
        /// }
        /// ```
        Black => "shadow-black",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fff;
        /// }
        /// ```
        White => "shadow-white",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f8fafc;
        /// }
        /// ```
        Slate50 => "shadow-slate-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f1f5f9;
        /// }
        /// ```
        Slate100 => "shadow-slate-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e2e8f0;
        /// }
        /// ```
        Slate200 => "shadow-slate-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #cbd5e1;
        /// }
        /// ```
        Slate300 => "shadow-slate-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #94a3b8;
        /// }
        /// ```
        Slate400 => "shadow-slate-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #64748b;
        /// }
        /// ```
        Slate500 => "shadow-slate-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #475569;
        /// }
        /// ```
        Slate600 => "shadow-slate-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #334155;
        /// }
        /// ```
        Slate700 => "shadow-slate-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #1e293b;
        /// }
        /// ```
        Slate800 => "shadow-slate-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0f172a;
        /// }
        /// ```
        Slate900 => "shadow-slate-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #020617;
        /// }
        /// ```
        Slate950 => "shadow-slate-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f9fafb;
        /// }
        /// ```
        Gray50 => "shadow-gray-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f3f4f6;
        /// }
        /// ```
        Gray100 => "shadow-gray-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e5e7eb;
        /// }
        /// ```
        Gray200 => "shadow-gray-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d1d5db;
        /// }
        /// ```
        Gray300 => "shadow-gray-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #9ca3af;
        /// }
        /// ```
        Gray400 => "shadow-gray-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #6b7280;
        /// }
        /// ```
        Gray500 => "shadow-gray-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #4b5563;
        /// }
        /// ```
        Gray600 => "shadow-gray-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #374151;
        /// }
        /// ```
        Gray700 => "shadow-gray-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #1f2937;
        /// }
        /// ```
        Gray800 => "shadow-gray-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #111827;
        /// }
        /// ```
        Gray900 => "shadow-gray-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #030712;
        /// }
        /// ```
        Gray950 => "shadow-gray-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fafafa;
        /// }
        /// ```
        Zinc50 => "shadow-zinc-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f4f4f5;
        /// }
        /// ```
        Zinc100 => "shadow-zinc-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e4e4e7;
        /// }
        /// ```
        Zinc200 => "shadow-zinc-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d4d4d8;
        /// }
        /// ```
        Zinc300 => "shadow-zinc-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a1a1aa;
        /// }
        /// ```
        Zinc400 => "shadow-zinc-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #71717a;
        /// }
        /// ```
        Zinc500 => "shadow-zinc-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #52525b;
        /// }
        /// ```
        Zinc600 => "shadow-zinc-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #3f3f46;
        /// }
        /// ```
        Zinc700 => "shadow-zinc-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #27272a;
        /// }
        /// ```
        Zinc800 => "shadow-zinc-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #18181b;
        /// }
        /// ```
        Zinc900 => "shadow-zinc-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #09090b;
        /// }
        /// ```
        Zinc950 => "shadow-zinc-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fafafa;
        /// }
        /// ```
        Neutral50 => "shadow-neutral-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f5f5f5;
        /// }
        /// ```
        Neutral100 => "shadow-neutral-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e5e5e5;
        /// }
        /// ```
        Neutral200 => "shadow-neutral-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d4d4d4;
        /// }
        /// ```
        Neutral300 => "shadow-neutral-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a3a3a3;
        /// }
        /// ```
        Neutral400 => "shadow-neutral-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #737373;
        /// }
        /// ```
        Neutral500 => "shadow-neutral-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #525252;
        /// }
        /// ```
        Neutral600 => "shadow-neutral-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #404040;
        /// }
        /// ```
        Neutral700 => "shadow-neutral-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #262626;
        /// }
        /// ```
        Neutral800 => "shadow-neutral-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #171717;
        /// }
        /// ```
        Neutral900 => "shadow-neutral-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0a0a0a;
        /// }
        /// ```
        Neutral950 => "shadow-neutral-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fafaf9;
        /// }
        /// ```
        Stone50 => "shadow-stone-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f5f5f4;
        /// }
        /// ```
        Stone100 => "shadow-stone-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e7e5e4;
        /// }
        /// ```
        Stone200 => "shadow-stone-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d6d3d1;
        /// }
        /// ```
        Stone300 => "shadow-stone-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a8a29e;
        /// }
        /// ```
        Stone400 => "shadow-stone-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #78716c;
        /// }
        /// ```
        Stone500 => "shadow-stone-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #57534e;
        /// }
        /// ```
        Stone600 => "shadow-stone-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #44403c;
        /// }
        /// ```
        Stone700 => "shadow-stone-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #292524;
        /// }
        /// ```
        Stone800 => "shadow-stone-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #1c1917;
        /// }
        /// ```
        Stone900 => "shadow-stone-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0c0a09;
        /// }
        /// ```
        Stone950 => "shadow-stone-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fef2f2;
        /// }
        /// ```
        Red50 => "shadow-red-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fee2e2;
        /// }
        /// ```
        Red100 => "shadow-red-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fecaca;
        /// }
        /// ```
        Red200 => "shadow-red-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fca5a5;
        /// }
        /// ```
        Red300 => "shadow-red-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f87171;
        /// }
        /// ```
        Red400 => "shadow-red-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ef4444;
        /// }
        /// ```
        Red500 => "shadow-red-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #dc2626;
        /// }
        /// ```
        Red600 => "shadow-red-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #b91c1c;
        /// }
        /// ```
        Red700 => "shadow-red-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #991b1b;
        /// }
        /// ```
        Red800 => "shadow-red-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #7f1d1d;
        /// }
        /// ```
        Red900 => "shadow-red-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #450a0a;
        /// }
        /// ```
        Red950 => "shadow-red-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fff7ed;
        /// }
        /// ```
        Orange50 => "shadow-orange-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ffedd5;
        /// }
        /// ```
        Orange100 => "shadow-orange-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fed7aa;
        /// }
        /// ```
        Orange200 => "shadow-orange-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fdba74;
        /// }
        /// ```
        Orange300 => "shadow-orange-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fb923c;
        /// }
        /// ```
        Orange400 => "shadow-orange-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f97316;
        /// }
        /// ```
        Orange500 => "shadow-orange-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ea580c;
        /// }
        /// ```
        Orange600 => "shadow-orange-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #c2410c;
        /// }
        /// ```
        Orange700 => "shadow-orange-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #9a3412;
        /// }
        /// ```
        Orange800 => "shadow-orange-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #7c2d12;
        /// }
        /// ```
        Orange900 => "shadow-orange-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #431407;
        /// }
        /// ```
        Orange950 => "shadow-orange-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fffbeb;
        /// }
        /// ```
        Amber50 => "shadow-amber-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fef3c7;
        /// }
        /// ```
        Amber100 => "shadow-amber-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fde68a;
        /// }
        /// ```
        Amber200 => "shadow-amber-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fcd34d;
        /// }
        /// ```
        Amber300 => "shadow-amber-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fbbf24;
        /// }
        /// ```
        Amber400 => "shadow-amber-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f59e0b;
        /// }
        /// ```
        Amber500 => "shadow-amber-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d97706;
        /// }
        /// ```
        Amber600 => "shadow-amber-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #b45309;
        /// }
        /// ```
        Amber700 => "shadow-amber-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #92400e;
        /// }
        /// ```
        Amber800 => "shadow-amber-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #78350f;
        /// }
        /// ```
        Amber900 => "shadow-amber-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #451a03;
        /// }
        /// ```
        Amber950 => "shadow-amber-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fefce8;
        /// }
        /// ```
        Yellow50 => "shadow-yellow-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fef9c3;
        /// }
        /// ```
        Yellow100 => "shadow-yellow-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fef08a;
        /// }
        /// ```
        Yellow200 => "shadow-yellow-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fde047;
        /// }
        /// ```
        Yellow300 => "shadow-yellow-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #facc15;
        /// }
        /// ```
        Yellow400 => "shadow-yellow-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #eab308;
        /// }
        /// ```
        Yellow500 => "shadow-yellow-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ca8a04;
        /// }
        /// ```
        Yellow600 => "shadow-yellow-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a16207;
        /// }
        /// ```
        Yellow700 => "shadow-yellow-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #854d0e;
        /// }
        /// ```
        Yellow800 => "shadow-yellow-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #713f12;
        /// }
        /// ```
        Yellow900 => "shadow-yellow-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #422006;
        /// }
        /// ```
        Yellow950 => "shadow-yellow-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f7fee7;
        /// }
        /// ```
        Lime50 => "shadow-lime-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ecfccb;
        /// }
        /// ```
        Lime100 => "shadow-lime-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d9f99d;
        /// }
        /// ```
        Lime200 => "shadow-lime-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #bef264;
        /// }
        /// ```
        Lime300 => "shadow-lime-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a3e635;
        /// }
        /// ```
        Lime400 => "shadow-lime-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #84cc16;
        /// }
        /// ```
        Lime500 => "shadow-lime-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #65a30d;
        /// }
        /// ```
        Lime600 => "shadow-lime-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #4d7c0f;
        /// }
        /// ```
        Lime700 => "shadow-lime-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #3f6212;
        /// }
        /// ```
        Lime800 => "shadow-lime-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #365314;
        /// }
        /// ```
        Lime900 => "shadow-lime-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #1a2e05;
        /// }
        /// ```
        Lime950 => "shadow-lime-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f0fdf4;
        /// }
        /// ```
        Green50 => "shadow-green-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #dcfce7;
        /// }
        /// ```
        Green100 => "shadow-green-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #bbf7d0;
        /// }
        /// ```
        Green200 => "shadow-green-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #86efac;
        /// }
        /// ```
        Green300 => "shadow-green-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #4ade80;
        /// }
        /// ```
        Green400 => "shadow-green-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #22c55e;
        /// }
        /// ```
        Green500 => "shadow-green-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #16a34a;
        /// }
        /// ```
        Green600 => "shadow-green-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #15803d;
        /// }
        /// ```
        Green700 => "shadow-green-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #166534;
        /// }
        /// ```
        Green800 => "shadow-green-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #14532d;
        /// }
        /// ```
        Green900 => "shadow-green-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #052e16;
        /// }
        /// ```
        Green950 => "shadow-green-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ecfdf5;
        /// }
        /// ```
        Emerald50 => "shadow-emerald-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d1fae5;
        /// }
        /// ```
        Emerald100 => "shadow-emerald-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a7f3d0;
        /// }
        /// ```
        Emerald200 => "shadow-emerald-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #6ee7b7;
        /// }
        /// ```
        Emerald300 => "shadow-emerald-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #34d399;
        /// }
        /// ```
        Emerald400 => "shadow-emerald-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #10b981;
        /// }
        /// ```
        Emerald500 => "shadow-emerald-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #059669;
        /// }
        /// ```
        Emerald600 => "shadow-emerald-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #047857;
        /// }
        /// ```
        Emerald700 => "shadow-emerald-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #065f46;
        /// }
        /// ```
        Emerald800 => "shadow-emerald-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #064e3b;
        /// }
        /// ```
        Emerald900 => "shadow-emerald-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #022c22;
        /// }
        /// ```
        Emerald950 => "shadow-emerald-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f0fdfa;
        /// }
        /// ```
        Teal50 => "shadow-teal-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ccfbf1;
        /// }
        /// ```
        Teal100 => "shadow-teal-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #99f6e4;
        /// }
        /// ```
        Teal200 => "shadow-teal-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #5eead4;
        /// }
        /// ```
        Teal300 => "shadow-teal-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #2dd4bf;
        /// }
        /// ```
        Teal400 => "shadow-teal-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #14b8a6;
        /// }
        /// ```
        Teal500 => "shadow-teal-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0d9488;
        /// }
        /// ```
        Teal600 => "shadow-teal-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0f766e;
        /// }
        /// ```
        Teal700 => "shadow-teal-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #115e59;
        /// }
        /// ```
        Teal800 => "shadow-teal-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #134e4a;
        /// }
        /// ```
        Teal900 => "shadow-teal-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #042f2e;
        /// }
        /// ```
        Teal950 => "shadow-teal-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ecfeff;
        /// }
        /// ```
        Cyan50 => "shadow-cyan-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #cffafe;
        /// }
        /// ```
        Cyan100 => "shadow-cyan-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a5f3fc;
        /// }
        /// ```
        Cyan200 => "shadow-cyan-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #67e8f9;
        /// }
        /// ```
        Cyan300 => "shadow-cyan-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #22d3ee;
        /// }
        /// ```
        Cyan400 => "shadow-cyan-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #06b6d4;
        /// }
        /// ```
        Cyan500 => "shadow-cyan-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0891b2;
        /// }
        /// ```
        Cyan600 => "shadow-cyan-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0e7490;
        /// }
        /// ```
        Cyan700 => "shadow-cyan-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #155e75;
        /// }
        /// ```
        Cyan800 => "shadow-cyan-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #164e63;
        /// }
        /// ```
        Cyan900 => "shadow-cyan-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #083344;
        /// }
        /// ```
        Cyan950 => "shadow-cyan-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f0f9ff;
        /// }
        /// ```
        Sky50 => "shadow-sky-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e0f2fe;
        /// }
        /// ```
        Sky100 => "shadow-sky-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #bae6fd;
        /// }
        /// ```
        Sky200 => "shadow-sky-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #7dd3fc;
        /// }
        /// ```
        Sky300 => "shadow-sky-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #38bdf8;
        /// }
        /// ```
        Sky400 => "shadow-sky-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0ea5e9;
        /// }
        /// ```
        Sky500 => "shadow-sky-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0284c7;
        /// }
        /// ```
        Sky600 => "shadow-sky-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0369a1;
        /// }
        /// ```
        Sky700 => "shadow-sky-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #075985;
        /// }
        /// ```
        Sky800 => "shadow-sky-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #0c4a6e;
        /// }
        /// ```
        Sky900 => "shadow-sky-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #082f49;
        /// }
        /// ```
        Sky950 => "shadow-sky-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #eff6ff;
        /// }
        /// ```
        Blue50 => "shadow-blue-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #dbeafe;
        /// }
        /// ```
        Blue100 => "shadow-blue-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #bfdbfe;
        /// }
        /// ```
        Blue200 => "shadow-blue-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #93c5fd;
        /// }
        /// ```
        Blue300 => "shadow-blue-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #60a5fa;
        /// }
        /// ```
        Blue400 => "shadow-blue-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #3b82f6;
        /// }
        /// ```
        Blue500 => "shadow-blue-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #2563eb;
        /// }
        /// ```
        Blue600 => "shadow-blue-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #1d4ed8;
        /// }
        /// ```
        Blue700 => "shadow-blue-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #1e40af;
        /// }
        /// ```
        Blue800 => "shadow-blue-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #1e3a8a;
        /// }
        /// ```
        Blue900 => "shadow-blue-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #172554;
        /// }
        /// ```
        Blue950 => "shadow-blue-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #eef2ff;
        /// }
        /// ```
        Indigo50 => "shadow-indigo-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e0e7ff;
        /// }
        /// ```
        Indigo100 => "shadow-indigo-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #c7d2fe;
        /// }
        /// ```
        Indigo200 => "shadow-indigo-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a5b4fc;
        /// }
        /// ```
        Indigo300 => "shadow-indigo-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #818cf8;
        /// }
        /// ```
        Indigo400 => "shadow-indigo-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #6366f1;
        /// }
        /// ```
        Indigo500 => "shadow-indigo-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #4f46e5;
        /// }
        /// ```
        Indigo600 => "shadow-indigo-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #4338ca;
        /// }
        /// ```
        Indigo700 => "shadow-indigo-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #3730a3;
        /// }
        /// ```
        Indigo800 => "shadow-indigo-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #312e81;
        /// }
        /// ```
        Indigo900 => "shadow-indigo-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #1e1b4b;
        /// }
        /// ```
        Indigo950 => "shadow-indigo-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f5f3ff;
        /// }
        /// ```
        Violet50 => "shadow-violet-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ede9fe;
        /// }
        /// ```
        Violet100 => "shadow-violet-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ddd6fe;
        /// }
        /// ```
        Violet200 => "shadow-violet-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #c4b5fd;
        /// }
        /// ```
        Violet300 => "shadow-violet-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a78bfa;
        /// }
        /// ```
        Violet400 => "shadow-violet-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #8b5cf6;
        /// }
        /// ```
        Violet500 => "shadow-violet-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #7c3aed;
        /// }
        /// ```
        Violet600 => "shadow-violet-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #6d28d9;
        /// }
        /// ```
        Violet700 => "shadow-violet-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #5b21b6;
        /// }
        /// ```
        Violet800 => "shadow-violet-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #4c1d95;
        /// }
        /// ```
        Violet900 => "shadow-violet-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #2e1065;
        /// }
        /// ```
        Violet950 => "shadow-violet-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #faf5ff;
        /// }
        /// ```
        Purple50 => "shadow-purple-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f3e8ff;
        /// }
        /// ```
        Purple100 => "shadow-purple-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e9d5ff;
        /// }
        /// ```
        Purple200 => "shadow-purple-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d8b4fe;
        /// }
        /// ```
        Purple300 => "shadow-purple-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #c084fc;
        /// }
        /// ```
        Purple400 => "shadow-purple-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a855f7;
        /// }
        /// ```
        Purple500 => "shadow-purple-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #9333ea;
        /// }
        /// ```
        Purple600 => "shadow-purple-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #7e22ce;
        /// }
        /// ```
        Purple700 => "shadow-purple-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #6b21a8;
        /// }
        /// ```
        Purple800 => "shadow-purple-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #581c87;
        /// }
        /// ```
        Purple900 => "shadow-purple-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #3b0764;
        /// }
        /// ```
        Purple950 => "shadow-purple-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fdf4ff;
        /// }
        /// ```
        Fuchsia50 => "shadow-fuchsia-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fae8ff;
        /// }
        /// ```
        Fuchsia100 => "shadow-fuchsia-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f5d0fe;
        /// }
        /// ```
        Fuchsia200 => "shadow-fuchsia-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f0abfc;
        /// }
        /// ```
        Fuchsia300 => "shadow-fuchsia-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e879f9;
        /// }
        /// ```
        Fuchsia400 => "shadow-fuchsia-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #d946ef;
        /// }
        /// ```
        Fuchsia500 => "shadow-fuchsia-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #c026d3;
        /// }
        /// ```
        Fuchsia600 => "shadow-fuchsia-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #a21caf;
        /// }
        /// ```
        Fuchsia700 => "shadow-fuchsia-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #86198f;
        /// }
        /// ```
        Fuchsia800 => "shadow-fuchsia-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #701a75;
        /// }
        /// ```
        Fuchsia900 => "shadow-fuchsia-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #4a044e;
        /// }
        /// ```
        Fuchsia950 => "shadow-fuchsia-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fdf2f8;
        /// }
        /// ```
        Pink50 => "shadow-pink-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fce7f3;
        /// }
        /// ```
        Pink100 => "shadow-pink-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fbcfe8;
        /// }
        /// ```
        Pink200 => "shadow-pink-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f9a8d4;
        /// }
        /// ```
        Pink300 => "shadow-pink-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f472b6;
        /// }
        /// ```
        Pink400 => "shadow-pink-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ec4899;
        /// }
        /// ```
        Pink500 => "shadow-pink-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #db2777;
        /// }
        /// ```
        Pink600 => "shadow-pink-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #be185d;
        /// }
        /// ```
        Pink700 => "shadow-pink-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #9d174d;
        /// }
        /// ```
        Pink800 => "shadow-pink-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #831843;
        /// }
        /// ```
        Pink900 => "shadow-pink-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #500724;
        /// }
        /// ```
        Pink950 => "shadow-pink-950",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fff1f2;
        /// }
        /// ```
        Rose50 => "shadow-rose-50",
        /// ```css
        /// {
        ///     --tw-shadow-color: #ffe4e6;
        /// }
        /// ```
        Rose100 => "shadow-rose-100",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fecdd3;
        /// }
        /// ```
        Rose200 => "shadow-rose-200",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fda4af;
        /// }
        /// ```
        Rose300 => "shadow-rose-300",
        /// ```css
        /// {
        ///     --tw-shadow-color: #fb7185;
        /// }
        /// ```
        Rose400 => "shadow-rose-400",
        /// ```css
        /// {
        ///     --tw-shadow-color: #f43f5e;
        /// }
        /// ```
        Rose500 => "shadow-rose-500",
        /// ```css
        /// {
        ///     --tw-shadow-color: #e11d48;
        /// }
        /// ```
        Rose600 => "shadow-rose-600",
        /// ```css
        /// {
        ///     --tw-shadow-color: #be123c;
        /// }
        /// ```
        Rose700 => "shadow-rose-700",
        /// ```css
        /// {
        ///     --tw-shadow-color: #9f1239;
        /// }
        /// ```
        Rose800 => "shadow-rose-800",
        /// ```css
        /// {
        ///     --tw-shadow-color: #881337;
        /// }
        /// ```
        Rose900 => "shadow-rose-900",
        /// ```css
        /// {
        ///     --tw-shadow-color: #4c0519;
        /// }
        /// ```
        Rose950 => "shadow-rose-950",
    }
    /// Utilities for controlling the opacity of an element.
    ///
    /// <https://tailwindcss.com/docs/opacity>
    Opacity {
        /// ```css
        /// {
        ///     opacity: 0;
        /// }
        /// ```
        _0 => "opacity-0",
        /// ```css
        /// {
        ///     opacity: 0.05;
        /// }
        /// ```
        _5 => "opacity-5",
        /// ```css
        /// {
        ///     opacity: 0.1;
        /// }
        /// ```
        _10 => "opacity-10",
        /// ```css
        /// {
        ///     opacity: 0.15;
        /// }
        /// ```
        _15 => "opacity-15",
        /// ```css
        /// {
        ///     opacity: 0.2;
        /// }
        /// ```
        _20 => "opacity-20",
        /// ```css
        /// {
        ///     opacity: 0.25;
        /// }
        /// ```
        _25 => "opacity-25",
        /// ```css
        /// {
        ///     opacity: 0.3;
        /// }
        /// ```
        _30 => "opacity-30",
        /// ```css
        /// {
        ///     opacity: 0.35;
        /// }
        /// ```
        _35 => "opacity-35",
        /// ```css
        /// {
        ///     opacity: 0.4;
        /// }
        /// ```
        _40 => "opacity-40",
        /// ```css
        /// {
        ///     opacity: 0.45;
        /// }
        /// ```
        _45 => "opacity-45",
        /// ```css
        /// {
        ///     opacity: 0.5;
        /// }
        /// ```
        _50 => "opacity-50",
        /// ```css
        /// {
        ///     opacity: 0.55;
        /// }
        /// ```
        _55 => "opacity-55",
        /// ```css
        /// {
        ///     opacity: 0.6;
        /// }
        /// ```
        _60 => "opacity-60",
        /// ```css
        /// {
        ///     opacity: 0.65;
        /// }
        /// ```
        _65 => "opacity-65",
        /// ```css
        /// {
        ///     opacity: 0.7;
        /// }
        /// ```
        _70 => "opacity-70",
        /// ```css
        /// {
        ///     opacity: 0.75;
        /// }
        /// ```
        _75 => "opacity-75",
        /// ```css
        /// {
        ///     opacity: 0.8;
        /// }
        /// ```
        _80 => "opacity-80",
        /// ```css
        /// {
        ///     opacity: 0.85;
        /// }
        /// ```
        _85 => "opacity-85",
        /// ```css
        /// {
        ///     opacity: 0.9;
        /// }
        /// ```
        _90 => "opacity-90",
        /// ```css
        /// {
        ///     opacity: 0.95;
        /// }
        /// ```
        _95 => "opacity-95",
        /// ```css
        /// {
        ///     opacity: 1;
        /// }
        /// ```
        _100 => "opacity-100",
    }
    /// Utilities for controlling how an element should blend with the background.
    ///
    /// <https://tailwindcss.com/docs/mix-blend-mode>
    MixBlendMode {
        /// ```css
        /// {
        ///     mix-blend-mode: normal;
        /// }
        /// ```
        Normal => "mix-blend-normal",
        /// ```css
        /// {
        ///     mix-blend-mode: multiply;
        /// }
        /// ```
        Multiply => "mix-blend-multiply",
        /// ```css
        /// {
        ///     mix-blend-mode: screen;
        /// }
        /// ```
        Screen => "mix-blend-screen",
        /// ```css
        /// {
        ///     mix-blend-mode: overlay;
        /// }
        /// ```
        Overlay => "mix-blend-overlay",
        /// ```css
        /// {
        ///     mix-blend-mode: darken;
        /// }
        /// ```
        Darken => "mix-blend-darken",
        /// ```css
        /// {
        ///     mix-blend-mode: lighten;
        /// }
        /// ```
        Lighten => "mix-blend-lighten",
        /// ```css
        /// {
        ///     mix-blend-mode: color-dodge;
        /// }
        /// ```
        ColorDodge => "mix-blend-color-dodge",
        /// ```css
        /// {
        ///     mix-blend-mode: color-burn;
        /// }
        /// ```
        ColorBurn => "mix-blend-color-burn",
        /// ```css
        /// {
        ///     mix-blend-mode: hard-light;
        /// }
        /// ```
        HardLight => "mix-blend-hard-light",
        /// ```css
        /// {
        ///     mix-blend-mode: soft-light;
        /// }
        /// ```
        SoftLight => "mix-blend-soft-light",
        /// ```css
        /// {
        ///     mix-blend-mode: difference;
        /// }
        /// ```
        Difference => "mix-blend-difference",
        /// ```css
        /// {
        ///     mix-blend-mode: exclusion;
        /// }
        /// ```
        Exclusion => "mix-blend-exclusion",
        /// ```css
        /// {
        ///     mix-blend-mode: hue;
        /// }
        /// ```
        Hue => "mix-blend-hue",
        /// ```css
        /// {
        ///     mix-blend-mode: saturation;
        /// }
        /// ```
        Saturation => "mix-blend-saturation",
        /// ```css
        /// {
        ///     mix-blend-mode: color;
        /// }
        /// ```
        Color => "mix-blend-color",
        /// ```css
        /// {
        ///     mix-blend-mode: luminosity;
        /// }
        /// ```
        Luminosity => "mix-blend-luminosity",
        /// ```css
        /// {
        ///     mix-blend-mode: plus-darker;
        /// }
        /// ```
        PlusDarker => "mix-blend-plus-darker",
        /// ```css
        /// {
        ///     mix-blend-mode: plus-lighter;
        /// }
        /// ```
        PlusLighter => "mix-blend-plus-lighter",
    }
    /// Utilities for controlling how an element's background image should blend with its background color.
    ///
    /// <https://tailwindcss.com/docs/background-blend-mode>
    BackgroundBlendMode {
        /// ```css
        /// {
        ///     background-blend-mode: normal;
        /// }
        /// ```
        Normal => "bg-blend-normal",
        /// ```css
        /// {
        ///     background-blend-mode: multiply;
        /// }
        /// ```
        Multiply => "bg-blend-multiply",
        /// ```css
        /// {
        ///     background-blend-mode: screen;
        /// }
        /// ```
        Screen => "bg-blend-screen",
        /// ```css
        /// {
        ///     background-blend-mode: overlay;
        /// }
        /// ```
        Overlay => "bg-blend-overlay",
        /// ```css
        /// {
        ///     background-blend-mode: darken;
        /// }
        /// ```
        Darken => "bg-blend-darken",
        /// ```css
        /// {
        ///     background-blend-mode: lighten;
        /// }
        /// ```
        Lighten => "bg-blend-lighten",
        /// ```css
        /// {
        ///     background-blend-mode: color-dodge;
        /// }
        /// ```
        ColorDodge => "bg-blend-color-dodge",
        /// ```css
        /// {
        ///     background-blend-mode: color-burn;
        /// }
        /// ```
        ColorBurn => "bg-blend-color-burn",
        /// ```css
        /// {
        ///     background-blend-mode: hard-light;
        /// }
        /// ```
        HardLight => "bg-blend-hard-light",
        /// ```css
        /// {
        ///     background-blend-mode: soft-light;
        /// }
        /// ```
        SoftLight => "bg-blend-soft-light",
        /// ```css
        /// {
        ///     background-blend-mode: difference;
        /// }
        /// ```
        Difference => "bg-blend-difference",
        /// ```css
        /// {
        ///     background-blend-mode: exclusion;
        /// }
        /// ```
        Exclusion => "bg-blend-exclusion",
        /// ```css
        /// {
        ///     background-blend-mode: hue;
        /// }
        /// ```
        Hue => "bg-blend-hue",
        /// ```css
        /// {
        ///     background-blend-mode: saturation;
        /// }
        /// ```
        Saturation => "bg-blend-saturation",
        /// ```css
        /// {
        ///     background-blend-mode: color;
        /// }
        /// ```
        Color => "bg-blend-color",
        /// ```css
        /// {
        ///     background-blend-mode: luminosity;
        /// }
        /// ```
        Luminosity => "bg-blend-luminosity",
    }
);
