def_types!(
    /// Utilities for controlling how a background image behaves when scrolling.
    ///
    /// <https://tailwindcss.com/docs/background-attachment>
    BackgroundAttachment {
        /// ```css
        /// {
        ///     background-attachment: fixed;
        /// }
        /// ```
        Fixed => "bg-fixed",
        /// ```css
        /// {
        ///     background-attachment: local;
        /// }
        /// ```
        Local => "bg-local",
        /// ```css
        /// {
        ///     background-attachment: scroll;
        /// }
        /// ```
        Scroll => "bg-scroll",
    }
    /// Utilities for controlling the bounding box of an element's background.
    ///
    /// <https://tailwindcss.com/docs/background-clip>
    BackgroundClip {
        /// ```css
        /// {
        ///     background-clip: border-box;
        /// }
        /// ```
        Border => "bg-clip-border",
        /// ```css
        /// {
        ///     background-clip: padding-box;
        /// }
        /// ```
        Padding => "bg-clip-padding",
        /// ```css
        /// {
        ///     background-clip: content-box;
        /// }
        /// ```
        Content => "bg-clip-content",
        /// ```css
        /// {
        ///     background-clip: text;
        /// }
        /// ```
        Text => "bg-clip-text",
    }
    /// Utilities for controlling an element's background color.
    ///
    /// <https://tailwindcss.com/docs/background-color>
    BackgroundColor {
        /// ```css
        /// {
        ///     background-color: inherit;
        /// }
        /// ```
        Inherit => "bg-inherit",
        /// ```css
        /// {
        ///     background-color: currentColor;
        /// }
        /// ```
        Current => "bg-current",
        /// ```css
        /// {
        ///     background-color: transparent;
        /// }
        /// ```
        Transparent => "bg-transparent",
        /// ```css
        /// {
        ///     background-color: rgb(0 0 0);
        /// }
        /// ```
        Black => "bg-black",
        /// ```css
        /// {
        ///     background-color: rgb(255 255 255);
        /// }
        /// ```
        White => "bg-white",
        /// ```css
        /// {
        ///     background-color: rgb(248 250 252);
        /// }
        /// ```
        Slate50 => "bg-slate-50",
        /// ```css
        /// {
        ///     background-color: rgb(241 245 249);
        /// }
        /// ```
        Slate100 => "bg-slate-100",
        /// ```css
        /// {
        ///     background-color: rgb(226 232 240);
        /// }
        /// ```
        Slate200 => "bg-slate-200",
        /// ```css
        /// {
        ///     background-color: rgb(203 213 225);
        /// }
        /// ```
        Slate300 => "bg-slate-300",
        /// ```css
        /// {
        ///     background-color: rgb(148 163 184);
        /// }
        /// ```
        Slate400 => "bg-slate-400",
        /// ```css
        /// {
        ///     background-color: rgb(100 116 139);
        /// }
        /// ```
        Slate500 => "bg-slate-500",
        /// ```css
        /// {
        ///     background-color: rgb(71 85 105);
        /// }
        /// ```
        Slate600 => "bg-slate-600",
        /// ```css
        /// {
        ///     background-color: rgb(51 65 85);
        /// }
        /// ```
        Slate700 => "bg-slate-700",
        /// ```css
        /// {
        ///     background-color: rgb(30 41 59);
        /// }
        /// ```
        Slate800 => "bg-slate-800",
        /// ```css
        /// {
        ///     background-color: rgb(15 23 42);
        /// }
        /// ```
        Slate900 => "bg-slate-900",
        /// ```css
        /// {
        ///     background-color: rgb(2 6 23);
        /// }
        /// ```
        Slate950 => "bg-slate-950",
        /// ```css
        /// {
        ///     background-color: rgb(249 250 251);
        /// }
        /// ```
        Gray50 => "bg-gray-50",
        /// ```css
        /// {
        ///     background-color: rgb(243 244 246);
        /// }
        /// ```
        Gray100 => "bg-gray-100",
        /// ```css
        /// {
        ///     background-color: rgb(229 231 235);
        /// }
        /// ```
        Gray200 => "bg-gray-200",
        /// ```css
        /// {
        ///     background-color: rgb(209 213 219);
        /// }
        /// ```
        Gray300 => "bg-gray-300",
        /// ```css
        /// {
        ///     background-color: rgb(156 163 175);
        /// }
        /// ```
        Gray400 => "bg-gray-400",
        /// ```css
        /// {
        ///     background-color: rgb(107 114 128);
        /// }
        /// ```
        Gray500 => "bg-gray-500",
        /// ```css
        /// {
        ///     background-color: rgb(75 85 99);
        /// }
        /// ```
        Gray600 => "bg-gray-600",
        /// ```css
        /// {
        ///     background-color: rgb(55 65 81);
        /// }
        /// ```
        Gray700 => "bg-gray-700",
        /// ```css
        /// {
        ///     background-color: rgb(31 41 55);
        /// }
        /// ```
        Gray800 => "bg-gray-800",
        /// ```css
        /// {
        ///     background-color: rgb(17 24 39);
        /// }
        /// ```
        Gray900 => "bg-gray-900",
        /// ```css
        /// {
        ///     background-color: rgb(3 7 18);
        /// }
        /// ```
        Gray950 => "bg-gray-950",
        /// ```css
        /// {
        ///     background-color: rgb(250 250 250);
        /// }
        /// ```
        Zinc50 => "bg-zinc-50",
        /// ```css
        /// {
        ///     background-color: rgb(244 244 245);
        /// }
        /// ```
        Zinc100 => "bg-zinc-100",
        /// ```css
        /// {
        ///     background-color: rgb(228 228 231);
        /// }
        /// ```
        Zinc200 => "bg-zinc-200",
        /// ```css
        /// {
        ///     background-color: rgb(212 212 216);
        /// }
        /// ```
        Zinc300 => "bg-zinc-300",
        /// ```css
        /// {
        ///     background-color: rgb(161 161 170);
        /// }
        /// ```
        Zinc400 => "bg-zinc-400",
        /// ```css
        /// {
        ///     background-color: rgb(113 113 122);
        /// }
        /// ```
        Zinc500 => "bg-zinc-500",
        /// ```css
        /// {
        ///     background-color: rgb(82 82 91);
        /// }
        /// ```
        Zinc600 => "bg-zinc-600",
        /// ```css
        /// {
        ///     background-color: rgb(63 63 70);
        /// }
        /// ```
        Zinc700 => "bg-zinc-700",
        /// ```css
        /// {
        ///     background-color: rgb(39 39 42);
        /// }
        /// ```
        Zinc800 => "bg-zinc-800",
        /// ```css
        /// {
        ///     background-color: rgb(24 24 27);
        /// }
        /// ```
        Zinc900 => "bg-zinc-900",
        /// ```css
        /// {
        ///     background-color: rgb(9 9 11);
        /// }
        /// ```
        Zinc950 => "bg-zinc-950",
        /// ```css
        /// {
        ///     background-color: rgb(250 250 250);
        /// }
        /// ```
        Neutral50 => "bg-neutral-50",
        /// ```css
        /// {
        ///     background-color: rgb(245 245 245);
        /// }
        /// ```
        Neutral100 => "bg-neutral-100",
        /// ```css
        /// {
        ///     background-color: rgb(229 229 229);
        /// }
        /// ```
        Neutral200 => "bg-neutral-200",
        /// ```css
        /// {
        ///     background-color: rgb(212 212 212);
        /// }
        /// ```
        Neutral300 => "bg-neutral-300",
        /// ```css
        /// {
        ///     background-color: rgb(163 163 163);
        /// }
        /// ```
        Neutral400 => "bg-neutral-400",
        /// ```css
        /// {
        ///     background-color: rgb(115 115 115);
        /// }
        /// ```
        Neutral500 => "bg-neutral-500",
        /// ```css
        /// {
        ///     background-color: rgb(82 82 82);
        /// }
        /// ```
        Neutral600 => "bg-neutral-600",
        /// ```css
        /// {
        ///     background-color: rgb(64 64 64);
        /// }
        /// ```
        Neutral700 => "bg-neutral-700",
        /// ```css
        /// {
        ///     background-color: rgb(38 38 38);
        /// }
        /// ```
        Neutral800 => "bg-neutral-800",
        /// ```css
        /// {
        ///     background-color: rgb(23 23 23);
        /// }
        /// ```
        Neutral900 => "bg-neutral-900",
        /// ```css
        /// {
        ///     background-color: rgb(10 10 10);
        /// }
        /// ```
        Neutral950 => "bg-neutral-950",
        /// ```css
        /// {
        ///     background-color: rgb(250 250 249);
        /// }
        /// ```
        Stone50 => "bg-stone-50",
        /// ```css
        /// {
        ///     background-color: rgb(245 245 244);
        /// }
        /// ```
        Stone100 => "bg-stone-100",
        /// ```css
        /// {
        ///     background-color: rgb(231 229 228);
        /// }
        /// ```
        Stone200 => "bg-stone-200",
        /// ```css
        /// {
        ///     background-color: rgb(214 211 209);
        /// }
        /// ```
        Stone300 => "bg-stone-300",
        /// ```css
        /// {
        ///     background-color: rgb(168 162 158);
        /// }
        /// ```
        Stone400 => "bg-stone-400",
        /// ```css
        /// {
        ///     background-color: rgb(120 113 108);
        /// }
        /// ```
        Stone500 => "bg-stone-500",
        /// ```css
        /// {
        ///     background-color: rgb(87 83 78);
        /// }
        /// ```
        Stone600 => "bg-stone-600",
        /// ```css
        /// {
        ///     background-color: rgb(68 64 60);
        /// }
        /// ```
        Stone700 => "bg-stone-700",
        /// ```css
        /// {
        ///     background-color: rgb(41 37 36);
        /// }
        /// ```
        Stone800 => "bg-stone-800",
        /// ```css
        /// {
        ///     background-color: rgb(28 25 23);
        /// }
        /// ```
        Stone900 => "bg-stone-900",
        /// ```css
        /// {
        ///     background-color: rgb(12 10 9);
        /// }
        /// ```
        Stone950 => "bg-stone-950",
        /// ```css
        /// {
        ///     background-color: rgb(254 242 242);
        /// }
        /// ```
        Red50 => "bg-red-50",
        /// ```css
        /// {
        ///     background-color: rgb(254 226 226);
        /// }
        /// ```
        Red100 => "bg-red-100",
        /// ```css
        /// {
        ///     background-color: rgb(254 202 202);
        /// }
        /// ```
        Red200 => "bg-red-200",
        /// ```css
        /// {
        ///     background-color: rgb(252 165 165);
        /// }
        /// ```
        Red300 => "bg-red-300",
        /// ```css
        /// {
        ///     background-color: rgb(248 113 113);
        /// }
        /// ```
        Red400 => "bg-red-400",
        /// ```css
        /// {
        ///     background-color: rgb(239 68 68);
        /// }
        /// ```
        Red500 => "bg-red-500",
        /// ```css
        /// {
        ///     background-color: rgb(220 38 38);
        /// }
        /// ```
        Red600 => "bg-red-600",
        /// ```css
        /// {
        ///     background-color: rgb(185 28 28);
        /// }
        /// ```
        Red700 => "bg-red-700",
        /// ```css
        /// {
        ///     background-color: rgb(153 27 27);
        /// }
        /// ```
        Red800 => "bg-red-800",
        /// ```css
        /// {
        ///     background-color: rgb(127 29 29);
        /// }
        /// ```
        Red900 => "bg-red-900",
        /// ```css
        /// {
        ///     background-color: rgb(69 10 10);
        /// }
        /// ```
        Red950 => "bg-red-950",
        /// ```css
        /// {
        ///     background-color: rgb(255 247 237);
        /// }
        /// ```
        Orange50 => "bg-orange-50",
        /// ```css
        /// {
        ///     background-color: rgb(255 237 213);
        /// }
        /// ```
        Orange100 => "bg-orange-100",
        /// ```css
        /// {
        ///     background-color: rgb(254 215 170);
        /// }
        /// ```
        Orange200 => "bg-orange-200",
        /// ```css
        /// {
        ///     background-color: rgb(253 186 116);
        /// }
        /// ```
        Orange300 => "bg-orange-300",
        /// ```css
        /// {
        ///     background-color: rgb(251 146 60);
        /// }
        /// ```
        Orange400 => "bg-orange-400",
        /// ```css
        /// {
        ///     background-color: rgb(249 115 22);
        /// }
        /// ```
        Orange500 => "bg-orange-500",
        /// ```css
        /// {
        ///     background-color: rgb(234 88 12);
        /// }
        /// ```
        Orange600 => "bg-orange-600",
        /// ```css
        /// {
        ///     background-color: rgb(194 65 12);
        /// }
        /// ```
        Orange700 => "bg-orange-700",
        /// ```css
        /// {
        ///     background-color: rgb(154 52 18);
        /// }
        /// ```
        Orange800 => "bg-orange-800",
        /// ```css
        /// {
        ///     background-color: rgb(124 45 18);
        /// }
        /// ```
        Orange900 => "bg-orange-900",
        /// ```css
        /// {
        ///     background-color: rgb(67 20 7);
        /// }
        /// ```
        Orange950 => "bg-orange-950",
        /// ```css
        /// {
        ///     background-color: rgb(255 251 235);
        /// }
        /// ```
        Amber50 => "bg-amber-50",
        /// ```css
        /// {
        ///     background-color: rgb(254 243 199);
        /// }
        /// ```
        Amber100 => "bg-amber-100",
        /// ```css
        /// {
        ///     background-color: rgb(253 230 138);
        /// }
        /// ```
        Amber200 => "bg-amber-200",
        /// ```css
        /// {
        ///     background-color: rgb(252 211 77);
        /// }
        /// ```
        Amber300 => "bg-amber-300",
        /// ```css
        /// {
        ///     background-color: rgb(251 191 36);
        /// }
        /// ```
        Amber400 => "bg-amber-400",
        /// ```css
        /// {
        ///     background-color: rgb(245 158 11);
        /// }
        /// ```
        Amber500 => "bg-amber-500",
        /// ```css
        /// {
        ///     background-color: rgb(217 119 6);
        /// }
        /// ```
        Amber600 => "bg-amber-600",
        /// ```css
        /// {
        ///     background-color: rgb(180 83 9);
        /// }
        /// ```
        Amber700 => "bg-amber-700",
        /// ```css
        /// {
        ///     background-color: rgb(146 64 14);
        /// }
        /// ```
        Amber800 => "bg-amber-800",
        /// ```css
        /// {
        ///     background-color: rgb(120 53 15);
        /// }
        /// ```
        Amber900 => "bg-amber-900",
        /// ```css
        /// {
        ///     background-color: rgb(69 26 3);
        /// }
        /// ```
        Amber950 => "bg-amber-950",
        /// ```css
        /// {
        ///     background-color: rgb(254 252 232);
        /// }
        /// ```
        Yellow50 => "bg-yellow-50",
        /// ```css
        /// {
        ///     background-color: rgb(254 249 195);
        /// }
        /// ```
        Yellow100 => "bg-yellow-100",
        /// ```css
        /// {
        ///     background-color: rgb(254 240 138);
        /// }
        /// ```
        Yellow200 => "bg-yellow-200",
        /// ```css
        /// {
        ///     background-color: rgb(253 224 71);
        /// }
        /// ```
        Yellow300 => "bg-yellow-300",
        /// ```css
        /// {
        ///     background-color: rgb(250 204 21);
        /// }
        /// ```
        Yellow400 => "bg-yellow-400",
        /// ```css
        /// {
        ///     background-color: rgb(234 179 8);
        /// }
        /// ```
        Yellow500 => "bg-yellow-500",
        /// ```css
        /// {
        ///     background-color: rgb(202 138 4);
        /// }
        /// ```
        Yellow600 => "bg-yellow-600",
        /// ```css
        /// {
        ///     background-color: rgb(161 98 7);
        /// }
        /// ```
        Yellow700 => "bg-yellow-700",
        /// ```css
        /// {
        ///     background-color: rgb(133 77 14);
        /// }
        /// ```
        Yellow800 => "bg-yellow-800",
        /// ```css
        /// {
        ///     background-color: rgb(113 63 18);
        /// }
        /// ```
        Yellow900 => "bg-yellow-900",
        /// ```css
        /// {
        ///     background-color: rgb(66 32 6);
        /// }
        /// ```
        Yellow950 => "bg-yellow-950",
        /// ```css
        /// {
        ///     background-color: rgb(247 254 231);
        /// }
        /// ```
        Lime50 => "bg-lime-50",
        /// ```css
        /// {
        ///     background-color: rgb(236 252 203);
        /// }
        /// ```
        Lime100 => "bg-lime-100",
        /// ```css
        /// {
        ///     background-color: rgb(217 249 157);
        /// }
        /// ```
        Lime200 => "bg-lime-200",
        /// ```css
        /// {
        ///     background-color: rgb(190 242 100);
        /// }
        /// ```
        Lime300 => "bg-lime-300",
        /// ```css
        /// {
        ///     background-color: rgb(163 230 53);
        /// }
        /// ```
        Lime400 => "bg-lime-400",
        /// ```css
        /// {
        ///     background-color: rgb(132 204 22);
        /// }
        /// ```
        Lime500 => "bg-lime-500",
        /// ```css
        /// {
        ///     background-color: rgb(101 163 13);
        /// }
        /// ```
        Lime600 => "bg-lime-600",
        /// ```css
        /// {
        ///     background-color: rgb(77 124 15);
        /// }
        /// ```
        Lime700 => "bg-lime-700",
        /// ```css
        /// {
        ///     background-color: rgb(63 98 18);
        /// }
        /// ```
        Lime800 => "bg-lime-800",
        /// ```css
        /// {
        ///     background-color: rgb(54 83 20);
        /// }
        /// ```
        Lime900 => "bg-lime-900",
        /// ```css
        /// {
        ///     background-color: rgb(26 46 5);
        /// }
        /// ```
        Lime950 => "bg-lime-950",
        /// ```css
        /// {
        ///     background-color: rgb(240 253 244);
        /// }
        /// ```
        Green50 => "bg-green-50",
        /// ```css
        /// {
        ///     background-color: rgb(220 252 231);
        /// }
        /// ```
        Green100 => "bg-green-100",
        /// ```css
        /// {
        ///     background-color: rgb(187 247 208);
        /// }
        /// ```
        Green200 => "bg-green-200",
        /// ```css
        /// {
        ///     background-color: rgb(134 239 172);
        /// }
        /// ```
        Green300 => "bg-green-300",
        /// ```css
        /// {
        ///     background-color: rgb(74 222 128);
        /// }
        /// ```
        Green400 => "bg-green-400",
        /// ```css
        /// {
        ///     background-color: rgb(34 197 94);
        /// }
        /// ```
        Green500 => "bg-green-500",
        /// ```css
        /// {
        ///     background-color: rgb(22 163 74);
        /// }
        /// ```
        Green600 => "bg-green-600",
        /// ```css
        /// {
        ///     background-color: rgb(21 128 61);
        /// }
        /// ```
        Green700 => "bg-green-700",
        /// ```css
        /// {
        ///     background-color: rgb(22 101 52);
        /// }
        /// ```
        Green800 => "bg-green-800",
        /// ```css
        /// {
        ///     background-color: rgb(20 83 45);
        /// }
        /// ```
        Green900 => "bg-green-900",
        /// ```css
        /// {
        ///     background-color: rgb(5 46 22);
        /// }
        /// ```
        Green950 => "bg-green-950",
        /// ```css
        /// {
        ///     background-color: rgb(236 253 245);
        /// }
        /// ```
        Emerald50 => "bg-emerald-50",
        /// ```css
        /// {
        ///     background-color: rgb(209 250 229);
        /// }
        /// ```
        Emerald100 => "bg-emerald-100",
        /// ```css
        /// {
        ///     background-color: rgb(167 243 208);
        /// }
        /// ```
        Emerald200 => "bg-emerald-200",
        /// ```css
        /// {
        ///     background-color: rgb(110 231 183);
        /// }
        /// ```
        Emerald300 => "bg-emerald-300",
        /// ```css
        /// {
        ///     background-color: rgb(52 211 153);
        /// }
        /// ```
        Emerald400 => "bg-emerald-400",
        /// ```css
        /// {
        ///     background-color: rgb(16 185 129);
        /// }
        /// ```
        Emerald500 => "bg-emerald-500",
        /// ```css
        /// {
        ///     background-color: rgb(5 150 105);
        /// }
        /// ```
        Emerald600 => "bg-emerald-600",
        /// ```css
        /// {
        ///     background-color: rgb(4 120 87);
        /// }
        /// ```
        Emerald700 => "bg-emerald-700",
        /// ```css
        /// {
        ///     background-color: rgb(6 95 70);
        /// }
        /// ```
        Emerald800 => "bg-emerald-800",
        /// ```css
        /// {
        ///     background-color: rgb(6 78 59);
        /// }
        /// ```
        Emerald900 => "bg-emerald-900",
        /// ```css
        /// {
        ///     background-color: rgb(2 44 34);
        /// }
        /// ```
        Emerald950 => "bg-emerald-950",
        /// ```css
        /// {
        ///     background-color: rgb(240 253 250);
        /// }
        /// ```
        Teal50 => "bg-teal-50",
        /// ```css
        /// {
        ///     background-color: rgb(204 251 241);
        /// }
        /// ```
        Teal100 => "bg-teal-100",
        /// ```css
        /// {
        ///     background-color: rgb(153 246 228);
        /// }
        /// ```
        Teal200 => "bg-teal-200",
        /// ```css
        /// {
        ///     background-color: rgb(94 234 212);
        /// }
        /// ```
        Teal300 => "bg-teal-300",
        /// ```css
        /// {
        ///     background-color: rgb(45 212 191);
        /// }
        /// ```
        Teal400 => "bg-teal-400",
        /// ```css
        /// {
        ///     background-color: rgb(20 184 166);
        /// }
        /// ```
        Teal500 => "bg-teal-500",
        /// ```css
        /// {
        ///     background-color: rgb(13 148 136);
        /// }
        /// ```
        Teal600 => "bg-teal-600",
        /// ```css
        /// {
        ///     background-color: rgb(15 118 110);
        /// }
        /// ```
        Teal700 => "bg-teal-700",
        /// ```css
        /// {
        ///     background-color: rgb(17 94 89);
        /// }
        /// ```
        Teal800 => "bg-teal-800",
        /// ```css
        /// {
        ///     background-color: rgb(19 78 74);
        /// }
        /// ```
        Teal900 => "bg-teal-900",
        /// ```css
        /// {
        ///     background-color: rgb(4 47 46);
        /// }
        /// ```
        Teal950 => "bg-teal-950",
        /// ```css
        /// {
        ///     background-color: rgb(236 254 255);
        /// }
        /// ```
        Cyan50 => "bg-cyan-50",
        /// ```css
        /// {
        ///     background-color: rgb(207 250 254);
        /// }
        /// ```
        Cyan100 => "bg-cyan-100",
        /// ```css
        /// {
        ///     background-color: rgb(165 243 252);
        /// }
        /// ```
        Cyan200 => "bg-cyan-200",
        /// ```css
        /// {
        ///     background-color: rgb(103 232 249);
        /// }
        /// ```
        Cyan300 => "bg-cyan-300",
        /// ```css
        /// {
        ///     background-color: rgb(34 211 238);
        /// }
        /// ```
        Cyan400 => "bg-cyan-400",
        /// ```css
        /// {
        ///     background-color: rgb(6 182 212);
        /// }
        /// ```
        Cyan500 => "bg-cyan-500",
        /// ```css
        /// {
        ///     background-color: rgb(8 145 178);
        /// }
        /// ```
        Cyan600 => "bg-cyan-600",
        /// ```css
        /// {
        ///     background-color: rgb(14 116 144);
        /// }
        /// ```
        Cyan700 => "bg-cyan-700",
        /// ```css
        /// {
        ///     background-color: rgb(21 94 117);
        /// }
        /// ```
        Cyan800 => "bg-cyan-800",
        /// ```css
        /// {
        ///     background-color: rgb(22 78 99);
        /// }
        /// ```
        Cyan900 => "bg-cyan-900",
        /// ```css
        /// {
        ///     background-color: rgb(8 51 68);
        /// }
        /// ```
        Cyan950 => "bg-cyan-950",
        /// ```css
        /// {
        ///     background-color: rgb(240 249 255);
        /// }
        /// ```
        Sky50 => "bg-sky-50",
        /// ```css
        /// {
        ///     background-color: rgb(224 242 254);
        /// }
        /// ```
        Sky100 => "bg-sky-100",
        /// ```css
        /// {
        ///     background-color: rgb(186 230 253);
        /// }
        /// ```
        Sky200 => "bg-sky-200",
        /// ```css
        /// {
        ///     background-color: rgb(125 211 252);
        /// }
        /// ```
        Sky300 => "bg-sky-300",
        /// ```css
        /// {
        ///     background-color: rgb(56 189 248);
        /// }
        /// ```
        Sky400 => "bg-sky-400",
        /// ```css
        /// {
        ///     background-color: rgb(14 165 233);
        /// }
        /// ```
        Sky500 => "bg-sky-500",
        /// ```css
        /// {
        ///     background-color: rgb(2 132 199);
        /// }
        /// ```
        Sky600 => "bg-sky-600",
        /// ```css
        /// {
        ///     background-color: rgb(3 105 161);
        /// }
        /// ```
        Sky700 => "bg-sky-700",
        /// ```css
        /// {
        ///     background-color: rgb(7 89 133);
        /// }
        /// ```
        Sky800 => "bg-sky-800",
        /// ```css
        /// {
        ///     background-color: rgb(12 74 110);
        /// }
        /// ```
        Sky900 => "bg-sky-900",
        /// ```css
        /// {
        ///     background-color: rgb(8 47 73);
        /// }
        /// ```
        Sky950 => "bg-sky-950",
        /// ```css
        /// {
        ///     background-color: rgb(239 246 255);
        /// }
        /// ```
        Blue50 => "bg-blue-50",
        /// ```css
        /// {
        ///     background-color: rgb(219 234 254);
        /// }
        /// ```
        Blue100 => "bg-blue-100",
        /// ```css
        /// {
        ///     background-color: rgb(191 219 254);
        /// }
        /// ```
        Blue200 => "bg-blue-200",
        /// ```css
        /// {
        ///     background-color: rgb(147 197 253);
        /// }
        /// ```
        Blue300 => "bg-blue-300",
        /// ```css
        /// {
        ///     background-color: rgb(96 165 250);
        /// }
        /// ```
        Blue400 => "bg-blue-400",
        /// ```css
        /// {
        ///     background-color: rgb(59 130 246);
        /// }
        /// ```
        Blue500 => "bg-blue-500",
        /// ```css
        /// {
        ///     background-color: rgb(37 99 235);
        /// }
        /// ```
        Blue600 => "bg-blue-600",
        /// ```css
        /// {
        ///     background-color: rgb(29 78 216);
        /// }
        /// ```
        Blue700 => "bg-blue-700",
        /// ```css
        /// {
        ///     background-color: rgb(30 64 175);
        /// }
        /// ```
        Blue800 => "bg-blue-800",
        /// ```css
        /// {
        ///     background-color: rgb(30 58 138);
        /// }
        /// ```
        Blue900 => "bg-blue-900",
        /// ```css
        /// {
        ///     background-color: rgb(23 37 84);
        /// }
        /// ```
        Blue950 => "bg-blue-950",
        /// ```css
        /// {
        ///     background-color: rgb(238 242 255);
        /// }
        /// ```
        Indigo50 => "bg-indigo-50",
        /// ```css
        /// {
        ///     background-color: rgb(224 231 255);
        /// }
        /// ```
        Indigo100 => "bg-indigo-100",
        /// ```css
        /// {
        ///     background-color: rgb(199 210 254);
        /// }
        /// ```
        Indigo200 => "bg-indigo-200",
        /// ```css
        /// {
        ///     background-color: rgb(165 180 252);
        /// }
        /// ```
        Indigo300 => "bg-indigo-300",
        /// ```css
        /// {
        ///     background-color: rgb(129 140 248);
        /// }
        /// ```
        Indigo400 => "bg-indigo-400",
        /// ```css
        /// {
        ///     background-color: rgb(99 102 241);
        /// }
        /// ```
        Indigo500 => "bg-indigo-500",
        /// ```css
        /// {
        ///     background-color: rgb(79 70 229);
        /// }
        /// ```
        Indigo600 => "bg-indigo-600",
        /// ```css
        /// {
        ///     background-color: rgb(67 56 202);
        /// }
        /// ```
        Indigo700 => "bg-indigo-700",
        /// ```css
        /// {
        ///     background-color: rgb(55 48 163);
        /// }
        /// ```
        Indigo800 => "bg-indigo-800",
        /// ```css
        /// {
        ///     background-color: rgb(49 46 129);
        /// }
        /// ```
        Indigo900 => "bg-indigo-900",
        /// ```css
        /// {
        ///     background-color: rgb(30 27 75);
        /// }
        /// ```
        Indigo950 => "bg-indigo-950",
        /// ```css
        /// {
        ///     background-color: rgb(245 243 255);
        /// }
        /// ```
        Violet50 => "bg-violet-50",
        /// ```css
        /// {
        ///     background-color: rgb(237 233 254);
        /// }
        /// ```
        Violet100 => "bg-violet-100",
        /// ```css
        /// {
        ///     background-color: rgb(221 214 254);
        /// }
        /// ```
        Violet200 => "bg-violet-200",
        /// ```css
        /// {
        ///     background-color: rgb(196 181 253);
        /// }
        /// ```
        Violet300 => "bg-violet-300",
        /// ```css
        /// {
        ///     background-color: rgb(167 139 250);
        /// }
        /// ```
        Violet400 => "bg-violet-400",
        /// ```css
        /// {
        ///     background-color: rgb(139 92 246);
        /// }
        /// ```
        Violet500 => "bg-violet-500",
        /// ```css
        /// {
        ///     background-color: rgb(124 58 237);
        /// }
        /// ```
        Violet600 => "bg-violet-600",
        /// ```css
        /// {
        ///     background-color: rgb(109 40 217);
        /// }
        /// ```
        Violet700 => "bg-violet-700",
        /// ```css
        /// {
        ///     background-color: rgb(91 33 182);
        /// }
        /// ```
        Violet800 => "bg-violet-800",
        /// ```css
        /// {
        ///     background-color: rgb(76 29 149);
        /// }
        /// ```
        Violet900 => "bg-violet-900",
        /// ```css
        /// {
        ///     background-color: rgb(46 16 101);
        /// }
        /// ```
        Violet950 => "bg-violet-950",
        /// ```css
        /// {
        ///     background-color: rgb(250 245 255);
        /// }
        /// ```
        Purple50 => "bg-purple-50",
        /// ```css
        /// {
        ///     background-color: rgb(243 232 255);
        /// }
        /// ```
        Purple100 => "bg-purple-100",
        /// ```css
        /// {
        ///     background-color: rgb(233 213 255);
        /// }
        /// ```
        Purple200 => "bg-purple-200",
        /// ```css
        /// {
        ///     background-color: rgb(216 180 254);
        /// }
        /// ```
        Purple300 => "bg-purple-300",
        /// ```css
        /// {
        ///     background-color: rgb(192 132 252);
        /// }
        /// ```
        Purple400 => "bg-purple-400",
        /// ```css
        /// {
        ///     background-color: rgb(168 85 247);
        /// }
        /// ```
        Purple500 => "bg-purple-500",
        /// ```css
        /// {
        ///     background-color: rgb(147 51 234);
        /// }
        /// ```
        Purple600 => "bg-purple-600",
        /// ```css
        /// {
        ///     background-color: rgb(126 34 206);
        /// }
        /// ```
        Purple700 => "bg-purple-700",
        /// ```css
        /// {
        ///     background-color: rgb(107 33 168);
        /// }
        /// ```
        Purple800 => "bg-purple-800",
        /// ```css
        /// {
        ///     background-color: rgb(88 28 135);
        /// }
        /// ```
        Purple900 => "bg-purple-900",
        /// ```css
        /// {
        ///     background-color: rgb(59 7 100);
        /// }
        /// ```
        Purple950 => "bg-purple-950",
        /// ```css
        /// {
        ///     background-color: rgb(253 244 255);
        /// }
        /// ```
        Fuchsia50 => "bg-fuchsia-50",
        /// ```css
        /// {
        ///     background-color: rgb(250 232 255);
        /// }
        /// ```
        Fuchsia100 => "bg-fuchsia-100",
        /// ```css
        /// {
        ///     background-color: rgb(245 208 254);
        /// }
        /// ```
        Fuchsia200 => "bg-fuchsia-200",
        /// ```css
        /// {
        ///     background-color: rgb(240 171 252);
        /// }
        /// ```
        Fuchsia300 => "bg-fuchsia-300",
        /// ```css
        /// {
        ///     background-color: rgb(232 121 249);
        /// }
        /// ```
        Fuchsia400 => "bg-fuchsia-400",
        /// ```css
        /// {
        ///     background-color: rgb(217 70 239);
        /// }
        /// ```
        Fuchsia500 => "bg-fuchsia-500",
        /// ```css
        /// {
        ///     background-color: rgb(192 38 211);
        /// }
        /// ```
        Fuchsia600 => "bg-fuchsia-600",
        /// ```css
        /// {
        ///     background-color: rgb(162 28 175);
        /// }
        /// ```
        Fuchsia700 => "bg-fuchsia-700",
        /// ```css
        /// {
        ///     background-color: rgb(134 25 143);
        /// }
        /// ```
        Fuchsia800 => "bg-fuchsia-800",
        /// ```css
        /// {
        ///     background-color: rgb(112 26 117);
        /// }
        /// ```
        Fuchsia900 => "bg-fuchsia-900",
        /// ```css
        /// {
        ///     background-color: rgb(74 4 78);
        /// }
        /// ```
        Fuchsia950 => "bg-fuchsia-950",
        /// ```css
        /// {
        ///     background-color: rgb(253 242 248);
        /// }
        /// ```
        Pink50 => "bg-pink-50",
        /// ```css
        /// {
        ///     background-color: rgb(252 231 243);
        /// }
        /// ```
        Pink100 => "bg-pink-100",
        /// ```css
        /// {
        ///     background-color: rgb(251 207 232);
        /// }
        /// ```
        Pink200 => "bg-pink-200",
        /// ```css
        /// {
        ///     background-color: rgb(249 168 212);
        /// }
        /// ```
        Pink300 => "bg-pink-300",
        /// ```css
        /// {
        ///     background-color: rgb(244 114 182);
        /// }
        /// ```
        Pink400 => "bg-pink-400",
        /// ```css
        /// {
        ///     background-color: rgb(236 72 153);
        /// }
        /// ```
        Pink500 => "bg-pink-500",
        /// ```css
        /// {
        ///     background-color: rgb(219 39 119);
        /// }
        /// ```
        Pink600 => "bg-pink-600",
        /// ```css
        /// {
        ///     background-color: rgb(190 24 93);
        /// }
        /// ```
        Pink700 => "bg-pink-700",
        /// ```css
        /// {
        ///     background-color: rgb(157 23 77);
        /// }
        /// ```
        Pink800 => "bg-pink-800",
        /// ```css
        /// {
        ///     background-color: rgb(131 24 67);
        /// }
        /// ```
        Pink900 => "bg-pink-900",
        /// ```css
        /// {
        ///     background-color: rgb(80 7 36);
        /// }
        /// ```
        Pink950 => "bg-pink-950",
        /// ```css
        /// {
        ///     background-color: rgb(255 241 242);
        /// }
        /// ```
        Rose50 => "bg-rose-50",
        /// ```css
        /// {
        ///     background-color: rgb(255 228 230);
        /// }
        /// ```
        Rose100 => "bg-rose-100",
        /// ```css
        /// {
        ///     background-color: rgb(254 205 211);
        /// }
        /// ```
        Rose200 => "bg-rose-200",
        /// ```css
        /// {
        ///     background-color: rgb(253 164 175);
        /// }
        /// ```
        Rose300 => "bg-rose-300",
        /// ```css
        /// {
        ///     background-color: rgb(251 113 133);
        /// }
        /// ```
        Rose400 => "bg-rose-400",
        /// ```css
        /// {
        ///     background-color: rgb(244 63 94);
        /// }
        /// ```
        Rose500 => "bg-rose-500",
        /// ```css
        /// {
        ///     background-color: rgb(225 29 72);
        /// }
        /// ```
        Rose600 => "bg-rose-600",
        /// ```css
        /// {
        ///     background-color: rgb(190 18 60);
        /// }
        /// ```
        Rose700 => "bg-rose-700",
        /// ```css
        /// {
        ///     background-color: rgb(159 18 57);
        /// }
        /// ```
        Rose800 => "bg-rose-800",
        /// ```css
        /// {
        ///     background-color: rgb(136 19 55);
        /// }
        /// ```
        Rose900 => "bg-rose-900",
        /// ```css
        /// {
        ///     background-color: rgb(76 5 25);
        /// }
        /// ```
        Rose950 => "bg-rose-950",
    }
    /// Utilities for controlling how an element's background is positioned relative to borders, padding, and content.
    ///
    /// <https://tailwindcss.com/docs/background-origin>
    BackgroundOrigin {
        /// ```css
        /// {
        ///     background-origin: border-box;
        /// }
        /// ```
        Border => "bg-origin-border",
        /// ```css
        /// {
        ///     background-origin: padding-box;
        /// }
        /// ```
        Padding => "bg-origin-padding",
        /// ```css
        /// {
        ///     background-origin: content-box;
        /// }
        /// ```
        Content => "bg-origin-content",
    }
    /// Utilities for controlling the position of an element's background image.
    ///
    /// <https://tailwindcss.com/docs/background-position>
    BackgroundPosition {
        /// ```css
        /// {
        ///     background-position: bottom;
        /// }
        /// ```
        Bottom => "bg-bottom",
        /// ```css
        /// {
        ///     background-position: center;
        /// }
        /// ```
        Center => "bg-center",
        /// ```css
        /// {
        ///     background-position: left;
        /// }
        /// ```
        Left => "bg-left",
        /// ```css
        /// {
        ///     background-position: left bottom;
        /// }
        /// ```
        LeftBottom => "bg-left-bottom",
        /// ```css
        /// {
        ///     background-position: left top;
        /// }
        /// ```
        LeftTop => "bg-left-top",
        /// ```css
        /// {
        ///     background-position: right;
        /// }
        /// ```
        Right => "bg-right",
        /// ```css
        /// {
        ///     background-position: right bottom;
        /// }
        /// ```
        RightBottom => "bg-right-bottom",
        /// ```css
        /// {
        ///     background-position: right top;
        /// }
        /// ```
        RightTop => "bg-right-top",
        /// ```css
        /// {
        ///     background-position: top;
        /// }
        /// ```
        Top => "bg-top",
    }
    /// Utilities for controlling the repetition of an element's background image.
    ///
    /// <https://tailwindcss.com/docs/background-repeat>
    BackgroundRepeat {
        /// ```css
        /// {
        ///     background-repeat: repeat;
        /// }
        /// ```
        Repeat => "bg-repeat",
        /// ```css
        /// {
        ///     background-repeat: no-repeat;
        /// }
        /// ```
        NoRepeat => "bg-no-repeat",
        /// ```css
        /// {
        ///     background-repeat: repeat-x;
        /// }
        /// ```
        RepeatX => "bg-repeat-x",
        /// ```css
        /// {
        ///     background-repeat: repeat-y;
        /// }
        /// ```
        RepeatY => "bg-repeat-y",
        /// ```css
        /// {
        ///     background-repeat: round;
        /// }
        /// ```
        RepeatRound => "bg-repeat-round",
        /// ```css
        /// {
        ///     background-repeat: space;
        /// }
        /// ```
        RepeatSpace => "bg-repeat-space",
    }
    /// Utilities for controlling the background size of an element's background image.
    ///
    /// <https://tailwindcss.com/docs/background-size>
    BackgroundSize {
        /// ```css
        /// {
        ///     background-size: auto;
        /// }
        /// ```
        Auto => "bg-auto",
        /// ```css
        /// {
        ///     background-size: cover;
        /// }
        /// ```
        Cover => "bg-cover",
        /// ```css
        /// {
        ///     background-size: contain;
        /// }
        /// ```
        Contain => "bg-contain",
    }
    /// Utilities for controlling an element's background image.
    ///
    /// <https://tailwindcss.com/docs/background-image>
    BackgroundImage {
        /// ```css
        /// {
        ///     background-image: none;
        /// }
        /// ```
        None => "bg-none",
        /// ```css
        /// {
        ///     background-image: linear-gradient(to top, var(--tw-gradient-stops));
        /// }
        /// ```
        GradientToT => "bg-gradient-to-t",
        /// ```css
        /// {
        ///     background-image: linear-gradient(to top right, var(--tw-gradient-stops));
        /// }
        /// ```
        GradientToTr => "bg-gradient-to-tr",
        /// ```css
        /// {
        ///     background-image: linear-gradient(to right, var(--tw-gradient-stops));
        /// }
        /// ```
        GradientToR => "bg-gradient-to-r",
        /// ```css
        /// {
        ///     background-image: linear-gradient(to bottom right, var(--tw-gradient-stops));
        /// }
        /// ```
        GradientToBr => "bg-gradient-to-br",
        /// ```css
        /// {
        ///     background-image: linear-gradient(to bottom, var(--tw-gradient-stops));
        /// }
        /// ```
        GradientToB => "bg-gradient-to-b",
        /// ```css
        /// {
        ///     background-image: linear-gradient(to bottom left, var(--tw-gradient-stops));
        /// }
        /// ```
        GradientToBl => "bg-gradient-to-bl",
        /// ```css
        /// {
        ///     background-image: linear-gradient(to left, var(--tw-gradient-stops));
        /// }
        /// ```
        GradientToL => "bg-gradient-to-l",
        /// ```css
        /// {
        ///     background-image: linear-gradient(to top left, var(--tw-gradient-stops));
        /// }
        /// ```
        GradientToTl => "bg-gradient-to-tl",
    }
    /// Utilities for controlling the color stops in background gradients.
    ///
    /// <https://tailwindcss.com/docs/gradient-color-stops>
    GradientColorStops {
        /// ```css
        /// {
        ///     --tw-gradient-from: inherit var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromInherit => "from-inherit",
        /// ```css
        /// {
        ///     --tw-gradient-from: currentColor var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCurrent => "from-current",
        /// ```css
        /// {
        ///     --tw-gradient-from: transparent var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTransparent => "from-transparent",
        /// ```css
        /// {
        ///     --tw-gradient-from: #000 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(0 0 0 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlack => "from-black",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(255 255 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromWhite => "from-white",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f8fafc var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(248 250 252 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate50 => "from-slate-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f1f5f9 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(241 245 249 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate100 => "from-slate-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e2e8f0 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(226 232 240 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate200 => "from-slate-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #cbd5e1 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(203 213 225 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate300 => "from-slate-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #94a3b8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(148 163 184 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate400 => "from-slate-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #64748b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(100 116 139 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate500 => "from-slate-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #475569 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(71 85 105 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate600 => "from-slate-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #334155 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(51 65 85 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate700 => "from-slate-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #1e293b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(30 41 59 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate800 => "from-slate-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0f172a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(15 23 42 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate900 => "from-slate-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #020617 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(2 6 23 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSlate950 => "from-slate-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f9fafb var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(249 250 251 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray50 => "from-gray-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f3f4f6 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(243 244 246 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray100 => "from-gray-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e5e7eb var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(229 231 235 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray200 => "from-gray-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d1d5db var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(209 213 219 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray300 => "from-gray-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #9ca3af var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(156 163 175 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray400 => "from-gray-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #6b7280 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(107 114 128 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray500 => "from-gray-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #4b5563 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(75 85 99 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray600 => "from-gray-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #374151 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(55 65 81 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray700 => "from-gray-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #1f2937 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(31 41 55 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray800 => "from-gray-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #111827 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(17 24 39 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray900 => "from-gray-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #030712 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(3 7 18 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGray950 => "from-gray-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fafafa var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc50 => "from-zinc-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f4f4f5 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(244 244 245 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc100 => "from-zinc-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e4e4e7 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(228 228 231 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc200 => "from-zinc-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d4d4d8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(212 212 216 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc300 => "from-zinc-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a1a1aa var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(161 161 170 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc400 => "from-zinc-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #71717a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(113 113 122 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc500 => "from-zinc-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #52525b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(82 82 91 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc600 => "from-zinc-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #3f3f46 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(63 63 70 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc700 => "from-zinc-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #27272a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(39 39 42 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc800 => "from-zinc-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #18181b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(24 24 27 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc900 => "from-zinc-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #09090b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(9 9 11 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromZinc950 => "from-zinc-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fafafa var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(250 250 250 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral50 => "from-neutral-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f5f5f5 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(245 245 245 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral100 => "from-neutral-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e5e5e5 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(229 229 229 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral200 => "from-neutral-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d4d4d4 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(212 212 212 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral300 => "from-neutral-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a3a3a3 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(163 163 163 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral400 => "from-neutral-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #737373 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(115 115 115 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral500 => "from-neutral-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #525252 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(82 82 82 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral600 => "from-neutral-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #404040 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(64 64 64 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral700 => "from-neutral-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #262626 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(38 38 38 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral800 => "from-neutral-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #171717 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(23 23 23 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral900 => "from-neutral-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0a0a0a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(10 10 10 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromNeutral950 => "from-neutral-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fafaf9 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(250 250 249 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone50 => "from-stone-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f5f5f4 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(245 245 244 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone100 => "from-stone-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e7e5e4 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(231 229 228 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone200 => "from-stone-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d6d3d1 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(214 211 209 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone300 => "from-stone-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a8a29e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(168 162 158 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone400 => "from-stone-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #78716c var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(120 113 108 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone500 => "from-stone-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #57534e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(87 83 78 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone600 => "from-stone-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #44403c var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(68 64 60 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone700 => "from-stone-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #292524 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(41 37 36 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone800 => "from-stone-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #1c1917 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(28 25 23 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone900 => "from-stone-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0c0a09 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(12 10 9 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromStone950 => "from-stone-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fef2f2 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 242 242 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed50 => "from-red-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fee2e2 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 226 226 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed100 => "from-red-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fecaca var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 202 202 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed200 => "from-red-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fca5a5 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(252 165 165 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed300 => "from-red-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f87171 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(248 113 113 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed400 => "from-red-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ef4444 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(239 68 68 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed500 => "from-red-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #dc2626 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(220 38 38 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed600 => "from-red-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #b91c1c var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(185 28 28 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed700 => "from-red-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #991b1b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(153 27 27 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed800 => "from-red-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #7f1d1d var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(127 29 29 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed900 => "from-red-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #450a0a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(69 10 10 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRed950 => "from-red-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fff7ed var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(255 247 237 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange50 => "from-orange-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ffedd5 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(255 237 213 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange100 => "from-orange-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fed7aa var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 215 170 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange200 => "from-orange-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fdba74 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(253 186 116 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange300 => "from-orange-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fb923c var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(251 146 60 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange400 => "from-orange-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f97316 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(249 115 22 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange500 => "from-orange-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ea580c var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(234 88 12 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange600 => "from-orange-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #c2410c var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(194 65 12 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange700 => "from-orange-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #9a3412 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(154 52 18 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange800 => "from-orange-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #7c2d12 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(124 45 18 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange900 => "from-orange-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #431407 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(67 20 7 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromOrange950 => "from-orange-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fffbeb var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(255 251 235 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber50 => "from-amber-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fef3c7 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 243 199 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber100 => "from-amber-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fde68a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(253 230 138 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber200 => "from-amber-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fcd34d var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(252 211 77 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber300 => "from-amber-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fbbf24 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(251 191 36 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber400 => "from-amber-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f59e0b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(245 158 11 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber500 => "from-amber-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d97706 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(217 119 6 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber600 => "from-amber-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #b45309 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(180 83 9 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber700 => "from-amber-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #92400e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(146 64 14 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber800 => "from-amber-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #78350f var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(120 53 15 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber900 => "from-amber-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #451a03 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(69 26 3 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromAmber950 => "from-amber-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fefce8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 252 232 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow50 => "from-yellow-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fef9c3 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 249 195 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow100 => "from-yellow-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fef08a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 240 138 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow200 => "from-yellow-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fde047 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(253 224 71 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow300 => "from-yellow-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #facc15 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(250 204 21 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow400 => "from-yellow-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #eab308 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(234 179 8 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow500 => "from-yellow-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ca8a04 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(202 138 4 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow600 => "from-yellow-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a16207 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(161 98 7 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow700 => "from-yellow-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #854d0e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(133 77 14 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow800 => "from-yellow-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #713f12 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(113 63 18 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow900 => "from-yellow-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #422006 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(66 32 6 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromYellow950 => "from-yellow-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f7fee7 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(247 254 231 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime50 => "from-lime-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ecfccb var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(236 252 203 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime100 => "from-lime-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d9f99d var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(217 249 157 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime200 => "from-lime-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #bef264 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(190 242 100 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime300 => "from-lime-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a3e635 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(163 230 53 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime400 => "from-lime-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #84cc16 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(132 204 22 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime500 => "from-lime-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #65a30d var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(101 163 13 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime600 => "from-lime-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #4d7c0f var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(77 124 15 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime700 => "from-lime-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #3f6212 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(63 98 18 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime800 => "from-lime-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #365314 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(54 83 20 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime900 => "from-lime-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #1a2e05 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(26 46 5 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromLime950 => "from-lime-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f0fdf4 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(240 253 244 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen50 => "from-green-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #dcfce7 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(220 252 231 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen100 => "from-green-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #bbf7d0 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(187 247 208 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen200 => "from-green-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #86efac var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(134 239 172 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen300 => "from-green-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #4ade80 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(74 222 128 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen400 => "from-green-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #22c55e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(34 197 94 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen500 => "from-green-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #16a34a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(22 163 74 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen600 => "from-green-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #15803d var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(21 128 61 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen700 => "from-green-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #166534 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(22 101 52 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen800 => "from-green-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #14532d var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(20 83 45 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen900 => "from-green-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #052e16 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(5 46 22 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromGreen950 => "from-green-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ecfdf5 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(236 253 245 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald50 => "from-emerald-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d1fae5 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(209 250 229 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald100 => "from-emerald-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a7f3d0 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(167 243 208 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald200 => "from-emerald-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #6ee7b7 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(110 231 183 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald300 => "from-emerald-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #34d399 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(52 211 153 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald400 => "from-emerald-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #10b981 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(16 185 129 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald500 => "from-emerald-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #059669 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(5 150 105 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald600 => "from-emerald-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #047857 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(4 120 87 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald700 => "from-emerald-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #065f46 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(6 95 70 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald800 => "from-emerald-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #064e3b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(6 78 59 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald900 => "from-emerald-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #022c22 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(2 44 34 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromEmerald950 => "from-emerald-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f0fdfa var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(240 253 250 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal50 => "from-teal-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ccfbf1 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(204 251 241 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal100 => "from-teal-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #99f6e4 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(153 246 228 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal200 => "from-teal-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #5eead4 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(94 234 212 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal300 => "from-teal-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #2dd4bf var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(45 212 191 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal400 => "from-teal-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #14b8a6 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(20 184 166 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal500 => "from-teal-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0d9488 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(13 148 136 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal600 => "from-teal-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0f766e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(15 118 110 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal700 => "from-teal-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #115e59 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(17 94 89 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal800 => "from-teal-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #134e4a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(19 78 74 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal900 => "from-teal-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #042f2e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(4 47 46 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromTeal950 => "from-teal-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ecfeff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(236 254 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan50 => "from-cyan-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #cffafe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(207 250 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan100 => "from-cyan-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a5f3fc var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(165 243 252 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan200 => "from-cyan-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #67e8f9 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(103 232 249 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan300 => "from-cyan-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #22d3ee var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(34 211 238 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan400 => "from-cyan-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #06b6d4 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(6 182 212 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan500 => "from-cyan-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0891b2 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(8 145 178 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan600 => "from-cyan-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0e7490 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(14 116 144 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan700 => "from-cyan-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #155e75 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(21 94 117 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan800 => "from-cyan-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #164e63 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(22 78 99 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan900 => "from-cyan-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #083344 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(8 51 68 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromCyan950 => "from-cyan-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f0f9ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(240 249 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky50 => "from-sky-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e0f2fe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(224 242 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky100 => "from-sky-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #bae6fd var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(186 230 253 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky200 => "from-sky-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #7dd3fc var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(125 211 252 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky300 => "from-sky-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #38bdf8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(56 189 248 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky400 => "from-sky-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0ea5e9 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(14 165 233 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky500 => "from-sky-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0284c7 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(2 132 199 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky600 => "from-sky-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0369a1 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(3 105 161 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky700 => "from-sky-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #075985 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(7 89 133 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky800 => "from-sky-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #0c4a6e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(12 74 110 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky900 => "from-sky-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #082f49 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(8 47 73 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromSky950 => "from-sky-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #eff6ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(239 246 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue50 => "from-blue-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #dbeafe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(219 234 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue100 => "from-blue-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #bfdbfe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(191 219 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue200 => "from-blue-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #93c5fd var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(147 197 253 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue300 => "from-blue-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #60a5fa var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(96 165 250 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue400 => "from-blue-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #3b82f6 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(59 130 246 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue500 => "from-blue-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #2563eb var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(37 99 235 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue600 => "from-blue-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #1d4ed8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(29 78 216 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue700 => "from-blue-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #1e40af var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(30 64 175 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue800 => "from-blue-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #1e3a8a var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(30 58 138 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue900 => "from-blue-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #172554 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(23 37 84 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromBlue950 => "from-blue-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #eef2ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(238 242 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo50 => "from-indigo-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e0e7ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(224 231 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo100 => "from-indigo-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #c7d2fe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(199 210 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo200 => "from-indigo-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a5b4fc var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(165 180 252 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo300 => "from-indigo-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #818cf8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(129 140 248 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo400 => "from-indigo-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #6366f1 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(99 102 241 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo500 => "from-indigo-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #4f46e5 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(79 70 229 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo600 => "from-indigo-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #4338ca var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(67 56 202 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo700 => "from-indigo-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #3730a3 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(55 48 163 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo800 => "from-indigo-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #312e81 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(49 46 129 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo900 => "from-indigo-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #1e1b4b var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(30 27 75 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromIndigo950 => "from-indigo-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f5f3ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(245 243 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet50 => "from-violet-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ede9fe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(237 233 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet100 => "from-violet-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ddd6fe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(221 214 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet200 => "from-violet-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #c4b5fd var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(196 181 253 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet300 => "from-violet-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a78bfa var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(167 139 250 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet400 => "from-violet-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #8b5cf6 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(139 92 246 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet500 => "from-violet-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #7c3aed var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(124 58 237 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet600 => "from-violet-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #6d28d9 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(109 40 217 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet700 => "from-violet-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #5b21b6 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(91 33 182 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet800 => "from-violet-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #4c1d95 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(76 29 149 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet900 => "from-violet-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #2e1065 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(46 16 101 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromViolet950 => "from-violet-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #faf5ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(250 245 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple50 => "from-purple-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f3e8ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(243 232 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple100 => "from-purple-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e9d5ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(233 213 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple200 => "from-purple-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d8b4fe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(216 180 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple300 => "from-purple-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #c084fc var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(192 132 252 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple400 => "from-purple-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a855f7 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(168 85 247 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple500 => "from-purple-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #9333ea var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(147 51 234 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple600 => "from-purple-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #7e22ce var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(126 34 206 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple700 => "from-purple-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #6b21a8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(107 33 168 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple800 => "from-purple-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #581c87 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(88 28 135 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple900 => "from-purple-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #3b0764 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(59 7 100 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPurple950 => "from-purple-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fdf4ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(253 244 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia50 => "from-fuchsia-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fae8ff var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(250 232 255 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia100 => "from-fuchsia-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f5d0fe var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(245 208 254 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia200 => "from-fuchsia-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f0abfc var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(240 171 252 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia300 => "from-fuchsia-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e879f9 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(232 121 249 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia400 => "from-fuchsia-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #d946ef var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(217 70 239 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia500 => "from-fuchsia-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #c026d3 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(192 38 211 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia600 => "from-fuchsia-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #a21caf var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(162 28 175 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia700 => "from-fuchsia-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #86198f var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(134 25 143 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia800 => "from-fuchsia-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #701a75 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(112 26 117 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia900 => "from-fuchsia-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #4a044e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(74 4 78 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromFuchsia950 => "from-fuchsia-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fdf2f8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(253 242 248 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink50 => "from-pink-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fce7f3 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(252 231 243 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink100 => "from-pink-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fbcfe8 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(251 207 232 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink200 => "from-pink-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f9a8d4 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(249 168 212 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink300 => "from-pink-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f472b6 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(244 114 182 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink400 => "from-pink-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ec4899 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(236 72 153 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink500 => "from-pink-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #db2777 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(219 39 119 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink600 => "from-pink-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #be185d var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(190 24 93 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink700 => "from-pink-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #9d174d var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(157 23 77 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink800 => "from-pink-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #831843 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(131 24 67 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink900 => "from-pink-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #500724 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(80 7 36 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromPink950 => "from-pink-950",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fff1f2 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(255 241 242 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose50 => "from-rose-50",
        /// ```css
        /// {
        ///     --tw-gradient-from: #ffe4e6 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(255 228 230 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose100 => "from-rose-100",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fecdd3 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(254 205 211 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose200 => "from-rose-200",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fda4af var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(253 164 175 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose300 => "from-rose-300",
        /// ```css
        /// {
        ///     --tw-gradient-from: #fb7185 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(251 113 133 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose400 => "from-rose-400",
        /// ```css
        /// {
        ///     --tw-gradient-from: #f43f5e var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(244 63 94 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose500 => "from-rose-500",
        /// ```css
        /// {
        ///     --tw-gradient-from: #e11d48 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(225 29 72 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose600 => "from-rose-600",
        /// ```css
        /// {
        ///     --tw-gradient-from: #be123c var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(190 18 60 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose700 => "from-rose-700",
        /// ```css
        /// {
        ///     --tw-gradient-from: #9f1239 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(159 18 57 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose800 => "from-rose-800",
        /// ```css
        /// {
        ///     --tw-gradient-from: #881337 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(136 19 55 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose900 => "from-rose-900",
        /// ```css
        /// {
        ///     --tw-gradient-from: #4c0519 var(--tw-gradient-from-position);
        ///     --tw-gradient-to: rgb(76 5 25 / 0) var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);
        /// }
        /// ```
        FromRose950 => "from-rose-950",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 0%;
        /// }
        /// ```
        From0Percent => "from-0%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 5%;
        /// }
        /// ```
        From5Percent => "from-5%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 10%;
        /// }
        /// ```
        From10Percent => "from-10%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 15%;
        /// }
        /// ```
        From15Percent => "from-15%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 20%;
        /// }
        /// ```
        From20Percent => "from-20%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 25%;
        /// }
        /// ```
        From25Percent => "from-25%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 30%;
        /// }
        /// ```
        From30Percent => "from-30%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 35%;
        /// }
        /// ```
        From35Percent => "from-35%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 40%;
        /// }
        /// ```
        From40Percent => "from-40%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 45%;
        /// }
        /// ```
        From45Percent => "from-45%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 50%;
        /// }
        /// ```
        From50Percent => "from-50%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 55%;
        /// }
        /// ```
        From55Percent => "from-55%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 60%;
        /// }
        /// ```
        From60Percent => "from-60%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 65%;
        /// }
        /// ```
        From65Percent => "from-65%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 70%;
        /// }
        /// ```
        From70Percent => "from-70%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 75%;
        /// }
        /// ```
        From75Percent => "from-75%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 80%;
        /// }
        /// ```
        From80Percent => "from-80%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 85%;
        /// }
        /// ```
        From85Percent => "from-85%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 90%;
        /// }
        /// ```
        From90Percent => "from-90%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 95%;
        /// }
        /// ```
        From95Percent => "from-95%",
        /// ```css
        /// {
        ///     --tw-gradient-from-position: 100%;
        /// }
        /// ```
        From100Percent => "from-100%",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(255 255 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), inherit var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaInherit => "via-inherit",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(255 255 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), currentColor var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCurrent => "via-current",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(0 0 0 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), transparent var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTransparent => "via-transparent",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(0 0 0 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #000 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlack => "via-black",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(255 255 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaWhite => "via-white",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(248 250 252 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f8fafc var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate50 => "via-slate-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(241 245 249 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f1f5f9 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate100 => "via-slate-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(226 232 240 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e2e8f0 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate200 => "via-slate-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(203 213 225 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #cbd5e1 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate300 => "via-slate-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(148 163 184 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #94a3b8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate400 => "via-slate-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(100 116 139 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #64748b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate500 => "via-slate-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(71 85 105 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #475569 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate600 => "via-slate-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(51 65 85 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #334155 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate700 => "via-slate-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(30 41 59 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #1e293b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate800 => "via-slate-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(15 23 42 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0f172a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate900 => "via-slate-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(2 6 23 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #020617 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSlate950 => "via-slate-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(249 250 251 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f9fafb var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray50 => "via-gray-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(243 244 246 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f3f4f6 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray100 => "via-gray-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(229 231 235 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e5e7eb var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray200 => "via-gray-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(209 213 219 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d1d5db var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray300 => "via-gray-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(156 163 175 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #9ca3af var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray400 => "via-gray-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(107 114 128 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #6b7280 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray500 => "via-gray-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(75 85 99 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #4b5563 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray600 => "via-gray-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(55 65 81 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #374151 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray700 => "via-gray-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(31 41 55 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #1f2937 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray800 => "via-gray-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(17 24 39 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #111827 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray900 => "via-gray-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(3 7 18 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #030712 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGray950 => "via-gray-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(250 250 250 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fafafa var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc50 => "via-zinc-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(244 244 245 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f4f4f5 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc100 => "via-zinc-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(228 228 231 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e4e4e7 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc200 => "via-zinc-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(212 212 216 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d4d4d8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc300 => "via-zinc-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(161 161 170 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a1a1aa var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc400 => "via-zinc-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(113 113 122 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #71717a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc500 => "via-zinc-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(82 82 91 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #52525b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc600 => "via-zinc-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(63 63 70 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #3f3f46 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc700 => "via-zinc-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(39 39 42 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #27272a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc800 => "via-zinc-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(24 24 27 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #18181b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc900 => "via-zinc-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(9 9 11 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #09090b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaZinc950 => "via-zinc-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(250 250 250 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fafafa var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral50 => "via-neutral-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(245 245 245 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f5f5f5 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral100 => "via-neutral-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(229 229 229 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e5e5e5 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral200 => "via-neutral-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(212 212 212 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d4d4d4 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral300 => "via-neutral-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(163 163 163 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a3a3a3 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral400 => "via-neutral-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(115 115 115 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #737373 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral500 => "via-neutral-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(82 82 82 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #525252 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral600 => "via-neutral-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(64 64 64 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #404040 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral700 => "via-neutral-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(38 38 38 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #262626 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral800 => "via-neutral-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(23 23 23 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #171717 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral900 => "via-neutral-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(10 10 10 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0a0a0a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaNeutral950 => "via-neutral-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(250 250 249 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fafaf9 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone50 => "via-stone-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(245 245 244 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f5f5f4 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone100 => "via-stone-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(231 229 228 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e7e5e4 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone200 => "via-stone-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(214 211 209 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d6d3d1 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone300 => "via-stone-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(168 162 158 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a8a29e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone400 => "via-stone-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(120 113 108 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #78716c var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone500 => "via-stone-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(87 83 78 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #57534e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone600 => "via-stone-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(68 64 60 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #44403c var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone700 => "via-stone-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(41 37 36 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #292524 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone800 => "via-stone-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(28 25 23 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #1c1917 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone900 => "via-stone-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(12 10 9 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0c0a09 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaStone950 => "via-stone-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 242 242 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fef2f2 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed50 => "via-red-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 226 226 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fee2e2 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed100 => "via-red-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 202 202 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fecaca var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed200 => "via-red-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(252 165 165 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fca5a5 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed300 => "via-red-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(248 113 113 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f87171 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed400 => "via-red-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(239 68 68 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ef4444 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed500 => "via-red-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(220 38 38 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #dc2626 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed600 => "via-red-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(185 28 28 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #b91c1c var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed700 => "via-red-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(153 27 27 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #991b1b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed800 => "via-red-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(127 29 29 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #7f1d1d var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed900 => "via-red-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(69 10 10 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #450a0a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRed950 => "via-red-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(255 247 237 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fff7ed var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange50 => "via-orange-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(255 237 213 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ffedd5 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange100 => "via-orange-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 215 170 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fed7aa var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange200 => "via-orange-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(253 186 116 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fdba74 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange300 => "via-orange-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(251 146 60 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fb923c var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange400 => "via-orange-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(249 115 22 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f97316 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange500 => "via-orange-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(234 88 12 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ea580c var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange600 => "via-orange-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(194 65 12 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #c2410c var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange700 => "via-orange-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(154 52 18 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #9a3412 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange800 => "via-orange-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(124 45 18 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #7c2d12 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange900 => "via-orange-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(67 20 7 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #431407 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaOrange950 => "via-orange-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(255 251 235 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fffbeb var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber50 => "via-amber-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 243 199 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fef3c7 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber100 => "via-amber-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(253 230 138 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fde68a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber200 => "via-amber-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(252 211 77 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fcd34d var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber300 => "via-amber-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(251 191 36 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fbbf24 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber400 => "via-amber-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(245 158 11 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f59e0b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber500 => "via-amber-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(217 119 6 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d97706 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber600 => "via-amber-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(180 83 9 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #b45309 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber700 => "via-amber-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(146 64 14 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #92400e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber800 => "via-amber-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(120 53 15 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #78350f var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber900 => "via-amber-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(69 26 3 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #451a03 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaAmber950 => "via-amber-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 252 232 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fefce8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow50 => "via-yellow-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 249 195 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fef9c3 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow100 => "via-yellow-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 240 138 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fef08a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow200 => "via-yellow-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(253 224 71 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fde047 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow300 => "via-yellow-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(250 204 21 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #facc15 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow400 => "via-yellow-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(234 179 8 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #eab308 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow500 => "via-yellow-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(202 138 4 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ca8a04 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow600 => "via-yellow-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(161 98 7 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a16207 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow700 => "via-yellow-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(133 77 14 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #854d0e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow800 => "via-yellow-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(113 63 18 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #713f12 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow900 => "via-yellow-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(66 32 6 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #422006 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaYellow950 => "via-yellow-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(247 254 231 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f7fee7 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime50 => "via-lime-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(236 252 203 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ecfccb var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime100 => "via-lime-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(217 249 157 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d9f99d var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime200 => "via-lime-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(190 242 100 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #bef264 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime300 => "via-lime-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(163 230 53 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a3e635 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime400 => "via-lime-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(132 204 22 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #84cc16 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime500 => "via-lime-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(101 163 13 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #65a30d var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime600 => "via-lime-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(77 124 15 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #4d7c0f var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime700 => "via-lime-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(63 98 18 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #3f6212 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime800 => "via-lime-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(54 83 20 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #365314 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime900 => "via-lime-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(26 46 5 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #1a2e05 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaLime950 => "via-lime-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(240 253 244 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f0fdf4 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen50 => "via-green-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(220 252 231 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #dcfce7 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen100 => "via-green-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(187 247 208 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #bbf7d0 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen200 => "via-green-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(134 239 172 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #86efac var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen300 => "via-green-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(74 222 128 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #4ade80 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen400 => "via-green-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(34 197 94 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #22c55e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen500 => "via-green-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(22 163 74 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #16a34a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen600 => "via-green-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(21 128 61 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #15803d var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen700 => "via-green-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(22 101 52 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #166534 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen800 => "via-green-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(20 83 45 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #14532d var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen900 => "via-green-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(5 46 22 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #052e16 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaGreen950 => "via-green-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(236 253 245 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ecfdf5 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald50 => "via-emerald-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(209 250 229 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d1fae5 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald100 => "via-emerald-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(167 243 208 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a7f3d0 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald200 => "via-emerald-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(110 231 183 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #6ee7b7 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald300 => "via-emerald-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(52 211 153 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #34d399 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald400 => "via-emerald-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(16 185 129 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #10b981 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald500 => "via-emerald-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(5 150 105 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #059669 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald600 => "via-emerald-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(4 120 87 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #047857 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald700 => "via-emerald-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(6 95 70 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #065f46 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald800 => "via-emerald-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(6 78 59 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #064e3b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald900 => "via-emerald-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(2 44 34 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #022c22 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaEmerald950 => "via-emerald-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(240 253 250 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f0fdfa var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal50 => "via-teal-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(204 251 241 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ccfbf1 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal100 => "via-teal-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(153 246 228 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #99f6e4 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal200 => "via-teal-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(94 234 212 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #5eead4 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal300 => "via-teal-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(45 212 191 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #2dd4bf var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal400 => "via-teal-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(20 184 166 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #14b8a6 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal500 => "via-teal-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(13 148 136 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0d9488 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal600 => "via-teal-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(15 118 110 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0f766e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal700 => "via-teal-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(17 94 89 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #115e59 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal800 => "via-teal-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(19 78 74 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #134e4a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal900 => "via-teal-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(4 47 46 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #042f2e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaTeal950 => "via-teal-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(236 254 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ecfeff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan50 => "via-cyan-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(207 250 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #cffafe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan100 => "via-cyan-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(165 243 252 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a5f3fc var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan200 => "via-cyan-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(103 232 249 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #67e8f9 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan300 => "via-cyan-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(34 211 238 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #22d3ee var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan400 => "via-cyan-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(6 182 212 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #06b6d4 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan500 => "via-cyan-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(8 145 178 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0891b2 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan600 => "via-cyan-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(14 116 144 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0e7490 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan700 => "via-cyan-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(21 94 117 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #155e75 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan800 => "via-cyan-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(22 78 99 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #164e63 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan900 => "via-cyan-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(8 51 68 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #083344 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaCyan950 => "via-cyan-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(240 249 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f0f9ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky50 => "via-sky-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(224 242 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e0f2fe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky100 => "via-sky-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(186 230 253 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #bae6fd var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky200 => "via-sky-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(125 211 252 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #7dd3fc var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky300 => "via-sky-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(56 189 248 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #38bdf8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky400 => "via-sky-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(14 165 233 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0ea5e9 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky500 => "via-sky-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(2 132 199 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0284c7 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky600 => "via-sky-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(3 105 161 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0369a1 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky700 => "via-sky-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(7 89 133 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #075985 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky800 => "via-sky-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(12 74 110 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #0c4a6e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky900 => "via-sky-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(8 47 73 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #082f49 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaSky950 => "via-sky-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(239 246 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #eff6ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue50 => "via-blue-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(219 234 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #dbeafe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue100 => "via-blue-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(191 219 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #bfdbfe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue200 => "via-blue-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(147 197 253 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #93c5fd var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue300 => "via-blue-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(96 165 250 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #60a5fa var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue400 => "via-blue-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(59 130 246 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #3b82f6 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue500 => "via-blue-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(37 99 235 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #2563eb var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue600 => "via-blue-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(29 78 216 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #1d4ed8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue700 => "via-blue-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(30 64 175 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #1e40af var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue800 => "via-blue-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(30 58 138 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #1e3a8a var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue900 => "via-blue-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(23 37 84 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #172554 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaBlue950 => "via-blue-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(238 242 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #eef2ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo50 => "via-indigo-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(224 231 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e0e7ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo100 => "via-indigo-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(199 210 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #c7d2fe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo200 => "via-indigo-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(165 180 252 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a5b4fc var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo300 => "via-indigo-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(129 140 248 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #818cf8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo400 => "via-indigo-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(99 102 241 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #6366f1 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo500 => "via-indigo-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(79 70 229 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #4f46e5 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo600 => "via-indigo-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(67 56 202 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #4338ca var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo700 => "via-indigo-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(55 48 163 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #3730a3 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo800 => "via-indigo-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(49 46 129 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #312e81 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo900 => "via-indigo-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(30 27 75 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #1e1b4b var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaIndigo950 => "via-indigo-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(245 243 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f5f3ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet50 => "via-violet-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(237 233 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ede9fe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet100 => "via-violet-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(221 214 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ddd6fe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet200 => "via-violet-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(196 181 253 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #c4b5fd var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet300 => "via-violet-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(167 139 250 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a78bfa var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet400 => "via-violet-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(139 92 246 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #8b5cf6 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet500 => "via-violet-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(124 58 237 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #7c3aed var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet600 => "via-violet-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(109 40 217 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #6d28d9 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet700 => "via-violet-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(91 33 182 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #5b21b6 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet800 => "via-violet-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(76 29 149 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #4c1d95 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet900 => "via-violet-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(46 16 101 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #2e1065 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaViolet950 => "via-violet-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(250 245 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #faf5ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple50 => "via-purple-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(243 232 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f3e8ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple100 => "via-purple-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(233 213 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e9d5ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple200 => "via-purple-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(216 180 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d8b4fe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple300 => "via-purple-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(192 132 252 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #c084fc var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple400 => "via-purple-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(168 85 247 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a855f7 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple500 => "via-purple-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(147 51 234 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #9333ea var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple600 => "via-purple-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(126 34 206 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #7e22ce var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple700 => "via-purple-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(107 33 168 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #6b21a8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple800 => "via-purple-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(88 28 135 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #581c87 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple900 => "via-purple-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(59 7 100 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #3b0764 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPurple950 => "via-purple-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(253 244 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fdf4ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia50 => "via-fuchsia-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(250 232 255 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fae8ff var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia100 => "via-fuchsia-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(245 208 254 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f5d0fe var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia200 => "via-fuchsia-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(240 171 252 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f0abfc var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia300 => "via-fuchsia-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(232 121 249 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e879f9 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia400 => "via-fuchsia-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(217 70 239 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #d946ef var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia500 => "via-fuchsia-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(192 38 211 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #c026d3 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia600 => "via-fuchsia-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(162 28 175 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #a21caf var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia700 => "via-fuchsia-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(134 25 143 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #86198f var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia800 => "via-fuchsia-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(112 26 117 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #701a75 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia900 => "via-fuchsia-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(74 4 78 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #4a044e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaFuchsia950 => "via-fuchsia-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(253 242 248 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fdf2f8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink50 => "via-pink-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(252 231 243 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fce7f3 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink100 => "via-pink-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(251 207 232 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fbcfe8 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink200 => "via-pink-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(249 168 212 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f9a8d4 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink300 => "via-pink-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(244 114 182 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f472b6 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink400 => "via-pink-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(236 72 153 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ec4899 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink500 => "via-pink-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(219 39 119 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #db2777 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink600 => "via-pink-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(190 24 93 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #be185d var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink700 => "via-pink-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(157 23 77 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #9d174d var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink800 => "via-pink-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(131 24 67 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #831843 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink900 => "via-pink-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(80 7 36 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #500724 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaPink950 => "via-pink-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(255 241 242 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fff1f2 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose50 => "via-rose-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(255 228 230 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #ffe4e6 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose100 => "via-rose-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(254 205 211 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fecdd3 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose200 => "via-rose-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(253 164 175 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fda4af var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose300 => "via-rose-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(251 113 133 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #fb7185 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose400 => "via-rose-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(244 63 94 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #f43f5e var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose500 => "via-rose-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(225 29 72 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #e11d48 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose600 => "via-rose-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(190 18 60 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #be123c var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose700 => "via-rose-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(159 18 57 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #9f1239 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose800 => "via-rose-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(136 19 55 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #881337 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose900 => "via-rose-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: rgb(76 5 25 / 0)  var(--tw-gradient-to-position);
        ///     --tw-gradient-stops: var(--tw-gradient-from), #4c0519 var(--tw-gradient-via-position), var(--tw-gradient-to);
        /// }
        /// ```
        ViaRose950 => "via-rose-950",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 0%;
        /// }
        /// ```
        Via0Percent => "via-0%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 5%;
        /// }
        /// ```
        Via5Percent => "via-5%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 10%;
        /// }
        /// ```
        Via10Percent => "via-10%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 15%;
        /// }
        /// ```
        Via15Percent => "via-15%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 20%;
        /// }
        /// ```
        Via20Percent => "via-20%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 25%;
        /// }
        /// ```
        Via25Percent => "via-25%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 30%;
        /// }
        /// ```
        Via30Percent => "via-30%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 35%;
        /// }
        /// ```
        Via35Percent => "via-35%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 40%;
        /// }
        /// ```
        Via40Percent => "via-40%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 45%;
        /// }
        /// ```
        Via45Percent => "via-45%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 50%;
        /// }
        /// ```
        Via50Percent => "via-50%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 55%;
        /// }
        /// ```
        Via55Percent => "via-55%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 60%;
        /// }
        /// ```
        Via60Percent => "via-60%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 65%;
        /// }
        /// ```
        Via65Percent => "via-65%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 70%;
        /// }
        /// ```
        Via70Percent => "via-70%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 75%;
        /// }
        /// ```
        Via75Percent => "via-75%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 80%;
        /// }
        /// ```
        Via80Percent => "via-80%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 85%;
        /// }
        /// ```
        Via85Percent => "via-85%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 90%;
        /// }
        /// ```
        Via90Percent => "via-90%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 95%;
        /// }
        /// ```
        Via95Percent => "via-95%",
        /// ```css
        /// {
        ///     --tw-gradient-via-position: 100%;
        /// }
        /// ```
        Via100Percent => "via-100%",
        /// ```css
        /// {
        ///     --tw-gradient-to: inherit var(--tw-gradient-to-position);
        /// }
        /// ```
        ToInherit => "to-inherit",
        /// ```css
        /// {
        ///     --tw-gradient-to: currentColor var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCurrent => "to-current",
        /// ```css
        /// {
        ///     --tw-gradient-to: transparent var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTransparent => "to-transparent",
        /// ```css
        /// {
        ///     --tw-gradient-to: #000 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlack => "to-black",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToWhite => "to-white",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f8fafc var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate50 => "to-slate-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f1f5f9 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate100 => "to-slate-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e2e8f0 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate200 => "to-slate-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #cbd5e1 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate300 => "to-slate-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #94a3b8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate400 => "to-slate-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #64748b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate500 => "to-slate-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #475569 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate600 => "to-slate-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #334155 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate700 => "to-slate-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #1e293b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate800 => "to-slate-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0f172a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate900 => "to-slate-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #020617 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSlate950 => "to-slate-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f9fafb var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray50 => "to-gray-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f3f4f6 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray100 => "to-gray-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e5e7eb var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray200 => "to-gray-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d1d5db var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray300 => "to-gray-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #9ca3af var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray400 => "to-gray-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #6b7280 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray500 => "to-gray-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #4b5563 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray600 => "to-gray-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #374151 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray700 => "to-gray-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #1f2937 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray800 => "to-gray-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #111827 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray900 => "to-gray-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #030712 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGray950 => "to-gray-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fafafa var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc50 => "to-zinc-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f4f4f5 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc100 => "to-zinc-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e4e4e7 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc200 => "to-zinc-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d4d4d8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc300 => "to-zinc-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a1a1aa var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc400 => "to-zinc-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #71717a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc500 => "to-zinc-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #52525b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc600 => "to-zinc-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #3f3f46 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc700 => "to-zinc-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #27272a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc800 => "to-zinc-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #18181b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc900 => "to-zinc-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #09090b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToZinc950 => "to-zinc-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fafafa var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral50 => "to-neutral-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f5f5f5 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral100 => "to-neutral-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e5e5e5 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral200 => "to-neutral-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d4d4d4 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral300 => "to-neutral-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a3a3a3 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral400 => "to-neutral-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #737373 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral500 => "to-neutral-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #525252 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral600 => "to-neutral-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #404040 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral700 => "to-neutral-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #262626 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral800 => "to-neutral-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #171717 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral900 => "to-neutral-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0a0a0a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToNeutral950 => "to-neutral-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fafaf9 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone50 => "to-stone-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f5f5f4 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone100 => "to-stone-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e7e5e4 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone200 => "to-stone-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d6d3d1 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone300 => "to-stone-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a8a29e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone400 => "to-stone-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #78716c var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone500 => "to-stone-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #57534e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone600 => "to-stone-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #44403c var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone700 => "to-stone-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #292524 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone800 => "to-stone-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #1c1917 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone900 => "to-stone-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0c0a09 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToStone950 => "to-stone-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fef2f2 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed50 => "to-red-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fee2e2 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed100 => "to-red-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fecaca var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed200 => "to-red-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fca5a5 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed300 => "to-red-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f87171 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed400 => "to-red-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ef4444 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed500 => "to-red-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #dc2626 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed600 => "to-red-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #b91c1c var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed700 => "to-red-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #991b1b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed800 => "to-red-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #7f1d1d var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed900 => "to-red-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #450a0a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRed950 => "to-red-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fff7ed var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange50 => "to-orange-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ffedd5 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange100 => "to-orange-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fed7aa var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange200 => "to-orange-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fdba74 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange300 => "to-orange-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fb923c var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange400 => "to-orange-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f97316 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange500 => "to-orange-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ea580c var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange600 => "to-orange-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #c2410c var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange700 => "to-orange-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #9a3412 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange800 => "to-orange-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #7c2d12 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange900 => "to-orange-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #431407 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToOrange950 => "to-orange-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fffbeb var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber50 => "to-amber-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fef3c7 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber100 => "to-amber-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fde68a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber200 => "to-amber-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fcd34d var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber300 => "to-amber-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fbbf24 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber400 => "to-amber-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f59e0b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber500 => "to-amber-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d97706 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber600 => "to-amber-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #b45309 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber700 => "to-amber-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #92400e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber800 => "to-amber-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #78350f var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber900 => "to-amber-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #451a03 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToAmber950 => "to-amber-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fefce8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow50 => "to-yellow-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fef9c3 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow100 => "to-yellow-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fef08a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow200 => "to-yellow-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fde047 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow300 => "to-yellow-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #facc15 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow400 => "to-yellow-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #eab308 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow500 => "to-yellow-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ca8a04 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow600 => "to-yellow-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a16207 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow700 => "to-yellow-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #854d0e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow800 => "to-yellow-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #713f12 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow900 => "to-yellow-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #422006 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToYellow950 => "to-yellow-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f7fee7 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime50 => "to-lime-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ecfccb var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime100 => "to-lime-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d9f99d var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime200 => "to-lime-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #bef264 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime300 => "to-lime-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a3e635 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime400 => "to-lime-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #84cc16 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime500 => "to-lime-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #65a30d var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime600 => "to-lime-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #4d7c0f var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime700 => "to-lime-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #3f6212 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime800 => "to-lime-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #365314 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime900 => "to-lime-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #1a2e05 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToLime950 => "to-lime-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f0fdf4 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen50 => "to-green-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #dcfce7 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen100 => "to-green-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #bbf7d0 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen200 => "to-green-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #86efac var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen300 => "to-green-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #4ade80 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen400 => "to-green-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #22c55e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen500 => "to-green-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #16a34a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen600 => "to-green-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #15803d var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen700 => "to-green-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #166534 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen800 => "to-green-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #14532d var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen900 => "to-green-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #052e16 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToGreen950 => "to-green-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ecfdf5 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald50 => "to-emerald-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d1fae5 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald100 => "to-emerald-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a7f3d0 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald200 => "to-emerald-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #6ee7b7 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald300 => "to-emerald-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #34d399 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald400 => "to-emerald-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #10b981 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald500 => "to-emerald-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #059669 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald600 => "to-emerald-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #047857 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald700 => "to-emerald-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #065f46 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald800 => "to-emerald-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #064e3b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald900 => "to-emerald-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #022c22 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToEmerald950 => "to-emerald-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f0fdfa var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal50 => "to-teal-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ccfbf1 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal100 => "to-teal-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #99f6e4 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal200 => "to-teal-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #5eead4 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal300 => "to-teal-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #2dd4bf var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal400 => "to-teal-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #14b8a6 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal500 => "to-teal-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0d9488 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal600 => "to-teal-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0f766e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal700 => "to-teal-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #115e59 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal800 => "to-teal-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #134e4a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal900 => "to-teal-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #042f2e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToTeal950 => "to-teal-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ecfeff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan50 => "to-cyan-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #cffafe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan100 => "to-cyan-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a5f3fc var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan200 => "to-cyan-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #67e8f9 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan300 => "to-cyan-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #22d3ee var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan400 => "to-cyan-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #06b6d4 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan500 => "to-cyan-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0891b2 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan600 => "to-cyan-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0e7490 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan700 => "to-cyan-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #155e75 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan800 => "to-cyan-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #164e63 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan900 => "to-cyan-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #083344 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToCyan950 => "to-cyan-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f0f9ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky50 => "to-sky-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e0f2fe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky100 => "to-sky-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #bae6fd var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky200 => "to-sky-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #7dd3fc var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky300 => "to-sky-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #38bdf8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky400 => "to-sky-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0ea5e9 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky500 => "to-sky-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0284c7 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky600 => "to-sky-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0369a1 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky700 => "to-sky-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #075985 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky800 => "to-sky-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #0c4a6e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky900 => "to-sky-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #082f49 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToSky950 => "to-sky-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #eff6ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue50 => "to-blue-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #dbeafe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue100 => "to-blue-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #bfdbfe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue200 => "to-blue-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #93c5fd var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue300 => "to-blue-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #60a5fa var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue400 => "to-blue-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #3b82f6 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue500 => "to-blue-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #2563eb var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue600 => "to-blue-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #1d4ed8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue700 => "to-blue-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #1e40af var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue800 => "to-blue-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #1e3a8a var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue900 => "to-blue-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #172554 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToBlue950 => "to-blue-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #eef2ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo50 => "to-indigo-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e0e7ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo100 => "to-indigo-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #c7d2fe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo200 => "to-indigo-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a5b4fc var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo300 => "to-indigo-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #818cf8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo400 => "to-indigo-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #6366f1 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo500 => "to-indigo-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #4f46e5 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo600 => "to-indigo-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #4338ca var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo700 => "to-indigo-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #3730a3 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo800 => "to-indigo-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #312e81 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo900 => "to-indigo-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #1e1b4b var(--tw-gradient-to-position);
        /// }
        /// ```
        ToIndigo950 => "to-indigo-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f5f3ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet50 => "to-violet-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ede9fe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet100 => "to-violet-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ddd6fe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet200 => "to-violet-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #c4b5fd var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet300 => "to-violet-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a78bfa var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet400 => "to-violet-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #8b5cf6 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet500 => "to-violet-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #7c3aed var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet600 => "to-violet-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #6d28d9 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet700 => "to-violet-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #5b21b6 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet800 => "to-violet-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #4c1d95 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet900 => "to-violet-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #2e1065 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToViolet950 => "to-violet-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #faf5ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple50 => "to-purple-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f3e8ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple100 => "to-purple-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e9d5ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple200 => "to-purple-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d8b4fe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple300 => "to-purple-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #c084fc var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple400 => "to-purple-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a855f7 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple500 => "to-purple-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #9333ea var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple600 => "to-purple-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #7e22ce var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple700 => "to-purple-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #6b21a8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple800 => "to-purple-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #581c87 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple900 => "to-purple-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #3b0764 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPurple950 => "to-purple-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fdf4ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia50 => "to-fuchsia-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fae8ff var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia100 => "to-fuchsia-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f5d0fe var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia200 => "to-fuchsia-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f0abfc var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia300 => "to-fuchsia-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e879f9 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia400 => "to-fuchsia-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #d946ef var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia500 => "to-fuchsia-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #c026d3 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia600 => "to-fuchsia-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #a21caf var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia700 => "to-fuchsia-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #86198f var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia800 => "to-fuchsia-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #701a75 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia900 => "to-fuchsia-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #4a044e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToFuchsia950 => "to-fuchsia-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fdf2f8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink50 => "to-pink-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fce7f3 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink100 => "to-pink-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fbcfe8 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink200 => "to-pink-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f9a8d4 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink300 => "to-pink-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f472b6 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink400 => "to-pink-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ec4899 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink500 => "to-pink-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #db2777 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink600 => "to-pink-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #be185d var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink700 => "to-pink-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #9d174d var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink800 => "to-pink-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #831843 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink900 => "to-pink-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #500724 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToPink950 => "to-pink-950",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fff1f2 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose50 => "to-rose-50",
        /// ```css
        /// {
        ///     --tw-gradient-to: #ffe4e6 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose100 => "to-rose-100",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fecdd3 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose200 => "to-rose-200",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fda4af var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose300 => "to-rose-300",
        /// ```css
        /// {
        ///     --tw-gradient-to: #fb7185 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose400 => "to-rose-400",
        /// ```css
        /// {
        ///     --tw-gradient-to: #f43f5e var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose500 => "to-rose-500",
        /// ```css
        /// {
        ///     --tw-gradient-to: #e11d48 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose600 => "to-rose-600",
        /// ```css
        /// {
        ///     --tw-gradient-to: #be123c var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose700 => "to-rose-700",
        /// ```css
        /// {
        ///     --tw-gradient-to: #9f1239 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose800 => "to-rose-800",
        /// ```css
        /// {
        ///     --tw-gradient-to: #881337 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose900 => "to-rose-900",
        /// ```css
        /// {
        ///     --tw-gradient-to: #4c0519 var(--tw-gradient-to-position);
        /// }
        /// ```
        ToRose950 => "to-rose-950",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 0%;
        /// }
        /// ```
        To0Percent => "to-0%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 5%;
        /// }
        /// ```
        To5Percent => "to-5%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 10%;
        /// }
        /// ```
        To10Percent => "to-10%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 15%;
        /// }
        /// ```
        To15Percent => "to-15%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 20%;
        /// }
        /// ```
        To20Percent => "to-20%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 25%;
        /// }
        /// ```
        To25Percent => "to-25%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 30%;
        /// }
        /// ```
        To30Percent => "to-30%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 35%;
        /// }
        /// ```
        To35Percent => "to-35%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 40%;
        /// }
        /// ```
        To40Percent => "to-40%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 45%;
        /// }
        /// ```
        To45Percent => "to-45%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 50%;
        /// }
        /// ```
        To50Percent => "to-50%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 55%;
        /// }
        /// ```
        To55Percent => "to-55%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 60%;
        /// }
        /// ```
        To60Percent => "to-60%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 65%;
        /// }
        /// ```
        To65Percent => "to-65%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 70%;
        /// }
        /// ```
        To70Percent => "to-70%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 75%;
        /// }
        /// ```
        To75Percent => "to-75%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 80%;
        /// }
        /// ```
        To80Percent => "to-80%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 85%;
        /// }
        /// ```
        To85Percent => "to-85%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 90%;
        /// }
        /// ```
        To90Percent => "to-90%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 95%;
        /// }
        /// ```
        To95Percent => "to-95%",
        /// ```css
        /// {
        ///     --tw-gradient-to-position: 100%;
        /// }
        /// ```
        To100Percent => "to-100%",
    }
);
