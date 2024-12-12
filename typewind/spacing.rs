def_types!(
    /// Utilities for controlling an element's padding.
    ///
    /// <https://tailwindcss.com/docs/padding>
    Padding {
        /// ```css
        /// {
        ///     padding: 0px;
        /// }
        /// ```
        P0 => "p-0",
        /// ```css
        /// {
        ///     padding-left: 0px;
        ///     padding-right: 0px;
        /// }
        /// ```
        Px0 => "px-0",
        /// ```css
        /// {
        ///     padding-top: 0px;
        ///     padding-bottom: 0px;
        /// }
        /// ```
        Py0 => "py-0",
        /// ```css
        /// {
        ///     padding-inline-start: 0px;
        /// }
        /// ```
        Ps0 => "ps-0",
        /// ```css
        /// {
        ///     padding-inline-end: 0px;
        /// }
        /// ```
        Pe0 => "pe-0",
        /// ```css
        /// {
        ///     padding-top: 0px;
        /// }
        /// ```
        Pt0 => "pt-0",
        /// ```css
        /// {
        ///     padding-right: 0px;
        /// }
        /// ```
        Pr0 => "pr-0",
        /// ```css
        /// {
        ///     padding-bottom: 0px;
        /// }
        /// ```
        Pb0 => "pb-0",
        /// ```css
        /// {
        ///     padding-left: 0px;
        /// }
        /// ```
        Pl0 => "pl-0",
        /// ```css
        /// {
        ///     padding: 1px;
        /// }
        /// ```
        PPx => "p-px",
        /// ```css
        /// {
        ///     padding-left: 1px;
        ///     padding-right: 1px;
        /// }
        /// ```
        PxPx => "px-px",
        /// ```css
        /// {
        ///     padding-top: 1px;
        ///     padding-bottom: 1px;
        /// }
        /// ```
        PyPx => "py-px",
        /// ```css
        /// {
        ///     padding-inline-start: 1px;
        /// }
        /// ```
        PsPx => "ps-px",
        /// ```css
        /// {
        ///     padding-inline-end: 1px;
        /// }
        /// ```
        PePx => "pe-px",
        /// ```css
        /// {
        ///     padding-top: 1px;
        /// }
        /// ```
        PtPx => "pt-px",
        /// ```css
        /// {
        ///     padding-right: 1px;
        /// }
        /// ```
        PrPx => "pr-px",
        /// ```css
        /// {
        ///     padding-bottom: 1px;
        /// }
        /// ```
        PbPx => "pb-px",
        /// ```css
        /// {
        ///     padding-left: 1px;
        /// }
        /// ```
        PlPx => "pl-px",
        /// ```css
        /// {
        ///     padding: 0.125rem; /* 2px */
        /// }
        /// ```
        P0_5 => "p-0.5",
        /// ```css
        /// {
        ///     padding-left: 0.125rem; /* 2px */
        ///     padding-right: 0.125rem; /* 2px */
        /// }
        /// ```
        Px0_5 => "px-0.5",
        /// ```css
        /// {
        ///     padding-top: 0.125rem; /* 2px */
        ///     padding-bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        Py0_5 => "py-0.5",
        /// ```css
        /// {
        ///     padding-inline-start: 0.125rem; /* 2px */
        /// }
        /// ```
        Ps0_5 => "ps-0.5",
        /// ```css
        /// {
        ///     padding-inline-end: 0.125rem; /* 2px */
        /// }
        /// ```
        Pe0_5 => "pe-0.5",
        /// ```css
        /// {
        ///     padding-top: 0.125rem; /* 2px */
        /// }
        /// ```
        Pt0_5 => "pt-0.5",
        /// ```css
        /// {
        ///     padding-right: 0.125rem; /* 2px */
        /// }
        /// ```
        Pr0_5 => "pr-0.5",
        /// ```css
        /// {
        ///     padding-bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        Pb0_5 => "pb-0.5",
        /// ```css
        /// {
        ///     padding-left: 0.125rem; /* 2px */
        /// }
        /// ```
        Pl0_5 => "pl-0.5",
        /// ```css
        /// {
        ///     padding: 0.25rem; /* 4px */
        /// }
        /// ```
        P1 => "p-1",
        /// ```css
        /// {
        ///     padding-left: 0.25rem; /* 4px */
        ///     padding-right: 0.25rem; /* 4px */
        /// }
        /// ```
        Px1 => "px-1",
        /// ```css
        /// {
        ///     padding-top: 0.25rem; /* 4px */
        ///     padding-bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        Py1 => "py-1",
        /// ```css
        /// {
        ///     padding-inline-start: 0.25rem; /* 4px */
        /// }
        /// ```
        Ps1 => "ps-1",
        /// ```css
        /// {
        ///     padding-inline-end: 0.25rem; /* 4px */
        /// }
        /// ```
        Pe1 => "pe-1",
        /// ```css
        /// {
        ///     padding-top: 0.25rem; /* 4px */
        /// }
        /// ```
        Pt1 => "pt-1",
        /// ```css
        /// {
        ///     padding-right: 0.25rem; /* 4px */
        /// }
        /// ```
        Pr1 => "pr-1",
        /// ```css
        /// {
        ///     padding-bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        Pb1 => "pb-1",
        /// ```css
        /// {
        ///     padding-left: 0.25rem; /* 4px */
        /// }
        /// ```
        Pl1 => "pl-1",
        /// ```css
        /// {
        ///     padding: 0.375rem; /* 6px */
        /// }
        /// ```
        P1_5 => "p-1.5",
        /// ```css
        /// {
        ///     padding-left: 0.375rem; /* 6px */
        ///     padding-right: 0.375rem; /* 6px */
        /// }
        /// ```
        Px1_5 => "px-1.5",
        /// ```css
        /// {
        ///     padding-top: 0.375rem; /* 6px */
        ///     padding-bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        Py1_5 => "py-1.5",
        /// ```css
        /// {
        ///     padding-inline-start: 0.375rem; /* 6px */
        /// }
        /// ```
        Ps1_5 => "ps-1.5",
        /// ```css
        /// {
        ///     padding-inline-end: 0.375rem; /* 6px */
        /// }
        /// ```
        Pe1_5 => "pe-1.5",
        /// ```css
        /// {
        ///     padding-top: 0.375rem; /* 6px */
        /// }
        /// ```
        Pt1_5 => "pt-1.5",
        /// ```css
        /// {
        ///     padding-right: 0.375rem; /* 6px */
        /// }
        /// ```
        Pr1_5 => "pr-1.5",
        /// ```css
        /// {
        ///     padding-bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        Pb1_5 => "pb-1.5",
        /// ```css
        /// {
        ///     padding-left: 0.375rem; /* 6px */
        /// }
        /// ```
        Pl1_5 => "pl-1.5",
        /// ```css
        /// {
        ///     padding: 0.5rem; /* 8px */
        /// }
        /// ```
        P2 => "p-2",
        /// ```css
        /// {
        ///     padding-left: 0.5rem; /* 8px */
        ///     padding-right: 0.5rem; /* 8px */
        /// }
        /// ```
        Px2 => "px-2",
        /// ```css
        /// {
        ///     padding-top: 0.5rem; /* 8px */
        ///     padding-bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        Py2 => "py-2",
        /// ```css
        /// {
        ///     padding-inline-start: 0.5rem; /* 8px */
        /// }
        /// ```
        Ps2 => "ps-2",
        /// ```css
        /// {
        ///     padding-inline-end: 0.5rem; /* 8px */
        /// }
        /// ```
        Pe2 => "pe-2",
        /// ```css
        /// {
        ///     padding-top: 0.5rem; /* 8px */
        /// }
        /// ```
        Pt2 => "pt-2",
        /// ```css
        /// {
        ///     padding-right: 0.5rem; /* 8px */
        /// }
        /// ```
        Pr2 => "pr-2",
        /// ```css
        /// {
        ///     padding-bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        Pb2 => "pb-2",
        /// ```css
        /// {
        ///     padding-left: 0.5rem; /* 8px */
        /// }
        /// ```
        Pl2 => "pl-2",
        /// ```css
        /// {
        ///     padding: 0.625rem; /* 10px */
        /// }
        /// ```
        P2_5 => "p-2.5",
        /// ```css
        /// {
        ///     padding-left: 0.625rem; /* 10px */
        ///     padding-right: 0.625rem; /* 10px */
        /// }
        /// ```
        Px2_5 => "px-2.5",
        /// ```css
        /// {
        ///     padding-top: 0.625rem; /* 10px */
        ///     padding-bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        Py2_5 => "py-2.5",
        /// ```css
        /// {
        ///     padding-inline-start: 0.625rem; /* 10px */
        /// }
        /// ```
        Ps2_5 => "ps-2.5",
        /// ```css
        /// {
        ///     padding-inline-end: 0.625rem; /* 10px */
        /// }
        /// ```
        Pe2_5 => "pe-2.5",
        /// ```css
        /// {
        ///     padding-top: 0.625rem; /* 10px */
        /// }
        /// ```
        Pt2_5 => "pt-2.5",
        /// ```css
        /// {
        ///     padding-right: 0.625rem; /* 10px */
        /// }
        /// ```
        Pr2_5 => "pr-2.5",
        /// ```css
        /// {
        ///     padding-bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        Pb2_5 => "pb-2.5",
        /// ```css
        /// {
        ///     padding-left: 0.625rem; /* 10px */
        /// }
        /// ```
        Pl2_5 => "pl-2.5",
        /// ```css
        /// {
        ///     padding: 0.75rem; /* 12px */
        /// }
        /// ```
        P3 => "p-3",
        /// ```css
        /// {
        ///     padding-left: 0.75rem; /* 12px */
        ///     padding-right: 0.75rem; /* 12px */
        /// }
        /// ```
        Px3 => "px-3",
        /// ```css
        /// {
        ///     padding-top: 0.75rem; /* 12px */
        ///     padding-bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        Py3 => "py-3",
        /// ```css
        /// {
        ///     padding-inline-start: 0.75rem; /* 12px */
        /// }
        /// ```
        Ps3 => "ps-3",
        /// ```css
        /// {
        ///     padding-inline-end: 0.75rem; /* 12px */
        /// }
        /// ```
        Pe3 => "pe-3",
        /// ```css
        /// {
        ///     padding-top: 0.75rem; /* 12px */
        /// }
        /// ```
        Pt3 => "pt-3",
        /// ```css
        /// {
        ///     padding-right: 0.75rem; /* 12px */
        /// }
        /// ```
        Pr3 => "pr-3",
        /// ```css
        /// {
        ///     padding-bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        Pb3 => "pb-3",
        /// ```css
        /// {
        ///     padding-left: 0.75rem; /* 12px */
        /// }
        /// ```
        Pl3 => "pl-3",
        /// ```css
        /// {
        ///     padding: 0.875rem; /* 14px */
        /// }
        /// ```
        P3_5 => "p-3.5",
        /// ```css
        /// {
        ///     padding-left: 0.875rem; /* 14px */
        ///     padding-right: 0.875rem; /* 14px */
        /// }
        /// ```
        Px3_5 => "px-3.5",
        /// ```css
        /// {
        ///     padding-top: 0.875rem; /* 14px */
        ///     padding-bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        Py3_5 => "py-3.5",
        /// ```css
        /// {
        ///     padding-inline-start: 0.875rem; /* 14px */
        /// }
        /// ```
        Ps3_5 => "ps-3.5",
        /// ```css
        /// {
        ///     padding-inline-end: 0.875rem; /* 14px */
        /// }
        /// ```
        Pe3_5 => "pe-3.5",
        /// ```css
        /// {
        ///     padding-top: 0.875rem; /* 14px */
        /// }
        /// ```
        Pt3_5 => "pt-3.5",
        /// ```css
        /// {
        ///     padding-right: 0.875rem; /* 14px */
        /// }
        /// ```
        Pr3_5 => "pr-3.5",
        /// ```css
        /// {
        ///     padding-bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        Pb3_5 => "pb-3.5",
        /// ```css
        /// {
        ///     padding-left: 0.875rem; /* 14px */
        /// }
        /// ```
        Pl3_5 => "pl-3.5",
        /// ```css
        /// {
        ///     padding: 1rem; /* 16px */
        /// }
        /// ```
        P4 => "p-4",
        /// ```css
        /// {
        ///     padding-left: 1rem; /* 16px */
        ///     padding-right: 1rem; /* 16px */
        /// }
        /// ```
        Px4 => "px-4",
        /// ```css
        /// {
        ///     padding-top: 1rem; /* 16px */
        ///     padding-bottom: 1rem; /* 16px */
        /// }
        /// ```
        Py4 => "py-4",
        /// ```css
        /// {
        ///     padding-inline-start: 1rem; /* 16px */
        /// }
        /// ```
        Ps4 => "ps-4",
        /// ```css
        /// {
        ///     padding-inline-end: 1rem; /* 16px */
        /// }
        /// ```
        Pe4 => "pe-4",
        /// ```css
        /// {
        ///     padding-top: 1rem; /* 16px */
        /// }
        /// ```
        Pt4 => "pt-4",
        /// ```css
        /// {
        ///     padding-right: 1rem; /* 16px */
        /// }
        /// ```
        Pr4 => "pr-4",
        /// ```css
        /// {
        ///     padding-bottom: 1rem; /* 16px */
        /// }
        /// ```
        Pb4 => "pb-4",
        /// ```css
        /// {
        ///     padding-left: 1rem; /* 16px */
        /// }
        /// ```
        Pl4 => "pl-4",
        /// ```css
        /// {
        ///     padding: 1.25rem; /* 20px */
        /// }
        /// ```
        P5 => "p-5",
        /// ```css
        /// {
        ///     padding-left: 1.25rem; /* 20px */
        ///     padding-right: 1.25rem; /* 20px */
        /// }
        /// ```
        Px5 => "px-5",
        /// ```css
        /// {
        ///     padding-top: 1.25rem; /* 20px */
        ///     padding-bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        Py5 => "py-5",
        /// ```css
        /// {
        ///     padding-inline-start: 1.25rem; /* 20px */
        /// }
        /// ```
        Ps5 => "ps-5",
        /// ```css
        /// {
        ///     padding-inline-end: 1.25rem; /* 20px */
        /// }
        /// ```
        Pe5 => "pe-5",
        /// ```css
        /// {
        ///     padding-top: 1.25rem; /* 20px */
        /// }
        /// ```
        Pt5 => "pt-5",
        /// ```css
        /// {
        ///     padding-right: 1.25rem; /* 20px */
        /// }
        /// ```
        Pr5 => "pr-5",
        /// ```css
        /// {
        ///     padding-bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        Pb5 => "pb-5",
        /// ```css
        /// {
        ///     padding-left: 1.25rem; /* 20px */
        /// }
        /// ```
        Pl5 => "pl-5",
        /// ```css
        /// {
        ///     padding: 1.5rem; /* 24px */
        /// }
        /// ```
        P6 => "p-6",
        /// ```css
        /// {
        ///     padding-left: 1.5rem; /* 24px */
        ///     padding-right: 1.5rem; /* 24px */
        /// }
        /// ```
        Px6 => "px-6",
        /// ```css
        /// {
        ///     padding-top: 1.5rem; /* 24px */
        ///     padding-bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        Py6 => "py-6",
        /// ```css
        /// {
        ///     padding-inline-start: 1.5rem; /* 24px */
        /// }
        /// ```
        Ps6 => "ps-6",
        /// ```css
        /// {
        ///     padding-inline-end: 1.5rem; /* 24px */
        /// }
        /// ```
        Pe6 => "pe-6",
        /// ```css
        /// {
        ///     padding-top: 1.5rem; /* 24px */
        /// }
        /// ```
        Pt6 => "pt-6",
        /// ```css
        /// {
        ///     padding-right: 1.5rem; /* 24px */
        /// }
        /// ```
        Pr6 => "pr-6",
        /// ```css
        /// {
        ///     padding-bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        Pb6 => "pb-6",
        /// ```css
        /// {
        ///     padding-left: 1.5rem; /* 24px */
        /// }
        /// ```
        Pl6 => "pl-6",
        /// ```css
        /// {
        ///     padding: 1.75rem; /* 28px */
        /// }
        /// ```
        P7 => "p-7",
        /// ```css
        /// {
        ///     padding-left: 1.75rem; /* 28px */
        ///     padding-right: 1.75rem; /* 28px */
        /// }
        /// ```
        Px7 => "px-7",
        /// ```css
        /// {
        ///     padding-top: 1.75rem; /* 28px */
        ///     padding-bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        Py7 => "py-7",
        /// ```css
        /// {
        ///     padding-inline-start: 1.75rem; /* 28px */
        /// }
        /// ```
        Ps7 => "ps-7",
        /// ```css
        /// {
        ///     padding-inline-end: 1.75rem; /* 28px */
        /// }
        /// ```
        Pe7 => "pe-7",
        /// ```css
        /// {
        ///     padding-top: 1.75rem; /* 28px */
        /// }
        /// ```
        Pt7 => "pt-7",
        /// ```css
        /// {
        ///     padding-right: 1.75rem; /* 28px */
        /// }
        /// ```
        Pr7 => "pr-7",
        /// ```css
        /// {
        ///     padding-bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        Pb7 => "pb-7",
        /// ```css
        /// {
        ///     padding-left: 1.75rem; /* 28px */
        /// }
        /// ```
        Pl7 => "pl-7",
        /// ```css
        /// {
        ///     padding: 2rem; /* 32px */
        /// }
        /// ```
        P8 => "p-8",
        /// ```css
        /// {
        ///     padding-left: 2rem; /* 32px */
        ///     padding-right: 2rem; /* 32px */
        /// }
        /// ```
        Px8 => "px-8",
        /// ```css
        /// {
        ///     padding-top: 2rem; /* 32px */
        ///     padding-bottom: 2rem; /* 32px */
        /// }
        /// ```
        Py8 => "py-8",
        /// ```css
        /// {
        ///     padding-inline-start: 2rem; /* 32px */
        /// }
        /// ```
        Ps8 => "ps-8",
        /// ```css
        /// {
        ///     padding-inline-end: 2rem; /* 32px */
        /// }
        /// ```
        Pe8 => "pe-8",
        /// ```css
        /// {
        ///     padding-top: 2rem; /* 32px */
        /// }
        /// ```
        Pt8 => "pt-8",
        /// ```css
        /// {
        ///     padding-right: 2rem; /* 32px */
        /// }
        /// ```
        Pr8 => "pr-8",
        /// ```css
        /// {
        ///     padding-bottom: 2rem; /* 32px */
        /// }
        /// ```
        Pb8 => "pb-8",
        /// ```css
        /// {
        ///     padding-left: 2rem; /* 32px */
        /// }
        /// ```
        Pl8 => "pl-8",
        /// ```css
        /// {
        ///     padding: 2.25rem; /* 36px */
        /// }
        /// ```
        P9 => "p-9",
        /// ```css
        /// {
        ///     padding-left: 2.25rem; /* 36px */
        ///     padding-right: 2.25rem; /* 36px */
        /// }
        /// ```
        Px9 => "px-9",
        /// ```css
        /// {
        ///     padding-top: 2.25rem; /* 36px */
        ///     padding-bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        Py9 => "py-9",
        /// ```css
        /// {
        ///     padding-inline-start: 2.25rem; /* 36px */
        /// }
        /// ```
        Ps9 => "ps-9",
        /// ```css
        /// {
        ///     padding-inline-end: 2.25rem; /* 36px */
        /// }
        /// ```
        Pe9 => "pe-9",
        /// ```css
        /// {
        ///     padding-top: 2.25rem; /* 36px */
        /// }
        /// ```
        Pt9 => "pt-9",
        /// ```css
        /// {
        ///     padding-right: 2.25rem; /* 36px */
        /// }
        /// ```
        Pr9 => "pr-9",
        /// ```css
        /// {
        ///     padding-bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        Pb9 => "pb-9",
        /// ```css
        /// {
        ///     padding-left: 2.25rem; /* 36px */
        /// }
        /// ```
        Pl9 => "pl-9",
        /// ```css
        /// {
        ///     padding: 2.5rem; /* 40px */
        /// }
        /// ```
        P10 => "p-10",
        /// ```css
        /// {
        ///     padding-left: 2.5rem; /* 40px */
        ///     padding-right: 2.5rem; /* 40px */
        /// }
        /// ```
        Px10 => "px-10",
        /// ```css
        /// {
        ///     padding-top: 2.5rem; /* 40px */
        ///     padding-bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        Py10 => "py-10",
        /// ```css
        /// {
        ///     padding-inline-start: 2.5rem; /* 40px */
        /// }
        /// ```
        Ps10 => "ps-10",
        /// ```css
        /// {
        ///     padding-inline-end: 2.5rem; /* 40px */
        /// }
        /// ```
        Pe10 => "pe-10",
        /// ```css
        /// {
        ///     padding-top: 2.5rem; /* 40px */
        /// }
        /// ```
        Pt10 => "pt-10",
        /// ```css
        /// {
        ///     padding-right: 2.5rem; /* 40px */
        /// }
        /// ```
        Pr10 => "pr-10",
        /// ```css
        /// {
        ///     padding-bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        Pb10 => "pb-10",
        /// ```css
        /// {
        ///     padding-left: 2.5rem; /* 40px */
        /// }
        /// ```
        Pl10 => "pl-10",
        /// ```css
        /// {
        ///     padding: 2.75rem; /* 44px */
        /// }
        /// ```
        P11 => "p-11",
        /// ```css
        /// {
        ///     padding-left: 2.75rem; /* 44px */
        ///     padding-right: 2.75rem; /* 44px */
        /// }
        /// ```
        Px11 => "px-11",
        /// ```css
        /// {
        ///     padding-top: 2.75rem; /* 44px */
        ///     padding-bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        Py11 => "py-11",
        /// ```css
        /// {
        ///     padding-inline-start: 2.75rem; /* 44px */
        /// }
        /// ```
        Ps11 => "ps-11",
        /// ```css
        /// {
        ///     padding-inline-end: 2.75rem; /* 44px */
        /// }
        /// ```
        Pe11 => "pe-11",
        /// ```css
        /// {
        ///     padding-top: 2.75rem; /* 44px */
        /// }
        /// ```
        Pt11 => "pt-11",
        /// ```css
        /// {
        ///     padding-right: 2.75rem; /* 44px */
        /// }
        /// ```
        Pr11 => "pr-11",
        /// ```css
        /// {
        ///     padding-bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        Pb11 => "pb-11",
        /// ```css
        /// {
        ///     padding-left: 2.75rem; /* 44px */
        /// }
        /// ```
        Pl11 => "pl-11",
        /// ```css
        /// {
        ///     padding: 3rem; /* 48px */
        /// }
        /// ```
        P12 => "p-12",
        /// ```css
        /// {
        ///     padding-left: 3rem; /* 48px */
        ///     padding-right: 3rem; /* 48px */
        /// }
        /// ```
        Px12 => "px-12",
        /// ```css
        /// {
        ///     padding-top: 3rem; /* 48px */
        ///     padding-bottom: 3rem; /* 48px */
        /// }
        /// ```
        Py12 => "py-12",
        /// ```css
        /// {
        ///     padding-inline-start: 3rem; /* 48px */
        /// }
        /// ```
        Ps12 => "ps-12",
        /// ```css
        /// {
        ///     padding-inline-end: 3rem; /* 48px */
        /// }
        /// ```
        Pe12 => "pe-12",
        /// ```css
        /// {
        ///     padding-top: 3rem; /* 48px */
        /// }
        /// ```
        Pt12 => "pt-12",
        /// ```css
        /// {
        ///     padding-right: 3rem; /* 48px */
        /// }
        /// ```
        Pr12 => "pr-12",
        /// ```css
        /// {
        ///     padding-bottom: 3rem; /* 48px */
        /// }
        /// ```
        Pb12 => "pb-12",
        /// ```css
        /// {
        ///     padding-left: 3rem; /* 48px */
        /// }
        /// ```
        Pl12 => "pl-12",
        /// ```css
        /// {
        ///     padding: 3.5rem; /* 56px */
        /// }
        /// ```
        P14 => "p-14",
        /// ```css
        /// {
        ///     padding-left: 3.5rem; /* 56px */
        ///     padding-right: 3.5rem; /* 56px */
        /// }
        /// ```
        Px14 => "px-14",
        /// ```css
        /// {
        ///     padding-top: 3.5rem; /* 56px */
        ///     padding-bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        Py14 => "py-14",
        /// ```css
        /// {
        ///     padding-inline-start: 3.5rem; /* 56px */
        /// }
        /// ```
        Ps14 => "ps-14",
        /// ```css
        /// {
        ///     padding-inline-end: 3.5rem; /* 56px */
        /// }
        /// ```
        Pe14 => "pe-14",
        /// ```css
        /// {
        ///     padding-top: 3.5rem; /* 56px */
        /// }
        /// ```
        Pt14 => "pt-14",
        /// ```css
        /// {
        ///     padding-right: 3.5rem; /* 56px */
        /// }
        /// ```
        Pr14 => "pr-14",
        /// ```css
        /// {
        ///     padding-bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        Pb14 => "pb-14",
        /// ```css
        /// {
        ///     padding-left: 3.5rem; /* 56px */
        /// }
        /// ```
        Pl14 => "pl-14",
        /// ```css
        /// {
        ///     padding: 4rem; /* 64px */
        /// }
        /// ```
        P16 => "p-16",
        /// ```css
        /// {
        ///     padding-left: 4rem; /* 64px */
        ///     padding-right: 4rem; /* 64px */
        /// }
        /// ```
        Px16 => "px-16",
        /// ```css
        /// {
        ///     padding-top: 4rem; /* 64px */
        ///     padding-bottom: 4rem; /* 64px */
        /// }
        /// ```
        Py16 => "py-16",
        /// ```css
        /// {
        ///     padding-inline-start: 4rem; /* 64px */
        /// }
        /// ```
        Ps16 => "ps-16",
        /// ```css
        /// {
        ///     padding-inline-end: 4rem; /* 64px */
        /// }
        /// ```
        Pe16 => "pe-16",
        /// ```css
        /// {
        ///     padding-top: 4rem; /* 64px */
        /// }
        /// ```
        Pt16 => "pt-16",
        /// ```css
        /// {
        ///     padding-right: 4rem; /* 64px */
        /// }
        /// ```
        Pr16 => "pr-16",
        /// ```css
        /// {
        ///     padding-bottom: 4rem; /* 64px */
        /// }
        /// ```
        Pb16 => "pb-16",
        /// ```css
        /// {
        ///     padding-left: 4rem; /* 64px */
        /// }
        /// ```
        Pl16 => "pl-16",
        /// ```css
        /// {
        ///     padding: 5rem; /* 80px */
        /// }
        /// ```
        P20 => "p-20",
        /// ```css
        /// {
        ///     padding-left: 5rem; /* 80px */
        ///     padding-right: 5rem; /* 80px */
        /// }
        /// ```
        Px20 => "px-20",
        /// ```css
        /// {
        ///     padding-top: 5rem; /* 80px */
        ///     padding-bottom: 5rem; /* 80px */
        /// }
        /// ```
        Py20 => "py-20",
        /// ```css
        /// {
        ///     padding-inline-start: 5rem; /* 80px */
        /// }
        /// ```
        Ps20 => "ps-20",
        /// ```css
        /// {
        ///     padding-inline-end: 5rem; /* 80px */
        /// }
        /// ```
        Pe20 => "pe-20",
        /// ```css
        /// {
        ///     padding-top: 5rem; /* 80px */
        /// }
        /// ```
        Pt20 => "pt-20",
        /// ```css
        /// {
        ///     padding-right: 5rem; /* 80px */
        /// }
        /// ```
        Pr20 => "pr-20",
        /// ```css
        /// {
        ///     padding-bottom: 5rem; /* 80px */
        /// }
        /// ```
        Pb20 => "pb-20",
        /// ```css
        /// {
        ///     padding-left: 5rem; /* 80px */
        /// }
        /// ```
        Pl20 => "pl-20",
        /// ```css
        /// {
        ///     padding: 6rem; /* 96px */
        /// }
        /// ```
        P24 => "p-24",
        /// ```css
        /// {
        ///     padding-left: 6rem; /* 96px */
        ///     padding-right: 6rem; /* 96px */
        /// }
        /// ```
        Px24 => "px-24",
        /// ```css
        /// {
        ///     padding-top: 6rem; /* 96px */
        ///     padding-bottom: 6rem; /* 96px */
        /// }
        /// ```
        Py24 => "py-24",
        /// ```css
        /// {
        ///     padding-inline-start: 6rem; /* 96px */
        /// }
        /// ```
        Ps24 => "ps-24",
        /// ```css
        /// {
        ///     padding-inline-end: 6rem; /* 96px */
        /// }
        /// ```
        Pe24 => "pe-24",
        /// ```css
        /// {
        ///     padding-top: 6rem; /* 96px */
        /// }
        /// ```
        Pt24 => "pt-24",
        /// ```css
        /// {
        ///     padding-right: 6rem; /* 96px */
        /// }
        /// ```
        Pr24 => "pr-24",
        /// ```css
        /// {
        ///     padding-bottom: 6rem; /* 96px */
        /// }
        /// ```
        Pb24 => "pb-24",
        /// ```css
        /// {
        ///     padding-left: 6rem; /* 96px */
        /// }
        /// ```
        Pl24 => "pl-24",
        /// ```css
        /// {
        ///     padding: 7rem; /* 112px */
        /// }
        /// ```
        P28 => "p-28",
        /// ```css
        /// {
        ///     padding-left: 7rem; /* 112px */
        ///     padding-right: 7rem; /* 112px */
        /// }
        /// ```
        Px28 => "px-28",
        /// ```css
        /// {
        ///     padding-top: 7rem; /* 112px */
        ///     padding-bottom: 7rem; /* 112px */
        /// }
        /// ```
        Py28 => "py-28",
        /// ```css
        /// {
        ///     padding-inline-start: 7rem; /* 112px */
        /// }
        /// ```
        Ps28 => "ps-28",
        /// ```css
        /// {
        ///     padding-inline-end: 7rem; /* 112px */
        /// }
        /// ```
        Pe28 => "pe-28",
        /// ```css
        /// {
        ///     padding-top: 7rem; /* 112px */
        /// }
        /// ```
        Pt28 => "pt-28",
        /// ```css
        /// {
        ///     padding-right: 7rem; /* 112px */
        /// }
        /// ```
        Pr28 => "pr-28",
        /// ```css
        /// {
        ///     padding-bottom: 7rem; /* 112px */
        /// }
        /// ```
        Pb28 => "pb-28",
        /// ```css
        /// {
        ///     padding-left: 7rem; /* 112px */
        /// }
        /// ```
        Pl28 => "pl-28",
        /// ```css
        /// {
        ///     padding: 8rem; /* 128px */
        /// }
        /// ```
        P32 => "p-32",
        /// ```css
        /// {
        ///     padding-left: 8rem; /* 128px */
        ///     padding-right: 8rem; /* 128px */
        /// }
        /// ```
        Px32 => "px-32",
        /// ```css
        /// {
        ///     padding-top: 8rem; /* 128px */
        ///     padding-bottom: 8rem; /* 128px */
        /// }
        /// ```
        Py32 => "py-32",
        /// ```css
        /// {
        ///     padding-inline-start: 8rem; /* 128px */
        /// }
        /// ```
        Ps32 => "ps-32",
        /// ```css
        /// {
        ///     padding-inline-end: 8rem; /* 128px */
        /// }
        /// ```
        Pe32 => "pe-32",
        /// ```css
        /// {
        ///     padding-top: 8rem; /* 128px */
        /// }
        /// ```
        Pt32 => "pt-32",
        /// ```css
        /// {
        ///     padding-right: 8rem; /* 128px */
        /// }
        /// ```
        Pr32 => "pr-32",
        /// ```css
        /// {
        ///     padding-bottom: 8rem; /* 128px */
        /// }
        /// ```
        Pb32 => "pb-32",
        /// ```css
        /// {
        ///     padding-left: 8rem; /* 128px */
        /// }
        /// ```
        Pl32 => "pl-32",
        /// ```css
        /// {
        ///     padding: 9rem; /* 144px */
        /// }
        /// ```
        P36 => "p-36",
        /// ```css
        /// {
        ///     padding-left: 9rem; /* 144px */
        ///     padding-right: 9rem; /* 144px */
        /// }
        /// ```
        Px36 => "px-36",
        /// ```css
        /// {
        ///     padding-top: 9rem; /* 144px */
        ///     padding-bottom: 9rem; /* 144px */
        /// }
        /// ```
        Py36 => "py-36",
        /// ```css
        /// {
        ///     padding-inline-start: 9rem; /* 144px */
        /// }
        /// ```
        Ps36 => "ps-36",
        /// ```css
        /// {
        ///     padding-inline-end: 9rem; /* 144px */
        /// }
        /// ```
        Pe36 => "pe-36",
        /// ```css
        /// {
        ///     padding-top: 9rem; /* 144px */
        /// }
        /// ```
        Pt36 => "pt-36",
        /// ```css
        /// {
        ///     padding-right: 9rem; /* 144px */
        /// }
        /// ```
        Pr36 => "pr-36",
        /// ```css
        /// {
        ///     padding-bottom: 9rem; /* 144px */
        /// }
        /// ```
        Pb36 => "pb-36",
        /// ```css
        /// {
        ///     padding-left: 9rem; /* 144px */
        /// }
        /// ```
        Pl36 => "pl-36",
        /// ```css
        /// {
        ///     padding: 10rem; /* 160px */
        /// }
        /// ```
        P40 => "p-40",
        /// ```css
        /// {
        ///     padding-left: 10rem; /* 160px */
        ///     padding-right: 10rem; /* 160px */
        /// }
        /// ```
        Px40 => "px-40",
        /// ```css
        /// {
        ///     padding-top: 10rem; /* 160px */
        ///     padding-bottom: 10rem; /* 160px */
        /// }
        /// ```
        Py40 => "py-40",
        /// ```css
        /// {
        ///     padding-inline-start: 10rem; /* 160px */
        /// }
        /// ```
        Ps40 => "ps-40",
        /// ```css
        /// {
        ///     padding-inline-end: 10rem; /* 160px */
        /// }
        /// ```
        Pe40 => "pe-40",
        /// ```css
        /// {
        ///     padding-top: 10rem; /* 160px */
        /// }
        /// ```
        Pt40 => "pt-40",
        /// ```css
        /// {
        ///     padding-right: 10rem; /* 160px */
        /// }
        /// ```
        Pr40 => "pr-40",
        /// ```css
        /// {
        ///     padding-bottom: 10rem; /* 160px */
        /// }
        /// ```
        Pb40 => "pb-40",
        /// ```css
        /// {
        ///     padding-left: 10rem; /* 160px */
        /// }
        /// ```
        Pl40 => "pl-40",
        /// ```css
        /// {
        ///     padding: 11rem; /* 176px */
        /// }
        /// ```
        P44 => "p-44",
        /// ```css
        /// {
        ///     padding-left: 11rem; /* 176px */
        ///     padding-right: 11rem; /* 176px */
        /// }
        /// ```
        Px44 => "px-44",
        /// ```css
        /// {
        ///     padding-top: 11rem; /* 176px */
        ///     padding-bottom: 11rem; /* 176px */
        /// }
        /// ```
        Py44 => "py-44",
        /// ```css
        /// {
        ///     padding-inline-start: 11rem; /* 176px */
        /// }
        /// ```
        Ps44 => "ps-44",
        /// ```css
        /// {
        ///     padding-inline-end: 11rem; /* 176px */
        /// }
        /// ```
        Pe44 => "pe-44",
        /// ```css
        /// {
        ///     padding-top: 11rem; /* 176px */
        /// }
        /// ```
        Pt44 => "pt-44",
        /// ```css
        /// {
        ///     padding-right: 11rem; /* 176px */
        /// }
        /// ```
        Pr44 => "pr-44",
        /// ```css
        /// {
        ///     padding-bottom: 11rem; /* 176px */
        /// }
        /// ```
        Pb44 => "pb-44",
        /// ```css
        /// {
        ///     padding-left: 11rem; /* 176px */
        /// }
        /// ```
        Pl44 => "pl-44",
        /// ```css
        /// {
        ///     padding: 12rem; /* 192px */
        /// }
        /// ```
        P48 => "p-48",
        /// ```css
        /// {
        ///     padding-left: 12rem; /* 192px */
        ///     padding-right: 12rem; /* 192px */
        /// }
        /// ```
        Px48 => "px-48",
        /// ```css
        /// {
        ///     padding-top: 12rem; /* 192px */
        ///     padding-bottom: 12rem; /* 192px */
        /// }
        /// ```
        Py48 => "py-48",
        /// ```css
        /// {
        ///     padding-inline-start: 12rem; /* 192px */
        /// }
        /// ```
        Ps48 => "ps-48",
        /// ```css
        /// {
        ///     padding-inline-end: 12rem; /* 192px */
        /// }
        /// ```
        Pe48 => "pe-48",
        /// ```css
        /// {
        ///     padding-top: 12rem; /* 192px */
        /// }
        /// ```
        Pt48 => "pt-48",
        /// ```css
        /// {
        ///     padding-right: 12rem; /* 192px */
        /// }
        /// ```
        Pr48 => "pr-48",
        /// ```css
        /// {
        ///     padding-bottom: 12rem; /* 192px */
        /// }
        /// ```
        Pb48 => "pb-48",
        /// ```css
        /// {
        ///     padding-left: 12rem; /* 192px */
        /// }
        /// ```
        Pl48 => "pl-48",
        /// ```css
        /// {
        ///     padding: 13rem; /* 208px */
        /// }
        /// ```
        P52 => "p-52",
        /// ```css
        /// {
        ///     padding-left: 13rem; /* 208px */
        ///     padding-right: 13rem; /* 208px */
        /// }
        /// ```
        Px52 => "px-52",
        /// ```css
        /// {
        ///     padding-top: 13rem; /* 208px */
        ///     padding-bottom: 13rem; /* 208px */
        /// }
        /// ```
        Py52 => "py-52",
        /// ```css
        /// {
        ///     padding-inline-start: 13rem; /* 208px */
        /// }
        /// ```
        Ps52 => "ps-52",
        /// ```css
        /// {
        ///     padding-inline-end: 13rem; /* 208px */
        /// }
        /// ```
        Pe52 => "pe-52",
        /// ```css
        /// {
        ///     padding-top: 13rem; /* 208px */
        /// }
        /// ```
        Pt52 => "pt-52",
        /// ```css
        /// {
        ///     padding-right: 13rem; /* 208px */
        /// }
        /// ```
        Pr52 => "pr-52",
        /// ```css
        /// {
        ///     padding-bottom: 13rem; /* 208px */
        /// }
        /// ```
        Pb52 => "pb-52",
        /// ```css
        /// {
        ///     padding-left: 13rem; /* 208px */
        /// }
        /// ```
        Pl52 => "pl-52",
        /// ```css
        /// {
        ///     padding: 14rem; /* 224px */
        /// }
        /// ```
        P56 => "p-56",
        /// ```css
        /// {
        ///     padding-left: 14rem; /* 224px */
        ///     padding-right: 14rem; /* 224px */
        /// }
        /// ```
        Px56 => "px-56",
        /// ```css
        /// {
        ///     padding-top: 14rem; /* 224px */
        ///     padding-bottom: 14rem; /* 224px */
        /// }
        /// ```
        Py56 => "py-56",
        /// ```css
        /// {
        ///     padding-inline-start: 14rem; /* 224px */
        /// }
        /// ```
        Ps56 => "ps-56",
        /// ```css
        /// {
        ///     padding-inline-end: 14rem; /* 224px */
        /// }
        /// ```
        Pe56 => "pe-56",
        /// ```css
        /// {
        ///     padding-top: 14rem; /* 224px */
        /// }
        /// ```
        Pt56 => "pt-56",
        /// ```css
        /// {
        ///     padding-right: 14rem; /* 224px */
        /// }
        /// ```
        Pr56 => "pr-56",
        /// ```css
        /// {
        ///     padding-bottom: 14rem; /* 224px */
        /// }
        /// ```
        Pb56 => "pb-56",
        /// ```css
        /// {
        ///     padding-left: 14rem; /* 224px */
        /// }
        /// ```
        Pl56 => "pl-56",
        /// ```css
        /// {
        ///     padding: 15rem; /* 240px */
        /// }
        /// ```
        P60 => "p-60",
        /// ```css
        /// {
        ///     padding-left: 15rem; /* 240px */
        ///     padding-right: 15rem; /* 240px */
        /// }
        /// ```
        Px60 => "px-60",
        /// ```css
        /// {
        ///     padding-top: 15rem; /* 240px */
        ///     padding-bottom: 15rem; /* 240px */
        /// }
        /// ```
        Py60 => "py-60",
        /// ```css
        /// {
        ///     padding-inline-start: 15rem; /* 240px */
        /// }
        /// ```
        Ps60 => "ps-60",
        /// ```css
        /// {
        ///     padding-inline-end: 15rem; /* 240px */
        /// }
        /// ```
        Pe60 => "pe-60",
        /// ```css
        /// {
        ///     padding-top: 15rem; /* 240px */
        /// }
        /// ```
        Pt60 => "pt-60",
        /// ```css
        /// {
        ///     padding-right: 15rem; /* 240px */
        /// }
        /// ```
        Pr60 => "pr-60",
        /// ```css
        /// {
        ///     padding-bottom: 15rem; /* 240px */
        /// }
        /// ```
        Pb60 => "pb-60",
        /// ```css
        /// {
        ///     padding-left: 15rem; /* 240px */
        /// }
        /// ```
        Pl60 => "pl-60",
        /// ```css
        /// {
        ///     padding: 16rem; /* 256px */
        /// }
        /// ```
        P64 => "p-64",
        /// ```css
        /// {
        ///     padding-left: 16rem; /* 256px */
        ///     padding-right: 16rem; /* 256px */
        /// }
        /// ```
        Px64 => "px-64",
        /// ```css
        /// {
        ///     padding-top: 16rem; /* 256px */
        ///     padding-bottom: 16rem; /* 256px */
        /// }
        /// ```
        Py64 => "py-64",
        /// ```css
        /// {
        ///     padding-inline-start: 16rem; /* 256px */
        /// }
        /// ```
        Ps64 => "ps-64",
        /// ```css
        /// {
        ///     padding-inline-end: 16rem; /* 256px */
        /// }
        /// ```
        Pe64 => "pe-64",
        /// ```css
        /// {
        ///     padding-top: 16rem; /* 256px */
        /// }
        /// ```
        Pt64 => "pt-64",
        /// ```css
        /// {
        ///     padding-right: 16rem; /* 256px */
        /// }
        /// ```
        Pr64 => "pr-64",
        /// ```css
        /// {
        ///     padding-bottom: 16rem; /* 256px */
        /// }
        /// ```
        Pb64 => "pb-64",
        /// ```css
        /// {
        ///     padding-left: 16rem; /* 256px */
        /// }
        /// ```
        Pl64 => "pl-64",
        /// ```css
        /// {
        ///     padding: 18rem; /* 288px */
        /// }
        /// ```
        P72 => "p-72",
        /// ```css
        /// {
        ///     padding-left: 18rem; /* 288px */
        ///     padding-right: 18rem; /* 288px */
        /// }
        /// ```
        Px72 => "px-72",
        /// ```css
        /// {
        ///     padding-top: 18rem; /* 288px */
        ///     padding-bottom: 18rem; /* 288px */
        /// }
        /// ```
        Py72 => "py-72",
        /// ```css
        /// {
        ///     padding-inline-start: 18rem; /* 288px */
        /// }
        /// ```
        Ps72 => "ps-72",
        /// ```css
        /// {
        ///     padding-inline-end: 18rem; /* 288px */
        /// }
        /// ```
        Pe72 => "pe-72",
        /// ```css
        /// {
        ///     padding-top: 18rem; /* 288px */
        /// }
        /// ```
        Pt72 => "pt-72",
        /// ```css
        /// {
        ///     padding-right: 18rem; /* 288px */
        /// }
        /// ```
        Pr72 => "pr-72",
        /// ```css
        /// {
        ///     padding-bottom: 18rem; /* 288px */
        /// }
        /// ```
        Pb72 => "pb-72",
        /// ```css
        /// {
        ///     padding-left: 18rem; /* 288px */
        /// }
        /// ```
        Pl72 => "pl-72",
        /// ```css
        /// {
        ///     padding: 20rem; /* 320px */
        /// }
        /// ```
        P80 => "p-80",
        /// ```css
        /// {
        ///     padding-left: 20rem; /* 320px */
        ///     padding-right: 20rem; /* 320px */
        /// }
        /// ```
        Px80 => "px-80",
        /// ```css
        /// {
        ///     padding-top: 20rem; /* 320px */
        ///     padding-bottom: 20rem; /* 320px */
        /// }
        /// ```
        Py80 => "py-80",
        /// ```css
        /// {
        ///     padding-inline-start: 20rem; /* 320px */
        /// }
        /// ```
        Ps80 => "ps-80",
        /// ```css
        /// {
        ///     padding-inline-end: 20rem; /* 320px */
        /// }
        /// ```
        Pe80 => "pe-80",
        /// ```css
        /// {
        ///     padding-top: 20rem; /* 320px */
        /// }
        /// ```
        Pt80 => "pt-80",
        /// ```css
        /// {
        ///     padding-right: 20rem; /* 320px */
        /// }
        /// ```
        Pr80 => "pr-80",
        /// ```css
        /// {
        ///     padding-bottom: 20rem; /* 320px */
        /// }
        /// ```
        Pb80 => "pb-80",
        /// ```css
        /// {
        ///     padding-left: 20rem; /* 320px */
        /// }
        /// ```
        Pl80 => "pl-80",
        /// ```css
        /// {
        ///     padding: 24rem; /* 384px */
        /// }
        /// ```
        P96 => "p-96",
        /// ```css
        /// {
        ///     padding-left: 24rem; /* 384px */
        ///     padding-right: 24rem; /* 384px */
        /// }
        /// ```
        Px96 => "px-96",
        /// ```css
        /// {
        ///     padding-top: 24rem; /* 384px */
        ///     padding-bottom: 24rem; /* 384px */
        /// }
        /// ```
        Py96 => "py-96",
        /// ```css
        /// {
        ///     padding-inline-start: 24rem; /* 384px */
        /// }
        /// ```
        Ps96 => "ps-96",
        /// ```css
        /// {
        ///     padding-inline-end: 24rem; /* 384px */
        /// }
        /// ```
        Pe96 => "pe-96",
        /// ```css
        /// {
        ///     padding-top: 24rem; /* 384px */
        /// }
        /// ```
        Pt96 => "pt-96",
        /// ```css
        /// {
        ///     padding-right: 24rem; /* 384px */
        /// }
        /// ```
        Pr96 => "pr-96",
        /// ```css
        /// {
        ///     padding-bottom: 24rem; /* 384px */
        /// }
        /// ```
        Pb96 => "pb-96",
        /// ```css
        /// {
        ///     padding-left: 24rem; /* 384px */
        /// }
        /// ```
        Pl96 => "pl-96",
    }
    /// Utilities for controlling an element's margin.
    ///
    /// <https://tailwindcss.com/docs/margin>
    Margin {
        /// ```css
        /// {
        ///     margin: 0px;
        /// }
        /// ```
        M0 => "m-0",
        /// ```css
        /// {
        ///     margin-left: 0px;
        ///     margin-right: 0px;
        /// }
        /// ```
        Mx0 => "mx-0",
        /// ```css
        /// {
        ///     margin-top: 0px;
        ///     margin-bottom: 0px;
        /// }
        /// ```
        My0 => "my-0",
        /// ```css
        /// {
        ///     margin-inline-start: 0px;
        /// }
        /// ```
        Ms0 => "ms-0",
        /// ```css
        /// {
        ///     margin-inline-end: 0px;
        /// }
        /// ```
        Me0 => "me-0",
        /// ```css
        /// {
        ///     margin-top: 0px;
        /// }
        /// ```
        Mt0 => "mt-0",
        /// ```css
        /// {
        ///     margin-right: 0px;
        /// }
        /// ```
        Mr0 => "mr-0",
        /// ```css
        /// {
        ///     margin-bottom: 0px;
        /// }
        /// ```
        Mb0 => "mb-0",
        /// ```css
        /// {
        ///     margin-left: 0px;
        /// }
        /// ```
        Ml0 => "ml-0",
        /// ```css
        /// {
        ///     margin: 1px;
        /// }
        /// ```
        MPx => "m-px",
        /// ```css
        /// {
        ///     margin-left: 1px;
        ///     margin-right: 1px;
        /// }
        /// ```
        MxPx => "mx-px",
        /// ```css
        /// {
        ///     margin-top: 1px;
        ///     margin-bottom: 1px;
        /// }
        /// ```
        MyPx => "my-px",
        /// ```css
        /// {
        ///     margin-inline-start: 1px;
        /// }
        /// ```
        MsPx => "ms-px",
        /// ```css
        /// {
        ///     margin-inline-end: 1px;
        /// }
        /// ```
        MePx => "me-px",
        /// ```css
        /// {
        ///     margin-top: 1px;
        /// }
        /// ```
        MtPx => "mt-px",
        /// ```css
        /// {
        ///     margin-right: 1px;
        /// }
        /// ```
        MrPx => "mr-px",
        /// ```css
        /// {
        ///     margin-bottom: 1px;
        /// }
        /// ```
        MbPx => "mb-px",
        /// ```css
        /// {
        ///     margin-left: 1px;
        /// }
        /// ```
        MlPx => "ml-px",
        /// ```css
        /// {
        ///     margin: 0.125rem; /* 2px */
        /// }
        /// ```
        M0_5 => "m-0.5",
        /// ```css
        /// {
        ///     margin-left: 0.125rem; /* 2px */
        ///     margin-right: 0.125rem; /* 2px */
        /// }
        /// ```
        Mx0_5 => "mx-0.5",
        /// ```css
        /// {
        ///     margin-top: 0.125rem; /* 2px */
        ///     margin-bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        My0_5 => "my-0.5",
        /// ```css
        /// {
        ///     margin-inline-start: 0.125rem; /* 2px */
        /// }
        /// ```
        Ms0_5 => "ms-0.5",
        /// ```css
        /// {
        ///     margin-inline-end: 0.125rem; /* 2px */
        /// }
        /// ```
        Me0_5 => "me-0.5",
        /// ```css
        /// {
        ///     margin-top: 0.125rem; /* 2px */
        /// }
        /// ```
        Mt0_5 => "mt-0.5",
        /// ```css
        /// {
        ///     margin-right: 0.125rem; /* 2px */
        /// }
        /// ```
        Mr0_5 => "mr-0.5",
        /// ```css
        /// {
        ///     margin-bottom: 0.125rem; /* 2px */
        /// }
        /// ```
        Mb0_5 => "mb-0.5",
        /// ```css
        /// {
        ///     margin-left: 0.125rem; /* 2px */
        /// }
        /// ```
        Ml0_5 => "ml-0.5",
        /// ```css
        /// {
        ///     margin: 0.25rem; /* 4px */
        /// }
        /// ```
        M1 => "m-1",
        /// ```css
        /// {
        ///     margin-left: 0.25rem; /* 4px */
        ///     margin-right: 0.25rem; /* 4px */
        /// }
        /// ```
        Mx1 => "mx-1",
        /// ```css
        /// {
        ///     margin-top: 0.25rem; /* 4px */
        ///     margin-bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        My1 => "my-1",
        /// ```css
        /// {
        ///     margin-inline-start: 0.25rem; /* 4px */
        /// }
        /// ```
        Ms1 => "ms-1",
        /// ```css
        /// {
        ///     margin-inline-end: 0.25rem; /* 4px */
        /// }
        /// ```
        Me1 => "me-1",
        /// ```css
        /// {
        ///     margin-top: 0.25rem; /* 4px */
        /// }
        /// ```
        Mt1 => "mt-1",
        /// ```css
        /// {
        ///     margin-right: 0.25rem; /* 4px */
        /// }
        /// ```
        Mr1 => "mr-1",
        /// ```css
        /// {
        ///     margin-bottom: 0.25rem; /* 4px */
        /// }
        /// ```
        Mb1 => "mb-1",
        /// ```css
        /// {
        ///     margin-left: 0.25rem; /* 4px */
        /// }
        /// ```
        Ml1 => "ml-1",
        /// ```css
        /// {
        ///     margin: 0.375rem; /* 6px */
        /// }
        /// ```
        M1_5 => "m-1.5",
        /// ```css
        /// {
        ///     margin-left: 0.375rem; /* 6px */
        ///     margin-right: 0.375rem; /* 6px */
        /// }
        /// ```
        Mx1_5 => "mx-1.5",
        /// ```css
        /// {
        ///     margin-top: 0.375rem; /* 6px */
        ///     margin-bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        My1_5 => "my-1.5",
        /// ```css
        /// {
        ///     margin-inline-start: 0.375rem; /* 6px */
        /// }
        /// ```
        Ms1_5 => "ms-1.5",
        /// ```css
        /// {
        ///     margin-inline-end: 0.375rem; /* 6px */
        /// }
        /// ```
        Me1_5 => "me-1.5",
        /// ```css
        /// {
        ///     margin-top: 0.375rem; /* 6px */
        /// }
        /// ```
        Mt1_5 => "mt-1.5",
        /// ```css
        /// {
        ///     margin-right: 0.375rem; /* 6px */
        /// }
        /// ```
        Mr1_5 => "mr-1.5",
        /// ```css
        /// {
        ///     margin-bottom: 0.375rem; /* 6px */
        /// }
        /// ```
        Mb1_5 => "mb-1.5",
        /// ```css
        /// {
        ///     margin-left: 0.375rem; /* 6px */
        /// }
        /// ```
        Ml1_5 => "ml-1.5",
        /// ```css
        /// {
        ///     margin: 0.5rem; /* 8px */
        /// }
        /// ```
        M2 => "m-2",
        /// ```css
        /// {
        ///     margin-left: 0.5rem; /* 8px */
        ///     margin-right: 0.5rem; /* 8px */
        /// }
        /// ```
        Mx2 => "mx-2",
        /// ```css
        /// {
        ///     margin-top: 0.5rem; /* 8px */
        ///     margin-bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        My2 => "my-2",
        /// ```css
        /// {
        ///     margin-inline-start: 0.5rem; /* 8px */
        /// }
        /// ```
        Ms2 => "ms-2",
        /// ```css
        /// {
        ///     margin-inline-end: 0.5rem; /* 8px */
        /// }
        /// ```
        Me2 => "me-2",
        /// ```css
        /// {
        ///     margin-top: 0.5rem; /* 8px */
        /// }
        /// ```
        Mt2 => "mt-2",
        /// ```css
        /// {
        ///     margin-right: 0.5rem; /* 8px */
        /// }
        /// ```
        Mr2 => "mr-2",
        /// ```css
        /// {
        ///     margin-bottom: 0.5rem; /* 8px */
        /// }
        /// ```
        Mb2 => "mb-2",
        /// ```css
        /// {
        ///     margin-left: 0.5rem; /* 8px */
        /// }
        /// ```
        Ml2 => "ml-2",
        /// ```css
        /// {
        ///     margin: 0.625rem; /* 10px */
        /// }
        /// ```
        M2_5 => "m-2.5",
        /// ```css
        /// {
        ///     margin-left: 0.625rem; /* 10px */
        ///     margin-right: 0.625rem; /* 10px */
        /// }
        /// ```
        Mx2_5 => "mx-2.5",
        /// ```css
        /// {
        ///     margin-top: 0.625rem; /* 10px */
        ///     margin-bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        My2_5 => "my-2.5",
        /// ```css
        /// {
        ///     margin-inline-start: 0.625rem; /* 10px */
        /// }
        /// ```
        Ms2_5 => "ms-2.5",
        /// ```css
        /// {
        ///     margin-inline-end: 0.625rem; /* 10px */
        /// }
        /// ```
        Me2_5 => "me-2.5",
        /// ```css
        /// {
        ///     margin-top: 0.625rem; /* 10px */
        /// }
        /// ```
        Mt2_5 => "mt-2.5",
        /// ```css
        /// {
        ///     margin-right: 0.625rem; /* 10px */
        /// }
        /// ```
        Mr2_5 => "mr-2.5",
        /// ```css
        /// {
        ///     margin-bottom: 0.625rem; /* 10px */
        /// }
        /// ```
        Mb2_5 => "mb-2.5",
        /// ```css
        /// {
        ///     margin-left: 0.625rem; /* 10px */
        /// }
        /// ```
        Ml2_5 => "ml-2.5",
        /// ```css
        /// {
        ///     margin: 0.75rem; /* 12px */
        /// }
        /// ```
        M3 => "m-3",
        /// ```css
        /// {
        ///     margin-left: 0.75rem; /* 12px */
        ///     margin-right: 0.75rem; /* 12px */
        /// }
        /// ```
        Mx3 => "mx-3",
        /// ```css
        /// {
        ///     margin-top: 0.75rem; /* 12px */
        ///     margin-bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        My3 => "my-3",
        /// ```css
        /// {
        ///     margin-inline-start: 0.75rem; /* 12px */
        /// }
        /// ```
        Ms3 => "ms-3",
        /// ```css
        /// {
        ///     margin-inline-end: 0.75rem; /* 12px */
        /// }
        /// ```
        Me3 => "me-3",
        /// ```css
        /// {
        ///     margin-top: 0.75rem; /* 12px */
        /// }
        /// ```
        Mt3 => "mt-3",
        /// ```css
        /// {
        ///     margin-right: 0.75rem; /* 12px */
        /// }
        /// ```
        Mr3 => "mr-3",
        /// ```css
        /// {
        ///     margin-bottom: 0.75rem; /* 12px */
        /// }
        /// ```
        Mb3 => "mb-3",
        /// ```css
        /// {
        ///     margin-left: 0.75rem; /* 12px */
        /// }
        /// ```
        Ml3 => "ml-3",
        /// ```css
        /// {
        ///     margin: 0.875rem; /* 14px */
        /// }
        /// ```
        M3_5 => "m-3.5",
        /// ```css
        /// {
        ///     margin-left: 0.875rem; /* 14px */
        ///     margin-right: 0.875rem; /* 14px */
        /// }
        /// ```
        Mx3_5 => "mx-3.5",
        /// ```css
        /// {
        ///     margin-top: 0.875rem; /* 14px */
        ///     margin-bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        My3_5 => "my-3.5",
        /// ```css
        /// {
        ///     margin-inline-start: 0.875rem; /* 14px */
        /// }
        /// ```
        Ms3_5 => "ms-3.5",
        /// ```css
        /// {
        ///     margin-inline-end: 0.875rem; /* 14px */
        /// }
        /// ```
        Me3_5 => "me-3.5",
        /// ```css
        /// {
        ///     margin-top: 0.875rem; /* 14px */
        /// }
        /// ```
        Mt3_5 => "mt-3.5",
        /// ```css
        /// {
        ///     margin-right: 0.875rem; /* 14px */
        /// }
        /// ```
        Mr3_5 => "mr-3.5",
        /// ```css
        /// {
        ///     margin-bottom: 0.875rem; /* 14px */
        /// }
        /// ```
        Mb3_5 => "mb-3.5",
        /// ```css
        /// {
        ///     margin-left: 0.875rem; /* 14px */
        /// }
        /// ```
        Ml3_5 => "ml-3.5",
        /// ```css
        /// {
        ///     margin: 1rem; /* 16px */
        /// }
        /// ```
        M4 => "m-4",
        /// ```css
        /// {
        ///     margin-left: 1rem; /* 16px */
        ///     margin-right: 1rem; /* 16px */
        /// }
        /// ```
        Mx4 => "mx-4",
        /// ```css
        /// {
        ///     margin-top: 1rem; /* 16px */
        ///     margin-bottom: 1rem; /* 16px */
        /// }
        /// ```
        My4 => "my-4",
        /// ```css
        /// {
        ///     margin-inline-start: 1rem; /* 16px */
        /// }
        /// ```
        Ms4 => "ms-4",
        /// ```css
        /// {
        ///     margin-inline-end: 1rem; /* 16px */
        /// }
        /// ```
        Me4 => "me-4",
        /// ```css
        /// {
        ///     margin-top: 1rem; /* 16px */
        /// }
        /// ```
        Mt4 => "mt-4",
        /// ```css
        /// {
        ///     margin-right: 1rem; /* 16px */
        /// }
        /// ```
        Mr4 => "mr-4",
        /// ```css
        /// {
        ///     margin-bottom: 1rem; /* 16px */
        /// }
        /// ```
        Mb4 => "mb-4",
        /// ```css
        /// {
        ///     margin-left: 1rem; /* 16px */
        /// }
        /// ```
        Ml4 => "ml-4",
        /// ```css
        /// {
        ///     margin: 1.25rem; /* 20px */
        /// }
        /// ```
        M5 => "m-5",
        /// ```css
        /// {
        ///     margin-left: 1.25rem; /* 20px */
        ///     margin-right: 1.25rem; /* 20px */
        /// }
        /// ```
        Mx5 => "mx-5",
        /// ```css
        /// {
        ///     margin-top: 1.25rem; /* 20px */
        ///     margin-bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        My5 => "my-5",
        /// ```css
        /// {
        ///     margin-inline-start: 1.25rem; /* 20px */
        /// }
        /// ```
        Ms5 => "ms-5",
        /// ```css
        /// {
        ///     margin-inline-end: 1.25rem; /* 20px */
        /// }
        /// ```
        Me5 => "me-5",
        /// ```css
        /// {
        ///     margin-top: 1.25rem; /* 20px */
        /// }
        /// ```
        Mt5 => "mt-5",
        /// ```css
        /// {
        ///     margin-right: 1.25rem; /* 20px */
        /// }
        /// ```
        Mr5 => "mr-5",
        /// ```css
        /// {
        ///     margin-bottom: 1.25rem; /* 20px */
        /// }
        /// ```
        Mb5 => "mb-5",
        /// ```css
        /// {
        ///     margin-left: 1.25rem; /* 20px */
        /// }
        /// ```
        Ml5 => "ml-5",
        /// ```css
        /// {
        ///     margin: 1.5rem; /* 24px */
        /// }
        /// ```
        M6 => "m-6",
        /// ```css
        /// {
        ///     margin-left: 1.5rem; /* 24px */
        ///     margin-right: 1.5rem; /* 24px */
        /// }
        /// ```
        Mx6 => "mx-6",
        /// ```css
        /// {
        ///     margin-top: 1.5rem; /* 24px */
        ///     margin-bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        My6 => "my-6",
        /// ```css
        /// {
        ///     margin-inline-start: 1.5rem; /* 24px */
        /// }
        /// ```
        Ms6 => "ms-6",
        /// ```css
        /// {
        ///     margin-inline-end: 1.5rem; /* 24px */
        /// }
        /// ```
        Me6 => "me-6",
        /// ```css
        /// {
        ///     margin-top: 1.5rem; /* 24px */
        /// }
        /// ```
        Mt6 => "mt-6",
        /// ```css
        /// {
        ///     margin-right: 1.5rem; /* 24px */
        /// }
        /// ```
        Mr6 => "mr-6",
        /// ```css
        /// {
        ///     margin-bottom: 1.5rem; /* 24px */
        /// }
        /// ```
        Mb6 => "mb-6",
        /// ```css
        /// {
        ///     margin-left: 1.5rem; /* 24px */
        /// }
        /// ```
        Ml6 => "ml-6",
        /// ```css
        /// {
        ///     margin: 1.75rem; /* 28px */
        /// }
        /// ```
        M7 => "m-7",
        /// ```css
        /// {
        ///     margin-left: 1.75rem; /* 28px */
        ///     margin-right: 1.75rem; /* 28px */
        /// }
        /// ```
        Mx7 => "mx-7",
        /// ```css
        /// {
        ///     margin-top: 1.75rem; /* 28px */
        ///     margin-bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        My7 => "my-7",
        /// ```css
        /// {
        ///     margin-inline-start: 1.75rem; /* 28px */
        /// }
        /// ```
        Ms7 => "ms-7",
        /// ```css
        /// {
        ///     margin-inline-end: 1.75rem; /* 28px */
        /// }
        /// ```
        Me7 => "me-7",
        /// ```css
        /// {
        ///     margin-top: 1.75rem; /* 28px */
        /// }
        /// ```
        Mt7 => "mt-7",
        /// ```css
        /// {
        ///     margin-right: 1.75rem; /* 28px */
        /// }
        /// ```
        Mr7 => "mr-7",
        /// ```css
        /// {
        ///     margin-bottom: 1.75rem; /* 28px */
        /// }
        /// ```
        Mb7 => "mb-7",
        /// ```css
        /// {
        ///     margin-left: 1.75rem; /* 28px */
        /// }
        /// ```
        Ml7 => "ml-7",
        /// ```css
        /// {
        ///     margin: 2rem; /* 32px */
        /// }
        /// ```
        M8 => "m-8",
        /// ```css
        /// {
        ///     margin-left: 2rem; /* 32px */
        ///     margin-right: 2rem; /* 32px */
        /// }
        /// ```
        Mx8 => "mx-8",
        /// ```css
        /// {
        ///     margin-top: 2rem; /* 32px */
        ///     margin-bottom: 2rem; /* 32px */
        /// }
        /// ```
        My8 => "my-8",
        /// ```css
        /// {
        ///     margin-inline-start: 2rem; /* 32px */
        /// }
        /// ```
        Ms8 => "ms-8",
        /// ```css
        /// {
        ///     margin-inline-end: 2rem; /* 32px */
        /// }
        /// ```
        Me8 => "me-8",
        /// ```css
        /// {
        ///     margin-top: 2rem; /* 32px */
        /// }
        /// ```
        Mt8 => "mt-8",
        /// ```css
        /// {
        ///     margin-right: 2rem; /* 32px */
        /// }
        /// ```
        Mr8 => "mr-8",
        /// ```css
        /// {
        ///     margin-bottom: 2rem; /* 32px */
        /// }
        /// ```
        Mb8 => "mb-8",
        /// ```css
        /// {
        ///     margin-left: 2rem; /* 32px */
        /// }
        /// ```
        Ml8 => "ml-8",
        /// ```css
        /// {
        ///     margin: 2.25rem; /* 36px */
        /// }
        /// ```
        M9 => "m-9",
        /// ```css
        /// {
        ///     margin-left: 2.25rem; /* 36px */
        ///     margin-right: 2.25rem; /* 36px */
        /// }
        /// ```
        Mx9 => "mx-9",
        /// ```css
        /// {
        ///     margin-top: 2.25rem; /* 36px */
        ///     margin-bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        My9 => "my-9",
        /// ```css
        /// {
        ///     margin-inline-start: 2.25rem; /* 36px */
        /// }
        /// ```
        Ms9 => "ms-9",
        /// ```css
        /// {
        ///     margin-inline-end: 2.25rem; /* 36px */
        /// }
        /// ```
        Me9 => "me-9",
        /// ```css
        /// {
        ///     margin-top: 2.25rem; /* 36px */
        /// }
        /// ```
        Mt9 => "mt-9",
        /// ```css
        /// {
        ///     margin-right: 2.25rem; /* 36px */
        /// }
        /// ```
        Mr9 => "mr-9",
        /// ```css
        /// {
        ///     margin-bottom: 2.25rem; /* 36px */
        /// }
        /// ```
        Mb9 => "mb-9",
        /// ```css
        /// {
        ///     margin-left: 2.25rem; /* 36px */
        /// }
        /// ```
        Ml9 => "ml-9",
        /// ```css
        /// {
        ///     margin: 2.5rem; /* 40px */
        /// }
        /// ```
        M10 => "m-10",
        /// ```css
        /// {
        ///     margin-left: 2.5rem; /* 40px */
        ///     margin-right: 2.5rem; /* 40px */
        /// }
        /// ```
        Mx10 => "mx-10",
        /// ```css
        /// {
        ///     margin-top: 2.5rem; /* 40px */
        ///     margin-bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        My10 => "my-10",
        /// ```css
        /// {
        ///     margin-inline-start: 2.5rem; /* 40px */
        /// }
        /// ```
        Ms10 => "ms-10",
        /// ```css
        /// {
        ///     margin-inline-end: 2.5rem; /* 40px */
        /// }
        /// ```
        Me10 => "me-10",
        /// ```css
        /// {
        ///     margin-top: 2.5rem; /* 40px */
        /// }
        /// ```
        Mt10 => "mt-10",
        /// ```css
        /// {
        ///     margin-right: 2.5rem; /* 40px */
        /// }
        /// ```
        Mr10 => "mr-10",
        /// ```css
        /// {
        ///     margin-bottom: 2.5rem; /* 40px */
        /// }
        /// ```
        Mb10 => "mb-10",
        /// ```css
        /// {
        ///     margin-left: 2.5rem; /* 40px */
        /// }
        /// ```
        Ml10 => "ml-10",
        /// ```css
        /// {
        ///     margin: 2.75rem; /* 44px */
        /// }
        /// ```
        M11 => "m-11",
        /// ```css
        /// {
        ///     margin-left: 2.75rem; /* 44px */
        ///     margin-right: 2.75rem; /* 44px */
        /// }
        /// ```
        Mx11 => "mx-11",
        /// ```css
        /// {
        ///     margin-top: 2.75rem; /* 44px */
        ///     margin-bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        My11 => "my-11",
        /// ```css
        /// {
        ///     margin-inline-start: 2.75rem; /* 44px */
        /// }
        /// ```
        Ms11 => "ms-11",
        /// ```css
        /// {
        ///     margin-inline-end: 2.75rem; /* 44px */
        /// }
        /// ```
        Me11 => "me-11",
        /// ```css
        /// {
        ///     margin-top: 2.75rem; /* 44px */
        /// }
        /// ```
        Mt11 => "mt-11",
        /// ```css
        /// {
        ///     margin-right: 2.75rem; /* 44px */
        /// }
        /// ```
        Mr11 => "mr-11",
        /// ```css
        /// {
        ///     margin-bottom: 2.75rem; /* 44px */
        /// }
        /// ```
        Mb11 => "mb-11",
        /// ```css
        /// {
        ///     margin-left: 2.75rem; /* 44px */
        /// }
        /// ```
        Ml11 => "ml-11",
        /// ```css
        /// {
        ///     margin: 3rem; /* 48px */
        /// }
        /// ```
        M12 => "m-12",
        /// ```css
        /// {
        ///     margin-left: 3rem; /* 48px */
        ///     margin-right: 3rem; /* 48px */
        /// }
        /// ```
        Mx12 => "mx-12",
        /// ```css
        /// {
        ///     margin-top: 3rem; /* 48px */
        ///     margin-bottom: 3rem; /* 48px */
        /// }
        /// ```
        My12 => "my-12",
        /// ```css
        /// {
        ///     margin-inline-start: 3rem; /* 48px */
        /// }
        /// ```
        Ms12 => "ms-12",
        /// ```css
        /// {
        ///     margin-inline-end: 3rem; /* 48px */
        /// }
        /// ```
        Me12 => "me-12",
        /// ```css
        /// {
        ///     margin-top: 3rem; /* 48px */
        /// }
        /// ```
        Mt12 => "mt-12",
        /// ```css
        /// {
        ///     margin-right: 3rem; /* 48px */
        /// }
        /// ```
        Mr12 => "mr-12",
        /// ```css
        /// {
        ///     margin-bottom: 3rem; /* 48px */
        /// }
        /// ```
        Mb12 => "mb-12",
        /// ```css
        /// {
        ///     margin-left: 3rem; /* 48px */
        /// }
        /// ```
        Ml12 => "ml-12",
        /// ```css
        /// {
        ///     margin: 3.5rem; /* 56px */
        /// }
        /// ```
        M14 => "m-14",
        /// ```css
        /// {
        ///     margin-left: 3.5rem; /* 56px */
        ///     margin-right: 3.5rem; /* 56px */
        /// }
        /// ```
        Mx14 => "mx-14",
        /// ```css
        /// {
        ///     margin-top: 3.5rem; /* 56px */
        ///     margin-bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        My14 => "my-14",
        /// ```css
        /// {
        ///     margin-inline-start: 3.5rem; /* 56px */
        /// }
        /// ```
        Ms14 => "ms-14",
        /// ```css
        /// {
        ///     margin-inline-end: 3.5rem; /* 56px */
        /// }
        /// ```
        Me14 => "me-14",
        /// ```css
        /// {
        ///     margin-top: 3.5rem; /* 56px */
        /// }
        /// ```
        Mt14 => "mt-14",
        /// ```css
        /// {
        ///     margin-right: 3.5rem; /* 56px */
        /// }
        /// ```
        Mr14 => "mr-14",
        /// ```css
        /// {
        ///     margin-bottom: 3.5rem; /* 56px */
        /// }
        /// ```
        Mb14 => "mb-14",
        /// ```css
        /// {
        ///     margin-left: 3.5rem; /* 56px */
        /// }
        /// ```
        Ml14 => "ml-14",
        /// ```css
        /// {
        ///     margin: 4rem; /* 64px */
        /// }
        /// ```
        M16 => "m-16",
        /// ```css
        /// {
        ///     margin-left: 4rem; /* 64px */
        ///     margin-right: 4rem; /* 64px */
        /// }
        /// ```
        Mx16 => "mx-16",
        /// ```css
        /// {
        ///     margin-top: 4rem; /* 64px */
        ///     margin-bottom: 4rem; /* 64px */
        /// }
        /// ```
        My16 => "my-16",
        /// ```css
        /// {
        ///     margin-inline-start: 4rem; /* 64px */
        /// }
        /// ```
        Ms16 => "ms-16",
        /// ```css
        /// {
        ///     margin-inline-end: 4rem; /* 64px */
        /// }
        /// ```
        Me16 => "me-16",
        /// ```css
        /// {
        ///     margin-top: 4rem; /* 64px */
        /// }
        /// ```
        Mt16 => "mt-16",
        /// ```css
        /// {
        ///     margin-right: 4rem; /* 64px */
        /// }
        /// ```
        Mr16 => "mr-16",
        /// ```css
        /// {
        ///     margin-bottom: 4rem; /* 64px */
        /// }
        /// ```
        Mb16 => "mb-16",
        /// ```css
        /// {
        ///     margin-left: 4rem; /* 64px */
        /// }
        /// ```
        Ml16 => "ml-16",
        /// ```css
        /// {
        ///     margin: 5rem; /* 80px */
        /// }
        /// ```
        M20 => "m-20",
        /// ```css
        /// {
        ///     margin-left: 5rem; /* 80px */
        ///     margin-right: 5rem; /* 80px */
        /// }
        /// ```
        Mx20 => "mx-20",
        /// ```css
        /// {
        ///     margin-top: 5rem; /* 80px */
        ///     margin-bottom: 5rem; /* 80px */
        /// }
        /// ```
        My20 => "my-20",
        /// ```css
        /// {
        ///     margin-inline-start: 5rem; /* 80px */
        /// }
        /// ```
        Ms20 => "ms-20",
        /// ```css
        /// {
        ///     margin-inline-end: 5rem; /* 80px */
        /// }
        /// ```
        Me20 => "me-20",
        /// ```css
        /// {
        ///     margin-top: 5rem; /* 80px */
        /// }
        /// ```
        Mt20 => "mt-20",
        /// ```css
        /// {
        ///     margin-right: 5rem; /* 80px */
        /// }
        /// ```
        Mr20 => "mr-20",
        /// ```css
        /// {
        ///     margin-bottom: 5rem; /* 80px */
        /// }
        /// ```
        Mb20 => "mb-20",
        /// ```css
        /// {
        ///     margin-left: 5rem; /* 80px */
        /// }
        /// ```
        Ml20 => "ml-20",
        /// ```css
        /// {
        ///     margin: 6rem; /* 96px */
        /// }
        /// ```
        M24 => "m-24",
        /// ```css
        /// {
        ///     margin-left: 6rem; /* 96px */
        ///     margin-right: 6rem; /* 96px */
        /// }
        /// ```
        Mx24 => "mx-24",
        /// ```css
        /// {
        ///     margin-top: 6rem; /* 96px */
        ///     margin-bottom: 6rem; /* 96px */
        /// }
        /// ```
        My24 => "my-24",
        /// ```css
        /// {
        ///     margin-inline-start: 6rem; /* 96px */
        /// }
        /// ```
        Ms24 => "ms-24",
        /// ```css
        /// {
        ///     margin-inline-end: 6rem; /* 96px */
        /// }
        /// ```
        Me24 => "me-24",
        /// ```css
        /// {
        ///     margin-top: 6rem; /* 96px */
        /// }
        /// ```
        Mt24 => "mt-24",
        /// ```css
        /// {
        ///     margin-right: 6rem; /* 96px */
        /// }
        /// ```
        Mr24 => "mr-24",
        /// ```css
        /// {
        ///     margin-bottom: 6rem; /* 96px */
        /// }
        /// ```
        Mb24 => "mb-24",
        /// ```css
        /// {
        ///     margin-left: 6rem; /* 96px */
        /// }
        /// ```
        Ml24 => "ml-24",
        /// ```css
        /// {
        ///     margin: 7rem; /* 112px */
        /// }
        /// ```
        M28 => "m-28",
        /// ```css
        /// {
        ///     margin-left: 7rem; /* 112px */
        ///     margin-right: 7rem; /* 112px */
        /// }
        /// ```
        Mx28 => "mx-28",
        /// ```css
        /// {
        ///     margin-top: 7rem; /* 112px */
        ///     margin-bottom: 7rem; /* 112px */
        /// }
        /// ```
        My28 => "my-28",
        /// ```css
        /// {
        ///     margin-inline-start: 7rem; /* 112px */
        /// }
        /// ```
        Ms28 => "ms-28",
        /// ```css
        /// {
        ///     margin-inline-end: 7rem; /* 112px */
        /// }
        /// ```
        Me28 => "me-28",
        /// ```css
        /// {
        ///     margin-top: 7rem; /* 112px */
        /// }
        /// ```
        Mt28 => "mt-28",
        /// ```css
        /// {
        ///     margin-right: 7rem; /* 112px */
        /// }
        /// ```
        Mr28 => "mr-28",
        /// ```css
        /// {
        ///     margin-bottom: 7rem; /* 112px */
        /// }
        /// ```
        Mb28 => "mb-28",
        /// ```css
        /// {
        ///     margin-left: 7rem; /* 112px */
        /// }
        /// ```
        Ml28 => "ml-28",
        /// ```css
        /// {
        ///     margin: 8rem; /* 128px */
        /// }
        /// ```
        M32 => "m-32",
        /// ```css
        /// {
        ///     margin-left: 8rem; /* 128px */
        ///     margin-right: 8rem; /* 128px */
        /// }
        /// ```
        Mx32 => "mx-32",
        /// ```css
        /// {
        ///     margin-top: 8rem; /* 128px */
        ///     margin-bottom: 8rem; /* 128px */
        /// }
        /// ```
        My32 => "my-32",
        /// ```css
        /// {
        ///     margin-inline-start: 8rem; /* 128px */
        /// }
        /// ```
        Ms32 => "ms-32",
        /// ```css
        /// {
        ///     margin-inline-end: 8rem; /* 128px */
        /// }
        /// ```
        Me32 => "me-32",
        /// ```css
        /// {
        ///     margin-top: 8rem; /* 128px */
        /// }
        /// ```
        Mt32 => "mt-32",
        /// ```css
        /// {
        ///     margin-right: 8rem; /* 128px */
        /// }
        /// ```
        Mr32 => "mr-32",
        /// ```css
        /// {
        ///     margin-bottom: 8rem; /* 128px */
        /// }
        /// ```
        Mb32 => "mb-32",
        /// ```css
        /// {
        ///     margin-left: 8rem; /* 128px */
        /// }
        /// ```
        Ml32 => "ml-32",
        /// ```css
        /// {
        ///     margin: 9rem; /* 144px */
        /// }
        /// ```
        M36 => "m-36",
        /// ```css
        /// {
        ///     margin-left: 9rem; /* 144px */
        ///     margin-right: 9rem; /* 144px */
        /// }
        /// ```
        Mx36 => "mx-36",
        /// ```css
        /// {
        ///     margin-top: 9rem; /* 144px */
        ///     margin-bottom: 9rem; /* 144px */
        /// }
        /// ```
        My36 => "my-36",
        /// ```css
        /// {
        ///     margin-inline-start: 9rem; /* 144px */
        /// }
        /// ```
        Ms36 => "ms-36",
        /// ```css
        /// {
        ///     margin-inline-end: 9rem; /* 144px */
        /// }
        /// ```
        Me36 => "me-36",
        /// ```css
        /// {
        ///     margin-top: 9rem; /* 144px */
        /// }
        /// ```
        Mt36 => "mt-36",
        /// ```css
        /// {
        ///     margin-right: 9rem; /* 144px */
        /// }
        /// ```
        Mr36 => "mr-36",
        /// ```css
        /// {
        ///     margin-bottom: 9rem; /* 144px */
        /// }
        /// ```
        Mb36 => "mb-36",
        /// ```css
        /// {
        ///     margin-left: 9rem; /* 144px */
        /// }
        /// ```
        Ml36 => "ml-36",
        /// ```css
        /// {
        ///     margin: 10rem; /* 160px */
        /// }
        /// ```
        M40 => "m-40",
        /// ```css
        /// {
        ///     margin-left: 10rem; /* 160px */
        ///     margin-right: 10rem; /* 160px */
        /// }
        /// ```
        Mx40 => "mx-40",
        /// ```css
        /// {
        ///     margin-top: 10rem; /* 160px */
        ///     margin-bottom: 10rem; /* 160px */
        /// }
        /// ```
        My40 => "my-40",
        /// ```css
        /// {
        ///     margin-inline-start: 10rem; /* 160px */
        /// }
        /// ```
        Ms40 => "ms-40",
        /// ```css
        /// {
        ///     margin-inline-end: 10rem; /* 160px */
        /// }
        /// ```
        Me40 => "me-40",
        /// ```css
        /// {
        ///     margin-top: 10rem; /* 160px */
        /// }
        /// ```
        Mt40 => "mt-40",
        /// ```css
        /// {
        ///     margin-right: 10rem; /* 160px */
        /// }
        /// ```
        Mr40 => "mr-40",
        /// ```css
        /// {
        ///     margin-bottom: 10rem; /* 160px */
        /// }
        /// ```
        Mb40 => "mb-40",
        /// ```css
        /// {
        ///     margin-left: 10rem; /* 160px */
        /// }
        /// ```
        Ml40 => "ml-40",
        /// ```css
        /// {
        ///     margin: 11rem; /* 176px */
        /// }
        /// ```
        M44 => "m-44",
        /// ```css
        /// {
        ///     margin-left: 11rem; /* 176px */
        ///     margin-right: 11rem; /* 176px */
        /// }
        /// ```
        Mx44 => "mx-44",
        /// ```css
        /// {
        ///     margin-top: 11rem; /* 176px */
        ///     margin-bottom: 11rem; /* 176px */
        /// }
        /// ```
        My44 => "my-44",
        /// ```css
        /// {
        ///     margin-inline-start: 11rem; /* 176px */
        /// }
        /// ```
        Ms44 => "ms-44",
        /// ```css
        /// {
        ///     margin-inline-end: 11rem; /* 176px */
        /// }
        /// ```
        Me44 => "me-44",
        /// ```css
        /// {
        ///     margin-top: 11rem; /* 176px */
        /// }
        /// ```
        Mt44 => "mt-44",
        /// ```css
        /// {
        ///     margin-right: 11rem; /* 176px */
        /// }
        /// ```
        Mr44 => "mr-44",
        /// ```css
        /// {
        ///     margin-bottom: 11rem; /* 176px */
        /// }
        /// ```
        Mb44 => "mb-44",
        /// ```css
        /// {
        ///     margin-left: 11rem; /* 176px */
        /// }
        /// ```
        Ml44 => "ml-44",
        /// ```css
        /// {
        ///     margin: 12rem; /* 192px */
        /// }
        /// ```
        M48 => "m-48",
        /// ```css
        /// {
        ///     margin-left: 12rem; /* 192px */
        ///     margin-right: 12rem; /* 192px */
        /// }
        /// ```
        Mx48 => "mx-48",
        /// ```css
        /// {
        ///     margin-top: 12rem; /* 192px */
        ///     margin-bottom: 12rem; /* 192px */
        /// }
        /// ```
        My48 => "my-48",
        /// ```css
        /// {
        ///     margin-inline-start: 12rem; /* 192px */
        /// }
        /// ```
        Ms48 => "ms-48",
        /// ```css
        /// {
        ///     margin-inline-end: 12rem; /* 192px */
        /// }
        /// ```
        Me48 => "me-48",
        /// ```css
        /// {
        ///     margin-top: 12rem; /* 192px */
        /// }
        /// ```
        Mt48 => "mt-48",
        /// ```css
        /// {
        ///     margin-right: 12rem; /* 192px */
        /// }
        /// ```
        Mr48 => "mr-48",
        /// ```css
        /// {
        ///     margin-bottom: 12rem; /* 192px */
        /// }
        /// ```
        Mb48 => "mb-48",
        /// ```css
        /// {
        ///     margin-left: 12rem; /* 192px */
        /// }
        /// ```
        Ml48 => "ml-48",
        /// ```css
        /// {
        ///     margin: 13rem; /* 208px */
        /// }
        /// ```
        M52 => "m-52",
        /// ```css
        /// {
        ///     margin-left: 13rem; /* 208px */
        ///     margin-right: 13rem; /* 208px */
        /// }
        /// ```
        Mx52 => "mx-52",
        /// ```css
        /// {
        ///     margin-top: 13rem; /* 208px */
        ///     margin-bottom: 13rem; /* 208px */
        /// }
        /// ```
        My52 => "my-52",
        /// ```css
        /// {
        ///     margin-inline-start: 13rem; /* 208px */
        /// }
        /// ```
        Ms52 => "ms-52",
        /// ```css
        /// {
        ///     margin-inline-end: 13rem; /* 208px */
        /// }
        /// ```
        Me52 => "me-52",
        /// ```css
        /// {
        ///     margin-top: 13rem; /* 208px */
        /// }
        /// ```
        Mt52 => "mt-52",
        /// ```css
        /// {
        ///     margin-right: 13rem; /* 208px */
        /// }
        /// ```
        Mr52 => "mr-52",
        /// ```css
        /// {
        ///     margin-bottom: 13rem; /* 208px */
        /// }
        /// ```
        Mb52 => "mb-52",
        /// ```css
        /// {
        ///     margin-left: 13rem; /* 208px */
        /// }
        /// ```
        Ml52 => "ml-52",
        /// ```css
        /// {
        ///     margin: 14rem; /* 224px */
        /// }
        /// ```
        M56 => "m-56",
        /// ```css
        /// {
        ///     margin-left: 14rem; /* 224px */
        ///     margin-right: 14rem; /* 224px */
        /// }
        /// ```
        Mx56 => "mx-56",
        /// ```css
        /// {
        ///     margin-top: 14rem; /* 224px */
        ///     margin-bottom: 14rem; /* 224px */
        /// }
        /// ```
        My56 => "my-56",
        /// ```css
        /// {
        ///     margin-inline-start: 14rem; /* 224px */
        /// }
        /// ```
        Ms56 => "ms-56",
        /// ```css
        /// {
        ///     margin-inline-end: 14rem; /* 224px */
        /// }
        /// ```
        Me56 => "me-56",
        /// ```css
        /// {
        ///     margin-top: 14rem; /* 224px */
        /// }
        /// ```
        Mt56 => "mt-56",
        /// ```css
        /// {
        ///     margin-right: 14rem; /* 224px */
        /// }
        /// ```
        Mr56 => "mr-56",
        /// ```css
        /// {
        ///     margin-bottom: 14rem; /* 224px */
        /// }
        /// ```
        Mb56 => "mb-56",
        /// ```css
        /// {
        ///     margin-left: 14rem; /* 224px */
        /// }
        /// ```
        Ml56 => "ml-56",
        /// ```css
        /// {
        ///     margin: 15rem; /* 240px */
        /// }
        /// ```
        M60 => "m-60",
        /// ```css
        /// {
        ///     margin-left: 15rem; /* 240px */
        ///     margin-right: 15rem; /* 240px */
        /// }
        /// ```
        Mx60 => "mx-60",
        /// ```css
        /// {
        ///     margin-top: 15rem; /* 240px */
        ///     margin-bottom: 15rem; /* 240px */
        /// }
        /// ```
        My60 => "my-60",
        /// ```css
        /// {
        ///     margin-inline-start: 15rem; /* 240px */
        /// }
        /// ```
        Ms60 => "ms-60",
        /// ```css
        /// {
        ///     margin-inline-end: 15rem; /* 240px */
        /// }
        /// ```
        Me60 => "me-60",
        /// ```css
        /// {
        ///     margin-top: 15rem; /* 240px */
        /// }
        /// ```
        Mt60 => "mt-60",
        /// ```css
        /// {
        ///     margin-right: 15rem; /* 240px */
        /// }
        /// ```
        Mr60 => "mr-60",
        /// ```css
        /// {
        ///     margin-bottom: 15rem; /* 240px */
        /// }
        /// ```
        Mb60 => "mb-60",
        /// ```css
        /// {
        ///     margin-left: 15rem; /* 240px */
        /// }
        /// ```
        Ml60 => "ml-60",
        /// ```css
        /// {
        ///     margin: 16rem; /* 256px */
        /// }
        /// ```
        M64 => "m-64",
        /// ```css
        /// {
        ///     margin-left: 16rem; /* 256px */
        ///     margin-right: 16rem; /* 256px */
        /// }
        /// ```
        Mx64 => "mx-64",
        /// ```css
        /// {
        ///     margin-top: 16rem; /* 256px */
        ///     margin-bottom: 16rem; /* 256px */
        /// }
        /// ```
        My64 => "my-64",
        /// ```css
        /// {
        ///     margin-inline-start: 16rem; /* 256px */
        /// }
        /// ```
        Ms64 => "ms-64",
        /// ```css
        /// {
        ///     margin-inline-end: 16rem; /* 256px */
        /// }
        /// ```
        Me64 => "me-64",
        /// ```css
        /// {
        ///     margin-top: 16rem; /* 256px */
        /// }
        /// ```
        Mt64 => "mt-64",
        /// ```css
        /// {
        ///     margin-right: 16rem; /* 256px */
        /// }
        /// ```
        Mr64 => "mr-64",
        /// ```css
        /// {
        ///     margin-bottom: 16rem; /* 256px */
        /// }
        /// ```
        Mb64 => "mb-64",
        /// ```css
        /// {
        ///     margin-left: 16rem; /* 256px */
        /// }
        /// ```
        Ml64 => "ml-64",
        /// ```css
        /// {
        ///     margin: 18rem; /* 288px */
        /// }
        /// ```
        M72 => "m-72",
        /// ```css
        /// {
        ///     margin-left: 18rem; /* 288px */
        ///     margin-right: 18rem; /* 288px */
        /// }
        /// ```
        Mx72 => "mx-72",
        /// ```css
        /// {
        ///     margin-top: 18rem; /* 288px */
        ///     margin-bottom: 18rem; /* 288px */
        /// }
        /// ```
        My72 => "my-72",
        /// ```css
        /// {
        ///     margin-inline-start: 18rem; /* 288px */
        /// }
        /// ```
        Ms72 => "ms-72",
        /// ```css
        /// {
        ///     margin-inline-end: 18rem; /* 288px */
        /// }
        /// ```
        Me72 => "me-72",
        /// ```css
        /// {
        ///     margin-top: 18rem; /* 288px */
        /// }
        /// ```
        Mt72 => "mt-72",
        /// ```css
        /// {
        ///     margin-right: 18rem; /* 288px */
        /// }
        /// ```
        Mr72 => "mr-72",
        /// ```css
        /// {
        ///     margin-bottom: 18rem; /* 288px */
        /// }
        /// ```
        Mb72 => "mb-72",
        /// ```css
        /// {
        ///     margin-left: 18rem; /* 288px */
        /// }
        /// ```
        Ml72 => "ml-72",
        /// ```css
        /// {
        ///     margin: 20rem; /* 320px */
        /// }
        /// ```
        M80 => "m-80",
        /// ```css
        /// {
        ///     margin-left: 20rem; /* 320px */
        ///     margin-right: 20rem; /* 320px */
        /// }
        /// ```
        Mx80 => "mx-80",
        /// ```css
        /// {
        ///     margin-top: 20rem; /* 320px */
        ///     margin-bottom: 20rem; /* 320px */
        /// }
        /// ```
        My80 => "my-80",
        /// ```css
        /// {
        ///     margin-inline-start: 20rem; /* 320px */
        /// }
        /// ```
        Ms80 => "ms-80",
        /// ```css
        /// {
        ///     margin-inline-end: 20rem; /* 320px */
        /// }
        /// ```
        Me80 => "me-80",
        /// ```css
        /// {
        ///     margin-top: 20rem; /* 320px */
        /// }
        /// ```
        Mt80 => "mt-80",
        /// ```css
        /// {
        ///     margin-right: 20rem; /* 320px */
        /// }
        /// ```
        Mr80 => "mr-80",
        /// ```css
        /// {
        ///     margin-bottom: 20rem; /* 320px */
        /// }
        /// ```
        Mb80 => "mb-80",
        /// ```css
        /// {
        ///     margin-left: 20rem; /* 320px */
        /// }
        /// ```
        Ml80 => "ml-80",
        /// ```css
        /// {
        ///     margin: 24rem; /* 384px */
        /// }
        /// ```
        M96 => "m-96",
        /// ```css
        /// {
        ///     margin-left: 24rem; /* 384px */
        ///     margin-right: 24rem; /* 384px */
        /// }
        /// ```
        Mx96 => "mx-96",
        /// ```css
        /// {
        ///     margin-top: 24rem; /* 384px */
        ///     margin-bottom: 24rem; /* 384px */
        /// }
        /// ```
        My96 => "my-96",
        /// ```css
        /// {
        ///     margin-inline-start: 24rem; /* 384px */
        /// }
        /// ```
        Ms96 => "ms-96",
        /// ```css
        /// {
        ///     margin-inline-end: 24rem; /* 384px */
        /// }
        /// ```
        Me96 => "me-96",
        /// ```css
        /// {
        ///     margin-top: 24rem; /* 384px */
        /// }
        /// ```
        Mt96 => "mt-96",
        /// ```css
        /// {
        ///     margin-right: 24rem; /* 384px */
        /// }
        /// ```
        Mr96 => "mr-96",
        /// ```css
        /// {
        ///     margin-bottom: 24rem; /* 384px */
        /// }
        /// ```
        Mb96 => "mb-96",
        /// ```css
        /// {
        ///     margin-left: 24rem; /* 384px */
        /// }
        /// ```
        Ml96 => "ml-96",
        /// ```css
        /// {
        ///     margin: auto;
        /// }
        /// ```
        MAuto => "m-auto",
        /// ```css
        /// {
        ///     margin-left: auto;
        ///     margin-right: auto;
        /// }
        /// ```
        MxAuto => "mx-auto",
        /// ```css
        /// {
        ///     margin-top: auto;
        ///     margin-bottom: auto;
        /// }
        /// ```
        MyAuto => "my-auto",
        /// ```css
        /// {
        ///     margin-inline-start: auto;
        /// }
        /// ```
        MsAuto => "ms-auto",
        /// ```css
        /// {
        ///     margin-inline-end: auto;
        /// }
        /// ```
        MeAuto => "me-auto",
        /// ```css
        /// {
        ///     margin-top: auto;
        /// }
        /// ```
        MtAuto => "mt-auto",
        /// ```css
        /// {
        ///     margin-right: auto;
        /// }
        /// ```
        MrAuto => "mr-auto",
        /// ```css
        /// {
        ///     margin-bottom: auto;
        /// }
        /// ```
        MbAuto => "mb-auto",
        /// ```css
        /// {
        ///     margin-left: auto;
        /// }
        /// ```
        MlAuto => "ml-auto",
    }
    /// Utilities for controlling the space between child elements.
    ///
    /// <https://tailwindcss.com/docs/space>
    SpaceBetween {
        /// ```css
        /// {
        ///     margin-left: 0px;
        /// }
        /// ```
        X0 => "space-x-0",
        /// ```css
        /// {
        ///     margin-top: 0px;
        /// }
        /// ```
        Y0 => "space-y-0",
        /// ```css
        /// {
        ///     margin-left: 0.125rem; /* 2px */
        /// }
        /// ```
        X0_5 => "space-x-0.5",
        /// ```css
        /// {
        ///     margin-top: 0.125rem; /* 2px */
        /// }
        /// ```
        Y0_5 => "space-y-0.5",
        /// ```css
        /// {
        ///     margin-left: 0.25rem; /* 4px */
        /// }
        /// ```
        X1 => "space-x-1",
        /// ```css
        /// {
        ///     margin-top: 0.25rem; /* 4px */
        /// }
        /// ```
        Y1 => "space-y-1",
        /// ```css
        /// {
        ///     margin-left: 0.375rem; /* 6px */
        /// }
        /// ```
        X1_5 => "space-x-1.5",
        /// ```css
        /// {
        ///     margin-top: 0.375rem; /* 6px */
        /// }
        /// ```
        Y1_5 => "space-y-1.5",
        /// ```css
        /// {
        ///     margin-left: 0.5rem; /* 8px */
        /// }
        /// ```
        X2 => "space-x-2",
        /// ```css
        /// {
        ///     margin-top: 0.5rem; /* 8px */
        /// }
        /// ```
        Y2 => "space-y-2",
        /// ```css
        /// {
        ///     margin-left: 0.625rem; /* 10px */
        /// }
        /// ```
        X2_5 => "space-x-2.5",
        /// ```css
        /// {
        ///     margin-top: 0.625rem; /* 10px */
        /// }
        /// ```
        Y2_5 => "space-y-2.5",
        /// ```css
        /// {
        ///     margin-left: 0.75rem; /* 12px */
        /// }
        /// ```
        X3 => "space-x-3",
        /// ```css
        /// {
        ///     margin-top: 0.75rem; /* 12px */
        /// }
        /// ```
        Y3 => "space-y-3",
        /// ```css
        /// {
        ///     margin-left: 0.875rem; /* 14px */
        /// }
        /// ```
        X3_5 => "space-x-3.5",
        /// ```css
        /// {
        ///     margin-top: 0.875rem; /* 14px */
        /// }
        /// ```
        Y3_5 => "space-y-3.5",
        /// ```css
        /// {
        ///     margin-left: 1rem; /* 16px */
        /// }
        /// ```
        X4 => "space-x-4",
        /// ```css
        /// {
        ///     margin-top: 1rem; /* 16px */
        /// }
        /// ```
        Y4 => "space-y-4",
        /// ```css
        /// {
        ///     margin-left: 1.25rem; /* 20px */
        /// }
        /// ```
        X5 => "space-x-5",
        /// ```css
        /// {
        ///     margin-top: 1.25rem; /* 20px */
        /// }
        /// ```
        Y5 => "space-y-5",
        /// ```css
        /// {
        ///     margin-left: 1.5rem; /* 24px */
        /// }
        /// ```
        X6 => "space-x-6",
        /// ```css
        /// {
        ///     margin-top: 1.5rem; /* 24px */
        /// }
        /// ```
        Y6 => "space-y-6",
        /// ```css
        /// {
        ///     margin-left: 1.75rem; /* 28px */
        /// }
        /// ```
        X7 => "space-x-7",
        /// ```css
        /// {
        ///     margin-top: 1.75rem; /* 28px */
        /// }
        /// ```
        Y7 => "space-y-7",
        /// ```css
        /// {
        ///     margin-left: 2rem; /* 32px */
        /// }
        /// ```
        X8 => "space-x-8",
        /// ```css
        /// {
        ///     margin-top: 2rem; /* 32px */
        /// }
        /// ```
        Y8 => "space-y-8",
        /// ```css
        /// {
        ///     margin-left: 2.25rem; /* 36px */
        /// }
        /// ```
        X9 => "space-x-9",
        /// ```css
        /// {
        ///     margin-top: 2.25rem; /* 36px */
        /// }
        /// ```
        Y9 => "space-y-9",
        /// ```css
        /// {
        ///     margin-left: 2.5rem; /* 40px */
        /// }
        /// ```
        X10 => "space-x-10",
        /// ```css
        /// {
        ///     margin-top: 2.5rem; /* 40px */
        /// }
        /// ```
        Y10 => "space-y-10",
        /// ```css
        /// {
        ///     margin-left: 2.75rem; /* 44px */
        /// }
        /// ```
        X11 => "space-x-11",
        /// ```css
        /// {
        ///     margin-top: 2.75rem; /* 44px */
        /// }
        /// ```
        Y11 => "space-y-11",
        /// ```css
        /// {
        ///     margin-left: 3rem; /* 48px */
        /// }
        /// ```
        X12 => "space-x-12",
        /// ```css
        /// {
        ///     margin-top: 3rem; /* 48px */
        /// }
        /// ```
        Y12 => "space-y-12",
        /// ```css
        /// {
        ///     margin-left: 3.5rem; /* 56px */
        /// }
        /// ```
        X14 => "space-x-14",
        /// ```css
        /// {
        ///     margin-top: 3.5rem; /* 56px */
        /// }
        /// ```
        Y14 => "space-y-14",
        /// ```css
        /// {
        ///     margin-left: 4rem; /* 64px */
        /// }
        /// ```
        X16 => "space-x-16",
        /// ```css
        /// {
        ///     margin-top: 4rem; /* 64px */
        /// }
        /// ```
        Y16 => "space-y-16",
        /// ```css
        /// {
        ///     margin-left: 5rem; /* 80px */
        /// }
        /// ```
        X20 => "space-x-20",
        /// ```css
        /// {
        ///     margin-top: 5rem; /* 80px */
        /// }
        /// ```
        Y20 => "space-y-20",
        /// ```css
        /// {
        ///     margin-left: 6rem; /* 96px */
        /// }
        /// ```
        X24 => "space-x-24",
        /// ```css
        /// {
        ///     margin-top: 6rem; /* 96px */
        /// }
        /// ```
        Y24 => "space-y-24",
        /// ```css
        /// {
        ///     margin-left: 7rem; /* 112px */
        /// }
        /// ```
        X28 => "space-x-28",
        /// ```css
        /// {
        ///     margin-top: 7rem; /* 112px */
        /// }
        /// ```
        Y28 => "space-y-28",
        /// ```css
        /// {
        ///     margin-left: 8rem; /* 128px */
        /// }
        /// ```
        X32 => "space-x-32",
        /// ```css
        /// {
        ///     margin-top: 8rem; /* 128px */
        /// }
        /// ```
        Y32 => "space-y-32",
        /// ```css
        /// {
        ///     margin-left: 9rem; /* 144px */
        /// }
        /// ```
        X36 => "space-x-36",
        /// ```css
        /// {
        ///     margin-top: 9rem; /* 144px */
        /// }
        /// ```
        Y36 => "space-y-36",
        /// ```css
        /// {
        ///     margin-left: 10rem; /* 160px */
        /// }
        /// ```
        X40 => "space-x-40",
        /// ```css
        /// {
        ///     margin-top: 10rem; /* 160px */
        /// }
        /// ```
        Y40 => "space-y-40",
        /// ```css
        /// {
        ///     margin-left: 11rem; /* 176px */
        /// }
        /// ```
        X44 => "space-x-44",
        /// ```css
        /// {
        ///     margin-top: 11rem; /* 176px */
        /// }
        /// ```
        Y44 => "space-y-44",
        /// ```css
        /// {
        ///     margin-left: 12rem; /* 192px */
        /// }
        /// ```
        X48 => "space-x-48",
        /// ```css
        /// {
        ///     margin-top: 12rem; /* 192px */
        /// }
        /// ```
        Y48 => "space-y-48",
        /// ```css
        /// {
        ///     margin-left: 13rem; /* 208px */
        /// }
        /// ```
        X52 => "space-x-52",
        /// ```css
        /// {
        ///     margin-top: 13rem; /* 208px */
        /// }
        /// ```
        Y52 => "space-y-52",
        /// ```css
        /// {
        ///     margin-left: 14rem; /* 224px */
        /// }
        /// ```
        X56 => "space-x-56",
        /// ```css
        /// {
        ///     margin-top: 14rem; /* 224px */
        /// }
        /// ```
        Y56 => "space-y-56",
        /// ```css
        /// {
        ///     margin-left: 15rem; /* 240px */
        /// }
        /// ```
        X60 => "space-x-60",
        /// ```css
        /// {
        ///     margin-top: 15rem; /* 240px */
        /// }
        /// ```
        Y60 => "space-y-60",
        /// ```css
        /// {
        ///     margin-left: 16rem; /* 256px */
        /// }
        /// ```
        X64 => "space-x-64",
        /// ```css
        /// {
        ///     margin-top: 16rem; /* 256px */
        /// }
        /// ```
        Y64 => "space-y-64",
        /// ```css
        /// {
        ///     margin-left: 18rem; /* 288px */
        /// }
        /// ```
        X72 => "space-x-72",
        /// ```css
        /// {
        ///     margin-top: 18rem; /* 288px */
        /// }
        /// ```
        Y72 => "space-y-72",
        /// ```css
        /// {
        ///     margin-left: 20rem; /* 320px */
        /// }
        /// ```
        X80 => "space-x-80",
        /// ```css
        /// {
        ///     margin-top: 20rem; /* 320px */
        /// }
        /// ```
        Y80 => "space-y-80",
        /// ```css
        /// {
        ///     margin-left: 24rem; /* 384px */
        /// }
        /// ```
        X96 => "space-x-96",
        /// ```css
        /// {
        ///     margin-top: 24rem; /* 384px */
        /// }
        /// ```
        Y96 => "space-y-96",
        /// ```css
        /// {
        ///     margin-left: 1px;
        /// }
        /// ```
        XPx => "space-x-px",
        /// ```css
        /// {
        ///     margin-top: 1px;
        /// }
        /// ```
        YPx => "space-y-px",
        /// ```css
        /// {
        ///     --tw-space-y-reverse: 1;
        /// }
        /// ```
        YReverse => "space-y-reverse",
        /// ```css
        /// {
        ///     --tw-space-x-reverse: 1;
        /// }
        /// ```
        XReverse => "space-x-reverse",
    }
);
