def_types!(
    /// Utilities for controlling the accented color of a form control.
    ///
    /// <https://tailwindcss.com/docs/accent-color>
    AccentColor {
        /// ```css
        /// {
        ///     accent-color: inherit;
        /// }
        /// ```
        Inherit => "accent-inherit",
        /// ```css
        /// {
        ///     accent-color: currentColor;
        /// }
        /// ```
        Current => "accent-current",
        /// ```css
        /// {
        ///     accent-color: transparent;
        /// }
        /// ```
        Transparent => "accent-transparent",
        /// ```css
        /// {
        ///     accent-color: #000;
        /// }
        /// ```
        Black => "accent-black",
        /// ```css
        /// {
        ///     accent-color: #fff;
        /// }
        /// ```
        White => "accent-white",
        /// ```css
        /// {
        ///     accent-color: #f8fafc;
        /// }
        /// ```
        Slate50 => "accent-slate-50",
        /// ```css
        /// {
        ///     accent-color: #f1f5f9;
        /// }
        /// ```
        Slate100 => "accent-slate-100",
        /// ```css
        /// {
        ///     accent-color: #e2e8f0;
        /// }
        /// ```
        Slate200 => "accent-slate-200",
        /// ```css
        /// {
        ///     accent-color: #cbd5e1;
        /// }
        /// ```
        Slate300 => "accent-slate-300",
        /// ```css
        /// {
        ///     accent-color: #94a3b8;
        /// }
        /// ```
        Slate400 => "accent-slate-400",
        /// ```css
        /// {
        ///     accent-color: #64748b;
        /// }
        /// ```
        Slate500 => "accent-slate-500",
        /// ```css
        /// {
        ///     accent-color: #475569;
        /// }
        /// ```
        Slate600 => "accent-slate-600",
        /// ```css
        /// {
        ///     accent-color: #334155;
        /// }
        /// ```
        Slate700 => "accent-slate-700",
        /// ```css
        /// {
        ///     accent-color: #1e293b;
        /// }
        /// ```
        Slate800 => "accent-slate-800",
        /// ```css
        /// {
        ///     accent-color: #0f172a;
        /// }
        /// ```
        Slate900 => "accent-slate-900",
        /// ```css
        /// {
        ///     accent-color: #020617;
        /// }
        /// ```
        Slate950 => "accent-slate-950",
        /// ```css
        /// {
        ///     accent-color: #f9fafb;
        /// }
        /// ```
        Gray50 => "accent-gray-50",
        /// ```css
        /// {
        ///     accent-color: #f3f4f6;
        /// }
        /// ```
        Gray100 => "accent-gray-100",
        /// ```css
        /// {
        ///     accent-color: #e5e7eb;
        /// }
        /// ```
        Gray200 => "accent-gray-200",
        /// ```css
        /// {
        ///     accent-color: #d1d5db;
        /// }
        /// ```
        Gray300 => "accent-gray-300",
        /// ```css
        /// {
        ///     accent-color: #9ca3af;
        /// }
        /// ```
        Gray400 => "accent-gray-400",
        /// ```css
        /// {
        ///     accent-color: #6b7280;
        /// }
        /// ```
        Gray500 => "accent-gray-500",
        /// ```css
        /// {
        ///     accent-color: #4b5563;
        /// }
        /// ```
        Gray600 => "accent-gray-600",
        /// ```css
        /// {
        ///     accent-color: #374151;
        /// }
        /// ```
        Gray700 => "accent-gray-700",
        /// ```css
        /// {
        ///     accent-color: #1f2937;
        /// }
        /// ```
        Gray800 => "accent-gray-800",
        /// ```css
        /// {
        ///     accent-color: #111827;
        /// }
        /// ```
        Gray900 => "accent-gray-900",
        /// ```css
        /// {
        ///     accent-color: #030712;
        /// }
        /// ```
        Gray950 => "accent-gray-950",
        /// ```css
        /// {
        ///     accent-color: #fafafa;
        /// }
        /// ```
        Zinc50 => "accent-zinc-50",
        /// ```css
        /// {
        ///     accent-color: #f4f4f5;
        /// }
        /// ```
        Zinc100 => "accent-zinc-100",
        /// ```css
        /// {
        ///     accent-color: #e4e4e7;
        /// }
        /// ```
        Zinc200 => "accent-zinc-200",
        /// ```css
        /// {
        ///     accent-color: #d4d4d8;
        /// }
        /// ```
        Zinc300 => "accent-zinc-300",
        /// ```css
        /// {
        ///     accent-color: #a1a1aa;
        /// }
        /// ```
        Zinc400 => "accent-zinc-400",
        /// ```css
        /// {
        ///     accent-color: #71717a;
        /// }
        /// ```
        Zinc500 => "accent-zinc-500",
        /// ```css
        /// {
        ///     accent-color: #52525b;
        /// }
        /// ```
        Zinc600 => "accent-zinc-600",
        /// ```css
        /// {
        ///     accent-color: #3f3f46;
        /// }
        /// ```
        Zinc700 => "accent-zinc-700",
        /// ```css
        /// {
        ///     accent-color: #27272a;
        /// }
        /// ```
        Zinc800 => "accent-zinc-800",
        /// ```css
        /// {
        ///     accent-color: #18181b;
        /// }
        /// ```
        Zinc900 => "accent-zinc-900",
        /// ```css
        /// {
        ///     accent-color: #09090b;
        /// }
        /// ```
        Zinc950 => "accent-zinc-950",
        /// ```css
        /// {
        ///     accent-color: #fafafa;
        /// }
        /// ```
        Neutral50 => "accent-neutral-50",
        /// ```css
        /// {
        ///     accent-color: #f5f5f5;
        /// }
        /// ```
        Neutral100 => "accent-neutral-100",
        /// ```css
        /// {
        ///     accent-color: #e5e5e5;
        /// }
        /// ```
        Neutral200 => "accent-neutral-200",
        /// ```css
        /// {
        ///     accent-color: #d4d4d4;
        /// }
        /// ```
        Neutral300 => "accent-neutral-300",
        /// ```css
        /// {
        ///     accent-color: #a3a3a3;
        /// }
        /// ```
        Neutral400 => "accent-neutral-400",
        /// ```css
        /// {
        ///     accent-color: #737373;
        /// }
        /// ```
        Neutral500 => "accent-neutral-500",
        /// ```css
        /// {
        ///     accent-color: #525252;
        /// }
        /// ```
        Neutral600 => "accent-neutral-600",
        /// ```css
        /// {
        ///     accent-color: #404040;
        /// }
        /// ```
        Neutral700 => "accent-neutral-700",
        /// ```css
        /// {
        ///     accent-color: #262626;
        /// }
        /// ```
        Neutral800 => "accent-neutral-800",
        /// ```css
        /// {
        ///     accent-color: #171717;
        /// }
        /// ```
        Neutral900 => "accent-neutral-900",
        /// ```css
        /// {
        ///     accent-color: #0a0a0a;
        /// }
        /// ```
        Neutral950 => "accent-neutral-950",
        /// ```css
        /// {
        ///     accent-color: #fafaf9;
        /// }
        /// ```
        Stone50 => "accent-stone-50",
        /// ```css
        /// {
        ///     accent-color: #f5f5f4;
        /// }
        /// ```
        Stone100 => "accent-stone-100",
        /// ```css
        /// {
        ///     accent-color: #e7e5e4;
        /// }
        /// ```
        Stone200 => "accent-stone-200",
        /// ```css
        /// {
        ///     accent-color: #d6d3d1;
        /// }
        /// ```
        Stone300 => "accent-stone-300",
        /// ```css
        /// {
        ///     accent-color: #a8a29e;
        /// }
        /// ```
        Stone400 => "accent-stone-400",
        /// ```css
        /// {
        ///     accent-color: #78716c;
        /// }
        /// ```
        Stone500 => "accent-stone-500",
        /// ```css
        /// {
        ///     accent-color: #57534e;
        /// }
        /// ```
        Stone600 => "accent-stone-600",
        /// ```css
        /// {
        ///     accent-color: #44403c;
        /// }
        /// ```
        Stone700 => "accent-stone-700",
        /// ```css
        /// {
        ///     accent-color: #292524;
        /// }
        /// ```
        Stone800 => "accent-stone-800",
        /// ```css
        /// {
        ///     accent-color: #1c1917;
        /// }
        /// ```
        Stone900 => "accent-stone-900",
        /// ```css
        /// {
        ///     accent-color: #0c0a09;
        /// }
        /// ```
        Stone950 => "accent-stone-950",
        /// ```css
        /// {
        ///     accent-color: #fef2f2;
        /// }
        /// ```
        Red50 => "accent-red-50",
        /// ```css
        /// {
        ///     accent-color: #fee2e2;
        /// }
        /// ```
        Red100 => "accent-red-100",
        /// ```css
        /// {
        ///     accent-color: #fecaca;
        /// }
        /// ```
        Red200 => "accent-red-200",
        /// ```css
        /// {
        ///     accent-color: #fca5a5;
        /// }
        /// ```
        Red300 => "accent-red-300",
        /// ```css
        /// {
        ///     accent-color: #f87171;
        /// }
        /// ```
        Red400 => "accent-red-400",
        /// ```css
        /// {
        ///     accent-color: #ef4444;
        /// }
        /// ```
        Red500 => "accent-red-500",
        /// ```css
        /// {
        ///     accent-color: #dc2626;
        /// }
        /// ```
        Red600 => "accent-red-600",
        /// ```css
        /// {
        ///     accent-color: #b91c1c;
        /// }
        /// ```
        Red700 => "accent-red-700",
        /// ```css
        /// {
        ///     accent-color: #991b1b;
        /// }
        /// ```
        Red800 => "accent-red-800",
        /// ```css
        /// {
        ///     accent-color: #7f1d1d;
        /// }
        /// ```
        Red900 => "accent-red-900",
        /// ```css
        /// {
        ///     accent-color: #450a0a;
        /// }
        /// ```
        Red950 => "accent-red-950",
        /// ```css
        /// {
        ///     accent-color: #fff7ed;
        /// }
        /// ```
        Orange50 => "accent-orange-50",
        /// ```css
        /// {
        ///     accent-color: #ffedd5;
        /// }
        /// ```
        Orange100 => "accent-orange-100",
        /// ```css
        /// {
        ///     accent-color: #fed7aa;
        /// }
        /// ```
        Orange200 => "accent-orange-200",
        /// ```css
        /// {
        ///     accent-color: #fdba74;
        /// }
        /// ```
        Orange300 => "accent-orange-300",
        /// ```css
        /// {
        ///     accent-color: #fb923c;
        /// }
        /// ```
        Orange400 => "accent-orange-400",
        /// ```css
        /// {
        ///     accent-color: #f97316;
        /// }
        /// ```
        Orange500 => "accent-orange-500",
        /// ```css
        /// {
        ///     accent-color: #ea580c;
        /// }
        /// ```
        Orange600 => "accent-orange-600",
        /// ```css
        /// {
        ///     accent-color: #c2410c;
        /// }
        /// ```
        Orange700 => "accent-orange-700",
        /// ```css
        /// {
        ///     accent-color: #9a3412;
        /// }
        /// ```
        Orange800 => "accent-orange-800",
        /// ```css
        /// {
        ///     accent-color: #7c2d12;
        /// }
        /// ```
        Orange900 => "accent-orange-900",
        /// ```css
        /// {
        ///     accent-color: #431407;
        /// }
        /// ```
        Orange950 => "accent-orange-950",
        /// ```css
        /// {
        ///     accent-color: #fffbeb;
        /// }
        /// ```
        Amber50 => "accent-amber-50",
        /// ```css
        /// {
        ///     accent-color: #fef3c7;
        /// }
        /// ```
        Amber100 => "accent-amber-100",
        /// ```css
        /// {
        ///     accent-color: #fde68a;
        /// }
        /// ```
        Amber200 => "accent-amber-200",
        /// ```css
        /// {
        ///     accent-color: #fcd34d;
        /// }
        /// ```
        Amber300 => "accent-amber-300",
        /// ```css
        /// {
        ///     accent-color: #fbbf24;
        /// }
        /// ```
        Amber400 => "accent-amber-400",
        /// ```css
        /// {
        ///     accent-color: #f59e0b;
        /// }
        /// ```
        Amber500 => "accent-amber-500",
        /// ```css
        /// {
        ///     accent-color: #d97706;
        /// }
        /// ```
        Amber600 => "accent-amber-600",
        /// ```css
        /// {
        ///     accent-color: #b45309;
        /// }
        /// ```
        Amber700 => "accent-amber-700",
        /// ```css
        /// {
        ///     accent-color: #92400e;
        /// }
        /// ```
        Amber800 => "accent-amber-800",
        /// ```css
        /// {
        ///     accent-color: #78350f;
        /// }
        /// ```
        Amber900 => "accent-amber-900",
        /// ```css
        /// {
        ///     accent-color: #451a03;
        /// }
        /// ```
        Amber950 => "accent-amber-950",
        /// ```css
        /// {
        ///     accent-color: #fefce8;
        /// }
        /// ```
        Yellow50 => "accent-yellow-50",
        /// ```css
        /// {
        ///     accent-color: #fef9c3;
        /// }
        /// ```
        Yellow100 => "accent-yellow-100",
        /// ```css
        /// {
        ///     accent-color: #fef08a;
        /// }
        /// ```
        Yellow200 => "accent-yellow-200",
        /// ```css
        /// {
        ///     accent-color: #fde047;
        /// }
        /// ```
        Yellow300 => "accent-yellow-300",
        /// ```css
        /// {
        ///     accent-color: #facc15;
        /// }
        /// ```
        Yellow400 => "accent-yellow-400",
        /// ```css
        /// {
        ///     accent-color: #eab308;
        /// }
        /// ```
        Yellow500 => "accent-yellow-500",
        /// ```css
        /// {
        ///     accent-color: #ca8a04;
        /// }
        /// ```
        Yellow600 => "accent-yellow-600",
        /// ```css
        /// {
        ///     accent-color: #a16207;
        /// }
        /// ```
        Yellow700 => "accent-yellow-700",
        /// ```css
        /// {
        ///     accent-color: #854d0e;
        /// }
        /// ```
        Yellow800 => "accent-yellow-800",
        /// ```css
        /// {
        ///     accent-color: #713f12;
        /// }
        /// ```
        Yellow900 => "accent-yellow-900",
        /// ```css
        /// {
        ///     accent-color: #422006;
        /// }
        /// ```
        Yellow950 => "accent-yellow-950",
        /// ```css
        /// {
        ///     accent-color: #f7fee7;
        /// }
        /// ```
        Lime50 => "accent-lime-50",
        /// ```css
        /// {
        ///     accent-color: #ecfccb;
        /// }
        /// ```
        Lime100 => "accent-lime-100",
        /// ```css
        /// {
        ///     accent-color: #d9f99d;
        /// }
        /// ```
        Lime200 => "accent-lime-200",
        /// ```css
        /// {
        ///     accent-color: #bef264;
        /// }
        /// ```
        Lime300 => "accent-lime-300",
        /// ```css
        /// {
        ///     accent-color: #a3e635;
        /// }
        /// ```
        Lime400 => "accent-lime-400",
        /// ```css
        /// {
        ///     accent-color: #84cc16;
        /// }
        /// ```
        Lime500 => "accent-lime-500",
        /// ```css
        /// {
        ///     accent-color: #65a30d;
        /// }
        /// ```
        Lime600 => "accent-lime-600",
        /// ```css
        /// {
        ///     accent-color: #4d7c0f;
        /// }
        /// ```
        Lime700 => "accent-lime-700",
        /// ```css
        /// {
        ///     accent-color: #3f6212;
        /// }
        /// ```
        Lime800 => "accent-lime-800",
        /// ```css
        /// {
        ///     accent-color: #365314;
        /// }
        /// ```
        Lime900 => "accent-lime-900",
        /// ```css
        /// {
        ///     accent-color: #1a2e05;
        /// }
        /// ```
        Lime950 => "accent-lime-950",
        /// ```css
        /// {
        ///     accent-color: #f0fdf4;
        /// }
        /// ```
        Green50 => "accent-green-50",
        /// ```css
        /// {
        ///     accent-color: #dcfce7;
        /// }
        /// ```
        Green100 => "accent-green-100",
        /// ```css
        /// {
        ///     accent-color: #bbf7d0;
        /// }
        /// ```
        Green200 => "accent-green-200",
        /// ```css
        /// {
        ///     accent-color: #86efac;
        /// }
        /// ```
        Green300 => "accent-green-300",
        /// ```css
        /// {
        ///     accent-color: #4ade80;
        /// }
        /// ```
        Green400 => "accent-green-400",
        /// ```css
        /// {
        ///     accent-color: #22c55e;
        /// }
        /// ```
        Green500 => "accent-green-500",
        /// ```css
        /// {
        ///     accent-color: #16a34a;
        /// }
        /// ```
        Green600 => "accent-green-600",
        /// ```css
        /// {
        ///     accent-color: #15803d;
        /// }
        /// ```
        Green700 => "accent-green-700",
        /// ```css
        /// {
        ///     accent-color: #166534;
        /// }
        /// ```
        Green800 => "accent-green-800",
        /// ```css
        /// {
        ///     accent-color: #14532d;
        /// }
        /// ```
        Green900 => "accent-green-900",
        /// ```css
        /// {
        ///     accent-color: #052e16;
        /// }
        /// ```
        Green950 => "accent-green-950",
        /// ```css
        /// {
        ///     accent-color: #ecfdf5;
        /// }
        /// ```
        Emerald50 => "accent-emerald-50",
        /// ```css
        /// {
        ///     accent-color: #d1fae5;
        /// }
        /// ```
        Emerald100 => "accent-emerald-100",
        /// ```css
        /// {
        ///     accent-color: #a7f3d0;
        /// }
        /// ```
        Emerald200 => "accent-emerald-200",
        /// ```css
        /// {
        ///     accent-color: #6ee7b7;
        /// }
        /// ```
        Emerald300 => "accent-emerald-300",
        /// ```css
        /// {
        ///     accent-color: #34d399;
        /// }
        /// ```
        Emerald400 => "accent-emerald-400",
        /// ```css
        /// {
        ///     accent-color: #10b981;
        /// }
        /// ```
        Emerald500 => "accent-emerald-500",
        /// ```css
        /// {
        ///     accent-color: #059669;
        /// }
        /// ```
        Emerald600 => "accent-emerald-600",
        /// ```css
        /// {
        ///     accent-color: #047857;
        /// }
        /// ```
        Emerald700 => "accent-emerald-700",
        /// ```css
        /// {
        ///     accent-color: #065f46;
        /// }
        /// ```
        Emerald800 => "accent-emerald-800",
        /// ```css
        /// {
        ///     accent-color: #064e3b;
        /// }
        /// ```
        Emerald900 => "accent-emerald-900",
        /// ```css
        /// {
        ///     accent-color: #022c22;
        /// }
        /// ```
        Emerald950 => "accent-emerald-950",
        /// ```css
        /// {
        ///     accent-color: #f0fdfa;
        /// }
        /// ```
        Teal50 => "accent-teal-50",
        /// ```css
        /// {
        ///     accent-color: #ccfbf1;
        /// }
        /// ```
        Teal100 => "accent-teal-100",
        /// ```css
        /// {
        ///     accent-color: #99f6e4;
        /// }
        /// ```
        Teal200 => "accent-teal-200",
        /// ```css
        /// {
        ///     accent-color: #5eead4;
        /// }
        /// ```
        Teal300 => "accent-teal-300",
        /// ```css
        /// {
        ///     accent-color: #2dd4bf;
        /// }
        /// ```
        Teal400 => "accent-teal-400",
        /// ```css
        /// {
        ///     accent-color: #14b8a6;
        /// }
        /// ```
        Teal500 => "accent-teal-500",
        /// ```css
        /// {
        ///     accent-color: #0d9488;
        /// }
        /// ```
        Teal600 => "accent-teal-600",
        /// ```css
        /// {
        ///     accent-color: #0f766e;
        /// }
        /// ```
        Teal700 => "accent-teal-700",
        /// ```css
        /// {
        ///     accent-color: #115e59;
        /// }
        /// ```
        Teal800 => "accent-teal-800",
        /// ```css
        /// {
        ///     accent-color: #134e4a;
        /// }
        /// ```
        Teal900 => "accent-teal-900",
        /// ```css
        /// {
        ///     accent-color: #042f2e;
        /// }
        /// ```
        Teal950 => "accent-teal-950",
        /// ```css
        /// {
        ///     accent-color: #ecfeff;
        /// }
        /// ```
        Cyan50 => "accent-cyan-50",
        /// ```css
        /// {
        ///     accent-color: #cffafe;
        /// }
        /// ```
        Cyan100 => "accent-cyan-100",
        /// ```css
        /// {
        ///     accent-color: #a5f3fc;
        /// }
        /// ```
        Cyan200 => "accent-cyan-200",
        /// ```css
        /// {
        ///     accent-color: #67e8f9;
        /// }
        /// ```
        Cyan300 => "accent-cyan-300",
        /// ```css
        /// {
        ///     accent-color: #22d3ee;
        /// }
        /// ```
        Cyan400 => "accent-cyan-400",
        /// ```css
        /// {
        ///     accent-color: #06b6d4;
        /// }
        /// ```
        Cyan500 => "accent-cyan-500",
        /// ```css
        /// {
        ///     accent-color: #0891b2;
        /// }
        /// ```
        Cyan600 => "accent-cyan-600",
        /// ```css
        /// {
        ///     accent-color: #0e7490;
        /// }
        /// ```
        Cyan700 => "accent-cyan-700",
        /// ```css
        /// {
        ///     accent-color: #155e75;
        /// }
        /// ```
        Cyan800 => "accent-cyan-800",
        /// ```css
        /// {
        ///     accent-color: #164e63;
        /// }
        /// ```
        Cyan900 => "accent-cyan-900",
        /// ```css
        /// {
        ///     accent-color: #083344;
        /// }
        /// ```
        Cyan950 => "accent-cyan-950",
        /// ```css
        /// {
        ///     accent-color: #f0f9ff;
        /// }
        /// ```
        Sky50 => "accent-sky-50",
        /// ```css
        /// {
        ///     accent-color: #e0f2fe;
        /// }
        /// ```
        Sky100 => "accent-sky-100",
        /// ```css
        /// {
        ///     accent-color: #bae6fd;
        /// }
        /// ```
        Sky200 => "accent-sky-200",
        /// ```css
        /// {
        ///     accent-color: #7dd3fc;
        /// }
        /// ```
        Sky300 => "accent-sky-300",
        /// ```css
        /// {
        ///     accent-color: #38bdf8;
        /// }
        /// ```
        Sky400 => "accent-sky-400",
        /// ```css
        /// {
        ///     accent-color: #0ea5e9;
        /// }
        /// ```
        Sky500 => "accent-sky-500",
        /// ```css
        /// {
        ///     accent-color: #0284c7;
        /// }
        /// ```
        Sky600 => "accent-sky-600",
        /// ```css
        /// {
        ///     accent-color: #0369a1;
        /// }
        /// ```
        Sky700 => "accent-sky-700",
        /// ```css
        /// {
        ///     accent-color: #075985;
        /// }
        /// ```
        Sky800 => "accent-sky-800",
        /// ```css
        /// {
        ///     accent-color: #0c4a6e;
        /// }
        /// ```
        Sky900 => "accent-sky-900",
        /// ```css
        /// {
        ///     accent-color: #082f49;
        /// }
        /// ```
        Sky950 => "accent-sky-950",
        /// ```css
        /// {
        ///     accent-color: #eff6ff;
        /// }
        /// ```
        Blue50 => "accent-blue-50",
        /// ```css
        /// {
        ///     accent-color: #dbeafe;
        /// }
        /// ```
        Blue100 => "accent-blue-100",
        /// ```css
        /// {
        ///     accent-color: #bfdbfe;
        /// }
        /// ```
        Blue200 => "accent-blue-200",
        /// ```css
        /// {
        ///     accent-color: #93c5fd;
        /// }
        /// ```
        Blue300 => "accent-blue-300",
        /// ```css
        /// {
        ///     accent-color: #60a5fa;
        /// }
        /// ```
        Blue400 => "accent-blue-400",
        /// ```css
        /// {
        ///     accent-color: #3b82f6;
        /// }
        /// ```
        Blue500 => "accent-blue-500",
        /// ```css
        /// {
        ///     accent-color: #2563eb;
        /// }
        /// ```
        Blue600 => "accent-blue-600",
        /// ```css
        /// {
        ///     accent-color: #1d4ed8;
        /// }
        /// ```
        Blue700 => "accent-blue-700",
        /// ```css
        /// {
        ///     accent-color: #1e40af;
        /// }
        /// ```
        Blue800 => "accent-blue-800",
        /// ```css
        /// {
        ///     accent-color: #1e3a8a;
        /// }
        /// ```
        Blue900 => "accent-blue-900",
        /// ```css
        /// {
        ///     accent-color: #172554;
        /// }
        /// ```
        Blue950 => "accent-blue-950",
        /// ```css
        /// {
        ///     accent-color: #eef2ff;
        /// }
        /// ```
        Indigo50 => "accent-indigo-50",
        /// ```css
        /// {
        ///     accent-color: #e0e7ff;
        /// }
        /// ```
        Indigo100 => "accent-indigo-100",
        /// ```css
        /// {
        ///     accent-color: #c7d2fe;
        /// }
        /// ```
        Indigo200 => "accent-indigo-200",
        /// ```css
        /// {
        ///     accent-color: #a5b4fc;
        /// }
        /// ```
        Indigo300 => "accent-indigo-300",
        /// ```css
        /// {
        ///     accent-color: #818cf8;
        /// }
        /// ```
        Indigo400 => "accent-indigo-400",
        /// ```css
        /// {
        ///     accent-color: #6366f1;
        /// }
        /// ```
        Indigo500 => "accent-indigo-500",
        /// ```css
        /// {
        ///     accent-color: #4f46e5;
        /// }
        /// ```
        Indigo600 => "accent-indigo-600",
        /// ```css
        /// {
        ///     accent-color: #4338ca;
        /// }
        /// ```
        Indigo700 => "accent-indigo-700",
        /// ```css
        /// {
        ///     accent-color: #3730a3;
        /// }
        /// ```
        Indigo800 => "accent-indigo-800",
        /// ```css
        /// {
        ///     accent-color: #312e81;
        /// }
        /// ```
        Indigo900 => "accent-indigo-900",
        /// ```css
        /// {
        ///     accent-color: #1e1b4b;
        /// }
        /// ```
        Indigo950 => "accent-indigo-950",
        /// ```css
        /// {
        ///     accent-color: #f5f3ff;
        /// }
        /// ```
        Violet50 => "accent-violet-50",
        /// ```css
        /// {
        ///     accent-color: #ede9fe;
        /// }
        /// ```
        Violet100 => "accent-violet-100",
        /// ```css
        /// {
        ///     accent-color: #ddd6fe;
        /// }
        /// ```
        Violet200 => "accent-violet-200",
        /// ```css
        /// {
        ///     accent-color: #c4b5fd;
        /// }
        /// ```
        Violet300 => "accent-violet-300",
        /// ```css
        /// {
        ///     accent-color: #a78bfa;
        /// }
        /// ```
        Violet400 => "accent-violet-400",
        /// ```css
        /// {
        ///     accent-color: #8b5cf6;
        /// }
        /// ```
        Violet500 => "accent-violet-500",
        /// ```css
        /// {
        ///     accent-color: #7c3aed;
        /// }
        /// ```
        Violet600 => "accent-violet-600",
        /// ```css
        /// {
        ///     accent-color: #6d28d9;
        /// }
        /// ```
        Violet700 => "accent-violet-700",
        /// ```css
        /// {
        ///     accent-color: #5b21b6;
        /// }
        /// ```
        Violet800 => "accent-violet-800",
        /// ```css
        /// {
        ///     accent-color: #4c1d95;
        /// }
        /// ```
        Violet900 => "accent-violet-900",
        /// ```css
        /// {
        ///     accent-color: #2e1065;
        /// }
        /// ```
        Violet950 => "accent-violet-950",
        /// ```css
        /// {
        ///     accent-color: #faf5ff;
        /// }
        /// ```
        Purple50 => "accent-purple-50",
        /// ```css
        /// {
        ///     accent-color: #f3e8ff;
        /// }
        /// ```
        Purple100 => "accent-purple-100",
        /// ```css
        /// {
        ///     accent-color: #e9d5ff;
        /// }
        /// ```
        Purple200 => "accent-purple-200",
        /// ```css
        /// {
        ///     accent-color: #d8b4fe;
        /// }
        /// ```
        Purple300 => "accent-purple-300",
        /// ```css
        /// {
        ///     accent-color: #c084fc;
        /// }
        /// ```
        Purple400 => "accent-purple-400",
        /// ```css
        /// {
        ///     accent-color: #a855f7;
        /// }
        /// ```
        Purple500 => "accent-purple-500",
        /// ```css
        /// {
        ///     accent-color: #9333ea;
        /// }
        /// ```
        Purple600 => "accent-purple-600",
        /// ```css
        /// {
        ///     accent-color: #7e22ce;
        /// }
        /// ```
        Purple700 => "accent-purple-700",
        /// ```css
        /// {
        ///     accent-color: #6b21a8;
        /// }
        /// ```
        Purple800 => "accent-purple-800",
        /// ```css
        /// {
        ///     accent-color: #581c87;
        /// }
        /// ```
        Purple900 => "accent-purple-900",
        /// ```css
        /// {
        ///     accent-color: #3b0764;
        /// }
        /// ```
        Purple950 => "accent-purple-950",
        /// ```css
        /// {
        ///     accent-color: #fdf4ff;
        /// }
        /// ```
        Fuchsia50 => "accent-fuchsia-50",
        /// ```css
        /// {
        ///     accent-color: #fae8ff;
        /// }
        /// ```
        Fuchsia100 => "accent-fuchsia-100",
        /// ```css
        /// {
        ///     accent-color: #f5d0fe;
        /// }
        /// ```
        Fuchsia200 => "accent-fuchsia-200",
        /// ```css
        /// {
        ///     accent-color: #f0abfc;
        /// }
        /// ```
        Fuchsia300 => "accent-fuchsia-300",
        /// ```css
        /// {
        ///     accent-color: #e879f9;
        /// }
        /// ```
        Fuchsia400 => "accent-fuchsia-400",
        /// ```css
        /// {
        ///     accent-color: #d946ef;
        /// }
        /// ```
        Fuchsia500 => "accent-fuchsia-500",
        /// ```css
        /// {
        ///     accent-color: #c026d3;
        /// }
        /// ```
        Fuchsia600 => "accent-fuchsia-600",
        /// ```css
        /// {
        ///     accent-color: #a21caf;
        /// }
        /// ```
        Fuchsia700 => "accent-fuchsia-700",
        /// ```css
        /// {
        ///     accent-color: #86198f;
        /// }
        /// ```
        Fuchsia800 => "accent-fuchsia-800",
        /// ```css
        /// {
        ///     accent-color: #701a75;
        /// }
        /// ```
        Fuchsia900 => "accent-fuchsia-900",
        /// ```css
        /// {
        ///     accent-color: #4a044e;
        /// }
        /// ```
        Fuchsia950 => "accent-fuchsia-950",
        /// ```css
        /// {
        ///     accent-color: #fdf2f8;
        /// }
        /// ```
        Pink50 => "accent-pink-50",
        /// ```css
        /// {
        ///     accent-color: #fce7f3;
        /// }
        /// ```
        Pink100 => "accent-pink-100",
        /// ```css
        /// {
        ///     accent-color: #fbcfe8;
        /// }
        /// ```
        Pink200 => "accent-pink-200",
        /// ```css
        /// {
        ///     accent-color: #f9a8d4;
        /// }
        /// ```
        Pink300 => "accent-pink-300",
        /// ```css
        /// {
        ///     accent-color: #f472b6;
        /// }
        /// ```
        Pink400 => "accent-pink-400",
        /// ```css
        /// {
        ///     accent-color: #ec4899;
        /// }
        /// ```
        Pink500 => "accent-pink-500",
        /// ```css
        /// {
        ///     accent-color: #db2777;
        /// }
        /// ```
        Pink600 => "accent-pink-600",
        /// ```css
        /// {
        ///     accent-color: #be185d;
        /// }
        /// ```
        Pink700 => "accent-pink-700",
        /// ```css
        /// {
        ///     accent-color: #9d174d;
        /// }
        /// ```
        Pink800 => "accent-pink-800",
        /// ```css
        /// {
        ///     accent-color: #831843;
        /// }
        /// ```
        Pink900 => "accent-pink-900",
        /// ```css
        /// {
        ///     accent-color: #500724;
        /// }
        /// ```
        Pink950 => "accent-pink-950",
        /// ```css
        /// {
        ///     accent-color: #fff1f2;
        /// }
        /// ```
        Rose50 => "accent-rose-50",
        /// ```css
        /// {
        ///     accent-color: #ffe4e6;
        /// }
        /// ```
        Rose100 => "accent-rose-100",
        /// ```css
        /// {
        ///     accent-color: #fecdd3;
        /// }
        /// ```
        Rose200 => "accent-rose-200",
        /// ```css
        /// {
        ///     accent-color: #fda4af;
        /// }
        /// ```
        Rose300 => "accent-rose-300",
        /// ```css
        /// {
        ///     accent-color: #fb7185;
        /// }
        /// ```
        Rose400 => "accent-rose-400",
        /// ```css
        /// {
        ///     accent-color: #f43f5e;
        /// }
        /// ```
        Rose500 => "accent-rose-500",
        /// ```css
        /// {
        ///     accent-color: #e11d48;
        /// }
        /// ```
        Rose600 => "accent-rose-600",
        /// ```css
        /// {
        ///     accent-color: #be123c;
        /// }
        /// ```
        Rose700 => "accent-rose-700",
        /// ```css
        /// {
        ///     accent-color: #9f1239;
        /// }
        /// ```
        Rose800 => "accent-rose-800",
        /// ```css
        /// {
        ///     accent-color: #881337;
        /// }
        /// ```
        Rose900 => "accent-rose-900",
        /// ```css
        /// {
        ///     accent-color: #4c0519;
        /// }
        /// ```
        Rose950 => "accent-rose-950",
        /// ```css
        /// {
        ///     accent-color: auto;
        /// }
        /// ```
        Auto => "accent-auto",
    }
    /// Utilities for suppressing native form control styling.
    ///
    /// <https://tailwindcss.com/docs/appearance>
    Appearance {
        /// ```css
        /// {
        ///     appearance: none;
        /// }
        /// ```
        None => "appearance-none",
        /// ```css
        /// {
        ///     appearance: auto;
        /// }
        /// ```
        Auto => "appearance-auto",
    }
    /// Utilities for controlling the cursor style when hovering over an element.
    ///
    /// <https://tailwindcss.com/docs/cursor>
    Cursor {
        /// ```css
        /// {
        ///     cursor: auto;
        /// }
        /// ```
        Auto => "cursor-auto",
        /// ```css
        /// {
        ///     cursor: default;
        /// }
        /// ```
        Default => "cursor-default",
        /// ```css
        /// {
        ///     cursor: pointer;
        /// }
        /// ```
        Pointer => "cursor-pointer",
        /// ```css
        /// {
        ///     cursor: wait;
        /// }
        /// ```
        Wait => "cursor-wait",
        /// ```css
        /// {
        ///     cursor: text;
        /// }
        /// ```
        Text => "cursor-text",
        /// ```css
        /// {
        ///     cursor: move;
        /// }
        /// ```
        Move => "cursor-move",
        /// ```css
        /// {
        ///     cursor: help;
        /// }
        /// ```
        Help => "cursor-help",
        /// ```css
        /// {
        ///     cursor: not-allowed;
        /// }
        /// ```
        NotAllowed => "cursor-not-allowed",
        /// ```css
        /// {
        ///     cursor: none;
        /// }
        /// ```
        None => "cursor-none",
        /// ```css
        /// {
        ///     cursor: context-menu;
        /// }
        /// ```
        ContextMenu => "cursor-context-menu",
        /// ```css
        /// {
        ///     cursor: progress;
        /// }
        /// ```
        Progress => "cursor-progress",
        /// ```css
        /// {
        ///     cursor: cell;
        /// }
        /// ```
        Cell => "cursor-cell",
        /// ```css
        /// {
        ///     cursor: crosshair;
        /// }
        /// ```
        Crosshair => "cursor-crosshair",
        /// ```css
        /// {
        ///     cursor: vertical-text;
        /// }
        /// ```
        VerticalText => "cursor-vertical-text",
        /// ```css
        /// {
        ///     cursor: alias;
        /// }
        /// ```
        Alias => "cursor-alias",
        /// ```css
        /// {
        ///     cursor: copy;
        /// }
        /// ```
        Copy => "cursor-copy",
        /// ```css
        /// {
        ///     cursor: no-drop;
        /// }
        /// ```
        NoDrop => "cursor-no-drop",
        /// ```css
        /// {
        ///     cursor: grab;
        /// }
        /// ```
        Grab => "cursor-grab",
        /// ```css
        /// {
        ///     cursor: grabbing;
        /// }
        /// ```
        Grabbing => "cursor-grabbing",
        /// ```css
        /// {
        ///     cursor: all-scroll;
        /// }
        /// ```
        AllScroll => "cursor-all-scroll",
        /// ```css
        /// {
        ///     cursor: col-resize;
        /// }
        /// ```
        ColResize => "cursor-col-resize",
        /// ```css
        /// {
        ///     cursor: row-resize;
        /// }
        /// ```
        RowResize => "cursor-row-resize",
        /// ```css
        /// {
        ///     cursor: n-resize;
        /// }
        /// ```
        NResize => "cursor-n-resize",
        /// ```css
        /// {
        ///     cursor: e-resize;
        /// }
        /// ```
        EResize => "cursor-e-resize",
        /// ```css
        /// {
        ///     cursor: s-resize;
        /// }
        /// ```
        SResize => "cursor-s-resize",
        /// ```css
        /// {
        ///     cursor: w-resize;
        /// }
        /// ```
        WResize => "cursor-w-resize",
        /// ```css
        /// {
        ///     cursor: ne-resize;
        /// }
        /// ```
        NeResize => "cursor-ne-resize",
        /// ```css
        /// {
        ///     cursor: nw-resize;
        /// }
        /// ```
        NwResize => "cursor-nw-resize",
        /// ```css
        /// {
        ///     cursor: se-resize;
        /// }
        /// ```
        SeResize => "cursor-se-resize",
        /// ```css
        /// {
        ///     cursor: sw-resize;
        /// }
        /// ```
        SwResize => "cursor-sw-resize",
        /// ```css
        /// {
        ///     cursor: ew-resize;
        /// }
        /// ```
        EwResize => "cursor-ew-resize",
        /// ```css
        /// {
        ///     cursor: ns-resize;
        /// }
        /// ```
        NsResize => "cursor-ns-resize",
        /// ```css
        /// {
        ///     cursor: nesw-resize;
        /// }
        /// ```
        NeswResize => "cursor-nesw-resize",
        /// ```css
        /// {
        ///     cursor: nwse-resize;
        /// }
        /// ```
        NwseResize => "cursor-nwse-resize",
        /// ```css
        /// {
        ///     cursor: zoom-in;
        /// }
        /// ```
        ZoomIn => "cursor-zoom-in",
        /// ```css
        /// {
        ///     cursor: zoom-out;
        /// }
        /// ```
        ZoomOut => "cursor-zoom-out",
    }
    /// Utilities for controlling the color of the text input cursor.
    ///
    /// <https://tailwindcss.com/docs/caret-color>
    CaretColor {
        /// ```css
        /// {
        ///     caret-color: inherit;
        /// }
        /// ```
        Inherit => "caret-inherit",
        /// ```css
        /// {
        ///     caret-color: currentColor;
        /// }
        /// ```
        Current => "caret-current",
        /// ```css
        /// {
        ///     caret-color: transparent;
        /// }
        /// ```
        Transparent => "caret-transparent",
        /// ```css
        /// {
        ///     caret-color: #000;
        /// }
        /// ```
        Black => "caret-black",
        /// ```css
        /// {
        ///     caret-color: #fff;
        /// }
        /// ```
        White => "caret-white",
        /// ```css
        /// {
        ///     caret-color: #f8fafc;
        /// }
        /// ```
        Slate50 => "caret-slate-50",
        /// ```css
        /// {
        ///     caret-color: #f1f5f9;
        /// }
        /// ```
        Slate100 => "caret-slate-100",
        /// ```css
        /// {
        ///     caret-color: #e2e8f0;
        /// }
        /// ```
        Slate200 => "caret-slate-200",
        /// ```css
        /// {
        ///     caret-color: #cbd5e1;
        /// }
        /// ```
        Slate300 => "caret-slate-300",
        /// ```css
        /// {
        ///     caret-color: #94a3b8;
        /// }
        /// ```
        Slate400 => "caret-slate-400",
        /// ```css
        /// {
        ///     caret-color: #64748b;
        /// }
        /// ```
        Slate500 => "caret-slate-500",
        /// ```css
        /// {
        ///     caret-color: #475569;
        /// }
        /// ```
        Slate600 => "caret-slate-600",
        /// ```css
        /// {
        ///     caret-color: #334155;
        /// }
        /// ```
        Slate700 => "caret-slate-700",
        /// ```css
        /// {
        ///     caret-color: #1e293b;
        /// }
        /// ```
        Slate800 => "caret-slate-800",
        /// ```css
        /// {
        ///     caret-color: #0f172a;
        /// }
        /// ```
        Slate900 => "caret-slate-900",
        /// ```css
        /// {
        ///     caret-color: #020617;
        /// }
        /// ```
        Slate950 => "caret-slate-950",
        /// ```css
        /// {
        ///     caret-color: #f9fafb;
        /// }
        /// ```
        Gray50 => "caret-gray-50",
        /// ```css
        /// {
        ///     caret-color: #f3f4f6;
        /// }
        /// ```
        Gray100 => "caret-gray-100",
        /// ```css
        /// {
        ///     caret-color: #e5e7eb;
        /// }
        /// ```
        Gray200 => "caret-gray-200",
        /// ```css
        /// {
        ///     caret-color: #d1d5db;
        /// }
        /// ```
        Gray300 => "caret-gray-300",
        /// ```css
        /// {
        ///     caret-color: #9ca3af;
        /// }
        /// ```
        Gray400 => "caret-gray-400",
        /// ```css
        /// {
        ///     caret-color: #6b7280;
        /// }
        /// ```
        Gray500 => "caret-gray-500",
        /// ```css
        /// {
        ///     caret-color: #4b5563;
        /// }
        /// ```
        Gray600 => "caret-gray-600",
        /// ```css
        /// {
        ///     caret-color: #374151;
        /// }
        /// ```
        Gray700 => "caret-gray-700",
        /// ```css
        /// {
        ///     caret-color: #1f2937;
        /// }
        /// ```
        Gray800 => "caret-gray-800",
        /// ```css
        /// {
        ///     caret-color: #111827;
        /// }
        /// ```
        Gray900 => "caret-gray-900",
        /// ```css
        /// {
        ///     caret-color: #030712;
        /// }
        /// ```
        Gray950 => "caret-gray-950",
        /// ```css
        /// {
        ///     caret-color: #fafafa;
        /// }
        /// ```
        Zinc50 => "caret-zinc-50",
        /// ```css
        /// {
        ///     caret-color: #f4f4f5;
        /// }
        /// ```
        Zinc100 => "caret-zinc-100",
        /// ```css
        /// {
        ///     caret-color: #e4e4e7;
        /// }
        /// ```
        Zinc200 => "caret-zinc-200",
        /// ```css
        /// {
        ///     caret-color: #d4d4d8;
        /// }
        /// ```
        Zinc300 => "caret-zinc-300",
        /// ```css
        /// {
        ///     caret-color: #a1a1aa;
        /// }
        /// ```
        Zinc400 => "caret-zinc-400",
        /// ```css
        /// {
        ///     caret-color: #71717a;
        /// }
        /// ```
        Zinc500 => "caret-zinc-500",
        /// ```css
        /// {
        ///     caret-color: #52525b;
        /// }
        /// ```
        Zinc600 => "caret-zinc-600",
        /// ```css
        /// {
        ///     caret-color: #3f3f46;
        /// }
        /// ```
        Zinc700 => "caret-zinc-700",
        /// ```css
        /// {
        ///     caret-color: #27272a;
        /// }
        /// ```
        Zinc800 => "caret-zinc-800",
        /// ```css
        /// {
        ///     caret-color: #18181b;
        /// }
        /// ```
        Zinc900 => "caret-zinc-900",
        /// ```css
        /// {
        ///     caret-color: #09090b;
        /// }
        /// ```
        Zinc950 => "caret-zinc-950",
        /// ```css
        /// {
        ///     caret-color: #fafafa;
        /// }
        /// ```
        Neutral50 => "caret-neutral-50",
        /// ```css
        /// {
        ///     caret-color: #f5f5f5;
        /// }
        /// ```
        Neutral100 => "caret-neutral-100",
        /// ```css
        /// {
        ///     caret-color: #e5e5e5;
        /// }
        /// ```
        Neutral200 => "caret-neutral-200",
        /// ```css
        /// {
        ///     caret-color: #d4d4d4;
        /// }
        /// ```
        Neutral300 => "caret-neutral-300",
        /// ```css
        /// {
        ///     caret-color: #a3a3a3;
        /// }
        /// ```
        Neutral400 => "caret-neutral-400",
        /// ```css
        /// {
        ///     caret-color: #737373;
        /// }
        /// ```
        Neutral500 => "caret-neutral-500",
        /// ```css
        /// {
        ///     caret-color: #525252;
        /// }
        /// ```
        Neutral600 => "caret-neutral-600",
        /// ```css
        /// {
        ///     caret-color: #404040;
        /// }
        /// ```
        Neutral700 => "caret-neutral-700",
        /// ```css
        /// {
        ///     caret-color: #262626;
        /// }
        /// ```
        Neutral800 => "caret-neutral-800",
        /// ```css
        /// {
        ///     caret-color: #171717;
        /// }
        /// ```
        Neutral900 => "caret-neutral-900",
        /// ```css
        /// {
        ///     caret-color: #0a0a0a;
        /// }
        /// ```
        Neutral950 => "caret-neutral-950",
        /// ```css
        /// {
        ///     caret-color: #fafaf9;
        /// }
        /// ```
        Stone50 => "caret-stone-50",
        /// ```css
        /// {
        ///     caret-color: #f5f5f4;
        /// }
        /// ```
        Stone100 => "caret-stone-100",
        /// ```css
        /// {
        ///     caret-color: #e7e5e4;
        /// }
        /// ```
        Stone200 => "caret-stone-200",
        /// ```css
        /// {
        ///     caret-color: #d6d3d1;
        /// }
        /// ```
        Stone300 => "caret-stone-300",
        /// ```css
        /// {
        ///     caret-color: #a8a29e;
        /// }
        /// ```
        Stone400 => "caret-stone-400",
        /// ```css
        /// {
        ///     caret-color: #78716c;
        /// }
        /// ```
        Stone500 => "caret-stone-500",
        /// ```css
        /// {
        ///     caret-color: #57534e;
        /// }
        /// ```
        Stone600 => "caret-stone-600",
        /// ```css
        /// {
        ///     caret-color: #44403c;
        /// }
        /// ```
        Stone700 => "caret-stone-700",
        /// ```css
        /// {
        ///     caret-color: #292524;
        /// }
        /// ```
        Stone800 => "caret-stone-800",
        /// ```css
        /// {
        ///     caret-color: #1c1917;
        /// }
        /// ```
        Stone900 => "caret-stone-900",
        /// ```css
        /// {
        ///     caret-color: #0c0a09;
        /// }
        /// ```
        Stone950 => "caret-stone-950",
        /// ```css
        /// {
        ///     caret-color: #fef2f2;
        /// }
        /// ```
        Red50 => "caret-red-50",
        /// ```css
        /// {
        ///     caret-color: #fee2e2;
        /// }
        /// ```
        Red100 => "caret-red-100",
        /// ```css
        /// {
        ///     caret-color: #fecaca;
        /// }
        /// ```
        Red200 => "caret-red-200",
        /// ```css
        /// {
        ///     caret-color: #fca5a5;
        /// }
        /// ```
        Red300 => "caret-red-300",
        /// ```css
        /// {
        ///     caret-color: #f87171;
        /// }
        /// ```
        Red400 => "caret-red-400",
        /// ```css
        /// {
        ///     caret-color: #ef4444;
        /// }
        /// ```
        Red500 => "caret-red-500",
        /// ```css
        /// {
        ///     caret-color: #dc2626;
        /// }
        /// ```
        Red600 => "caret-red-600",
        /// ```css
        /// {
        ///     caret-color: #b91c1c;
        /// }
        /// ```
        Red700 => "caret-red-700",
        /// ```css
        /// {
        ///     caret-color: #991b1b;
        /// }
        /// ```
        Red800 => "caret-red-800",
        /// ```css
        /// {
        ///     caret-color: #7f1d1d;
        /// }
        /// ```
        Red900 => "caret-red-900",
        /// ```css
        /// {
        ///     caret-color: #450a0a;
        /// }
        /// ```
        Red950 => "caret-red-950",
        /// ```css
        /// {
        ///     caret-color: #fff7ed;
        /// }
        /// ```
        Orange50 => "caret-orange-50",
        /// ```css
        /// {
        ///     caret-color: #ffedd5;
        /// }
        /// ```
        Orange100 => "caret-orange-100",
        /// ```css
        /// {
        ///     caret-color: #fed7aa;
        /// }
        /// ```
        Orange200 => "caret-orange-200",
        /// ```css
        /// {
        ///     caret-color: #fdba74;
        /// }
        /// ```
        Orange300 => "caret-orange-300",
        /// ```css
        /// {
        ///     caret-color: #fb923c;
        /// }
        /// ```
        Orange400 => "caret-orange-400",
        /// ```css
        /// {
        ///     caret-color: #f97316;
        /// }
        /// ```
        Orange500 => "caret-orange-500",
        /// ```css
        /// {
        ///     caret-color: #ea580c;
        /// }
        /// ```
        Orange600 => "caret-orange-600",
        /// ```css
        /// {
        ///     caret-color: #c2410c;
        /// }
        /// ```
        Orange700 => "caret-orange-700",
        /// ```css
        /// {
        ///     caret-color: #9a3412;
        /// }
        /// ```
        Orange800 => "caret-orange-800",
        /// ```css
        /// {
        ///     caret-color: #7c2d12;
        /// }
        /// ```
        Orange900 => "caret-orange-900",
        /// ```css
        /// {
        ///     caret-color: #431407;
        /// }
        /// ```
        Orange950 => "caret-orange-950",
        /// ```css
        /// {
        ///     caret-color: #fffbeb;
        /// }
        /// ```
        Amber50 => "caret-amber-50",
        /// ```css
        /// {
        ///     caret-color: #fef3c7;
        /// }
        /// ```
        Amber100 => "caret-amber-100",
        /// ```css
        /// {
        ///     caret-color: #fde68a;
        /// }
        /// ```
        Amber200 => "caret-amber-200",
        /// ```css
        /// {
        ///     caret-color: #fcd34d;
        /// }
        /// ```
        Amber300 => "caret-amber-300",
        /// ```css
        /// {
        ///     caret-color: #fbbf24;
        /// }
        /// ```
        Amber400 => "caret-amber-400",
        /// ```css
        /// {
        ///     caret-color: #f59e0b;
        /// }
        /// ```
        Amber500 => "caret-amber-500",
        /// ```css
        /// {
        ///     caret-color: #d97706;
        /// }
        /// ```
        Amber600 => "caret-amber-600",
        /// ```css
        /// {
        ///     caret-color: #b45309;
        /// }
        /// ```
        Amber700 => "caret-amber-700",
        /// ```css
        /// {
        ///     caret-color: #92400e;
        /// }
        /// ```
        Amber800 => "caret-amber-800",
        /// ```css
        /// {
        ///     caret-color: #78350f;
        /// }
        /// ```
        Amber900 => "caret-amber-900",
        /// ```css
        /// {
        ///     caret-color: #451a03;
        /// }
        /// ```
        Amber950 => "caret-amber-950",
        /// ```css
        /// {
        ///     caret-color: #fefce8;
        /// }
        /// ```
        Yellow50 => "caret-yellow-50",
        /// ```css
        /// {
        ///     caret-color: #fef9c3;
        /// }
        /// ```
        Yellow100 => "caret-yellow-100",
        /// ```css
        /// {
        ///     caret-color: #fef08a;
        /// }
        /// ```
        Yellow200 => "caret-yellow-200",
        /// ```css
        /// {
        ///     caret-color: #fde047;
        /// }
        /// ```
        Yellow300 => "caret-yellow-300",
        /// ```css
        /// {
        ///     caret-color: #facc15;
        /// }
        /// ```
        Yellow400 => "caret-yellow-400",
        /// ```css
        /// {
        ///     caret-color: #eab308;
        /// }
        /// ```
        Yellow500 => "caret-yellow-500",
        /// ```css
        /// {
        ///     caret-color: #ca8a04;
        /// }
        /// ```
        Yellow600 => "caret-yellow-600",
        /// ```css
        /// {
        ///     caret-color: #a16207;
        /// }
        /// ```
        Yellow700 => "caret-yellow-700",
        /// ```css
        /// {
        ///     caret-color: #854d0e;
        /// }
        /// ```
        Yellow800 => "caret-yellow-800",
        /// ```css
        /// {
        ///     caret-color: #713f12;
        /// }
        /// ```
        Yellow900 => "caret-yellow-900",
        /// ```css
        /// {
        ///     caret-color: #422006;
        /// }
        /// ```
        Yellow950 => "caret-yellow-950",
        /// ```css
        /// {
        ///     caret-color: #f7fee7;
        /// }
        /// ```
        Lime50 => "caret-lime-50",
        /// ```css
        /// {
        ///     caret-color: #ecfccb;
        /// }
        /// ```
        Lime100 => "caret-lime-100",
        /// ```css
        /// {
        ///     caret-color: #d9f99d;
        /// }
        /// ```
        Lime200 => "caret-lime-200",
        /// ```css
        /// {
        ///     caret-color: #bef264;
        /// }
        /// ```
        Lime300 => "caret-lime-300",
        /// ```css
        /// {
        ///     caret-color: #a3e635;
        /// }
        /// ```
        Lime400 => "caret-lime-400",
        /// ```css
        /// {
        ///     caret-color: #84cc16;
        /// }
        /// ```
        Lime500 => "caret-lime-500",
        /// ```css
        /// {
        ///     caret-color: #65a30d;
        /// }
        /// ```
        Lime600 => "caret-lime-600",
        /// ```css
        /// {
        ///     caret-color: #4d7c0f;
        /// }
        /// ```
        Lime700 => "caret-lime-700",
        /// ```css
        /// {
        ///     caret-color: #3f6212;
        /// }
        /// ```
        Lime800 => "caret-lime-800",
        /// ```css
        /// {
        ///     caret-color: #365314;
        /// }
        /// ```
        Lime900 => "caret-lime-900",
        /// ```css
        /// {
        ///     caret-color: #1a2e05;
        /// }
        /// ```
        Lime950 => "caret-lime-950",
        /// ```css
        /// {
        ///     caret-color: #f0fdf4;
        /// }
        /// ```
        Green50 => "caret-green-50",
        /// ```css
        /// {
        ///     caret-color: #dcfce7;
        /// }
        /// ```
        Green100 => "caret-green-100",
        /// ```css
        /// {
        ///     caret-color: #bbf7d0;
        /// }
        /// ```
        Green200 => "caret-green-200",
        /// ```css
        /// {
        ///     caret-color: #86efac;
        /// }
        /// ```
        Green300 => "caret-green-300",
        /// ```css
        /// {
        ///     caret-color: #4ade80;
        /// }
        /// ```
        Green400 => "caret-green-400",
        /// ```css
        /// {
        ///     caret-color: #22c55e;
        /// }
        /// ```
        Green500 => "caret-green-500",
        /// ```css
        /// {
        ///     caret-color: #16a34a;
        /// }
        /// ```
        Green600 => "caret-green-600",
        /// ```css
        /// {
        ///     caret-color: #15803d;
        /// }
        /// ```
        Green700 => "caret-green-700",
        /// ```css
        /// {
        ///     caret-color: #166534;
        /// }
        /// ```
        Green800 => "caret-green-800",
        /// ```css
        /// {
        ///     caret-color: #14532d;
        /// }
        /// ```
        Green900 => "caret-green-900",
        /// ```css
        /// {
        ///     caret-color: #052e16;
        /// }
        /// ```
        Green950 => "caret-green-950",
        /// ```css
        /// {
        ///     caret-color: #ecfdf5;
        /// }
        /// ```
        Emerald50 => "caret-emerald-50",
        /// ```css
        /// {
        ///     caret-color: #d1fae5;
        /// }
        /// ```
        Emerald100 => "caret-emerald-100",
        /// ```css
        /// {
        ///     caret-color: #a7f3d0;
        /// }
        /// ```
        Emerald200 => "caret-emerald-200",
        /// ```css
        /// {
        ///     caret-color: #6ee7b7;
        /// }
        /// ```
        Emerald300 => "caret-emerald-300",
        /// ```css
        /// {
        ///     caret-color: #34d399;
        /// }
        /// ```
        Emerald400 => "caret-emerald-400",
        /// ```css
        /// {
        ///     caret-color: #10b981;
        /// }
        /// ```
        Emerald500 => "caret-emerald-500",
        /// ```css
        /// {
        ///     caret-color: #059669;
        /// }
        /// ```
        Emerald600 => "caret-emerald-600",
        /// ```css
        /// {
        ///     caret-color: #047857;
        /// }
        /// ```
        Emerald700 => "caret-emerald-700",
        /// ```css
        /// {
        ///     caret-color: #065f46;
        /// }
        /// ```
        Emerald800 => "caret-emerald-800",
        /// ```css
        /// {
        ///     caret-color: #064e3b;
        /// }
        /// ```
        Emerald900 => "caret-emerald-900",
        /// ```css
        /// {
        ///     caret-color: #022c22;
        /// }
        /// ```
        Emerald950 => "caret-emerald-950",
        /// ```css
        /// {
        ///     caret-color: #f0fdfa;
        /// }
        /// ```
        Teal50 => "caret-teal-50",
        /// ```css
        /// {
        ///     caret-color: #ccfbf1;
        /// }
        /// ```
        Teal100 => "caret-teal-100",
        /// ```css
        /// {
        ///     caret-color: #99f6e4;
        /// }
        /// ```
        Teal200 => "caret-teal-200",
        /// ```css
        /// {
        ///     caret-color: #5eead4;
        /// }
        /// ```
        Teal300 => "caret-teal-300",
        /// ```css
        /// {
        ///     caret-color: #2dd4bf;
        /// }
        /// ```
        Teal400 => "caret-teal-400",
        /// ```css
        /// {
        ///     caret-color: #14b8a6;
        /// }
        /// ```
        Teal500 => "caret-teal-500",
        /// ```css
        /// {
        ///     caret-color: #0d9488;
        /// }
        /// ```
        Teal600 => "caret-teal-600",
        /// ```css
        /// {
        ///     caret-color: #0f766e;
        /// }
        /// ```
        Teal700 => "caret-teal-700",
        /// ```css
        /// {
        ///     caret-color: #115e59;
        /// }
        /// ```
        Teal800 => "caret-teal-800",
        /// ```css
        /// {
        ///     caret-color: #134e4a;
        /// }
        /// ```
        Teal900 => "caret-teal-900",
        /// ```css
        /// {
        ///     caret-color: #042f2e;
        /// }
        /// ```
        Teal950 => "caret-teal-950",
        /// ```css
        /// {
        ///     caret-color: #ecfeff;
        /// }
        /// ```
        Cyan50 => "caret-cyan-50",
        /// ```css
        /// {
        ///     caret-color: #cffafe;
        /// }
        /// ```
        Cyan100 => "caret-cyan-100",
        /// ```css
        /// {
        ///     caret-color: #a5f3fc;
        /// }
        /// ```
        Cyan200 => "caret-cyan-200",
        /// ```css
        /// {
        ///     caret-color: #67e8f9;
        /// }
        /// ```
        Cyan300 => "caret-cyan-300",
        /// ```css
        /// {
        ///     caret-color: #22d3ee;
        /// }
        /// ```
        Cyan400 => "caret-cyan-400",
        /// ```css
        /// {
        ///     caret-color: #06b6d4;
        /// }
        /// ```
        Cyan500 => "caret-cyan-500",
        /// ```css
        /// {
        ///     caret-color: #0891b2;
        /// }
        /// ```
        Cyan600 => "caret-cyan-600",
        /// ```css
        /// {
        ///     caret-color: #0e7490;
        /// }
        /// ```
        Cyan700 => "caret-cyan-700",
        /// ```css
        /// {
        ///     caret-color: #155e75;
        /// }
        /// ```
        Cyan800 => "caret-cyan-800",
        /// ```css
        /// {
        ///     caret-color: #164e63;
        /// }
        /// ```
        Cyan900 => "caret-cyan-900",
        /// ```css
        /// {
        ///     caret-color: #083344;
        /// }
        /// ```
        Cyan950 => "caret-cyan-950",
        /// ```css
        /// {
        ///     caret-color: #f0f9ff;
        /// }
        /// ```
        Sky50 => "caret-sky-50",
        /// ```css
        /// {
        ///     caret-color: #e0f2fe;
        /// }
        /// ```
        Sky100 => "caret-sky-100",
        /// ```css
        /// {
        ///     caret-color: #bae6fd;
        /// }
        /// ```
        Sky200 => "caret-sky-200",
        /// ```css
        /// {
        ///     caret-color: #7dd3fc;
        /// }
        /// ```
        Sky300 => "caret-sky-300",
        /// ```css
        /// {
        ///     caret-color: #38bdf8;
        /// }
        /// ```
        Sky400 => "caret-sky-400",
        /// ```css
        /// {
        ///     caret-color: #0ea5e9;
        /// }
        /// ```
        Sky500 => "caret-sky-500",
        /// ```css
        /// {
        ///     caret-color: #0284c7;
        /// }
        /// ```
        Sky600 => "caret-sky-600",
        /// ```css
        /// {
        ///     caret-color: #0369a1;
        /// }
        /// ```
        Sky700 => "caret-sky-700",
        /// ```css
        /// {
        ///     caret-color: #075985;
        /// }
        /// ```
        Sky800 => "caret-sky-800",
        /// ```css
        /// {
        ///     caret-color: #0c4a6e;
        /// }
        /// ```
        Sky900 => "caret-sky-900",
        /// ```css
        /// {
        ///     caret-color: #082f49;
        /// }
        /// ```
        Sky950 => "caret-sky-950",
        /// ```css
        /// {
        ///     caret-color: #eff6ff;
        /// }
        /// ```
        Blue50 => "caret-blue-50",
        /// ```css
        /// {
        ///     caret-color: #dbeafe;
        /// }
        /// ```
        Blue100 => "caret-blue-100",
        /// ```css
        /// {
        ///     caret-color: #bfdbfe;
        /// }
        /// ```
        Blue200 => "caret-blue-200",
        /// ```css
        /// {
        ///     caret-color: #93c5fd;
        /// }
        /// ```
        Blue300 => "caret-blue-300",
        /// ```css
        /// {
        ///     caret-color: #60a5fa;
        /// }
        /// ```
        Blue400 => "caret-blue-400",
        /// ```css
        /// {
        ///     caret-color: #3b82f6;
        /// }
        /// ```
        Blue500 => "caret-blue-500",
        /// ```css
        /// {
        ///     caret-color: #2563eb;
        /// }
        /// ```
        Blue600 => "caret-blue-600",
        /// ```css
        /// {
        ///     caret-color: #1d4ed8;
        /// }
        /// ```
        Blue700 => "caret-blue-700",
        /// ```css
        /// {
        ///     caret-color: #1e40af;
        /// }
        /// ```
        Blue800 => "caret-blue-800",
        /// ```css
        /// {
        ///     caret-color: #1e3a8a;
        /// }
        /// ```
        Blue900 => "caret-blue-900",
        /// ```css
        /// {
        ///     caret-color: #172554;
        /// }
        /// ```
        Blue950 => "caret-blue-950",
        /// ```css
        /// {
        ///     caret-color: #eef2ff;
        /// }
        /// ```
        Indigo50 => "caret-indigo-50",
        /// ```css
        /// {
        ///     caret-color: #e0e7ff;
        /// }
        /// ```
        Indigo100 => "caret-indigo-100",
        /// ```css
        /// {
        ///     caret-color: #c7d2fe;
        /// }
        /// ```
        Indigo200 => "caret-indigo-200",
        /// ```css
        /// {
        ///     caret-color: #a5b4fc;
        /// }
        /// ```
        Indigo300 => "caret-indigo-300",
        /// ```css
        /// {
        ///     caret-color: #818cf8;
        /// }
        /// ```
        Indigo400 => "caret-indigo-400",
        /// ```css
        /// {
        ///     caret-color: #6366f1;
        /// }
        /// ```
        Indigo500 => "caret-indigo-500",
        /// ```css
        /// {
        ///     caret-color: #4f46e5;
        /// }
        /// ```
        Indigo600 => "caret-indigo-600",
        /// ```css
        /// {
        ///     caret-color: #4338ca;
        /// }
        /// ```
        Indigo700 => "caret-indigo-700",
        /// ```css
        /// {
        ///     caret-color: #3730a3;
        /// }
        /// ```
        Indigo800 => "caret-indigo-800",
        /// ```css
        /// {
        ///     caret-color: #312e81;
        /// }
        /// ```
        Indigo900 => "caret-indigo-900",
        /// ```css
        /// {
        ///     caret-color: #1e1b4b;
        /// }
        /// ```
        Indigo950 => "caret-indigo-950",
        /// ```css
        /// {
        ///     caret-color: #f5f3ff;
        /// }
        /// ```
        Violet50 => "caret-violet-50",
        /// ```css
        /// {
        ///     caret-color: #ede9fe;
        /// }
        /// ```
        Violet100 => "caret-violet-100",
        /// ```css
        /// {
        ///     caret-color: #ddd6fe;
        /// }
        /// ```
        Violet200 => "caret-violet-200",
        /// ```css
        /// {
        ///     caret-color: #c4b5fd;
        /// }
        /// ```
        Violet300 => "caret-violet-300",
        /// ```css
        /// {
        ///     caret-color: #a78bfa;
        /// }
        /// ```
        Violet400 => "caret-violet-400",
        /// ```css
        /// {
        ///     caret-color: #8b5cf6;
        /// }
        /// ```
        Violet500 => "caret-violet-500",
        /// ```css
        /// {
        ///     caret-color: #7c3aed;
        /// }
        /// ```
        Violet600 => "caret-violet-600",
        /// ```css
        /// {
        ///     caret-color: #6d28d9;
        /// }
        /// ```
        Violet700 => "caret-violet-700",
        /// ```css
        /// {
        ///     caret-color: #5b21b6;
        /// }
        /// ```
        Violet800 => "caret-violet-800",
        /// ```css
        /// {
        ///     caret-color: #4c1d95;
        /// }
        /// ```
        Violet900 => "caret-violet-900",
        /// ```css
        /// {
        ///     caret-color: #2e1065;
        /// }
        /// ```
        Violet950 => "caret-violet-950",
        /// ```css
        /// {
        ///     caret-color: #faf5ff;
        /// }
        /// ```
        Purple50 => "caret-purple-50",
        /// ```css
        /// {
        ///     caret-color: #f3e8ff;
        /// }
        /// ```
        Purple100 => "caret-purple-100",
        /// ```css
        /// {
        ///     caret-color: #e9d5ff;
        /// }
        /// ```
        Purple200 => "caret-purple-200",
        /// ```css
        /// {
        ///     caret-color: #d8b4fe;
        /// }
        /// ```
        Purple300 => "caret-purple-300",
        /// ```css
        /// {
        ///     caret-color: #c084fc;
        /// }
        /// ```
        Purple400 => "caret-purple-400",
        /// ```css
        /// {
        ///     caret-color: #a855f7;
        /// }
        /// ```
        Purple500 => "caret-purple-500",
        /// ```css
        /// {
        ///     caret-color: #9333ea;
        /// }
        /// ```
        Purple600 => "caret-purple-600",
        /// ```css
        /// {
        ///     caret-color: #7e22ce;
        /// }
        /// ```
        Purple700 => "caret-purple-700",
        /// ```css
        /// {
        ///     caret-color: #6b21a8;
        /// }
        /// ```
        Purple800 => "caret-purple-800",
        /// ```css
        /// {
        ///     caret-color: #581c87;
        /// }
        /// ```
        Purple900 => "caret-purple-900",
        /// ```css
        /// {
        ///     caret-color: #3b0764;
        /// }
        /// ```
        Purple950 => "caret-purple-950",
        /// ```css
        /// {
        ///     caret-color: #fdf4ff;
        /// }
        /// ```
        Fuchsia50 => "caret-fuchsia-50",
        /// ```css
        /// {
        ///     caret-color: #fae8ff;
        /// }
        /// ```
        Fuchsia100 => "caret-fuchsia-100",
        /// ```css
        /// {
        ///     caret-color: #f5d0fe;
        /// }
        /// ```
        Fuchsia200 => "caret-fuchsia-200",
        /// ```css
        /// {
        ///     caret-color: #f0abfc;
        /// }
        /// ```
        Fuchsia300 => "caret-fuchsia-300",
        /// ```css
        /// {
        ///     caret-color: #e879f9;
        /// }
        /// ```
        Fuchsia400 => "caret-fuchsia-400",
        /// ```css
        /// {
        ///     caret-color: #d946ef;
        /// }
        /// ```
        Fuchsia500 => "caret-fuchsia-500",
        /// ```css
        /// {
        ///     caret-color: #c026d3;
        /// }
        /// ```
        Fuchsia600 => "caret-fuchsia-600",
        /// ```css
        /// {
        ///     caret-color: #a21caf;
        /// }
        /// ```
        Fuchsia700 => "caret-fuchsia-700",
        /// ```css
        /// {
        ///     caret-color: #86198f;
        /// }
        /// ```
        Fuchsia800 => "caret-fuchsia-800",
        /// ```css
        /// {
        ///     caret-color: #701a75;
        /// }
        /// ```
        Fuchsia900 => "caret-fuchsia-900",
        /// ```css
        /// {
        ///     caret-color: #4a044e;
        /// }
        /// ```
        Fuchsia950 => "caret-fuchsia-950",
        /// ```css
        /// {
        ///     caret-color: #fdf2f8;
        /// }
        /// ```
        Pink50 => "caret-pink-50",
        /// ```css
        /// {
        ///     caret-color: #fce7f3;
        /// }
        /// ```
        Pink100 => "caret-pink-100",
        /// ```css
        /// {
        ///     caret-color: #fbcfe8;
        /// }
        /// ```
        Pink200 => "caret-pink-200",
        /// ```css
        /// {
        ///     caret-color: #f9a8d4;
        /// }
        /// ```
        Pink300 => "caret-pink-300",
        /// ```css
        /// {
        ///     caret-color: #f472b6;
        /// }
        /// ```
        Pink400 => "caret-pink-400",
        /// ```css
        /// {
        ///     caret-color: #ec4899;
        /// }
        /// ```
        Pink500 => "caret-pink-500",
        /// ```css
        /// {
        ///     caret-color: #db2777;
        /// }
        /// ```
        Pink600 => "caret-pink-600",
        /// ```css
        /// {
        ///     caret-color: #be185d;
        /// }
        /// ```
        Pink700 => "caret-pink-700",
        /// ```css
        /// {
        ///     caret-color: #9d174d;
        /// }
        /// ```
        Pink800 => "caret-pink-800",
        /// ```css
        /// {
        ///     caret-color: #831843;
        /// }
        /// ```
        Pink900 => "caret-pink-900",
        /// ```css
        /// {
        ///     caret-color: #500724;
        /// }
        /// ```
        Pink950 => "caret-pink-950",
        /// ```css
        /// {
        ///     caret-color: #fff1f2;
        /// }
        /// ```
        Rose50 => "caret-rose-50",
        /// ```css
        /// {
        ///     caret-color: #ffe4e6;
        /// }
        /// ```
        Rose100 => "caret-rose-100",
        /// ```css
        /// {
        ///     caret-color: #fecdd3;
        /// }
        /// ```
        Rose200 => "caret-rose-200",
        /// ```css
        /// {
        ///     caret-color: #fda4af;
        /// }
        /// ```
        Rose300 => "caret-rose-300",
        /// ```css
        /// {
        ///     caret-color: #fb7185;
        /// }
        /// ```
        Rose400 => "caret-rose-400",
        /// ```css
        /// {
        ///     caret-color: #f43f5e;
        /// }
        /// ```
        Rose500 => "caret-rose-500",
        /// ```css
        /// {
        ///     caret-color: #e11d48;
        /// }
        /// ```
        Rose600 => "caret-rose-600",
        /// ```css
        /// {
        ///     caret-color: #be123c;
        /// }
        /// ```
        Rose700 => "caret-rose-700",
        /// ```css
        /// {
        ///     caret-color: #9f1239;
        /// }
        /// ```
        Rose800 => "caret-rose-800",
        /// ```css
        /// {
        ///     caret-color: #881337;
        /// }
        /// ```
        Rose900 => "caret-rose-900",
        /// ```css
        /// {
        ///     caret-color: #4c0519;
        /// }
        /// ```
        Rose950 => "caret-rose-950",
    }
    /// Utilities for controlling whether an element responds to pointer events.
    ///
    /// <https://tailwindcss.com/docs/pointer-events>
    PointerEvents {
        /// ```css
        /// {
        ///     pointer-events: none;
        /// }
        /// ```
        None => "pointer-events-none",
        /// ```css
        /// {
        ///     pointer-events: auto;
        /// }
        /// ```
        Auto => "pointer-events-auto",
    }
    /// Utilities for controlling how an element can be resized.
    ///
    /// <https://tailwindcss.com/docs/resize>
    Resize {
        /// ```css
        /// {
        ///     resize: none;
        /// }
        /// ```
        None => "resize-none",
        /// ```css
        /// {
        ///     resize: vertical;
        /// }
        /// ```
        Y => "resize-y",
        /// ```css
        /// {
        ///     resize: horizontal;
        /// }
        /// ```
        X => "resize-x",
        /// ```css
        /// {
        ///     resize: both;
        /// }
        /// ```
        Resize => "resize",
    }
    /// Utilities for controlling the scroll behavior of an element.
    ///
    /// <https://tailwindcss.com/docs/scroll-behavior>
    ScrollBehavior {
        /// ```css
        /// {
        ///     scroll-behavior: auto;
        /// }
        /// ```
        Auto => "scroll-auto",
        /// ```css
        /// {
        ///     scroll-behavior: smooth;
        /// }
        /// ```
        Smooth => "scroll-smooth",
    }
    /// Utilities for controlling the scroll offset around items in a snap container.
    ///
    /// <https://tailwindcss.com/docs/scroll-margin>
    ScrollMargin {
        /// ```css
        /// {
        ///     scroll-margin: 0px;
        /// }
        /// ```
        M0 => "scroll-m-0",
        /// ```css
        /// {
        ///     scroll-margin-left: 0px;
        ///     scroll-margin-right: 0px;
        /// }
        /// ```
        Mx0 => "scroll-mx-0",
        /// ```css
        /// {
        ///     scroll-margin-top: 0px;
        ///     scroll-margin-bottom: 0px;
        /// }
        /// ```
        My0 => "scroll-my-0",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 0px;
        /// }
        /// ```
        Ms0 => "scroll-ms-0",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 0px;
        /// }
        /// ```
        Me0 => "scroll-me-0",
        /// ```css
        /// {
        ///     scroll-margin-top: 0px;
        /// }
        /// ```
        Mt0 => "scroll-mt-0",
        /// ```css
        /// {
        ///     scroll-margin-right: 0px;
        /// }
        /// ```
        Mr0 => "scroll-mr-0",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 0px;
        /// }
        /// ```
        Mb0 => "scroll-mb-0",
        /// ```css
        /// {
        ///     scroll-margin-left: 0px;
        /// }
        /// ```
        Ml0 => "scroll-ml-0",
        /// ```css
        /// {
        ///     scroll-margin: 1px;
        /// }
        /// ```
        MPx => "scroll-m-px",
        /// ```css
        /// {
        ///     scroll-margin-left: 1px;
        ///     scroll-margin-right: 1px;
        /// }
        /// ```
        MxPx => "scroll-mx-px",
        /// ```css
        /// {
        ///     scroll-margin-top: 1px;
        ///     scroll-margin-bottom: 1px;
        /// }
        /// ```
        MyPx => "scroll-my-px",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 1px;
        /// }
        /// ```
        MsPx => "scroll-ms-px",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 1px;
        /// }
        /// ```
        MePx => "scroll-me-px",
        /// ```css
        /// {
        ///     scroll-margin-top: 1px;
        /// }
        /// ```
        MtPx => "scroll-mt-px",
        /// ```css
        /// {
        ///     scroll-margin-right: 1px;
        /// }
        /// ```
        MrPx => "scroll-mr-px",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 1px;
        /// }
        /// ```
        MbPx => "scroll-mb-px",
        /// ```css
        /// {
        ///     scroll-margin-left: 1px;
        /// }
        /// ```
        MlPx => "scroll-ml-px",
        /// ```css
        /// {
        ///     scroll-margin: 0.125rem; /* 2px */
        /// }
        /// ```
        M0_5 => "scroll-m-0.5",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.125rem; /* 2px */
        ///     scroll-margin-right: 0.125rem; /* 2px */
        /// }
        /// ```
        Mx0_5 => "scroll-mx-0.5",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.125rem; /* 2px */
        ///     scroll-margin-bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        My0_5 => "scroll-my-0.5",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 0.125rem; /* 2px */
        /// }
        /// ```
        Ms0_5 => "scroll-ms-0.5",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 0.125rem; /* 2px */
        /// }
        /// ```
        Me0_5 => "scroll-me-0.5",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.125rem; /* 2px */
        /// }
        /// ```
        Mt0_5 => "scroll-mt-0.5",
        /// ```css
        /// {
        ///     scroll-margin-right: 0.125rem; /* 2px */
        /// }
        /// ```
        Mr0_5 => "scroll-mr-0.5",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        Mb0_5 => "scroll-mb-0.5",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.125rem; /* 2px */
        /// }
        /// ```
        Ml0_5 => "scroll-ml-0.5",
        /// ```css
        /// {
        ///     scroll-margin: 0.25rem; /* 4px */
        /// }
        /// ```
        M1 => "scroll-m-1",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.25rem; /* 4px */
        ///     scroll-margin-right: 0.25rem; /* 4px */
        /// }
        /// ```
        Mx1 => "scroll-mx-1",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.25rem; /* 4px */
        ///     scroll-margin-bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        My1 => "scroll-my-1",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 0.25rem; /* 4px */
        /// }
        /// ```
        Ms1 => "scroll-ms-1",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 0.25rem; /* 4px */
        /// }
        /// ```
        Me1 => "scroll-me-1",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.25rem; /* 4px */
        /// }
        /// ```
        Mt1 => "scroll-mt-1",
        /// ```css
        /// {
        ///     scroll-margin-right: 0.25rem; /* 4px */
        /// }
        /// ```
        Mr1 => "scroll-mr-1",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        Mb1 => "scroll-mb-1",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.25rem; /* 4px */
        /// }
        /// ```
        Ml1 => "scroll-ml-1",
        /// ```css
        /// {
        ///     scroll-margin: 0.375rem; /* 6px */
        /// }
        /// ```
        M1_5 => "scroll-m-1.5",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.375rem; /* 6px */
        ///     scroll-margin-right: 0.375rem; /* 6px */
        /// }
        /// ```
        Mx1_5 => "scroll-mx-1.5",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.375rem; /* 6px */
        ///     scroll-margin-bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        My1_5 => "scroll-my-1.5",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 0.375rem; /* 6px */
        /// }
        /// ```
        Ms1_5 => "scroll-ms-1.5",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 0.375rem; /* 6px */
        /// }
        /// ```
        Me1_5 => "scroll-me-1.5",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.375rem; /* 6px */
        /// }
        /// ```
        Mt1_5 => "scroll-mt-1.5",
        /// ```css
        /// {
        ///     scroll-margin-right: 0.375rem; /* 6px */
        /// }
        /// ```
        Mr1_5 => "scroll-mr-1.5",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        Mb1_5 => "scroll-mb-1.5",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.375rem; /* 6px */
        /// }
        /// ```
        Ml1_5 => "scroll-ml-1.5",
        /// ```css
        /// {
        ///     scroll-margin: 0.5rem; /* 8px */
        /// }
        /// ```
        M2 => "scroll-m-2",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.5rem; /* 8px */
        ///     scroll-margin-right: 0.5rem; /* 8px */
        /// }
        /// ```
        Mx2 => "scroll-mx-2",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.5rem; /* 8px */
        ///     scroll-margin-bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        My2 => "scroll-my-2",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 0.5rem; /* 8px */
        /// }
        /// ```
        Ms2 => "scroll-ms-2",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 0.5rem; /* 8px */
        /// }
        /// ```
        Me2 => "scroll-me-2",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.5rem; /* 8px */
        /// }
        /// ```
        Mt2 => "scroll-mt-2",
        /// ```css
        /// {
        ///     scroll-margin-right: 0.5rem; /* 8px */
        /// }
        /// ```
        Mr2 => "scroll-mr-2",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        Mb2 => "scroll-mb-2",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.5rem; /* 8px */
        /// }
        /// ```
        Ml2 => "scroll-ml-2",
        /// ```css
        /// {
        ///     scroll-margin: 0.625rem; /* 10px */
        /// }
        /// ```
        M2_5 => "scroll-m-2.5",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.625rem; /* 10px */
        ///     scroll-margin-right: 0.625rem; /* 10px */
        /// }
        /// ```
        Mx2_5 => "scroll-mx-2.5",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.625rem; /* 10px */
        ///     scroll-margin-bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        My2_5 => "scroll-my-2.5",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 0.625rem; /* 10px */
        /// }
        /// ```
        Ms2_5 => "scroll-ms-2.5",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 0.625rem; /* 10px */
        /// }
        /// ```
        Me2_5 => "scroll-me-2.5",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.625rem; /* 10px */
        /// }
        /// ```
        Mt2_5 => "scroll-mt-2.5",
        /// ```css
        /// {
        ///     scroll-margin-right: 0.625rem; /* 10px */
        /// }
        /// ```
        Mr2_5 => "scroll-mr-2.5",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        Mb2_5 => "scroll-mb-2.5",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.625rem; /* 10px */
        /// }
        /// ```
        Ml2_5 => "scroll-ml-2.5",
        /// ```css
        /// {
        ///     scroll-margin: 0.75rem; /* 12px */
        /// }
        /// ```
        M3 => "scroll-m-3",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.75rem; /* 12px */
        ///     scroll-margin-right: 0.75rem; /* 12px */
        /// }
        /// ```
        Mx3 => "scroll-mx-3",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.75rem; /* 12px */
        ///     scroll-margin-bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        My3 => "scroll-my-3",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 0.75rem; /* 12px */
        /// }
        /// ```
        Ms3 => "scroll-ms-3",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 0.75rem; /* 12px */
        /// }
        /// ```
        Me3 => "scroll-me-3",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.75rem; /* 12px */
        /// }
        /// ```
        Mt3 => "scroll-mt-3",
        /// ```css
        /// {
        ///     scroll-margin-right: 0.75rem; /* 12px */
        /// }
        /// ```
        Mr3 => "scroll-mr-3",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        Mb3 => "scroll-mb-3",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.75rem; /* 12px */
        /// }
        /// ```
        Ml3 => "scroll-ml-3",
        /// ```css
        /// {
        ///     scroll-margin: 0.875rem; /* 14px */
        /// }
        /// ```
        M3_5 => "scroll-m-3.5",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.875rem; /* 14px */
        ///     scroll-margin-right: 0.875rem; /* 14px */
        /// }
        /// ```
        Mx3_5 => "scroll-mx-3.5",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.875rem; /* 14px */
        ///     scroll-margin-bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        My3_5 => "scroll-my-3.5",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 0.875rem; /* 14px */
        /// }
        /// ```
        Ms3_5 => "scroll-ms-3.5",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 0.875rem; /* 14px */
        /// }
        /// ```
        Me3_5 => "scroll-me-3.5",
        /// ```css
        /// {
        ///     scroll-margin-top: 0.875rem; /* 14px */
        /// }
        /// ```
        Mt3_5 => "scroll-mt-3.5",
        /// ```css
        /// {
        ///     scroll-margin-right: 0.875rem; /* 14px */
        /// }
        /// ```
        Mr3_5 => "scroll-mr-3.5",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        Mb3_5 => "scroll-mb-3.5",
        /// ```css
        /// {
        ///     scroll-margin-left: 0.875rem; /* 14px */
        /// }
        /// ```
        Ml3_5 => "scroll-ml-3.5",
        /// ```css
        /// {
        ///     scroll-margin: 1rem; /* 16px */
        /// }
        /// ```
        M4 => "scroll-m-4",
        /// ```css
        /// {
        ///     scroll-margin-left: 1rem; /* 16px */
        ///     scroll-margin-right: 1rem; /* 16px */
        /// }
        /// ```
        Mx4 => "scroll-mx-4",
        /// ```css
        /// {
        ///     scroll-margin-top: 1rem; /* 16px */
        ///     scroll-margin-bottom: 1rem; /* 16px */
        /// }
        /// ```
        My4 => "scroll-my-4",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 1rem; /* 16px */
        /// }
        /// ```
        Ms4 => "scroll-ms-4",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 1rem; /* 16px */
        /// }
        /// ```
        Me4 => "scroll-me-4",
        /// ```css
        /// {
        ///     scroll-margin-top: 1rem; /* 16px */
        /// }
        /// ```
        Mt4 => "scroll-mt-4",
        /// ```css
        /// {
        ///     scroll-margin-right: 1rem; /* 16px */
        /// }
        /// ```
        Mr4 => "scroll-mr-4",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 1rem; /* 16px */
        /// }
        /// ```
        Mb4 => "scroll-mb-4",
        /// ```css
        /// {
        ///     scroll-margin-left: 1rem; /* 16px */
        /// }
        /// ```
        Ml4 => "scroll-ml-4",
        /// ```css
        /// {
        ///     scroll-margin: 1.25rem; /* 20px */
        /// }
        /// ```
        M5 => "scroll-m-5",
        /// ```css
        /// {
        ///     scroll-margin-left: 1.25rem; /* 20px */
        ///     scroll-margin-right: 1.25rem; /* 20px */
        /// }
        /// ```
        Mx5 => "scroll-mx-5",
        /// ```css
        /// {
        ///     scroll-margin-top: 1.25rem; /* 20px */
        ///     scroll-margin-bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        My5 => "scroll-my-5",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 1.25rem; /* 20px */
        /// }
        /// ```
        Ms5 => "scroll-ms-5",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 1.25rem; /* 20px */
        /// }
        /// ```
        Me5 => "scroll-me-5",
        /// ```css
        /// {
        ///     scroll-margin-top: 1.25rem; /* 20px */
        /// }
        /// ```
        Mt5 => "scroll-mt-5",
        /// ```css
        /// {
        ///     scroll-margin-right: 1.25rem; /* 20px */
        /// }
        /// ```
        Mr5 => "scroll-mr-5",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        Mb5 => "scroll-mb-5",
        /// ```css
        /// {
        ///     scroll-margin-left: 1.25rem; /* 20px */
        /// }
        /// ```
        Ml5 => "scroll-ml-5",
        /// ```css
        /// {
        ///     scroll-margin: 1.5rem; /* 24px */
        /// }
        /// ```
        M6 => "scroll-m-6",
        /// ```css
        /// {
        ///     scroll-margin-left: 1.5rem; /* 24px */
        ///     scroll-margin-right: 1.5rem; /* 24px */
        /// }
        /// ```
        Mx6 => "scroll-mx-6",
        /// ```css
        /// {
        ///     scroll-margin-top: 1.5rem; /* 24px */
        ///     scroll-margin-bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        My6 => "scroll-my-6",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 1.5rem; /* 24px */
        /// }
        /// ```
        Ms6 => "scroll-ms-6",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 1.5rem; /* 24px */
        /// }
        /// ```
        Me6 => "scroll-me-6",
        /// ```css
        /// {
        ///     scroll-margin-top: 1.5rem; /* 24px */
        /// }
        /// ```
        Mt6 => "scroll-mt-6",
        /// ```css
        /// {
        ///     scroll-margin-right: 1.5rem; /* 24px */
        /// }
        /// ```
        Mr6 => "scroll-mr-6",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        Mb6 => "scroll-mb-6",
        /// ```css
        /// {
        ///     scroll-margin-left: 1.5rem; /* 24px */
        /// }
        /// ```
        Ml6 => "scroll-ml-6",
        /// ```css
        /// {
        ///     scroll-margin: 1.75rem; /* 28px */
        /// }
        /// ```
        M7 => "scroll-m-7",
        /// ```css
        /// {
        ///     scroll-margin-left: 1.75rem; /* 28px */
        ///     scroll-margin-right: 1.75rem; /* 28px */
        /// }
        /// ```
        Mx7 => "scroll-mx-7",
        /// ```css
        /// {
        ///     scroll-margin-top: 1.75rem; /* 28px */
        ///     scroll-margin-bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        My7 => "scroll-my-7",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 1.75rem; /* 28px */
        /// }
        /// ```
        Ms7 => "scroll-ms-7",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 1.75rem; /* 28px */
        /// }
        /// ```
        Me7 => "scroll-me-7",
        /// ```css
        /// {
        ///     scroll-margin-top: 1.75rem; /* 28px */
        /// }
        /// ```
        Mt7 => "scroll-mt-7",
        /// ```css
        /// {
        ///     scroll-margin-right: 1.75rem; /* 28px */
        /// }
        /// ```
        Mr7 => "scroll-mr-7",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        Mb7 => "scroll-mb-7",
        /// ```css
        /// {
        ///     scroll-margin-left: 1.75rem; /* 28px */
        /// }
        /// ```
        Ml7 => "scroll-ml-7",
        /// ```css
        /// {
        ///     scroll-margin: 2rem; /* 32px */
        /// }
        /// ```
        M8 => "scroll-m-8",
        /// ```css
        /// {
        ///     scroll-margin-left: 2rem; /* 32px */
        ///     scroll-margin-right: 2rem; /* 32px */
        /// }
        /// ```
        Mx8 => "scroll-mx-8",
        /// ```css
        /// {
        ///     scroll-margin-top: 2rem; /* 32px */
        ///     scroll-margin-bottom: 2rem; /* 32px */
        /// }
        /// ```
        My8 => "scroll-my-8",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 2rem; /* 32px */
        /// }
        /// ```
        Ms8 => "scroll-ms-8",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 2rem; /* 32px */
        /// }
        /// ```
        Me8 => "scroll-me-8",
        /// ```css
        /// {
        ///     scroll-margin-top: 2rem; /* 32px */
        /// }
        /// ```
        Mt8 => "scroll-mt-8",
        /// ```css
        /// {
        ///     scroll-margin-right: 2rem; /* 32px */
        /// }
        /// ```
        Mr8 => "scroll-mr-8",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 2rem; /* 32px */
        /// }
        /// ```
        Mb8 => "scroll-mb-8",
        /// ```css
        /// {
        ///     scroll-margin-left: 2rem; /* 32px */
        /// }
        /// ```
        Ml8 => "scroll-ml-8",
        /// ```css
        /// {
        ///     scroll-margin: 2.25rem; /* 36px */
        /// }
        /// ```
        M9 => "scroll-m-9",
        /// ```css
        /// {
        ///     scroll-margin-left: 2.25rem; /* 36px */
        ///     scroll-margin-right: 2.25rem; /* 36px */
        /// }
        /// ```
        Mx9 => "scroll-mx-9",
        /// ```css
        /// {
        ///     scroll-margin-top: 2.25rem; /* 36px */
        ///     scroll-margin-bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        My9 => "scroll-my-9",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 2.25rem; /* 36px */
        /// }
        /// ```
        Ms9 => "scroll-ms-9",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 2.25rem; /* 36px */
        /// }
        /// ```
        Me9 => "scroll-me-9",
        /// ```css
        /// {
        ///     scroll-margin-top: 2.25rem; /* 36px */
        /// }
        /// ```
        Mt9 => "scroll-mt-9",
        /// ```css
        /// {
        ///     scroll-margin-right: 2.25rem; /* 36px */
        /// }
        /// ```
        Mr9 => "scroll-mr-9",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        Mb9 => "scroll-mb-9",
        /// ```css
        /// {
        ///     scroll-margin-left: 2.25rem; /* 36px */
        /// }
        /// ```
        Ml9 => "scroll-ml-9",
        /// ```css
        /// {
        ///     scroll-margin: 2.5rem; /* 40px */
        /// }
        /// ```
        M10 => "scroll-m-10",
        /// ```css
        /// {
        ///     scroll-margin-left: 2.5rem; /* 40px */
        ///     scroll-margin-right: 2.5rem; /* 40px */
        /// }
        /// ```
        Mx10 => "scroll-mx-10",
        /// ```css
        /// {
        ///     scroll-margin-top: 2.5rem; /* 40px */
        ///     scroll-margin-bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        My10 => "scroll-my-10",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 2.5rem; /* 40px */
        /// }
        /// ```
        Ms10 => "scroll-ms-10",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 2.5rem; /* 40px */
        /// }
        /// ```
        Me10 => "scroll-me-10",
        /// ```css
        /// {
        ///     scroll-margin-top: 2.5rem; /* 40px */
        /// }
        /// ```
        Mt10 => "scroll-mt-10",
        /// ```css
        /// {
        ///     scroll-margin-right: 2.5rem; /* 40px */
        /// }
        /// ```
        Mr10 => "scroll-mr-10",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        Mb10 => "scroll-mb-10",
        /// ```css
        /// {
        ///     scroll-margin-left: 2.5rem; /* 40px */
        /// }
        /// ```
        Ml10 => "scroll-ml-10",
        /// ```css
        /// {
        ///     scroll-margin: 2.75rem; /* 44px */
        /// }
        /// ```
        M11 => "scroll-m-11",
        /// ```css
        /// {
        ///     scroll-margin-left: 2.75rem; /* 44px */
        ///     scroll-margin-right: 2.75rem; /* 44px */
        /// }
        /// ```
        Mx11 => "scroll-mx-11",
        /// ```css
        /// {
        ///     scroll-margin-top: 2.75rem; /* 44px */
        ///     scroll-margin-bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        My11 => "scroll-my-11",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 2.75rem; /* 44px */
        /// }
        /// ```
        Ms11 => "scroll-ms-11",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 2.75rem; /* 44px */
        /// }
        /// ```
        Me11 => "scroll-me-11",
        /// ```css
        /// {
        ///     scroll-margin-top: 2.75rem; /* 44px */
        /// }
        /// ```
        Mt11 => "scroll-mt-11",
        /// ```css
        /// {
        ///     scroll-margin-right: 2.75rem; /* 44px */
        /// }
        /// ```
        Mr11 => "scroll-mr-11",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        Mb11 => "scroll-mb-11",
        /// ```css
        /// {
        ///     scroll-margin-left: 2.75rem; /* 44px */
        /// }
        /// ```
        Ml11 => "scroll-ml-11",
        /// ```css
        /// {
        ///     scroll-margin: 3rem; /* 48px */
        /// }
        /// ```
        M12 => "scroll-m-12",
        /// ```css
        /// {
        ///     scroll-margin-left: 3rem; /* 48px */
        ///     scroll-margin-right: 3rem; /* 48px */
        /// }
        /// ```
        Mx12 => "scroll-mx-12",
        /// ```css
        /// {
        ///     scroll-margin-top: 3rem; /* 48px */
        ///     scroll-margin-bottom: 3rem; /* 48px */
        /// }
        /// ```
        My12 => "scroll-my-12",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 3rem; /* 48px */
        /// }
        /// ```
        Ms12 => "scroll-ms-12",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 3rem; /* 48px */
        /// }
        /// ```
        Me12 => "scroll-me-12",
        /// ```css
        /// {
        ///     scroll-margin-top: 3rem; /* 48px */
        /// }
        /// ```
        Mt12 => "scroll-mt-12",
        /// ```css
        /// {
        ///     scroll-margin-right: 3rem; /* 48px */
        /// }
        /// ```
        Mr12 => "scroll-mr-12",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 3rem; /* 48px */
        /// }
        /// ```
        Mb12 => "scroll-mb-12",
        /// ```css
        /// {
        ///     scroll-margin-left: 3rem; /* 48px */
        /// }
        /// ```
        Ml12 => "scroll-ml-12",
        /// ```css
        /// {
        ///     scroll-margin: 3.5rem; /* 56px */
        /// }
        /// ```
        M14 => "scroll-m-14",
        /// ```css
        /// {
        ///     scroll-margin-left: 3.5rem; /* 56px */
        ///     scroll-margin-right: 3.5rem; /* 56px */
        /// }
        /// ```
        Mx14 => "scroll-mx-14",
        /// ```css
        /// {
        ///     scroll-margin-top: 3.5rem; /* 56px */
        ///     scroll-margin-bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        My14 => "scroll-my-14",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 3.5rem; /* 56px */
        /// }
        /// ```
        Ms14 => "scroll-ms-14",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 3.5rem; /* 56px */
        /// }
        /// ```
        Me14 => "scroll-me-14",
        /// ```css
        /// {
        ///     scroll-margin-top: 3.5rem; /* 56px */
        /// }
        /// ```
        Mt14 => "scroll-mt-14",
        /// ```css
        /// {
        ///     scroll-margin-right: 3.5rem; /* 56px */
        /// }
        /// ```
        Mr14 => "scroll-mr-14",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        Mb14 => "scroll-mb-14",
        /// ```css
        /// {
        ///     scroll-margin-left: 3.5rem; /* 56px */
        /// }
        /// ```
        Ml14 => "scroll-ml-14",
        /// ```css
        /// {
        ///     scroll-margin: 4rem; /* 64px */
        /// }
        /// ```
        M16 => "scroll-m-16",
        /// ```css
        /// {
        ///     scroll-margin-left: 4rem; /* 64px */
        ///     scroll-margin-right: 4rem; /* 64px */
        /// }
        /// ```
        Mx16 => "scroll-mx-16",
        /// ```css
        /// {
        ///     scroll-margin-top: 4rem; /* 64px */
        ///     scroll-margin-bottom: 4rem; /* 64px */
        /// }
        /// ```
        My16 => "scroll-my-16",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 4rem; /* 64px */
        /// }
        /// ```
        Ms16 => "scroll-ms-16",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 4rem; /* 64px */
        /// }
        /// ```
        Me16 => "scroll-me-16",
        /// ```css
        /// {
        ///     scroll-margin-top: 4rem; /* 64px */
        /// }
        /// ```
        Mt16 => "scroll-mt-16",
        /// ```css
        /// {
        ///     scroll-margin-right: 4rem; /* 64px */
        /// }
        /// ```
        Mr16 => "scroll-mr-16",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 4rem; /* 64px */
        /// }
        /// ```
        Mb16 => "scroll-mb-16",
        /// ```css
        /// {
        ///     scroll-margin-left: 4rem; /* 64px */
        /// }
        /// ```
        Ml16 => "scroll-ml-16",
        /// ```css
        /// {
        ///     scroll-margin: 5rem; /* 80px */
        /// }
        /// ```
        M20 => "scroll-m-20",
        /// ```css
        /// {
        ///     scroll-margin-left: 5rem; /* 80px */
        ///     scroll-margin-right: 5rem; /* 80px */
        /// }
        /// ```
        Mx20 => "scroll-mx-20",
        /// ```css
        /// {
        ///     scroll-margin-top: 5rem; /* 80px */
        ///     scroll-margin-bottom: 5rem; /* 80px */
        /// }
        /// ```
        My20 => "scroll-my-20",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 5rem; /* 80px */
        /// }
        /// ```
        Ms20 => "scroll-ms-20",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 5rem; /* 80px */
        /// }
        /// ```
        Me20 => "scroll-me-20",
        /// ```css
        /// {
        ///     scroll-margin-top: 5rem; /* 80px */
        /// }
        /// ```
        Mt20 => "scroll-mt-20",
        /// ```css
        /// {
        ///     scroll-margin-right: 5rem; /* 80px */
        /// }
        /// ```
        Mr20 => "scroll-mr-20",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 5rem; /* 80px */
        /// }
        /// ```
        Mb20 => "scroll-mb-20",
        /// ```css
        /// {
        ///     scroll-margin-left: 5rem; /* 80px */
        /// }
        /// ```
        Ml20 => "scroll-ml-20",
        /// ```css
        /// {
        ///     scroll-margin: 6rem; /* 96px */
        /// }
        /// ```
        M24 => "scroll-m-24",
        /// ```css
        /// {
        ///     scroll-margin-left: 6rem; /* 96px */
        ///     scroll-margin-right: 6rem; /* 96px */
        /// }
        /// ```
        Mx24 => "scroll-mx-24",
        /// ```css
        /// {
        ///     scroll-margin-top: 6rem; /* 96px */
        ///     scroll-margin-bottom: 6rem; /* 96px */
        /// }
        /// ```
        My24 => "scroll-my-24",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 6rem; /* 96px */
        /// }
        /// ```
        Ms24 => "scroll-ms-24",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 6rem; /* 96px */
        /// }
        /// ```
        Me24 => "scroll-me-24",
        /// ```css
        /// {
        ///     scroll-margin-top: 6rem; /* 96px */
        /// }
        /// ```
        Mt24 => "scroll-mt-24",
        /// ```css
        /// {
        ///     scroll-margin-right: 6rem; /* 96px */
        /// }
        /// ```
        Mr24 => "scroll-mr-24",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 6rem; /* 96px */
        /// }
        /// ```
        Mb24 => "scroll-mb-24",
        /// ```css
        /// {
        ///     scroll-margin-left: 6rem; /* 96px */
        /// }
        /// ```
        Ml24 => "scroll-ml-24",
        /// ```css
        /// {
        ///     scroll-margin: 7rem; /* 112px */
        /// }
        /// ```
        M28 => "scroll-m-28",
        /// ```css
        /// {
        ///     scroll-margin-left: 7rem; /* 112px */
        ///     scroll-margin-right: 7rem; /* 112px */
        /// }
        /// ```
        Mx28 => "scroll-mx-28",
        /// ```css
        /// {
        ///     scroll-margin-top: 7rem; /* 112px */
        ///     scroll-margin-bottom: 7rem; /* 112px */
        /// }
        /// ```
        My28 => "scroll-my-28",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 7rem; /* 112px */
        /// }
        /// ```
        Ms28 => "scroll-ms-28",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 7rem; /* 112px */
        /// }
        /// ```
        Me28 => "scroll-me-28",
        /// ```css
        /// {
        ///     scroll-margin-top: 7rem; /* 112px */
        /// }
        /// ```
        Mt28 => "scroll-mt-28",
        /// ```css
        /// {
        ///     scroll-margin-right: 7rem; /* 112px */
        /// }
        /// ```
        Mr28 => "scroll-mr-28",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 7rem; /* 112px */
        /// }
        /// ```
        Mb28 => "scroll-mb-28",
        /// ```css
        /// {
        ///     scroll-margin-left: 7rem; /* 112px */
        /// }
        /// ```
        Ml28 => "scroll-ml-28",
        /// ```css
        /// {
        ///     scroll-margin: 8rem; /* 128px */
        /// }
        /// ```
        M32 => "scroll-m-32",
        /// ```css
        /// {
        ///     scroll-margin-left: 8rem; /* 128px */
        ///     scroll-margin-right: 8rem; /* 128px */
        /// }
        /// ```
        Mx32 => "scroll-mx-32",
        /// ```css
        /// {
        ///     scroll-margin-top: 8rem; /* 128px */
        ///     scroll-margin-bottom: 8rem; /* 128px */
        /// }
        /// ```
        My32 => "scroll-my-32",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 8rem; /* 128px */
        /// }
        /// ```
        Ms32 => "scroll-ms-32",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 8rem; /* 128px */
        /// }
        /// ```
        Me32 => "scroll-me-32",
        /// ```css
        /// {
        ///     scroll-margin-top: 8rem; /* 128px */
        /// }
        /// ```
        Mt32 => "scroll-mt-32",
        /// ```css
        /// {
        ///     scroll-margin-right: 8rem; /* 128px */
        /// }
        /// ```
        Mr32 => "scroll-mr-32",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 8rem; /* 128px */
        /// }
        /// ```
        Mb32 => "scroll-mb-32",
        /// ```css
        /// {
        ///     scroll-margin-left: 8rem; /* 128px */
        /// }
        /// ```
        Ml32 => "scroll-ml-32",
        /// ```css
        /// {
        ///     scroll-margin: 9rem; /* 144px */
        /// }
        /// ```
        M36 => "scroll-m-36",
        /// ```css
        /// {
        ///     scroll-margin-left: 9rem; /* 144px */
        ///     scroll-margin-right: 9rem; /* 144px */
        /// }
        /// ```
        Mx36 => "scroll-mx-36",
        /// ```css
        /// {
        ///     scroll-margin-top: 9rem; /* 144px */
        ///     scroll-margin-bottom: 9rem; /* 144px */
        /// }
        /// ```
        My36 => "scroll-my-36",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 9rem; /* 144px */
        /// }
        /// ```
        Ms36 => "scroll-ms-36",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 9rem; /* 144px */
        /// }
        /// ```
        Me36 => "scroll-me-36",
        /// ```css
        /// {
        ///     scroll-margin-top: 9rem; /* 144px */
        /// }
        /// ```
        Mt36 => "scroll-mt-36",
        /// ```css
        /// {
        ///     scroll-margin-right: 9rem; /* 144px */
        /// }
        /// ```
        Mr36 => "scroll-mr-36",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 9rem; /* 144px */
        /// }
        /// ```
        Mb36 => "scroll-mb-36",
        /// ```css
        /// {
        ///     scroll-margin-left: 9rem; /* 144px */
        /// }
        /// ```
        Ml36 => "scroll-ml-36",
        /// ```css
        /// {
        ///     scroll-margin: 10rem; /* 160px */
        /// }
        /// ```
        M40 => "scroll-m-40",
        /// ```css
        /// {
        ///     scroll-margin-left: 10rem; /* 160px */
        ///     scroll-margin-right: 10rem; /* 160px */
        /// }
        /// ```
        Mx40 => "scroll-mx-40",
        /// ```css
        /// {
        ///     scroll-margin-top: 10rem; /* 160px */
        ///     scroll-margin-bottom: 10rem; /* 160px */
        /// }
        /// ```
        My40 => "scroll-my-40",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 10rem; /* 160px */
        /// }
        /// ```
        Ms40 => "scroll-ms-40",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 10rem; /* 160px */
        /// }
        /// ```
        Me40 => "scroll-me-40",
        /// ```css
        /// {
        ///     scroll-margin-top: 10rem; /* 160px */
        /// }
        /// ```
        Mt40 => "scroll-mt-40",
        /// ```css
        /// {
        ///     scroll-margin-right: 10rem; /* 160px */
        /// }
        /// ```
        Mr40 => "scroll-mr-40",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 10rem; /* 160px */
        /// }
        /// ```
        Mb40 => "scroll-mb-40",
        /// ```css
        /// {
        ///     scroll-margin-left: 10rem; /* 160px */
        /// }
        /// ```
        Ml40 => "scroll-ml-40",
        /// ```css
        /// {
        ///     scroll-margin: 11rem; /* 176px */
        /// }
        /// ```
        M44 => "scroll-m-44",
        /// ```css
        /// {
        ///     scroll-margin-left: 11rem; /* 176px */
        ///     scroll-margin-right: 11rem; /* 176px */
        /// }
        /// ```
        Mx44 => "scroll-mx-44",
        /// ```css
        /// {
        ///     scroll-margin-top: 11rem; /* 176px */
        ///     scroll-margin-bottom: 11rem; /* 176px */
        /// }
        /// ```
        My44 => "scroll-my-44",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 11rem; /* 176px */
        /// }
        /// ```
        Ms44 => "scroll-ms-44",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 11rem; /* 176px */
        /// }
        /// ```
        Me44 => "scroll-me-44",
        /// ```css
        /// {
        ///     scroll-margin-top: 11rem; /* 176px */
        /// }
        /// ```
        Mt44 => "scroll-mt-44",
        /// ```css
        /// {
        ///     scroll-margin-right: 11rem; /* 176px */
        /// }
        /// ```
        Mr44 => "scroll-mr-44",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 11rem; /* 176px */
        /// }
        /// ```
        Mb44 => "scroll-mb-44",
        /// ```css
        /// {
        ///     scroll-margin-left: 11rem; /* 176px */
        /// }
        /// ```
        Ml44 => "scroll-ml-44",
        /// ```css
        /// {
        ///     scroll-margin: 12rem; /* 192px */
        /// }
        /// ```
        M48 => "scroll-m-48",
        /// ```css
        /// {
        ///     scroll-margin-left: 12rem; /* 192px */
        ///     scroll-margin-right: 12rem; /* 192px */
        /// }
        /// ```
        Mx48 => "scroll-mx-48",
        /// ```css
        /// {
        ///     scroll-margin-top: 12rem; /* 192px */
        ///     scroll-margin-bottom: 12rem; /* 192px */
        /// }
        /// ```
        My48 => "scroll-my-48",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 12rem; /* 192px */
        /// }
        /// ```
        Ms48 => "scroll-ms-48",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 12rem; /* 192px */
        /// }
        /// ```
        Me48 => "scroll-me-48",
        /// ```css
        /// {
        ///     scroll-margin-top: 12rem; /* 192px */
        /// }
        /// ```
        Mt48 => "scroll-mt-48",
        /// ```css
        /// {
        ///     scroll-margin-right: 12rem; /* 192px */
        /// }
        /// ```
        Mr48 => "scroll-mr-48",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 12rem; /* 192px */
        /// }
        /// ```
        Mb48 => "scroll-mb-48",
        /// ```css
        /// {
        ///     scroll-margin-left: 12rem; /* 192px */
        /// }
        /// ```
        Ml48 => "scroll-ml-48",
        /// ```css
        /// {
        ///     scroll-margin: 13rem; /* 208px */
        /// }
        /// ```
        M52 => "scroll-m-52",
        /// ```css
        /// {
        ///     scroll-margin-left: 13rem; /* 208px */
        ///     scroll-margin-right: 13rem; /* 208px */
        /// }
        /// ```
        Mx52 => "scroll-mx-52",
        /// ```css
        /// {
        ///     scroll-margin-top: 13rem; /* 208px */
        ///     scroll-margin-bottom: 13rem; /* 208px */
        /// }
        /// ```
        My52 => "scroll-my-52",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 13rem; /* 208px */
        /// }
        /// ```
        Ms52 => "scroll-ms-52",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 13rem; /* 208px */
        /// }
        /// ```
        Me52 => "scroll-me-52",
        /// ```css
        /// {
        ///     scroll-margin-top: 13rem; /* 208px */
        /// }
        /// ```
        Mt52 => "scroll-mt-52",
        /// ```css
        /// {
        ///     scroll-margin-right: 13rem; /* 208px */
        /// }
        /// ```
        Mr52 => "scroll-mr-52",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 13rem; /* 208px */
        /// }
        /// ```
        Mb52 => "scroll-mb-52",
        /// ```css
        /// {
        ///     scroll-margin-left: 13rem; /* 208px */
        /// }
        /// ```
        Ml52 => "scroll-ml-52",
        /// ```css
        /// {
        ///     scroll-margin: 14rem; /* 224px */
        /// }
        /// ```
        M56 => "scroll-m-56",
        /// ```css
        /// {
        ///     scroll-margin-left: 14rem; /* 224px */
        ///     scroll-margin-right: 14rem; /* 224px */
        /// }
        /// ```
        Mx56 => "scroll-mx-56",
        /// ```css
        /// {
        ///     scroll-margin-top: 14rem; /* 224px */
        ///     scroll-margin-bottom: 14rem; /* 224px */
        /// }
        /// ```
        My56 => "scroll-my-56",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 14rem; /* 224px */
        /// }
        /// ```
        Ms56 => "scroll-ms-56",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 14rem; /* 224px */
        /// }
        /// ```
        Me56 => "scroll-me-56",
        /// ```css
        /// {
        ///     scroll-margin-top: 14rem; /* 224px */
        /// }
        /// ```
        Mt56 => "scroll-mt-56",
        /// ```css
        /// {
        ///     scroll-margin-right: 14rem; /* 224px */
        /// }
        /// ```
        Mr56 => "scroll-mr-56",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 14rem; /* 224px */
        /// }
        /// ```
        Mb56 => "scroll-mb-56",
        /// ```css
        /// {
        ///     scroll-margin-left: 14rem; /* 224px */
        /// }
        /// ```
        Ml56 => "scroll-ml-56",
        /// ```css
        /// {
        ///     scroll-margin: 15rem; /* 240px */
        /// }
        /// ```
        M60 => "scroll-m-60",
        /// ```css
        /// {
        ///     scroll-margin-left: 15rem; /* 240px */
        ///     scroll-margin-right: 15rem; /* 240px */
        /// }
        /// ```
        Mx60 => "scroll-mx-60",
        /// ```css
        /// {
        ///     scroll-margin-top: 15rem; /* 240px */
        ///     scroll-margin-bottom: 15rem; /* 240px */
        /// }
        /// ```
        My60 => "scroll-my-60",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 15rem; /* 240px */
        /// }
        /// ```
        Ms60 => "scroll-ms-60",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 15rem; /* 240px */
        /// }
        /// ```
        Me60 => "scroll-me-60",
        /// ```css
        /// {
        ///     scroll-margin-top: 15rem; /* 240px */
        /// }
        /// ```
        Mt60 => "scroll-mt-60",
        /// ```css
        /// {
        ///     scroll-margin-right: 15rem; /* 240px */
        /// }
        /// ```
        Mr60 => "scroll-mr-60",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 15rem; /* 240px */
        /// }
        /// ```
        Mb60 => "scroll-mb-60",
        /// ```css
        /// {
        ///     scroll-margin-left: 15rem; /* 240px */
        /// }
        /// ```
        Ml60 => "scroll-ml-60",
        /// ```css
        /// {
        ///     scroll-margin: 16rem; /* 256px */
        /// }
        /// ```
        M64 => "scroll-m-64",
        /// ```css
        /// {
        ///     scroll-margin-left: 16rem; /* 256px */
        ///     scroll-margin-right: 16rem; /* 256px */
        /// }
        /// ```
        Mx64 => "scroll-mx-64",
        /// ```css
        /// {
        ///     scroll-margin-top: 16rem; /* 256px */
        ///     scroll-margin-bottom: 16rem; /* 256px */
        /// }
        /// ```
        My64 => "scroll-my-64",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 16rem; /* 256px */
        /// }
        /// ```
        Ms64 => "scroll-ms-64",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 16rem; /* 256px */
        /// }
        /// ```
        Me64 => "scroll-me-64",
        /// ```css
        /// {
        ///     scroll-margin-top: 16rem; /* 256px */
        /// }
        /// ```
        Mt64 => "scroll-mt-64",
        /// ```css
        /// {
        ///     scroll-margin-right: 16rem; /* 256px */
        /// }
        /// ```
        Mr64 => "scroll-mr-64",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 16rem; /* 256px */
        /// }
        /// ```
        Mb64 => "scroll-mb-64",
        /// ```css
        /// {
        ///     scroll-margin-left: 16rem; /* 256px */
        /// }
        /// ```
        Ml64 => "scroll-ml-64",
        /// ```css
        /// {
        ///     scroll-margin: 18rem; /* 288px */
        /// }
        /// ```
        M72 => "scroll-m-72",
        /// ```css
        /// {
        ///     scroll-margin-left: 18rem; /* 288px */
        ///     scroll-margin-right: 18rem; /* 288px */
        /// }
        /// ```
        Mx72 => "scroll-mx-72",
        /// ```css
        /// {
        ///     scroll-margin-top: 18rem; /* 288px */
        ///     scroll-margin-bottom: 18rem; /* 288px */
        /// }
        /// ```
        My72 => "scroll-my-72",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 18rem; /* 288px */
        /// }
        /// ```
        Ms72 => "scroll-ms-72",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 18rem; /* 288px */
        /// }
        /// ```
        Me72 => "scroll-me-72",
        /// ```css
        /// {
        ///     scroll-margin-top: 18rem; /* 288px */
        /// }
        /// ```
        Mt72 => "scroll-mt-72",
        /// ```css
        /// {
        ///     scroll-margin-right: 18rem; /* 288px */
        /// }
        /// ```
        Mr72 => "scroll-mr-72",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 18rem; /* 288px */
        /// }
        /// ```
        Mb72 => "scroll-mb-72",
        /// ```css
        /// {
        ///     scroll-margin-left: 18rem; /* 288px */
        /// }
        /// ```
        Ml72 => "scroll-ml-72",
        /// ```css
        /// {
        ///     scroll-margin: 20rem; /* 320px */
        /// }
        /// ```
        M80 => "scroll-m-80",
        /// ```css
        /// {
        ///     scroll-margin-left: 20rem; /* 320px */
        ///     scroll-margin-right: 20rem; /* 320px */
        /// }
        /// ```
        Mx80 => "scroll-mx-80",
        /// ```css
        /// {
        ///     scroll-margin-top: 20rem; /* 320px */
        ///     scroll-margin-bottom: 20rem; /* 320px */
        /// }
        /// ```
        My80 => "scroll-my-80",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 20rem; /* 320px */
        /// }
        /// ```
        Ms80 => "scroll-ms-80",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 20rem; /* 320px */
        /// }
        /// ```
        Me80 => "scroll-me-80",
        /// ```css
        /// {
        ///     scroll-margin-top: 20rem; /* 320px */
        /// }
        /// ```
        Mt80 => "scroll-mt-80",
        /// ```css
        /// {
        ///     scroll-margin-right: 20rem; /* 320px */
        /// }
        /// ```
        Mr80 => "scroll-mr-80",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 20rem; /* 320px */
        /// }
        /// ```
        Mb80 => "scroll-mb-80",
        /// ```css
        /// {
        ///     scroll-margin-left: 20rem; /* 320px */
        /// }
        /// ```
        Ml80 => "scroll-ml-80",
        /// ```css
        /// {
        ///     scroll-margin: 24rem; /* 384px */
        /// }
        /// ```
        M96 => "scroll-m-96",
        /// ```css
        /// {
        ///     scroll-margin-left: 24rem; /* 384px */
        ///     scroll-margin-right: 24rem; /* 384px */
        /// }
        /// ```
        Mx96 => "scroll-mx-96",
        /// ```css
        /// {
        ///     scroll-margin-top: 24rem; /* 384px */
        ///     scroll-margin-bottom: 24rem; /* 384px */
        /// }
        /// ```
        My96 => "scroll-my-96",
        /// ```css
        /// {
        ///     scroll-margin-inline-start: 24rem; /* 384px */
        /// }
        /// ```
        Ms96 => "scroll-ms-96",
        /// ```css
        /// {
        ///     scroll-margin-inline-end: 24rem; /* 384px */
        /// }
        /// ```
        Me96 => "scroll-me-96",
        /// ```css
        /// {
        ///     scroll-margin-top: 24rem; /* 384px */
        /// }
        /// ```
        Mt96 => "scroll-mt-96",
        /// ```css
        /// {
        ///     scroll-margin-right: 24rem; /* 384px */
        /// }
        /// ```
        Mr96 => "scroll-mr-96",
        /// ```css
        /// {
        ///     scroll-margin-bottom: 24rem; /* 384px */
        /// }
        /// ```
        Mb96 => "scroll-mb-96",
        /// ```css
        /// {
        ///     scroll-margin-left: 24rem; /* 384px */
        /// }
        /// ```
        Ml96 => "scroll-ml-96",
    }
    /// Utilities for controlling an element's scroll offset within a snap container.
    ///
    /// <https://tailwindcss.com/docs/scroll-padding>
    ScrollPadding {
        /// ```css
        /// {
        ///     scroll-padding: 0px;
        /// }
        /// ```
        P0 => "scroll-p-0",
        /// ```css
        /// {
        ///     scroll-padding-left: 0px;
        ///     scroll-padding-right: 0px;
        /// }
        /// ```
        Px0 => "scroll-px-0",
        /// ```css
        /// {
        ///     scroll-padding-top: 0px;
        ///     scroll-padding-bottom: 0px;
        /// }
        /// ```
        Py0 => "scroll-py-0",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 0px;
        /// }
        /// ```
        Ps0 => "scroll-ps-0",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 0px;
        /// }
        /// ```
        Pe0 => "scroll-pe-0",
        /// ```css
        /// {
        ///     scroll-padding-top: 0px;
        /// }
        /// ```
        Pt0 => "scroll-pt-0",
        /// ```css
        /// {
        ///     scroll-padding-right: 0px;
        /// }
        /// ```
        Pr0 => "scroll-pr-0",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 0px;
        /// }
        /// ```
        Pb0 => "scroll-pb-0",
        /// ```css
        /// {
        ///     scroll-padding-left: 0px;
        /// }
        /// ```
        Pl0 => "scroll-pl-0",
        /// ```css
        /// {
        ///     scroll-padding: 1px;
        /// }
        /// ```
        PPx => "scroll-p-px",
        /// ```css
        /// {
        ///     scroll-padding-left: 1px;
        ///     scroll-padding-right: 1px;
        /// }
        /// ```
        PxPx => "scroll-px-px",
        /// ```css
        /// {
        ///     scroll-padding-top: 1px;
        ///     scroll-padding-bottom: 1px;
        /// }
        /// ```
        PyPx => "scroll-py-px",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 1px;
        /// }
        /// ```
        PsPx => "scroll-ps-px",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 1px;
        /// }
        /// ```
        PePx => "scroll-pe-px",
        /// ```css
        /// {
        ///     scroll-padding-top: 1px;
        /// }
        /// ```
        PtPx => "scroll-pt-px",
        /// ```css
        /// {
        ///     scroll-padding-right: 1px;
        /// }
        /// ```
        PrPx => "scroll-pr-px",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 1px;
        /// }
        /// ```
        PbPx => "scroll-pb-px",
        /// ```css
        /// {
        ///     scroll-padding-left: 1px;
        /// }
        /// ```
        PlPx => "scroll-pl-px",
        /// ```css
        /// {
        ///     scroll-padding: 0.125rem; /* 2px */
        /// }
        /// ```
        P0_5 => "scroll-p-0.5",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.125rem; /* 2px */
        ///     scroll-padding-right: 0.125rem; /* 2px */
        /// }
        /// ```
        Px0_5 => "scroll-px-0.5",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.125rem; /* 2px */
        ///     scroll-padding-bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        Py0_5 => "scroll-py-0.5",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 0.125rem; /* 2px */
        /// }
        /// ```
        Ps0_5 => "scroll-ps-0.5",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 0.125rem; /* 2px */
        /// }
        /// ```
        Pe0_5 => "scroll-pe-0.5",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.125rem; /* 2px */
        /// }
        /// ```
        Pt0_5 => "scroll-pt-0.5",
        /// ```css
        /// {
        ///     scroll-padding-right: 0.125rem; /* 2px */
        /// }
        /// ```
        Pr0_5 => "scroll-pr-0.5",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        Pb0_5 => "scroll-pb-0.5",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.125rem; /* 2px */
        /// }
        /// ```
        Pl0_5 => "scroll-pl-0.5",
        /// ```css
        /// {
        ///     scroll-padding: 0.25rem; /* 4px */
        /// }
        /// ```
        P1 => "scroll-p-1",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.25rem; /* 4px */
        ///     scroll-padding-right: 0.25rem; /* 4px */
        /// }
        /// ```
        Px1 => "scroll-px-1",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.25rem; /* 4px */
        ///     scroll-padding-bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        Py1 => "scroll-py-1",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 0.25rem; /* 4px */
        /// }
        /// ```
        Ps1 => "scroll-ps-1",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 0.25rem; /* 4px */
        /// }
        /// ```
        Pe1 => "scroll-pe-1",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.25rem; /* 4px */
        /// }
        /// ```
        Pt1 => "scroll-pt-1",
        /// ```css
        /// {
        ///     scroll-padding-right: 0.25rem; /* 4px */
        /// }
        /// ```
        Pr1 => "scroll-pr-1",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        Pb1 => "scroll-pb-1",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.25rem; /* 4px */
        /// }
        /// ```
        Pl1 => "scroll-pl-1",
        /// ```css
        /// {
        ///     scroll-padding: 0.375rem; /* 6px */
        /// }
        /// ```
        P1_5 => "scroll-p-1.5",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.375rem; /* 6px */
        ///     scroll-padding-right: 0.375rem; /* 6px */
        /// }
        /// ```
        Px1_5 => "scroll-px-1.5",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.375rem; /* 6px */
        ///     scroll-padding-bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        Py1_5 => "scroll-py-1.5",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 0.375rem; /* 6px */
        /// }
        /// ```
        Ps1_5 => "scroll-ps-1.5",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 0.375rem; /* 6px */
        /// }
        /// ```
        Pe1_5 => "scroll-pe-1.5",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.375rem; /* 6px */
        /// }
        /// ```
        Pt1_5 => "scroll-pt-1.5",
        /// ```css
        /// {
        ///     scroll-padding-right: 0.375rem; /* 6px */
        /// }
        /// ```
        Pr1_5 => "scroll-pr-1.5",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        Pb1_5 => "scroll-pb-1.5",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.375rem; /* 6px */
        /// }
        /// ```
        Pl1_5 => "scroll-pl-1.5",
        /// ```css
        /// {
        ///     scroll-padding: 0.5rem; /* 8px */
        /// }
        /// ```
        P2 => "scroll-p-2",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.5rem; /* 8px */
        ///     scroll-padding-right: 0.5rem; /* 8px */
        /// }
        /// ```
        Px2 => "scroll-px-2",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.5rem; /* 8px */
        ///     scroll-padding-bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        Py2 => "scroll-py-2",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 0.5rem; /* 8px */
        /// }
        /// ```
        Ps2 => "scroll-ps-2",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 0.5rem; /* 8px */
        /// }
        /// ```
        Pe2 => "scroll-pe-2",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.5rem; /* 8px */
        /// }
        /// ```
        Pt2 => "scroll-pt-2",
        /// ```css
        /// {
        ///     scroll-padding-right: 0.5rem; /* 8px */
        /// }
        /// ```
        Pr2 => "scroll-pr-2",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        Pb2 => "scroll-pb-2",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.5rem; /* 8px */
        /// }
        /// ```
        Pl2 => "scroll-pl-2",
        /// ```css
        /// {
        ///     scroll-padding: 0.625rem; /* 10px */
        /// }
        /// ```
        P2_5 => "scroll-p-2.5",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.625rem; /* 10px */
        ///     scroll-padding-right: 0.625rem; /* 10px */
        /// }
        /// ```
        Px2_5 => "scroll-px-2.5",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.625rem; /* 10px */
        ///     scroll-padding-bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        Py2_5 => "scroll-py-2.5",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 0.625rem; /* 10px */
        /// }
        /// ```
        Ps2_5 => "scroll-ps-2.5",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 0.625rem; /* 10px */
        /// }
        /// ```
        Pe2_5 => "scroll-pe-2.5",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.625rem; /* 10px */
        /// }
        /// ```
        Pt2_5 => "scroll-pt-2.5",
        /// ```css
        /// {
        ///     scroll-padding-right: 0.625rem; /* 10px */
        /// }
        /// ```
        Pr2_5 => "scroll-pr-2.5",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        Pb2_5 => "scroll-pb-2.5",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.625rem; /* 10px */
        /// }
        /// ```
        Pl2_5 => "scroll-pl-2.5",
        /// ```css
        /// {
        ///     scroll-padding: 0.75rem; /* 12px */
        /// }
        /// ```
        P3 => "scroll-p-3",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.75rem; /* 12px */
        ///     scroll-padding-right: 0.75rem; /* 12px */
        /// }
        /// ```
        Px3 => "scroll-px-3",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.75rem; /* 12px */
        ///     scroll-padding-bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        Py3 => "scroll-py-3",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 0.75rem; /* 12px */
        /// }
        /// ```
        Ps3 => "scroll-ps-3",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 0.75rem; /* 12px */
        /// }
        /// ```
        Pe3 => "scroll-pe-3",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.75rem; /* 12px */
        /// }
        /// ```
        Pt3 => "scroll-pt-3",
        /// ```css
        /// {
        ///     scroll-padding-right: 0.75rem; /* 12px */
        /// }
        /// ```
        Pr3 => "scroll-pr-3",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        Pb3 => "scroll-pb-3",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.75rem; /* 12px */
        /// }
        /// ```
        Pl3 => "scroll-pl-3",
        /// ```css
        /// {
        ///     scroll-padding: 0.875rem; /* 14px */
        /// }
        /// ```
        P3_5 => "scroll-p-3.5",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.875rem; /* 14px */
        ///     scroll-padding-right: 0.875rem; /* 14px */
        /// }
        /// ```
        Px3_5 => "scroll-px-3.5",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.875rem; /* 14px */
        ///     scroll-padding-bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        Py3_5 => "scroll-py-3.5",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 0.875rem; /* 14px */
        /// }
        /// ```
        Ps3_5 => "scroll-ps-3.5",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 0.875rem; /* 14px */
        /// }
        /// ```
        Pe3_5 => "scroll-pe-3.5",
        /// ```css
        /// {
        ///     scroll-padding-top: 0.875rem; /* 14px */
        /// }
        /// ```
        Pt3_5 => "scroll-pt-3.5",
        /// ```css
        /// {
        ///     scroll-padding-right: 0.875rem; /* 14px */
        /// }
        /// ```
        Pr3_5 => "scroll-pr-3.5",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        Pb3_5 => "scroll-pb-3.5",
        /// ```css
        /// {
        ///     scroll-padding-left: 0.875rem; /* 14px */
        /// }
        /// ```
        Pl3_5 => "scroll-pl-3.5",
        /// ```css
        /// {
        ///     scroll-padding: 1rem; /* 16px */
        /// }
        /// ```
        P4 => "scroll-p-4",
        /// ```css
        /// {
        ///     scroll-padding-left: 1rem; /* 16px */
        ///     scroll-padding-right: 1rem; /* 16px */
        /// }
        /// ```
        Px4 => "scroll-px-4",
        /// ```css
        /// {
        ///     scroll-padding-top: 1rem; /* 16px */
        ///     scroll-padding-bottom: 1rem; /* 16px */
        /// }
        /// ```
        Py4 => "scroll-py-4",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 1rem; /* 16px */
        /// }
        /// ```
        Ps4 => "scroll-ps-4",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 1rem; /* 16px */
        /// }
        /// ```
        Pe4 => "scroll-pe-4",
        /// ```css
        /// {
        ///     scroll-padding-top: 1rem; /* 16px */
        /// }
        /// ```
        Pt4 => "scroll-pt-4",
        /// ```css
        /// {
        ///     scroll-padding-right: 1rem; /* 16px */
        /// }
        /// ```
        Pr4 => "scroll-pr-4",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 1rem; /* 16px */
        /// }
        /// ```
        Pb4 => "scroll-pb-4",
        /// ```css
        /// {
        ///     scroll-padding-left: 1rem; /* 16px */
        /// }
        /// ```
        Pl4 => "scroll-pl-4",
        /// ```css
        /// {
        ///     scroll-padding: 1.25rem; /* 20px */
        /// }
        /// ```
        P5 => "scroll-p-5",
        /// ```css
        /// {
        ///     scroll-padding-left: 1.25rem; /* 20px */
        ///     scroll-padding-right: 1.25rem; /* 20px */
        /// }
        /// ```
        Px5 => "scroll-px-5",
        /// ```css
        /// {
        ///     scroll-padding-top: 1.25rem; /* 20px */
        ///     scroll-padding-bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        Py5 => "scroll-py-5",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 1.25rem; /* 20px */
        /// }
        /// ```
        Ps5 => "scroll-ps-5",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 1.25rem; /* 20px */
        /// }
        /// ```
        Pe5 => "scroll-pe-5",
        /// ```css
        /// {
        ///     scroll-padding-top: 1.25rem; /* 20px */
        /// }
        /// ```
        Pt5 => "scroll-pt-5",
        /// ```css
        /// {
        ///     scroll-padding-right: 1.25rem; /* 20px */
        /// }
        /// ```
        Pr5 => "scroll-pr-5",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        Pb5 => "scroll-pb-5",
        /// ```css
        /// {
        ///     scroll-padding-left: 1.25rem; /* 20px */
        /// }
        /// ```
        Pl5 => "scroll-pl-5",
        /// ```css
        /// {
        ///     scroll-padding: 1.5rem; /* 24px */
        /// }
        /// ```
        P6 => "scroll-p-6",
        /// ```css
        /// {
        ///     scroll-padding-left: 1.5rem; /* 24px */
        ///     scroll-padding-right: 1.5rem; /* 24px */
        /// }
        /// ```
        Px6 => "scroll-px-6",
        /// ```css
        /// {
        ///     scroll-padding-top: 1.5rem; /* 24px */
        ///     scroll-padding-bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        Py6 => "scroll-py-6",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 1.5rem; /* 24px */
        /// }
        /// ```
        Ps6 => "scroll-ps-6",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 1.5rem; /* 24px */
        /// }
        /// ```
        Pe6 => "scroll-pe-6",
        /// ```css
        /// {
        ///     scroll-padding-top: 1.5rem; /* 24px */
        /// }
        /// ```
        Pt6 => "scroll-pt-6",
        /// ```css
        /// {
        ///     scroll-padding-right: 1.5rem; /* 24px */
        /// }
        /// ```
        Pr6 => "scroll-pr-6",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        Pb6 => "scroll-pb-6",
        /// ```css
        /// {
        ///     scroll-padding-left: 1.5rem; /* 24px */
        /// }
        /// ```
        Pl6 => "scroll-pl-6",
        /// ```css
        /// {
        ///     scroll-padding: 1.75rem; /* 28px */
        /// }
        /// ```
        P7 => "scroll-p-7",
        /// ```css
        /// {
        ///     scroll-padding-left: 1.75rem; /* 28px */
        ///     scroll-padding-right: 1.75rem; /* 28px */
        /// }
        /// ```
        Px7 => "scroll-px-7",
        /// ```css
        /// {
        ///     scroll-padding-top: 1.75rem; /* 28px */
        ///     scroll-padding-bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        Py7 => "scroll-py-7",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 1.75rem; /* 28px */
        /// }
        /// ```
        Ps7 => "scroll-ps-7",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 1.75rem; /* 28px */
        /// }
        /// ```
        Pe7 => "scroll-pe-7",
        /// ```css
        /// {
        ///     scroll-padding-top: 1.75rem; /* 28px */
        /// }
        /// ```
        Pt7 => "scroll-pt-7",
        /// ```css
        /// {
        ///     scroll-padding-right: 1.75rem; /* 28px */
        /// }
        /// ```
        Pr7 => "scroll-pr-7",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        Pb7 => "scroll-pb-7",
        /// ```css
        /// {
        ///     scroll-padding-left: 1.75rem; /* 28px */
        /// }
        /// ```
        Pl7 => "scroll-pl-7",
        /// ```css
        /// {
        ///     scroll-padding: 2rem; /* 32px */
        /// }
        /// ```
        P8 => "scroll-p-8",
        /// ```css
        /// {
        ///     scroll-padding-left: 2rem; /* 32px */
        ///     scroll-padding-right: 2rem; /* 32px */
        /// }
        /// ```
        Px8 => "scroll-px-8",
        /// ```css
        /// {
        ///     scroll-padding-top: 2rem; /* 32px */
        ///     scroll-padding-bottom: 2rem; /* 32px */
        /// }
        /// ```
        Py8 => "scroll-py-8",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 2rem; /* 32px */
        /// }
        /// ```
        Ps8 => "scroll-ps-8",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 2rem; /* 32px */
        /// }
        /// ```
        Pe8 => "scroll-pe-8",
        /// ```css
        /// {
        ///     scroll-padding-top: 2rem; /* 32px */
        /// }
        /// ```
        Pt8 => "scroll-pt-8",
        /// ```css
        /// {
        ///     scroll-padding-right: 2rem; /* 32px */
        /// }
        /// ```
        Pr8 => "scroll-pr-8",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 2rem; /* 32px */
        /// }
        /// ```
        Pb8 => "scroll-pb-8",
        /// ```css
        /// {
        ///     scroll-padding-left: 2rem; /* 32px */
        /// }
        /// ```
        Pl8 => "scroll-pl-8",
        /// ```css
        /// {
        ///     scroll-padding: 2.25rem; /* 36px */
        /// }
        /// ```
        P9 => "scroll-p-9",
        /// ```css
        /// {
        ///     scroll-padding-left: 2.25rem; /* 36px */
        ///     scroll-padding-right: 2.25rem; /* 36px */
        /// }
        /// ```
        Px9 => "scroll-px-9",
        /// ```css
        /// {
        ///     scroll-padding-top: 2.25rem; /* 36px */
        ///     scroll-padding-bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        Py9 => "scroll-py-9",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 2.25rem; /* 36px */
        /// }
        /// ```
        Ps9 => "scroll-ps-9",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 2.25rem; /* 36px */
        /// }
        /// ```
        Pe9 => "scroll-pe-9",
        /// ```css
        /// {
        ///     scroll-padding-top: 2.25rem; /* 36px */
        /// }
        /// ```
        Pt9 => "scroll-pt-9",
        /// ```css
        /// {
        ///     scroll-padding-right: 2.25rem; /* 36px */
        /// }
        /// ```
        Pr9 => "scroll-pr-9",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        Pb9 => "scroll-pb-9",
        /// ```css
        /// {
        ///     scroll-padding-left: 2.25rem; /* 36px */
        /// }
        /// ```
        Pl9 => "scroll-pl-9",
        /// ```css
        /// {
        ///     scroll-padding: 2.5rem; /* 40px */
        /// }
        /// ```
        P10 => "scroll-p-10",
        /// ```css
        /// {
        ///     scroll-padding-left: 2.5rem; /* 40px */
        ///     scroll-padding-right: 2.5rem; /* 40px */
        /// }
        /// ```
        Px10 => "scroll-px-10",
        /// ```css
        /// {
        ///     scroll-padding-top: 2.5rem; /* 40px */
        ///     scroll-padding-bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        Py10 => "scroll-py-10",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 2.5rem; /* 40px */
        /// }
        /// ```
        Ps10 => "scroll-ps-10",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 2.5rem; /* 40px */
        /// }
        /// ```
        Pe10 => "scroll-pe-10",
        /// ```css
        /// {
        ///     scroll-padding-top: 2.5rem; /* 40px */
        /// }
        /// ```
        Pt10 => "scroll-pt-10",
        /// ```css
        /// {
        ///     scroll-padding-right: 2.5rem; /* 40px */
        /// }
        /// ```
        Pr10 => "scroll-pr-10",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        Pb10 => "scroll-pb-10",
        /// ```css
        /// {
        ///     scroll-padding-left: 2.5rem; /* 40px */
        /// }
        /// ```
        Pl10 => "scroll-pl-10",
        /// ```css
        /// {
        ///     scroll-padding: 2.75rem; /* 44px */
        /// }
        /// ```
        P11 => "scroll-p-11",
        /// ```css
        /// {
        ///     scroll-padding-left: 2.75rem; /* 44px */
        ///     scroll-padding-right: 2.75rem; /* 44px */
        /// }
        /// ```
        Px11 => "scroll-px-11",
        /// ```css
        /// {
        ///     scroll-padding-top: 2.75rem; /* 44px */
        ///     scroll-padding-bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        Py11 => "scroll-py-11",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 2.75rem; /* 44px */
        /// }
        /// ```
        Ps11 => "scroll-ps-11",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 2.75rem; /* 44px */
        /// }
        /// ```
        Pe11 => "scroll-pe-11",
        /// ```css
        /// {
        ///     scroll-padding-top: 2.75rem; /* 44px */
        /// }
        /// ```
        Pt11 => "scroll-pt-11",
        /// ```css
        /// {
        ///     scroll-padding-right: 2.75rem; /* 44px */
        /// }
        /// ```
        Pr11 => "scroll-pr-11",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        Pb11 => "scroll-pb-11",
        /// ```css
        /// {
        ///     scroll-padding-left: 2.75rem; /* 44px */
        /// }
        /// ```
        Pl11 => "scroll-pl-11",
        /// ```css
        /// {
        ///     scroll-padding: 3rem; /* 48px */
        /// }
        /// ```
        P12 => "scroll-p-12",
        /// ```css
        /// {
        ///     scroll-padding-left: 3rem; /* 48px */
        ///     scroll-padding-right: 3rem; /* 48px */
        /// }
        /// ```
        Px12 => "scroll-px-12",
        /// ```css
        /// {
        ///     scroll-padding-top: 3rem; /* 48px */
        ///     scroll-padding-bottom: 3rem; /* 48px */
        /// }
        /// ```
        Py12 => "scroll-py-12",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 3rem; /* 48px */
        /// }
        /// ```
        Ps12 => "scroll-ps-12",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 3rem; /* 48px */
        /// }
        /// ```
        Pe12 => "scroll-pe-12",
        /// ```css
        /// {
        ///     scroll-padding-top: 3rem; /* 48px */
        /// }
        /// ```
        Pt12 => "scroll-pt-12",
        /// ```css
        /// {
        ///     scroll-padding-right: 3rem; /* 48px */
        /// }
        /// ```
        Pr12 => "scroll-pr-12",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 3rem; /* 48px */
        /// }
        /// ```
        Pb12 => "scroll-pb-12",
        /// ```css
        /// {
        ///     scroll-padding-left: 3rem; /* 48px */
        /// }
        /// ```
        Pl12 => "scroll-pl-12",
        /// ```css
        /// {
        ///     scroll-padding: 3.5rem; /* 56px */
        /// }
        /// ```
        P14 => "scroll-p-14",
        /// ```css
        /// {
        ///     scroll-padding-left: 3.5rem; /* 56px */
        ///     scroll-padding-right: 3.5rem; /* 56px */
        /// }
        /// ```
        Px14 => "scroll-px-14",
        /// ```css
        /// {
        ///     scroll-padding-top: 3.5rem; /* 56px */
        ///     scroll-padding-bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        Py14 => "scroll-py-14",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 3.5rem; /* 56px */
        /// }
        /// ```
        Ps14 => "scroll-ps-14",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 3.5rem; /* 56px */
        /// }
        /// ```
        Pe14 => "scroll-pe-14",
        /// ```css
        /// {
        ///     scroll-padding-top: 3.5rem; /* 56px */
        /// }
        /// ```
        Pt14 => "scroll-pt-14",
        /// ```css
        /// {
        ///     scroll-padding-right: 3.5rem; /* 56px */
        /// }
        /// ```
        Pr14 => "scroll-pr-14",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        Pb14 => "scroll-pb-14",
        /// ```css
        /// {
        ///     scroll-padding-left: 3.5rem; /* 56px */
        /// }
        /// ```
        Pl14 => "scroll-pl-14",
        /// ```css
        /// {
        ///     scroll-padding: 4rem; /* 64px */
        /// }
        /// ```
        P16 => "scroll-p-16",
        /// ```css
        /// {
        ///     scroll-padding-left: 4rem; /* 64px */
        ///     scroll-padding-right: 4rem; /* 64px */
        /// }
        /// ```
        Px16 => "scroll-px-16",
        /// ```css
        /// {
        ///     scroll-padding-top: 4rem; /* 64px */
        ///     scroll-padding-bottom: 4rem; /* 64px */
        /// }
        /// ```
        Py16 => "scroll-py-16",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 4rem; /* 64px */
        /// }
        /// ```
        Ps16 => "scroll-ps-16",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 4rem; /* 64px */
        /// }
        /// ```
        Pe16 => "scroll-pe-16",
        /// ```css
        /// {
        ///     scroll-padding-top: 4rem; /* 64px */
        /// }
        /// ```
        Pt16 => "scroll-pt-16",
        /// ```css
        /// {
        ///     scroll-padding-right: 4rem; /* 64px */
        /// }
        /// ```
        Pr16 => "scroll-pr-16",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 4rem; /* 64px */
        /// }
        /// ```
        Pb16 => "scroll-pb-16",
        /// ```css
        /// {
        ///     scroll-padding-left: 4rem; /* 64px */
        /// }
        /// ```
        Pl16 => "scroll-pl-16",
        /// ```css
        /// {
        ///     scroll-padding: 5rem; /* 80px */
        /// }
        /// ```
        P20 => "scroll-p-20",
        /// ```css
        /// {
        ///     scroll-padding-left: 5rem; /* 80px */
        ///     scroll-padding-right: 5rem; /* 80px */
        /// }
        /// ```
        Px20 => "scroll-px-20",
        /// ```css
        /// {
        ///     scroll-padding-top: 5rem; /* 80px */
        ///     scroll-padding-bottom: 5rem; /* 80px */
        /// }
        /// ```
        Py20 => "scroll-py-20",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 5rem; /* 80px */
        /// }
        /// ```
        Ps20 => "scroll-ps-20",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 5rem; /* 80px */
        /// }
        /// ```
        Pe20 => "scroll-pe-20",
        /// ```css
        /// {
        ///     scroll-padding-top: 5rem; /* 80px */
        /// }
        /// ```
        Pt20 => "scroll-pt-20",
        /// ```css
        /// {
        ///     scroll-padding-right: 5rem; /* 80px */
        /// }
        /// ```
        Pr20 => "scroll-pr-20",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 5rem; /* 80px */
        /// }
        /// ```
        Pb20 => "scroll-pb-20",
        /// ```css
        /// {
        ///     scroll-padding-left: 5rem; /* 80px */
        /// }
        /// ```
        Pl20 => "scroll-pl-20",
        /// ```css
        /// {
        ///     scroll-padding: 6rem; /* 96px */
        /// }
        /// ```
        P24 => "scroll-p-24",
        /// ```css
        /// {
        ///     scroll-padding-left: 6rem; /* 96px */
        ///     scroll-padding-right: 6rem; /* 96px */
        /// }
        /// ```
        Px24 => "scroll-px-24",
        /// ```css
        /// {
        ///     scroll-padding-top: 6rem; /* 96px */
        ///     scroll-padding-bottom: 6rem; /* 96px */
        /// }
        /// ```
        Py24 => "scroll-py-24",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 6rem; /* 96px */
        /// }
        /// ```
        Ps24 => "scroll-ps-24",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 6rem; /* 96px */
        /// }
        /// ```
        Pe24 => "scroll-pe-24",
        /// ```css
        /// {
        ///     scroll-padding-top: 6rem; /* 96px */
        /// }
        /// ```
        Pt24 => "scroll-pt-24",
        /// ```css
        /// {
        ///     scroll-padding-right: 6rem; /* 96px */
        /// }
        /// ```
        Pr24 => "scroll-pr-24",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 6rem; /* 96px */
        /// }
        /// ```
        Pb24 => "scroll-pb-24",
        /// ```css
        /// {
        ///     scroll-padding-left: 6rem; /* 96px */
        /// }
        /// ```
        Pl24 => "scroll-pl-24",
        /// ```css
        /// {
        ///     scroll-padding: 7rem; /* 112px */
        /// }
        /// ```
        P28 => "scroll-p-28",
        /// ```css
        /// {
        ///     scroll-padding-left: 7rem; /* 112px */
        ///     scroll-padding-right: 7rem; /* 112px */
        /// }
        /// ```
        Px28 => "scroll-px-28",
        /// ```css
        /// {
        ///     scroll-padding-top: 7rem; /* 112px */
        ///     scroll-padding-bottom: 7rem; /* 112px */
        /// }
        /// ```
        Py28 => "scroll-py-28",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 7rem; /* 112px */
        /// }
        /// ```
        Ps28 => "scroll-ps-28",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 7rem; /* 112px */
        /// }
        /// ```
        Pe28 => "scroll-pe-28",
        /// ```css
        /// {
        ///     scroll-padding-top: 7rem; /* 112px */
        /// }
        /// ```
        Pt28 => "scroll-pt-28",
        /// ```css
        /// {
        ///     scroll-padding-right: 7rem; /* 112px */
        /// }
        /// ```
        Pr28 => "scroll-pr-28",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 7rem; /* 112px */
        /// }
        /// ```
        Pb28 => "scroll-pb-28",
        /// ```css
        /// {
        ///     scroll-padding-left: 7rem; /* 112px */
        /// }
        /// ```
        Pl28 => "scroll-pl-28",
        /// ```css
        /// {
        ///     scroll-padding: 8rem; /* 128px */
        /// }
        /// ```
        P32 => "scroll-p-32",
        /// ```css
        /// {
        ///     scroll-padding-left: 8rem; /* 128px */
        ///     scroll-padding-right: 8rem; /* 128px */
        /// }
        /// ```
        Px32 => "scroll-px-32",
        /// ```css
        /// {
        ///     scroll-padding-top: 8rem; /* 128px */
        ///     scroll-padding-bottom: 8rem; /* 128px */
        /// }
        /// ```
        Py32 => "scroll-py-32",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 8rem; /* 128px */
        /// }
        /// ```
        Ps32 => "scroll-ps-32",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 8rem; /* 128px */
        /// }
        /// ```
        Pe32 => "scroll-pe-32",
        /// ```css
        /// {
        ///     scroll-padding-top: 8rem; /* 128px */
        /// }
        /// ```
        Pt32 => "scroll-pt-32",
        /// ```css
        /// {
        ///     scroll-padding-right: 8rem; /* 128px */
        /// }
        /// ```
        Pr32 => "scroll-pr-32",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 8rem; /* 128px */
        /// }
        /// ```
        Pb32 => "scroll-pb-32",
        /// ```css
        /// {
        ///     scroll-padding-left: 8rem; /* 128px */
        /// }
        /// ```
        Pl32 => "scroll-pl-32",
        /// ```css
        /// {
        ///     scroll-padding: 9rem; /* 144px */
        /// }
        /// ```
        P36 => "scroll-p-36",
        /// ```css
        /// {
        ///     scroll-padding-left: 9rem; /* 144px */
        ///     scroll-padding-right: 9rem; /* 144px */
        /// }
        /// ```
        Px36 => "scroll-px-36",
        /// ```css
        /// {
        ///     scroll-padding-top: 9rem; /* 144px */
        ///     scroll-padding-bottom: 9rem; /* 144px */
        /// }
        /// ```
        Py36 => "scroll-py-36",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 9rem; /* 144px */
        /// }
        /// ```
        Ps36 => "scroll-ps-36",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 9rem; /* 144px */
        /// }
        /// ```
        Pe36 => "scroll-pe-36",
        /// ```css
        /// {
        ///     scroll-padding-top: 9rem; /* 144px */
        /// }
        /// ```
        Pt36 => "scroll-pt-36",
        /// ```css
        /// {
        ///     scroll-padding-right: 9rem; /* 144px */
        /// }
        /// ```
        Pr36 => "scroll-pr-36",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 9rem; /* 144px */
        /// }
        /// ```
        Pb36 => "scroll-pb-36",
        /// ```css
        /// {
        ///     scroll-padding-left: 9rem; /* 144px */
        /// }
        /// ```
        Pl36 => "scroll-pl-36",
        /// ```css
        /// {
        ///     scroll-padding: 10rem; /* 160px */
        /// }
        /// ```
        P40 => "scroll-p-40",
        /// ```css
        /// {
        ///     scroll-padding-left: 10rem; /* 160px */
        ///     scroll-padding-right: 10rem; /* 160px */
        /// }
        /// ```
        Px40 => "scroll-px-40",
        /// ```css
        /// {
        ///     scroll-padding-top: 10rem; /* 160px */
        ///     scroll-padding-bottom: 10rem; /* 160px */
        /// }
        /// ```
        Py40 => "scroll-py-40",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 10rem; /* 160px */
        /// }
        /// ```
        Ps40 => "scroll-ps-40",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 10rem; /* 160px */
        /// }
        /// ```
        Pe40 => "scroll-pe-40",
        /// ```css
        /// {
        ///     scroll-padding-top: 10rem; /* 160px */
        /// }
        /// ```
        Pt40 => "scroll-pt-40",
        /// ```css
        /// {
        ///     scroll-padding-right: 10rem; /* 160px */
        /// }
        /// ```
        Pr40 => "scroll-pr-40",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 10rem; /* 160px */
        /// }
        /// ```
        Pb40 => "scroll-pb-40",
        /// ```css
        /// {
        ///     scroll-padding-left: 10rem; /* 160px */
        /// }
        /// ```
        Pl40 => "scroll-pl-40",
        /// ```css
        /// {
        ///     scroll-padding: 11rem; /* 176px */
        /// }
        /// ```
        P44 => "scroll-p-44",
        /// ```css
        /// {
        ///     scroll-padding-left: 11rem; /* 176px */
        ///     scroll-padding-right: 11rem; /* 176px */
        /// }
        /// ```
        Px44 => "scroll-px-44",
        /// ```css
        /// {
        ///     scroll-padding-top: 11rem; /* 176px */
        ///     scroll-padding-bottom: 11rem; /* 176px */
        /// }
        /// ```
        Py44 => "scroll-py-44",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 11rem; /* 176px */
        /// }
        /// ```
        Ps44 => "scroll-ps-44",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 11rem; /* 176px */
        /// }
        /// ```
        Pe44 => "scroll-pe-44",
        /// ```css
        /// {
        ///     scroll-padding-top: 11rem; /* 176px */
        /// }
        /// ```
        Pt44 => "scroll-pt-44",
        /// ```css
        /// {
        ///     scroll-padding-right: 11rem; /* 176px */
        /// }
        /// ```
        Pr44 => "scroll-pr-44",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 11rem; /* 176px */
        /// }
        /// ```
        Pb44 => "scroll-pb-44",
        /// ```css
        /// {
        ///     scroll-padding-left: 11rem; /* 176px */
        /// }
        /// ```
        Pl44 => "scroll-pl-44",
        /// ```css
        /// {
        ///     scroll-padding: 12rem; /* 192px */
        /// }
        /// ```
        P48 => "scroll-p-48",
        /// ```css
        /// {
        ///     scroll-padding-left: 12rem; /* 192px */
        ///     scroll-padding-right: 12rem; /* 192px */
        /// }
        /// ```
        Px48 => "scroll-px-48",
        /// ```css
        /// {
        ///     scroll-padding-top: 12rem; /* 192px */
        ///     scroll-padding-bottom: 12rem; /* 192px */
        /// }
        /// ```
        Py48 => "scroll-py-48",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 12rem; /* 192px */
        /// }
        /// ```
        Ps48 => "scroll-ps-48",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 12rem; /* 192px */
        /// }
        /// ```
        Pe48 => "scroll-pe-48",
        /// ```css
        /// {
        ///     scroll-padding-top: 12rem; /* 192px */
        /// }
        /// ```
        Pt48 => "scroll-pt-48",
        /// ```css
        /// {
        ///     scroll-padding-right: 12rem; /* 192px */
        /// }
        /// ```
        Pr48 => "scroll-pr-48",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 12rem; /* 192px */
        /// }
        /// ```
        Pb48 => "scroll-pb-48",
        /// ```css
        /// {
        ///     scroll-padding-left: 12rem; /* 192px */
        /// }
        /// ```
        Pl48 => "scroll-pl-48",
        /// ```css
        /// {
        ///     scroll-padding: 13rem; /* 208px */
        /// }
        /// ```
        P52 => "scroll-p-52",
        /// ```css
        /// {
        ///     scroll-padding-left: 13rem; /* 208px */
        ///     scroll-padding-right: 13rem; /* 208px */
        /// }
        /// ```
        Px52 => "scroll-px-52",
        /// ```css
        /// {
        ///     scroll-padding-top: 13rem; /* 208px */
        ///     scroll-padding-bottom: 13rem; /* 208px */
        /// }
        /// ```
        Py52 => "scroll-py-52",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 13rem; /* 208px */
        /// }
        /// ```
        Ps52 => "scroll-ps-52",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 13rem; /* 208px */
        /// }
        /// ```
        Pe52 => "scroll-pe-52",
        /// ```css
        /// {
        ///     scroll-padding-top: 13rem; /* 208px */
        /// }
        /// ```
        Pt52 => "scroll-pt-52",
        /// ```css
        /// {
        ///     scroll-padding-right: 13rem; /* 208px */
        /// }
        /// ```
        Pr52 => "scroll-pr-52",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 13rem; /* 208px */
        /// }
        /// ```
        Pb52 => "scroll-pb-52",
        /// ```css
        /// {
        ///     scroll-padding-left: 13rem; /* 208px */
        /// }
        /// ```
        Pl52 => "scroll-pl-52",
        /// ```css
        /// {
        ///     scroll-padding: 14rem; /* 224px */
        /// }
        /// ```
        P56 => "scroll-p-56",
        /// ```css
        /// {
        ///     scroll-padding-left: 14rem; /* 224px */
        ///     scroll-padding-right: 14rem; /* 224px */
        /// }
        /// ```
        Px56 => "scroll-px-56",
        /// ```css
        /// {
        ///     scroll-padding-top: 14rem; /* 224px */
        ///     scroll-padding-bottom: 14rem; /* 224px */
        /// }
        /// ```
        Py56 => "scroll-py-56",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 14rem; /* 224px */
        /// }
        /// ```
        Ps56 => "scroll-ps-56",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 14rem; /* 224px */
        /// }
        /// ```
        Pe56 => "scroll-pe-56",
        /// ```css
        /// {
        ///     scroll-padding-top: 14rem; /* 224px */
        /// }
        /// ```
        Pt56 => "scroll-pt-56",
        /// ```css
        /// {
        ///     scroll-padding-right: 14rem; /* 224px */
        /// }
        /// ```
        Pr56 => "scroll-pr-56",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 14rem; /* 224px */
        /// }
        /// ```
        Pb56 => "scroll-pb-56",
        /// ```css
        /// {
        ///     scroll-padding-left: 14rem; /* 224px */
        /// }
        /// ```
        Pl56 => "scroll-pl-56",
        /// ```css
        /// {
        ///     scroll-padding: 15rem; /* 240px */
        /// }
        /// ```
        P60 => "scroll-p-60",
        /// ```css
        /// {
        ///     scroll-padding-left: 15rem; /* 240px */
        ///     scroll-padding-right: 15rem; /* 240px */
        /// }
        /// ```
        Px60 => "scroll-px-60",
        /// ```css
        /// {
        ///     scroll-padding-top: 15rem; /* 240px */
        ///     scroll-padding-bottom: 15rem; /* 240px */
        /// }
        /// ```
        Py60 => "scroll-py-60",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 15rem; /* 240px */
        /// }
        /// ```
        Ps60 => "scroll-ps-60",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 15rem; /* 240px */
        /// }
        /// ```
        Pe60 => "scroll-pe-60",
        /// ```css
        /// {
        ///     scroll-padding-top: 15rem; /* 240px */
        /// }
        /// ```
        Pt60 => "scroll-pt-60",
        /// ```css
        /// {
        ///     scroll-padding-right: 15rem; /* 240px */
        /// }
        /// ```
        Pr60 => "scroll-pr-60",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 15rem; /* 240px */
        /// }
        /// ```
        Pb60 => "scroll-pb-60",
        /// ```css
        /// {
        ///     scroll-padding-left: 15rem; /* 240px */
        /// }
        /// ```
        Pl60 => "scroll-pl-60",
        /// ```css
        /// {
        ///     scroll-padding: 16rem; /* 256px */
        /// }
        /// ```
        P64 => "scroll-p-64",
        /// ```css
        /// {
        ///     scroll-padding-left: 16rem; /* 256px */
        ///     scroll-padding-right: 16rem; /* 256px */
        /// }
        /// ```
        Px64 => "scroll-px-64",
        /// ```css
        /// {
        ///     scroll-padding-top: 16rem; /* 256px */
        ///     scroll-padding-bottom: 16rem; /* 256px */
        /// }
        /// ```
        Py64 => "scroll-py-64",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 16rem; /* 256px */
        /// }
        /// ```
        Ps64 => "scroll-ps-64",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 16rem; /* 256px */
        /// }
        /// ```
        Pe64 => "scroll-pe-64",
        /// ```css
        /// {
        ///     scroll-padding-top: 16rem; /* 256px */
        /// }
        /// ```
        Pt64 => "scroll-pt-64",
        /// ```css
        /// {
        ///     scroll-padding-right: 16rem; /* 256px */
        /// }
        /// ```
        Pr64 => "scroll-pr-64",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 16rem; /* 256px */
        /// }
        /// ```
        Pb64 => "scroll-pb-64",
        /// ```css
        /// {
        ///     scroll-padding-left: 16rem; /* 256px */
        /// }
        /// ```
        Pl64 => "scroll-pl-64",
        /// ```css
        /// {
        ///     scroll-padding: 18rem; /* 288px */
        /// }
        /// ```
        P72 => "scroll-p-72",
        /// ```css
        /// {
        ///     scroll-padding-left: 18rem; /* 288px */
        ///     scroll-padding-right: 18rem; /* 288px */
        /// }
        /// ```
        Px72 => "scroll-px-72",
        /// ```css
        /// {
        ///     scroll-padding-top: 18rem; /* 288px */
        ///     scroll-padding-bottom: 18rem; /* 288px */
        /// }
        /// ```
        Py72 => "scroll-py-72",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 18rem; /* 288px */
        /// }
        /// ```
        Ps72 => "scroll-ps-72",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 18rem; /* 288px */
        /// }
        /// ```
        Pe72 => "scroll-pe-72",
        /// ```css
        /// {
        ///     scroll-padding-top: 18rem; /* 288px */
        /// }
        /// ```
        Pt72 => "scroll-pt-72",
        /// ```css
        /// {
        ///     scroll-padding-right: 18rem; /* 288px */
        /// }
        /// ```
        Pr72 => "scroll-pr-72",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 18rem; /* 288px */
        /// }
        /// ```
        Pb72 => "scroll-pb-72",
        /// ```css
        /// {
        ///     scroll-padding-left: 18rem; /* 288px */
        /// }
        /// ```
        Pl72 => "scroll-pl-72",
        /// ```css
        /// {
        ///     scroll-padding: 20rem; /* 320px */
        /// }
        /// ```
        P80 => "scroll-p-80",
        /// ```css
        /// {
        ///     scroll-padding-left: 20rem; /* 320px */
        ///     scroll-padding-right: 20rem; /* 320px */
        /// }
        /// ```
        Px80 => "scroll-px-80",
        /// ```css
        /// {
        ///     scroll-padding-top: 20rem; /* 320px */
        ///     scroll-padding-bottom: 20rem; /* 320px */
        /// }
        /// ```
        Py80 => "scroll-py-80",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 20rem; /* 320px */
        /// }
        /// ```
        Ps80 => "scroll-ps-80",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 20rem; /* 320px */
        /// }
        /// ```
        Pe80 => "scroll-pe-80",
        /// ```css
        /// {
        ///     scroll-padding-top: 20rem; /* 320px */
        /// }
        /// ```
        Pt80 => "scroll-pt-80",
        /// ```css
        /// {
        ///     scroll-padding-right: 20rem; /* 320px */
        /// }
        /// ```
        Pr80 => "scroll-pr-80",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 20rem; /* 320px */
        /// }
        /// ```
        Pb80 => "scroll-pb-80",
        /// ```css
        /// {
        ///     scroll-padding-left: 20rem; /* 320px */
        /// }
        /// ```
        Pl80 => "scroll-pl-80",
        /// ```css
        /// {
        ///     scroll-padding: 24rem; /* 384px */
        /// }
        /// ```
        P96 => "scroll-p-96",
        /// ```css
        /// {
        ///     scroll-padding-left: 24rem; /* 384px */
        ///     scroll-padding-right: 24rem; /* 384px */
        /// }
        /// ```
        Px96 => "scroll-px-96",
        /// ```css
        /// {
        ///     scroll-padding-top: 24rem; /* 384px */
        ///     scroll-padding-bottom: 24rem; /* 384px */
        /// }
        /// ```
        Py96 => "scroll-py-96",
        /// ```css
        /// {
        ///     scroll-padding-inline-start: 24rem; /* 384px */
        /// }
        /// ```
        Ps96 => "scroll-ps-96",
        /// ```css
        /// {
        ///     scroll-padding-inline-end: 24rem; /* 384px */
        /// }
        /// ```
        Pe96 => "scroll-pe-96",
        /// ```css
        /// {
        ///     scroll-padding-top: 24rem; /* 384px */
        /// }
        /// ```
        Pt96 => "scroll-pt-96",
        /// ```css
        /// {
        ///     scroll-padding-right: 24rem; /* 384px */
        /// }
        /// ```
        Pr96 => "scroll-pr-96",
        /// ```css
        /// {
        ///     scroll-padding-bottom: 24rem; /* 384px */
        /// }
        /// ```
        Pb96 => "scroll-pb-96",
        /// ```css
        /// {
        ///     scroll-padding-left: 24rem; /* 384px */
        /// }
        /// ```
        Pl96 => "scroll-pl-96",
    }
    /// Utilities for controlling the scroll snap alignment of an element.
    ///
    /// <https://tailwindcss.com/docs/scroll-snap-align>
    ScrollSnapAlign {
        /// ```css
        /// {
        ///     scroll-snap-align: start;
        /// }
        /// ```
        Start => "snap-start",
        /// ```css
        /// {
        ///     scroll-snap-align: end;
        /// }
        /// ```
        End => "snap-end",
        /// ```css
        /// {
        ///     scroll-snap-align: center;
        /// }
        /// ```
        Center => "snap-center",
        /// ```css
        /// {
        ///     scroll-snap-align: none;
        /// }
        /// ```
        AlignNone => "snap-align-none",
    }
    /// Utilities for controlling whether you can skip past possible snap positions.
    ///
    /// <https://tailwindcss.com/docs/scroll-snap-stop>
    ScrollSnapStop {
        /// ```css
        /// {
        ///     scroll-snap-stop: normal;
        /// }
        /// ```
        Normal => "snap-normal",
        /// ```css
        /// {
        ///     scroll-snap-stop: always;
        /// }
        /// ```
        Always => "snap-always",
    }
    /// Utilities for controlling how strictly snap points are enforced in a snap container.
    ///
    /// <https://tailwindcss.com/docs/scroll-snap-type>
    ScrollSnapType {
        /// ```css
        /// {
        ///     scroll-snap-type: none;
        /// }
        /// ```
        None => "snap-none",
        /// ```css
        /// {
        ///     scroll-snap-type: x var(--tw-scroll-snap-strictness);
        /// }
        /// ```
        X => "snap-x",
        /// ```css
        /// {
        ///     scroll-snap-type: y var(--tw-scroll-snap-strictness);
        /// }
        /// ```
        Y => "snap-y",
        /// ```css
        /// {
        ///     scroll-snap-type: both var(--tw-scroll-snap-strictness);
        /// }
        /// ```
        Both => "snap-both",
        /// ```css
        /// {
        ///     --tw-scroll-snap-strictness: mandatory;
        /// }
        /// ```
        Mandatory => "snap-mandatory",
        /// ```css
        /// {
        ///     --tw-scroll-snap-strictness: proximity;
        /// }
        /// ```
        Proximity => "snap-proximity",
    }
    /// Utilities for controlling how an element can be scrolled and zoomed on touchscreens.
    ///
    /// <https://tailwindcss.com/docs/touch-action>
    TouchAction {
        /// ```css
        /// {
        ///     touch-action: auto;
        /// }
        /// ```
        Auto => "touch-auto",
        /// ```css
        /// {
        ///     touch-action: none;
        /// }
        /// ```
        None => "touch-none",
        /// ```css
        /// {
        ///     touch-action: pan-x;
        /// }
        /// ```
        PanX => "touch-pan-x",
        /// ```css
        /// {
        ///     touch-action: pan-left;
        /// }
        /// ```
        PanLeft => "touch-pan-left",
        /// ```css
        /// {
        ///     touch-action: pan-right;
        /// }
        /// ```
        PanRight => "touch-pan-right",
        /// ```css
        /// {
        ///     touch-action: pan-y;
        /// }
        /// ```
        PanY => "touch-pan-y",
        /// ```css
        /// {
        ///     touch-action: pan-up;
        /// }
        /// ```
        PanUp => "touch-pan-up",
        /// ```css
        /// {
        ///     touch-action: pan-down;
        /// }
        /// ```
        PanDown => "touch-pan-down",
        /// ```css
        /// {
        ///     touch-action: pinch-zoom;
        /// }
        /// ```
        PinchZoom => "touch-pinch-zoom",
        /// ```css
        /// {
        ///     touch-action: manipulation;
        /// }
        /// ```
        Manipulation => "touch-manipulation",
    }
    /// Utilities for controlling whether the user can select text in an element.
    ///
    /// <https://tailwindcss.com/docs/user-select>
    UserSelect {
        /// ```css
        /// {
        ///     user-select: none;
        /// }
        /// ```
        None => "select-none",
        /// ```css
        /// {
        ///     user-select: text;
        /// }
        /// ```
        Text => "select-text",
        /// ```css
        /// {
        ///     user-select: all;
        /// }
        /// ```
        All => "select-all",
        /// ```css
        /// {
        ///     user-select: auto;
        /// }
        /// ```
        Auto => "select-auto",
    }
    /// Utilities for optimizing upcoming animations of elements that are expected to change.
    ///
    /// <https://tailwindcss.com/docs/will-change>
    WillChange {
        /// ```css
        /// {
        ///     will-change: auto;
        /// }
        /// ```
        Auto => "will-change-auto",
        /// ```css
        /// {
        ///     will-change: scroll-position;
        /// }
        /// ```
        Scroll => "will-change-scroll",
        /// ```css
        /// {
        ///     will-change: contents;
        /// }
        /// ```
        Contents => "will-change-contents",
        /// ```css
        /// {
        ///     will-change: transform;
        /// }
        /// ```
        Transform => "will-change-transform",
    }
);
