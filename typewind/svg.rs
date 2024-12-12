def_types!(
    /// Utilities for styling the fill of SVG elements.
    ///
    /// <https://tailwindcss.com/docs/fill>
    Fill {
        /// ```css
        /// {
        ///     fill: none;
        /// }
        /// ```
        None => "fill-none",
        /// ```css
        /// {
        ///     fill: inherit;
        /// }
        /// ```
        Inherit => "fill-inherit",
        /// ```css
        /// {
        ///     fill: currentColor;
        /// }
        /// ```
        Current => "fill-current",
        /// ```css
        /// {
        ///     fill: transparent;
        /// }
        /// ```
        Transparent => "fill-transparent",
        /// ```css
        /// {
        ///     fill: #000;
        /// }
        /// ```
        Black => "fill-black",
        /// ```css
        /// {
        ///     fill: #fff;
        /// }
        /// ```
        White => "fill-white",
        /// ```css
        /// {
        ///     fill: #f8fafc;
        /// }
        /// ```
        Slate50 => "fill-slate-50",
        /// ```css
        /// {
        ///     fill: #f1f5f9;
        /// }
        /// ```
        Slate100 => "fill-slate-100",
        /// ```css
        /// {
        ///     fill: #e2e8f0;
        /// }
        /// ```
        Slate200 => "fill-slate-200",
        /// ```css
        /// {
        ///     fill: #cbd5e1;
        /// }
        /// ```
        Slate300 => "fill-slate-300",
        /// ```css
        /// {
        ///     fill: #94a3b8;
        /// }
        /// ```
        Slate400 => "fill-slate-400",
        /// ```css
        /// {
        ///     fill: #64748b;
        /// }
        /// ```
        Slate500 => "fill-slate-500",
        /// ```css
        /// {
        ///     fill: #475569;
        /// }
        /// ```
        Slate600 => "fill-slate-600",
        /// ```css
        /// {
        ///     fill: #334155;
        /// }
        /// ```
        Slate700 => "fill-slate-700",
        /// ```css
        /// {
        ///     fill: #1e293b;
        /// }
        /// ```
        Slate800 => "fill-slate-800",
        /// ```css
        /// {
        ///     fill: #0f172a;
        /// }
        /// ```
        Slate900 => "fill-slate-900",
        /// ```css
        /// {
        ///     fill: #020617;
        /// }
        /// ```
        Slate950 => "fill-slate-950",
        /// ```css
        /// {
        ///     fill: #f9fafb;
        /// }
        /// ```
        Gray50 => "fill-gray-50",
        /// ```css
        /// {
        ///     fill: #f3f4f6;
        /// }
        /// ```
        Gray100 => "fill-gray-100",
        /// ```css
        /// {
        ///     fill: #e5e7eb;
        /// }
        /// ```
        Gray200 => "fill-gray-200",
        /// ```css
        /// {
        ///     fill: #d1d5db;
        /// }
        /// ```
        Gray300 => "fill-gray-300",
        /// ```css
        /// {
        ///     fill: #9ca3af;
        /// }
        /// ```
        Gray400 => "fill-gray-400",
        /// ```css
        /// {
        ///     fill: #6b7280;
        /// }
        /// ```
        Gray500 => "fill-gray-500",
        /// ```css
        /// {
        ///     fill: #4b5563;
        /// }
        /// ```
        Gray600 => "fill-gray-600",
        /// ```css
        /// {
        ///     fill: #374151;
        /// }
        /// ```
        Gray700 => "fill-gray-700",
        /// ```css
        /// {
        ///     fill: #1f2937;
        /// }
        /// ```
        Gray800 => "fill-gray-800",
        /// ```css
        /// {
        ///     fill: #111827;
        /// }
        /// ```
        Gray900 => "fill-gray-900",
        /// ```css
        /// {
        ///     fill: #030712;
        /// }
        /// ```
        Gray950 => "fill-gray-950",
        /// ```css
        /// {
        ///     fill: #fafafa;
        /// }
        /// ```
        Zinc50 => "fill-zinc-50",
        /// ```css
        /// {
        ///     fill: #f4f4f5;
        /// }
        /// ```
        Zinc100 => "fill-zinc-100",
        /// ```css
        /// {
        ///     fill: #e4e4e7;
        /// }
        /// ```
        Zinc200 => "fill-zinc-200",
        /// ```css
        /// {
        ///     fill: #d4d4d8;
        /// }
        /// ```
        Zinc300 => "fill-zinc-300",
        /// ```css
        /// {
        ///     fill: #a1a1aa;
        /// }
        /// ```
        Zinc400 => "fill-zinc-400",
        /// ```css
        /// {
        ///     fill: #71717a;
        /// }
        /// ```
        Zinc500 => "fill-zinc-500",
        /// ```css
        /// {
        ///     fill: #52525b;
        /// }
        /// ```
        Zinc600 => "fill-zinc-600",
        /// ```css
        /// {
        ///     fill: #3f3f46;
        /// }
        /// ```
        Zinc700 => "fill-zinc-700",
        /// ```css
        /// {
        ///     fill: #27272a;
        /// }
        /// ```
        Zinc800 => "fill-zinc-800",
        /// ```css
        /// {
        ///     fill: #18181b;
        /// }
        /// ```
        Zinc900 => "fill-zinc-900",
        /// ```css
        /// {
        ///     fill: #09090b;
        /// }
        /// ```
        Zinc950 => "fill-zinc-950",
        /// ```css
        /// {
        ///     fill: #fafafa;
        /// }
        /// ```
        Neutral50 => "fill-neutral-50",
        /// ```css
        /// {
        ///     fill: #f5f5f5;
        /// }
        /// ```
        Neutral100 => "fill-neutral-100",
        /// ```css
        /// {
        ///     fill: #e5e5e5;
        /// }
        /// ```
        Neutral200 => "fill-neutral-200",
        /// ```css
        /// {
        ///     fill: #d4d4d4;
        /// }
        /// ```
        Neutral300 => "fill-neutral-300",
        /// ```css
        /// {
        ///     fill: #a3a3a3;
        /// }
        /// ```
        Neutral400 => "fill-neutral-400",
        /// ```css
        /// {
        ///     fill: #737373;
        /// }
        /// ```
        Neutral500 => "fill-neutral-500",
        /// ```css
        /// {
        ///     fill: #525252;
        /// }
        /// ```
        Neutral600 => "fill-neutral-600",
        /// ```css
        /// {
        ///     fill: #404040;
        /// }
        /// ```
        Neutral700 => "fill-neutral-700",
        /// ```css
        /// {
        ///     fill: #262626;
        /// }
        /// ```
        Neutral800 => "fill-neutral-800",
        /// ```css
        /// {
        ///     fill: #171717;
        /// }
        /// ```
        Neutral900 => "fill-neutral-900",
        /// ```css
        /// {
        ///     fill: #0a0a0a;
        /// }
        /// ```
        Neutral950 => "fill-neutral-950",
        /// ```css
        /// {
        ///     fill: #fafaf9;
        /// }
        /// ```
        Stone50 => "fill-stone-50",
        /// ```css
        /// {
        ///     fill: #f5f5f4;
        /// }
        /// ```
        Stone100 => "fill-stone-100",
        /// ```css
        /// {
        ///     fill: #e7e5e4;
        /// }
        /// ```
        Stone200 => "fill-stone-200",
        /// ```css
        /// {
        ///     fill: #d6d3d1;
        /// }
        /// ```
        Stone300 => "fill-stone-300",
        /// ```css
        /// {
        ///     fill: #a8a29e;
        /// }
        /// ```
        Stone400 => "fill-stone-400",
        /// ```css
        /// {
        ///     fill: #78716c;
        /// }
        /// ```
        Stone500 => "fill-stone-500",
        /// ```css
        /// {
        ///     fill: #57534e;
        /// }
        /// ```
        Stone600 => "fill-stone-600",
        /// ```css
        /// {
        ///     fill: #44403c;
        /// }
        /// ```
        Stone700 => "fill-stone-700",
        /// ```css
        /// {
        ///     fill: #292524;
        /// }
        /// ```
        Stone800 => "fill-stone-800",
        /// ```css
        /// {
        ///     fill: #1c1917;
        /// }
        /// ```
        Stone900 => "fill-stone-900",
        /// ```css
        /// {
        ///     fill: #0c0a09;
        /// }
        /// ```
        Stone950 => "fill-stone-950",
        /// ```css
        /// {
        ///     fill: #fef2f2;
        /// }
        /// ```
        Red50 => "fill-red-50",
        /// ```css
        /// {
        ///     fill: #fee2e2;
        /// }
        /// ```
        Red100 => "fill-red-100",
        /// ```css
        /// {
        ///     fill: #fecaca;
        /// }
        /// ```
        Red200 => "fill-red-200",
        /// ```css
        /// {
        ///     fill: #fca5a5;
        /// }
        /// ```
        Red300 => "fill-red-300",
        /// ```css
        /// {
        ///     fill: #f87171;
        /// }
        /// ```
        Red400 => "fill-red-400",
        /// ```css
        /// {
        ///     fill: #ef4444;
        /// }
        /// ```
        Red500 => "fill-red-500",
        /// ```css
        /// {
        ///     fill: #dc2626;
        /// }
        /// ```
        Red600 => "fill-red-600",
        /// ```css
        /// {
        ///     fill: #b91c1c;
        /// }
        /// ```
        Red700 => "fill-red-700",
        /// ```css
        /// {
        ///     fill: #991b1b;
        /// }
        /// ```
        Red800 => "fill-red-800",
        /// ```css
        /// {
        ///     fill: #7f1d1d;
        /// }
        /// ```
        Red900 => "fill-red-900",
        /// ```css
        /// {
        ///     fill: #450a0a;
        /// }
        /// ```
        Red950 => "fill-red-950",
        /// ```css
        /// {
        ///     fill: #fff7ed;
        /// }
        /// ```
        Orange50 => "fill-orange-50",
        /// ```css
        /// {
        ///     fill: #ffedd5;
        /// }
        /// ```
        Orange100 => "fill-orange-100",
        /// ```css
        /// {
        ///     fill: #fed7aa;
        /// }
        /// ```
        Orange200 => "fill-orange-200",
        /// ```css
        /// {
        ///     fill: #fdba74;
        /// }
        /// ```
        Orange300 => "fill-orange-300",
        /// ```css
        /// {
        ///     fill: #fb923c;
        /// }
        /// ```
        Orange400 => "fill-orange-400",
        /// ```css
        /// {
        ///     fill: #f97316;
        /// }
        /// ```
        Orange500 => "fill-orange-500",
        /// ```css
        /// {
        ///     fill: #ea580c;
        /// }
        /// ```
        Orange600 => "fill-orange-600",
        /// ```css
        /// {
        ///     fill: #c2410c;
        /// }
        /// ```
        Orange700 => "fill-orange-700",
        /// ```css
        /// {
        ///     fill: #9a3412;
        /// }
        /// ```
        Orange800 => "fill-orange-800",
        /// ```css
        /// {
        ///     fill: #7c2d12;
        /// }
        /// ```
        Orange900 => "fill-orange-900",
        /// ```css
        /// {
        ///     fill: #431407;
        /// }
        /// ```
        Orange950 => "fill-orange-950",
        /// ```css
        /// {
        ///     fill: #fffbeb;
        /// }
        /// ```
        Amber50 => "fill-amber-50",
        /// ```css
        /// {
        ///     fill: #fef3c7;
        /// }
        /// ```
        Amber100 => "fill-amber-100",
        /// ```css
        /// {
        ///     fill: #fde68a;
        /// }
        /// ```
        Amber200 => "fill-amber-200",
        /// ```css
        /// {
        ///     fill: #fcd34d;
        /// }
        /// ```
        Amber300 => "fill-amber-300",
        /// ```css
        /// {
        ///     fill: #fbbf24;
        /// }
        /// ```
        Amber400 => "fill-amber-400",
        /// ```css
        /// {
        ///     fill: #f59e0b;
        /// }
        /// ```
        Amber500 => "fill-amber-500",
        /// ```css
        /// {
        ///     fill: #d97706;
        /// }
        /// ```
        Amber600 => "fill-amber-600",
        /// ```css
        /// {
        ///     fill: #b45309;
        /// }
        /// ```
        Amber700 => "fill-amber-700",
        /// ```css
        /// {
        ///     fill: #92400e;
        /// }
        /// ```
        Amber800 => "fill-amber-800",
        /// ```css
        /// {
        ///     fill: #78350f;
        /// }
        /// ```
        Amber900 => "fill-amber-900",
        /// ```css
        /// {
        ///     fill: #451a03;
        /// }
        /// ```
        Amber950 => "fill-amber-950",
        /// ```css
        /// {
        ///     fill: #fefce8;
        /// }
        /// ```
        Yellow50 => "fill-yellow-50",
        /// ```css
        /// {
        ///     fill: #fef9c3;
        /// }
        /// ```
        Yellow100 => "fill-yellow-100",
        /// ```css
        /// {
        ///     fill: #fef08a;
        /// }
        /// ```
        Yellow200 => "fill-yellow-200",
        /// ```css
        /// {
        ///     fill: #fde047;
        /// }
        /// ```
        Yellow300 => "fill-yellow-300",
        /// ```css
        /// {
        ///     fill: #facc15;
        /// }
        /// ```
        Yellow400 => "fill-yellow-400",
        /// ```css
        /// {
        ///     fill: #eab308;
        /// }
        /// ```
        Yellow500 => "fill-yellow-500",
        /// ```css
        /// {
        ///     fill: #ca8a04;
        /// }
        /// ```
        Yellow600 => "fill-yellow-600",
        /// ```css
        /// {
        ///     fill: #a16207;
        /// }
        /// ```
        Yellow700 => "fill-yellow-700",
        /// ```css
        /// {
        ///     fill: #854d0e;
        /// }
        /// ```
        Yellow800 => "fill-yellow-800",
        /// ```css
        /// {
        ///     fill: #713f12;
        /// }
        /// ```
        Yellow900 => "fill-yellow-900",
        /// ```css
        /// {
        ///     fill: #422006;
        /// }
        /// ```
        Yellow950 => "fill-yellow-950",
        /// ```css
        /// {
        ///     fill: #f7fee7;
        /// }
        /// ```
        Lime50 => "fill-lime-50",
        /// ```css
        /// {
        ///     fill: #ecfccb;
        /// }
        /// ```
        Lime100 => "fill-lime-100",
        /// ```css
        /// {
        ///     fill: #d9f99d;
        /// }
        /// ```
        Lime200 => "fill-lime-200",
        /// ```css
        /// {
        ///     fill: #bef264;
        /// }
        /// ```
        Lime300 => "fill-lime-300",
        /// ```css
        /// {
        ///     fill: #a3e635;
        /// }
        /// ```
        Lime400 => "fill-lime-400",
        /// ```css
        /// {
        ///     fill: #84cc16;
        /// }
        /// ```
        Lime500 => "fill-lime-500",
        /// ```css
        /// {
        ///     fill: #65a30d;
        /// }
        /// ```
        Lime600 => "fill-lime-600",
        /// ```css
        /// {
        ///     fill: #4d7c0f;
        /// }
        /// ```
        Lime700 => "fill-lime-700",
        /// ```css
        /// {
        ///     fill: #3f6212;
        /// }
        /// ```
        Lime800 => "fill-lime-800",
        /// ```css
        /// {
        ///     fill: #365314;
        /// }
        /// ```
        Lime900 => "fill-lime-900",
        /// ```css
        /// {
        ///     fill: #1a2e05;
        /// }
        /// ```
        Lime950 => "fill-lime-950",
        /// ```css
        /// {
        ///     fill: #f0fdf4;
        /// }
        /// ```
        Green50 => "fill-green-50",
        /// ```css
        /// {
        ///     fill: #dcfce7;
        /// }
        /// ```
        Green100 => "fill-green-100",
        /// ```css
        /// {
        ///     fill: #bbf7d0;
        /// }
        /// ```
        Green200 => "fill-green-200",
        /// ```css
        /// {
        ///     fill: #86efac;
        /// }
        /// ```
        Green300 => "fill-green-300",
        /// ```css
        /// {
        ///     fill: #4ade80;
        /// }
        /// ```
        Green400 => "fill-green-400",
        /// ```css
        /// {
        ///     fill: #22c55e;
        /// }
        /// ```
        Green500 => "fill-green-500",
        /// ```css
        /// {
        ///     fill: #16a34a;
        /// }
        /// ```
        Green600 => "fill-green-600",
        /// ```css
        /// {
        ///     fill: #15803d;
        /// }
        /// ```
        Green700 => "fill-green-700",
        /// ```css
        /// {
        ///     fill: #166534;
        /// }
        /// ```
        Green800 => "fill-green-800",
        /// ```css
        /// {
        ///     fill: #14532d;
        /// }
        /// ```
        Green900 => "fill-green-900",
        /// ```css
        /// {
        ///     fill: #052e16;
        /// }
        /// ```
        Green950 => "fill-green-950",
        /// ```css
        /// {
        ///     fill: #ecfdf5;
        /// }
        /// ```
        Emerald50 => "fill-emerald-50",
        /// ```css
        /// {
        ///     fill: #d1fae5;
        /// }
        /// ```
        Emerald100 => "fill-emerald-100",
        /// ```css
        /// {
        ///     fill: #a7f3d0;
        /// }
        /// ```
        Emerald200 => "fill-emerald-200",
        /// ```css
        /// {
        ///     fill: #6ee7b7;
        /// }
        /// ```
        Emerald300 => "fill-emerald-300",
        /// ```css
        /// {
        ///     fill: #34d399;
        /// }
        /// ```
        Emerald400 => "fill-emerald-400",
        /// ```css
        /// {
        ///     fill: #10b981;
        /// }
        /// ```
        Emerald500 => "fill-emerald-500",
        /// ```css
        /// {
        ///     fill: #059669;
        /// }
        /// ```
        Emerald600 => "fill-emerald-600",
        /// ```css
        /// {
        ///     fill: #047857;
        /// }
        /// ```
        Emerald700 => "fill-emerald-700",
        /// ```css
        /// {
        ///     fill: #065f46;
        /// }
        /// ```
        Emerald800 => "fill-emerald-800",
        /// ```css
        /// {
        ///     fill: #064e3b;
        /// }
        /// ```
        Emerald900 => "fill-emerald-900",
        /// ```css
        /// {
        ///     fill: #022c22;
        /// }
        /// ```
        Emerald950 => "fill-emerald-950",
        /// ```css
        /// {
        ///     fill: #f0fdfa;
        /// }
        /// ```
        Teal50 => "fill-teal-50",
        /// ```css
        /// {
        ///     fill: #ccfbf1;
        /// }
        /// ```
        Teal100 => "fill-teal-100",
        /// ```css
        /// {
        ///     fill: #99f6e4;
        /// }
        /// ```
        Teal200 => "fill-teal-200",
        /// ```css
        /// {
        ///     fill: #5eead4;
        /// }
        /// ```
        Teal300 => "fill-teal-300",
        /// ```css
        /// {
        ///     fill: #2dd4bf;
        /// }
        /// ```
        Teal400 => "fill-teal-400",
        /// ```css
        /// {
        ///     fill: #14b8a6;
        /// }
        /// ```
        Teal500 => "fill-teal-500",
        /// ```css
        /// {
        ///     fill: #0d9488;
        /// }
        /// ```
        Teal600 => "fill-teal-600",
        /// ```css
        /// {
        ///     fill: #0f766e;
        /// }
        /// ```
        Teal700 => "fill-teal-700",
        /// ```css
        /// {
        ///     fill: #115e59;
        /// }
        /// ```
        Teal800 => "fill-teal-800",
        /// ```css
        /// {
        ///     fill: #134e4a;
        /// }
        /// ```
        Teal900 => "fill-teal-900",
        /// ```css
        /// {
        ///     fill: #042f2e;
        /// }
        /// ```
        Teal950 => "fill-teal-950",
        /// ```css
        /// {
        ///     fill: #ecfeff;
        /// }
        /// ```
        Cyan50 => "fill-cyan-50",
        /// ```css
        /// {
        ///     fill: #cffafe;
        /// }
        /// ```
        Cyan100 => "fill-cyan-100",
        /// ```css
        /// {
        ///     fill: #a5f3fc;
        /// }
        /// ```
        Cyan200 => "fill-cyan-200",
        /// ```css
        /// {
        ///     fill: #67e8f9;
        /// }
        /// ```
        Cyan300 => "fill-cyan-300",
        /// ```css
        /// {
        ///     fill: #22d3ee;
        /// }
        /// ```
        Cyan400 => "fill-cyan-400",
        /// ```css
        /// {
        ///     fill: #06b6d4;
        /// }
        /// ```
        Cyan500 => "fill-cyan-500",
        /// ```css
        /// {
        ///     fill: #0891b2;
        /// }
        /// ```
        Cyan600 => "fill-cyan-600",
        /// ```css
        /// {
        ///     fill: #0e7490;
        /// }
        /// ```
        Cyan700 => "fill-cyan-700",
        /// ```css
        /// {
        ///     fill: #155e75;
        /// }
        /// ```
        Cyan800 => "fill-cyan-800",
        /// ```css
        /// {
        ///     fill: #164e63;
        /// }
        /// ```
        Cyan900 => "fill-cyan-900",
        /// ```css
        /// {
        ///     fill: #083344;
        /// }
        /// ```
        Cyan950 => "fill-cyan-950",
        /// ```css
        /// {
        ///     fill: #f0f9ff;
        /// }
        /// ```
        Sky50 => "fill-sky-50",
        /// ```css
        /// {
        ///     fill: #e0f2fe;
        /// }
        /// ```
        Sky100 => "fill-sky-100",
        /// ```css
        /// {
        ///     fill: #bae6fd;
        /// }
        /// ```
        Sky200 => "fill-sky-200",
        /// ```css
        /// {
        ///     fill: #7dd3fc;
        /// }
        /// ```
        Sky300 => "fill-sky-300",
        /// ```css
        /// {
        ///     fill: #38bdf8;
        /// }
        /// ```
        Sky400 => "fill-sky-400",
        /// ```css
        /// {
        ///     fill: #0ea5e9;
        /// }
        /// ```
        Sky500 => "fill-sky-500",
        /// ```css
        /// {
        ///     fill: #0284c7;
        /// }
        /// ```
        Sky600 => "fill-sky-600",
        /// ```css
        /// {
        ///     fill: #0369a1;
        /// }
        /// ```
        Sky700 => "fill-sky-700",
        /// ```css
        /// {
        ///     fill: #075985;
        /// }
        /// ```
        Sky800 => "fill-sky-800",
        /// ```css
        /// {
        ///     fill: #0c4a6e;
        /// }
        /// ```
        Sky900 => "fill-sky-900",
        /// ```css
        /// {
        ///     fill: #082f49;
        /// }
        /// ```
        Sky950 => "fill-sky-950",
        /// ```css
        /// {
        ///     fill: #eff6ff;
        /// }
        /// ```
        Blue50 => "fill-blue-50",
        /// ```css
        /// {
        ///     fill: #dbeafe;
        /// }
        /// ```
        Blue100 => "fill-blue-100",
        /// ```css
        /// {
        ///     fill: #bfdbfe;
        /// }
        /// ```
        Blue200 => "fill-blue-200",
        /// ```css
        /// {
        ///     fill: #93c5fd;
        /// }
        /// ```
        Blue300 => "fill-blue-300",
        /// ```css
        /// {
        ///     fill: #60a5fa;
        /// }
        /// ```
        Blue400 => "fill-blue-400",
        /// ```css
        /// {
        ///     fill: #3b82f6;
        /// }
        /// ```
        Blue500 => "fill-blue-500",
        /// ```css
        /// {
        ///     fill: #2563eb;
        /// }
        /// ```
        Blue600 => "fill-blue-600",
        /// ```css
        /// {
        ///     fill: #1d4ed8;
        /// }
        /// ```
        Blue700 => "fill-blue-700",
        /// ```css
        /// {
        ///     fill: #1e40af;
        /// }
        /// ```
        Blue800 => "fill-blue-800",
        /// ```css
        /// {
        ///     fill: #1e3a8a;
        /// }
        /// ```
        Blue900 => "fill-blue-900",
        /// ```css
        /// {
        ///     fill: #172554;
        /// }
        /// ```
        Blue950 => "fill-blue-950",
        /// ```css
        /// {
        ///     fill: #eef2ff;
        /// }
        /// ```
        Indigo50 => "fill-indigo-50",
        /// ```css
        /// {
        ///     fill: #e0e7ff;
        /// }
        /// ```
        Indigo100 => "fill-indigo-100",
        /// ```css
        /// {
        ///     fill: #c7d2fe;
        /// }
        /// ```
        Indigo200 => "fill-indigo-200",
        /// ```css
        /// {
        ///     fill: #a5b4fc;
        /// }
        /// ```
        Indigo300 => "fill-indigo-300",
        /// ```css
        /// {
        ///     fill: #818cf8;
        /// }
        /// ```
        Indigo400 => "fill-indigo-400",
        /// ```css
        /// {
        ///     fill: #6366f1;
        /// }
        /// ```
        Indigo500 => "fill-indigo-500",
        /// ```css
        /// {
        ///     fill: #4f46e5;
        /// }
        /// ```
        Indigo600 => "fill-indigo-600",
        /// ```css
        /// {
        ///     fill: #4338ca;
        /// }
        /// ```
        Indigo700 => "fill-indigo-700",
        /// ```css
        /// {
        ///     fill: #3730a3;
        /// }
        /// ```
        Indigo800 => "fill-indigo-800",
        /// ```css
        /// {
        ///     fill: #312e81;
        /// }
        /// ```
        Indigo900 => "fill-indigo-900",
        /// ```css
        /// {
        ///     fill: #1e1b4b;
        /// }
        /// ```
        Indigo950 => "fill-indigo-950",
        /// ```css
        /// {
        ///     fill: #f5f3ff;
        /// }
        /// ```
        Violet50 => "fill-violet-50",
        /// ```css
        /// {
        ///     fill: #ede9fe;
        /// }
        /// ```
        Violet100 => "fill-violet-100",
        /// ```css
        /// {
        ///     fill: #ddd6fe;
        /// }
        /// ```
        Violet200 => "fill-violet-200",
        /// ```css
        /// {
        ///     fill: #c4b5fd;
        /// }
        /// ```
        Violet300 => "fill-violet-300",
        /// ```css
        /// {
        ///     fill: #a78bfa;
        /// }
        /// ```
        Violet400 => "fill-violet-400",
        /// ```css
        /// {
        ///     fill: #8b5cf6;
        /// }
        /// ```
        Violet500 => "fill-violet-500",
        /// ```css
        /// {
        ///     fill: #7c3aed;
        /// }
        /// ```
        Violet600 => "fill-violet-600",
        /// ```css
        /// {
        ///     fill: #6d28d9;
        /// }
        /// ```
        Violet700 => "fill-violet-700",
        /// ```css
        /// {
        ///     fill: #5b21b6;
        /// }
        /// ```
        Violet800 => "fill-violet-800",
        /// ```css
        /// {
        ///     fill: #4c1d95;
        /// }
        /// ```
        Violet900 => "fill-violet-900",
        /// ```css
        /// {
        ///     fill: #2e1065;
        /// }
        /// ```
        Violet950 => "fill-violet-950",
        /// ```css
        /// {
        ///     fill: #faf5ff;
        /// }
        /// ```
        Purple50 => "fill-purple-50",
        /// ```css
        /// {
        ///     fill: #f3e8ff;
        /// }
        /// ```
        Purple100 => "fill-purple-100",
        /// ```css
        /// {
        ///     fill: #e9d5ff;
        /// }
        /// ```
        Purple200 => "fill-purple-200",
        /// ```css
        /// {
        ///     fill: #d8b4fe;
        /// }
        /// ```
        Purple300 => "fill-purple-300",
        /// ```css
        /// {
        ///     fill: #c084fc;
        /// }
        /// ```
        Purple400 => "fill-purple-400",
        /// ```css
        /// {
        ///     fill: #a855f7;
        /// }
        /// ```
        Purple500 => "fill-purple-500",
        /// ```css
        /// {
        ///     fill: #9333ea;
        /// }
        /// ```
        Purple600 => "fill-purple-600",
        /// ```css
        /// {
        ///     fill: #7e22ce;
        /// }
        /// ```
        Purple700 => "fill-purple-700",
        /// ```css
        /// {
        ///     fill: #6b21a8;
        /// }
        /// ```
        Purple800 => "fill-purple-800",
        /// ```css
        /// {
        ///     fill: #581c87;
        /// }
        /// ```
        Purple900 => "fill-purple-900",
        /// ```css
        /// {
        ///     fill: #3b0764;
        /// }
        /// ```
        Purple950 => "fill-purple-950",
        /// ```css
        /// {
        ///     fill: #fdf4ff;
        /// }
        /// ```
        Fuchsia50 => "fill-fuchsia-50",
        /// ```css
        /// {
        ///     fill: #fae8ff;
        /// }
        /// ```
        Fuchsia100 => "fill-fuchsia-100",
        /// ```css
        /// {
        ///     fill: #f5d0fe;
        /// }
        /// ```
        Fuchsia200 => "fill-fuchsia-200",
        /// ```css
        /// {
        ///     fill: #f0abfc;
        /// }
        /// ```
        Fuchsia300 => "fill-fuchsia-300",
        /// ```css
        /// {
        ///     fill: #e879f9;
        /// }
        /// ```
        Fuchsia400 => "fill-fuchsia-400",
        /// ```css
        /// {
        ///     fill: #d946ef;
        /// }
        /// ```
        Fuchsia500 => "fill-fuchsia-500",
        /// ```css
        /// {
        ///     fill: #c026d3;
        /// }
        /// ```
        Fuchsia600 => "fill-fuchsia-600",
        /// ```css
        /// {
        ///     fill: #a21caf;
        /// }
        /// ```
        Fuchsia700 => "fill-fuchsia-700",
        /// ```css
        /// {
        ///     fill: #86198f;
        /// }
        /// ```
        Fuchsia800 => "fill-fuchsia-800",
        /// ```css
        /// {
        ///     fill: #701a75;
        /// }
        /// ```
        Fuchsia900 => "fill-fuchsia-900",
        /// ```css
        /// {
        ///     fill: #4a044e;
        /// }
        /// ```
        Fuchsia950 => "fill-fuchsia-950",
        /// ```css
        /// {
        ///     fill: #fdf2f8;
        /// }
        /// ```
        Pink50 => "fill-pink-50",
        /// ```css
        /// {
        ///     fill: #fce7f3;
        /// }
        /// ```
        Pink100 => "fill-pink-100",
        /// ```css
        /// {
        ///     fill: #fbcfe8;
        /// }
        /// ```
        Pink200 => "fill-pink-200",
        /// ```css
        /// {
        ///     fill: #f9a8d4;
        /// }
        /// ```
        Pink300 => "fill-pink-300",
        /// ```css
        /// {
        ///     fill: #f472b6;
        /// }
        /// ```
        Pink400 => "fill-pink-400",
        /// ```css
        /// {
        ///     fill: #ec4899;
        /// }
        /// ```
        Pink500 => "fill-pink-500",
        /// ```css
        /// {
        ///     fill: #db2777;
        /// }
        /// ```
        Pink600 => "fill-pink-600",
        /// ```css
        /// {
        ///     fill: #be185d;
        /// }
        /// ```
        Pink700 => "fill-pink-700",
        /// ```css
        /// {
        ///     fill: #9d174d;
        /// }
        /// ```
        Pink800 => "fill-pink-800",
        /// ```css
        /// {
        ///     fill: #831843;
        /// }
        /// ```
        Pink900 => "fill-pink-900",
        /// ```css
        /// {
        ///     fill: #500724;
        /// }
        /// ```
        Pink950 => "fill-pink-950",
        /// ```css
        /// {
        ///     fill: #fff1f2;
        /// }
        /// ```
        Rose50 => "fill-rose-50",
        /// ```css
        /// {
        ///     fill: #ffe4e6;
        /// }
        /// ```
        Rose100 => "fill-rose-100",
        /// ```css
        /// {
        ///     fill: #fecdd3;
        /// }
        /// ```
        Rose200 => "fill-rose-200",
        /// ```css
        /// {
        ///     fill: #fda4af;
        /// }
        /// ```
        Rose300 => "fill-rose-300",
        /// ```css
        /// {
        ///     fill: #fb7185;
        /// }
        /// ```
        Rose400 => "fill-rose-400",
        /// ```css
        /// {
        ///     fill: #f43f5e;
        /// }
        /// ```
        Rose500 => "fill-rose-500",
        /// ```css
        /// {
        ///     fill: #e11d48;
        /// }
        /// ```
        Rose600 => "fill-rose-600",
        /// ```css
        /// {
        ///     fill: #be123c;
        /// }
        /// ```
        Rose700 => "fill-rose-700",
        /// ```css
        /// {
        ///     fill: #9f1239;
        /// }
        /// ```
        Rose800 => "fill-rose-800",
        /// ```css
        /// {
        ///     fill: #881337;
        /// }
        /// ```
        Rose900 => "fill-rose-900",
        /// ```css
        /// {
        ///     fill: #4c0519;
        /// }
        /// ```
        Rose950 => "fill-rose-950",
    }
    /// Utilities for styling the stroke of SVG elements.
    ///
    /// <https://tailwindcss.com/docs/stroke>
    Stroke {
        /// ```css
        /// {
        ///     stroke: none;
        /// }
        /// ```
        None => "stroke-none",
        /// ```css
        /// {
        ///     stroke: inherit;
        /// }
        /// ```
        Inherit => "stroke-inherit",
        /// ```css
        /// {
        ///     stroke: currentColor;
        /// }
        /// ```
        Current => "stroke-current",
        /// ```css
        /// {
        ///     stroke: transparent;
        /// }
        /// ```
        Transparent => "stroke-transparent",
        /// ```css
        /// {
        ///     stroke: #000;
        /// }
        /// ```
        Black => "stroke-black",
        /// ```css
        /// {
        ///     stroke: #fff;
        /// }
        /// ```
        White => "stroke-white",
        /// ```css
        /// {
        ///     stroke: #f8fafc;
        /// }
        /// ```
        Slate50 => "stroke-slate-50",
        /// ```css
        /// {
        ///     stroke: #f1f5f9;
        /// }
        /// ```
        Slate100 => "stroke-slate-100",
        /// ```css
        /// {
        ///     stroke: #e2e8f0;
        /// }
        /// ```
        Slate200 => "stroke-slate-200",
        /// ```css
        /// {
        ///     stroke: #cbd5e1;
        /// }
        /// ```
        Slate300 => "stroke-slate-300",
        /// ```css
        /// {
        ///     stroke: #94a3b8;
        /// }
        /// ```
        Slate400 => "stroke-slate-400",
        /// ```css
        /// {
        ///     stroke: #64748b;
        /// }
        /// ```
        Slate500 => "stroke-slate-500",
        /// ```css
        /// {
        ///     stroke: #475569;
        /// }
        /// ```
        Slate600 => "stroke-slate-600",
        /// ```css
        /// {
        ///     stroke: #334155;
        /// }
        /// ```
        Slate700 => "stroke-slate-700",
        /// ```css
        /// {
        ///     stroke: #1e293b;
        /// }
        /// ```
        Slate800 => "stroke-slate-800",
        /// ```css
        /// {
        ///     stroke: #0f172a;
        /// }
        /// ```
        Slate900 => "stroke-slate-900",
        /// ```css
        /// {
        ///     stroke: #020617;
        /// }
        /// ```
        Slate950 => "stroke-slate-950",
        /// ```css
        /// {
        ///     stroke: #f9fafb;
        /// }
        /// ```
        Gray50 => "stroke-gray-50",
        /// ```css
        /// {
        ///     stroke: #f3f4f6;
        /// }
        /// ```
        Gray100 => "stroke-gray-100",
        /// ```css
        /// {
        ///     stroke: #e5e7eb;
        /// }
        /// ```
        Gray200 => "stroke-gray-200",
        /// ```css
        /// {
        ///     stroke: #d1d5db;
        /// }
        /// ```
        Gray300 => "stroke-gray-300",
        /// ```css
        /// {
        ///     stroke: #9ca3af;
        /// }
        /// ```
        Gray400 => "stroke-gray-400",
        /// ```css
        /// {
        ///     stroke: #6b7280;
        /// }
        /// ```
        Gray500 => "stroke-gray-500",
        /// ```css
        /// {
        ///     stroke: #4b5563;
        /// }
        /// ```
        Gray600 => "stroke-gray-600",
        /// ```css
        /// {
        ///     stroke: #374151;
        /// }
        /// ```
        Gray700 => "stroke-gray-700",
        /// ```css
        /// {
        ///     stroke: #1f2937;
        /// }
        /// ```
        Gray800 => "stroke-gray-800",
        /// ```css
        /// {
        ///     stroke: #111827;
        /// }
        /// ```
        Gray900 => "stroke-gray-900",
        /// ```css
        /// {
        ///     stroke: #030712;
        /// }
        /// ```
        Gray950 => "stroke-gray-950",
        /// ```css
        /// {
        ///     stroke: #fafafa;
        /// }
        /// ```
        Zinc50 => "stroke-zinc-50",
        /// ```css
        /// {
        ///     stroke: #f4f4f5;
        /// }
        /// ```
        Zinc100 => "stroke-zinc-100",
        /// ```css
        /// {
        ///     stroke: #e4e4e7;
        /// }
        /// ```
        Zinc200 => "stroke-zinc-200",
        /// ```css
        /// {
        ///     stroke: #d4d4d8;
        /// }
        /// ```
        Zinc300 => "stroke-zinc-300",
        /// ```css
        /// {
        ///     stroke: #a1a1aa;
        /// }
        /// ```
        Zinc400 => "stroke-zinc-400",
        /// ```css
        /// {
        ///     stroke: #71717a;
        /// }
        /// ```
        Zinc500 => "stroke-zinc-500",
        /// ```css
        /// {
        ///     stroke: #52525b;
        /// }
        /// ```
        Zinc600 => "stroke-zinc-600",
        /// ```css
        /// {
        ///     stroke: #3f3f46;
        /// }
        /// ```
        Zinc700 => "stroke-zinc-700",
        /// ```css
        /// {
        ///     stroke: #27272a;
        /// }
        /// ```
        Zinc800 => "stroke-zinc-800",
        /// ```css
        /// {
        ///     stroke: #18181b;
        /// }
        /// ```
        Zinc900 => "stroke-zinc-900",
        /// ```css
        /// {
        ///     stroke: #09090b;
        /// }
        /// ```
        Zinc950 => "stroke-zinc-950",
        /// ```css
        /// {
        ///     stroke: #fafafa;
        /// }
        /// ```
        Neutral50 => "stroke-neutral-50",
        /// ```css
        /// {
        ///     stroke: #f5f5f5;
        /// }
        /// ```
        Neutral100 => "stroke-neutral-100",
        /// ```css
        /// {
        ///     stroke: #e5e5e5;
        /// }
        /// ```
        Neutral200 => "stroke-neutral-200",
        /// ```css
        /// {
        ///     stroke: #d4d4d4;
        /// }
        /// ```
        Neutral300 => "stroke-neutral-300",
        /// ```css
        /// {
        ///     stroke: #a3a3a3;
        /// }
        /// ```
        Neutral400 => "stroke-neutral-400",
        /// ```css
        /// {
        ///     stroke: #737373;
        /// }
        /// ```
        Neutral500 => "stroke-neutral-500",
        /// ```css
        /// {
        ///     stroke: #525252;
        /// }
        /// ```
        Neutral600 => "stroke-neutral-600",
        /// ```css
        /// {
        ///     stroke: #404040;
        /// }
        /// ```
        Neutral700 => "stroke-neutral-700",
        /// ```css
        /// {
        ///     stroke: #262626;
        /// }
        /// ```
        Neutral800 => "stroke-neutral-800",
        /// ```css
        /// {
        ///     stroke: #171717;
        /// }
        /// ```
        Neutral900 => "stroke-neutral-900",
        /// ```css
        /// {
        ///     stroke: #0a0a0a;
        /// }
        /// ```
        Neutral950 => "stroke-neutral-950",
        /// ```css
        /// {
        ///     stroke: #fafaf9;
        /// }
        /// ```
        Stone50 => "stroke-stone-50",
        /// ```css
        /// {
        ///     stroke: #f5f5f4;
        /// }
        /// ```
        Stone100 => "stroke-stone-100",
        /// ```css
        /// {
        ///     stroke: #e7e5e4;
        /// }
        /// ```
        Stone200 => "stroke-stone-200",
        /// ```css
        /// {
        ///     stroke: #d6d3d1;
        /// }
        /// ```
        Stone300 => "stroke-stone-300",
        /// ```css
        /// {
        ///     stroke: #a8a29e;
        /// }
        /// ```
        Stone400 => "stroke-stone-400",
        /// ```css
        /// {
        ///     stroke: #78716c;
        /// }
        /// ```
        Stone500 => "stroke-stone-500",
        /// ```css
        /// {
        ///     stroke: #57534e;
        /// }
        /// ```
        Stone600 => "stroke-stone-600",
        /// ```css
        /// {
        ///     stroke: #44403c;
        /// }
        /// ```
        Stone700 => "stroke-stone-700",
        /// ```css
        /// {
        ///     stroke: #292524;
        /// }
        /// ```
        Stone800 => "stroke-stone-800",
        /// ```css
        /// {
        ///     stroke: #1c1917;
        /// }
        /// ```
        Stone900 => "stroke-stone-900",
        /// ```css
        /// {
        ///     stroke: #0c0a09;
        /// }
        /// ```
        Stone950 => "stroke-stone-950",
        /// ```css
        /// {
        ///     stroke: #fef2f2;
        /// }
        /// ```
        Red50 => "stroke-red-50",
        /// ```css
        /// {
        ///     stroke: #fee2e2;
        /// }
        /// ```
        Red100 => "stroke-red-100",
        /// ```css
        /// {
        ///     stroke: #fecaca;
        /// }
        /// ```
        Red200 => "stroke-red-200",
        /// ```css
        /// {
        ///     stroke: #fca5a5;
        /// }
        /// ```
        Red300 => "stroke-red-300",
        /// ```css
        /// {
        ///     stroke: #f87171;
        /// }
        /// ```
        Red400 => "stroke-red-400",
        /// ```css
        /// {
        ///     stroke: #ef4444;
        /// }
        /// ```
        Red500 => "stroke-red-500",
        /// ```css
        /// {
        ///     stroke: #dc2626;
        /// }
        /// ```
        Red600 => "stroke-red-600",
        /// ```css
        /// {
        ///     stroke: #b91c1c;
        /// }
        /// ```
        Red700 => "stroke-red-700",
        /// ```css
        /// {
        ///     stroke: #991b1b;
        /// }
        /// ```
        Red800 => "stroke-red-800",
        /// ```css
        /// {
        ///     stroke: #7f1d1d;
        /// }
        /// ```
        Red900 => "stroke-red-900",
        /// ```css
        /// {
        ///     stroke: #450a0a;
        /// }
        /// ```
        Red950 => "stroke-red-950",
        /// ```css
        /// {
        ///     stroke: #fff7ed;
        /// }
        /// ```
        Orange50 => "stroke-orange-50",
        /// ```css
        /// {
        ///     stroke: #ffedd5;
        /// }
        /// ```
        Orange100 => "stroke-orange-100",
        /// ```css
        /// {
        ///     stroke: #fed7aa;
        /// }
        /// ```
        Orange200 => "stroke-orange-200",
        /// ```css
        /// {
        ///     stroke: #fdba74;
        /// }
        /// ```
        Orange300 => "stroke-orange-300",
        /// ```css
        /// {
        ///     stroke: #fb923c;
        /// }
        /// ```
        Orange400 => "stroke-orange-400",
        /// ```css
        /// {
        ///     stroke: #f97316;
        /// }
        /// ```
        Orange500 => "stroke-orange-500",
        /// ```css
        /// {
        ///     stroke: #ea580c;
        /// }
        /// ```
        Orange600 => "stroke-orange-600",
        /// ```css
        /// {
        ///     stroke: #c2410c;
        /// }
        /// ```
        Orange700 => "stroke-orange-700",
        /// ```css
        /// {
        ///     stroke: #9a3412;
        /// }
        /// ```
        Orange800 => "stroke-orange-800",
        /// ```css
        /// {
        ///     stroke: #7c2d12;
        /// }
        /// ```
        Orange900 => "stroke-orange-900",
        /// ```css
        /// {
        ///     stroke: #431407;
        /// }
        /// ```
        Orange950 => "stroke-orange-950",
        /// ```css
        /// {
        ///     stroke: #fffbeb;
        /// }
        /// ```
        Amber50 => "stroke-amber-50",
        /// ```css
        /// {
        ///     stroke: #fef3c7;
        /// }
        /// ```
        Amber100 => "stroke-amber-100",
        /// ```css
        /// {
        ///     stroke: #fde68a;
        /// }
        /// ```
        Amber200 => "stroke-amber-200",
        /// ```css
        /// {
        ///     stroke: #fcd34d;
        /// }
        /// ```
        Amber300 => "stroke-amber-300",
        /// ```css
        /// {
        ///     stroke: #fbbf24;
        /// }
        /// ```
        Amber400 => "stroke-amber-400",
        /// ```css
        /// {
        ///     stroke: #f59e0b;
        /// }
        /// ```
        Amber500 => "stroke-amber-500",
        /// ```css
        /// {
        ///     stroke: #d97706;
        /// }
        /// ```
        Amber600 => "stroke-amber-600",
        /// ```css
        /// {
        ///     stroke: #b45309;
        /// }
        /// ```
        Amber700 => "stroke-amber-700",
        /// ```css
        /// {
        ///     stroke: #92400e;
        /// }
        /// ```
        Amber800 => "stroke-amber-800",
        /// ```css
        /// {
        ///     stroke: #78350f;
        /// }
        /// ```
        Amber900 => "stroke-amber-900",
        /// ```css
        /// {
        ///     stroke: #451a03;
        /// }
        /// ```
        Amber950 => "stroke-amber-950",
        /// ```css
        /// {
        ///     stroke: #fefce8;
        /// }
        /// ```
        Yellow50 => "stroke-yellow-50",
        /// ```css
        /// {
        ///     stroke: #fef9c3;
        /// }
        /// ```
        Yellow100 => "stroke-yellow-100",
        /// ```css
        /// {
        ///     stroke: #fef08a;
        /// }
        /// ```
        Yellow200 => "stroke-yellow-200",
        /// ```css
        /// {
        ///     stroke: #fde047;
        /// }
        /// ```
        Yellow300 => "stroke-yellow-300",
        /// ```css
        /// {
        ///     stroke: #facc15;
        /// }
        /// ```
        Yellow400 => "stroke-yellow-400",
        /// ```css
        /// {
        ///     stroke: #eab308;
        /// }
        /// ```
        Yellow500 => "stroke-yellow-500",
        /// ```css
        /// {
        ///     stroke: #ca8a04;
        /// }
        /// ```
        Yellow600 => "stroke-yellow-600",
        /// ```css
        /// {
        ///     stroke: #a16207;
        /// }
        /// ```
        Yellow700 => "stroke-yellow-700",
        /// ```css
        /// {
        ///     stroke: #854d0e;
        /// }
        /// ```
        Yellow800 => "stroke-yellow-800",
        /// ```css
        /// {
        ///     stroke: #713f12;
        /// }
        /// ```
        Yellow900 => "stroke-yellow-900",
        /// ```css
        /// {
        ///     stroke: #422006;
        /// }
        /// ```
        Yellow950 => "stroke-yellow-950",
        /// ```css
        /// {
        ///     stroke: #f7fee7;
        /// }
        /// ```
        Lime50 => "stroke-lime-50",
        /// ```css
        /// {
        ///     stroke: #ecfccb;
        /// }
        /// ```
        Lime100 => "stroke-lime-100",
        /// ```css
        /// {
        ///     stroke: #d9f99d;
        /// }
        /// ```
        Lime200 => "stroke-lime-200",
        /// ```css
        /// {
        ///     stroke: #bef264;
        /// }
        /// ```
        Lime300 => "stroke-lime-300",
        /// ```css
        /// {
        ///     stroke: #a3e635;
        /// }
        /// ```
        Lime400 => "stroke-lime-400",
        /// ```css
        /// {
        ///     stroke: #84cc16;
        /// }
        /// ```
        Lime500 => "stroke-lime-500",
        /// ```css
        /// {
        ///     stroke: #65a30d;
        /// }
        /// ```
        Lime600 => "stroke-lime-600",
        /// ```css
        /// {
        ///     stroke: #4d7c0f;
        /// }
        /// ```
        Lime700 => "stroke-lime-700",
        /// ```css
        /// {
        ///     stroke: #3f6212;
        /// }
        /// ```
        Lime800 => "stroke-lime-800",
        /// ```css
        /// {
        ///     stroke: #365314;
        /// }
        /// ```
        Lime900 => "stroke-lime-900",
        /// ```css
        /// {
        ///     stroke: #1a2e05;
        /// }
        /// ```
        Lime950 => "stroke-lime-950",
        /// ```css
        /// {
        ///     stroke: #f0fdf4;
        /// }
        /// ```
        Green50 => "stroke-green-50",
        /// ```css
        /// {
        ///     stroke: #dcfce7;
        /// }
        /// ```
        Green100 => "stroke-green-100",
        /// ```css
        /// {
        ///     stroke: #bbf7d0;
        /// }
        /// ```
        Green200 => "stroke-green-200",
        /// ```css
        /// {
        ///     stroke: #86efac;
        /// }
        /// ```
        Green300 => "stroke-green-300",
        /// ```css
        /// {
        ///     stroke: #4ade80;
        /// }
        /// ```
        Green400 => "stroke-green-400",
        /// ```css
        /// {
        ///     stroke: #22c55e;
        /// }
        /// ```
        Green500 => "stroke-green-500",
        /// ```css
        /// {
        ///     stroke: #16a34a;
        /// }
        /// ```
        Green600 => "stroke-green-600",
        /// ```css
        /// {
        ///     stroke: #15803d;
        /// }
        /// ```
        Green700 => "stroke-green-700",
        /// ```css
        /// {
        ///     stroke: #166534;
        /// }
        /// ```
        Green800 => "stroke-green-800",
        /// ```css
        /// {
        ///     stroke: #14532d;
        /// }
        /// ```
        Green900 => "stroke-green-900",
        /// ```css
        /// {
        ///     stroke: #052e16;
        /// }
        /// ```
        Green950 => "stroke-green-950",
        /// ```css
        /// {
        ///     stroke: #ecfdf5;
        /// }
        /// ```
        Emerald50 => "stroke-emerald-50",
        /// ```css
        /// {
        ///     stroke: #d1fae5;
        /// }
        /// ```
        Emerald100 => "stroke-emerald-100",
        /// ```css
        /// {
        ///     stroke: #a7f3d0;
        /// }
        /// ```
        Emerald200 => "stroke-emerald-200",
        /// ```css
        /// {
        ///     stroke: #6ee7b7;
        /// }
        /// ```
        Emerald300 => "stroke-emerald-300",
        /// ```css
        /// {
        ///     stroke: #34d399;
        /// }
        /// ```
        Emerald400 => "stroke-emerald-400",
        /// ```css
        /// {
        ///     stroke: #10b981;
        /// }
        /// ```
        Emerald500 => "stroke-emerald-500",
        /// ```css
        /// {
        ///     stroke: #059669;
        /// }
        /// ```
        Emerald600 => "stroke-emerald-600",
        /// ```css
        /// {
        ///     stroke: #047857;
        /// }
        /// ```
        Emerald700 => "stroke-emerald-700",
        /// ```css
        /// {
        ///     stroke: #065f46;
        /// }
        /// ```
        Emerald800 => "stroke-emerald-800",
        /// ```css
        /// {
        ///     stroke: #064e3b;
        /// }
        /// ```
        Emerald900 => "stroke-emerald-900",
        /// ```css
        /// {
        ///     stroke: #022c22;
        /// }
        /// ```
        Emerald950 => "stroke-emerald-950",
        /// ```css
        /// {
        ///     stroke: #f0fdfa;
        /// }
        /// ```
        Teal50 => "stroke-teal-50",
        /// ```css
        /// {
        ///     stroke: #ccfbf1;
        /// }
        /// ```
        Teal100 => "stroke-teal-100",
        /// ```css
        /// {
        ///     stroke: #99f6e4;
        /// }
        /// ```
        Teal200 => "stroke-teal-200",
        /// ```css
        /// {
        ///     stroke: #5eead4;
        /// }
        /// ```
        Teal300 => "stroke-teal-300",
        /// ```css
        /// {
        ///     stroke: #2dd4bf;
        /// }
        /// ```
        Teal400 => "stroke-teal-400",
        /// ```css
        /// {
        ///     stroke: #14b8a6;
        /// }
        /// ```
        Teal500 => "stroke-teal-500",
        /// ```css
        /// {
        ///     stroke: #0d9488;
        /// }
        /// ```
        Teal600 => "stroke-teal-600",
        /// ```css
        /// {
        ///     stroke: #0f766e;
        /// }
        /// ```
        Teal700 => "stroke-teal-700",
        /// ```css
        /// {
        ///     stroke: #115e59;
        /// }
        /// ```
        Teal800 => "stroke-teal-800",
        /// ```css
        /// {
        ///     stroke: #134e4a;
        /// }
        /// ```
        Teal900 => "stroke-teal-900",
        /// ```css
        /// {
        ///     stroke: #042f2e;
        /// }
        /// ```
        Teal950 => "stroke-teal-950",
        /// ```css
        /// {
        ///     stroke: #ecfeff;
        /// }
        /// ```
        Cyan50 => "stroke-cyan-50",
        /// ```css
        /// {
        ///     stroke: #cffafe;
        /// }
        /// ```
        Cyan100 => "stroke-cyan-100",
        /// ```css
        /// {
        ///     stroke: #a5f3fc;
        /// }
        /// ```
        Cyan200 => "stroke-cyan-200",
        /// ```css
        /// {
        ///     stroke: #67e8f9;
        /// }
        /// ```
        Cyan300 => "stroke-cyan-300",
        /// ```css
        /// {
        ///     stroke: #22d3ee;
        /// }
        /// ```
        Cyan400 => "stroke-cyan-400",
        /// ```css
        /// {
        ///     stroke: #06b6d4;
        /// }
        /// ```
        Cyan500 => "stroke-cyan-500",
        /// ```css
        /// {
        ///     stroke: #0891b2;
        /// }
        /// ```
        Cyan600 => "stroke-cyan-600",
        /// ```css
        /// {
        ///     stroke: #0e7490;
        /// }
        /// ```
        Cyan700 => "stroke-cyan-700",
        /// ```css
        /// {
        ///     stroke: #155e75;
        /// }
        /// ```
        Cyan800 => "stroke-cyan-800",
        /// ```css
        /// {
        ///     stroke: #164e63;
        /// }
        /// ```
        Cyan900 => "stroke-cyan-900",
        /// ```css
        /// {
        ///     stroke: #083344;
        /// }
        /// ```
        Cyan950 => "stroke-cyan-950",
        /// ```css
        /// {
        ///     stroke: #f0f9ff;
        /// }
        /// ```
        Sky50 => "stroke-sky-50",
        /// ```css
        /// {
        ///     stroke: #e0f2fe;
        /// }
        /// ```
        Sky100 => "stroke-sky-100",
        /// ```css
        /// {
        ///     stroke: #bae6fd;
        /// }
        /// ```
        Sky200 => "stroke-sky-200",
        /// ```css
        /// {
        ///     stroke: #7dd3fc;
        /// }
        /// ```
        Sky300 => "stroke-sky-300",
        /// ```css
        /// {
        ///     stroke: #38bdf8;
        /// }
        /// ```
        Sky400 => "stroke-sky-400",
        /// ```css
        /// {
        ///     stroke: #0ea5e9;
        /// }
        /// ```
        Sky500 => "stroke-sky-500",
        /// ```css
        /// {
        ///     stroke: #0284c7;
        /// }
        /// ```
        Sky600 => "stroke-sky-600",
        /// ```css
        /// {
        ///     stroke: #0369a1;
        /// }
        /// ```
        Sky700 => "stroke-sky-700",
        /// ```css
        /// {
        ///     stroke: #075985;
        /// }
        /// ```
        Sky800 => "stroke-sky-800",
        /// ```css
        /// {
        ///     stroke: #0c4a6e;
        /// }
        /// ```
        Sky900 => "stroke-sky-900",
        /// ```css
        /// {
        ///     stroke: #082f49;
        /// }
        /// ```
        Sky950 => "stroke-sky-950",
        /// ```css
        /// {
        ///     stroke: #eff6ff;
        /// }
        /// ```
        Blue50 => "stroke-blue-50",
        /// ```css
        /// {
        ///     stroke: #dbeafe;
        /// }
        /// ```
        Blue100 => "stroke-blue-100",
        /// ```css
        /// {
        ///     stroke: #bfdbfe;
        /// }
        /// ```
        Blue200 => "stroke-blue-200",
        /// ```css
        /// {
        ///     stroke: #93c5fd;
        /// }
        /// ```
        Blue300 => "stroke-blue-300",
        /// ```css
        /// {
        ///     stroke: #60a5fa;
        /// }
        /// ```
        Blue400 => "stroke-blue-400",
        /// ```css
        /// {
        ///     stroke: #3b82f6;
        /// }
        /// ```
        Blue500 => "stroke-blue-500",
        /// ```css
        /// {
        ///     stroke: #2563eb;
        /// }
        /// ```
        Blue600 => "stroke-blue-600",
        /// ```css
        /// {
        ///     stroke: #1d4ed8;
        /// }
        /// ```
        Blue700 => "stroke-blue-700",
        /// ```css
        /// {
        ///     stroke: #1e40af;
        /// }
        /// ```
        Blue800 => "stroke-blue-800",
        /// ```css
        /// {
        ///     stroke: #1e3a8a;
        /// }
        /// ```
        Blue900 => "stroke-blue-900",
        /// ```css
        /// {
        ///     stroke: #172554;
        /// }
        /// ```
        Blue950 => "stroke-blue-950",
        /// ```css
        /// {
        ///     stroke: #eef2ff;
        /// }
        /// ```
        Indigo50 => "stroke-indigo-50",
        /// ```css
        /// {
        ///     stroke: #e0e7ff;
        /// }
        /// ```
        Indigo100 => "stroke-indigo-100",
        /// ```css
        /// {
        ///     stroke: #c7d2fe;
        /// }
        /// ```
        Indigo200 => "stroke-indigo-200",
        /// ```css
        /// {
        ///     stroke: #a5b4fc;
        /// }
        /// ```
        Indigo300 => "stroke-indigo-300",
        /// ```css
        /// {
        ///     stroke: #818cf8;
        /// }
        /// ```
        Indigo400 => "stroke-indigo-400",
        /// ```css
        /// {
        ///     stroke: #6366f1;
        /// }
        /// ```
        Indigo500 => "stroke-indigo-500",
        /// ```css
        /// {
        ///     stroke: #4f46e5;
        /// }
        /// ```
        Indigo600 => "stroke-indigo-600",
        /// ```css
        /// {
        ///     stroke: #4338ca;
        /// }
        /// ```
        Indigo700 => "stroke-indigo-700",
        /// ```css
        /// {
        ///     stroke: #3730a3;
        /// }
        /// ```
        Indigo800 => "stroke-indigo-800",
        /// ```css
        /// {
        ///     stroke: #312e81;
        /// }
        /// ```
        Indigo900 => "stroke-indigo-900",
        /// ```css
        /// {
        ///     stroke: #1e1b4b;
        /// }
        /// ```
        Indigo950 => "stroke-indigo-950",
        /// ```css
        /// {
        ///     stroke: #f5f3ff;
        /// }
        /// ```
        Violet50 => "stroke-violet-50",
        /// ```css
        /// {
        ///     stroke: #ede9fe;
        /// }
        /// ```
        Violet100 => "stroke-violet-100",
        /// ```css
        /// {
        ///     stroke: #ddd6fe;
        /// }
        /// ```
        Violet200 => "stroke-violet-200",
        /// ```css
        /// {
        ///     stroke: #c4b5fd;
        /// }
        /// ```
        Violet300 => "stroke-violet-300",
        /// ```css
        /// {
        ///     stroke: #a78bfa;
        /// }
        /// ```
        Violet400 => "stroke-violet-400",
        /// ```css
        /// {
        ///     stroke: #8b5cf6;
        /// }
        /// ```
        Violet500 => "stroke-violet-500",
        /// ```css
        /// {
        ///     stroke: #7c3aed;
        /// }
        /// ```
        Violet600 => "stroke-violet-600",
        /// ```css
        /// {
        ///     stroke: #6d28d9;
        /// }
        /// ```
        Violet700 => "stroke-violet-700",
        /// ```css
        /// {
        ///     stroke: #5b21b6;
        /// }
        /// ```
        Violet800 => "stroke-violet-800",
        /// ```css
        /// {
        ///     stroke: #4c1d95;
        /// }
        /// ```
        Violet900 => "stroke-violet-900",
        /// ```css
        /// {
        ///     stroke: #2e1065;
        /// }
        /// ```
        Violet950 => "stroke-violet-950",
        /// ```css
        /// {
        ///     stroke: #faf5ff;
        /// }
        /// ```
        Purple50 => "stroke-purple-50",
        /// ```css
        /// {
        ///     stroke: #f3e8ff;
        /// }
        /// ```
        Purple100 => "stroke-purple-100",
        /// ```css
        /// {
        ///     stroke: #e9d5ff;
        /// }
        /// ```
        Purple200 => "stroke-purple-200",
        /// ```css
        /// {
        ///     stroke: #d8b4fe;
        /// }
        /// ```
        Purple300 => "stroke-purple-300",
        /// ```css
        /// {
        ///     stroke: #c084fc;
        /// }
        /// ```
        Purple400 => "stroke-purple-400",
        /// ```css
        /// {
        ///     stroke: #a855f7;
        /// }
        /// ```
        Purple500 => "stroke-purple-500",
        /// ```css
        /// {
        ///     stroke: #9333ea;
        /// }
        /// ```
        Purple600 => "stroke-purple-600",
        /// ```css
        /// {
        ///     stroke: #7e22ce;
        /// }
        /// ```
        Purple700 => "stroke-purple-700",
        /// ```css
        /// {
        ///     stroke: #6b21a8;
        /// }
        /// ```
        Purple800 => "stroke-purple-800",
        /// ```css
        /// {
        ///     stroke: #581c87;
        /// }
        /// ```
        Purple900 => "stroke-purple-900",
        /// ```css
        /// {
        ///     stroke: #3b0764;
        /// }
        /// ```
        Purple950 => "stroke-purple-950",
        /// ```css
        /// {
        ///     stroke: #fdf4ff;
        /// }
        /// ```
        Fuchsia50 => "stroke-fuchsia-50",
        /// ```css
        /// {
        ///     stroke: #fae8ff;
        /// }
        /// ```
        Fuchsia100 => "stroke-fuchsia-100",
        /// ```css
        /// {
        ///     stroke: #f5d0fe;
        /// }
        /// ```
        Fuchsia200 => "stroke-fuchsia-200",
        /// ```css
        /// {
        ///     stroke: #f0abfc;
        /// }
        /// ```
        Fuchsia300 => "stroke-fuchsia-300",
        /// ```css
        /// {
        ///     stroke: #e879f9;
        /// }
        /// ```
        Fuchsia400 => "stroke-fuchsia-400",
        /// ```css
        /// {
        ///     stroke: #d946ef;
        /// }
        /// ```
        Fuchsia500 => "stroke-fuchsia-500",
        /// ```css
        /// {
        ///     stroke: #c026d3;
        /// }
        /// ```
        Fuchsia600 => "stroke-fuchsia-600",
        /// ```css
        /// {
        ///     stroke: #a21caf;
        /// }
        /// ```
        Fuchsia700 => "stroke-fuchsia-700",
        /// ```css
        /// {
        ///     stroke: #86198f;
        /// }
        /// ```
        Fuchsia800 => "stroke-fuchsia-800",
        /// ```css
        /// {
        ///     stroke: #701a75;
        /// }
        /// ```
        Fuchsia900 => "stroke-fuchsia-900",
        /// ```css
        /// {
        ///     stroke: #4a044e;
        /// }
        /// ```
        Fuchsia950 => "stroke-fuchsia-950",
        /// ```css
        /// {
        ///     stroke: #fdf2f8;
        /// }
        /// ```
        Pink50 => "stroke-pink-50",
        /// ```css
        /// {
        ///     stroke: #fce7f3;
        /// }
        /// ```
        Pink100 => "stroke-pink-100",
        /// ```css
        /// {
        ///     stroke: #fbcfe8;
        /// }
        /// ```
        Pink200 => "stroke-pink-200",
        /// ```css
        /// {
        ///     stroke: #f9a8d4;
        /// }
        /// ```
        Pink300 => "stroke-pink-300",
        /// ```css
        /// {
        ///     stroke: #f472b6;
        /// }
        /// ```
        Pink400 => "stroke-pink-400",
        /// ```css
        /// {
        ///     stroke: #ec4899;
        /// }
        /// ```
        Pink500 => "stroke-pink-500",
        /// ```css
        /// {
        ///     stroke: #db2777;
        /// }
        /// ```
        Pink600 => "stroke-pink-600",
        /// ```css
        /// {
        ///     stroke: #be185d;
        /// }
        /// ```
        Pink700 => "stroke-pink-700",
        /// ```css
        /// {
        ///     stroke: #9d174d;
        /// }
        /// ```
        Pink800 => "stroke-pink-800",
        /// ```css
        /// {
        ///     stroke: #831843;
        /// }
        /// ```
        Pink900 => "stroke-pink-900",
        /// ```css
        /// {
        ///     stroke: #500724;
        /// }
        /// ```
        Pink950 => "stroke-pink-950",
        /// ```css
        /// {
        ///     stroke: #fff1f2;
        /// }
        /// ```
        Rose50 => "stroke-rose-50",
        /// ```css
        /// {
        ///     stroke: #ffe4e6;
        /// }
        /// ```
        Rose100 => "stroke-rose-100",
        /// ```css
        /// {
        ///     stroke: #fecdd3;
        /// }
        /// ```
        Rose200 => "stroke-rose-200",
        /// ```css
        /// {
        ///     stroke: #fda4af;
        /// }
        /// ```
        Rose300 => "stroke-rose-300",
        /// ```css
        /// {
        ///     stroke: #fb7185;
        /// }
        /// ```
        Rose400 => "stroke-rose-400",
        /// ```css
        /// {
        ///     stroke: #f43f5e;
        /// }
        /// ```
        Rose500 => "stroke-rose-500",
        /// ```css
        /// {
        ///     stroke: #e11d48;
        /// }
        /// ```
        Rose600 => "stroke-rose-600",
        /// ```css
        /// {
        ///     stroke: #be123c;
        /// }
        /// ```
        Rose700 => "stroke-rose-700",
        /// ```css
        /// {
        ///     stroke: #9f1239;
        /// }
        /// ```
        Rose800 => "stroke-rose-800",
        /// ```css
        /// {
        ///     stroke: #881337;
        /// }
        /// ```
        Rose900 => "stroke-rose-900",
        /// ```css
        /// {
        ///     stroke: #4c0519;
        /// }
        /// ```
        Rose950 => "stroke-rose-950",
    }
    /// Utilities for styling the stroke width of SVG elements.
    ///
    /// <https://tailwindcss.com/docs/stroke-width>
    StrokeWidth {
        /// ```css
        /// {
        ///     stroke-width: 0;
        /// }
        /// ```
        _0 => "stroke-0",
        /// ```css
        /// {
        ///     stroke-width: 1;
        /// }
        /// ```
        _1 => "stroke-1",
        /// ```css
        /// {
        ///     stroke-width: 2;
        /// }
        /// ```
        _2 => "stroke-2",
    }
);
