def_types!(
    /// Utilities for improving accessibility with screen readers.
    ///
    /// <https://tailwindcss.com/docs/screen-readers>
    ScreenReaders {
        /// ```css
        /// {
        ///     position: absolute;
        ///     width: 1px;
        ///     height: 1px;
        ///     padding: 0;
        ///     margin: -1px;
        ///     overflow: hidden;
        ///     clip: rect(0, 0, 0, 0);
        ///     white-space: nowrap;
        ///     border-width: 0;
        /// }
        /// ```
        SrOnly => "sr-only",
        /// ```css
        /// {
        ///     position: static;
        ///     width: auto;
        ///     height: auto;
        ///     padding: 0;
        ///     margin: 0;
        ///     overflow: visible;
        ///     clip: auto;
        ///     white-space: normal;
        /// }
        /// ```
        NotSrOnly => "not-sr-only",
    }
    /// Utilities for opting in and out of forced colors.
    ///
    /// <https://tailwindcss.com/docs/forced-color-adjust>
    ForcedColorAdjust {
        /// ```css
        /// {
        ///     forced-color-adjust: auto;
        /// }
        /// ```
        Auto => "forced-color-adjust-auto",
        /// ```css
        /// {
        ///     forced-color-adjust: none;
        /// }
        /// ```
        None => "forced-color-adjust-none",
    }
);
