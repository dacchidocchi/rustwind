def_types!(
    /// Utilities for controlling the border radius of an element.
    ///
    /// <https://tailwindcss.com/docs/border-radius>
    BorderRadius {
        /// ```css
        /// {
        ///     border-radius: 0px;
        /// }
        /// ```
        None => "rounded-none",
        /// ```css
        /// {
        ///     border-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        Sm => "rounded-sm",
        /// ```css
        /// {
        ///     border-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Rounded => "rounded",
        /// ```css
        /// {
        ///     border-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        Md => "rounded-md",
        /// ```css
        /// {
        ///     border-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        Lg => "rounded-lg",
        /// ```css
        /// {
        ///     border-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        Xl => "rounded-xl",
        /// ```css
        /// {
        ///     border-radius: 1rem; /* 16px */
        /// }
        /// ```
        _2xl => "rounded-2xl",
        /// ```css
        /// {
        ///     border-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        _3xl => "rounded-3xl",
        /// ```css
        /// {
        ///     border-radius: 9999px;
        /// }
        /// ```
        Full => "rounded-full",
        /// ```css
        /// {
        ///     border-start-start-radius: 0px;
        ///     border-end-start-radius: 0px;
        /// }
        /// ```
        SNone => "rounded-s-none",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.125rem; /* 2px */
        ///     border-end-start-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        SSm => "rounded-s-sm",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.25rem; /* 4px */
        ///     border-end-start-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        S => "rounded-s",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.375rem; /* 6px */
        ///     border-end-start-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        SMd => "rounded-s-md",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.5rem; /* 8px */
        ///     border-end-start-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        SLg => "rounded-s-lg",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.75rem; /* 12px */
        ///     border-end-start-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        SXl => "rounded-s-xl",
        /// ```css
        /// {
        ///     border-start-start-radius: 1rem; /* 16px */
        ///     border-end-start-radius: 1rem; /* 16px */
        /// }
        /// ```
        S2xl => "rounded-s-2xl",
        /// ```css
        /// {
        ///     border-start-start-radius: 1.5rem; /* 24px */
        ///     border-end-start-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        S3xl => "rounded-s-3xl",
        /// ```css
        /// {
        ///     border-start-start-radius: 9999px;
        ///     border-end-start-radius: 9999px;
        /// }
        /// ```
        SFull => "rounded-s-full",
        /// ```css
        /// {
        ///     border-start-end-radius: 0px;
        ///     border-end-end-radius: 0px;
        /// }
        /// ```
        ENone => "rounded-e-none",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.125rem; /* 2px */
        ///     border-end-end-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        ESm => "rounded-e-sm",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.25rem; /* 4px */
        ///     border-end-end-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        E => "rounded-e",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.375rem; /* 6px */
        ///     border-end-end-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        EMd => "rounded-e-md",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.5rem; /* 8px */
        ///     border-end-end-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        ELg => "rounded-e-lg",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.75rem; /* 12px */
        ///     border-end-end-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        EXl => "rounded-e-xl",
        /// ```css
        /// {
        ///     border-start-end-radius: 1rem; /* 16px */
        ///     border-end-end-radius: 1rem; /* 16px */
        /// }
        /// ```
        E2xl => "rounded-e-2xl",
        /// ```css
        /// {
        ///     border-start-end-radius: 1.5rem; /* 24px */
        ///     border-end-end-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        E3xl => "rounded-e-3xl",
        /// ```css
        /// {
        ///     border-start-end-radius: 9999px;
        ///     border-end-end-radius: 9999px;
        /// }
        /// ```
        EFull => "rounded-e-full",
        /// ```css
        /// {
        ///     border-top-left-radius: 0px;
        ///     border-top-right-radius: 0px;
        /// }
        /// ```
        TNone => "rounded-t-none",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.125rem; /* 2px */
        ///     border-top-right-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        TSm => "rounded-t-sm",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.25rem; /* 4px */
        ///     border-top-right-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        T => "rounded-t",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.375rem; /* 6px */
        ///     border-top-right-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        TMd => "rounded-t-md",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.5rem; /* 8px */
        ///     border-top-right-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        TLg => "rounded-t-lg",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.75rem; /* 12px */
        ///     border-top-right-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        TXl => "rounded-t-xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 1rem; /* 16px */
        ///     border-top-right-radius: 1rem; /* 16px */
        /// }
        /// ```
        T2xl => "rounded-t-2xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 1.5rem; /* 24px */
        ///     border-top-right-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        T3xl => "rounded-t-3xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 9999px;
        ///     border-top-right-radius: 9999px;
        /// }
        /// ```
        TFull => "rounded-t-full",
        /// ```css
        /// {
        ///     border-top-right-radius: 0px;
        ///     border-bottom-right-radius: 0px;
        /// }
        /// ```
        RNone => "rounded-r-none",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.125rem; /* 2px */
        ///     border-bottom-right-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        RSm => "rounded-r-sm",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.25rem; /* 4px */
        ///     border-bottom-right-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        R => "rounded-r",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.375rem; /* 6px */
        ///     border-bottom-right-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        RMd => "rounded-r-md",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.5rem; /* 8px */
        ///     border-bottom-right-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        RLg => "rounded-r-lg",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.75rem; /* 12px */
        ///     border-bottom-right-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        RXl => "rounded-r-xl",
        /// ```css
        /// {
        ///     border-top-right-radius: 1rem; /* 16px */
        ///     border-bottom-right-radius: 1rem; /* 16px */
        /// }
        /// ```
        R2xl => "rounded-r-2xl",
        /// ```css
        /// {
        ///     border-top-right-radius: 1.5rem; /* 24px */
        ///     border-bottom-right-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        R3xl => "rounded-r-3xl",
        /// ```css
        /// {
        ///     border-top-right-radius: 9999px;
        ///     border-bottom-right-radius: 9999px;
        /// }
        /// ```
        RFull => "rounded-r-full",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0px;
        ///     border-bottom-left-radius: 0px;
        /// }
        /// ```
        BNone => "rounded-b-none",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.125rem; /* 2px */
        ///     border-bottom-left-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        BSm => "rounded-b-sm",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.25rem; /* 4px */
        ///     border-bottom-left-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        B => "rounded-b",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.375rem; /* 6px */
        ///     border-bottom-left-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        BMd => "rounded-b-md",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.5rem; /* 8px */
        ///     border-bottom-left-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        BLg => "rounded-b-lg",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.75rem; /* 12px */
        ///     border-bottom-left-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        BXl => "rounded-b-xl",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 1rem; /* 16px */
        ///     border-bottom-left-radius: 1rem; /* 16px */
        /// }
        /// ```
        B2xl => "rounded-b-2xl",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 1.5rem; /* 24px */
        ///     border-bottom-left-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        B3xl => "rounded-b-3xl",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 9999px;
        ///     border-bottom-left-radius: 9999px;
        /// }
        /// ```
        BFull => "rounded-b-full",
        /// ```css
        /// {
        ///     border-top-left-radius: 0px;
        ///     border-bottom-left-radius: 0px;
        /// }
        /// ```
        LNone => "rounded-l-none",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.125rem; /* 2px */
        ///     border-bottom-left-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        LSm => "rounded-l-sm",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.25rem; /* 4px */
        ///     border-bottom-left-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        L => "rounded-l",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.375rem; /* 6px */
        ///     border-bottom-left-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        LMd => "rounded-l-md",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.5rem; /* 8px */
        ///     border-bottom-left-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        LLg => "rounded-l-lg",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.75rem; /* 12px */
        ///     border-bottom-left-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        LXl => "rounded-l-xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 1rem; /* 16px */
        ///     border-bottom-left-radius: 1rem; /* 16px */
        /// }
        /// ```
        L2xl => "rounded-l-2xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 1.5rem; /* 24px */
        ///     border-bottom-left-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        L3xl => "rounded-l-3xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 9999px;
        ///     border-bottom-left-radius: 9999px;
        /// }
        /// ```
        LFull => "rounded-l-full",
        /// ```css
        /// {
        ///     border-start-start-radius: 0px;
        /// }
        /// ```
        SsNone => "rounded-ss-none",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        SsSm => "rounded-ss-sm",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Ss => "rounded-ss",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        SsMd => "rounded-ss-md",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        SsLg => "rounded-ss-lg",
        /// ```css
        /// {
        ///     border-start-start-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        SsXl => "rounded-ss-xl",
        /// ```css
        /// {
        ///     border-start-start-radius: 1rem; /* 16px */
        /// }
        /// ```
        Ss2xl => "rounded-ss-2xl",
        /// ```css
        /// {
        ///     border-start-start-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        Ss3xl => "rounded-ss-3xl",
        /// ```css
        /// {
        ///     border-start-start-radius: 9999px;
        /// }
        /// ```
        SsFull => "rounded-ss-full",
        /// ```css
        /// {
        ///     border-start-end-radius: 0px;
        /// }
        /// ```
        SeNone => "rounded-se-none",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        SeSm => "rounded-se-sm",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Se => "rounded-se",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        SeMd => "rounded-se-md",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        SeLg => "rounded-se-lg",
        /// ```css
        /// {
        ///     border-start-end-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        SeXl => "rounded-se-xl",
        /// ```css
        /// {
        ///     border-start-end-radius: 1rem; /* 16px */
        /// }
        /// ```
        Se2xl => "rounded-se-2xl",
        /// ```css
        /// {
        ///     border-start-end-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        Se3xl => "rounded-se-3xl",
        /// ```css
        /// {
        ///     border-start-end-radius: 9999px;
        /// }
        /// ```
        SeFull => "rounded-se-full",
        /// ```css
        /// {
        ///     border-end-end-radius: 0px;
        /// }
        /// ```
        EeNone => "rounded-ee-none",
        /// ```css
        /// {
        ///     border-end-end-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        EeSm => "rounded-ee-sm",
        /// ```css
        /// {
        ///     border-end-end-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Ee => "rounded-ee",
        /// ```css
        /// {
        ///     border-end-end-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        EeMd => "rounded-ee-md",
        /// ```css
        /// {
        ///     border-end-end-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        EeLg => "rounded-ee-lg",
        /// ```css
        /// {
        ///     border-end-end-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        EeXl => "rounded-ee-xl",
        /// ```css
        /// {
        ///     border-end-end-radius: 1rem; /* 16px */
        /// }
        /// ```
        Ee2xl => "rounded-ee-2xl",
        /// ```css
        /// {
        ///     border-end-end-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        Ee3xl => "rounded-ee-3xl",
        /// ```css
        /// {
        ///     border-end-end-radius: 9999px;
        /// }
        /// ```
        EeFull => "rounded-ee-full",
        /// ```css
        /// {
        ///     border-end-start-radius: 0px;
        /// }
        /// ```
        EsNone => "rounded-es-none",
        /// ```css
        /// {
        ///     border-end-start-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        EsSm => "rounded-es-sm",
        /// ```css
        /// {
        ///     border-end-start-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Es => "rounded-es",
        /// ```css
        /// {
        ///     border-end-start-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        EsMd => "rounded-es-md",
        /// ```css
        /// {
        ///     border-end-start-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        EsLg => "rounded-es-lg",
        /// ```css
        /// {
        ///     border-end-start-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        EsXl => "rounded-es-xl",
        /// ```css
        /// {
        ///     border-end-start-radius: 1rem; /* 16px */
        /// }
        /// ```
        Es2xl => "rounded-es-2xl",
        /// ```css
        /// {
        ///     border-end-start-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        Es3xl => "rounded-es-3xl",
        /// ```css
        /// {
        ///     border-end-start-radius: 9999px;
        /// }
        /// ```
        EsFull => "rounded-es-full",
        /// ```css
        /// {
        ///     border-top-left-radius: 0px;
        /// }
        /// ```
        TlNone => "rounded-tl-none",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        TlSm => "rounded-tl-sm",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Tl => "rounded-tl",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        TlMd => "rounded-tl-md",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        TlLg => "rounded-tl-lg",
        /// ```css
        /// {
        ///     border-top-left-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        TlXl => "rounded-tl-xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 1rem; /* 16px */
        /// }
        /// ```
        Tl2xl => "rounded-tl-2xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        Tl3xl => "rounded-tl-3xl",
        /// ```css
        /// {
        ///     border-top-left-radius: 9999px;
        /// }
        /// ```
        TlFull => "rounded-tl-full",
        /// ```css
        /// {
        ///     border-top-right-radius: 0px;
        /// }
        /// ```
        TrNone => "rounded-tr-none",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        TrSm => "rounded-tr-sm",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Tr => "rounded-tr",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        TrMd => "rounded-tr-md",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        TrLg => "rounded-tr-lg",
        /// ```css
        /// {
        ///     border-top-right-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        TrXl => "rounded-tr-xl",
        /// ```css
        /// {
        ///     border-top-right-radius: 1rem; /* 16px */
        /// }
        /// ```
        Tr2xl => "rounded-tr-2xl",
        /// ```css
        /// {
        ///     border-top-right-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        Tr3xl => "rounded-tr-3xl",
        /// ```css
        /// {
        ///     border-top-right-radius: 9999px;
        /// }
        /// ```
        TrFull => "rounded-tr-full",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0px;
        /// }
        /// ```
        BrNone => "rounded-br-none",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        BrSm => "rounded-br-sm",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Br => "rounded-br",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        BrMd => "rounded-br-md",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        BrLg => "rounded-br-lg",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        BrXl => "rounded-br-xl",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 1rem; /* 16px */
        /// }
        /// ```
        Br2xl => "rounded-br-2xl",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        Br3xl => "rounded-br-3xl",
        /// ```css
        /// {
        ///     border-bottom-right-radius: 9999px;
        /// }
        /// ```
        BrFull => "rounded-br-full",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 0px;
        /// }
        /// ```
        BlNone => "rounded-bl-none",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 0.125rem; /* 2px */
        /// }
        /// ```
        BlSm => "rounded-bl-sm",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 0.25rem; /* 4px */
        /// }
        /// ```
        Bl => "rounded-bl",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 0.375rem; /* 6px */
        /// }
        /// ```
        BlMd => "rounded-bl-md",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 0.5rem; /* 8px */
        /// }
        /// ```
        BlLg => "rounded-bl-lg",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 0.75rem; /* 12px */
        /// }
        /// ```
        BlXl => "rounded-bl-xl",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 1rem; /* 16px */
        /// }
        /// ```
        Bl2xl => "rounded-bl-2xl",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 1.5rem; /* 24px */
        /// }
        /// ```
        Bl3xl => "rounded-bl-3xl",
        /// ```css
        /// {
        ///     border-bottom-left-radius: 9999px;
        /// }
        /// ```
        BlFull => "rounded-bl-full",
    }
    /// Utilities for controlling the width of an element's borders.
    ///
    /// <https://tailwindcss.com/docs/border-width>
    BorderWidth {
        /// ```css
        /// {
        ///     border-width: 0px;
        /// }
        /// ```
        _0 => "border-0",
        /// ```css
        /// {
        ///     border-width: 2px;
        /// }
        /// ```
        _2 => "border-2",
        /// ```css
        /// {
        ///     border-width: 4px;
        /// }
        /// ```
        _4 => "border-4",
        /// ```css
        /// {
        ///     border-width: 8px;
        /// }
        /// ```
        _8 => "border-8",
        /// ```css
        /// {
        ///     border-width: 1px;
        /// }
        /// ```
        Border => "border",
        /// ```css
        /// {
        ///     border-left-width: 0px;
        ///     border-right-width: 0px;
        /// }
        /// ```
        X0 => "border-x-0",
        /// ```css
        /// {
        ///     border-left-width: 2px;
        ///     border-right-width: 2px;
        /// }
        /// ```
        X2 => "border-x-2",
        /// ```css
        /// {
        ///     border-left-width: 4px;
        ///     border-right-width: 4px;
        /// }
        /// ```
        X4 => "border-x-4",
        /// ```css
        /// {
        ///     border-left-width: 8px;
        ///     border-right-width: 8px;
        /// }
        /// ```
        X8 => "border-x-8",
        /// ```css
        /// {
        ///     border-left-width: 1px;
        ///     border-right-width: 1px;
        /// }
        /// ```
        X => "border-x",
        /// ```css
        /// {
        ///     border-top-width: 0px;
        ///     border-bottom-width: 0px;
        /// }
        /// ```
        Y0 => "border-y-0",
        /// ```css
        /// {
        ///     border-top-width: 2px;
        ///     border-bottom-width: 2px;
        /// }
        /// ```
        Y2 => "border-y-2",
        /// ```css
        /// {
        ///     border-top-width: 4px;
        ///     border-bottom-width: 4px;
        /// }
        /// ```
        Y4 => "border-y-4",
        /// ```css
        /// {
        ///     border-top-width: 8px;
        ///     border-bottom-width: 8px;
        /// }
        /// ```
        Y8 => "border-y-8",
        /// ```css
        /// {
        ///     border-top-width: 1px;
        ///     border-bottom-width: 1px;
        /// }
        /// ```
        Y => "border-y",
        /// ```css
        /// {
        ///     border-inline-start-width: 0px;
        /// }
        /// ```
        S0 => "border-s-0",
        /// ```css
        /// {
        ///     border-inline-start-width: 2px;
        /// }
        /// ```
        S2 => "border-s-2",
        /// ```css
        /// {
        ///     border-inline-start-width: 4px;
        /// }
        /// ```
        S4 => "border-s-4",
        /// ```css
        /// {
        ///     border-inline-start-width: 8px;
        /// }
        /// ```
        S8 => "border-s-8",
        /// ```css
        /// {
        ///     border-inline-start-width: 1px;
        /// }
        /// ```
        S => "border-s",
        /// ```css
        /// {
        ///     border-inline-end-width: 0px;
        /// }
        /// ```
        E0 => "border-e-0",
        /// ```css
        /// {
        ///     border-inline-end-width: 2px;
        /// }
        /// ```
        E2 => "border-e-2",
        /// ```css
        /// {
        ///     border-inline-end-width: 4px;
        /// }
        /// ```
        E4 => "border-e-4",
        /// ```css
        /// {
        ///     border-inline-end-width: 8px;
        /// }
        /// ```
        E8 => "border-e-8",
        /// ```css
        /// {
        ///     border-inline-end-width: 1px;
        /// }
        /// ```
        E => "border-e",
        /// ```css
        /// {
        ///     border-top-width: 0px;
        /// }
        /// ```
        T0 => "border-t-0",
        /// ```css
        /// {
        ///     border-top-width: 2px;
        /// }
        /// ```
        T2 => "border-t-2",
        /// ```css
        /// {
        ///     border-top-width: 4px;
        /// }
        /// ```
        T4 => "border-t-4",
        /// ```css
        /// {
        ///     border-top-width: 8px;
        /// }
        /// ```
        T8 => "border-t-8",
        /// ```css
        /// {
        ///     border-top-width: 1px;
        /// }
        /// ```
        T => "border-t",
        /// ```css
        /// {
        ///     border-right-width: 0px;
        /// }
        /// ```
        R0 => "border-r-0",
        /// ```css
        /// {
        ///     border-right-width: 2px;
        /// }
        /// ```
        R2 => "border-r-2",
        /// ```css
        /// {
        ///     border-right-width: 4px;
        /// }
        /// ```
        R4 => "border-r-4",
        /// ```css
        /// {
        ///     border-right-width: 8px;
        /// }
        /// ```
        R8 => "border-r-8",
        /// ```css
        /// {
        ///     border-right-width: 1px;
        /// }
        /// ```
        R => "border-r",
        /// ```css
        /// {
        ///     border-bottom-width: 0px;
        /// }
        /// ```
        B0 => "border-b-0",
        /// ```css
        /// {
        ///     border-bottom-width: 2px;
        /// }
        /// ```
        B2 => "border-b-2",
        /// ```css
        /// {
        ///     border-bottom-width: 4px;
        /// }
        /// ```
        B4 => "border-b-4",
        /// ```css
        /// {
        ///     border-bottom-width: 8px;
        /// }
        /// ```
        B8 => "border-b-8",
        /// ```css
        /// {
        ///     border-bottom-width: 1px;
        /// }
        /// ```
        B => "border-b",
        /// ```css
        /// {
        ///     border-left-width: 0px;
        /// }
        /// ```
        L0 => "border-l-0",
        /// ```css
        /// {
        ///     border-left-width: 2px;
        /// }
        /// ```
        L2 => "border-l-2",
        /// ```css
        /// {
        ///     border-left-width: 4px;
        /// }
        /// ```
        L4 => "border-l-4",
        /// ```css
        /// {
        ///     border-left-width: 8px;
        /// }
        /// ```
        L8 => "border-l-8",
        /// ```css
        /// {
        ///     border-left-width: 1px;
        /// }
        /// ```
        L => "border-l",
    }
    /// Utilities for controlling the color of an element's borders.
    ///
    /// <https://tailwindcss.com/docs/border-color>
    BorderColor {
        /// ```css
        /// {
        ///     border-color: inherit;
        /// }
        /// ```
        Inherit => "border-inherit",
        /// ```css
        /// {
        ///     border-color: currentColor;
        /// }
        /// ```
        Current => "border-current",
        /// ```css
        /// {
        ///     border-color: transparent;
        /// }
        /// ```
        Transparent => "border-transparent",
        /// ```css
        /// {
        ///     border-color: rgb(0 0 0);
        /// }
        /// ```
        Black => "border-black",
        /// ```css
        /// {
        ///     border-color: rgb(255 255 255);
        /// }
        /// ```
        White => "border-white",
        /// ```css
        /// {
        ///     border-color: rgb(248 250 252);
        /// }
        /// ```
        Slate50 => "border-slate-50",
        /// ```css
        /// {
        ///     border-color: rgb(241 245 249);
        /// }
        /// ```
        Slate100 => "border-slate-100",
        /// ```css
        /// {
        ///     border-color: rgb(226 232 240);
        /// }
        /// ```
        Slate200 => "border-slate-200",
        /// ```css
        /// {
        ///     border-color: rgb(203 213 225);
        /// }
        /// ```
        Slate300 => "border-slate-300",
        /// ```css
        /// {
        ///     border-color: rgb(148 163 184);
        /// }
        /// ```
        Slate400 => "border-slate-400",
        /// ```css
        /// {
        ///     border-color: rgb(100 116 139);
        /// }
        /// ```
        Slate500 => "border-slate-500",
        /// ```css
        /// {
        ///     border-color: rgb(71 85 105);
        /// }
        /// ```
        Slate600 => "border-slate-600",
        /// ```css
        /// {
        ///     border-color: rgb(51 65 85);
        /// }
        /// ```
        Slate700 => "border-slate-700",
        /// ```css
        /// {
        ///     border-color: rgb(30 41 59);
        /// }
        /// ```
        Slate800 => "border-slate-800",
        /// ```css
        /// {
        ///     border-color: rgb(15 23 42);
        /// }
        /// ```
        Slate900 => "border-slate-900",
        /// ```css
        /// {
        ///     border-color: rgb(2 6 23);
        /// }
        /// ```
        Slate950 => "border-slate-950",
        /// ```css
        /// {
        ///     border-color: rgb(249 250 251);
        /// }
        /// ```
        Gray50 => "border-gray-50",
        /// ```css
        /// {
        ///     border-color: rgb(243 244 246);
        /// }
        /// ```
        Gray100 => "border-gray-100",
        /// ```css
        /// {
        ///     border-color: rgb(229 231 235);
        /// }
        /// ```
        Gray200 => "border-gray-200",
        /// ```css
        /// {
        ///     border-color: rgb(209 213 219);
        /// }
        /// ```
        Gray300 => "border-gray-300",
        /// ```css
        /// {
        ///     border-color: rgb(156 163 175);
        /// }
        /// ```
        Gray400 => "border-gray-400",
        /// ```css
        /// {
        ///     border-color: rgb(107 114 128);
        /// }
        /// ```
        Gray500 => "border-gray-500",
        /// ```css
        /// {
        ///     border-color: rgb(75 85 99);
        /// }
        /// ```
        Gray600 => "border-gray-600",
        /// ```css
        /// {
        ///     border-color: rgb(55 65 81);
        /// }
        /// ```
        Gray700 => "border-gray-700",
        /// ```css
        /// {
        ///     border-color: rgb(31 41 55);
        /// }
        /// ```
        Gray800 => "border-gray-800",
        /// ```css
        /// {
        ///     border-color: rgb(17 24 39);
        /// }
        /// ```
        Gray900 => "border-gray-900",
        /// ```css
        /// {
        ///     border-color: rgb(3 7 18);
        /// }
        /// ```
        Gray950 => "border-gray-950",
        /// ```css
        /// {
        ///     border-color: rgb(250 250 250);
        /// }
        /// ```
        Zinc50 => "border-zinc-50",
        /// ```css
        /// {
        ///     border-color: rgb(244 244 245);
        /// }
        /// ```
        Zinc100 => "border-zinc-100",
        /// ```css
        /// {
        ///     border-color: rgb(228 228 231);
        /// }
        /// ```
        Zinc200 => "border-zinc-200",
        /// ```css
        /// {
        ///     border-color: rgb(212 212 216);
        /// }
        /// ```
        Zinc300 => "border-zinc-300",
        /// ```css
        /// {
        ///     border-color: rgb(161 161 170);
        /// }
        /// ```
        Zinc400 => "border-zinc-400",
        /// ```css
        /// {
        ///     border-color: rgb(113 113 122);
        /// }
        /// ```
        Zinc500 => "border-zinc-500",
        /// ```css
        /// {
        ///     border-color: rgb(82 82 91);
        /// }
        /// ```
        Zinc600 => "border-zinc-600",
        /// ```css
        /// {
        ///     border-color: rgb(63 63 70);
        /// }
        /// ```
        Zinc700 => "border-zinc-700",
        /// ```css
        /// {
        ///     border-color: rgb(39 39 42);
        /// }
        /// ```
        Zinc800 => "border-zinc-800",
        /// ```css
        /// {
        ///     border-color: rgb(24 24 27);
        /// }
        /// ```
        Zinc900 => "border-zinc-900",
        /// ```css
        /// {
        ///     border-color: rgb(9 9 11);
        /// }
        /// ```
        Zinc950 => "border-zinc-950",
        /// ```css
        /// {
        ///     border-color: rgb(250 250 250);
        /// }
        /// ```
        Neutral50 => "border-neutral-50",
        /// ```css
        /// {
        ///     border-color: rgb(245 245 245);
        /// }
        /// ```
        Neutral100 => "border-neutral-100",
        /// ```css
        /// {
        ///     border-color: rgb(229 229 229);
        /// }
        /// ```
        Neutral200 => "border-neutral-200",
        /// ```css
        /// {
        ///     border-color: rgb(212 212 212);
        /// }
        /// ```
        Neutral300 => "border-neutral-300",
        /// ```css
        /// {
        ///     border-color: rgb(163 163 163);
        /// }
        /// ```
        Neutral400 => "border-neutral-400",
        /// ```css
        /// {
        ///     border-color: rgb(115 115 115);
        /// }
        /// ```
        Neutral500 => "border-neutral-500",
        /// ```css
        /// {
        ///     border-color: rgb(82 82 82);
        /// }
        /// ```
        Neutral600 => "border-neutral-600",
        /// ```css
        /// {
        ///     border-color: rgb(64 64 64);
        /// }
        /// ```
        Neutral700 => "border-neutral-700",
        /// ```css
        /// {
        ///     border-color: rgb(38 38 38);
        /// }
        /// ```
        Neutral800 => "border-neutral-800",
        /// ```css
        /// {
        ///     border-color: rgb(23 23 23);
        /// }
        /// ```
        Neutral900 => "border-neutral-900",
        /// ```css
        /// {
        ///     border-color: rgb(10 10 10);
        /// }
        /// ```
        Neutral950 => "border-neutral-950",
        /// ```css
        /// {
        ///     border-color: rgb(250 250 249);
        /// }
        /// ```
        Stone50 => "border-stone-50",
        /// ```css
        /// {
        ///     border-color: rgb(245 245 244);
        /// }
        /// ```
        Stone100 => "border-stone-100",
        /// ```css
        /// {
        ///     border-color: rgb(231 229 228);
        /// }
        /// ```
        Stone200 => "border-stone-200",
        /// ```css
        /// {
        ///     border-color: rgb(214 211 209);
        /// }
        /// ```
        Stone300 => "border-stone-300",
        /// ```css
        /// {
        ///     border-color: rgb(168 162 158);
        /// }
        /// ```
        Stone400 => "border-stone-400",
        /// ```css
        /// {
        ///     border-color: rgb(120 113 108);
        /// }
        /// ```
        Stone500 => "border-stone-500",
        /// ```css
        /// {
        ///     border-color: rgb(87 83 78);
        /// }
        /// ```
        Stone600 => "border-stone-600",
        /// ```css
        /// {
        ///     border-color: rgb(68 64 60);
        /// }
        /// ```
        Stone700 => "border-stone-700",
        /// ```css
        /// {
        ///     border-color: rgb(41 37 36);
        /// }
        /// ```
        Stone800 => "border-stone-800",
        /// ```css
        /// {
        ///     border-color: rgb(28 25 23);
        /// }
        /// ```
        Stone900 => "border-stone-900",
        /// ```css
        /// {
        ///     border-color: rgb(12 10 9);
        /// }
        /// ```
        Stone950 => "border-stone-950",
        /// ```css
        /// {
        ///     border-color: rgb(254 242 242);
        /// }
        /// ```
        Red50 => "border-red-50",
        /// ```css
        /// {
        ///     border-color: rgb(254 226 226);
        /// }
        /// ```
        Red100 => "border-red-100",
        /// ```css
        /// {
        ///     border-color: rgb(254 202 202);
        /// }
        /// ```
        Red200 => "border-red-200",
        /// ```css
        /// {
        ///     border-color: rgb(252 165 165);
        /// }
        /// ```
        Red300 => "border-red-300",
        /// ```css
        /// {
        ///     border-color: rgb(248 113 113);
        /// }
        /// ```
        Red400 => "border-red-400",
        /// ```css
        /// {
        ///     border-color: rgb(239 68 68);
        /// }
        /// ```
        Red500 => "border-red-500",
        /// ```css
        /// {
        ///     border-color: rgb(220 38 38);
        /// }
        /// ```
        Red600 => "border-red-600",
        /// ```css
        /// {
        ///     border-color: rgb(185 28 28);
        /// }
        /// ```
        Red700 => "border-red-700",
        /// ```css
        /// {
        ///     border-color: rgb(153 27 27);
        /// }
        /// ```
        Red800 => "border-red-800",
        /// ```css
        /// {
        ///     border-color: rgb(127 29 29);
        /// }
        /// ```
        Red900 => "border-red-900",
        /// ```css
        /// {
        ///     border-color: rgb(69 10 10);
        /// }
        /// ```
        Red950 => "border-red-950",
        /// ```css
        /// {
        ///     border-color: rgb(255 247 237);
        /// }
        /// ```
        Orange50 => "border-orange-50",
        /// ```css
        /// {
        ///     border-color: rgb(255 237 213);
        /// }
        /// ```
        Orange100 => "border-orange-100",
        /// ```css
        /// {
        ///     border-color: rgb(254 215 170);
        /// }
        /// ```
        Orange200 => "border-orange-200",
        /// ```css
        /// {
        ///     border-color: rgb(253 186 116);
        /// }
        /// ```
        Orange300 => "border-orange-300",
        /// ```css
        /// {
        ///     border-color: rgb(251 146 60);
        /// }
        /// ```
        Orange400 => "border-orange-400",
        /// ```css
        /// {
        ///     border-color: rgb(249 115 22);
        /// }
        /// ```
        Orange500 => "border-orange-500",
        /// ```css
        /// {
        ///     border-color: rgb(234 88 12);
        /// }
        /// ```
        Orange600 => "border-orange-600",
        /// ```css
        /// {
        ///     border-color: rgb(194 65 12);
        /// }
        /// ```
        Orange700 => "border-orange-700",
        /// ```css
        /// {
        ///     border-color: rgb(154 52 18);
        /// }
        /// ```
        Orange800 => "border-orange-800",
        /// ```css
        /// {
        ///     border-color: rgb(124 45 18);
        /// }
        /// ```
        Orange900 => "border-orange-900",
        /// ```css
        /// {
        ///     border-color: rgb(67 20 7);
        /// }
        /// ```
        Orange950 => "border-orange-950",
        /// ```css
        /// {
        ///     border-color: rgb(255 251 235);
        /// }
        /// ```
        Amber50 => "border-amber-50",
        /// ```css
        /// {
        ///     border-color: rgb(254 243 199);
        /// }
        /// ```
        Amber100 => "border-amber-100",
        /// ```css
        /// {
        ///     border-color: rgb(253 230 138);
        /// }
        /// ```
        Amber200 => "border-amber-200",
        /// ```css
        /// {
        ///     border-color: rgb(252 211 77);
        /// }
        /// ```
        Amber300 => "border-amber-300",
        /// ```css
        /// {
        ///     border-color: rgb(251 191 36);
        /// }
        /// ```
        Amber400 => "border-amber-400",
        /// ```css
        /// {
        ///     border-color: rgb(245 158 11);
        /// }
        /// ```
        Amber500 => "border-amber-500",
        /// ```css
        /// {
        ///     border-color: rgb(217 119 6);
        /// }
        /// ```
        Amber600 => "border-amber-600",
        /// ```css
        /// {
        ///     border-color: rgb(180 83 9);
        /// }
        /// ```
        Amber700 => "border-amber-700",
        /// ```css
        /// {
        ///     border-color: rgb(146 64 14);
        /// }
        /// ```
        Amber800 => "border-amber-800",
        /// ```css
        /// {
        ///     border-color: rgb(120 53 15);
        /// }
        /// ```
        Amber900 => "border-amber-900",
        /// ```css
        /// {
        ///     border-color: rgb(69 26 3);
        /// }
        /// ```
        Amber950 => "border-amber-950",
        /// ```css
        /// {
        ///     border-color: rgb(254 252 232);
        /// }
        /// ```
        Yellow50 => "border-yellow-50",
        /// ```css
        /// {
        ///     border-color: rgb(254 249 195);
        /// }
        /// ```
        Yellow100 => "border-yellow-100",
        /// ```css
        /// {
        ///     border-color: rgb(254 240 138);
        /// }
        /// ```
        Yellow200 => "border-yellow-200",
        /// ```css
        /// {
        ///     border-color: rgb(253 224 71);
        /// }
        /// ```
        Yellow300 => "border-yellow-300",
        /// ```css
        /// {
        ///     border-color: rgb(250 204 21);
        /// }
        /// ```
        Yellow400 => "border-yellow-400",
        /// ```css
        /// {
        ///     border-color: rgb(234 179 8);
        /// }
        /// ```
        Yellow500 => "border-yellow-500",
        /// ```css
        /// {
        ///     border-color: rgb(202 138 4);
        /// }
        /// ```
        Yellow600 => "border-yellow-600",
        /// ```css
        /// {
        ///     border-color: rgb(161 98 7);
        /// }
        /// ```
        Yellow700 => "border-yellow-700",
        /// ```css
        /// {
        ///     border-color: rgb(133 77 14);
        /// }
        /// ```
        Yellow800 => "border-yellow-800",
        /// ```css
        /// {
        ///     border-color: rgb(113 63 18);
        /// }
        /// ```
        Yellow900 => "border-yellow-900",
        /// ```css
        /// {
        ///     border-color: rgb(66 32 6);
        /// }
        /// ```
        Yellow950 => "border-yellow-950",
        /// ```css
        /// {
        ///     border-color: rgb(247 254 231);
        /// }
        /// ```
        Lime50 => "border-lime-50",
        /// ```css
        /// {
        ///     border-color: rgb(236 252 203);
        /// }
        /// ```
        Lime100 => "border-lime-100",
        /// ```css
        /// {
        ///     border-color: rgb(217 249 157);
        /// }
        /// ```
        Lime200 => "border-lime-200",
        /// ```css
        /// {
        ///     border-color: rgb(190 242 100);
        /// }
        /// ```
        Lime300 => "border-lime-300",
        /// ```css
        /// {
        ///     border-color: rgb(163 230 53);
        /// }
        /// ```
        Lime400 => "border-lime-400",
        /// ```css
        /// {
        ///     border-color: rgb(132 204 22);
        /// }
        /// ```
        Lime500 => "border-lime-500",
        /// ```css
        /// {
        ///     border-color: rgb(101 163 13);
        /// }
        /// ```
        Lime600 => "border-lime-600",
        /// ```css
        /// {
        ///     border-color: rgb(77 124 15);
        /// }
        /// ```
        Lime700 => "border-lime-700",
        /// ```css
        /// {
        ///     border-color: rgb(63 98 18);
        /// }
        /// ```
        Lime800 => "border-lime-800",
        /// ```css
        /// {
        ///     border-color: rgb(54 83 20);
        /// }
        /// ```
        Lime900 => "border-lime-900",
        /// ```css
        /// {
        ///     border-color: rgb(26 46 5);
        /// }
        /// ```
        Lime950 => "border-lime-950",
        /// ```css
        /// {
        ///     border-color: rgb(240 253 244);
        /// }
        /// ```
        Green50 => "border-green-50",
        /// ```css
        /// {
        ///     border-color: rgb(220 252 231);
        /// }
        /// ```
        Green100 => "border-green-100",
        /// ```css
        /// {
        ///     border-color: rgb(187 247 208);
        /// }
        /// ```
        Green200 => "border-green-200",
        /// ```css
        /// {
        ///     border-color: rgb(134 239 172);
        /// }
        /// ```
        Green300 => "border-green-300",
        /// ```css
        /// {
        ///     border-color: rgb(74 222 128);
        /// }
        /// ```
        Green400 => "border-green-400",
        /// ```css
        /// {
        ///     border-color: rgb(34 197 94);
        /// }
        /// ```
        Green500 => "border-green-500",
        /// ```css
        /// {
        ///     border-color: rgb(22 163 74);
        /// }
        /// ```
        Green600 => "border-green-600",
        /// ```css
        /// {
        ///     border-color: rgb(21 128 61);
        /// }
        /// ```
        Green700 => "border-green-700",
        /// ```css
        /// {
        ///     border-color: rgb(22 101 52);
        /// }
        /// ```
        Green800 => "border-green-800",
        /// ```css
        /// {
        ///     border-color: rgb(20 83 45);
        /// }
        /// ```
        Green900 => "border-green-900",
        /// ```css
        /// {
        ///     border-color: rgb(5 46 22);
        /// }
        /// ```
        Green950 => "border-green-950",
        /// ```css
        /// {
        ///     border-color: rgb(236 253 245);
        /// }
        /// ```
        Emerald50 => "border-emerald-50",
        /// ```css
        /// {
        ///     border-color: rgb(209 250 229);
        /// }
        /// ```
        Emerald100 => "border-emerald-100",
        /// ```css
        /// {
        ///     border-color: rgb(167 243 208);
        /// }
        /// ```
        Emerald200 => "border-emerald-200",
        /// ```css
        /// {
        ///     border-color: rgb(110 231 183);
        /// }
        /// ```
        Emerald300 => "border-emerald-300",
        /// ```css
        /// {
        ///     border-color: rgb(52 211 153);
        /// }
        /// ```
        Emerald400 => "border-emerald-400",
        /// ```css
        /// {
        ///     border-color: rgb(16 185 129);
        /// }
        /// ```
        Emerald500 => "border-emerald-500",
        /// ```css
        /// {
        ///     border-color: rgb(5 150 105);
        /// }
        /// ```
        Emerald600 => "border-emerald-600",
        /// ```css
        /// {
        ///     border-color: rgb(4 120 87);
        /// }
        /// ```
        Emerald700 => "border-emerald-700",
        /// ```css
        /// {
        ///     border-color: rgb(6 95 70);
        /// }
        /// ```
        Emerald800 => "border-emerald-800",
        /// ```css
        /// {
        ///     border-color: rgb(6 78 59);
        /// }
        /// ```
        Emerald900 => "border-emerald-900",
        /// ```css
        /// {
        ///     border-color: rgb(2 44 34);
        /// }
        /// ```
        Emerald950 => "border-emerald-950",
        /// ```css
        /// {
        ///     border-color: rgb(240 253 250);
        /// }
        /// ```
        Teal50 => "border-teal-50",
        /// ```css
        /// {
        ///     border-color: rgb(204 251 241);
        /// }
        /// ```
        Teal100 => "border-teal-100",
        /// ```css
        /// {
        ///     border-color: rgb(153 246 228);
        /// }
        /// ```
        Teal200 => "border-teal-200",
        /// ```css
        /// {
        ///     border-color: rgb(94 234 212);
        /// }
        /// ```
        Teal300 => "border-teal-300",
        /// ```css
        /// {
        ///     border-color: rgb(45 212 191);
        /// }
        /// ```
        Teal400 => "border-teal-400",
        /// ```css
        /// {
        ///     border-color: rgb(20 184 166);
        /// }
        /// ```
        Teal500 => "border-teal-500",
        /// ```css
        /// {
        ///     border-color: rgb(13 148 136);
        /// }
        /// ```
        Teal600 => "border-teal-600",
        /// ```css
        /// {
        ///     border-color: rgb(15 118 110);
        /// }
        /// ```
        Teal700 => "border-teal-700",
        /// ```css
        /// {
        ///     border-color: rgb(17 94 89);
        /// }
        /// ```
        Teal800 => "border-teal-800",
        /// ```css
        /// {
        ///     border-color: rgb(19 78 74);
        /// }
        /// ```
        Teal900 => "border-teal-900",
        /// ```css
        /// {
        ///     border-color: rgb(4 47 46);
        /// }
        /// ```
        Teal950 => "border-teal-950",
        /// ```css
        /// {
        ///     border-color: rgb(236 254 255);
        /// }
        /// ```
        Cyan50 => "border-cyan-50",
        /// ```css
        /// {
        ///     border-color: rgb(207 250 254);
        /// }
        /// ```
        Cyan100 => "border-cyan-100",
        /// ```css
        /// {
        ///     border-color: rgb(165 243 252);
        /// }
        /// ```
        Cyan200 => "border-cyan-200",
        /// ```css
        /// {
        ///     border-color: rgb(103 232 249);
        /// }
        /// ```
        Cyan300 => "border-cyan-300",
        /// ```css
        /// {
        ///     border-color: rgb(34 211 238);
        /// }
        /// ```
        Cyan400 => "border-cyan-400",
        /// ```css
        /// {
        ///     border-color: rgb(6 182 212);
        /// }
        /// ```
        Cyan500 => "border-cyan-500",
        /// ```css
        /// {
        ///     border-color: rgb(8 145 178);
        /// }
        /// ```
        Cyan600 => "border-cyan-600",
        /// ```css
        /// {
        ///     border-color: rgb(14 116 144);
        /// }
        /// ```
        Cyan700 => "border-cyan-700",
        /// ```css
        /// {
        ///     border-color: rgb(21 94 117);
        /// }
        /// ```
        Cyan800 => "border-cyan-800",
        /// ```css
        /// {
        ///     border-color: rgb(22 78 99);
        /// }
        /// ```
        Cyan900 => "border-cyan-900",
        /// ```css
        /// {
        ///     border-color: rgb(8 51 68);
        /// }
        /// ```
        Cyan950 => "border-cyan-950",
        /// ```css
        /// {
        ///     border-color: rgb(240 249 255);
        /// }
        /// ```
        Sky50 => "border-sky-50",
        /// ```css
        /// {
        ///     border-color: rgb(224 242 254);
        /// }
        /// ```
        Sky100 => "border-sky-100",
        /// ```css
        /// {
        ///     border-color: rgb(186 230 253);
        /// }
        /// ```
        Sky200 => "border-sky-200",
        /// ```css
        /// {
        ///     border-color: rgb(125 211 252);
        /// }
        /// ```
        Sky300 => "border-sky-300",
        /// ```css
        /// {
        ///     border-color: rgb(56 189 248);
        /// }
        /// ```
        Sky400 => "border-sky-400",
        /// ```css
        /// {
        ///     border-color: rgb(14 165 233);
        /// }
        /// ```
        Sky500 => "border-sky-500",
        /// ```css
        /// {
        ///     border-color: rgb(2 132 199);
        /// }
        /// ```
        Sky600 => "border-sky-600",
        /// ```css
        /// {
        ///     border-color: rgb(3 105 161);
        /// }
        /// ```
        Sky700 => "border-sky-700",
        /// ```css
        /// {
        ///     border-color: rgb(7 89 133);
        /// }
        /// ```
        Sky800 => "border-sky-800",
        /// ```css
        /// {
        ///     border-color: rgb(12 74 110);
        /// }
        /// ```
        Sky900 => "border-sky-900",
        /// ```css
        /// {
        ///     border-color: rgb(8 47 73);
        /// }
        /// ```
        Sky950 => "border-sky-950",
        /// ```css
        /// {
        ///     border-color: rgb(239 246 255);
        /// }
        /// ```
        Blue50 => "border-blue-50",
        /// ```css
        /// {
        ///     border-color: rgb(219 234 254);
        /// }
        /// ```
        Blue100 => "border-blue-100",
        /// ```css
        /// {
        ///     border-color: rgb(191 219 254);
        /// }
        /// ```
        Blue200 => "border-blue-200",
        /// ```css
        /// {
        ///     border-color: rgb(147 197 253);
        /// }
        /// ```
        Blue300 => "border-blue-300",
        /// ```css
        /// {
        ///     border-color: rgb(96 165 250);
        /// }
        /// ```
        Blue400 => "border-blue-400",
        /// ```css
        /// {
        ///     border-color: rgb(59 130 246);
        /// }
        /// ```
        Blue500 => "border-blue-500",
        /// ```css
        /// {
        ///     border-color: rgb(37 99 235);
        /// }
        /// ```
        Blue600 => "border-blue-600",
        /// ```css
        /// {
        ///     border-color: rgb(29 78 216);
        /// }
        /// ```
        Blue700 => "border-blue-700",
        /// ```css
        /// {
        ///     border-color: rgb(30 64 175);
        /// }
        /// ```
        Blue800 => "border-blue-800",
        /// ```css
        /// {
        ///     border-color: rgb(30 58 138);
        /// }
        /// ```
        Blue900 => "border-blue-900",
        /// ```css
        /// {
        ///     border-color: rgb(23 37 84);
        /// }
        /// ```
        Blue950 => "border-blue-950",
        /// ```css
        /// {
        ///     border-color: rgb(238 242 255);
        /// }
        /// ```
        Indigo50 => "border-indigo-50",
        /// ```css
        /// {
        ///     border-color: rgb(224 231 255);
        /// }
        /// ```
        Indigo100 => "border-indigo-100",
        /// ```css
        /// {
        ///     border-color: rgb(199 210 254);
        /// }
        /// ```
        Indigo200 => "border-indigo-200",
        /// ```css
        /// {
        ///     border-color: rgb(165 180 252);
        /// }
        /// ```
        Indigo300 => "border-indigo-300",
        /// ```css
        /// {
        ///     border-color: rgb(129 140 248);
        /// }
        /// ```
        Indigo400 => "border-indigo-400",
        /// ```css
        /// {
        ///     border-color: rgb(99 102 241);
        /// }
        /// ```
        Indigo500 => "border-indigo-500",
        /// ```css
        /// {
        ///     border-color: rgb(79 70 229);
        /// }
        /// ```
        Indigo600 => "border-indigo-600",
        /// ```css
        /// {
        ///     border-color: rgb(67 56 202);
        /// }
        /// ```
        Indigo700 => "border-indigo-700",
        /// ```css
        /// {
        ///     border-color: rgb(55 48 163);
        /// }
        /// ```
        Indigo800 => "border-indigo-800",
        /// ```css
        /// {
        ///     border-color: rgb(49 46 129);
        /// }
        /// ```
        Indigo900 => "border-indigo-900",
        /// ```css
        /// {
        ///     border-color: rgb(30 27 75);
        /// }
        /// ```
        Indigo950 => "border-indigo-950",
        /// ```css
        /// {
        ///     border-color: rgb(245 243 255);
        /// }
        /// ```
        Violet50 => "border-violet-50",
        /// ```css
        /// {
        ///     border-color: rgb(237 233 254);
        /// }
        /// ```
        Violet100 => "border-violet-100",
        /// ```css
        /// {
        ///     border-color: rgb(221 214 254);
        /// }
        /// ```
        Violet200 => "border-violet-200",
        /// ```css
        /// {
        ///     border-color: rgb(196 181 253);
        /// }
        /// ```
        Violet300 => "border-violet-300",
        /// ```css
        /// {
        ///     border-color: rgb(167 139 250);
        /// }
        /// ```
        Violet400 => "border-violet-400",
        /// ```css
        /// {
        ///     border-color: rgb(139 92 246);
        /// }
        /// ```
        Violet500 => "border-violet-500",
        /// ```css
        /// {
        ///     border-color: rgb(124 58 237);
        /// }
        /// ```
        Violet600 => "border-violet-600",
        /// ```css
        /// {
        ///     border-color: rgb(109 40 217);
        /// }
        /// ```
        Violet700 => "border-violet-700",
        /// ```css
        /// {
        ///     border-color: rgb(91 33 182);
        /// }
        /// ```
        Violet800 => "border-violet-800",
        /// ```css
        /// {
        ///     border-color: rgb(76 29 149);
        /// }
        /// ```
        Violet900 => "border-violet-900",
        /// ```css
        /// {
        ///     border-color: rgb(46 16 101);
        /// }
        /// ```
        Violet950 => "border-violet-950",
        /// ```css
        /// {
        ///     border-color: rgb(250 245 255);
        /// }
        /// ```
        Purple50 => "border-purple-50",
        /// ```css
        /// {
        ///     border-color: rgb(243 232 255);
        /// }
        /// ```
        Purple100 => "border-purple-100",
        /// ```css
        /// {
        ///     border-color: rgb(233 213 255);
        /// }
        /// ```
        Purple200 => "border-purple-200",
        /// ```css
        /// {
        ///     border-color: rgb(216 180 254);
        /// }
        /// ```
        Purple300 => "border-purple-300",
        /// ```css
        /// {
        ///     border-color: rgb(192 132 252);
        /// }
        /// ```
        Purple400 => "border-purple-400",
        /// ```css
        /// {
        ///     border-color: rgb(168 85 247);
        /// }
        /// ```
        Purple500 => "border-purple-500",
        /// ```css
        /// {
        ///     border-color: rgb(147 51 234);
        /// }
        /// ```
        Purple600 => "border-purple-600",
        /// ```css
        /// {
        ///     border-color: rgb(126 34 206);
        /// }
        /// ```
        Purple700 => "border-purple-700",
        /// ```css
        /// {
        ///     border-color: rgb(107 33 168);
        /// }
        /// ```
        Purple800 => "border-purple-800",
        /// ```css
        /// {
        ///     border-color: rgb(88 28 135);
        /// }
        /// ```
        Purple900 => "border-purple-900",
        /// ```css
        /// {
        ///     border-color: rgb(59 7 100);
        /// }
        /// ```
        Purple950 => "border-purple-950",
        /// ```css
        /// {
        ///     border-color: rgb(253 244 255);
        /// }
        /// ```
        Fuchsia50 => "border-fuchsia-50",
        /// ```css
        /// {
        ///     border-color: rgb(250 232 255);
        /// }
        /// ```
        Fuchsia100 => "border-fuchsia-100",
        /// ```css
        /// {
        ///     border-color: rgb(245 208 254);
        /// }
        /// ```
        Fuchsia200 => "border-fuchsia-200",
        /// ```css
        /// {
        ///     border-color: rgb(240 171 252);
        /// }
        /// ```
        Fuchsia300 => "border-fuchsia-300",
        /// ```css
        /// {
        ///     border-color: rgb(232 121 249);
        /// }
        /// ```
        Fuchsia400 => "border-fuchsia-400",
        /// ```css
        /// {
        ///     border-color: rgb(217 70 239);
        /// }
        /// ```
        Fuchsia500 => "border-fuchsia-500",
        /// ```css
        /// {
        ///     border-color: rgb(192 38 211);
        /// }
        /// ```
        Fuchsia600 => "border-fuchsia-600",
        /// ```css
        /// {
        ///     border-color: rgb(162 28 175);
        /// }
        /// ```
        Fuchsia700 => "border-fuchsia-700",
        /// ```css
        /// {
        ///     border-color: rgb(134 25 143);
        /// }
        /// ```
        Fuchsia800 => "border-fuchsia-800",
        /// ```css
        /// {
        ///     border-color: rgb(112 26 117);
        /// }
        /// ```
        Fuchsia900 => "border-fuchsia-900",
        /// ```css
        /// {
        ///     border-color: rgb(74 4 78);
        /// }
        /// ```
        Fuchsia950 => "border-fuchsia-950",
        /// ```css
        /// {
        ///     border-color: rgb(253 242 248);
        /// }
        /// ```
        Pink50 => "border-pink-50",
        /// ```css
        /// {
        ///     border-color: rgb(252 231 243);
        /// }
        /// ```
        Pink100 => "border-pink-100",
        /// ```css
        /// {
        ///     border-color: rgb(251 207 232);
        /// }
        /// ```
        Pink200 => "border-pink-200",
        /// ```css
        /// {
        ///     border-color: rgb(249 168 212);
        /// }
        /// ```
        Pink300 => "border-pink-300",
        /// ```css
        /// {
        ///     border-color: rgb(244 114 182);
        /// }
        /// ```
        Pink400 => "border-pink-400",
        /// ```css
        /// {
        ///     border-color: rgb(236 72 153);
        /// }
        /// ```
        Pink500 => "border-pink-500",
        /// ```css
        /// {
        ///     border-color: rgb(219 39 119);
        /// }
        /// ```
        Pink600 => "border-pink-600",
        /// ```css
        /// {
        ///     border-color: rgb(190 24 93);
        /// }
        /// ```
        Pink700 => "border-pink-700",
        /// ```css
        /// {
        ///     border-color: rgb(157 23 77);
        /// }
        /// ```
        Pink800 => "border-pink-800",
        /// ```css
        /// {
        ///     border-color: rgb(131 24 67);
        /// }
        /// ```
        Pink900 => "border-pink-900",
        /// ```css
        /// {
        ///     border-color: rgb(80 7 36);
        /// }
        /// ```
        Pink950 => "border-pink-950",
        /// ```css
        /// {
        ///     border-color: rgb(255 241 242);
        /// }
        /// ```
        Rose50 => "border-rose-50",
        /// ```css
        /// {
        ///     border-color: rgb(255 228 230);
        /// }
        /// ```
        Rose100 => "border-rose-100",
        /// ```css
        /// {
        ///     border-color: rgb(254 205 211);
        /// }
        /// ```
        Rose200 => "border-rose-200",
        /// ```css
        /// {
        ///     border-color: rgb(253 164 175);
        /// }
        /// ```
        Rose300 => "border-rose-300",
        /// ```css
        /// {
        ///     border-color: rgb(251 113 133);
        /// }
        /// ```
        Rose400 => "border-rose-400",
        /// ```css
        /// {
        ///     border-color: rgb(244 63 94);
        /// }
        /// ```
        Rose500 => "border-rose-500",
        /// ```css
        /// {
        ///     border-color: rgb(225 29 72);
        /// }
        /// ```
        Rose600 => "border-rose-600",
        /// ```css
        /// {
        ///     border-color: rgb(190 18 60);
        /// }
        /// ```
        Rose700 => "border-rose-700",
        /// ```css
        /// {
        ///     border-color: rgb(159 18 57);
        /// }
        /// ```
        Rose800 => "border-rose-800",
        /// ```css
        /// {
        ///     border-color: rgb(136 19 55);
        /// }
        /// ```
        Rose900 => "border-rose-900",
        /// ```css
        /// {
        ///     border-color: rgb(76 5 25);
        /// }
        /// ```
        Rose950 => "border-rose-950",
        /// ```css
        /// {
        ///     border-left-color: inherit;
        ///     border-right-color: inherit;
        /// }
        /// ```
        XInherit => "border-x-inherit",
        /// ```css
        /// {
        ///     border-left-color: currentColor;
        ///     border-right-color: currentColor;
        /// }
        /// ```
        XCurrent => "border-x-current",
        /// ```css
        /// {
        ///     border-left-color: transparent;
        ///     border-right-color: transparent;
        /// }
        /// ```
        XTransparent => "border-x-transparent",
        /// ```css
        /// {
        ///     border-left-color: rgb(0 0 0);
        ///     border-right-color: rgb(0 0 0);
        /// }
        /// ```
        XBlack => "border-x-black",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 255 255);
        ///     border-right-color: rgb(255 255 255);
        /// }
        /// ```
        XWhite => "border-x-white",
        /// ```css
        /// {
        ///     border-left-color: rgb(248 250 252);
        ///     border-right-color: rgb(248 250 252);
        /// }
        /// ```
        XSlate50 => "border-x-slate-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(241 245 249);
        ///     border-right-color: rgb(241 245 249);
        /// }
        /// ```
        XSlate100 => "border-x-slate-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(226 232 240);
        ///     border-right-color: rgb(226 232 240);
        /// }
        /// ```
        XSlate200 => "border-x-slate-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(203 213 225);
        ///     border-right-color: rgb(203 213 225);
        /// }
        /// ```
        XSlate300 => "border-x-slate-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(148 163 184);
        ///     border-right-color: rgb(148 163 184);
        /// }
        /// ```
        XSlate400 => "border-x-slate-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(100 116 139);
        ///     border-right-color: rgb(100 116 139);
        /// }
        /// ```
        XSlate500 => "border-x-slate-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(71 85 105);
        ///     border-right-color: rgb(71 85 105);
        /// }
        /// ```
        XSlate600 => "border-x-slate-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(51 65 85);
        ///     border-right-color: rgb(51 65 85);
        /// }
        /// ```
        XSlate700 => "border-x-slate-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(30 41 59);
        ///     border-right-color: rgb(30 41 59);
        /// }
        /// ```
        XSlate800 => "border-x-slate-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(15 23 42);
        ///     border-right-color: rgb(15 23 42);
        /// }
        /// ```
        XSlate900 => "border-x-slate-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(2 6 23);
        ///     border-right-color: rgb(2 6 23);
        /// }
        /// ```
        XSlate950 => "border-x-slate-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(249 250 251);
        ///     border-right-color: rgb(249 250 251);
        /// }
        /// ```
        XGray50 => "border-x-gray-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(243 244 246);
        ///     border-right-color: rgb(243 244 246);
        /// }
        /// ```
        XGray100 => "border-x-gray-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(229 231 235);
        ///     border-right-color: rgb(229 231 235);
        /// }
        /// ```
        XGray200 => "border-x-gray-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(209 213 219);
        ///     border-right-color: rgb(209 213 219);
        /// }
        /// ```
        XGray300 => "border-x-gray-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(156 163 175);
        ///     border-right-color: rgb(156 163 175);
        /// }
        /// ```
        XGray400 => "border-x-gray-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(107 114 128);
        ///     border-right-color: rgb(107 114 128);
        /// }
        /// ```
        XGray500 => "border-x-gray-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(75 85 99);
        ///     border-right-color: rgb(75 85 99);
        /// }
        /// ```
        XGray600 => "border-x-gray-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(55 65 81);
        ///     border-right-color: rgb(55 65 81);
        /// }
        /// ```
        XGray700 => "border-x-gray-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(31 41 55);
        ///     border-right-color: rgb(31 41 55);
        /// }
        /// ```
        XGray800 => "border-x-gray-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(17 24 39);
        ///     border-right-color: rgb(17 24 39);
        /// }
        /// ```
        XGray900 => "border-x-gray-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(3 7 18);
        ///     border-right-color: rgb(3 7 18);
        /// }
        /// ```
        XGray950 => "border-x-gray-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 250 250);
        ///     border-right-color: rgb(250 250 250);
        /// }
        /// ```
        XZinc50 => "border-x-zinc-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(244 244 245);
        ///     border-right-color: rgb(244 244 245);
        /// }
        /// ```
        XZinc100 => "border-x-zinc-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(228 228 231);
        ///     border-right-color: rgb(228 228 231);
        /// }
        /// ```
        XZinc200 => "border-x-zinc-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(212 212 216);
        ///     border-right-color: rgb(212 212 216);
        /// }
        /// ```
        XZinc300 => "border-x-zinc-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(161 161 170);
        ///     border-right-color: rgb(161 161 170);
        /// }
        /// ```
        XZinc400 => "border-x-zinc-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(113 113 122);
        ///     border-right-color: rgb(113 113 122);
        /// }
        /// ```
        XZinc500 => "border-x-zinc-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(82 82 91);
        ///     border-right-color: rgb(82 82 91);
        /// }
        /// ```
        XZinc600 => "border-x-zinc-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(63 63 70);
        ///     border-right-color: rgb(63 63 70);
        /// }
        /// ```
        XZinc700 => "border-x-zinc-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(39 39 42);
        ///     border-right-color: rgb(39 39 42);
        /// }
        /// ```
        XZinc800 => "border-x-zinc-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(24 24 27);
        ///     border-right-color: rgb(24 24 27);
        /// }
        /// ```
        XZinc900 => "border-x-zinc-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(9 9 11);
        ///     border-right-color: rgb(9 9 11);
        /// }
        /// ```
        XZinc950 => "border-x-zinc-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 250 250);
        ///     border-right-color: rgb(250 250 250);
        /// }
        /// ```
        XNeutral50 => "border-x-neutral-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 245 245);
        ///     border-right-color: rgb(245 245 245);
        /// }
        /// ```
        XNeutral100 => "border-x-neutral-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(229 229 229);
        ///     border-right-color: rgb(229 229 229);
        /// }
        /// ```
        XNeutral200 => "border-x-neutral-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(212 212 212);
        ///     border-right-color: rgb(212 212 212);
        /// }
        /// ```
        XNeutral300 => "border-x-neutral-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(163 163 163);
        ///     border-right-color: rgb(163 163 163);
        /// }
        /// ```
        XNeutral400 => "border-x-neutral-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(115 115 115);
        ///     border-right-color: rgb(115 115 115);
        /// }
        /// ```
        XNeutral500 => "border-x-neutral-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(82 82 82);
        ///     border-right-color: rgb(82 82 82);
        /// }
        /// ```
        XNeutral600 => "border-x-neutral-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(64 64 64);
        ///     border-right-color: rgb(64 64 64);
        /// }
        /// ```
        XNeutral700 => "border-x-neutral-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(38 38 38);
        ///     border-right-color: rgb(38 38 38);
        /// }
        /// ```
        XNeutral800 => "border-x-neutral-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(23 23 23);
        ///     border-right-color: rgb(23 23 23);
        /// }
        /// ```
        XNeutral900 => "border-x-neutral-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(10 10 10);
        ///     border-right-color: rgb(10 10 10);
        /// }
        /// ```
        XNeutral950 => "border-x-neutral-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 250 249);
        ///     border-right-color: rgb(250 250 249);
        /// }
        /// ```
        XStone50 => "border-x-stone-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 245 244);
        ///     border-right-color: rgb(245 245 244);
        /// }
        /// ```
        XStone100 => "border-x-stone-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(231 229 228);
        ///     border-right-color: rgb(231 229 228);
        /// }
        /// ```
        XStone200 => "border-x-stone-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(214 211 209);
        ///     border-right-color: rgb(214 211 209);
        /// }
        /// ```
        XStone300 => "border-x-stone-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(168 162 158);
        ///     border-right-color: rgb(168 162 158);
        /// }
        /// ```
        XStone400 => "border-x-stone-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(120 113 108);
        ///     border-right-color: rgb(120 113 108);
        /// }
        /// ```
        XStone500 => "border-x-stone-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(87 83 78);
        ///     border-right-color: rgb(87 83 78);
        /// }
        /// ```
        XStone600 => "border-x-stone-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(68 64 60);
        ///     border-right-color: rgb(68 64 60);
        /// }
        /// ```
        XStone700 => "border-x-stone-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(41 37 36);
        ///     border-right-color: rgb(41 37 36);
        /// }
        /// ```
        XStone800 => "border-x-stone-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(28 25 23);
        ///     border-right-color: rgb(28 25 23);
        /// }
        /// ```
        XStone900 => "border-x-stone-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(12 10 9);
        ///     border-right-color: rgb(12 10 9);
        /// }
        /// ```
        XStone950 => "border-x-stone-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 242 242);
        ///     border-right-color: rgb(254 242 242);
        /// }
        /// ```
        XRed50 => "border-x-red-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 226 226);
        ///     border-right-color: rgb(254 226 226);
        /// }
        /// ```
        XRed100 => "border-x-red-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 202 202);
        ///     border-right-color: rgb(254 202 202);
        /// }
        /// ```
        XRed200 => "border-x-red-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(252 165 165);
        ///     border-right-color: rgb(252 165 165);
        /// }
        /// ```
        XRed300 => "border-x-red-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(248 113 113);
        ///     border-right-color: rgb(248 113 113);
        /// }
        /// ```
        XRed400 => "border-x-red-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(239 68 68);
        ///     border-right-color: rgb(239 68 68);
        /// }
        /// ```
        XRed500 => "border-x-red-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(220 38 38);
        ///     border-right-color: rgb(220 38 38);
        /// }
        /// ```
        XRed600 => "border-x-red-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(185 28 28);
        ///     border-right-color: rgb(185 28 28);
        /// }
        /// ```
        XRed700 => "border-x-red-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(153 27 27);
        ///     border-right-color: rgb(153 27 27);
        /// }
        /// ```
        XRed800 => "border-x-red-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(127 29 29);
        ///     border-right-color: rgb(127 29 29);
        /// }
        /// ```
        XRed900 => "border-x-red-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(69 10 10);
        ///     border-right-color: rgb(69 10 10);
        /// }
        /// ```
        XRed950 => "border-x-red-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 247 237);
        ///     border-right-color: rgb(255 247 237);
        /// }
        /// ```
        XOrange50 => "border-x-orange-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 237 213);
        ///     border-right-color: rgb(255 237 213);
        /// }
        /// ```
        XOrange100 => "border-x-orange-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 215 170);
        ///     border-right-color: rgb(254 215 170);
        /// }
        /// ```
        XOrange200 => "border-x-orange-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 186 116);
        ///     border-right-color: rgb(253 186 116);
        /// }
        /// ```
        XOrange300 => "border-x-orange-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(251 146 60);
        ///     border-right-color: rgb(251 146 60);
        /// }
        /// ```
        XOrange400 => "border-x-orange-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(249 115 22);
        ///     border-right-color: rgb(249 115 22);
        /// }
        /// ```
        XOrange500 => "border-x-orange-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(234 88 12);
        ///     border-right-color: rgb(234 88 12);
        /// }
        /// ```
        XOrange600 => "border-x-orange-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(194 65 12);
        ///     border-right-color: rgb(194 65 12);
        /// }
        /// ```
        XOrange700 => "border-x-orange-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(154 52 18);
        ///     border-right-color: rgb(154 52 18);
        /// }
        /// ```
        XOrange800 => "border-x-orange-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(124 45 18);
        ///     border-right-color: rgb(124 45 18);
        /// }
        /// ```
        XOrange900 => "border-x-orange-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(67 20 7);
        ///     border-right-color: rgb(67 20 7);
        /// }
        /// ```
        XOrange950 => "border-x-orange-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 251 235);
        ///     border-right-color: rgb(255 251 235);
        /// }
        /// ```
        XAmber50 => "border-x-amber-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 243 199);
        ///     border-right-color: rgb(254 243 199);
        /// }
        /// ```
        XAmber100 => "border-x-amber-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 230 138);
        ///     border-right-color: rgb(253 230 138);
        /// }
        /// ```
        XAmber200 => "border-x-amber-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(252 211 77);
        ///     border-right-color: rgb(252 211 77);
        /// }
        /// ```
        XAmber300 => "border-x-amber-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(251 191 36);
        ///     border-right-color: rgb(251 191 36);
        /// }
        /// ```
        XAmber400 => "border-x-amber-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 158 11);
        ///     border-right-color: rgb(245 158 11);
        /// }
        /// ```
        XAmber500 => "border-x-amber-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(217 119 6);
        ///     border-right-color: rgb(217 119 6);
        /// }
        /// ```
        XAmber600 => "border-x-amber-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(180 83 9);
        ///     border-right-color: rgb(180 83 9);
        /// }
        /// ```
        XAmber700 => "border-x-amber-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(146 64 14);
        ///     border-right-color: rgb(146 64 14);
        /// }
        /// ```
        XAmber800 => "border-x-amber-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(120 53 15);
        ///     border-right-color: rgb(120 53 15);
        /// }
        /// ```
        XAmber900 => "border-x-amber-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(69 26 3);
        ///     border-right-color: rgb(69 26 3);
        /// }
        /// ```
        XAmber950 => "border-x-amber-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 252 232);
        ///     border-right-color: rgb(254 252 232);
        /// }
        /// ```
        XYellow50 => "border-x-yellow-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 249 195);
        ///     border-right-color: rgb(254 249 195);
        /// }
        /// ```
        XYellow100 => "border-x-yellow-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 240 138);
        ///     border-right-color: rgb(254 240 138);
        /// }
        /// ```
        XYellow200 => "border-x-yellow-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 224 71);
        ///     border-right-color: rgb(253 224 71);
        /// }
        /// ```
        XYellow300 => "border-x-yellow-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 204 21);
        ///     border-right-color: rgb(250 204 21);
        /// }
        /// ```
        XYellow400 => "border-x-yellow-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(234 179 8);
        ///     border-right-color: rgb(234 179 8);
        /// }
        /// ```
        XYellow500 => "border-x-yellow-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(202 138 4);
        ///     border-right-color: rgb(202 138 4);
        /// }
        /// ```
        XYellow600 => "border-x-yellow-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(161 98 7);
        ///     border-right-color: rgb(161 98 7);
        /// }
        /// ```
        XYellow700 => "border-x-yellow-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(133 77 14);
        ///     border-right-color: rgb(133 77 14);
        /// }
        /// ```
        XYellow800 => "border-x-yellow-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(113 63 18);
        ///     border-right-color: rgb(113 63 18);
        /// }
        /// ```
        XYellow900 => "border-x-yellow-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(66 32 6);
        ///     border-right-color: rgb(66 32 6);
        /// }
        /// ```
        XYellow950 => "border-x-yellow-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(247 254 231);
        ///     border-right-color: rgb(247 254 231);
        /// }
        /// ```
        XLime50 => "border-x-lime-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(236 252 203);
        ///     border-right-color: rgb(236 252 203);
        /// }
        /// ```
        XLime100 => "border-x-lime-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(217 249 157);
        ///     border-right-color: rgb(217 249 157);
        /// }
        /// ```
        XLime200 => "border-x-lime-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(190 242 100);
        ///     border-right-color: rgb(190 242 100);
        /// }
        /// ```
        XLime300 => "border-x-lime-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(163 230 53);
        ///     border-right-color: rgb(163 230 53);
        /// }
        /// ```
        XLime400 => "border-x-lime-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(132 204 22);
        ///     border-right-color: rgb(132 204 22);
        /// }
        /// ```
        XLime500 => "border-x-lime-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(101 163 13);
        ///     border-right-color: rgb(101 163 13);
        /// }
        /// ```
        XLime600 => "border-x-lime-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(77 124 15);
        ///     border-right-color: rgb(77 124 15);
        /// }
        /// ```
        XLime700 => "border-x-lime-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(63 98 18);
        ///     border-right-color: rgb(63 98 18);
        /// }
        /// ```
        XLime800 => "border-x-lime-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(54 83 20);
        ///     border-right-color: rgb(54 83 20);
        /// }
        /// ```
        XLime900 => "border-x-lime-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(26 46 5);
        ///     border-right-color: rgb(26 46 5);
        /// }
        /// ```
        XLime950 => "border-x-lime-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(240 253 244);
        ///     border-right-color: rgb(240 253 244);
        /// }
        /// ```
        XGreen50 => "border-x-green-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(220 252 231);
        ///     border-right-color: rgb(220 252 231);
        /// }
        /// ```
        XGreen100 => "border-x-green-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(187 247 208);
        ///     border-right-color: rgb(187 247 208);
        /// }
        /// ```
        XGreen200 => "border-x-green-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(134 239 172);
        ///     border-right-color: rgb(134 239 172);
        /// }
        /// ```
        XGreen300 => "border-x-green-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(74 222 128);
        ///     border-right-color: rgb(74 222 128);
        /// }
        /// ```
        XGreen400 => "border-x-green-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(34 197 94);
        ///     border-right-color: rgb(34 197 94);
        /// }
        /// ```
        XGreen500 => "border-x-green-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(22 163 74);
        ///     border-right-color: rgb(22 163 74);
        /// }
        /// ```
        XGreen600 => "border-x-green-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(21 128 61);
        ///     border-right-color: rgb(21 128 61);
        /// }
        /// ```
        XGreen700 => "border-x-green-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(22 101 52);
        ///     border-right-color: rgb(22 101 52);
        /// }
        /// ```
        XGreen800 => "border-x-green-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(20 83 45);
        ///     border-right-color: rgb(20 83 45);
        /// }
        /// ```
        XGreen900 => "border-x-green-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(5 46 22);
        ///     border-right-color: rgb(5 46 22);
        /// }
        /// ```
        XGreen950 => "border-x-green-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(236 253 245);
        ///     border-right-color: rgb(236 253 245);
        /// }
        /// ```
        XEmerald50 => "border-x-emerald-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(209 250 229);
        ///     border-right-color: rgb(209 250 229);
        /// }
        /// ```
        XEmerald100 => "border-x-emerald-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(167 243 208);
        ///     border-right-color: rgb(167 243 208);
        /// }
        /// ```
        XEmerald200 => "border-x-emerald-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(110 231 183);
        ///     border-right-color: rgb(110 231 183);
        /// }
        /// ```
        XEmerald300 => "border-x-emerald-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(52 211 153);
        ///     border-right-color: rgb(52 211 153);
        /// }
        /// ```
        XEmerald400 => "border-x-emerald-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(16 185 129);
        ///     border-right-color: rgb(16 185 129);
        /// }
        /// ```
        XEmerald500 => "border-x-emerald-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(5 150 105);
        ///     border-right-color: rgb(5 150 105);
        /// }
        /// ```
        XEmerald600 => "border-x-emerald-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(4 120 87);
        ///     border-right-color: rgb(4 120 87);
        /// }
        /// ```
        XEmerald700 => "border-x-emerald-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(6 95 70);
        ///     border-right-color: rgb(6 95 70);
        /// }
        /// ```
        XEmerald800 => "border-x-emerald-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(6 78 59);
        ///     border-right-color: rgb(6 78 59);
        /// }
        /// ```
        XEmerald900 => "border-x-emerald-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(2 44 34);
        ///     border-right-color: rgb(2 44 34);
        /// }
        /// ```
        XEmerald950 => "border-x-emerald-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(240 253 250);
        ///     border-right-color: rgb(240 253 250);
        /// }
        /// ```
        XTeal50 => "border-x-teal-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(204 251 241);
        ///     border-right-color: rgb(204 251 241);
        /// }
        /// ```
        XTeal100 => "border-x-teal-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(153 246 228);
        ///     border-right-color: rgb(153 246 228);
        /// }
        /// ```
        XTeal200 => "border-x-teal-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(94 234 212);
        ///     border-right-color: rgb(94 234 212);
        /// }
        /// ```
        XTeal300 => "border-x-teal-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(45 212 191);
        ///     border-right-color: rgb(45 212 191);
        /// }
        /// ```
        XTeal400 => "border-x-teal-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(20 184 166);
        ///     border-right-color: rgb(20 184 166);
        /// }
        /// ```
        XTeal500 => "border-x-teal-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(13 148 136);
        ///     border-right-color: rgb(13 148 136);
        /// }
        /// ```
        XTeal600 => "border-x-teal-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(15 118 110);
        ///     border-right-color: rgb(15 118 110);
        /// }
        /// ```
        XTeal700 => "border-x-teal-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(17 94 89);
        ///     border-right-color: rgb(17 94 89);
        /// }
        /// ```
        XTeal800 => "border-x-teal-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(19 78 74);
        ///     border-right-color: rgb(19 78 74);
        /// }
        /// ```
        XTeal900 => "border-x-teal-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(4 47 46);
        ///     border-right-color: rgb(4 47 46);
        /// }
        /// ```
        XTeal950 => "border-x-teal-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(236 254 255);
        ///     border-right-color: rgb(236 254 255);
        /// }
        /// ```
        XCyan50 => "border-x-cyan-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(207 250 254);
        ///     border-right-color: rgb(207 250 254);
        /// }
        /// ```
        XCyan100 => "border-x-cyan-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(165 243 252);
        ///     border-right-color: rgb(165 243 252);
        /// }
        /// ```
        XCyan200 => "border-x-cyan-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(103 232 249);
        ///     border-right-color: rgb(103 232 249);
        /// }
        /// ```
        XCyan300 => "border-x-cyan-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(34 211 238);
        ///     border-right-color: rgb(34 211 238);
        /// }
        /// ```
        XCyan400 => "border-x-cyan-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(6 182 212);
        ///     border-right-color: rgb(6 182 212);
        /// }
        /// ```
        XCyan500 => "border-x-cyan-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(8 145 178);
        ///     border-right-color: rgb(8 145 178);
        /// }
        /// ```
        XCyan600 => "border-x-cyan-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(14 116 144);
        ///     border-right-color: rgb(14 116 144);
        /// }
        /// ```
        XCyan700 => "border-x-cyan-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(21 94 117);
        ///     border-right-color: rgb(21 94 117);
        /// }
        /// ```
        XCyan800 => "border-x-cyan-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(22 78 99);
        ///     border-right-color: rgb(22 78 99);
        /// }
        /// ```
        XCyan900 => "border-x-cyan-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(8 51 68);
        ///     border-right-color: rgb(8 51 68);
        /// }
        /// ```
        XCyan950 => "border-x-cyan-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(240 249 255);
        ///     border-right-color: rgb(240 249 255);
        /// }
        /// ```
        XSky50 => "border-x-sky-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(224 242 254);
        ///     border-right-color: rgb(224 242 254);
        /// }
        /// ```
        XSky100 => "border-x-sky-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(186 230 253);
        ///     border-right-color: rgb(186 230 253);
        /// }
        /// ```
        XSky200 => "border-x-sky-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(125 211 252);
        ///     border-right-color: rgb(125 211 252);
        /// }
        /// ```
        XSky300 => "border-x-sky-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(56 189 248);
        ///     border-right-color: rgb(56 189 248);
        /// }
        /// ```
        XSky400 => "border-x-sky-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(14 165 233);
        ///     border-right-color: rgb(14 165 233);
        /// }
        /// ```
        XSky500 => "border-x-sky-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(2 132 199);
        ///     border-right-color: rgb(2 132 199);
        /// }
        /// ```
        XSky600 => "border-x-sky-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(3 105 161);
        ///     border-right-color: rgb(3 105 161);
        /// }
        /// ```
        XSky700 => "border-x-sky-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(7 89 133);
        ///     border-right-color: rgb(7 89 133);
        /// }
        /// ```
        XSky800 => "border-x-sky-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(12 74 110);
        ///     border-right-color: rgb(12 74 110);
        /// }
        /// ```
        XSky900 => "border-x-sky-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(8 47 73);
        ///     border-right-color: rgb(8 47 73);
        /// }
        /// ```
        XSky950 => "border-x-sky-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(239 246 255);
        ///     border-right-color: rgb(239 246 255);
        /// }
        /// ```
        XBlue50 => "border-x-blue-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(219 234 254);
        ///     border-right-color: rgb(219 234 254);
        /// }
        /// ```
        XBlue100 => "border-x-blue-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(191 219 254);
        ///     border-right-color: rgb(191 219 254);
        /// }
        /// ```
        XBlue200 => "border-x-blue-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(147 197 253);
        ///     border-right-color: rgb(147 197 253);
        /// }
        /// ```
        XBlue300 => "border-x-blue-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(96 165 250);
        ///     border-right-color: rgb(96 165 250);
        /// }
        /// ```
        XBlue400 => "border-x-blue-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(59 130 246);
        ///     border-right-color: rgb(59 130 246);
        /// }
        /// ```
        XBlue500 => "border-x-blue-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(37 99 235);
        ///     border-right-color: rgb(37 99 235);
        /// }
        /// ```
        XBlue600 => "border-x-blue-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(29 78 216);
        ///     border-right-color: rgb(29 78 216);
        /// }
        /// ```
        XBlue700 => "border-x-blue-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(30 64 175);
        ///     border-right-color: rgb(30 64 175);
        /// }
        /// ```
        XBlue800 => "border-x-blue-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(30 58 138);
        ///     border-right-color: rgb(30 58 138);
        /// }
        /// ```
        XBlue900 => "border-x-blue-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(23 37 84);
        ///     border-right-color: rgb(23 37 84);
        /// }
        /// ```
        XBlue950 => "border-x-blue-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(238 242 255);
        ///     border-right-color: rgb(238 242 255);
        /// }
        /// ```
        XIndigo50 => "border-x-indigo-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(224 231 255);
        ///     border-right-color: rgb(224 231 255);
        /// }
        /// ```
        XIndigo100 => "border-x-indigo-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(199 210 254);
        ///     border-right-color: rgb(199 210 254);
        /// }
        /// ```
        XIndigo200 => "border-x-indigo-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(165 180 252);
        ///     border-right-color: rgb(165 180 252);
        /// }
        /// ```
        XIndigo300 => "border-x-indigo-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(129 140 248);
        ///     border-right-color: rgb(129 140 248);
        /// }
        /// ```
        XIndigo400 => "border-x-indigo-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(99 102 241);
        ///     border-right-color: rgb(99 102 241);
        /// }
        /// ```
        XIndigo500 => "border-x-indigo-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(79 70 229);
        ///     border-right-color: rgb(79 70 229);
        /// }
        /// ```
        XIndigo600 => "border-x-indigo-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(67 56 202);
        ///     border-right-color: rgb(67 56 202);
        /// }
        /// ```
        XIndigo700 => "border-x-indigo-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(55 48 163);
        ///     border-right-color: rgb(55 48 163);
        /// }
        /// ```
        XIndigo800 => "border-x-indigo-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(49 46 129);
        ///     border-right-color: rgb(49 46 129);
        /// }
        /// ```
        XIndigo900 => "border-x-indigo-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(30 27 75);
        ///     border-right-color: rgb(30 27 75);
        /// }
        /// ```
        XIndigo950 => "border-x-indigo-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 243 255);
        ///     border-right-color: rgb(245 243 255);
        /// }
        /// ```
        XViolet50 => "border-x-violet-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(237 233 254);
        ///     border-right-color: rgb(237 233 254);
        /// }
        /// ```
        XViolet100 => "border-x-violet-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(221 214 254);
        ///     border-right-color: rgb(221 214 254);
        /// }
        /// ```
        XViolet200 => "border-x-violet-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(196 181 253);
        ///     border-right-color: rgb(196 181 253);
        /// }
        /// ```
        XViolet300 => "border-x-violet-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(167 139 250);
        ///     border-right-color: rgb(167 139 250);
        /// }
        /// ```
        XViolet400 => "border-x-violet-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(139 92 246);
        ///     border-right-color: rgb(139 92 246);
        /// }
        /// ```
        XViolet500 => "border-x-violet-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(124 58 237);
        ///     border-right-color: rgb(124 58 237);
        /// }
        /// ```
        XViolet600 => "border-x-violet-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(109 40 217);
        ///     border-right-color: rgb(109 40 217);
        /// }
        /// ```
        XViolet700 => "border-x-violet-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(91 33 182);
        ///     border-right-color: rgb(91 33 182);
        /// }
        /// ```
        XViolet800 => "border-x-violet-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(76 29 149);
        ///     border-right-color: rgb(76 29 149);
        /// }
        /// ```
        XViolet900 => "border-x-violet-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(46 16 101);
        ///     border-right-color: rgb(46 16 101);
        /// }
        /// ```
        XViolet950 => "border-x-violet-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 245 255);
        ///     border-right-color: rgb(250 245 255);
        /// }
        /// ```
        XPurple50 => "border-x-purple-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(243 232 255);
        ///     border-right-color: rgb(243 232 255);
        /// }
        /// ```
        XPurple100 => "border-x-purple-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(233 213 255);
        ///     border-right-color: rgb(233 213 255);
        /// }
        /// ```
        XPurple200 => "border-x-purple-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(216 180 254);
        ///     border-right-color: rgb(216 180 254);
        /// }
        /// ```
        XPurple300 => "border-x-purple-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(192 132 252);
        ///     border-right-color: rgb(192 132 252);
        /// }
        /// ```
        XPurple400 => "border-x-purple-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(168 85 247);
        ///     border-right-color: rgb(168 85 247);
        /// }
        /// ```
        XPurple500 => "border-x-purple-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(147 51 234);
        ///     border-right-color: rgb(147 51 234);
        /// }
        /// ```
        XPurple600 => "border-x-purple-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(126 34 206);
        ///     border-right-color: rgb(126 34 206);
        /// }
        /// ```
        XPurple700 => "border-x-purple-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(107 33 168);
        ///     border-right-color: rgb(107 33 168);
        /// }
        /// ```
        XPurple800 => "border-x-purple-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(88 28 135);
        ///     border-right-color: rgb(88 28 135);
        /// }
        /// ```
        XPurple900 => "border-x-purple-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(59 7 100);
        ///     border-right-color: rgb(59 7 100);
        /// }
        /// ```
        XPurple950 => "border-x-purple-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 244 255);
        ///     border-right-color: rgb(253 244 255);
        /// }
        /// ```
        XFuchsia50 => "border-x-fuchsia-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 232 255);
        ///     border-right-color: rgb(250 232 255);
        /// }
        /// ```
        XFuchsia100 => "border-x-fuchsia-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 208 254);
        ///     border-right-color: rgb(245 208 254);
        /// }
        /// ```
        XFuchsia200 => "border-x-fuchsia-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(240 171 252);
        ///     border-right-color: rgb(240 171 252);
        /// }
        /// ```
        XFuchsia300 => "border-x-fuchsia-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(232 121 249);
        ///     border-right-color: rgb(232 121 249);
        /// }
        /// ```
        XFuchsia400 => "border-x-fuchsia-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(217 70 239);
        ///     border-right-color: rgb(217 70 239);
        /// }
        /// ```
        XFuchsia500 => "border-x-fuchsia-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(192 38 211);
        ///     border-right-color: rgb(192 38 211);
        /// }
        /// ```
        XFuchsia600 => "border-x-fuchsia-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(162 28 175);
        ///     border-right-color: rgb(162 28 175);
        /// }
        /// ```
        XFuchsia700 => "border-x-fuchsia-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(134 25 143);
        ///     border-right-color: rgb(134 25 143);
        /// }
        /// ```
        XFuchsia800 => "border-x-fuchsia-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(112 26 117);
        ///     border-right-color: rgb(112 26 117);
        /// }
        /// ```
        XFuchsia900 => "border-x-fuchsia-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(74 4 78);
        ///     border-right-color: rgb(74 4 78);
        /// }
        /// ```
        XFuchsia950 => "border-x-fuchsia-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 242 248);
        ///     border-right-color: rgb(253 242 248);
        /// }
        /// ```
        XPink50 => "border-x-pink-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(252 231 243);
        ///     border-right-color: rgb(252 231 243);
        /// }
        /// ```
        XPink100 => "border-x-pink-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(251 207 232);
        ///     border-right-color: rgb(251 207 232);
        /// }
        /// ```
        XPink200 => "border-x-pink-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(249 168 212);
        ///     border-right-color: rgb(249 168 212);
        /// }
        /// ```
        XPink300 => "border-x-pink-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(244 114 182);
        ///     border-right-color: rgb(244 114 182);
        /// }
        /// ```
        XPink400 => "border-x-pink-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(236 72 153);
        ///     border-right-color: rgb(236 72 153);
        /// }
        /// ```
        XPink500 => "border-x-pink-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(219 39 119);
        ///     border-right-color: rgb(219 39 119);
        /// }
        /// ```
        XPink600 => "border-x-pink-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(190 24 93);
        ///     border-right-color: rgb(190 24 93);
        /// }
        /// ```
        XPink700 => "border-x-pink-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(157 23 77);
        ///     border-right-color: rgb(157 23 77);
        /// }
        /// ```
        XPink800 => "border-x-pink-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(131 24 67);
        ///     border-right-color: rgb(131 24 67);
        /// }
        /// ```
        XPink900 => "border-x-pink-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(80 7 36);
        ///     border-right-color: rgb(80 7 36);
        /// }
        /// ```
        XPink950 => "border-x-pink-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 241 242);
        ///     border-right-color: rgb(255 241 242);
        /// }
        /// ```
        XRose50 => "border-x-rose-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 228 230);
        ///     border-right-color: rgb(255 228 230);
        /// }
        /// ```
        XRose100 => "border-x-rose-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 205 211);
        ///     border-right-color: rgb(254 205 211);
        /// }
        /// ```
        XRose200 => "border-x-rose-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 164 175);
        ///     border-right-color: rgb(253 164 175);
        /// }
        /// ```
        XRose300 => "border-x-rose-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(251 113 133);
        ///     border-right-color: rgb(251 113 133);
        /// }
        /// ```
        XRose400 => "border-x-rose-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(244 63 94);
        ///     border-right-color: rgb(244 63 94);
        /// }
        /// ```
        XRose500 => "border-x-rose-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(225 29 72);
        ///     border-right-color: rgb(225 29 72);
        /// }
        /// ```
        XRose600 => "border-x-rose-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(190 18 60);
        ///     border-right-color: rgb(190 18 60);
        /// }
        /// ```
        XRose700 => "border-x-rose-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(159 18 57);
        ///     border-right-color: rgb(159 18 57);
        /// }
        /// ```
        XRose800 => "border-x-rose-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(136 19 55);
        ///     border-right-color: rgb(136 19 55);
        /// }
        /// ```
        XRose900 => "border-x-rose-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(76 5 25);
        ///     border-right-color: rgb(76 5 25);
        /// }
        /// ```
        XRose950 => "border-x-rose-950",
        /// ```css
        /// {
        ///     border-top-color: inherit;
        ///     border-bottom-color: inherit;
        /// }
        /// ```
        YInherit => "border-y-inherit",
        /// ```css
        /// {
        ///     border-top-color: currentColor;
        ///     border-bottom-color: currentColor;
        /// }
        /// ```
        YCurrent => "border-y-current",
        /// ```css
        /// {
        ///     border-top-color: transparent;
        ///     border-bottom-color: transparent;
        /// }
        /// ```
        YTransparent => "border-y-transparent",
        /// ```css
        /// {
        ///     border-top-color: rgb(0 0 0);
        ///     border-bottom-color: rgb(0 0 0);
        /// }
        /// ```
        YBlack => "border-y-black",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 255 255);
        ///     border-bottom-color: rgb(255 255 255);
        /// }
        /// ```
        YWhite => "border-y-white",
        /// ```css
        /// {
        ///     border-top-color: rgb(248 250 252);
        ///     border-bottom-color: rgb(248 250 252);
        /// }
        /// ```
        YSlate50 => "border-y-slate-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(241 245 249);
        ///     border-bottom-color: rgb(241 245 249);
        /// }
        /// ```
        YSlate100 => "border-y-slate-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(226 232 240);
        ///     border-bottom-color: rgb(226 232 240);
        /// }
        /// ```
        YSlate200 => "border-y-slate-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(203 213 225);
        ///     border-bottom-color: rgb(203 213 225);
        /// }
        /// ```
        YSlate300 => "border-y-slate-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(148 163 184);
        ///     border-bottom-color: rgb(148 163 184);
        /// }
        /// ```
        YSlate400 => "border-y-slate-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(100 116 139);
        ///     border-bottom-color: rgb(100 116 139);
        /// }
        /// ```
        YSlate500 => "border-y-slate-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(71 85 105);
        ///     border-bottom-color: rgb(71 85 105);
        /// }
        /// ```
        YSlate600 => "border-y-slate-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(51 65 85);
        ///     border-bottom-color: rgb(51 65 85);
        /// }
        /// ```
        YSlate700 => "border-y-slate-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(30 41 59);
        ///     border-bottom-color: rgb(30 41 59);
        /// }
        /// ```
        YSlate800 => "border-y-slate-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(15 23 42);
        ///     border-bottom-color: rgb(15 23 42);
        /// }
        /// ```
        YSlate900 => "border-y-slate-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(2 6 23);
        ///     border-bottom-color: rgb(2 6 23);
        /// }
        /// ```
        YSlate950 => "border-y-slate-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(249 250 251);
        ///     border-bottom-color: rgb(249 250 251);
        /// }
        /// ```
        YGray50 => "border-y-gray-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(243 244 246);
        ///     border-bottom-color: rgb(243 244 246);
        /// }
        /// ```
        YGray100 => "border-y-gray-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(229 231 235);
        ///     border-bottom-color: rgb(229 231 235);
        /// }
        /// ```
        YGray200 => "border-y-gray-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(209 213 219);
        ///     border-bottom-color: rgb(209 213 219);
        /// }
        /// ```
        YGray300 => "border-y-gray-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(156 163 175);
        ///     border-bottom-color: rgb(156 163 175);
        /// }
        /// ```
        YGray400 => "border-y-gray-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(107 114 128);
        ///     border-bottom-color: rgb(107 114 128);
        /// }
        /// ```
        YGray500 => "border-y-gray-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(75 85 99);
        ///     border-bottom-color: rgb(75 85 99);
        /// }
        /// ```
        YGray600 => "border-y-gray-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(55 65 81);
        ///     border-bottom-color: rgb(55 65 81);
        /// }
        /// ```
        YGray700 => "border-y-gray-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(31 41 55);
        ///     border-bottom-color: rgb(31 41 55);
        /// }
        /// ```
        YGray800 => "border-y-gray-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(17 24 39);
        ///     border-bottom-color: rgb(17 24 39);
        /// }
        /// ```
        YGray900 => "border-y-gray-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(3 7 18);
        ///     border-bottom-color: rgb(3 7 18);
        /// }
        /// ```
        YGray950 => "border-y-gray-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 250 250);
        ///     border-bottom-color: rgb(250 250 250);
        /// }
        /// ```
        YZinc50 => "border-y-zinc-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(244 244 245);
        ///     border-bottom-color: rgb(244 244 245);
        /// }
        /// ```
        YZinc100 => "border-y-zinc-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(228 228 231);
        ///     border-bottom-color: rgb(228 228 231);
        /// }
        /// ```
        YZinc200 => "border-y-zinc-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(212 212 216);
        ///     border-bottom-color: rgb(212 212 216);
        /// }
        /// ```
        YZinc300 => "border-y-zinc-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(161 161 170);
        ///     border-bottom-color: rgb(161 161 170);
        /// }
        /// ```
        YZinc400 => "border-y-zinc-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(113 113 122);
        ///     border-bottom-color: rgb(113 113 122);
        /// }
        /// ```
        YZinc500 => "border-y-zinc-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(82 82 91);
        ///     border-bottom-color: rgb(82 82 91);
        /// }
        /// ```
        YZinc600 => "border-y-zinc-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(63 63 70);
        ///     border-bottom-color: rgb(63 63 70);
        /// }
        /// ```
        YZinc700 => "border-y-zinc-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(39 39 42);
        ///     border-bottom-color: rgb(39 39 42);
        /// }
        /// ```
        YZinc800 => "border-y-zinc-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(24 24 27);
        ///     border-bottom-color: rgb(24 24 27);
        /// }
        /// ```
        YZinc900 => "border-y-zinc-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(9 9 11);
        ///     border-bottom-color: rgb(9 9 11);
        /// }
        /// ```
        YZinc950 => "border-y-zinc-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 250 250);
        ///     border-bottom-color: rgb(250 250 250);
        /// }
        /// ```
        YNeutral50 => "border-y-neutral-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 245 245);
        ///     border-bottom-color: rgb(245 245 245);
        /// }
        /// ```
        YNeutral100 => "border-y-neutral-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(229 229 229);
        ///     border-bottom-color: rgb(229 229 229);
        /// }
        /// ```
        YNeutral200 => "border-y-neutral-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(212 212 212);
        ///     border-bottom-color: rgb(212 212 212);
        /// }
        /// ```
        YNeutral300 => "border-y-neutral-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(163 163 163);
        ///     border-bottom-color: rgb(163 163 163);
        /// }
        /// ```
        YNeutral400 => "border-y-neutral-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(115 115 115);
        ///     border-bottom-color: rgb(115 115 115);
        /// }
        /// ```
        YNeutral500 => "border-y-neutral-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(82 82 82);
        ///     border-bottom-color: rgb(82 82 82);
        /// }
        /// ```
        YNeutral600 => "border-y-neutral-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(64 64 64);
        ///     border-bottom-color: rgb(64 64 64);
        /// }
        /// ```
        YNeutral700 => "border-y-neutral-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(38 38 38);
        ///     border-bottom-color: rgb(38 38 38);
        /// }
        /// ```
        YNeutral800 => "border-y-neutral-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(23 23 23);
        ///     border-bottom-color: rgb(23 23 23);
        /// }
        /// ```
        YNeutral900 => "border-y-neutral-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(10 10 10);
        ///     border-bottom-color: rgb(10 10 10);
        /// }
        /// ```
        YNeutral950 => "border-y-neutral-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 250 249);
        ///     border-bottom-color: rgb(250 250 249);
        /// }
        /// ```
        YStone50 => "border-y-stone-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 245 244);
        ///     border-bottom-color: rgb(245 245 244);
        /// }
        /// ```
        YStone100 => "border-y-stone-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(231 229 228);
        ///     border-bottom-color: rgb(231 229 228);
        /// }
        /// ```
        YStone200 => "border-y-stone-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(214 211 209);
        ///     border-bottom-color: rgb(214 211 209);
        /// }
        /// ```
        YStone300 => "border-y-stone-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(168 162 158);
        ///     border-bottom-color: rgb(168 162 158);
        /// }
        /// ```
        YStone400 => "border-y-stone-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(120 113 108);
        ///     border-bottom-color: rgb(120 113 108);
        /// }
        /// ```
        YStone500 => "border-y-stone-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(87 83 78);
        ///     border-bottom-color: rgb(87 83 78);
        /// }
        /// ```
        YStone600 => "border-y-stone-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(68 64 60);
        ///     border-bottom-color: rgb(68 64 60);
        /// }
        /// ```
        YStone700 => "border-y-stone-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(41 37 36);
        ///     border-bottom-color: rgb(41 37 36);
        /// }
        /// ```
        YStone800 => "border-y-stone-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(28 25 23);
        ///     border-bottom-color: rgb(28 25 23);
        /// }
        /// ```
        YStone900 => "border-y-stone-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(12 10 9);
        ///     border-bottom-color: rgb(12 10 9);
        /// }
        /// ```
        YStone950 => "border-y-stone-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 242 242);
        ///     border-bottom-color: rgb(254 242 242);
        /// }
        /// ```
        YRed50 => "border-y-red-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 226 226);
        ///     border-bottom-color: rgb(254 226 226);
        /// }
        /// ```
        YRed100 => "border-y-red-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 202 202);
        ///     border-bottom-color: rgb(254 202 202);
        /// }
        /// ```
        YRed200 => "border-y-red-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(252 165 165);
        ///     border-bottom-color: rgb(252 165 165);
        /// }
        /// ```
        YRed300 => "border-y-red-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(248 113 113);
        ///     border-bottom-color: rgb(248 113 113);
        /// }
        /// ```
        YRed400 => "border-y-red-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(239 68 68);
        ///     border-bottom-color: rgb(239 68 68);
        /// }
        /// ```
        YRed500 => "border-y-red-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(220 38 38);
        ///     border-bottom-color: rgb(220 38 38);
        /// }
        /// ```
        YRed600 => "border-y-red-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(185 28 28);
        ///     border-bottom-color: rgb(185 28 28);
        /// }
        /// ```
        YRed700 => "border-y-red-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(153 27 27);
        ///     border-bottom-color: rgb(153 27 27);
        /// }
        /// ```
        YRed800 => "border-y-red-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(127 29 29);
        ///     border-bottom-color: rgb(127 29 29);
        /// }
        /// ```
        YRed900 => "border-y-red-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(69 10 10);
        ///     border-bottom-color: rgb(69 10 10);
        /// }
        /// ```
        YRed950 => "border-y-red-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 247 237);
        ///     border-bottom-color: rgb(255 247 237);
        /// }
        /// ```
        YOrange50 => "border-y-orange-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 237 213);
        ///     border-bottom-color: rgb(255 237 213);
        /// }
        /// ```
        YOrange100 => "border-y-orange-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 215 170);
        ///     border-bottom-color: rgb(254 215 170);
        /// }
        /// ```
        YOrange200 => "border-y-orange-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 186 116);
        ///     border-bottom-color: rgb(253 186 116);
        /// }
        /// ```
        YOrange300 => "border-y-orange-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(251 146 60);
        ///     border-bottom-color: rgb(251 146 60);
        /// }
        /// ```
        YOrange400 => "border-y-orange-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(249 115 22);
        ///     border-bottom-color: rgb(249 115 22);
        /// }
        /// ```
        YOrange500 => "border-y-orange-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(234 88 12);
        ///     border-bottom-color: rgb(234 88 12);
        /// }
        /// ```
        YOrange600 => "border-y-orange-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(194 65 12);
        ///     border-bottom-color: rgb(194 65 12);
        /// }
        /// ```
        YOrange700 => "border-y-orange-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(154 52 18);
        ///     border-bottom-color: rgb(154 52 18);
        /// }
        /// ```
        YOrange800 => "border-y-orange-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(124 45 18);
        ///     border-bottom-color: rgb(124 45 18);
        /// }
        /// ```
        YOrange900 => "border-y-orange-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(67 20 7);
        ///     border-bottom-color: rgb(67 20 7);
        /// }
        /// ```
        YOrange950 => "border-y-orange-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 251 235);
        ///     border-bottom-color: rgb(255 251 235);
        /// }
        /// ```
        YAmber50 => "border-y-amber-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 243 199);
        ///     border-bottom-color: rgb(254 243 199);
        /// }
        /// ```
        YAmber100 => "border-y-amber-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 230 138);
        ///     border-bottom-color: rgb(253 230 138);
        /// }
        /// ```
        YAmber200 => "border-y-amber-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(252 211 77);
        ///     border-bottom-color: rgb(252 211 77);
        /// }
        /// ```
        YAmber300 => "border-y-amber-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(251 191 36);
        ///     border-bottom-color: rgb(251 191 36);
        /// }
        /// ```
        YAmber400 => "border-y-amber-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 158 11);
        ///     border-bottom-color: rgb(245 158 11);
        /// }
        /// ```
        YAmber500 => "border-y-amber-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(217 119 6);
        ///     border-bottom-color: rgb(217 119 6);
        /// }
        /// ```
        YAmber600 => "border-y-amber-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(180 83 9);
        ///     border-bottom-color: rgb(180 83 9);
        /// }
        /// ```
        YAmber700 => "border-y-amber-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(146 64 14);
        ///     border-bottom-color: rgb(146 64 14);
        /// }
        /// ```
        YAmber800 => "border-y-amber-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(120 53 15);
        ///     border-bottom-color: rgb(120 53 15);
        /// }
        /// ```
        YAmber900 => "border-y-amber-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(69 26 3);
        ///     border-bottom-color: rgb(69 26 3);
        /// }
        /// ```
        YAmber950 => "border-y-amber-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 252 232);
        ///     border-bottom-color: rgb(254 252 232);
        /// }
        /// ```
        YYellow50 => "border-y-yellow-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 249 195);
        ///     border-bottom-color: rgb(254 249 195);
        /// }
        /// ```
        YYellow100 => "border-y-yellow-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 240 138);
        ///     border-bottom-color: rgb(254 240 138);
        /// }
        /// ```
        YYellow200 => "border-y-yellow-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 224 71);
        ///     border-bottom-color: rgb(253 224 71);
        /// }
        /// ```
        YYellow300 => "border-y-yellow-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 204 21);
        ///     border-bottom-color: rgb(250 204 21);
        /// }
        /// ```
        YYellow400 => "border-y-yellow-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(234 179 8);
        ///     border-bottom-color: rgb(234 179 8);
        /// }
        /// ```
        YYellow500 => "border-y-yellow-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(202 138 4);
        ///     border-bottom-color: rgb(202 138 4);
        /// }
        /// ```
        YYellow600 => "border-y-yellow-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(161 98 7);
        ///     border-bottom-color: rgb(161 98 7);
        /// }
        /// ```
        YYellow700 => "border-y-yellow-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(133 77 14);
        ///     border-bottom-color: rgb(133 77 14);
        /// }
        /// ```
        YYellow800 => "border-y-yellow-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(113 63 18);
        ///     border-bottom-color: rgb(113 63 18);
        /// }
        /// ```
        YYellow900 => "border-y-yellow-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(66 32 6);
        ///     border-bottom-color: rgb(66 32 6);
        /// }
        /// ```
        YYellow950 => "border-y-yellow-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(247 254 231);
        ///     border-bottom-color: rgb(247 254 231);
        /// }
        /// ```
        YLime50 => "border-y-lime-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(236 252 203);
        ///     border-bottom-color: rgb(236 252 203);
        /// }
        /// ```
        YLime100 => "border-y-lime-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(217 249 157);
        ///     border-bottom-color: rgb(217 249 157);
        /// }
        /// ```
        YLime200 => "border-y-lime-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(190 242 100);
        ///     border-bottom-color: rgb(190 242 100);
        /// }
        /// ```
        YLime300 => "border-y-lime-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(163 230 53);
        ///     border-bottom-color: rgb(163 230 53);
        /// }
        /// ```
        YLime400 => "border-y-lime-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(132 204 22);
        ///     border-bottom-color: rgb(132 204 22);
        /// }
        /// ```
        YLime500 => "border-y-lime-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(101 163 13);
        ///     border-bottom-color: rgb(101 163 13);
        /// }
        /// ```
        YLime600 => "border-y-lime-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(77 124 15);
        ///     border-bottom-color: rgb(77 124 15);
        /// }
        /// ```
        YLime700 => "border-y-lime-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(63 98 18);
        ///     border-bottom-color: rgb(63 98 18);
        /// }
        /// ```
        YLime800 => "border-y-lime-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(54 83 20);
        ///     border-bottom-color: rgb(54 83 20);
        /// }
        /// ```
        YLime900 => "border-y-lime-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(26 46 5);
        ///     border-bottom-color: rgb(26 46 5);
        /// }
        /// ```
        YLime950 => "border-y-lime-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(240 253 244);
        ///     border-bottom-color: rgb(240 253 244);
        /// }
        /// ```
        YGreen50 => "border-y-green-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(220 252 231);
        ///     border-bottom-color: rgb(220 252 231);
        /// }
        /// ```
        YGreen100 => "border-y-green-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(187 247 208);
        ///     border-bottom-color: rgb(187 247 208);
        /// }
        /// ```
        YGreen200 => "border-y-green-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(134 239 172);
        ///     border-bottom-color: rgb(134 239 172);
        /// }
        /// ```
        YGreen300 => "border-y-green-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(74 222 128);
        ///     border-bottom-color: rgb(74 222 128);
        /// }
        /// ```
        YGreen400 => "border-y-green-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(34 197 94);
        ///     border-bottom-color: rgb(34 197 94);
        /// }
        /// ```
        YGreen500 => "border-y-green-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(22 163 74);
        ///     border-bottom-color: rgb(22 163 74);
        /// }
        /// ```
        YGreen600 => "border-y-green-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(21 128 61);
        ///     border-bottom-color: rgb(21 128 61);
        /// }
        /// ```
        YGreen700 => "border-y-green-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(22 101 52);
        ///     border-bottom-color: rgb(22 101 52);
        /// }
        /// ```
        YGreen800 => "border-y-green-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(20 83 45);
        ///     border-bottom-color: rgb(20 83 45);
        /// }
        /// ```
        YGreen900 => "border-y-green-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(5 46 22);
        ///     border-bottom-color: rgb(5 46 22);
        /// }
        /// ```
        YGreen950 => "border-y-green-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(236 253 245);
        ///     border-bottom-color: rgb(236 253 245);
        /// }
        /// ```
        YEmerald50 => "border-y-emerald-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(209 250 229);
        ///     border-bottom-color: rgb(209 250 229);
        /// }
        /// ```
        YEmerald100 => "border-y-emerald-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(167 243 208);
        ///     border-bottom-color: rgb(167 243 208);
        /// }
        /// ```
        YEmerald200 => "border-y-emerald-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(110 231 183);
        ///     border-bottom-color: rgb(110 231 183);
        /// }
        /// ```
        YEmerald300 => "border-y-emerald-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(52 211 153);
        ///     border-bottom-color: rgb(52 211 153);
        /// }
        /// ```
        YEmerald400 => "border-y-emerald-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(16 185 129);
        ///     border-bottom-color: rgb(16 185 129);
        /// }
        /// ```
        YEmerald500 => "border-y-emerald-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(5 150 105);
        ///     border-bottom-color: rgb(5 150 105);
        /// }
        /// ```
        YEmerald600 => "border-y-emerald-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(4 120 87);
        ///     border-bottom-color: rgb(4 120 87);
        /// }
        /// ```
        YEmerald700 => "border-y-emerald-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(6 95 70);
        ///     border-bottom-color: rgb(6 95 70);
        /// }
        /// ```
        YEmerald800 => "border-y-emerald-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(6 78 59);
        ///     border-bottom-color: rgb(6 78 59);
        /// }
        /// ```
        YEmerald900 => "border-y-emerald-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(2 44 34);
        ///     border-bottom-color: rgb(2 44 34);
        /// }
        /// ```
        YEmerald950 => "border-y-emerald-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(240 253 250);
        ///     border-bottom-color: rgb(240 253 250);
        /// }
        /// ```
        YTeal50 => "border-y-teal-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(204 251 241);
        ///     border-bottom-color: rgb(204 251 241);
        /// }
        /// ```
        YTeal100 => "border-y-teal-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(153 246 228);
        ///     border-bottom-color: rgb(153 246 228);
        /// }
        /// ```
        YTeal200 => "border-y-teal-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(94 234 212);
        ///     border-bottom-color: rgb(94 234 212);
        /// }
        /// ```
        YTeal300 => "border-y-teal-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(45 212 191);
        ///     border-bottom-color: rgb(45 212 191);
        /// }
        /// ```
        YTeal400 => "border-y-teal-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(20 184 166);
        ///     border-bottom-color: rgb(20 184 166);
        /// }
        /// ```
        YTeal500 => "border-y-teal-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(13 148 136);
        ///     border-bottom-color: rgb(13 148 136);
        /// }
        /// ```
        YTeal600 => "border-y-teal-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(15 118 110);
        ///     border-bottom-color: rgb(15 118 110);
        /// }
        /// ```
        YTeal700 => "border-y-teal-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(17 94 89);
        ///     border-bottom-color: rgb(17 94 89);
        /// }
        /// ```
        YTeal800 => "border-y-teal-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(19 78 74);
        ///     border-bottom-color: rgb(19 78 74);
        /// }
        /// ```
        YTeal900 => "border-y-teal-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(4 47 46);
        ///     border-bottom-color: rgb(4 47 46);
        /// }
        /// ```
        YTeal950 => "border-y-teal-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(236 254 255);
        ///     border-bottom-color: rgb(236 254 255);
        /// }
        /// ```
        YCyan50 => "border-y-cyan-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(207 250 254);
        ///     border-bottom-color: rgb(207 250 254);
        /// }
        /// ```
        YCyan100 => "border-y-cyan-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(165 243 252);
        ///     border-bottom-color: rgb(165 243 252);
        /// }
        /// ```
        YCyan200 => "border-y-cyan-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(103 232 249);
        ///     border-bottom-color: rgb(103 232 249);
        /// }
        /// ```
        YCyan300 => "border-y-cyan-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(34 211 238);
        ///     border-bottom-color: rgb(34 211 238);
        /// }
        /// ```
        YCyan400 => "border-y-cyan-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(6 182 212);
        ///     border-bottom-color: rgb(6 182 212);
        /// }
        /// ```
        YCyan500 => "border-y-cyan-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(8 145 178);
        ///     border-bottom-color: rgb(8 145 178);
        /// }
        /// ```
        YCyan600 => "border-y-cyan-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(14 116 144);
        ///     border-bottom-color: rgb(14 116 144);
        /// }
        /// ```
        YCyan700 => "border-y-cyan-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(21 94 117);
        ///     border-bottom-color: rgb(21 94 117);
        /// }
        /// ```
        YCyan800 => "border-y-cyan-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(22 78 99);
        ///     border-bottom-color: rgb(22 78 99);
        /// }
        /// ```
        YCyan900 => "border-y-cyan-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(8 51 68);
        ///     border-bottom-color: rgb(8 51 68);
        /// }
        /// ```
        YCyan950 => "border-y-cyan-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(240 249 255);
        ///     border-bottom-color: rgb(240 249 255);
        /// }
        /// ```
        YSky50 => "border-y-sky-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(224 242 254);
        ///     border-bottom-color: rgb(224 242 254);
        /// }
        /// ```
        YSky100 => "border-y-sky-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(186 230 253);
        ///     border-bottom-color: rgb(186 230 253);
        /// }
        /// ```
        YSky200 => "border-y-sky-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(125 211 252);
        ///     border-bottom-color: rgb(125 211 252);
        /// }
        /// ```
        YSky300 => "border-y-sky-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(56 189 248);
        ///     border-bottom-color: rgb(56 189 248);
        /// }
        /// ```
        YSky400 => "border-y-sky-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(14 165 233);
        ///     border-bottom-color: rgb(14 165 233);
        /// }
        /// ```
        YSky500 => "border-y-sky-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(2 132 199);
        ///     border-bottom-color: rgb(2 132 199);
        /// }
        /// ```
        YSky600 => "border-y-sky-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(3 105 161);
        ///     border-bottom-color: rgb(3 105 161);
        /// }
        /// ```
        YSky700 => "border-y-sky-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(7 89 133);
        ///     border-bottom-color: rgb(7 89 133);
        /// }
        /// ```
        YSky800 => "border-y-sky-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(12 74 110);
        ///     border-bottom-color: rgb(12 74 110);
        /// }
        /// ```
        YSky900 => "border-y-sky-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(8 47 73);
        ///     border-bottom-color: rgb(8 47 73);
        /// }
        /// ```
        YSky950 => "border-y-sky-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(239 246 255);
        ///     border-bottom-color: rgb(239 246 255);
        /// }
        /// ```
        YBlue50 => "border-y-blue-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(219 234 254);
        ///     border-bottom-color: rgb(219 234 254);
        /// }
        /// ```
        YBlue100 => "border-y-blue-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(191 219 254);
        ///     border-bottom-color: rgb(191 219 254);
        /// }
        /// ```
        YBlue200 => "border-y-blue-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(147 197 253);
        ///     border-bottom-color: rgb(147 197 253);
        /// }
        /// ```
        YBlue300 => "border-y-blue-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(96 165 250);
        ///     border-bottom-color: rgb(96 165 250);
        /// }
        /// ```
        YBlue400 => "border-y-blue-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(59 130 246);
        ///     border-bottom-color: rgb(59 130 246);
        /// }
        /// ```
        YBlue500 => "border-y-blue-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(37 99 235);
        ///     border-bottom-color: rgb(37 99 235);
        /// }
        /// ```
        YBlue600 => "border-y-blue-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(29 78 216);
        ///     border-bottom-color: rgb(29 78 216);
        /// }
        /// ```
        YBlue700 => "border-y-blue-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(30 64 175);
        ///     border-bottom-color: rgb(30 64 175);
        /// }
        /// ```
        YBlue800 => "border-y-blue-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(30 58 138);
        ///     border-bottom-color: rgb(30 58 138);
        /// }
        /// ```
        YBlue900 => "border-y-blue-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(23 37 84);
        ///     border-bottom-color: rgb(23 37 84);
        /// }
        /// ```
        YBlue950 => "border-y-blue-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(238 242 255);
        ///     border-bottom-color: rgb(238 242 255);
        /// }
        /// ```
        YIndigo50 => "border-y-indigo-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(224 231 255);
        ///     border-bottom-color: rgb(224 231 255);
        /// }
        /// ```
        YIndigo100 => "border-y-indigo-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(199 210 254);
        ///     border-bottom-color: rgb(199 210 254);
        /// }
        /// ```
        YIndigo200 => "border-y-indigo-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(165 180 252);
        ///     border-bottom-color: rgb(165 180 252);
        /// }
        /// ```
        YIndigo300 => "border-y-indigo-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(129 140 248);
        ///     border-bottom-color: rgb(129 140 248);
        /// }
        /// ```
        YIndigo400 => "border-y-indigo-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(99 102 241);
        ///     border-bottom-color: rgb(99 102 241);
        /// }
        /// ```
        YIndigo500 => "border-y-indigo-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(79 70 229);
        ///     border-bottom-color: rgb(79 70 229);
        /// }
        /// ```
        YIndigo600 => "border-y-indigo-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(67 56 202);
        ///     border-bottom-color: rgb(67 56 202);
        /// }
        /// ```
        YIndigo700 => "border-y-indigo-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(55 48 163);
        ///     border-bottom-color: rgb(55 48 163);
        /// }
        /// ```
        YIndigo800 => "border-y-indigo-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(49 46 129);
        ///     border-bottom-color: rgb(49 46 129);
        /// }
        /// ```
        YIndigo900 => "border-y-indigo-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(30 27 75);
        ///     border-bottom-color: rgb(30 27 75);
        /// }
        /// ```
        YIndigo950 => "border-y-indigo-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 243 255);
        ///     border-bottom-color: rgb(245 243 255);
        /// }
        /// ```
        YViolet50 => "border-y-violet-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(237 233 254);
        ///     border-bottom-color: rgb(237 233 254);
        /// }
        /// ```
        YViolet100 => "border-y-violet-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(221 214 254);
        ///     border-bottom-color: rgb(221 214 254);
        /// }
        /// ```
        YViolet200 => "border-y-violet-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(196 181 253);
        ///     border-bottom-color: rgb(196 181 253);
        /// }
        /// ```
        YViolet300 => "border-y-violet-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(167 139 250);
        ///     border-bottom-color: rgb(167 139 250);
        /// }
        /// ```
        YViolet400 => "border-y-violet-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(139 92 246);
        ///     border-bottom-color: rgb(139 92 246);
        /// }
        /// ```
        YViolet500 => "border-y-violet-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(124 58 237);
        ///     border-bottom-color: rgb(124 58 237);
        /// }
        /// ```
        YViolet600 => "border-y-violet-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(109 40 217);
        ///     border-bottom-color: rgb(109 40 217);
        /// }
        /// ```
        YViolet700 => "border-y-violet-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(91 33 182);
        ///     border-bottom-color: rgb(91 33 182);
        /// }
        /// ```
        YViolet800 => "border-y-violet-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(76 29 149);
        ///     border-bottom-color: rgb(76 29 149);
        /// }
        /// ```
        YViolet900 => "border-y-violet-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(46 16 101);
        ///     border-bottom-color: rgb(46 16 101);
        /// }
        /// ```
        YViolet950 => "border-y-violet-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 245 255);
        ///     border-bottom-color: rgb(250 245 255);
        /// }
        /// ```
        YPurple50 => "border-y-purple-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(243 232 255);
        ///     border-bottom-color: rgb(243 232 255);
        /// }
        /// ```
        YPurple100 => "border-y-purple-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(233 213 255);
        ///     border-bottom-color: rgb(233 213 255);
        /// }
        /// ```
        YPurple200 => "border-y-purple-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(216 180 254);
        ///     border-bottom-color: rgb(216 180 254);
        /// }
        /// ```
        YPurple300 => "border-y-purple-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(192 132 252);
        ///     border-bottom-color: rgb(192 132 252);
        /// }
        /// ```
        YPurple400 => "border-y-purple-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(168 85 247);
        ///     border-bottom-color: rgb(168 85 247);
        /// }
        /// ```
        YPurple500 => "border-y-purple-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(147 51 234);
        ///     border-bottom-color: rgb(147 51 234);
        /// }
        /// ```
        YPurple600 => "border-y-purple-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(126 34 206);
        ///     border-bottom-color: rgb(126 34 206);
        /// }
        /// ```
        YPurple700 => "border-y-purple-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(107 33 168);
        ///     border-bottom-color: rgb(107 33 168);
        /// }
        /// ```
        YPurple800 => "border-y-purple-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(88 28 135);
        ///     border-bottom-color: rgb(88 28 135);
        /// }
        /// ```
        YPurple900 => "border-y-purple-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(59 7 100);
        ///     border-bottom-color: rgb(59 7 100);
        /// }
        /// ```
        YPurple950 => "border-y-purple-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 244 255);
        ///     border-bottom-color: rgb(253 244 255);
        /// }
        /// ```
        YFuchsia50 => "border-y-fuchsia-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 232 255);
        ///     border-bottom-color: rgb(250 232 255);
        /// }
        /// ```
        YFuchsia100 => "border-y-fuchsia-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 208 254);
        ///     border-bottom-color: rgb(245 208 254);
        /// }
        /// ```
        YFuchsia200 => "border-y-fuchsia-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(240 171 252);
        ///     border-bottom-color: rgb(240 171 252);
        /// }
        /// ```
        YFuchsia300 => "border-y-fuchsia-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(232 121 249);
        ///     border-bottom-color: rgb(232 121 249);
        /// }
        /// ```
        YFuchsia400 => "border-y-fuchsia-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(217 70 239);
        ///     border-bottom-color: rgb(217 70 239);
        /// }
        /// ```
        YFuchsia500 => "border-y-fuchsia-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(192 38 211);
        ///     border-bottom-color: rgb(192 38 211);
        /// }
        /// ```
        YFuchsia600 => "border-y-fuchsia-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(162 28 175);
        ///     border-bottom-color: rgb(162 28 175);
        /// }
        /// ```
        YFuchsia700 => "border-y-fuchsia-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(134 25 143);
        ///     border-bottom-color: rgb(134 25 143);
        /// }
        /// ```
        YFuchsia800 => "border-y-fuchsia-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(112 26 117);
        ///     border-bottom-color: rgb(112 26 117);
        /// }
        /// ```
        YFuchsia900 => "border-y-fuchsia-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(74 4 78);
        ///     border-bottom-color: rgb(74 4 78);
        /// }
        /// ```
        YFuchsia950 => "border-y-fuchsia-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 242 248);
        ///     border-bottom-color: rgb(253 242 248);
        /// }
        /// ```
        YPink50 => "border-y-pink-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(252 231 243);
        ///     border-bottom-color: rgb(252 231 243);
        /// }
        /// ```
        YPink100 => "border-y-pink-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(251 207 232);
        ///     border-bottom-color: rgb(251 207 232);
        /// }
        /// ```
        YPink200 => "border-y-pink-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(249 168 212);
        ///     border-bottom-color: rgb(249 168 212);
        /// }
        /// ```
        YPink300 => "border-y-pink-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(244 114 182);
        ///     border-bottom-color: rgb(244 114 182);
        /// }
        /// ```
        YPink400 => "border-y-pink-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(236 72 153);
        ///     border-bottom-color: rgb(236 72 153);
        /// }
        /// ```
        YPink500 => "border-y-pink-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(219 39 119);
        ///     border-bottom-color: rgb(219 39 119);
        /// }
        /// ```
        YPink600 => "border-y-pink-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(190 24 93);
        ///     border-bottom-color: rgb(190 24 93);
        /// }
        /// ```
        YPink700 => "border-y-pink-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(157 23 77);
        ///     border-bottom-color: rgb(157 23 77);
        /// }
        /// ```
        YPink800 => "border-y-pink-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(131 24 67);
        ///     border-bottom-color: rgb(131 24 67);
        /// }
        /// ```
        YPink900 => "border-y-pink-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(80 7 36);
        ///     border-bottom-color: rgb(80 7 36);
        /// }
        /// ```
        YPink950 => "border-y-pink-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 241 242);
        ///     border-bottom-color: rgb(255 241 242);
        /// }
        /// ```
        YRose50 => "border-y-rose-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 228 230);
        ///     border-bottom-color: rgb(255 228 230);
        /// }
        /// ```
        YRose100 => "border-y-rose-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 205 211);
        ///     border-bottom-color: rgb(254 205 211);
        /// }
        /// ```
        YRose200 => "border-y-rose-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 164 175);
        ///     border-bottom-color: rgb(253 164 175);
        /// }
        /// ```
        YRose300 => "border-y-rose-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(251 113 133);
        ///     border-bottom-color: rgb(251 113 133);
        /// }
        /// ```
        YRose400 => "border-y-rose-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(244 63 94);
        ///     border-bottom-color: rgb(244 63 94);
        /// }
        /// ```
        YRose500 => "border-y-rose-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(225 29 72);
        ///     border-bottom-color: rgb(225 29 72);
        /// }
        /// ```
        YRose600 => "border-y-rose-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(190 18 60);
        ///     border-bottom-color: rgb(190 18 60);
        /// }
        /// ```
        YRose700 => "border-y-rose-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(159 18 57);
        ///     border-bottom-color: rgb(159 18 57);
        /// }
        /// ```
        YRose800 => "border-y-rose-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(136 19 55);
        ///     border-bottom-color: rgb(136 19 55);
        /// }
        /// ```
        YRose900 => "border-y-rose-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(76 5 25);
        ///     border-bottom-color: rgb(76 5 25);
        /// }
        /// ```
        YRose950 => "border-y-rose-950",
        /// ```css
        /// {
        ///     border-inline-start-color: inherit;
        /// }
        /// ```
        SInherit => "border-s-inherit",
        /// ```css
        /// {
        ///     border-inline-start-color: currentColor;
        /// }
        /// ```
        SCurrent => "border-s-current",
        /// ```css
        /// {
        ///     border-inline-start-color: transparent;
        /// }
        /// ```
        STransparent => "border-s-transparent",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(0 0 0);
        /// }
        /// ```
        SBlack => "border-s-black",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(255 255 255);
        /// }
        /// ```
        SWhite => "border-s-white",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(248 250 252);
        /// }
        /// ```
        SSlate50 => "border-s-slate-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(241 245 249);
        /// }
        /// ```
        SSlate100 => "border-s-slate-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(226 232 240);
        /// }
        /// ```
        SSlate200 => "border-s-slate-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(203 213 225);
        /// }
        /// ```
        SSlate300 => "border-s-slate-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(148 163 184);
        /// }
        /// ```
        SSlate400 => "border-s-slate-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(100 116 139);
        /// }
        /// ```
        SSlate500 => "border-s-slate-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(71 85 105);
        /// }
        /// ```
        SSlate600 => "border-s-slate-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(51 65 85);
        /// }
        /// ```
        SSlate700 => "border-s-slate-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(30 41 59);
        /// }
        /// ```
        SSlate800 => "border-s-slate-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(15 23 42);
        /// }
        /// ```
        SSlate900 => "border-s-slate-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(2 6 23);
        /// }
        /// ```
        SSlate950 => "border-s-slate-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(249 250 251);
        /// }
        /// ```
        SGray50 => "border-s-gray-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(243 244 246);
        /// }
        /// ```
        SGray100 => "border-s-gray-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(229 231 235);
        /// }
        /// ```
        SGray200 => "border-s-gray-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(209 213 219);
        /// }
        /// ```
        SGray300 => "border-s-gray-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(156 163 175);
        /// }
        /// ```
        SGray400 => "border-s-gray-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(107 114 128);
        /// }
        /// ```
        SGray500 => "border-s-gray-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(75 85 99);
        /// }
        /// ```
        SGray600 => "border-s-gray-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(55 65 81);
        /// }
        /// ```
        SGray700 => "border-s-gray-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(31 41 55);
        /// }
        /// ```
        SGray800 => "border-s-gray-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(17 24 39);
        /// }
        /// ```
        SGray900 => "border-s-gray-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(3 7 18);
        /// }
        /// ```
        SGray950 => "border-s-gray-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(250 250 250);
        /// }
        /// ```
        SZinc50 => "border-s-zinc-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(244 244 245);
        /// }
        /// ```
        SZinc100 => "border-s-zinc-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(228 228 231);
        /// }
        /// ```
        SZinc200 => "border-s-zinc-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(212 212 216);
        /// }
        /// ```
        SZinc300 => "border-s-zinc-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(161 161 170);
        /// }
        /// ```
        SZinc400 => "border-s-zinc-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(113 113 122);
        /// }
        /// ```
        SZinc500 => "border-s-zinc-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(82 82 91);
        /// }
        /// ```
        SZinc600 => "border-s-zinc-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(63 63 70);
        /// }
        /// ```
        SZinc700 => "border-s-zinc-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(39 39 42);
        /// }
        /// ```
        SZinc800 => "border-s-zinc-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(24 24 27);
        /// }
        /// ```
        SZinc900 => "border-s-zinc-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(9 9 11);
        /// }
        /// ```
        SZinc950 => "border-s-zinc-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(250 250 250);
        /// }
        /// ```
        SNeutral50 => "border-s-neutral-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(245 245 245);
        /// }
        /// ```
        SNeutral100 => "border-s-neutral-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(229 229 229);
        /// }
        /// ```
        SNeutral200 => "border-s-neutral-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(212 212 212);
        /// }
        /// ```
        SNeutral300 => "border-s-neutral-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(163 163 163);
        /// }
        /// ```
        SNeutral400 => "border-s-neutral-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(115 115 115);
        /// }
        /// ```
        SNeutral500 => "border-s-neutral-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(82 82 82);
        /// }
        /// ```
        SNeutral600 => "border-s-neutral-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(64 64 64);
        /// }
        /// ```
        SNeutral700 => "border-s-neutral-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(38 38 38);
        /// }
        /// ```
        SNeutral800 => "border-s-neutral-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(23 23 23);
        /// }
        /// ```
        SNeutral900 => "border-s-neutral-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(10 10 10);
        /// }
        /// ```
        SNeutral950 => "border-s-neutral-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(250 250 249);
        /// }
        /// ```
        SStone50 => "border-s-stone-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(245 245 244);
        /// }
        /// ```
        SStone100 => "border-s-stone-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(231 229 228);
        /// }
        /// ```
        SStone200 => "border-s-stone-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(214 211 209);
        /// }
        /// ```
        SStone300 => "border-s-stone-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(168 162 158);
        /// }
        /// ```
        SStone400 => "border-s-stone-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(120 113 108);
        /// }
        /// ```
        SStone500 => "border-s-stone-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(87 83 78);
        /// }
        /// ```
        SStone600 => "border-s-stone-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(68 64 60);
        /// }
        /// ```
        SStone700 => "border-s-stone-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(41 37 36);
        /// }
        /// ```
        SStone800 => "border-s-stone-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(28 25 23);
        /// }
        /// ```
        SStone900 => "border-s-stone-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(12 10 9);
        /// }
        /// ```
        SStone950 => "border-s-stone-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 242 242);
        /// }
        /// ```
        SRed50 => "border-s-red-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 226 226);
        /// }
        /// ```
        SRed100 => "border-s-red-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 202 202);
        /// }
        /// ```
        SRed200 => "border-s-red-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(252 165 165);
        /// }
        /// ```
        SRed300 => "border-s-red-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(248 113 113);
        /// }
        /// ```
        SRed400 => "border-s-red-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(239 68 68);
        /// }
        /// ```
        SRed500 => "border-s-red-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(220 38 38);
        /// }
        /// ```
        SRed600 => "border-s-red-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(185 28 28);
        /// }
        /// ```
        SRed700 => "border-s-red-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(153 27 27);
        /// }
        /// ```
        SRed800 => "border-s-red-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(127 29 29);
        /// }
        /// ```
        SRed900 => "border-s-red-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(69 10 10);
        /// }
        /// ```
        SRed950 => "border-s-red-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(255 247 237);
        /// }
        /// ```
        SOrange50 => "border-s-orange-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(255 237 213);
        /// }
        /// ```
        SOrange100 => "border-s-orange-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 215 170);
        /// }
        /// ```
        SOrange200 => "border-s-orange-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(253 186 116);
        /// }
        /// ```
        SOrange300 => "border-s-orange-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(251 146 60);
        /// }
        /// ```
        SOrange400 => "border-s-orange-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(249 115 22);
        /// }
        /// ```
        SOrange500 => "border-s-orange-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(234 88 12);
        /// }
        /// ```
        SOrange600 => "border-s-orange-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(194 65 12);
        /// }
        /// ```
        SOrange700 => "border-s-orange-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(154 52 18);
        /// }
        /// ```
        SOrange800 => "border-s-orange-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(124 45 18);
        /// }
        /// ```
        SOrange900 => "border-s-orange-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(67 20 7);
        /// }
        /// ```
        SOrange950 => "border-s-orange-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(255 251 235);
        /// }
        /// ```
        SAmber50 => "border-s-amber-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 243 199);
        /// }
        /// ```
        SAmber100 => "border-s-amber-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(253 230 138);
        /// }
        /// ```
        SAmber200 => "border-s-amber-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(252 211 77);
        /// }
        /// ```
        SAmber300 => "border-s-amber-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(251 191 36);
        /// }
        /// ```
        SAmber400 => "border-s-amber-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(245 158 11);
        /// }
        /// ```
        SAmber500 => "border-s-amber-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(217 119 6);
        /// }
        /// ```
        SAmber600 => "border-s-amber-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(180 83 9);
        /// }
        /// ```
        SAmber700 => "border-s-amber-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(146 64 14);
        /// }
        /// ```
        SAmber800 => "border-s-amber-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(120 53 15);
        /// }
        /// ```
        SAmber900 => "border-s-amber-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(69 26 3);
        /// }
        /// ```
        SAmber950 => "border-s-amber-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 252 232);
        /// }
        /// ```
        SYellow50 => "border-s-yellow-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 249 195);
        /// }
        /// ```
        SYellow100 => "border-s-yellow-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 240 138);
        /// }
        /// ```
        SYellow200 => "border-s-yellow-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(253 224 71);
        /// }
        /// ```
        SYellow300 => "border-s-yellow-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(250 204 21);
        /// }
        /// ```
        SYellow400 => "border-s-yellow-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(234 179 8);
        /// }
        /// ```
        SYellow500 => "border-s-yellow-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(202 138 4);
        /// }
        /// ```
        SYellow600 => "border-s-yellow-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(161 98 7);
        /// }
        /// ```
        SYellow700 => "border-s-yellow-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(133 77 14);
        /// }
        /// ```
        SYellow800 => "border-s-yellow-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(113 63 18);
        /// }
        /// ```
        SYellow900 => "border-s-yellow-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(66 32 6);
        /// }
        /// ```
        SYellow950 => "border-s-yellow-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(247 254 231);
        /// }
        /// ```
        SLime50 => "border-s-lime-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(236 252 203);
        /// }
        /// ```
        SLime100 => "border-s-lime-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(217 249 157);
        /// }
        /// ```
        SLime200 => "border-s-lime-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(190 242 100);
        /// }
        /// ```
        SLime300 => "border-s-lime-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(163 230 53);
        /// }
        /// ```
        SLime400 => "border-s-lime-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(132 204 22);
        /// }
        /// ```
        SLime500 => "border-s-lime-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(101 163 13);
        /// }
        /// ```
        SLime600 => "border-s-lime-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(77 124 15);
        /// }
        /// ```
        SLime700 => "border-s-lime-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(63 98 18);
        /// }
        /// ```
        SLime800 => "border-s-lime-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(54 83 20);
        /// }
        /// ```
        SLime900 => "border-s-lime-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(26 46 5);
        /// }
        /// ```
        SLime950 => "border-s-lime-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(240 253 244);
        /// }
        /// ```
        SGreen50 => "border-s-green-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(220 252 231);
        /// }
        /// ```
        SGreen100 => "border-s-green-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(187 247 208);
        /// }
        /// ```
        SGreen200 => "border-s-green-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(134 239 172);
        /// }
        /// ```
        SGreen300 => "border-s-green-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(74 222 128);
        /// }
        /// ```
        SGreen400 => "border-s-green-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(34 197 94);
        /// }
        /// ```
        SGreen500 => "border-s-green-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(22 163 74);
        /// }
        /// ```
        SGreen600 => "border-s-green-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(21 128 61);
        /// }
        /// ```
        SGreen700 => "border-s-green-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(22 101 52);
        /// }
        /// ```
        SGreen800 => "border-s-green-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(20 83 45);
        /// }
        /// ```
        SGreen900 => "border-s-green-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(5 46 22);
        /// }
        /// ```
        SGreen950 => "border-s-green-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(236 253 245);
        /// }
        /// ```
        SEmerald50 => "border-s-emerald-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(209 250 229);
        /// }
        /// ```
        SEmerald100 => "border-s-emerald-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(167 243 208);
        /// }
        /// ```
        SEmerald200 => "border-s-emerald-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(110 231 183);
        /// }
        /// ```
        SEmerald300 => "border-s-emerald-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(52 211 153);
        /// }
        /// ```
        SEmerald400 => "border-s-emerald-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(16 185 129);
        /// }
        /// ```
        SEmerald500 => "border-s-emerald-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(5 150 105);
        /// }
        /// ```
        SEmerald600 => "border-s-emerald-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(4 120 87);
        /// }
        /// ```
        SEmerald700 => "border-s-emerald-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(6 95 70);
        /// }
        /// ```
        SEmerald800 => "border-s-emerald-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(6 78 59);
        /// }
        /// ```
        SEmerald900 => "border-s-emerald-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(2 44 34);
        /// }
        /// ```
        SEmerald950 => "border-s-emerald-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(240 253 250);
        /// }
        /// ```
        STeal50 => "border-s-teal-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(204 251 241);
        /// }
        /// ```
        STeal100 => "border-s-teal-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(153 246 228);
        /// }
        /// ```
        STeal200 => "border-s-teal-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(94 234 212);
        /// }
        /// ```
        STeal300 => "border-s-teal-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(45 212 191);
        /// }
        /// ```
        STeal400 => "border-s-teal-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(20 184 166);
        /// }
        /// ```
        STeal500 => "border-s-teal-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(13 148 136);
        /// }
        /// ```
        STeal600 => "border-s-teal-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(15 118 110);
        /// }
        /// ```
        STeal700 => "border-s-teal-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(17 94 89);
        /// }
        /// ```
        STeal800 => "border-s-teal-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(19 78 74);
        /// }
        /// ```
        STeal900 => "border-s-teal-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(4 47 46);
        /// }
        /// ```
        STeal950 => "border-s-teal-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(236 254 255);
        /// }
        /// ```
        SCyan50 => "border-s-cyan-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(207 250 254);
        /// }
        /// ```
        SCyan100 => "border-s-cyan-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(165 243 252);
        /// }
        /// ```
        SCyan200 => "border-s-cyan-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(103 232 249);
        /// }
        /// ```
        SCyan300 => "border-s-cyan-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(34 211 238);
        /// }
        /// ```
        SCyan400 => "border-s-cyan-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(6 182 212);
        /// }
        /// ```
        SCyan500 => "border-s-cyan-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(8 145 178);
        /// }
        /// ```
        SCyan600 => "border-s-cyan-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(14 116 144);
        /// }
        /// ```
        SCyan700 => "border-s-cyan-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(21 94 117);
        /// }
        /// ```
        SCyan800 => "border-s-cyan-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(22 78 99);
        /// }
        /// ```
        SCyan900 => "border-s-cyan-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(8 51 68);
        /// }
        /// ```
        SCyan950 => "border-s-cyan-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(240 249 255);
        /// }
        /// ```
        SSky50 => "border-s-sky-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(224 242 254);
        /// }
        /// ```
        SSky100 => "border-s-sky-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(186 230 253);
        /// }
        /// ```
        SSky200 => "border-s-sky-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(125 211 252);
        /// }
        /// ```
        SSky300 => "border-s-sky-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(56 189 248);
        /// }
        /// ```
        SSky400 => "border-s-sky-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(14 165 233);
        /// }
        /// ```
        SSky500 => "border-s-sky-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(2 132 199);
        /// }
        /// ```
        SSky600 => "border-s-sky-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(3 105 161);
        /// }
        /// ```
        SSky700 => "border-s-sky-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(7 89 133);
        /// }
        /// ```
        SSky800 => "border-s-sky-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(12 74 110);
        /// }
        /// ```
        SSky900 => "border-s-sky-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(8 47 73);
        /// }
        /// ```
        SSky950 => "border-s-sky-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(239 246 255);
        /// }
        /// ```
        SBlue50 => "border-s-blue-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(219 234 254);
        /// }
        /// ```
        SBlue100 => "border-s-blue-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(191 219 254);
        /// }
        /// ```
        SBlue200 => "border-s-blue-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(147 197 253);
        /// }
        /// ```
        SBlue300 => "border-s-blue-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(96 165 250);
        /// }
        /// ```
        SBlue400 => "border-s-blue-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(59 130 246);
        /// }
        /// ```
        SBlue500 => "border-s-blue-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(37 99 235);
        /// }
        /// ```
        SBlue600 => "border-s-blue-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(29 78 216);
        /// }
        /// ```
        SBlue700 => "border-s-blue-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(30 64 175);
        /// }
        /// ```
        SBlue800 => "border-s-blue-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(30 58 138);
        /// }
        /// ```
        SBlue900 => "border-s-blue-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(23 37 84);
        /// }
        /// ```
        SBlue950 => "border-s-blue-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(238 242 255);
        /// }
        /// ```
        SIndigo50 => "border-s-indigo-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(224 231 255);
        /// }
        /// ```
        SIndigo100 => "border-s-indigo-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(199 210 254);
        /// }
        /// ```
        SIndigo200 => "border-s-indigo-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(165 180 252);
        /// }
        /// ```
        SIndigo300 => "border-s-indigo-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(129 140 248);
        /// }
        /// ```
        SIndigo400 => "border-s-indigo-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(99 102 241);
        /// }
        /// ```
        SIndigo500 => "border-s-indigo-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(79 70 229);
        /// }
        /// ```
        SIndigo600 => "border-s-indigo-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(67 56 202);
        /// }
        /// ```
        SIndigo700 => "border-s-indigo-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(55 48 163);
        /// }
        /// ```
        SIndigo800 => "border-s-indigo-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(49 46 129);
        /// }
        /// ```
        SIndigo900 => "border-s-indigo-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(30 27 75);
        /// }
        /// ```
        SIndigo950 => "border-s-indigo-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(245 243 255);
        /// }
        /// ```
        SViolet50 => "border-s-violet-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(237 233 254);
        /// }
        /// ```
        SViolet100 => "border-s-violet-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(221 214 254);
        /// }
        /// ```
        SViolet200 => "border-s-violet-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(196 181 253);
        /// }
        /// ```
        SViolet300 => "border-s-violet-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(167 139 250);
        /// }
        /// ```
        SViolet400 => "border-s-violet-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(139 92 246);
        /// }
        /// ```
        SViolet500 => "border-s-violet-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(124 58 237);
        /// }
        /// ```
        SViolet600 => "border-s-violet-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(109 40 217);
        /// }
        /// ```
        SViolet700 => "border-s-violet-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(91 33 182);
        /// }
        /// ```
        SViolet800 => "border-s-violet-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(76 29 149);
        /// }
        /// ```
        SViolet900 => "border-s-violet-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(46 16 101);
        /// }
        /// ```
        SViolet950 => "border-s-violet-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(250 245 255);
        /// }
        /// ```
        SPurple50 => "border-s-purple-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(243 232 255);
        /// }
        /// ```
        SPurple100 => "border-s-purple-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(233 213 255);
        /// }
        /// ```
        SPurple200 => "border-s-purple-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(216 180 254);
        /// }
        /// ```
        SPurple300 => "border-s-purple-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(192 132 252);
        /// }
        /// ```
        SPurple400 => "border-s-purple-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(168 85 247);
        /// }
        /// ```
        SPurple500 => "border-s-purple-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(147 51 234);
        /// }
        /// ```
        SPurple600 => "border-s-purple-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(126 34 206);
        /// }
        /// ```
        SPurple700 => "border-s-purple-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(107 33 168);
        /// }
        /// ```
        SPurple800 => "border-s-purple-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(88 28 135);
        /// }
        /// ```
        SPurple900 => "border-s-purple-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(59 7 100);
        /// }
        /// ```
        SPurple950 => "border-s-purple-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(253 244 255);
        /// }
        /// ```
        SFuchsia50 => "border-s-fuchsia-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(250 232 255);
        /// }
        /// ```
        SFuchsia100 => "border-s-fuchsia-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(245 208 254);
        /// }
        /// ```
        SFuchsia200 => "border-s-fuchsia-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(240 171 252);
        /// }
        /// ```
        SFuchsia300 => "border-s-fuchsia-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(232 121 249);
        /// }
        /// ```
        SFuchsia400 => "border-s-fuchsia-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(217 70 239);
        /// }
        /// ```
        SFuchsia500 => "border-s-fuchsia-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(192 38 211);
        /// }
        /// ```
        SFuchsia600 => "border-s-fuchsia-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(162 28 175);
        /// }
        /// ```
        SFuchsia700 => "border-s-fuchsia-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(134 25 143);
        /// }
        /// ```
        SFuchsia800 => "border-s-fuchsia-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(112 26 117);
        /// }
        /// ```
        SFuchsia900 => "border-s-fuchsia-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(74 4 78);
        /// }
        /// ```
        SFuchsia950 => "border-s-fuchsia-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(253 242 248);
        /// }
        /// ```
        SPink50 => "border-s-pink-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(252 231 243);
        /// }
        /// ```
        SPink100 => "border-s-pink-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(251 207 232);
        /// }
        /// ```
        SPink200 => "border-s-pink-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(249 168 212);
        /// }
        /// ```
        SPink300 => "border-s-pink-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(244 114 182);
        /// }
        /// ```
        SPink400 => "border-s-pink-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(236 72 153);
        /// }
        /// ```
        SPink500 => "border-s-pink-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(219 39 119);
        /// }
        /// ```
        SPink600 => "border-s-pink-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(190 24 93);
        /// }
        /// ```
        SPink700 => "border-s-pink-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(157 23 77);
        /// }
        /// ```
        SPink800 => "border-s-pink-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(131 24 67);
        /// }
        /// ```
        SPink900 => "border-s-pink-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(80 7 36);
        /// }
        /// ```
        SPink950 => "border-s-pink-950",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(255 241 242);
        /// }
        /// ```
        SRose50 => "border-s-rose-50",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(255 228 230);
        /// }
        /// ```
        SRose100 => "border-s-rose-100",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(254 205 211);
        /// }
        /// ```
        SRose200 => "border-s-rose-200",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(253 164 175);
        /// }
        /// ```
        SRose300 => "border-s-rose-300",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(251 113 133);
        /// }
        /// ```
        SRose400 => "border-s-rose-400",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(244 63 94);
        /// }
        /// ```
        SRose500 => "border-s-rose-500",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(225 29 72);
        /// }
        /// ```
        SRose600 => "border-s-rose-600",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(190 18 60);
        /// }
        /// ```
        SRose700 => "border-s-rose-700",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(159 18 57);
        /// }
        /// ```
        SRose800 => "border-s-rose-800",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(136 19 55);
        /// }
        /// ```
        SRose900 => "border-s-rose-900",
        /// ```css
        /// {
        ///     border-inline-start-color: rgb(76 5 25);
        /// }
        /// ```
        SRose950 => "border-s-rose-950",
        /// ```css
        /// {
        ///     border-inline-end-color: inherit;
        /// }
        /// ```
        EInherit => "border-e-inherit",
        /// ```css
        /// {
        ///     border-inline-end-color: currentColor;
        /// }
        /// ```
        ECurrent => "border-e-current",
        /// ```css
        /// {
        ///     border-inline-end-color: transparent;
        /// }
        /// ```
        ETransparent => "border-e-transparent",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(0 0 0);
        /// }
        /// ```
        EBlack => "border-e-black",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(255 255 255);
        /// }
        /// ```
        EWhite => "border-e-white",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(248 250 252);
        /// }
        /// ```
        ESlate50 => "border-e-slate-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(241 245 249);
        /// }
        /// ```
        ESlate100 => "border-e-slate-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(226 232 240);
        /// }
        /// ```
        ESlate200 => "border-e-slate-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(203 213 225);
        /// }
        /// ```
        ESlate300 => "border-e-slate-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(148 163 184);
        /// }
        /// ```
        ESlate400 => "border-e-slate-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(100 116 139);
        /// }
        /// ```
        ESlate500 => "border-e-slate-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(71 85 105);
        /// }
        /// ```
        ESlate600 => "border-e-slate-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(51 65 85);
        /// }
        /// ```
        ESlate700 => "border-e-slate-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(30 41 59);
        /// }
        /// ```
        ESlate800 => "border-e-slate-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(15 23 42);
        /// }
        /// ```
        ESlate900 => "border-e-slate-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(2 6 23);
        /// }
        /// ```
        ESlate950 => "border-e-slate-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(249 250 251);
        /// }
        /// ```
        EGray50 => "border-e-gray-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(243 244 246);
        /// }
        /// ```
        EGray100 => "border-e-gray-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(229 231 235);
        /// }
        /// ```
        EGray200 => "border-e-gray-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(209 213 219);
        /// }
        /// ```
        EGray300 => "border-e-gray-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(156 163 175);
        /// }
        /// ```
        EGray400 => "border-e-gray-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(107 114 128);
        /// }
        /// ```
        EGray500 => "border-e-gray-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(75 85 99);
        /// }
        /// ```
        EGray600 => "border-e-gray-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(55 65 81);
        /// }
        /// ```
        EGray700 => "border-e-gray-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(31 41 55);
        /// }
        /// ```
        EGray800 => "border-e-gray-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(17 24 39);
        /// }
        /// ```
        EGray900 => "border-e-gray-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(3 7 18);
        /// }
        /// ```
        EGray950 => "border-e-gray-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(250 250 250);
        /// }
        /// ```
        EZinc50 => "border-e-zinc-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(244 244 245);
        /// }
        /// ```
        EZinc100 => "border-e-zinc-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(228 228 231);
        /// }
        /// ```
        EZinc200 => "border-e-zinc-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(212 212 216);
        /// }
        /// ```
        EZinc300 => "border-e-zinc-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(161 161 170);
        /// }
        /// ```
        EZinc400 => "border-e-zinc-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(113 113 122);
        /// }
        /// ```
        EZinc500 => "border-e-zinc-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(82 82 91);
        /// }
        /// ```
        EZinc600 => "border-e-zinc-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(63 63 70);
        /// }
        /// ```
        EZinc700 => "border-e-zinc-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(39 39 42);
        /// }
        /// ```
        EZinc800 => "border-e-zinc-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(24 24 27);
        /// }
        /// ```
        EZinc900 => "border-e-zinc-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(9 9 11);
        /// }
        /// ```
        EZinc950 => "border-e-zinc-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(250 250 250);
        /// }
        /// ```
        ENeutral50 => "border-e-neutral-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(245 245 245);
        /// }
        /// ```
        ENeutral100 => "border-e-neutral-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(229 229 229);
        /// }
        /// ```
        ENeutral200 => "border-e-neutral-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(212 212 212);
        /// }
        /// ```
        ENeutral300 => "border-e-neutral-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(163 163 163);
        /// }
        /// ```
        ENeutral400 => "border-e-neutral-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(115 115 115);
        /// }
        /// ```
        ENeutral500 => "border-e-neutral-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(82 82 82);
        /// }
        /// ```
        ENeutral600 => "border-e-neutral-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(64 64 64);
        /// }
        /// ```
        ENeutral700 => "border-e-neutral-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(38 38 38);
        /// }
        /// ```
        ENeutral800 => "border-e-neutral-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(23 23 23);
        /// }
        /// ```
        ENeutral900 => "border-e-neutral-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(10 10 10);
        /// }
        /// ```
        ENeutral950 => "border-e-neutral-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(250 250 249);
        /// }
        /// ```
        EStone50 => "border-e-stone-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(245 245 244);
        /// }
        /// ```
        EStone100 => "border-e-stone-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(231 229 228);
        /// }
        /// ```
        EStone200 => "border-e-stone-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(214 211 209);
        /// }
        /// ```
        EStone300 => "border-e-stone-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(168 162 158);
        /// }
        /// ```
        EStone400 => "border-e-stone-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(120 113 108);
        /// }
        /// ```
        EStone500 => "border-e-stone-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(87 83 78);
        /// }
        /// ```
        EStone600 => "border-e-stone-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(68 64 60);
        /// }
        /// ```
        EStone700 => "border-e-stone-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(41 37 36);
        /// }
        /// ```
        EStone800 => "border-e-stone-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(28 25 23);
        /// }
        /// ```
        EStone900 => "border-e-stone-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(12 10 9);
        /// }
        /// ```
        EStone950 => "border-e-stone-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 242 242);
        /// }
        /// ```
        ERed50 => "border-e-red-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 226 226);
        /// }
        /// ```
        ERed100 => "border-e-red-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 202 202);
        /// }
        /// ```
        ERed200 => "border-e-red-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(252 165 165);
        /// }
        /// ```
        ERed300 => "border-e-red-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(248 113 113);
        /// }
        /// ```
        ERed400 => "border-e-red-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(239 68 68);
        /// }
        /// ```
        ERed500 => "border-e-red-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(220 38 38);
        /// }
        /// ```
        ERed600 => "border-e-red-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(185 28 28);
        /// }
        /// ```
        ERed700 => "border-e-red-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(153 27 27);
        /// }
        /// ```
        ERed800 => "border-e-red-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(127 29 29);
        /// }
        /// ```
        ERed900 => "border-e-red-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(69 10 10);
        /// }
        /// ```
        ERed950 => "border-e-red-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(255 247 237);
        /// }
        /// ```
        EOrange50 => "border-e-orange-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(255 237 213);
        /// }
        /// ```
        EOrange100 => "border-e-orange-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 215 170);
        /// }
        /// ```
        EOrange200 => "border-e-orange-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(253 186 116);
        /// }
        /// ```
        EOrange300 => "border-e-orange-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(251 146 60);
        /// }
        /// ```
        EOrange400 => "border-e-orange-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(249 115 22);
        /// }
        /// ```
        EOrange500 => "border-e-orange-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(234 88 12);
        /// }
        /// ```
        EOrange600 => "border-e-orange-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(194 65 12);
        /// }
        /// ```
        EOrange700 => "border-e-orange-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(154 52 18);
        /// }
        /// ```
        EOrange800 => "border-e-orange-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(124 45 18);
        /// }
        /// ```
        EOrange900 => "border-e-orange-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(67 20 7);
        /// }
        /// ```
        EOrange950 => "border-e-orange-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(255 251 235);
        /// }
        /// ```
        EAmber50 => "border-e-amber-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 243 199);
        /// }
        /// ```
        EAmber100 => "border-e-amber-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(253 230 138);
        /// }
        /// ```
        EAmber200 => "border-e-amber-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(252 211 77);
        /// }
        /// ```
        EAmber300 => "border-e-amber-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(251 191 36);
        /// }
        /// ```
        EAmber400 => "border-e-amber-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(245 158 11);
        /// }
        /// ```
        EAmber500 => "border-e-amber-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(217 119 6);
        /// }
        /// ```
        EAmber600 => "border-e-amber-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(180 83 9);
        /// }
        /// ```
        EAmber700 => "border-e-amber-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(146 64 14);
        /// }
        /// ```
        EAmber800 => "border-e-amber-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(120 53 15);
        /// }
        /// ```
        EAmber900 => "border-e-amber-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(69 26 3);
        /// }
        /// ```
        EAmber950 => "border-e-amber-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 252 232);
        /// }
        /// ```
        EYellow50 => "border-e-yellow-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 249 195);
        /// }
        /// ```
        EYellow100 => "border-e-yellow-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 240 138);
        /// }
        /// ```
        EYellow200 => "border-e-yellow-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(253 224 71);
        /// }
        /// ```
        EYellow300 => "border-e-yellow-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(250 204 21);
        /// }
        /// ```
        EYellow400 => "border-e-yellow-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(234 179 8);
        /// }
        /// ```
        EYellow500 => "border-e-yellow-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(202 138 4);
        /// }
        /// ```
        EYellow600 => "border-e-yellow-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(161 98 7);
        /// }
        /// ```
        EYellow700 => "border-e-yellow-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(133 77 14);
        /// }
        /// ```
        EYellow800 => "border-e-yellow-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(113 63 18);
        /// }
        /// ```
        EYellow900 => "border-e-yellow-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(66 32 6);
        /// }
        /// ```
        EYellow950 => "border-e-yellow-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(247 254 231);
        /// }
        /// ```
        ELime50 => "border-e-lime-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(236 252 203);
        /// }
        /// ```
        ELime100 => "border-e-lime-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(217 249 157);
        /// }
        /// ```
        ELime200 => "border-e-lime-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(190 242 100);
        /// }
        /// ```
        ELime300 => "border-e-lime-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(163 230 53);
        /// }
        /// ```
        ELime400 => "border-e-lime-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(132 204 22);
        /// }
        /// ```
        ELime500 => "border-e-lime-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(101 163 13);
        /// }
        /// ```
        ELime600 => "border-e-lime-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(77 124 15);
        /// }
        /// ```
        ELime700 => "border-e-lime-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(63 98 18);
        /// }
        /// ```
        ELime800 => "border-e-lime-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(54 83 20);
        /// }
        /// ```
        ELime900 => "border-e-lime-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(26 46 5);
        /// }
        /// ```
        ELime950 => "border-e-lime-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(240 253 244);
        /// }
        /// ```
        EGreen50 => "border-e-green-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(220 252 231);
        /// }
        /// ```
        EGreen100 => "border-e-green-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(187 247 208);
        /// }
        /// ```
        EGreen200 => "border-e-green-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(134 239 172);
        /// }
        /// ```
        EGreen300 => "border-e-green-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(74 222 128);
        /// }
        /// ```
        EGreen400 => "border-e-green-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(34 197 94);
        /// }
        /// ```
        EGreen500 => "border-e-green-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(22 163 74);
        /// }
        /// ```
        EGreen600 => "border-e-green-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(21 128 61);
        /// }
        /// ```
        EGreen700 => "border-e-green-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(22 101 52);
        /// }
        /// ```
        EGreen800 => "border-e-green-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(20 83 45);
        /// }
        /// ```
        EGreen900 => "border-e-green-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(5 46 22);
        /// }
        /// ```
        EGreen950 => "border-e-green-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(236 253 245);
        /// }
        /// ```
        EEmerald50 => "border-e-emerald-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(209 250 229);
        /// }
        /// ```
        EEmerald100 => "border-e-emerald-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(167 243 208);
        /// }
        /// ```
        EEmerald200 => "border-e-emerald-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(110 231 183);
        /// }
        /// ```
        EEmerald300 => "border-e-emerald-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(52 211 153);
        /// }
        /// ```
        EEmerald400 => "border-e-emerald-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(16 185 129);
        /// }
        /// ```
        EEmerald500 => "border-e-emerald-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(5 150 105);
        /// }
        /// ```
        EEmerald600 => "border-e-emerald-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(4 120 87);
        /// }
        /// ```
        EEmerald700 => "border-e-emerald-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(6 95 70);
        /// }
        /// ```
        EEmerald800 => "border-e-emerald-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(6 78 59);
        /// }
        /// ```
        EEmerald900 => "border-e-emerald-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(2 44 34);
        /// }
        /// ```
        EEmerald950 => "border-e-emerald-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(240 253 250);
        /// }
        /// ```
        ETeal50 => "border-e-teal-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(204 251 241);
        /// }
        /// ```
        ETeal100 => "border-e-teal-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(153 246 228);
        /// }
        /// ```
        ETeal200 => "border-e-teal-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(94 234 212);
        /// }
        /// ```
        ETeal300 => "border-e-teal-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(45 212 191);
        /// }
        /// ```
        ETeal400 => "border-e-teal-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(20 184 166);
        /// }
        /// ```
        ETeal500 => "border-e-teal-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(13 148 136);
        /// }
        /// ```
        ETeal600 => "border-e-teal-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(15 118 110);
        /// }
        /// ```
        ETeal700 => "border-e-teal-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(17 94 89);
        /// }
        /// ```
        ETeal800 => "border-e-teal-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(19 78 74);
        /// }
        /// ```
        ETeal900 => "border-e-teal-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(4 47 46);
        /// }
        /// ```
        ETeal950 => "border-e-teal-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(236 254 255);
        /// }
        /// ```
        ECyan50 => "border-e-cyan-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(207 250 254);
        /// }
        /// ```
        ECyan100 => "border-e-cyan-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(165 243 252);
        /// }
        /// ```
        ECyan200 => "border-e-cyan-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(103 232 249);
        /// }
        /// ```
        ECyan300 => "border-e-cyan-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(34 211 238);
        /// }
        /// ```
        ECyan400 => "border-e-cyan-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(6 182 212);
        /// }
        /// ```
        ECyan500 => "border-e-cyan-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(8 145 178);
        /// }
        /// ```
        ECyan600 => "border-e-cyan-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(14 116 144);
        /// }
        /// ```
        ECyan700 => "border-e-cyan-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(21 94 117);
        /// }
        /// ```
        ECyan800 => "border-e-cyan-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(22 78 99);
        /// }
        /// ```
        ECyan900 => "border-e-cyan-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(8 51 68);
        /// }
        /// ```
        ECyan950 => "border-e-cyan-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(240 249 255);
        /// }
        /// ```
        ESky50 => "border-e-sky-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(224 242 254);
        /// }
        /// ```
        ESky100 => "border-e-sky-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(186 230 253);
        /// }
        /// ```
        ESky200 => "border-e-sky-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(125 211 252);
        /// }
        /// ```
        ESky300 => "border-e-sky-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(56 189 248);
        /// }
        /// ```
        ESky400 => "border-e-sky-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(14 165 233);
        /// }
        /// ```
        ESky500 => "border-e-sky-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(2 132 199);
        /// }
        /// ```
        ESky600 => "border-e-sky-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(3 105 161);
        /// }
        /// ```
        ESky700 => "border-e-sky-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(7 89 133);
        /// }
        /// ```
        ESky800 => "border-e-sky-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(12 74 110);
        /// }
        /// ```
        ESky900 => "border-e-sky-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(8 47 73);
        /// }
        /// ```
        ESky950 => "border-e-sky-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(239 246 255);
        /// }
        /// ```
        EBlue50 => "border-e-blue-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(219 234 254);
        /// }
        /// ```
        EBlue100 => "border-e-blue-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(191 219 254);
        /// }
        /// ```
        EBlue200 => "border-e-blue-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(147 197 253);
        /// }
        /// ```
        EBlue300 => "border-e-blue-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(96 165 250);
        /// }
        /// ```
        EBlue400 => "border-e-blue-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(59 130 246);
        /// }
        /// ```
        EBlue500 => "border-e-blue-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(37 99 235);
        /// }
        /// ```
        EBlue600 => "border-e-blue-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(29 78 216);
        /// }
        /// ```
        EBlue700 => "border-e-blue-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(30 64 175);
        /// }
        /// ```
        EBlue800 => "border-e-blue-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(30 58 138);
        /// }
        /// ```
        EBlue900 => "border-e-blue-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(23 37 84);
        /// }
        /// ```
        EBlue950 => "border-e-blue-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(238 242 255);
        /// }
        /// ```
        EIndigo50 => "border-e-indigo-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(224 231 255);
        /// }
        /// ```
        EIndigo100 => "border-e-indigo-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(199 210 254);
        /// }
        /// ```
        EIndigo200 => "border-e-indigo-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(165 180 252);
        /// }
        /// ```
        EIndigo300 => "border-e-indigo-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(129 140 248);
        /// }
        /// ```
        EIndigo400 => "border-e-indigo-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(99 102 241);
        /// }
        /// ```
        EIndigo500 => "border-e-indigo-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(79 70 229);
        /// }
        /// ```
        EIndigo600 => "border-e-indigo-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(67 56 202);
        /// }
        /// ```
        EIndigo700 => "border-e-indigo-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(55 48 163);
        /// }
        /// ```
        EIndigo800 => "border-e-indigo-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(49 46 129);
        /// }
        /// ```
        EIndigo900 => "border-e-indigo-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(30 27 75);
        /// }
        /// ```
        EIndigo950 => "border-e-indigo-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(245 243 255);
        /// }
        /// ```
        EViolet50 => "border-e-violet-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(237 233 254);
        /// }
        /// ```
        EViolet100 => "border-e-violet-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(221 214 254);
        /// }
        /// ```
        EViolet200 => "border-e-violet-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(196 181 253);
        /// }
        /// ```
        EViolet300 => "border-e-violet-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(167 139 250);
        /// }
        /// ```
        EViolet400 => "border-e-violet-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(139 92 246);
        /// }
        /// ```
        EViolet500 => "border-e-violet-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(124 58 237);
        /// }
        /// ```
        EViolet600 => "border-e-violet-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(109 40 217);
        /// }
        /// ```
        EViolet700 => "border-e-violet-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(91 33 182);
        /// }
        /// ```
        EViolet800 => "border-e-violet-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(76 29 149);
        /// }
        /// ```
        EViolet900 => "border-e-violet-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(46 16 101);
        /// }
        /// ```
        EViolet950 => "border-e-violet-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(250 245 255);
        /// }
        /// ```
        EPurple50 => "border-e-purple-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(243 232 255);
        /// }
        /// ```
        EPurple100 => "border-e-purple-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(233 213 255);
        /// }
        /// ```
        EPurple200 => "border-e-purple-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(216 180 254);
        /// }
        /// ```
        EPurple300 => "border-e-purple-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(192 132 252);
        /// }
        /// ```
        EPurple400 => "border-e-purple-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(168 85 247);
        /// }
        /// ```
        EPurple500 => "border-e-purple-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(147 51 234);
        /// }
        /// ```
        EPurple600 => "border-e-purple-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(126 34 206);
        /// }
        /// ```
        EPurple700 => "border-e-purple-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(107 33 168);
        /// }
        /// ```
        EPurple800 => "border-e-purple-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(88 28 135);
        /// }
        /// ```
        EPurple900 => "border-e-purple-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(59 7 100);
        /// }
        /// ```
        EPurple950 => "border-e-purple-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(253 244 255);
        /// }
        /// ```
        EFuchsia50 => "border-e-fuchsia-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(250 232 255);
        /// }
        /// ```
        EFuchsia100 => "border-e-fuchsia-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(245 208 254);
        /// }
        /// ```
        EFuchsia200 => "border-e-fuchsia-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(240 171 252);
        /// }
        /// ```
        EFuchsia300 => "border-e-fuchsia-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(232 121 249);
        /// }
        /// ```
        EFuchsia400 => "border-e-fuchsia-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(217 70 239);
        /// }
        /// ```
        EFuchsia500 => "border-e-fuchsia-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(192 38 211);
        /// }
        /// ```
        EFuchsia600 => "border-e-fuchsia-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(162 28 175);
        /// }
        /// ```
        EFuchsia700 => "border-e-fuchsia-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(134 25 143);
        /// }
        /// ```
        EFuchsia800 => "border-e-fuchsia-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(112 26 117);
        /// }
        /// ```
        EFuchsia900 => "border-e-fuchsia-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(74 4 78);
        /// }
        /// ```
        EFuchsia950 => "border-e-fuchsia-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(253 242 248);
        /// }
        /// ```
        EPink50 => "border-e-pink-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(252 231 243);
        /// }
        /// ```
        EPink100 => "border-e-pink-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(251 207 232);
        /// }
        /// ```
        EPink200 => "border-e-pink-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(249 168 212);
        /// }
        /// ```
        EPink300 => "border-e-pink-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(244 114 182);
        /// }
        /// ```
        EPink400 => "border-e-pink-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(236 72 153);
        /// }
        /// ```
        EPink500 => "border-e-pink-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(219 39 119);
        /// }
        /// ```
        EPink600 => "border-e-pink-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(190 24 93);
        /// }
        /// ```
        EPink700 => "border-e-pink-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(157 23 77);
        /// }
        /// ```
        EPink800 => "border-e-pink-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(131 24 67);
        /// }
        /// ```
        EPink900 => "border-e-pink-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(80 7 36);
        /// }
        /// ```
        EPink950 => "border-e-pink-950",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(255 241 242);
        /// }
        /// ```
        ERose50 => "border-e-rose-50",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(255 228 230);
        /// }
        /// ```
        ERose100 => "border-e-rose-100",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(254 205 211);
        /// }
        /// ```
        ERose200 => "border-e-rose-200",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(253 164 175);
        /// }
        /// ```
        ERose300 => "border-e-rose-300",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(251 113 133);
        /// }
        /// ```
        ERose400 => "border-e-rose-400",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(244 63 94);
        /// }
        /// ```
        ERose500 => "border-e-rose-500",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(225 29 72);
        /// }
        /// ```
        ERose600 => "border-e-rose-600",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(190 18 60);
        /// }
        /// ```
        ERose700 => "border-e-rose-700",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(159 18 57);
        /// }
        /// ```
        ERose800 => "border-e-rose-800",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(136 19 55);
        /// }
        /// ```
        ERose900 => "border-e-rose-900",
        /// ```css
        /// {
        ///     border-inline-end-color: rgb(76 5 25);
        /// }
        /// ```
        ERose950 => "border-e-rose-950",
        /// ```css
        /// {
        ///     border-top-color: inherit;
        /// }
        /// ```
        TInherit => "border-t-inherit",
        /// ```css
        /// {
        ///     border-top-color: currentColor;
        /// }
        /// ```
        TCurrent => "border-t-current",
        /// ```css
        /// {
        ///     border-top-color: transparent;
        /// }
        /// ```
        TTransparent => "border-t-transparent",
        /// ```css
        /// {
        ///     border-top-color: rgb(0 0 0);
        /// }
        /// ```
        TBlack => "border-t-black",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 255 255);
        /// }
        /// ```
        TWhite => "border-t-white",
        /// ```css
        /// {
        ///     border-top-color: rgb(248 250 252);
        /// }
        /// ```
        TSlate50 => "border-t-slate-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(241 245 249);
        /// }
        /// ```
        TSlate100 => "border-t-slate-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(226 232 240);
        /// }
        /// ```
        TSlate200 => "border-t-slate-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(203 213 225);
        /// }
        /// ```
        TSlate300 => "border-t-slate-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(148 163 184);
        /// }
        /// ```
        TSlate400 => "border-t-slate-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(100 116 139);
        /// }
        /// ```
        TSlate500 => "border-t-slate-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(71 85 105);
        /// }
        /// ```
        TSlate600 => "border-t-slate-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(51 65 85);
        /// }
        /// ```
        TSlate700 => "border-t-slate-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(30 41 59);
        /// }
        /// ```
        TSlate800 => "border-t-slate-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(15 23 42);
        /// }
        /// ```
        TSlate900 => "border-t-slate-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(2 6 23);
        /// }
        /// ```
        TSlate950 => "border-t-slate-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(249 250 251);
        /// }
        /// ```
        TGray50 => "border-t-gray-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(243 244 246);
        /// }
        /// ```
        TGray100 => "border-t-gray-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(229 231 235);
        /// }
        /// ```
        TGray200 => "border-t-gray-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(209 213 219);
        /// }
        /// ```
        TGray300 => "border-t-gray-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(156 163 175);
        /// }
        /// ```
        TGray400 => "border-t-gray-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(107 114 128);
        /// }
        /// ```
        TGray500 => "border-t-gray-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(75 85 99);
        /// }
        /// ```
        TGray600 => "border-t-gray-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(55 65 81);
        /// }
        /// ```
        TGray700 => "border-t-gray-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(31 41 55);
        /// }
        /// ```
        TGray800 => "border-t-gray-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(17 24 39);
        /// }
        /// ```
        TGray900 => "border-t-gray-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(3 7 18);
        /// }
        /// ```
        TGray950 => "border-t-gray-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 250 250);
        /// }
        /// ```
        TZinc50 => "border-t-zinc-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(244 244 245);
        /// }
        /// ```
        TZinc100 => "border-t-zinc-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(228 228 231);
        /// }
        /// ```
        TZinc200 => "border-t-zinc-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(212 212 216);
        /// }
        /// ```
        TZinc300 => "border-t-zinc-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(161 161 170);
        /// }
        /// ```
        TZinc400 => "border-t-zinc-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(113 113 122);
        /// }
        /// ```
        TZinc500 => "border-t-zinc-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(82 82 91);
        /// }
        /// ```
        TZinc600 => "border-t-zinc-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(63 63 70);
        /// }
        /// ```
        TZinc700 => "border-t-zinc-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(39 39 42);
        /// }
        /// ```
        TZinc800 => "border-t-zinc-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(24 24 27);
        /// }
        /// ```
        TZinc900 => "border-t-zinc-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(9 9 11);
        /// }
        /// ```
        TZinc950 => "border-t-zinc-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 250 250);
        /// }
        /// ```
        TNeutral50 => "border-t-neutral-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 245 245);
        /// }
        /// ```
        TNeutral100 => "border-t-neutral-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(229 229 229);
        /// }
        /// ```
        TNeutral200 => "border-t-neutral-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(212 212 212);
        /// }
        /// ```
        TNeutral300 => "border-t-neutral-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(163 163 163);
        /// }
        /// ```
        TNeutral400 => "border-t-neutral-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(115 115 115);
        /// }
        /// ```
        TNeutral500 => "border-t-neutral-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(82 82 82);
        /// }
        /// ```
        TNeutral600 => "border-t-neutral-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(64 64 64);
        /// }
        /// ```
        TNeutral700 => "border-t-neutral-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(38 38 38);
        /// }
        /// ```
        TNeutral800 => "border-t-neutral-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(23 23 23);
        /// }
        /// ```
        TNeutral900 => "border-t-neutral-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(10 10 10);
        /// }
        /// ```
        TNeutral950 => "border-t-neutral-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 250 249);
        /// }
        /// ```
        TStone50 => "border-t-stone-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 245 244);
        /// }
        /// ```
        TStone100 => "border-t-stone-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(231 229 228);
        /// }
        /// ```
        TStone200 => "border-t-stone-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(214 211 209);
        /// }
        /// ```
        TStone300 => "border-t-stone-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(168 162 158);
        /// }
        /// ```
        TStone400 => "border-t-stone-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(120 113 108);
        /// }
        /// ```
        TStone500 => "border-t-stone-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(87 83 78);
        /// }
        /// ```
        TStone600 => "border-t-stone-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(68 64 60);
        /// }
        /// ```
        TStone700 => "border-t-stone-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(41 37 36);
        /// }
        /// ```
        TStone800 => "border-t-stone-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(28 25 23);
        /// }
        /// ```
        TStone900 => "border-t-stone-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(12 10 9);
        /// }
        /// ```
        TStone950 => "border-t-stone-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 242 242);
        /// }
        /// ```
        TRed50 => "border-t-red-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 226 226);
        /// }
        /// ```
        TRed100 => "border-t-red-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 202 202);
        /// }
        /// ```
        TRed200 => "border-t-red-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(252 165 165);
        /// }
        /// ```
        TRed300 => "border-t-red-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(248 113 113);
        /// }
        /// ```
        TRed400 => "border-t-red-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(239 68 68);
        /// }
        /// ```
        TRed500 => "border-t-red-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(220 38 38);
        /// }
        /// ```
        TRed600 => "border-t-red-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(185 28 28);
        /// }
        /// ```
        TRed700 => "border-t-red-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(153 27 27);
        /// }
        /// ```
        TRed800 => "border-t-red-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(127 29 29);
        /// }
        /// ```
        TRed900 => "border-t-red-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(69 10 10);
        /// }
        /// ```
        TRed950 => "border-t-red-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 247 237);
        /// }
        /// ```
        TOrange50 => "border-t-orange-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 237 213);
        /// }
        /// ```
        TOrange100 => "border-t-orange-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 215 170);
        /// }
        /// ```
        TOrange200 => "border-t-orange-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 186 116);
        /// }
        /// ```
        TOrange300 => "border-t-orange-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(251 146 60);
        /// }
        /// ```
        TOrange400 => "border-t-orange-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(249 115 22);
        /// }
        /// ```
        TOrange500 => "border-t-orange-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(234 88 12);
        /// }
        /// ```
        TOrange600 => "border-t-orange-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(194 65 12);
        /// }
        /// ```
        TOrange700 => "border-t-orange-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(154 52 18);
        /// }
        /// ```
        TOrange800 => "border-t-orange-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(124 45 18);
        /// }
        /// ```
        TOrange900 => "border-t-orange-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(67 20 7);
        /// }
        /// ```
        TOrange950 => "border-t-orange-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 251 235);
        /// }
        /// ```
        TAmber50 => "border-t-amber-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 243 199);
        /// }
        /// ```
        TAmber100 => "border-t-amber-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 230 138);
        /// }
        /// ```
        TAmber200 => "border-t-amber-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(252 211 77);
        /// }
        /// ```
        TAmber300 => "border-t-amber-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(251 191 36);
        /// }
        /// ```
        TAmber400 => "border-t-amber-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 158 11);
        /// }
        /// ```
        TAmber500 => "border-t-amber-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(217 119 6);
        /// }
        /// ```
        TAmber600 => "border-t-amber-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(180 83 9);
        /// }
        /// ```
        TAmber700 => "border-t-amber-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(146 64 14);
        /// }
        /// ```
        TAmber800 => "border-t-amber-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(120 53 15);
        /// }
        /// ```
        TAmber900 => "border-t-amber-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(69 26 3);
        /// }
        /// ```
        TAmber950 => "border-t-amber-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 252 232);
        /// }
        /// ```
        TYellow50 => "border-t-yellow-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 249 195);
        /// }
        /// ```
        TYellow100 => "border-t-yellow-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 240 138);
        /// }
        /// ```
        TYellow200 => "border-t-yellow-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 224 71);
        /// }
        /// ```
        TYellow300 => "border-t-yellow-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 204 21);
        /// }
        /// ```
        TYellow400 => "border-t-yellow-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(234 179 8);
        /// }
        /// ```
        TYellow500 => "border-t-yellow-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(202 138 4);
        /// }
        /// ```
        TYellow600 => "border-t-yellow-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(161 98 7);
        /// }
        /// ```
        TYellow700 => "border-t-yellow-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(133 77 14);
        /// }
        /// ```
        TYellow800 => "border-t-yellow-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(113 63 18);
        /// }
        /// ```
        TYellow900 => "border-t-yellow-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(66 32 6);
        /// }
        /// ```
        TYellow950 => "border-t-yellow-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(247 254 231);
        /// }
        /// ```
        TLime50 => "border-t-lime-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(236 252 203);
        /// }
        /// ```
        TLime100 => "border-t-lime-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(217 249 157);
        /// }
        /// ```
        TLime200 => "border-t-lime-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(190 242 100);
        /// }
        /// ```
        TLime300 => "border-t-lime-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(163 230 53);
        /// }
        /// ```
        TLime400 => "border-t-lime-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(132 204 22);
        /// }
        /// ```
        TLime500 => "border-t-lime-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(101 163 13);
        /// }
        /// ```
        TLime600 => "border-t-lime-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(77 124 15);
        /// }
        /// ```
        TLime700 => "border-t-lime-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(63 98 18);
        /// }
        /// ```
        TLime800 => "border-t-lime-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(54 83 20);
        /// }
        /// ```
        TLime900 => "border-t-lime-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(26 46 5);
        /// }
        /// ```
        TLime950 => "border-t-lime-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(240 253 244);
        /// }
        /// ```
        TGreen50 => "border-t-green-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(220 252 231);
        /// }
        /// ```
        TGreen100 => "border-t-green-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(187 247 208);
        /// }
        /// ```
        TGreen200 => "border-t-green-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(134 239 172);
        /// }
        /// ```
        TGreen300 => "border-t-green-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(74 222 128);
        /// }
        /// ```
        TGreen400 => "border-t-green-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(34 197 94);
        /// }
        /// ```
        TGreen500 => "border-t-green-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(22 163 74);
        /// }
        /// ```
        TGreen600 => "border-t-green-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(21 128 61);
        /// }
        /// ```
        TGreen700 => "border-t-green-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(22 101 52);
        /// }
        /// ```
        TGreen800 => "border-t-green-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(20 83 45);
        /// }
        /// ```
        TGreen900 => "border-t-green-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(5 46 22);
        /// }
        /// ```
        TGreen950 => "border-t-green-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(236 253 245);
        /// }
        /// ```
        TEmerald50 => "border-t-emerald-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(209 250 229);
        /// }
        /// ```
        TEmerald100 => "border-t-emerald-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(167 243 208);
        /// }
        /// ```
        TEmerald200 => "border-t-emerald-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(110 231 183);
        /// }
        /// ```
        TEmerald300 => "border-t-emerald-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(52 211 153);
        /// }
        /// ```
        TEmerald400 => "border-t-emerald-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(16 185 129);
        /// }
        /// ```
        TEmerald500 => "border-t-emerald-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(5 150 105);
        /// }
        /// ```
        TEmerald600 => "border-t-emerald-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(4 120 87);
        /// }
        /// ```
        TEmerald700 => "border-t-emerald-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(6 95 70);
        /// }
        /// ```
        TEmerald800 => "border-t-emerald-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(6 78 59);
        /// }
        /// ```
        TEmerald900 => "border-t-emerald-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(2 44 34);
        /// }
        /// ```
        TEmerald950 => "border-t-emerald-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(240 253 250);
        /// }
        /// ```
        TTeal50 => "border-t-teal-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(204 251 241);
        /// }
        /// ```
        TTeal100 => "border-t-teal-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(153 246 228);
        /// }
        /// ```
        TTeal200 => "border-t-teal-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(94 234 212);
        /// }
        /// ```
        TTeal300 => "border-t-teal-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(45 212 191);
        /// }
        /// ```
        TTeal400 => "border-t-teal-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(20 184 166);
        /// }
        /// ```
        TTeal500 => "border-t-teal-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(13 148 136);
        /// }
        /// ```
        TTeal600 => "border-t-teal-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(15 118 110);
        /// }
        /// ```
        TTeal700 => "border-t-teal-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(17 94 89);
        /// }
        /// ```
        TTeal800 => "border-t-teal-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(19 78 74);
        /// }
        /// ```
        TTeal900 => "border-t-teal-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(4 47 46);
        /// }
        /// ```
        TTeal950 => "border-t-teal-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(236 254 255);
        /// }
        /// ```
        TCyan50 => "border-t-cyan-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(207 250 254);
        /// }
        /// ```
        TCyan100 => "border-t-cyan-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(165 243 252);
        /// }
        /// ```
        TCyan200 => "border-t-cyan-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(103 232 249);
        /// }
        /// ```
        TCyan300 => "border-t-cyan-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(34 211 238);
        /// }
        /// ```
        TCyan400 => "border-t-cyan-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(6 182 212);
        /// }
        /// ```
        TCyan500 => "border-t-cyan-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(8 145 178);
        /// }
        /// ```
        TCyan600 => "border-t-cyan-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(14 116 144);
        /// }
        /// ```
        TCyan700 => "border-t-cyan-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(21 94 117);
        /// }
        /// ```
        TCyan800 => "border-t-cyan-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(22 78 99);
        /// }
        /// ```
        TCyan900 => "border-t-cyan-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(8 51 68);
        /// }
        /// ```
        TCyan950 => "border-t-cyan-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(240 249 255);
        /// }
        /// ```
        TSky50 => "border-t-sky-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(224 242 254);
        /// }
        /// ```
        TSky100 => "border-t-sky-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(186 230 253);
        /// }
        /// ```
        TSky200 => "border-t-sky-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(125 211 252);
        /// }
        /// ```
        TSky300 => "border-t-sky-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(56 189 248);
        /// }
        /// ```
        TSky400 => "border-t-sky-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(14 165 233);
        /// }
        /// ```
        TSky500 => "border-t-sky-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(2 132 199);
        /// }
        /// ```
        TSky600 => "border-t-sky-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(3 105 161);
        /// }
        /// ```
        TSky700 => "border-t-sky-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(7 89 133);
        /// }
        /// ```
        TSky800 => "border-t-sky-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(12 74 110);
        /// }
        /// ```
        TSky900 => "border-t-sky-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(8 47 73);
        /// }
        /// ```
        TSky950 => "border-t-sky-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(239 246 255);
        /// }
        /// ```
        TBlue50 => "border-t-blue-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(219 234 254);
        /// }
        /// ```
        TBlue100 => "border-t-blue-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(191 219 254);
        /// }
        /// ```
        TBlue200 => "border-t-blue-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(147 197 253);
        /// }
        /// ```
        TBlue300 => "border-t-blue-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(96 165 250);
        /// }
        /// ```
        TBlue400 => "border-t-blue-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(59 130 246);
        /// }
        /// ```
        TBlue500 => "border-t-blue-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(37 99 235);
        /// }
        /// ```
        TBlue600 => "border-t-blue-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(29 78 216);
        /// }
        /// ```
        TBlue700 => "border-t-blue-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(30 64 175);
        /// }
        /// ```
        TBlue800 => "border-t-blue-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(30 58 138);
        /// }
        /// ```
        TBlue900 => "border-t-blue-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(23 37 84);
        /// }
        /// ```
        TBlue950 => "border-t-blue-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(238 242 255);
        /// }
        /// ```
        TIndigo50 => "border-t-indigo-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(224 231 255);
        /// }
        /// ```
        TIndigo100 => "border-t-indigo-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(199 210 254);
        /// }
        /// ```
        TIndigo200 => "border-t-indigo-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(165 180 252);
        /// }
        /// ```
        TIndigo300 => "border-t-indigo-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(129 140 248);
        /// }
        /// ```
        TIndigo400 => "border-t-indigo-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(99 102 241);
        /// }
        /// ```
        TIndigo500 => "border-t-indigo-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(79 70 229);
        /// }
        /// ```
        TIndigo600 => "border-t-indigo-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(67 56 202);
        /// }
        /// ```
        TIndigo700 => "border-t-indigo-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(55 48 163);
        /// }
        /// ```
        TIndigo800 => "border-t-indigo-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(49 46 129);
        /// }
        /// ```
        TIndigo900 => "border-t-indigo-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(30 27 75);
        /// }
        /// ```
        TIndigo950 => "border-t-indigo-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 243 255);
        /// }
        /// ```
        TViolet50 => "border-t-violet-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(237 233 254);
        /// }
        /// ```
        TViolet100 => "border-t-violet-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(221 214 254);
        /// }
        /// ```
        TViolet200 => "border-t-violet-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(196 181 253);
        /// }
        /// ```
        TViolet300 => "border-t-violet-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(167 139 250);
        /// }
        /// ```
        TViolet400 => "border-t-violet-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(139 92 246);
        /// }
        /// ```
        TViolet500 => "border-t-violet-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(124 58 237);
        /// }
        /// ```
        TViolet600 => "border-t-violet-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(109 40 217);
        /// }
        /// ```
        TViolet700 => "border-t-violet-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(91 33 182);
        /// }
        /// ```
        TViolet800 => "border-t-violet-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(76 29 149);
        /// }
        /// ```
        TViolet900 => "border-t-violet-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(46 16 101);
        /// }
        /// ```
        TViolet950 => "border-t-violet-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 245 255);
        /// }
        /// ```
        TPurple50 => "border-t-purple-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(243 232 255);
        /// }
        /// ```
        TPurple100 => "border-t-purple-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(233 213 255);
        /// }
        /// ```
        TPurple200 => "border-t-purple-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(216 180 254);
        /// }
        /// ```
        TPurple300 => "border-t-purple-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(192 132 252);
        /// }
        /// ```
        TPurple400 => "border-t-purple-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(168 85 247);
        /// }
        /// ```
        TPurple500 => "border-t-purple-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(147 51 234);
        /// }
        /// ```
        TPurple600 => "border-t-purple-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(126 34 206);
        /// }
        /// ```
        TPurple700 => "border-t-purple-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(107 33 168);
        /// }
        /// ```
        TPurple800 => "border-t-purple-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(88 28 135);
        /// }
        /// ```
        TPurple900 => "border-t-purple-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(59 7 100);
        /// }
        /// ```
        TPurple950 => "border-t-purple-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 244 255);
        /// }
        /// ```
        TFuchsia50 => "border-t-fuchsia-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(250 232 255);
        /// }
        /// ```
        TFuchsia100 => "border-t-fuchsia-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(245 208 254);
        /// }
        /// ```
        TFuchsia200 => "border-t-fuchsia-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(240 171 252);
        /// }
        /// ```
        TFuchsia300 => "border-t-fuchsia-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(232 121 249);
        /// }
        /// ```
        TFuchsia400 => "border-t-fuchsia-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(217 70 239);
        /// }
        /// ```
        TFuchsia500 => "border-t-fuchsia-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(192 38 211);
        /// }
        /// ```
        TFuchsia600 => "border-t-fuchsia-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(162 28 175);
        /// }
        /// ```
        TFuchsia700 => "border-t-fuchsia-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(134 25 143);
        /// }
        /// ```
        TFuchsia800 => "border-t-fuchsia-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(112 26 117);
        /// }
        /// ```
        TFuchsia900 => "border-t-fuchsia-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(74 4 78);
        /// }
        /// ```
        TFuchsia950 => "border-t-fuchsia-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 242 248);
        /// }
        /// ```
        TPink50 => "border-t-pink-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(252 231 243);
        /// }
        /// ```
        TPink100 => "border-t-pink-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(251 207 232);
        /// }
        /// ```
        TPink200 => "border-t-pink-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(249 168 212);
        /// }
        /// ```
        TPink300 => "border-t-pink-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(244 114 182);
        /// }
        /// ```
        TPink400 => "border-t-pink-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(236 72 153);
        /// }
        /// ```
        TPink500 => "border-t-pink-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(219 39 119);
        /// }
        /// ```
        TPink600 => "border-t-pink-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(190 24 93);
        /// }
        /// ```
        TPink700 => "border-t-pink-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(157 23 77);
        /// }
        /// ```
        TPink800 => "border-t-pink-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(131 24 67);
        /// }
        /// ```
        TPink900 => "border-t-pink-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(80 7 36);
        /// }
        /// ```
        TPink950 => "border-t-pink-950",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 241 242);
        /// }
        /// ```
        TRose50 => "border-t-rose-50",
        /// ```css
        /// {
        ///     border-top-color: rgb(255 228 230);
        /// }
        /// ```
        TRose100 => "border-t-rose-100",
        /// ```css
        /// {
        ///     border-top-color: rgb(254 205 211);
        /// }
        /// ```
        TRose200 => "border-t-rose-200",
        /// ```css
        /// {
        ///     border-top-color: rgb(253 164 175);
        /// }
        /// ```
        TRose300 => "border-t-rose-300",
        /// ```css
        /// {
        ///     border-top-color: rgb(251 113 133);
        /// }
        /// ```
        TRose400 => "border-t-rose-400",
        /// ```css
        /// {
        ///     border-top-color: rgb(244 63 94);
        /// }
        /// ```
        TRose500 => "border-t-rose-500",
        /// ```css
        /// {
        ///     border-top-color: rgb(225 29 72);
        /// }
        /// ```
        TRose600 => "border-t-rose-600",
        /// ```css
        /// {
        ///     border-top-color: rgb(190 18 60);
        /// }
        /// ```
        TRose700 => "border-t-rose-700",
        /// ```css
        /// {
        ///     border-top-color: rgb(159 18 57);
        /// }
        /// ```
        TRose800 => "border-t-rose-800",
        /// ```css
        /// {
        ///     border-top-color: rgb(136 19 55);
        /// }
        /// ```
        TRose900 => "border-t-rose-900",
        /// ```css
        /// {
        ///     border-top-color: rgb(76 5 25);
        /// }
        /// ```
        TRose950 => "border-t-rose-950",
        /// ```css
        /// {
        ///     border-right-color: inherit;
        /// }
        /// ```
        RInherit => "border-r-inherit",
        /// ```css
        /// {
        ///     border-right-color: currentColor;
        /// }
        /// ```
        RCurrent => "border-r-current",
        /// ```css
        /// {
        ///     border-right-color: transparent;
        /// }
        /// ```
        RTransparent => "border-r-transparent",
        /// ```css
        /// {
        ///     border-right-color: rgb(0 0 0);
        /// }
        /// ```
        RBlack => "border-r-black",
        /// ```css
        /// {
        ///     border-right-color: rgb(255 255 255);
        /// }
        /// ```
        RWhite => "border-r-white",
        /// ```css
        /// {
        ///     border-right-color: rgb(248 250 252);
        /// }
        /// ```
        RSlate50 => "border-r-slate-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(241 245 249);
        /// }
        /// ```
        RSlate100 => "border-r-slate-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(226 232 240);
        /// }
        /// ```
        RSlate200 => "border-r-slate-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(203 213 225);
        /// }
        /// ```
        RSlate300 => "border-r-slate-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(148 163 184);
        /// }
        /// ```
        RSlate400 => "border-r-slate-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(100 116 139);
        /// }
        /// ```
        RSlate500 => "border-r-slate-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(71 85 105);
        /// }
        /// ```
        RSlate600 => "border-r-slate-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(51 65 85);
        /// }
        /// ```
        RSlate700 => "border-r-slate-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(30 41 59);
        /// }
        /// ```
        RSlate800 => "border-r-slate-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(15 23 42);
        /// }
        /// ```
        RSlate900 => "border-r-slate-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(2 6 23);
        /// }
        /// ```
        RSlate950 => "border-r-slate-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(249 250 251);
        /// }
        /// ```
        RGray50 => "border-r-gray-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(243 244 246);
        /// }
        /// ```
        RGray100 => "border-r-gray-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(229 231 235);
        /// }
        /// ```
        RGray200 => "border-r-gray-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(209 213 219);
        /// }
        /// ```
        RGray300 => "border-r-gray-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(156 163 175);
        /// }
        /// ```
        RGray400 => "border-r-gray-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(107 114 128);
        /// }
        /// ```
        RGray500 => "border-r-gray-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(75 85 99);
        /// }
        /// ```
        RGray600 => "border-r-gray-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(55 65 81);
        /// }
        /// ```
        RGray700 => "border-r-gray-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(31 41 55);
        /// }
        /// ```
        RGray800 => "border-r-gray-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(17 24 39);
        /// }
        /// ```
        RGray900 => "border-r-gray-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(3 7 18);
        /// }
        /// ```
        RGray950 => "border-r-gray-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(250 250 250);
        /// }
        /// ```
        RZinc50 => "border-r-zinc-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(244 244 245);
        /// }
        /// ```
        RZinc100 => "border-r-zinc-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(228 228 231);
        /// }
        /// ```
        RZinc200 => "border-r-zinc-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(212 212 216);
        /// }
        /// ```
        RZinc300 => "border-r-zinc-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(161 161 170);
        /// }
        /// ```
        RZinc400 => "border-r-zinc-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(113 113 122);
        /// }
        /// ```
        RZinc500 => "border-r-zinc-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(82 82 91);
        /// }
        /// ```
        RZinc600 => "border-r-zinc-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(63 63 70);
        /// }
        /// ```
        RZinc700 => "border-r-zinc-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(39 39 42);
        /// }
        /// ```
        RZinc800 => "border-r-zinc-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(24 24 27);
        /// }
        /// ```
        RZinc900 => "border-r-zinc-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(9 9 11);
        /// }
        /// ```
        RZinc950 => "border-r-zinc-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(250 250 250);
        /// }
        /// ```
        RNeutral50 => "border-r-neutral-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(245 245 245);
        /// }
        /// ```
        RNeutral100 => "border-r-neutral-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(229 229 229);
        /// }
        /// ```
        RNeutral200 => "border-r-neutral-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(212 212 212);
        /// }
        /// ```
        RNeutral300 => "border-r-neutral-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(163 163 163);
        /// }
        /// ```
        RNeutral400 => "border-r-neutral-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(115 115 115);
        /// }
        /// ```
        RNeutral500 => "border-r-neutral-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(82 82 82);
        /// }
        /// ```
        RNeutral600 => "border-r-neutral-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(64 64 64);
        /// }
        /// ```
        RNeutral700 => "border-r-neutral-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(38 38 38);
        /// }
        /// ```
        RNeutral800 => "border-r-neutral-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(23 23 23);
        /// }
        /// ```
        RNeutral900 => "border-r-neutral-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(10 10 10);
        /// }
        /// ```
        RNeutral950 => "border-r-neutral-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(250 250 249);
        /// }
        /// ```
        RStone50 => "border-r-stone-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(245 245 244);
        /// }
        /// ```
        RStone100 => "border-r-stone-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(231 229 228);
        /// }
        /// ```
        RStone200 => "border-r-stone-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(214 211 209);
        /// }
        /// ```
        RStone300 => "border-r-stone-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(168 162 158);
        /// }
        /// ```
        RStone400 => "border-r-stone-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(120 113 108);
        /// }
        /// ```
        RStone500 => "border-r-stone-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(87 83 78);
        /// }
        /// ```
        RStone600 => "border-r-stone-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(68 64 60);
        /// }
        /// ```
        RStone700 => "border-r-stone-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(41 37 36);
        /// }
        /// ```
        RStone800 => "border-r-stone-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(28 25 23);
        /// }
        /// ```
        RStone900 => "border-r-stone-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(12 10 9);
        /// }
        /// ```
        RStone950 => "border-r-stone-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 242 242);
        /// }
        /// ```
        RRed50 => "border-r-red-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 226 226);
        /// }
        /// ```
        RRed100 => "border-r-red-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 202 202);
        /// }
        /// ```
        RRed200 => "border-r-red-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(252 165 165);
        /// }
        /// ```
        RRed300 => "border-r-red-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(248 113 113);
        /// }
        /// ```
        RRed400 => "border-r-red-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(239 68 68);
        /// }
        /// ```
        RRed500 => "border-r-red-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(220 38 38);
        /// }
        /// ```
        RRed600 => "border-r-red-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(185 28 28);
        /// }
        /// ```
        RRed700 => "border-r-red-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(153 27 27);
        /// }
        /// ```
        RRed800 => "border-r-red-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(127 29 29);
        /// }
        /// ```
        RRed900 => "border-r-red-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(69 10 10);
        /// }
        /// ```
        RRed950 => "border-r-red-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(255 247 237);
        /// }
        /// ```
        ROrange50 => "border-r-orange-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(255 237 213);
        /// }
        /// ```
        ROrange100 => "border-r-orange-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 215 170);
        /// }
        /// ```
        ROrange200 => "border-r-orange-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(253 186 116);
        /// }
        /// ```
        ROrange300 => "border-r-orange-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(251 146 60);
        /// }
        /// ```
        ROrange400 => "border-r-orange-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(249 115 22);
        /// }
        /// ```
        ROrange500 => "border-r-orange-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(234 88 12);
        /// }
        /// ```
        ROrange600 => "border-r-orange-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(194 65 12);
        /// }
        /// ```
        ROrange700 => "border-r-orange-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(154 52 18);
        /// }
        /// ```
        ROrange800 => "border-r-orange-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(124 45 18);
        /// }
        /// ```
        ROrange900 => "border-r-orange-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(67 20 7);
        /// }
        /// ```
        ROrange950 => "border-r-orange-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(255 251 235);
        /// }
        /// ```
        RAmber50 => "border-r-amber-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 243 199);
        /// }
        /// ```
        RAmber100 => "border-r-amber-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(253 230 138);
        /// }
        /// ```
        RAmber200 => "border-r-amber-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(252 211 77);
        /// }
        /// ```
        RAmber300 => "border-r-amber-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(251 191 36);
        /// }
        /// ```
        RAmber400 => "border-r-amber-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(245 158 11);
        /// }
        /// ```
        RAmber500 => "border-r-amber-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(217 119 6);
        /// }
        /// ```
        RAmber600 => "border-r-amber-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(180 83 9);
        /// }
        /// ```
        RAmber700 => "border-r-amber-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(146 64 14);
        /// }
        /// ```
        RAmber800 => "border-r-amber-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(120 53 15);
        /// }
        /// ```
        RAmber900 => "border-r-amber-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(69 26 3);
        /// }
        /// ```
        RAmber950 => "border-r-amber-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 252 232);
        /// }
        /// ```
        RYellow50 => "border-r-yellow-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 249 195);
        /// }
        /// ```
        RYellow100 => "border-r-yellow-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 240 138);
        /// }
        /// ```
        RYellow200 => "border-r-yellow-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(253 224 71);
        /// }
        /// ```
        RYellow300 => "border-r-yellow-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(250 204 21);
        /// }
        /// ```
        RYellow400 => "border-r-yellow-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(234 179 8);
        /// }
        /// ```
        RYellow500 => "border-r-yellow-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(202 138 4);
        /// }
        /// ```
        RYellow600 => "border-r-yellow-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(161 98 7);
        /// }
        /// ```
        RYellow700 => "border-r-yellow-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(133 77 14);
        /// }
        /// ```
        RYellow800 => "border-r-yellow-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(113 63 18);
        /// }
        /// ```
        RYellow900 => "border-r-yellow-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(66 32 6);
        /// }
        /// ```
        RYellow950 => "border-r-yellow-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(247 254 231);
        /// }
        /// ```
        RLime50 => "border-r-lime-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(236 252 203);
        /// }
        /// ```
        RLime100 => "border-r-lime-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(217 249 157);
        /// }
        /// ```
        RLime200 => "border-r-lime-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(190 242 100);
        /// }
        /// ```
        RLime300 => "border-r-lime-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(163 230 53);
        /// }
        /// ```
        RLime400 => "border-r-lime-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(132 204 22);
        /// }
        /// ```
        RLime500 => "border-r-lime-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(101 163 13);
        /// }
        /// ```
        RLime600 => "border-r-lime-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(77 124 15);
        /// }
        /// ```
        RLime700 => "border-r-lime-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(63 98 18);
        /// }
        /// ```
        RLime800 => "border-r-lime-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(54 83 20);
        /// }
        /// ```
        RLime900 => "border-r-lime-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(26 46 5);
        /// }
        /// ```
        RLime950 => "border-r-lime-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(240 253 244);
        /// }
        /// ```
        RGreen50 => "border-r-green-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(220 252 231);
        /// }
        /// ```
        RGreen100 => "border-r-green-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(187 247 208);
        /// }
        /// ```
        RGreen200 => "border-r-green-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(134 239 172);
        /// }
        /// ```
        RGreen300 => "border-r-green-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(74 222 128);
        /// }
        /// ```
        RGreen400 => "border-r-green-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(34 197 94);
        /// }
        /// ```
        RGreen500 => "border-r-green-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(22 163 74);
        /// }
        /// ```
        RGreen600 => "border-r-green-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(21 128 61);
        /// }
        /// ```
        RGreen700 => "border-r-green-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(22 101 52);
        /// }
        /// ```
        RGreen800 => "border-r-green-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(20 83 45);
        /// }
        /// ```
        RGreen900 => "border-r-green-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(5 46 22);
        /// }
        /// ```
        RGreen950 => "border-r-green-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(236 253 245);
        /// }
        /// ```
        REmerald50 => "border-r-emerald-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(209 250 229);
        /// }
        /// ```
        REmerald100 => "border-r-emerald-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(167 243 208);
        /// }
        /// ```
        REmerald200 => "border-r-emerald-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(110 231 183);
        /// }
        /// ```
        REmerald300 => "border-r-emerald-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(52 211 153);
        /// }
        /// ```
        REmerald400 => "border-r-emerald-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(16 185 129);
        /// }
        /// ```
        REmerald500 => "border-r-emerald-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(5 150 105);
        /// }
        /// ```
        REmerald600 => "border-r-emerald-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(4 120 87);
        /// }
        /// ```
        REmerald700 => "border-r-emerald-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(6 95 70);
        /// }
        /// ```
        REmerald800 => "border-r-emerald-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(6 78 59);
        /// }
        /// ```
        REmerald900 => "border-r-emerald-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(2 44 34);
        /// }
        /// ```
        REmerald950 => "border-r-emerald-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(240 253 250);
        /// }
        /// ```
        RTeal50 => "border-r-teal-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(204 251 241);
        /// }
        /// ```
        RTeal100 => "border-r-teal-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(153 246 228);
        /// }
        /// ```
        RTeal200 => "border-r-teal-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(94 234 212);
        /// }
        /// ```
        RTeal300 => "border-r-teal-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(45 212 191);
        /// }
        /// ```
        RTeal400 => "border-r-teal-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(20 184 166);
        /// }
        /// ```
        RTeal500 => "border-r-teal-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(13 148 136);
        /// }
        /// ```
        RTeal600 => "border-r-teal-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(15 118 110);
        /// }
        /// ```
        RTeal700 => "border-r-teal-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(17 94 89);
        /// }
        /// ```
        RTeal800 => "border-r-teal-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(19 78 74);
        /// }
        /// ```
        RTeal900 => "border-r-teal-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(4 47 46);
        /// }
        /// ```
        RTeal950 => "border-r-teal-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(236 254 255);
        /// }
        /// ```
        RCyan50 => "border-r-cyan-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(207 250 254);
        /// }
        /// ```
        RCyan100 => "border-r-cyan-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(165 243 252);
        /// }
        /// ```
        RCyan200 => "border-r-cyan-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(103 232 249);
        /// }
        /// ```
        RCyan300 => "border-r-cyan-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(34 211 238);
        /// }
        /// ```
        RCyan400 => "border-r-cyan-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(6 182 212);
        /// }
        /// ```
        RCyan500 => "border-r-cyan-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(8 145 178);
        /// }
        /// ```
        RCyan600 => "border-r-cyan-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(14 116 144);
        /// }
        /// ```
        RCyan700 => "border-r-cyan-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(21 94 117);
        /// }
        /// ```
        RCyan800 => "border-r-cyan-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(22 78 99);
        /// }
        /// ```
        RCyan900 => "border-r-cyan-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(8 51 68);
        /// }
        /// ```
        RCyan950 => "border-r-cyan-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(240 249 255);
        /// }
        /// ```
        RSky50 => "border-r-sky-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(224 242 254);
        /// }
        /// ```
        RSky100 => "border-r-sky-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(186 230 253);
        /// }
        /// ```
        RSky200 => "border-r-sky-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(125 211 252);
        /// }
        /// ```
        RSky300 => "border-r-sky-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(56 189 248);
        /// }
        /// ```
        RSky400 => "border-r-sky-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(14 165 233);
        /// }
        /// ```
        RSky500 => "border-r-sky-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(2 132 199);
        /// }
        /// ```
        RSky600 => "border-r-sky-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(3 105 161);
        /// }
        /// ```
        RSky700 => "border-r-sky-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(7 89 133);
        /// }
        /// ```
        RSky800 => "border-r-sky-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(12 74 110);
        /// }
        /// ```
        RSky900 => "border-r-sky-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(8 47 73);
        /// }
        /// ```
        RSky950 => "border-r-sky-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(239 246 255);
        /// }
        /// ```
        RBlue50 => "border-r-blue-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(219 234 254);
        /// }
        /// ```
        RBlue100 => "border-r-blue-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(191 219 254);
        /// }
        /// ```
        RBlue200 => "border-r-blue-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(147 197 253);
        /// }
        /// ```
        RBlue300 => "border-r-blue-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(96 165 250);
        /// }
        /// ```
        RBlue400 => "border-r-blue-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(59 130 246);
        /// }
        /// ```
        RBlue500 => "border-r-blue-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(37 99 235);
        /// }
        /// ```
        RBlue600 => "border-r-blue-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(29 78 216);
        /// }
        /// ```
        RBlue700 => "border-r-blue-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(30 64 175);
        /// }
        /// ```
        RBlue800 => "border-r-blue-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(30 58 138);
        /// }
        /// ```
        RBlue900 => "border-r-blue-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(23 37 84);
        /// }
        /// ```
        RBlue950 => "border-r-blue-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(238 242 255);
        /// }
        /// ```
        RIndigo50 => "border-r-indigo-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(224 231 255);
        /// }
        /// ```
        RIndigo100 => "border-r-indigo-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(199 210 254);
        /// }
        /// ```
        RIndigo200 => "border-r-indigo-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(165 180 252);
        /// }
        /// ```
        RIndigo300 => "border-r-indigo-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(129 140 248);
        /// }
        /// ```
        RIndigo400 => "border-r-indigo-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(99 102 241);
        /// }
        /// ```
        RIndigo500 => "border-r-indigo-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(79 70 229);
        /// }
        /// ```
        RIndigo600 => "border-r-indigo-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(67 56 202);
        /// }
        /// ```
        RIndigo700 => "border-r-indigo-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(55 48 163);
        /// }
        /// ```
        RIndigo800 => "border-r-indigo-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(49 46 129);
        /// }
        /// ```
        RIndigo900 => "border-r-indigo-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(30 27 75);
        /// }
        /// ```
        RIndigo950 => "border-r-indigo-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(245 243 255);
        /// }
        /// ```
        RViolet50 => "border-r-violet-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(237 233 254);
        /// }
        /// ```
        RViolet100 => "border-r-violet-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(221 214 254);
        /// }
        /// ```
        RViolet200 => "border-r-violet-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(196 181 253);
        /// }
        /// ```
        RViolet300 => "border-r-violet-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(167 139 250);
        /// }
        /// ```
        RViolet400 => "border-r-violet-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(139 92 246);
        /// }
        /// ```
        RViolet500 => "border-r-violet-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(124 58 237);
        /// }
        /// ```
        RViolet600 => "border-r-violet-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(109 40 217);
        /// }
        /// ```
        RViolet700 => "border-r-violet-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(91 33 182);
        /// }
        /// ```
        RViolet800 => "border-r-violet-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(76 29 149);
        /// }
        /// ```
        RViolet900 => "border-r-violet-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(46 16 101);
        /// }
        /// ```
        RViolet950 => "border-r-violet-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(250 245 255);
        /// }
        /// ```
        RPurple50 => "border-r-purple-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(243 232 255);
        /// }
        /// ```
        RPurple100 => "border-r-purple-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(233 213 255);
        /// }
        /// ```
        RPurple200 => "border-r-purple-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(216 180 254);
        /// }
        /// ```
        RPurple300 => "border-r-purple-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(192 132 252);
        /// }
        /// ```
        RPurple400 => "border-r-purple-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(168 85 247);
        /// }
        /// ```
        RPurple500 => "border-r-purple-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(147 51 234);
        /// }
        /// ```
        RPurple600 => "border-r-purple-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(126 34 206);
        /// }
        /// ```
        RPurple700 => "border-r-purple-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(107 33 168);
        /// }
        /// ```
        RPurple800 => "border-r-purple-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(88 28 135);
        /// }
        /// ```
        RPurple900 => "border-r-purple-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(59 7 100);
        /// }
        /// ```
        RPurple950 => "border-r-purple-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(253 244 255);
        /// }
        /// ```
        RFuchsia50 => "border-r-fuchsia-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(250 232 255);
        /// }
        /// ```
        RFuchsia100 => "border-r-fuchsia-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(245 208 254);
        /// }
        /// ```
        RFuchsia200 => "border-r-fuchsia-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(240 171 252);
        /// }
        /// ```
        RFuchsia300 => "border-r-fuchsia-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(232 121 249);
        /// }
        /// ```
        RFuchsia400 => "border-r-fuchsia-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(217 70 239);
        /// }
        /// ```
        RFuchsia500 => "border-r-fuchsia-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(192 38 211);
        /// }
        /// ```
        RFuchsia600 => "border-r-fuchsia-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(162 28 175);
        /// }
        /// ```
        RFuchsia700 => "border-r-fuchsia-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(134 25 143);
        /// }
        /// ```
        RFuchsia800 => "border-r-fuchsia-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(112 26 117);
        /// }
        /// ```
        RFuchsia900 => "border-r-fuchsia-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(74 4 78);
        /// }
        /// ```
        RFuchsia950 => "border-r-fuchsia-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(253 242 248);
        /// }
        /// ```
        RPink50 => "border-r-pink-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(252 231 243);
        /// }
        /// ```
        RPink100 => "border-r-pink-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(251 207 232);
        /// }
        /// ```
        RPink200 => "border-r-pink-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(249 168 212);
        /// }
        /// ```
        RPink300 => "border-r-pink-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(244 114 182);
        /// }
        /// ```
        RPink400 => "border-r-pink-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(236 72 153);
        /// }
        /// ```
        RPink500 => "border-r-pink-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(219 39 119);
        /// }
        /// ```
        RPink600 => "border-r-pink-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(190 24 93);
        /// }
        /// ```
        RPink700 => "border-r-pink-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(157 23 77);
        /// }
        /// ```
        RPink800 => "border-r-pink-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(131 24 67);
        /// }
        /// ```
        RPink900 => "border-r-pink-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(80 7 36);
        /// }
        /// ```
        RPink950 => "border-r-pink-950",
        /// ```css
        /// {
        ///     border-right-color: rgb(255 241 242);
        /// }
        /// ```
        RRose50 => "border-r-rose-50",
        /// ```css
        /// {
        ///     border-right-color: rgb(255 228 230);
        /// }
        /// ```
        RRose100 => "border-r-rose-100",
        /// ```css
        /// {
        ///     border-right-color: rgb(254 205 211);
        /// }
        /// ```
        RRose200 => "border-r-rose-200",
        /// ```css
        /// {
        ///     border-right-color: rgb(253 164 175);
        /// }
        /// ```
        RRose300 => "border-r-rose-300",
        /// ```css
        /// {
        ///     border-right-color: rgb(251 113 133);
        /// }
        /// ```
        RRose400 => "border-r-rose-400",
        /// ```css
        /// {
        ///     border-right-color: rgb(244 63 94);
        /// }
        /// ```
        RRose500 => "border-r-rose-500",
        /// ```css
        /// {
        ///     border-right-color: rgb(225 29 72);
        /// }
        /// ```
        RRose600 => "border-r-rose-600",
        /// ```css
        /// {
        ///     border-right-color: rgb(190 18 60);
        /// }
        /// ```
        RRose700 => "border-r-rose-700",
        /// ```css
        /// {
        ///     border-right-color: rgb(159 18 57);
        /// }
        /// ```
        RRose800 => "border-r-rose-800",
        /// ```css
        /// {
        ///     border-right-color: rgb(136 19 55);
        /// }
        /// ```
        RRose900 => "border-r-rose-900",
        /// ```css
        /// {
        ///     border-right-color: rgb(76 5 25);
        /// }
        /// ```
        RRose950 => "border-r-rose-950",
        /// ```css
        /// {
        ///     border-bottom-color: inherit;
        /// }
        /// ```
        BInherit => "border-b-inherit",
        /// ```css
        /// {
        ///     border-bottom-color: currentColor;
        /// }
        /// ```
        BCurrent => "border-b-current",
        /// ```css
        /// {
        ///     border-bottom-color: transparent;
        /// }
        /// ```
        BTransparent => "border-b-transparent",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(0 0 0);
        /// }
        /// ```
        BBlack => "border-b-black",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(255 255 255);
        /// }
        /// ```
        BWhite => "border-b-white",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(248 250 252);
        /// }
        /// ```
        BSlate50 => "border-b-slate-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(241 245 249);
        /// }
        /// ```
        BSlate100 => "border-b-slate-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(226 232 240);
        /// }
        /// ```
        BSlate200 => "border-b-slate-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(203 213 225);
        /// }
        /// ```
        BSlate300 => "border-b-slate-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(148 163 184);
        /// }
        /// ```
        BSlate400 => "border-b-slate-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(100 116 139);
        /// }
        /// ```
        BSlate500 => "border-b-slate-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(71 85 105);
        /// }
        /// ```
        BSlate600 => "border-b-slate-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(51 65 85);
        /// }
        /// ```
        BSlate700 => "border-b-slate-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(30 41 59);
        /// }
        /// ```
        BSlate800 => "border-b-slate-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(15 23 42);
        /// }
        /// ```
        BSlate900 => "border-b-slate-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(2 6 23);
        /// }
        /// ```
        BSlate950 => "border-b-slate-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(249 250 251);
        /// }
        /// ```
        BGray50 => "border-b-gray-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(243 244 246);
        /// }
        /// ```
        BGray100 => "border-b-gray-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(229 231 235);
        /// }
        /// ```
        BGray200 => "border-b-gray-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(209 213 219);
        /// }
        /// ```
        BGray300 => "border-b-gray-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(156 163 175);
        /// }
        /// ```
        BGray400 => "border-b-gray-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(107 114 128);
        /// }
        /// ```
        BGray500 => "border-b-gray-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(75 85 99);
        /// }
        /// ```
        BGray600 => "border-b-gray-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(55 65 81);
        /// }
        /// ```
        BGray700 => "border-b-gray-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(31 41 55);
        /// }
        /// ```
        BGray800 => "border-b-gray-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(17 24 39);
        /// }
        /// ```
        BGray900 => "border-b-gray-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(3 7 18);
        /// }
        /// ```
        BGray950 => "border-b-gray-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(250 250 250);
        /// }
        /// ```
        BZinc50 => "border-b-zinc-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(244 244 245);
        /// }
        /// ```
        BZinc100 => "border-b-zinc-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(228 228 231);
        /// }
        /// ```
        BZinc200 => "border-b-zinc-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(212 212 216);
        /// }
        /// ```
        BZinc300 => "border-b-zinc-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(161 161 170);
        /// }
        /// ```
        BZinc400 => "border-b-zinc-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(113 113 122);
        /// }
        /// ```
        BZinc500 => "border-b-zinc-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(82 82 91);
        /// }
        /// ```
        BZinc600 => "border-b-zinc-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(63 63 70);
        /// }
        /// ```
        BZinc700 => "border-b-zinc-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(39 39 42);
        /// }
        /// ```
        BZinc800 => "border-b-zinc-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(24 24 27);
        /// }
        /// ```
        BZinc900 => "border-b-zinc-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(9 9 11);
        /// }
        /// ```
        BZinc950 => "border-b-zinc-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(250 250 250);
        /// }
        /// ```
        BNeutral50 => "border-b-neutral-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(245 245 245);
        /// }
        /// ```
        BNeutral100 => "border-b-neutral-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(229 229 229);
        /// }
        /// ```
        BNeutral200 => "border-b-neutral-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(212 212 212);
        /// }
        /// ```
        BNeutral300 => "border-b-neutral-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(163 163 163);
        /// }
        /// ```
        BNeutral400 => "border-b-neutral-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(115 115 115);
        /// }
        /// ```
        BNeutral500 => "border-b-neutral-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(82 82 82);
        /// }
        /// ```
        BNeutral600 => "border-b-neutral-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(64 64 64);
        /// }
        /// ```
        BNeutral700 => "border-b-neutral-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(38 38 38);
        /// }
        /// ```
        BNeutral800 => "border-b-neutral-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(23 23 23);
        /// }
        /// ```
        BNeutral900 => "border-b-neutral-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(10 10 10);
        /// }
        /// ```
        BNeutral950 => "border-b-neutral-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(250 250 249);
        /// }
        /// ```
        BStone50 => "border-b-stone-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(245 245 244);
        /// }
        /// ```
        BStone100 => "border-b-stone-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(231 229 228);
        /// }
        /// ```
        BStone200 => "border-b-stone-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(214 211 209);
        /// }
        /// ```
        BStone300 => "border-b-stone-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(168 162 158);
        /// }
        /// ```
        BStone400 => "border-b-stone-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(120 113 108);
        /// }
        /// ```
        BStone500 => "border-b-stone-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(87 83 78);
        /// }
        /// ```
        BStone600 => "border-b-stone-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(68 64 60);
        /// }
        /// ```
        BStone700 => "border-b-stone-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(41 37 36);
        /// }
        /// ```
        BStone800 => "border-b-stone-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(28 25 23);
        /// }
        /// ```
        BStone900 => "border-b-stone-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(12 10 9);
        /// }
        /// ```
        BStone950 => "border-b-stone-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 242 242);
        /// }
        /// ```
        BRed50 => "border-b-red-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 226 226);
        /// }
        /// ```
        BRed100 => "border-b-red-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 202 202);
        /// }
        /// ```
        BRed200 => "border-b-red-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(252 165 165);
        /// }
        /// ```
        BRed300 => "border-b-red-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(248 113 113);
        /// }
        /// ```
        BRed400 => "border-b-red-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(239 68 68);
        /// }
        /// ```
        BRed500 => "border-b-red-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(220 38 38);
        /// }
        /// ```
        BRed600 => "border-b-red-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(185 28 28);
        /// }
        /// ```
        BRed700 => "border-b-red-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(153 27 27);
        /// }
        /// ```
        BRed800 => "border-b-red-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(127 29 29);
        /// }
        /// ```
        BRed900 => "border-b-red-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(69 10 10);
        /// }
        /// ```
        BRed950 => "border-b-red-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(255 247 237);
        /// }
        /// ```
        BOrange50 => "border-b-orange-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(255 237 213);
        /// }
        /// ```
        BOrange100 => "border-b-orange-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 215 170);
        /// }
        /// ```
        BOrange200 => "border-b-orange-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(253 186 116);
        /// }
        /// ```
        BOrange300 => "border-b-orange-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(251 146 60);
        /// }
        /// ```
        BOrange400 => "border-b-orange-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(249 115 22);
        /// }
        /// ```
        BOrange500 => "border-b-orange-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(234 88 12);
        /// }
        /// ```
        BOrange600 => "border-b-orange-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(194 65 12);
        /// }
        /// ```
        BOrange700 => "border-b-orange-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(154 52 18);
        /// }
        /// ```
        BOrange800 => "border-b-orange-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(124 45 18);
        /// }
        /// ```
        BOrange900 => "border-b-orange-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(67 20 7);
        /// }
        /// ```
        BOrange950 => "border-b-orange-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(255 251 235);
        /// }
        /// ```
        BAmber50 => "border-b-amber-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 243 199);
        /// }
        /// ```
        BAmber100 => "border-b-amber-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(253 230 138);
        /// }
        /// ```
        BAmber200 => "border-b-amber-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(252 211 77);
        /// }
        /// ```
        BAmber300 => "border-b-amber-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(251 191 36);
        /// }
        /// ```
        BAmber400 => "border-b-amber-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(245 158 11);
        /// }
        /// ```
        BAmber500 => "border-b-amber-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(217 119 6);
        /// }
        /// ```
        BAmber600 => "border-b-amber-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(180 83 9);
        /// }
        /// ```
        BAmber700 => "border-b-amber-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(146 64 14);
        /// }
        /// ```
        BAmber800 => "border-b-amber-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(120 53 15);
        /// }
        /// ```
        BAmber900 => "border-b-amber-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(69 26 3);
        /// }
        /// ```
        BAmber950 => "border-b-amber-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 252 232);
        /// }
        /// ```
        BYellow50 => "border-b-yellow-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 249 195);
        /// }
        /// ```
        BYellow100 => "border-b-yellow-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 240 138);
        /// }
        /// ```
        BYellow200 => "border-b-yellow-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(253 224 71);
        /// }
        /// ```
        BYellow300 => "border-b-yellow-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(250 204 21);
        /// }
        /// ```
        BYellow400 => "border-b-yellow-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(234 179 8);
        /// }
        /// ```
        BYellow500 => "border-b-yellow-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(202 138 4);
        /// }
        /// ```
        BYellow600 => "border-b-yellow-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(161 98 7);
        /// }
        /// ```
        BYellow700 => "border-b-yellow-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(133 77 14);
        /// }
        /// ```
        BYellow800 => "border-b-yellow-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(113 63 18);
        /// }
        /// ```
        BYellow900 => "border-b-yellow-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(66 32 6);
        /// }
        /// ```
        BYellow950 => "border-b-yellow-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(247 254 231);
        /// }
        /// ```
        BLime50 => "border-b-lime-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(236 252 203);
        /// }
        /// ```
        BLime100 => "border-b-lime-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(217 249 157);
        /// }
        /// ```
        BLime200 => "border-b-lime-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(190 242 100);
        /// }
        /// ```
        BLime300 => "border-b-lime-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(163 230 53);
        /// }
        /// ```
        BLime400 => "border-b-lime-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(132 204 22);
        /// }
        /// ```
        BLime500 => "border-b-lime-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(101 163 13);
        /// }
        /// ```
        BLime600 => "border-b-lime-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(77 124 15);
        /// }
        /// ```
        BLime700 => "border-b-lime-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(63 98 18);
        /// }
        /// ```
        BLime800 => "border-b-lime-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(54 83 20);
        /// }
        /// ```
        BLime900 => "border-b-lime-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(26 46 5);
        /// }
        /// ```
        BLime950 => "border-b-lime-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(240 253 244);
        /// }
        /// ```
        BGreen50 => "border-b-green-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(220 252 231);
        /// }
        /// ```
        BGreen100 => "border-b-green-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(187 247 208);
        /// }
        /// ```
        BGreen200 => "border-b-green-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(134 239 172);
        /// }
        /// ```
        BGreen300 => "border-b-green-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(74 222 128);
        /// }
        /// ```
        BGreen400 => "border-b-green-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(34 197 94);
        /// }
        /// ```
        BGreen500 => "border-b-green-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(22 163 74);
        /// }
        /// ```
        BGreen600 => "border-b-green-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(21 128 61);
        /// }
        /// ```
        BGreen700 => "border-b-green-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(22 101 52);
        /// }
        /// ```
        BGreen800 => "border-b-green-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(20 83 45);
        /// }
        /// ```
        BGreen900 => "border-b-green-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(5 46 22);
        /// }
        /// ```
        BGreen950 => "border-b-green-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(236 253 245);
        /// }
        /// ```
        BEmerald50 => "border-b-emerald-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(209 250 229);
        /// }
        /// ```
        BEmerald100 => "border-b-emerald-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(167 243 208);
        /// }
        /// ```
        BEmerald200 => "border-b-emerald-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(110 231 183);
        /// }
        /// ```
        BEmerald300 => "border-b-emerald-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(52 211 153);
        /// }
        /// ```
        BEmerald400 => "border-b-emerald-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(16 185 129);
        /// }
        /// ```
        BEmerald500 => "border-b-emerald-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(5 150 105);
        /// }
        /// ```
        BEmerald600 => "border-b-emerald-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(4 120 87);
        /// }
        /// ```
        BEmerald700 => "border-b-emerald-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(6 95 70);
        /// }
        /// ```
        BEmerald800 => "border-b-emerald-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(6 78 59);
        /// }
        /// ```
        BEmerald900 => "border-b-emerald-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(2 44 34);
        /// }
        /// ```
        BEmerald950 => "border-b-emerald-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(240 253 250);
        /// }
        /// ```
        BTeal50 => "border-b-teal-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(204 251 241);
        /// }
        /// ```
        BTeal100 => "border-b-teal-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(153 246 228);
        /// }
        /// ```
        BTeal200 => "border-b-teal-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(94 234 212);
        /// }
        /// ```
        BTeal300 => "border-b-teal-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(45 212 191);
        /// }
        /// ```
        BTeal400 => "border-b-teal-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(20 184 166);
        /// }
        /// ```
        BTeal500 => "border-b-teal-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(13 148 136);
        /// }
        /// ```
        BTeal600 => "border-b-teal-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(15 118 110);
        /// }
        /// ```
        BTeal700 => "border-b-teal-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(17 94 89);
        /// }
        /// ```
        BTeal800 => "border-b-teal-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(19 78 74);
        /// }
        /// ```
        BTeal900 => "border-b-teal-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(4 47 46);
        /// }
        /// ```
        BTeal950 => "border-b-teal-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(236 254 255);
        /// }
        /// ```
        BCyan50 => "border-b-cyan-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(207 250 254);
        /// }
        /// ```
        BCyan100 => "border-b-cyan-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(165 243 252);
        /// }
        /// ```
        BCyan200 => "border-b-cyan-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(103 232 249);
        /// }
        /// ```
        BCyan300 => "border-b-cyan-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(34 211 238);
        /// }
        /// ```
        BCyan400 => "border-b-cyan-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(6 182 212);
        /// }
        /// ```
        BCyan500 => "border-b-cyan-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(8 145 178);
        /// }
        /// ```
        BCyan600 => "border-b-cyan-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(14 116 144);
        /// }
        /// ```
        BCyan700 => "border-b-cyan-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(21 94 117);
        /// }
        /// ```
        BCyan800 => "border-b-cyan-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(22 78 99);
        /// }
        /// ```
        BCyan900 => "border-b-cyan-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(8 51 68);
        /// }
        /// ```
        BCyan950 => "border-b-cyan-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(240 249 255);
        /// }
        /// ```
        BSky50 => "border-b-sky-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(224 242 254);
        /// }
        /// ```
        BSky100 => "border-b-sky-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(186 230 253);
        /// }
        /// ```
        BSky200 => "border-b-sky-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(125 211 252);
        /// }
        /// ```
        BSky300 => "border-b-sky-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(56 189 248);
        /// }
        /// ```
        BSky400 => "border-b-sky-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(14 165 233);
        /// }
        /// ```
        BSky500 => "border-b-sky-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(2 132 199);
        /// }
        /// ```
        BSky600 => "border-b-sky-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(3 105 161);
        /// }
        /// ```
        BSky700 => "border-b-sky-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(7 89 133);
        /// }
        /// ```
        BSky800 => "border-b-sky-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(12 74 110);
        /// }
        /// ```
        BSky900 => "border-b-sky-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(8 47 73);
        /// }
        /// ```
        BSky950 => "border-b-sky-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(239 246 255);
        /// }
        /// ```
        BBlue50 => "border-b-blue-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(219 234 254);
        /// }
        /// ```
        BBlue100 => "border-b-blue-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(191 219 254);
        /// }
        /// ```
        BBlue200 => "border-b-blue-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(147 197 253);
        /// }
        /// ```
        BBlue300 => "border-b-blue-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(96 165 250);
        /// }
        /// ```
        BBlue400 => "border-b-blue-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(59 130 246);
        /// }
        /// ```
        BBlue500 => "border-b-blue-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(37 99 235);
        /// }
        /// ```
        BBlue600 => "border-b-blue-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(29 78 216);
        /// }
        /// ```
        BBlue700 => "border-b-blue-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(30 64 175);
        /// }
        /// ```
        BBlue800 => "border-b-blue-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(30 58 138);
        /// }
        /// ```
        BBlue900 => "border-b-blue-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(23 37 84);
        /// }
        /// ```
        BBlue950 => "border-b-blue-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(238 242 255);
        /// }
        /// ```
        BIndigo50 => "border-b-indigo-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(224 231 255);
        /// }
        /// ```
        BIndigo100 => "border-b-indigo-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(199 210 254);
        /// }
        /// ```
        BIndigo200 => "border-b-indigo-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(165 180 252);
        /// }
        /// ```
        BIndigo300 => "border-b-indigo-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(129 140 248);
        /// }
        /// ```
        BIndigo400 => "border-b-indigo-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(99 102 241);
        /// }
        /// ```
        BIndigo500 => "border-b-indigo-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(79 70 229);
        /// }
        /// ```
        BIndigo600 => "border-b-indigo-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(67 56 202);
        /// }
        /// ```
        BIndigo700 => "border-b-indigo-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(55 48 163);
        /// }
        /// ```
        BIndigo800 => "border-b-indigo-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(49 46 129);
        /// }
        /// ```
        BIndigo900 => "border-b-indigo-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(30 27 75);
        /// }
        /// ```
        BIndigo950 => "border-b-indigo-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(245 243 255);
        /// }
        /// ```
        BViolet50 => "border-b-violet-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(237 233 254);
        /// }
        /// ```
        BViolet100 => "border-b-violet-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(221 214 254);
        /// }
        /// ```
        BViolet200 => "border-b-violet-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(196 181 253);
        /// }
        /// ```
        BViolet300 => "border-b-violet-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(167 139 250);
        /// }
        /// ```
        BViolet400 => "border-b-violet-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(139 92 246);
        /// }
        /// ```
        BViolet500 => "border-b-violet-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(124 58 237);
        /// }
        /// ```
        BViolet600 => "border-b-violet-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(109 40 217);
        /// }
        /// ```
        BViolet700 => "border-b-violet-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(91 33 182);
        /// }
        /// ```
        BViolet800 => "border-b-violet-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(76 29 149);
        /// }
        /// ```
        BViolet900 => "border-b-violet-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(46 16 101);
        /// }
        /// ```
        BViolet950 => "border-b-violet-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(250 245 255);
        /// }
        /// ```
        BPurple50 => "border-b-purple-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(243 232 255);
        /// }
        /// ```
        BPurple100 => "border-b-purple-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(233 213 255);
        /// }
        /// ```
        BPurple200 => "border-b-purple-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(216 180 254);
        /// }
        /// ```
        BPurple300 => "border-b-purple-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(192 132 252);
        /// }
        /// ```
        BPurple400 => "border-b-purple-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(168 85 247);
        /// }
        /// ```
        BPurple500 => "border-b-purple-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(147 51 234);
        /// }
        /// ```
        BPurple600 => "border-b-purple-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(126 34 206);
        /// }
        /// ```
        BPurple700 => "border-b-purple-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(107 33 168);
        /// }
        /// ```
        BPurple800 => "border-b-purple-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(88 28 135);
        /// }
        /// ```
        BPurple900 => "border-b-purple-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(59 7 100);
        /// }
        /// ```
        BPurple950 => "border-b-purple-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(253 244 255);
        /// }
        /// ```
        BFuchsia50 => "border-b-fuchsia-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(250 232 255);
        /// }
        /// ```
        BFuchsia100 => "border-b-fuchsia-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(245 208 254);
        /// }
        /// ```
        BFuchsia200 => "border-b-fuchsia-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(240 171 252);
        /// }
        /// ```
        BFuchsia300 => "border-b-fuchsia-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(232 121 249);
        /// }
        /// ```
        BFuchsia400 => "border-b-fuchsia-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(217 70 239);
        /// }
        /// ```
        BFuchsia500 => "border-b-fuchsia-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(192 38 211);
        /// }
        /// ```
        BFuchsia600 => "border-b-fuchsia-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(162 28 175);
        /// }
        /// ```
        BFuchsia700 => "border-b-fuchsia-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(134 25 143);
        /// }
        /// ```
        BFuchsia800 => "border-b-fuchsia-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(112 26 117);
        /// }
        /// ```
        BFuchsia900 => "border-b-fuchsia-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(74 4 78);
        /// }
        /// ```
        BFuchsia950 => "border-b-fuchsia-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(253 242 248);
        /// }
        /// ```
        BPink50 => "border-b-pink-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(252 231 243);
        /// }
        /// ```
        BPink100 => "border-b-pink-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(251 207 232);
        /// }
        /// ```
        BPink200 => "border-b-pink-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(249 168 212);
        /// }
        /// ```
        BPink300 => "border-b-pink-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(244 114 182);
        /// }
        /// ```
        BPink400 => "border-b-pink-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(236 72 153);
        /// }
        /// ```
        BPink500 => "border-b-pink-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(219 39 119);
        /// }
        /// ```
        BPink600 => "border-b-pink-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(190 24 93);
        /// }
        /// ```
        BPink700 => "border-b-pink-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(157 23 77);
        /// }
        /// ```
        BPink800 => "border-b-pink-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(131 24 67);
        /// }
        /// ```
        BPink900 => "border-b-pink-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(80 7 36);
        /// }
        /// ```
        BPink950 => "border-b-pink-950",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(255 241 242);
        /// }
        /// ```
        BRose50 => "border-b-rose-50",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(255 228 230);
        /// }
        /// ```
        BRose100 => "border-b-rose-100",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(254 205 211);
        /// }
        /// ```
        BRose200 => "border-b-rose-200",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(253 164 175);
        /// }
        /// ```
        BRose300 => "border-b-rose-300",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(251 113 133);
        /// }
        /// ```
        BRose400 => "border-b-rose-400",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(244 63 94);
        /// }
        /// ```
        BRose500 => "border-b-rose-500",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(225 29 72);
        /// }
        /// ```
        BRose600 => "border-b-rose-600",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(190 18 60);
        /// }
        /// ```
        BRose700 => "border-b-rose-700",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(159 18 57);
        /// }
        /// ```
        BRose800 => "border-b-rose-800",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(136 19 55);
        /// }
        /// ```
        BRose900 => "border-b-rose-900",
        /// ```css
        /// {
        ///     border-bottom-color: rgb(76 5 25);
        /// }
        /// ```
        BRose950 => "border-b-rose-950",
        /// ```css
        /// {
        ///     border-left-color: inherit;
        /// }
        /// ```
        LInherit => "border-l-inherit",
        /// ```css
        /// {
        ///     border-left-color: currentColor;
        /// }
        /// ```
        LCurrent => "border-l-current",
        /// ```css
        /// {
        ///     border-left-color: transparent;
        /// }
        /// ```
        LTransparent => "border-l-transparent",
        /// ```css
        /// {
        ///     border-left-color: rgb(0 0 0);
        /// }
        /// ```
        LBlack => "border-l-black",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 255 255);
        /// }
        /// ```
        LWhite => "border-l-white",
        /// ```css
        /// {
        ///     border-left-color: rgb(248 250 252);
        /// }
        /// ```
        LSlate50 => "border-l-slate-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(241 245 249);
        /// }
        /// ```
        LSlate100 => "border-l-slate-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(226 232 240);
        /// }
        /// ```
        LSlate200 => "border-l-slate-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(203 213 225);
        /// }
        /// ```
        LSlate300 => "border-l-slate-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(148 163 184);
        /// }
        /// ```
        LSlate400 => "border-l-slate-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(100 116 139);
        /// }
        /// ```
        LSlate500 => "border-l-slate-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(71 85 105);
        /// }
        /// ```
        LSlate600 => "border-l-slate-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(51 65 85);
        /// }
        /// ```
        LSlate700 => "border-l-slate-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(30 41 59);
        /// }
        /// ```
        LSlate800 => "border-l-slate-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(15 23 42);
        /// }
        /// ```
        LSlate900 => "border-l-slate-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(2 6 23);
        /// }
        /// ```
        LSlate950 => "border-l-slate-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(249 250 251);
        /// }
        /// ```
        LGray50 => "border-l-gray-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(243 244 246);
        /// }
        /// ```
        LGray100 => "border-l-gray-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(229 231 235);
        /// }
        /// ```
        LGray200 => "border-l-gray-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(209 213 219);
        /// }
        /// ```
        LGray300 => "border-l-gray-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(156 163 175);
        /// }
        /// ```
        LGray400 => "border-l-gray-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(107 114 128);
        /// }
        /// ```
        LGray500 => "border-l-gray-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(75 85 99);
        /// }
        /// ```
        LGray600 => "border-l-gray-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(55 65 81);
        /// }
        /// ```
        LGray700 => "border-l-gray-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(31 41 55);
        /// }
        /// ```
        LGray800 => "border-l-gray-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(17 24 39);
        /// }
        /// ```
        LGray900 => "border-l-gray-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(3 7 18);
        /// }
        /// ```
        LGray950 => "border-l-gray-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 250 250);
        /// }
        /// ```
        LZinc50 => "border-l-zinc-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(244 244 245);
        /// }
        /// ```
        LZinc100 => "border-l-zinc-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(228 228 231);
        /// }
        /// ```
        LZinc200 => "border-l-zinc-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(212 212 216);
        /// }
        /// ```
        LZinc300 => "border-l-zinc-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(161 161 170);
        /// }
        /// ```
        LZinc400 => "border-l-zinc-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(113 113 122);
        /// }
        /// ```
        LZinc500 => "border-l-zinc-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(82 82 91);
        /// }
        /// ```
        LZinc600 => "border-l-zinc-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(63 63 70);
        /// }
        /// ```
        LZinc700 => "border-l-zinc-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(39 39 42);
        /// }
        /// ```
        LZinc800 => "border-l-zinc-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(24 24 27);
        /// }
        /// ```
        LZinc900 => "border-l-zinc-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(9 9 11);
        /// }
        /// ```
        LZinc950 => "border-l-zinc-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 250 250);
        /// }
        /// ```
        LNeutral50 => "border-l-neutral-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 245 245);
        /// }
        /// ```
        LNeutral100 => "border-l-neutral-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(229 229 229);
        /// }
        /// ```
        LNeutral200 => "border-l-neutral-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(212 212 212);
        /// }
        /// ```
        LNeutral300 => "border-l-neutral-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(163 163 163);
        /// }
        /// ```
        LNeutral400 => "border-l-neutral-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(115 115 115);
        /// }
        /// ```
        LNeutral500 => "border-l-neutral-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(82 82 82);
        /// }
        /// ```
        LNeutral600 => "border-l-neutral-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(64 64 64);
        /// }
        /// ```
        LNeutral700 => "border-l-neutral-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(38 38 38);
        /// }
        /// ```
        LNeutral800 => "border-l-neutral-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(23 23 23);
        /// }
        /// ```
        LNeutral900 => "border-l-neutral-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(10 10 10);
        /// }
        /// ```
        LNeutral950 => "border-l-neutral-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 250 249);
        /// }
        /// ```
        LStone50 => "border-l-stone-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 245 244);
        /// }
        /// ```
        LStone100 => "border-l-stone-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(231 229 228);
        /// }
        /// ```
        LStone200 => "border-l-stone-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(214 211 209);
        /// }
        /// ```
        LStone300 => "border-l-stone-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(168 162 158);
        /// }
        /// ```
        LStone400 => "border-l-stone-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(120 113 108);
        /// }
        /// ```
        LStone500 => "border-l-stone-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(87 83 78);
        /// }
        /// ```
        LStone600 => "border-l-stone-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(68 64 60);
        /// }
        /// ```
        LStone700 => "border-l-stone-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(41 37 36);
        /// }
        /// ```
        LStone800 => "border-l-stone-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(28 25 23);
        /// }
        /// ```
        LStone900 => "border-l-stone-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(12 10 9);
        /// }
        /// ```
        LStone950 => "border-l-stone-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 242 242);
        /// }
        /// ```
        LRed50 => "border-l-red-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 226 226);
        /// }
        /// ```
        LRed100 => "border-l-red-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 202 202);
        /// }
        /// ```
        LRed200 => "border-l-red-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(252 165 165);
        /// }
        /// ```
        LRed300 => "border-l-red-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(248 113 113);
        /// }
        /// ```
        LRed400 => "border-l-red-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(239 68 68);
        /// }
        /// ```
        LRed500 => "border-l-red-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(220 38 38);
        /// }
        /// ```
        LRed600 => "border-l-red-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(185 28 28);
        /// }
        /// ```
        LRed700 => "border-l-red-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(153 27 27);
        /// }
        /// ```
        LRed800 => "border-l-red-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(127 29 29);
        /// }
        /// ```
        LRed900 => "border-l-red-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(69 10 10);
        /// }
        /// ```
        LRed950 => "border-l-red-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 247 237);
        /// }
        /// ```
        LOrange50 => "border-l-orange-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 237 213);
        /// }
        /// ```
        LOrange100 => "border-l-orange-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 215 170);
        /// }
        /// ```
        LOrange200 => "border-l-orange-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 186 116);
        /// }
        /// ```
        LOrange300 => "border-l-orange-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(251 146 60);
        /// }
        /// ```
        LOrange400 => "border-l-orange-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(249 115 22);
        /// }
        /// ```
        LOrange500 => "border-l-orange-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(234 88 12);
        /// }
        /// ```
        LOrange600 => "border-l-orange-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(194 65 12);
        /// }
        /// ```
        LOrange700 => "border-l-orange-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(154 52 18);
        /// }
        /// ```
        LOrange800 => "border-l-orange-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(124 45 18);
        /// }
        /// ```
        LOrange900 => "border-l-orange-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(67 20 7);
        /// }
        /// ```
        LOrange950 => "border-l-orange-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 251 235);
        /// }
        /// ```
        LAmber50 => "border-l-amber-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 243 199);
        /// }
        /// ```
        LAmber100 => "border-l-amber-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 230 138);
        /// }
        /// ```
        LAmber200 => "border-l-amber-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(252 211 77);
        /// }
        /// ```
        LAmber300 => "border-l-amber-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(251 191 36);
        /// }
        /// ```
        LAmber400 => "border-l-amber-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 158 11);
        /// }
        /// ```
        LAmber500 => "border-l-amber-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(217 119 6);
        /// }
        /// ```
        LAmber600 => "border-l-amber-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(180 83 9);
        /// }
        /// ```
        LAmber700 => "border-l-amber-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(146 64 14);
        /// }
        /// ```
        LAmber800 => "border-l-amber-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(120 53 15);
        /// }
        /// ```
        LAmber900 => "border-l-amber-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(69 26 3);
        /// }
        /// ```
        LAmber950 => "border-l-amber-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 252 232);
        /// }
        /// ```
        LYellow50 => "border-l-yellow-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 249 195);
        /// }
        /// ```
        LYellow100 => "border-l-yellow-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 240 138);
        /// }
        /// ```
        LYellow200 => "border-l-yellow-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 224 71);
        /// }
        /// ```
        LYellow300 => "border-l-yellow-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 204 21);
        /// }
        /// ```
        LYellow400 => "border-l-yellow-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(234 179 8);
        /// }
        /// ```
        LYellow500 => "border-l-yellow-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(202 138 4);
        /// }
        /// ```
        LYellow600 => "border-l-yellow-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(161 98 7);
        /// }
        /// ```
        LYellow700 => "border-l-yellow-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(133 77 14);
        /// }
        /// ```
        LYellow800 => "border-l-yellow-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(113 63 18);
        /// }
        /// ```
        LYellow900 => "border-l-yellow-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(66 32 6);
        /// }
        /// ```
        LYellow950 => "border-l-yellow-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(247 254 231);
        /// }
        /// ```
        LLime50 => "border-l-lime-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(236 252 203);
        /// }
        /// ```
        LLime100 => "border-l-lime-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(217 249 157);
        /// }
        /// ```
        LLime200 => "border-l-lime-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(190 242 100);
        /// }
        /// ```
        LLime300 => "border-l-lime-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(163 230 53);
        /// }
        /// ```
        LLime400 => "border-l-lime-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(132 204 22);
        /// }
        /// ```
        LLime500 => "border-l-lime-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(101 163 13);
        /// }
        /// ```
        LLime600 => "border-l-lime-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(77 124 15);
        /// }
        /// ```
        LLime700 => "border-l-lime-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(63 98 18);
        /// }
        /// ```
        LLime800 => "border-l-lime-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(54 83 20);
        /// }
        /// ```
        LLime900 => "border-l-lime-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(26 46 5);
        /// }
        /// ```
        LLime950 => "border-l-lime-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(240 253 244);
        /// }
        /// ```
        LGreen50 => "border-l-green-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(220 252 231);
        /// }
        /// ```
        LGreen100 => "border-l-green-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(187 247 208);
        /// }
        /// ```
        LGreen200 => "border-l-green-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(134 239 172);
        /// }
        /// ```
        LGreen300 => "border-l-green-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(74 222 128);
        /// }
        /// ```
        LGreen400 => "border-l-green-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(34 197 94);
        /// }
        /// ```
        LGreen500 => "border-l-green-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(22 163 74);
        /// }
        /// ```
        LGreen600 => "border-l-green-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(21 128 61);
        /// }
        /// ```
        LGreen700 => "border-l-green-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(22 101 52);
        /// }
        /// ```
        LGreen800 => "border-l-green-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(20 83 45);
        /// }
        /// ```
        LGreen900 => "border-l-green-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(5 46 22);
        /// }
        /// ```
        LGreen950 => "border-l-green-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(236 253 245);
        /// }
        /// ```
        LEmerald50 => "border-l-emerald-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(209 250 229);
        /// }
        /// ```
        LEmerald100 => "border-l-emerald-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(167 243 208);
        /// }
        /// ```
        LEmerald200 => "border-l-emerald-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(110 231 183);
        /// }
        /// ```
        LEmerald300 => "border-l-emerald-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(52 211 153);
        /// }
        /// ```
        LEmerald400 => "border-l-emerald-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(16 185 129);
        /// }
        /// ```
        LEmerald500 => "border-l-emerald-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(5 150 105);
        /// }
        /// ```
        LEmerald600 => "border-l-emerald-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(4 120 87);
        /// }
        /// ```
        LEmerald700 => "border-l-emerald-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(6 95 70);
        /// }
        /// ```
        LEmerald800 => "border-l-emerald-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(6 78 59);
        /// }
        /// ```
        LEmerald900 => "border-l-emerald-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(2 44 34);
        /// }
        /// ```
        LEmerald950 => "border-l-emerald-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(240 253 250);
        /// }
        /// ```
        LTeal50 => "border-l-teal-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(204 251 241);
        /// }
        /// ```
        LTeal100 => "border-l-teal-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(153 246 228);
        /// }
        /// ```
        LTeal200 => "border-l-teal-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(94 234 212);
        /// }
        /// ```
        LTeal300 => "border-l-teal-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(45 212 191);
        /// }
        /// ```
        LTeal400 => "border-l-teal-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(20 184 166);
        /// }
        /// ```
        LTeal500 => "border-l-teal-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(13 148 136);
        /// }
        /// ```
        LTeal600 => "border-l-teal-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(15 118 110);
        /// }
        /// ```
        LTeal700 => "border-l-teal-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(17 94 89);
        /// }
        /// ```
        LTeal800 => "border-l-teal-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(19 78 74);
        /// }
        /// ```
        LTeal900 => "border-l-teal-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(4 47 46);
        /// }
        /// ```
        LTeal950 => "border-l-teal-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(236 254 255);
        /// }
        /// ```
        LCyan50 => "border-l-cyan-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(207 250 254);
        /// }
        /// ```
        LCyan100 => "border-l-cyan-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(165 243 252);
        /// }
        /// ```
        LCyan200 => "border-l-cyan-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(103 232 249);
        /// }
        /// ```
        LCyan300 => "border-l-cyan-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(34 211 238);
        /// }
        /// ```
        LCyan400 => "border-l-cyan-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(6 182 212);
        /// }
        /// ```
        LCyan500 => "border-l-cyan-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(8 145 178);
        /// }
        /// ```
        LCyan600 => "border-l-cyan-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(14 116 144);
        /// }
        /// ```
        LCyan700 => "border-l-cyan-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(21 94 117);
        /// }
        /// ```
        LCyan800 => "border-l-cyan-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(22 78 99);
        /// }
        /// ```
        LCyan900 => "border-l-cyan-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(8 51 68);
        /// }
        /// ```
        LCyan950 => "border-l-cyan-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(240 249 255);
        /// }
        /// ```
        LSky50 => "border-l-sky-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(224 242 254);
        /// }
        /// ```
        LSky100 => "border-l-sky-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(186 230 253);
        /// }
        /// ```
        LSky200 => "border-l-sky-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(125 211 252);
        /// }
        /// ```
        LSky300 => "border-l-sky-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(56 189 248);
        /// }
        /// ```
        LSky400 => "border-l-sky-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(14 165 233);
        /// }
        /// ```
        LSky500 => "border-l-sky-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(2 132 199);
        /// }
        /// ```
        LSky600 => "border-l-sky-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(3 105 161);
        /// }
        /// ```
        LSky700 => "border-l-sky-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(7 89 133);
        /// }
        /// ```
        LSky800 => "border-l-sky-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(12 74 110);
        /// }
        /// ```
        LSky900 => "border-l-sky-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(8 47 73);
        /// }
        /// ```
        LSky950 => "border-l-sky-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(239 246 255);
        /// }
        /// ```
        LBlue50 => "border-l-blue-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(219 234 254);
        /// }
        /// ```
        LBlue100 => "border-l-blue-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(191 219 254);
        /// }
        /// ```
        LBlue200 => "border-l-blue-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(147 197 253);
        /// }
        /// ```
        LBlue300 => "border-l-blue-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(96 165 250);
        /// }
        /// ```
        LBlue400 => "border-l-blue-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(59 130 246);
        /// }
        /// ```
        LBlue500 => "border-l-blue-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(37 99 235);
        /// }
        /// ```
        LBlue600 => "border-l-blue-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(29 78 216);
        /// }
        /// ```
        LBlue700 => "border-l-blue-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(30 64 175);
        /// }
        /// ```
        LBlue800 => "border-l-blue-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(30 58 138);
        /// }
        /// ```
        LBlue900 => "border-l-blue-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(23 37 84);
        /// }
        /// ```
        LBlue950 => "border-l-blue-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(238 242 255);
        /// }
        /// ```
        LIndigo50 => "border-l-indigo-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(224 231 255);
        /// }
        /// ```
        LIndigo100 => "border-l-indigo-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(199 210 254);
        /// }
        /// ```
        LIndigo200 => "border-l-indigo-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(165 180 252);
        /// }
        /// ```
        LIndigo300 => "border-l-indigo-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(129 140 248);
        /// }
        /// ```
        LIndigo400 => "border-l-indigo-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(99 102 241);
        /// }
        /// ```
        LIndigo500 => "border-l-indigo-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(79 70 229);
        /// }
        /// ```
        LIndigo600 => "border-l-indigo-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(67 56 202);
        /// }
        /// ```
        LIndigo700 => "border-l-indigo-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(55 48 163);
        /// }
        /// ```
        LIndigo800 => "border-l-indigo-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(49 46 129);
        /// }
        /// ```
        LIndigo900 => "border-l-indigo-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(30 27 75);
        /// }
        /// ```
        LIndigo950 => "border-l-indigo-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 243 255);
        /// }
        /// ```
        LViolet50 => "border-l-violet-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(237 233 254);
        /// }
        /// ```
        LViolet100 => "border-l-violet-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(221 214 254);
        /// }
        /// ```
        LViolet200 => "border-l-violet-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(196 181 253);
        /// }
        /// ```
        LViolet300 => "border-l-violet-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(167 139 250);
        /// }
        /// ```
        LViolet400 => "border-l-violet-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(139 92 246);
        /// }
        /// ```
        LViolet500 => "border-l-violet-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(124 58 237);
        /// }
        /// ```
        LViolet600 => "border-l-violet-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(109 40 217);
        /// }
        /// ```
        LViolet700 => "border-l-violet-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(91 33 182);
        /// }
        /// ```
        LViolet800 => "border-l-violet-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(76 29 149);
        /// }
        /// ```
        LViolet900 => "border-l-violet-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(46 16 101);
        /// }
        /// ```
        LViolet950 => "border-l-violet-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 245 255);
        /// }
        /// ```
        LPurple50 => "border-l-purple-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(243 232 255);
        /// }
        /// ```
        LPurple100 => "border-l-purple-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(233 213 255);
        /// }
        /// ```
        LPurple200 => "border-l-purple-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(216 180 254);
        /// }
        /// ```
        LPurple300 => "border-l-purple-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(192 132 252);
        /// }
        /// ```
        LPurple400 => "border-l-purple-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(168 85 247);
        /// }
        /// ```
        LPurple500 => "border-l-purple-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(147 51 234);
        /// }
        /// ```
        LPurple600 => "border-l-purple-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(126 34 206);
        /// }
        /// ```
        LPurple700 => "border-l-purple-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(107 33 168);
        /// }
        /// ```
        LPurple800 => "border-l-purple-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(88 28 135);
        /// }
        /// ```
        LPurple900 => "border-l-purple-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(59 7 100);
        /// }
        /// ```
        LPurple950 => "border-l-purple-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 244 255);
        /// }
        /// ```
        LFuchsia50 => "border-l-fuchsia-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(250 232 255);
        /// }
        /// ```
        LFuchsia100 => "border-l-fuchsia-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(245 208 254);
        /// }
        /// ```
        LFuchsia200 => "border-l-fuchsia-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(240 171 252);
        /// }
        /// ```
        LFuchsia300 => "border-l-fuchsia-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(232 121 249);
        /// }
        /// ```
        LFuchsia400 => "border-l-fuchsia-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(217 70 239);
        /// }
        /// ```
        LFuchsia500 => "border-l-fuchsia-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(192 38 211);
        /// }
        /// ```
        LFuchsia600 => "border-l-fuchsia-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(162 28 175);
        /// }
        /// ```
        LFuchsia700 => "border-l-fuchsia-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(134 25 143);
        /// }
        /// ```
        LFuchsia800 => "border-l-fuchsia-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(112 26 117);
        /// }
        /// ```
        LFuchsia900 => "border-l-fuchsia-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(74 4 78);
        /// }
        /// ```
        LFuchsia950 => "border-l-fuchsia-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 242 248);
        /// }
        /// ```
        LPink50 => "border-l-pink-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(252 231 243);
        /// }
        /// ```
        LPink100 => "border-l-pink-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(251 207 232);
        /// }
        /// ```
        LPink200 => "border-l-pink-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(249 168 212);
        /// }
        /// ```
        LPink300 => "border-l-pink-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(244 114 182);
        /// }
        /// ```
        LPink400 => "border-l-pink-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(236 72 153);
        /// }
        /// ```
        LPink500 => "border-l-pink-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(219 39 119);
        /// }
        /// ```
        LPink600 => "border-l-pink-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(190 24 93);
        /// }
        /// ```
        LPink700 => "border-l-pink-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(157 23 77);
        /// }
        /// ```
        LPink800 => "border-l-pink-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(131 24 67);
        /// }
        /// ```
        LPink900 => "border-l-pink-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(80 7 36);
        /// }
        /// ```
        LPink950 => "border-l-pink-950",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 241 242);
        /// }
        /// ```
        LRose50 => "border-l-rose-50",
        /// ```css
        /// {
        ///     border-left-color: rgb(255 228 230);
        /// }
        /// ```
        LRose100 => "border-l-rose-100",
        /// ```css
        /// {
        ///     border-left-color: rgb(254 205 211);
        /// }
        /// ```
        LRose200 => "border-l-rose-200",
        /// ```css
        /// {
        ///     border-left-color: rgb(253 164 175);
        /// }
        /// ```
        LRose300 => "border-l-rose-300",
        /// ```css
        /// {
        ///     border-left-color: rgb(251 113 133);
        /// }
        /// ```
        LRose400 => "border-l-rose-400",
        /// ```css
        /// {
        ///     border-left-color: rgb(244 63 94);
        /// }
        /// ```
        LRose500 => "border-l-rose-500",
        /// ```css
        /// {
        ///     border-left-color: rgb(225 29 72);
        /// }
        /// ```
        LRose600 => "border-l-rose-600",
        /// ```css
        /// {
        ///     border-left-color: rgb(190 18 60);
        /// }
        /// ```
        LRose700 => "border-l-rose-700",
        /// ```css
        /// {
        ///     border-left-color: rgb(159 18 57);
        /// }
        /// ```
        LRose800 => "border-l-rose-800",
        /// ```css
        /// {
        ///     border-left-color: rgb(136 19 55);
        /// }
        /// ```
        LRose900 => "border-l-rose-900",
        /// ```css
        /// {
        ///     border-left-color: rgb(76 5 25);
        /// }
        /// ```
        LRose950 => "border-l-rose-950",
    }
    /// Utilities for controlling the style of an element's borders.
    ///
    /// <https://tailwindcss.com/docs/border-style>
    BorderStyle {
        /// ```css
        /// {
        ///     border-style: solid;
        /// }
        /// ```
        Solid => "border-solid",
        /// ```css
        /// {
        ///     border-style: dashed;
        /// }
        /// ```
        Dashed => "border-dashed",
        /// ```css
        /// {
        ///     border-style: dotted;
        /// }
        /// ```
        Dotted => "border-dotted",
        /// ```css
        /// {
        ///     border-style: double;
        /// }
        /// ```
        Double => "border-double",
        /// ```css
        /// {
        ///     border-style: hidden;
        /// }
        /// ```
        Hidden => "border-hidden",
        /// ```css
        /// {
        ///     border-style: none;
        /// }
        /// ```
        None => "border-none",
    }
    /// Utilities for controlling the border width between elements.
    ///
    /// <https://tailwindcss.com/docs/divide-width>
    DivideWidth {
        /// ```css
        /// {
        ///     border-right-width: 0px;
        ///     border-left-width: 0px;
        /// }
        /// ```
        X0 => "divide-x-0",
        /// ```css
        /// {
        ///     border-right-width: 0px;
        ///     border-left-width: 2px;
        /// }
        /// ```
        X2 => "divide-x-2",
        /// ```css
        /// {
        ///     border-right-width: 0px;
        ///     border-left-width: 4px;
        /// }
        /// ```
        X4 => "divide-x-4",
        /// ```css
        /// {
        ///     border-right-width: 0px;
        ///     border-left-width: 8px;
        /// }
        /// ```
        X8 => "divide-x-8",
        /// ```css
        /// {
        ///     border-right-width: 0px;
        ///     border-left-width: 1px;
        /// }
        /// ```
        X => "divide-x",
        /// ```css
        /// {
        ///     border-top-width: 0px;
        ///     border-bottom-width: 0px;
        /// }
        /// ```
        Y0 => "divide-y-0",
        /// ```css
        /// {
        ///     border-top-width: 2px;
        ///     border-bottom-width: 0px;
        /// }
        /// ```
        Y2 => "divide-y-2",
        /// ```css
        /// {
        ///     border-top-width: 4px;
        ///     border-bottom-width: 0px;
        /// }
        /// ```
        Y4 => "divide-y-4",
        /// ```css
        /// {
        ///     border-top-width: 8px;
        ///     border-bottom-width: 0px;
        /// }
        /// ```
        Y8 => "divide-y-8",
        /// ```css
        /// {
        ///     border-top-width: 1px;
        ///     border-bottom-width: 0px;
        /// }
        /// ```
        Y => "divide-y",
        /// ```css
        /// {
        ///     --tw-divide-y-reverse: 1;
        /// }
        /// ```
        YReverse => "divide-y-reverse",
        /// ```css
        /// {
        ///     --tw-divide-x-reverse: 1;
        /// }
        /// ```
        XReverse => "divide-x-reverse",
    }
    /// Utilities for controlling the border color between elements.
    ///
    /// <https://tailwindcss.com/docs/divide-color>
    DivideColor {
        /// ```css
        /// {
        ///     border-color: inherit;
        /// }
        /// ```
        Inherit => "divide-inherit",
        /// ```css
        /// {
        ///     border-color: currentColor;
        /// }
        /// ```
        Current => "divide-current",
        /// ```css
        /// {
        ///     border-color: transparent;
        /// }
        /// ```
        Transparent => "divide-transparent",
        /// ```css
        /// {
        ///     border-color: rgb(0 0 0);
        /// }
        /// ```
        Black => "divide-black",
        /// ```css
        /// {
        ///     border-color: rgb(255 255 255);
        /// }
        /// ```
        White => "divide-white",
        /// ```css
        /// {
        ///     border-color: rgb(248 250 252);
        /// }
        /// ```
        Slate50 => "divide-slate-50",
        /// ```css
        /// {
        ///     border-color: rgb(241 245 249);
        /// }
        /// ```
        Slate100 => "divide-slate-100",
        /// ```css
        /// {
        ///     border-color: rgb(226 232 240);
        /// }
        /// ```
        Slate200 => "divide-slate-200",
        /// ```css
        /// {
        ///     border-color: rgb(203 213 225);
        /// }
        /// ```
        Slate300 => "divide-slate-300",
        /// ```css
        /// {
        ///     border-color: rgb(148 163 184);
        /// }
        /// ```
        Slate400 => "divide-slate-400",
        /// ```css
        /// {
        ///     border-color: rgb(100 116 139);
        /// }
        /// ```
        Slate500 => "divide-slate-500",
        /// ```css
        /// {
        ///     border-color: rgb(71 85 105);
        /// }
        /// ```
        Slate600 => "divide-slate-600",
        /// ```css
        /// {
        ///     border-color: rgb(51 65 85);
        /// }
        /// ```
        Slate700 => "divide-slate-700",
        /// ```css
        /// {
        ///     border-color: rgb(30 41 59);
        /// }
        /// ```
        Slate800 => "divide-slate-800",
        /// ```css
        /// {
        ///     border-color: rgb(15 23 42);
        /// }
        /// ```
        Slate900 => "divide-slate-900",
        /// ```css
        /// {
        ///     border-color: rgb(2 6 23);
        /// }
        /// ```
        Slate950 => "divide-slate-950",
        /// ```css
        /// {
        ///     border-color: rgb(249 250 251);
        /// }
        /// ```
        Gray50 => "divide-gray-50",
        /// ```css
        /// {
        ///     border-color: rgb(243 244 246);
        /// }
        /// ```
        Gray100 => "divide-gray-100",
        /// ```css
        /// {
        ///     border-color: rgb(229 231 235);
        /// }
        /// ```
        Gray200 => "divide-gray-200",
        /// ```css
        /// {
        ///     border-color: rgb(209 213 219);
        /// }
        /// ```
        Gray300 => "divide-gray-300",
        /// ```css
        /// {
        ///     border-color: rgb(156 163 175);
        /// }
        /// ```
        Gray400 => "divide-gray-400",
        /// ```css
        /// {
        ///     border-color: rgb(107 114 128);
        /// }
        /// ```
        Gray500 => "divide-gray-500",
        /// ```css
        /// {
        ///     border-color: rgb(75 85 99);
        /// }
        /// ```
        Gray600 => "divide-gray-600",
        /// ```css
        /// {
        ///     border-color: rgb(55 65 81);
        /// }
        /// ```
        Gray700 => "divide-gray-700",
        /// ```css
        /// {
        ///     border-color: rgb(31 41 55);
        /// }
        /// ```
        Gray800 => "divide-gray-800",
        /// ```css
        /// {
        ///     border-color: rgb(17 24 39);
        /// }
        /// ```
        Gray900 => "divide-gray-900",
        /// ```css
        /// {
        ///     border-color: rgb(3 7 18);
        /// }
        /// ```
        Gray950 => "divide-gray-950",
        /// ```css
        /// {
        ///     border-color: rgb(250 250 250);
        /// }
        /// ```
        Zinc50 => "divide-zinc-50",
        /// ```css
        /// {
        ///     border-color: rgb(244 244 245);
        /// }
        /// ```
        Zinc100 => "divide-zinc-100",
        /// ```css
        /// {
        ///     border-color: rgb(228 228 231);
        /// }
        /// ```
        Zinc200 => "divide-zinc-200",
        /// ```css
        /// {
        ///     border-color: rgb(212 212 216);
        /// }
        /// ```
        Zinc300 => "divide-zinc-300",
        /// ```css
        /// {
        ///     border-color: rgb(161 161 170);
        /// }
        /// ```
        Zinc400 => "divide-zinc-400",
        /// ```css
        /// {
        ///     border-color: rgb(113 113 122);
        /// }
        /// ```
        Zinc500 => "divide-zinc-500",
        /// ```css
        /// {
        ///     border-color: rgb(82 82 91);
        /// }
        /// ```
        Zinc600 => "divide-zinc-600",
        /// ```css
        /// {
        ///     border-color: rgb(63 63 70);
        /// }
        /// ```
        Zinc700 => "divide-zinc-700",
        /// ```css
        /// {
        ///     border-color: rgb(39 39 42);
        /// }
        /// ```
        Zinc800 => "divide-zinc-800",
        /// ```css
        /// {
        ///     border-color: rgb(24 24 27);
        /// }
        /// ```
        Zinc900 => "divide-zinc-900",
        /// ```css
        /// {
        ///     border-color: rgb(9 9 11);
        /// }
        /// ```
        Zinc950 => "divide-zinc-950",
        /// ```css
        /// {
        ///     border-color: rgb(250 250 250);
        /// }
        /// ```
        Neutral50 => "divide-neutral-50",
        /// ```css
        /// {
        ///     border-color: rgb(245 245 245);
        /// }
        /// ```
        Neutral100 => "divide-neutral-100",
        /// ```css
        /// {
        ///     border-color: rgb(229 229 229);
        /// }
        /// ```
        Neutral200 => "divide-neutral-200",
        /// ```css
        /// {
        ///     border-color: rgb(212 212 212);
        /// }
        /// ```
        Neutral300 => "divide-neutral-300",
        /// ```css
        /// {
        ///     border-color: rgb(163 163 163);
        /// }
        /// ```
        Neutral400 => "divide-neutral-400",
        /// ```css
        /// {
        ///     border-color: rgb(115 115 115);
        /// }
        /// ```
        Neutral500 => "divide-neutral-500",
        /// ```css
        /// {
        ///     border-color: rgb(82 82 82);
        /// }
        /// ```
        Neutral600 => "divide-neutral-600",
        /// ```css
        /// {
        ///     border-color: rgb(64 64 64);
        /// }
        /// ```
        Neutral700 => "divide-neutral-700",
        /// ```css
        /// {
        ///     border-color: rgb(38 38 38);
        /// }
        /// ```
        Neutral800 => "divide-neutral-800",
        /// ```css
        /// {
        ///     border-color: rgb(23 23 23);
        /// }
        /// ```
        Neutral900 => "divide-neutral-900",
        /// ```css
        /// {
        ///     border-color: rgb(10 10 10);
        /// }
        /// ```
        Neutral950 => "divide-neutral-950",
        /// ```css
        /// {
        ///     border-color: rgb(250 250 249);
        /// }
        /// ```
        Stone50 => "divide-stone-50",
        /// ```css
        /// {
        ///     border-color: rgb(245 245 244);
        /// }
        /// ```
        Stone100 => "divide-stone-100",
        /// ```css
        /// {
        ///     border-color: rgb(231 229 228);
        /// }
        /// ```
        Stone200 => "divide-stone-200",
        /// ```css
        /// {
        ///     border-color: rgb(214 211 209);
        /// }
        /// ```
        Stone300 => "divide-stone-300",
        /// ```css
        /// {
        ///     border-color: rgb(168 162 158);
        /// }
        /// ```
        Stone400 => "divide-stone-400",
        /// ```css
        /// {
        ///     border-color: rgb(120 113 108);
        /// }
        /// ```
        Stone500 => "divide-stone-500",
        /// ```css
        /// {
        ///     border-color: rgb(87 83 78);
        /// }
        /// ```
        Stone600 => "divide-stone-600",
        /// ```css
        /// {
        ///     border-color: rgb(68 64 60);
        /// }
        /// ```
        Stone700 => "divide-stone-700",
        /// ```css
        /// {
        ///     border-color: rgb(41 37 36);
        /// }
        /// ```
        Stone800 => "divide-stone-800",
        /// ```css
        /// {
        ///     border-color: rgb(28 25 23);
        /// }
        /// ```
        Stone900 => "divide-stone-900",
        /// ```css
        /// {
        ///     border-color: rgb(12 10 9);
        /// }
        /// ```
        Stone950 => "divide-stone-950",
        /// ```css
        /// {
        ///     border-color: rgb(254 242 242);
        /// }
        /// ```
        Red50 => "divide-red-50",
        /// ```css
        /// {
        ///     border-color: rgb(254 226 226);
        /// }
        /// ```
        Red100 => "divide-red-100",
        /// ```css
        /// {
        ///     border-color: rgb(254 202 202);
        /// }
        /// ```
        Red200 => "divide-red-200",
        /// ```css
        /// {
        ///     border-color: rgb(252 165 165);
        /// }
        /// ```
        Red300 => "divide-red-300",
        /// ```css
        /// {
        ///     border-color: rgb(248 113 113);
        /// }
        /// ```
        Red400 => "divide-red-400",
        /// ```css
        /// {
        ///     border-color: rgb(239 68 68);
        /// }
        /// ```
        Red500 => "divide-red-500",
        /// ```css
        /// {
        ///     border-color: rgb(220 38 38);
        /// }
        /// ```
        Red600 => "divide-red-600",
        /// ```css
        /// {
        ///     border-color: rgb(185 28 28);
        /// }
        /// ```
        Red700 => "divide-red-700",
        /// ```css
        /// {
        ///     border-color: rgb(153 27 27);
        /// }
        /// ```
        Red800 => "divide-red-800",
        /// ```css
        /// {
        ///     border-color: rgb(127 29 29);
        /// }
        /// ```
        Red900 => "divide-red-900",
        /// ```css
        /// {
        ///     border-color: rgb(69 10 10);
        /// }
        /// ```
        Red950 => "divide-red-950",
        /// ```css
        /// {
        ///     border-color: rgb(255 247 237);
        /// }
        /// ```
        Orange50 => "divide-orange-50",
        /// ```css
        /// {
        ///     border-color: rgb(255 237 213);
        /// }
        /// ```
        Orange100 => "divide-orange-100",
        /// ```css
        /// {
        ///     border-color: rgb(254 215 170);
        /// }
        /// ```
        Orange200 => "divide-orange-200",
        /// ```css
        /// {
        ///     border-color: rgb(253 186 116);
        /// }
        /// ```
        Orange300 => "divide-orange-300",
        /// ```css
        /// {
        ///     border-color: rgb(251 146 60);
        /// }
        /// ```
        Orange400 => "divide-orange-400",
        /// ```css
        /// {
        ///     border-color: rgb(249 115 22);
        /// }
        /// ```
        Orange500 => "divide-orange-500",
        /// ```css
        /// {
        ///     border-color: rgb(234 88 12);
        /// }
        /// ```
        Orange600 => "divide-orange-600",
        /// ```css
        /// {
        ///     border-color: rgb(194 65 12);
        /// }
        /// ```
        Orange700 => "divide-orange-700",
        /// ```css
        /// {
        ///     border-color: rgb(154 52 18);
        /// }
        /// ```
        Orange800 => "divide-orange-800",
        /// ```css
        /// {
        ///     border-color: rgb(124 45 18);
        /// }
        /// ```
        Orange900 => "divide-orange-900",
        /// ```css
        /// {
        ///     border-color: rgb(67 20 7);
        /// }
        /// ```
        Orange950 => "divide-orange-950",
        /// ```css
        /// {
        ///     border-color: rgb(255 251 235);
        /// }
        /// ```
        Amber50 => "divide-amber-50",
        /// ```css
        /// {
        ///     border-color: rgb(254 243 199);
        /// }
        /// ```
        Amber100 => "divide-amber-100",
        /// ```css
        /// {
        ///     border-color: rgb(253 230 138);
        /// }
        /// ```
        Amber200 => "divide-amber-200",
        /// ```css
        /// {
        ///     border-color: rgb(252 211 77);
        /// }
        /// ```
        Amber300 => "divide-amber-300",
        /// ```css
        /// {
        ///     border-color: rgb(251 191 36);
        /// }
        /// ```
        Amber400 => "divide-amber-400",
        /// ```css
        /// {
        ///     border-color: rgb(245 158 11);
        /// }
        /// ```
        Amber500 => "divide-amber-500",
        /// ```css
        /// {
        ///     border-color: rgb(217 119 6);
        /// }
        /// ```
        Amber600 => "divide-amber-600",
        /// ```css
        /// {
        ///     border-color: rgb(180 83 9);
        /// }
        /// ```
        Amber700 => "divide-amber-700",
        /// ```css
        /// {
        ///     border-color: rgb(146 64 14);
        /// }
        /// ```
        Amber800 => "divide-amber-800",
        /// ```css
        /// {
        ///     border-color: rgb(120 53 15);
        /// }
        /// ```
        Amber900 => "divide-amber-900",
        /// ```css
        /// {
        ///     border-color: rgb(69 26 3);
        /// }
        /// ```
        Amber950 => "divide-amber-950",
        /// ```css
        /// {
        ///     border-color: rgb(254 252 232);
        /// }
        /// ```
        Yellow50 => "divide-yellow-50",
        /// ```css
        /// {
        ///     border-color: rgb(254 249 195);
        /// }
        /// ```
        Yellow100 => "divide-yellow-100",
        /// ```css
        /// {
        ///     border-color: rgb(254 240 138);
        /// }
        /// ```
        Yellow200 => "divide-yellow-200",
        /// ```css
        /// {
        ///     border-color: rgb(253 224 71);
        /// }
        /// ```
        Yellow300 => "divide-yellow-300",
        /// ```css
        /// {
        ///     border-color: rgb(250 204 21);
        /// }
        /// ```
        Yellow400 => "divide-yellow-400",
        /// ```css
        /// {
        ///     border-color: rgb(234 179 8);
        /// }
        /// ```
        Yellow500 => "divide-yellow-500",
        /// ```css
        /// {
        ///     border-color: rgb(202 138 4);
        /// }
        /// ```
        Yellow600 => "divide-yellow-600",
        /// ```css
        /// {
        ///     border-color: rgb(161 98 7);
        /// }
        /// ```
        Yellow700 => "divide-yellow-700",
        /// ```css
        /// {
        ///     border-color: rgb(133 77 14);
        /// }
        /// ```
        Yellow800 => "divide-yellow-800",
        /// ```css
        /// {
        ///     border-color: rgb(113 63 18);
        /// }
        /// ```
        Yellow900 => "divide-yellow-900",
        /// ```css
        /// {
        ///     border-color: rgb(66 32 6);
        /// }
        /// ```
        Yellow950 => "divide-yellow-950",
        /// ```css
        /// {
        ///     border-color: rgb(247 254 231);
        /// }
        /// ```
        Lime50 => "divide-lime-50",
        /// ```css
        /// {
        ///     border-color: rgb(236 252 203);
        /// }
        /// ```
        Lime100 => "divide-lime-100",
        /// ```css
        /// {
        ///     border-color: rgb(217 249 157);
        /// }
        /// ```
        Lime200 => "divide-lime-200",
        /// ```css
        /// {
        ///     border-color: rgb(190 242 100);
        /// }
        /// ```
        Lime300 => "divide-lime-300",
        /// ```css
        /// {
        ///     border-color: rgb(163 230 53);
        /// }
        /// ```
        Lime400 => "divide-lime-400",
        /// ```css
        /// {
        ///     border-color: rgb(132 204 22);
        /// }
        /// ```
        Lime500 => "divide-lime-500",
        /// ```css
        /// {
        ///     border-color: rgb(101 163 13);
        /// }
        /// ```
        Lime600 => "divide-lime-600",
        /// ```css
        /// {
        ///     border-color: rgb(77 124 15);
        /// }
        /// ```
        Lime700 => "divide-lime-700",
        /// ```css
        /// {
        ///     border-color: rgb(63 98 18);
        /// }
        /// ```
        Lime800 => "divide-lime-800",
        /// ```css
        /// {
        ///     border-color: rgb(54 83 20);
        /// }
        /// ```
        Lime900 => "divide-lime-900",
        /// ```css
        /// {
        ///     border-color: rgb(26 46 5);
        /// }
        /// ```
        Lime950 => "divide-lime-950",
        /// ```css
        /// {
        ///     border-color: rgb(240 253 244);
        /// }
        /// ```
        Green50 => "divide-green-50",
        /// ```css
        /// {
        ///     border-color: rgb(220 252 231);
        /// }
        /// ```
        Green100 => "divide-green-100",
        /// ```css
        /// {
        ///     border-color: rgb(187 247 208);
        /// }
        /// ```
        Green200 => "divide-green-200",
        /// ```css
        /// {
        ///     border-color: rgb(134 239 172);
        /// }
        /// ```
        Green300 => "divide-green-300",
        /// ```css
        /// {
        ///     border-color: rgb(74 222 128);
        /// }
        /// ```
        Green400 => "divide-green-400",
        /// ```css
        /// {
        ///     border-color: rgb(34 197 94);
        /// }
        /// ```
        Green500 => "divide-green-500",
        /// ```css
        /// {
        ///     border-color: rgb(22 163 74);
        /// }
        /// ```
        Green600 => "divide-green-600",
        /// ```css
        /// {
        ///     border-color: rgb(21 128 61);
        /// }
        /// ```
        Green700 => "divide-green-700",
        /// ```css
        /// {
        ///     border-color: rgb(22 101 52);
        /// }
        /// ```
        Green800 => "divide-green-800",
        /// ```css
        /// {
        ///     border-color: rgb(20 83 45);
        /// }
        /// ```
        Green900 => "divide-green-900",
        /// ```css
        /// {
        ///     border-color: rgb(5 46 22);
        /// }
        /// ```
        Green950 => "divide-green-950",
        /// ```css
        /// {
        ///     border-color: rgb(236 253 245);
        /// }
        /// ```
        Emerald50 => "divide-emerald-50",
        /// ```css
        /// {
        ///     border-color: rgb(209 250 229);
        /// }
        /// ```
        Emerald100 => "divide-emerald-100",
        /// ```css
        /// {
        ///     border-color: rgb(167 243 208);
        /// }
        /// ```
        Emerald200 => "divide-emerald-200",
        /// ```css
        /// {
        ///     border-color: rgb(110 231 183);
        /// }
        /// ```
        Emerald300 => "divide-emerald-300",
        /// ```css
        /// {
        ///     border-color: rgb(52 211 153);
        /// }
        /// ```
        Emerald400 => "divide-emerald-400",
        /// ```css
        /// {
        ///     border-color: rgb(16 185 129);
        /// }
        /// ```
        Emerald500 => "divide-emerald-500",
        /// ```css
        /// {
        ///     border-color: rgb(5 150 105);
        /// }
        /// ```
        Emerald600 => "divide-emerald-600",
        /// ```css
        /// {
        ///     border-color: rgb(4 120 87);
        /// }
        /// ```
        Emerald700 => "divide-emerald-700",
        /// ```css
        /// {
        ///     border-color: rgb(6 95 70);
        /// }
        /// ```
        Emerald800 => "divide-emerald-800",
        /// ```css
        /// {
        ///     border-color: rgb(6 78 59);
        /// }
        /// ```
        Emerald900 => "divide-emerald-900",
        /// ```css
        /// {
        ///     border-color: rgb(2 44 34);
        /// }
        /// ```
        Emerald950 => "divide-emerald-950",
        /// ```css
        /// {
        ///     border-color: rgb(240 253 250);
        /// }
        /// ```
        Teal50 => "divide-teal-50",
        /// ```css
        /// {
        ///     border-color: rgb(204 251 241);
        /// }
        /// ```
        Teal100 => "divide-teal-100",
        /// ```css
        /// {
        ///     border-color: rgb(153 246 228);
        /// }
        /// ```
        Teal200 => "divide-teal-200",
        /// ```css
        /// {
        ///     border-color: rgb(94 234 212);
        /// }
        /// ```
        Teal300 => "divide-teal-300",
        /// ```css
        /// {
        ///     border-color: rgb(45 212 191);
        /// }
        /// ```
        Teal400 => "divide-teal-400",
        /// ```css
        /// {
        ///     border-color: rgb(20 184 166);
        /// }
        /// ```
        Teal500 => "divide-teal-500",
        /// ```css
        /// {
        ///     border-color: rgb(13 148 136);
        /// }
        /// ```
        Teal600 => "divide-teal-600",
        /// ```css
        /// {
        ///     border-color: rgb(15 118 110);
        /// }
        /// ```
        Teal700 => "divide-teal-700",
        /// ```css
        /// {
        ///     border-color: rgb(17 94 89);
        /// }
        /// ```
        Teal800 => "divide-teal-800",
        /// ```css
        /// {
        ///     border-color: rgb(19 78 74);
        /// }
        /// ```
        Teal900 => "divide-teal-900",
        /// ```css
        /// {
        ///     border-color: rgb(4 47 46);
        /// }
        /// ```
        Teal950 => "divide-teal-950",
        /// ```css
        /// {
        ///     border-color: rgb(236 254 255);
        /// }
        /// ```
        Cyan50 => "divide-cyan-50",
        /// ```css
        /// {
        ///     border-color: rgb(207 250 254);
        /// }
        /// ```
        Cyan100 => "divide-cyan-100",
        /// ```css
        /// {
        ///     border-color: rgb(165 243 252);
        /// }
        /// ```
        Cyan200 => "divide-cyan-200",
        /// ```css
        /// {
        ///     border-color: rgb(103 232 249);
        /// }
        /// ```
        Cyan300 => "divide-cyan-300",
        /// ```css
        /// {
        ///     border-color: rgb(34 211 238);
        /// }
        /// ```
        Cyan400 => "divide-cyan-400",
        /// ```css
        /// {
        ///     border-color: rgb(6 182 212);
        /// }
        /// ```
        Cyan500 => "divide-cyan-500",
        /// ```css
        /// {
        ///     border-color: rgb(8 145 178);
        /// }
        /// ```
        Cyan600 => "divide-cyan-600",
        /// ```css
        /// {
        ///     border-color: rgb(14 116 144);
        /// }
        /// ```
        Cyan700 => "divide-cyan-700",
        /// ```css
        /// {
        ///     border-color: rgb(21 94 117);
        /// }
        /// ```
        Cyan800 => "divide-cyan-800",
        /// ```css
        /// {
        ///     border-color: rgb(22 78 99);
        /// }
        /// ```
        Cyan900 => "divide-cyan-900",
        /// ```css
        /// {
        ///     border-color: rgb(8 51 68);
        /// }
        /// ```
        Cyan950 => "divide-cyan-950",
        /// ```css
        /// {
        ///     border-color: rgb(240 249 255);
        /// }
        /// ```
        Sky50 => "divide-sky-50",
        /// ```css
        /// {
        ///     border-color: rgb(224 242 254);
        /// }
        /// ```
        Sky100 => "divide-sky-100",
        /// ```css
        /// {
        ///     border-color: rgb(186 230 253);
        /// }
        /// ```
        Sky200 => "divide-sky-200",
        /// ```css
        /// {
        ///     border-color: rgb(125 211 252);
        /// }
        /// ```
        Sky300 => "divide-sky-300",
        /// ```css
        /// {
        ///     border-color: rgb(56 189 248);
        /// }
        /// ```
        Sky400 => "divide-sky-400",
        /// ```css
        /// {
        ///     border-color: rgb(14 165 233);
        /// }
        /// ```
        Sky500 => "divide-sky-500",
        /// ```css
        /// {
        ///     border-color: rgb(2 132 199);
        /// }
        /// ```
        Sky600 => "divide-sky-600",
        /// ```css
        /// {
        ///     border-color: rgb(3 105 161);
        /// }
        /// ```
        Sky700 => "divide-sky-700",
        /// ```css
        /// {
        ///     border-color: rgb(7 89 133);
        /// }
        /// ```
        Sky800 => "divide-sky-800",
        /// ```css
        /// {
        ///     border-color: rgb(12 74 110);
        /// }
        /// ```
        Sky900 => "divide-sky-900",
        /// ```css
        /// {
        ///     border-color: rgb(8 47 73);
        /// }
        /// ```
        Sky950 => "divide-sky-950",
        /// ```css
        /// {
        ///     border-color: rgb(239 246 255);
        /// }
        /// ```
        Blue50 => "divide-blue-50",
        /// ```css
        /// {
        ///     border-color: rgb(219 234 254);
        /// }
        /// ```
        Blue100 => "divide-blue-100",
        /// ```css
        /// {
        ///     border-color: rgb(191 219 254);
        /// }
        /// ```
        Blue200 => "divide-blue-200",
        /// ```css
        /// {
        ///     border-color: rgb(147 197 253);
        /// }
        /// ```
        Blue300 => "divide-blue-300",
        /// ```css
        /// {
        ///     border-color: rgb(96 165 250);
        /// }
        /// ```
        Blue400 => "divide-blue-400",
        /// ```css
        /// {
        ///     border-color: rgb(59 130 246);
        /// }
        /// ```
        Blue500 => "divide-blue-500",
        /// ```css
        /// {
        ///     border-color: rgb(37 99 235);
        /// }
        /// ```
        Blue600 => "divide-blue-600",
        /// ```css
        /// {
        ///     border-color: rgb(29 78 216);
        /// }
        /// ```
        Blue700 => "divide-blue-700",
        /// ```css
        /// {
        ///     border-color: rgb(30 64 175);
        /// }
        /// ```
        Blue800 => "divide-blue-800",
        /// ```css
        /// {
        ///     border-color: rgb(30 58 138);
        /// }
        /// ```
        Blue900 => "divide-blue-900",
        /// ```css
        /// {
        ///     border-color: rgb(23 37 84);
        /// }
        /// ```
        Blue950 => "divide-blue-950",
        /// ```css
        /// {
        ///     border-color: rgb(238 242 255);
        /// }
        /// ```
        Indigo50 => "divide-indigo-50",
        /// ```css
        /// {
        ///     border-color: rgb(224 231 255);
        /// }
        /// ```
        Indigo100 => "divide-indigo-100",
        /// ```css
        /// {
        ///     border-color: rgb(199 210 254);
        /// }
        /// ```
        Indigo200 => "divide-indigo-200",
        /// ```css
        /// {
        ///     border-color: rgb(165 180 252);
        /// }
        /// ```
        Indigo300 => "divide-indigo-300",
        /// ```css
        /// {
        ///     border-color: rgb(129 140 248);
        /// }
        /// ```
        Indigo400 => "divide-indigo-400",
        /// ```css
        /// {
        ///     border-color: rgb(99 102 241);
        /// }
        /// ```
        Indigo500 => "divide-indigo-500",
        /// ```css
        /// {
        ///     border-color: rgb(79 70 229);
        /// }
        /// ```
        Indigo600 => "divide-indigo-600",
        /// ```css
        /// {
        ///     border-color: rgb(67 56 202);
        /// }
        /// ```
        Indigo700 => "divide-indigo-700",
        /// ```css
        /// {
        ///     border-color: rgb(55 48 163);
        /// }
        /// ```
        Indigo800 => "divide-indigo-800",
        /// ```css
        /// {
        ///     border-color: rgb(49 46 129);
        /// }
        /// ```
        Indigo900 => "divide-indigo-900",
        /// ```css
        /// {
        ///     border-color: rgb(30 27 75);
        /// }
        /// ```
        Indigo950 => "divide-indigo-950",
        /// ```css
        /// {
        ///     border-color: rgb(245 243 255);
        /// }
        /// ```
        Violet50 => "divide-violet-50",
        /// ```css
        /// {
        ///     border-color: rgb(237 233 254);
        /// }
        /// ```
        Violet100 => "divide-violet-100",
        /// ```css
        /// {
        ///     border-color: rgb(221 214 254);
        /// }
        /// ```
        Violet200 => "divide-violet-200",
        /// ```css
        /// {
        ///     border-color: rgb(196 181 253);
        /// }
        /// ```
        Violet300 => "divide-violet-300",
        /// ```css
        /// {
        ///     border-color: rgb(167 139 250);
        /// }
        /// ```
        Violet400 => "divide-violet-400",
        /// ```css
        /// {
        ///     border-color: rgb(139 92 246);
        /// }
        /// ```
        Violet500 => "divide-violet-500",
        /// ```css
        /// {
        ///     border-color: rgb(124 58 237);
        /// }
        /// ```
        Violet600 => "divide-violet-600",
        /// ```css
        /// {
        ///     border-color: rgb(109 40 217);
        /// }
        /// ```
        Violet700 => "divide-violet-700",
        /// ```css
        /// {
        ///     border-color: rgb(91 33 182);
        /// }
        /// ```
        Violet800 => "divide-violet-800",
        /// ```css
        /// {
        ///     border-color: rgb(76 29 149);
        /// }
        /// ```
        Violet900 => "divide-violet-900",
        /// ```css
        /// {
        ///     border-color: rgb(46 16 101);
        /// }
        /// ```
        Violet950 => "divide-violet-950",
        /// ```css
        /// {
        ///     border-color: rgb(250 245 255);
        /// }
        /// ```
        Purple50 => "divide-purple-50",
        /// ```css
        /// {
        ///     border-color: rgb(243 232 255);
        /// }
        /// ```
        Purple100 => "divide-purple-100",
        /// ```css
        /// {
        ///     border-color: rgb(233 213 255);
        /// }
        /// ```
        Purple200 => "divide-purple-200",
        /// ```css
        /// {
        ///     border-color: rgb(216 180 254);
        /// }
        /// ```
        Purple300 => "divide-purple-300",
        /// ```css
        /// {
        ///     border-color: rgb(192 132 252);
        /// }
        /// ```
        Purple400 => "divide-purple-400",
        /// ```css
        /// {
        ///     border-color: rgb(168 85 247);
        /// }
        /// ```
        Purple500 => "divide-purple-500",
        /// ```css
        /// {
        ///     border-color: rgb(147 51 234);
        /// }
        /// ```
        Purple600 => "divide-purple-600",
        /// ```css
        /// {
        ///     border-color: rgb(126 34 206);
        /// }
        /// ```
        Purple700 => "divide-purple-700",
        /// ```css
        /// {
        ///     border-color: rgb(107 33 168);
        /// }
        /// ```
        Purple800 => "divide-purple-800",
        /// ```css
        /// {
        ///     border-color: rgb(88 28 135);
        /// }
        /// ```
        Purple900 => "divide-purple-900",
        /// ```css
        /// {
        ///     border-color: rgb(59 7 100);
        /// }
        /// ```
        Purple950 => "divide-purple-950",
        /// ```css
        /// {
        ///     border-color: rgb(253 244 255);
        /// }
        /// ```
        Fuchsia50 => "divide-fuchsia-50",
        /// ```css
        /// {
        ///     border-color: rgb(250 232 255);
        /// }
        /// ```
        Fuchsia100 => "divide-fuchsia-100",
        /// ```css
        /// {
        ///     border-color: rgb(245 208 254);
        /// }
        /// ```
        Fuchsia200 => "divide-fuchsia-200",
        /// ```css
        /// {
        ///     border-color: rgb(240 171 252);
        /// }
        /// ```
        Fuchsia300 => "divide-fuchsia-300",
        /// ```css
        /// {
        ///     border-color: rgb(232 121 249);
        /// }
        /// ```
        Fuchsia400 => "divide-fuchsia-400",
        /// ```css
        /// {
        ///     border-color: rgb(217 70 239);
        /// }
        /// ```
        Fuchsia500 => "divide-fuchsia-500",
        /// ```css
        /// {
        ///     border-color: rgb(192 38 211);
        /// }
        /// ```
        Fuchsia600 => "divide-fuchsia-600",
        /// ```css
        /// {
        ///     border-color: rgb(162 28 175);
        /// }
        /// ```
        Fuchsia700 => "divide-fuchsia-700",
        /// ```css
        /// {
        ///     border-color: rgb(134 25 143);
        /// }
        /// ```
        Fuchsia800 => "divide-fuchsia-800",
        /// ```css
        /// {
        ///     border-color: rgb(112 26 117);
        /// }
        /// ```
        Fuchsia900 => "divide-fuchsia-900",
        /// ```css
        /// {
        ///     border-color: rgb(74 4 78);
        /// }
        /// ```
        Fuchsia950 => "divide-fuchsia-950",
        /// ```css
        /// {
        ///     border-color: rgb(253 242 248);
        /// }
        /// ```
        Pink50 => "divide-pink-50",
        /// ```css
        /// {
        ///     border-color: rgb(252 231 243);
        /// }
        /// ```
        Pink100 => "divide-pink-100",
        /// ```css
        /// {
        ///     border-color: rgb(251 207 232);
        /// }
        /// ```
        Pink200 => "divide-pink-200",
        /// ```css
        /// {
        ///     border-color: rgb(249 168 212);
        /// }
        /// ```
        Pink300 => "divide-pink-300",
        /// ```css
        /// {
        ///     border-color: rgb(244 114 182);
        /// }
        /// ```
        Pink400 => "divide-pink-400",
        /// ```css
        /// {
        ///     border-color: rgb(236 72 153);
        /// }
        /// ```
        Pink500 => "divide-pink-500",
        /// ```css
        /// {
        ///     border-color: rgb(219 39 119);
        /// }
        /// ```
        Pink600 => "divide-pink-600",
        /// ```css
        /// {
        ///     border-color: rgb(190 24 93);
        /// }
        /// ```
        Pink700 => "divide-pink-700",
        /// ```css
        /// {
        ///     border-color: rgb(157 23 77);
        /// }
        /// ```
        Pink800 => "divide-pink-800",
        /// ```css
        /// {
        ///     border-color: rgb(131 24 67);
        /// }
        /// ```
        Pink900 => "divide-pink-900",
        /// ```css
        /// {
        ///     border-color: rgb(80 7 36);
        /// }
        /// ```
        Pink950 => "divide-pink-950",
        /// ```css
        /// {
        ///     border-color: rgb(255 241 242);
        /// }
        /// ```
        Rose50 => "divide-rose-50",
        /// ```css
        /// {
        ///     border-color: rgb(255 228 230);
        /// }
        /// ```
        Rose100 => "divide-rose-100",
        /// ```css
        /// {
        ///     border-color: rgb(254 205 211);
        /// }
        /// ```
        Rose200 => "divide-rose-200",
        /// ```css
        /// {
        ///     border-color: rgb(253 164 175);
        /// }
        /// ```
        Rose300 => "divide-rose-300",
        /// ```css
        /// {
        ///     border-color: rgb(251 113 133);
        /// }
        /// ```
        Rose400 => "divide-rose-400",
        /// ```css
        /// {
        ///     border-color: rgb(244 63 94);
        /// }
        /// ```
        Rose500 => "divide-rose-500",
        /// ```css
        /// {
        ///     border-color: rgb(225 29 72);
        /// }
        /// ```
        Rose600 => "divide-rose-600",
        /// ```css
        /// {
        ///     border-color: rgb(190 18 60);
        /// }
        /// ```
        Rose700 => "divide-rose-700",
        /// ```css
        /// {
        ///     border-color: rgb(159 18 57);
        /// }
        /// ```
        Rose800 => "divide-rose-800",
        /// ```css
        /// {
        ///     border-color: rgb(136 19 55);
        /// }
        /// ```
        Rose900 => "divide-rose-900",
        /// ```css
        /// {
        ///     border-color: rgb(76 5 25);
        /// }
        /// ```
        Rose950 => "divide-rose-950",
    }
    /// Utilities for controlling the border style between elements.
    ///
    /// <https://tailwindcss.com/docs/divide-style>
    DivideStyle {
        /// ```css
        /// {
        ///     border-style: solid;
        /// }
        /// ```
        Solid => "divide-solid",
        /// ```css
        /// {
        ///     border-style: dashed;
        /// }
        /// ```
        Dashed => "divide-dashed",
        /// ```css
        /// {
        ///     border-style: dotted;
        /// }
        /// ```
        Dotted => "divide-dotted",
        /// ```css
        /// {
        ///     border-style: double;
        /// }
        /// ```
        Double => "divide-double",
        /// ```css
        /// {
        ///     border-style: none;
        /// }
        /// ```
        None => "divide-none",
    }
    /// Utilities for controlling the width of an element's outline.
    ///
    /// <https://tailwindcss.com/docs/outline-width>
    OutlineWidth {
        /// ```css
        /// {
        ///     outline-width: 0px;
        /// }
        /// ```
        _0 => "outline-0",
        /// ```css
        /// {
        ///     outline-width: 1px;
        /// }
        /// ```
        _1 => "outline-1",
        /// ```css
        /// {
        ///     outline-width: 2px;
        /// }
        /// ```
        _2 => "outline-2",
        /// ```css
        /// {
        ///     outline-width: 4px;
        /// }
        /// ```
        _4 => "outline-4",
        /// ```css
        /// {
        ///     outline-width: 8px;
        /// }
        /// ```
        _8 => "outline-8",
    }
    /// Utilities for controlling the color of an element's outline.
    ///
    /// <https://tailwindcss.com/docs/outline-color>
    OutlineColor {
        /// ```css
        /// {
        ///     outline-color: inherit;
        /// }
        /// ```
        Inherit => "outline-inherit",
        /// ```css
        /// {
        ///     outline-color: currentColor;
        /// }
        /// ```
        Current => "outline-current",
        /// ```css
        /// {
        ///     outline-color: transparent;
        /// }
        /// ```
        Transparent => "outline-transparent",
        /// ```css
        /// {
        ///     outline-color: #000;
        /// }
        /// ```
        Black => "outline-black",
        /// ```css
        /// {
        ///     outline-color: #fff;
        /// }
        /// ```
        White => "outline-white",
        /// ```css
        /// {
        ///     outline-color: #f8fafc;
        /// }
        /// ```
        Slate50 => "outline-slate-50",
        /// ```css
        /// {
        ///     outline-color: #f1f5f9;
        /// }
        /// ```
        Slate100 => "outline-slate-100",
        /// ```css
        /// {
        ///     outline-color: #e2e8f0;
        /// }
        /// ```
        Slate200 => "outline-slate-200",
        /// ```css
        /// {
        ///     outline-color: #cbd5e1;
        /// }
        /// ```
        Slate300 => "outline-slate-300",
        /// ```css
        /// {
        ///     outline-color: #94a3b8;
        /// }
        /// ```
        Slate400 => "outline-slate-400",
        /// ```css
        /// {
        ///     outline-color: #64748b;
        /// }
        /// ```
        Slate500 => "outline-slate-500",
        /// ```css
        /// {
        ///     outline-color: #475569;
        /// }
        /// ```
        Slate600 => "outline-slate-600",
        /// ```css
        /// {
        ///     outline-color: #334155;
        /// }
        /// ```
        Slate700 => "outline-slate-700",
        /// ```css
        /// {
        ///     outline-color: #1e293b;
        /// }
        /// ```
        Slate800 => "outline-slate-800",
        /// ```css
        /// {
        ///     outline-color: #0f172a;
        /// }
        /// ```
        Slate900 => "outline-slate-900",
        /// ```css
        /// {
        ///     outline-color: #020617;
        /// }
        /// ```
        Slate950 => "outline-slate-950",
        /// ```css
        /// {
        ///     outline-color: #f9fafb;
        /// }
        /// ```
        Gray50 => "outline-gray-50",
        /// ```css
        /// {
        ///     outline-color: #f3f4f6;
        /// }
        /// ```
        Gray100 => "outline-gray-100",
        /// ```css
        /// {
        ///     outline-color: #e5e7eb;
        /// }
        /// ```
        Gray200 => "outline-gray-200",
        /// ```css
        /// {
        ///     outline-color: #d1d5db;
        /// }
        /// ```
        Gray300 => "outline-gray-300",
        /// ```css
        /// {
        ///     outline-color: #9ca3af;
        /// }
        /// ```
        Gray400 => "outline-gray-400",
        /// ```css
        /// {
        ///     outline-color: #6b7280;
        /// }
        /// ```
        Gray500 => "outline-gray-500",
        /// ```css
        /// {
        ///     outline-color: #4b5563;
        /// }
        /// ```
        Gray600 => "outline-gray-600",
        /// ```css
        /// {
        ///     outline-color: #374151;
        /// }
        /// ```
        Gray700 => "outline-gray-700",
        /// ```css
        /// {
        ///     outline-color: #1f2937;
        /// }
        /// ```
        Gray800 => "outline-gray-800",
        /// ```css
        /// {
        ///     outline-color: #111827;
        /// }
        /// ```
        Gray900 => "outline-gray-900",
        /// ```css
        /// {
        ///     outline-color: #030712;
        /// }
        /// ```
        Gray950 => "outline-gray-950",
        /// ```css
        /// {
        ///     outline-color: #fafafa;
        /// }
        /// ```
        Zinc50 => "outline-zinc-50",
        /// ```css
        /// {
        ///     outline-color: #f4f4f5;
        /// }
        /// ```
        Zinc100 => "outline-zinc-100",
        /// ```css
        /// {
        ///     outline-color: #e4e4e7;
        /// }
        /// ```
        Zinc200 => "outline-zinc-200",
        /// ```css
        /// {
        ///     outline-color: #d4d4d8;
        /// }
        /// ```
        Zinc300 => "outline-zinc-300",
        /// ```css
        /// {
        ///     outline-color: #a1a1aa;
        /// }
        /// ```
        Zinc400 => "outline-zinc-400",
        /// ```css
        /// {
        ///     outline-color: #71717a;
        /// }
        /// ```
        Zinc500 => "outline-zinc-500",
        /// ```css
        /// {
        ///     outline-color: #52525b;
        /// }
        /// ```
        Zinc600 => "outline-zinc-600",
        /// ```css
        /// {
        ///     outline-color: #3f3f46;
        /// }
        /// ```
        Zinc700 => "outline-zinc-700",
        /// ```css
        /// {
        ///     outline-color: #27272a;
        /// }
        /// ```
        Zinc800 => "outline-zinc-800",
        /// ```css
        /// {
        ///     outline-color: #18181b;
        /// }
        /// ```
        Zinc900 => "outline-zinc-900",
        /// ```css
        /// {
        ///     outline-color: #09090b;
        /// }
        /// ```
        Zinc950 => "outline-zinc-950",
        /// ```css
        /// {
        ///     outline-color: #fafafa;
        /// }
        /// ```
        Neutral50 => "outline-neutral-50",
        /// ```css
        /// {
        ///     outline-color: #f5f5f5;
        /// }
        /// ```
        Neutral100 => "outline-neutral-100",
        /// ```css
        /// {
        ///     outline-color: #e5e5e5;
        /// }
        /// ```
        Neutral200 => "outline-neutral-200",
        /// ```css
        /// {
        ///     outline-color: #d4d4d4;
        /// }
        /// ```
        Neutral300 => "outline-neutral-300",
        /// ```css
        /// {
        ///     outline-color: #a3a3a3;
        /// }
        /// ```
        Neutral400 => "outline-neutral-400",
        /// ```css
        /// {
        ///     outline-color: #737373;
        /// }
        /// ```
        Neutral500 => "outline-neutral-500",
        /// ```css
        /// {
        ///     outline-color: #525252;
        /// }
        /// ```
        Neutral600 => "outline-neutral-600",
        /// ```css
        /// {
        ///     outline-color: #404040;
        /// }
        /// ```
        Neutral700 => "outline-neutral-700",
        /// ```css
        /// {
        ///     outline-color: #262626;
        /// }
        /// ```
        Neutral800 => "outline-neutral-800",
        /// ```css
        /// {
        ///     outline-color: #171717;
        /// }
        /// ```
        Neutral900 => "outline-neutral-900",
        /// ```css
        /// {
        ///     outline-color: #0a0a0a;
        /// }
        /// ```
        Neutral950 => "outline-neutral-950",
        /// ```css
        /// {
        ///     outline-color: #fafaf9;
        /// }
        /// ```
        Stone50 => "outline-stone-50",
        /// ```css
        /// {
        ///     outline-color: #f5f5f4;
        /// }
        /// ```
        Stone100 => "outline-stone-100",
        /// ```css
        /// {
        ///     outline-color: #e7e5e4;
        /// }
        /// ```
        Stone200 => "outline-stone-200",
        /// ```css
        /// {
        ///     outline-color: #d6d3d1;
        /// }
        /// ```
        Stone300 => "outline-stone-300",
        /// ```css
        /// {
        ///     outline-color: #a8a29e;
        /// }
        /// ```
        Stone400 => "outline-stone-400",
        /// ```css
        /// {
        ///     outline-color: #78716c;
        /// }
        /// ```
        Stone500 => "outline-stone-500",
        /// ```css
        /// {
        ///     outline-color: #57534e;
        /// }
        /// ```
        Stone600 => "outline-stone-600",
        /// ```css
        /// {
        ///     outline-color: #44403c;
        /// }
        /// ```
        Stone700 => "outline-stone-700",
        /// ```css
        /// {
        ///     outline-color: #292524;
        /// }
        /// ```
        Stone800 => "outline-stone-800",
        /// ```css
        /// {
        ///     outline-color: #1c1917;
        /// }
        /// ```
        Stone900 => "outline-stone-900",
        /// ```css
        /// {
        ///     outline-color: #0c0a09;
        /// }
        /// ```
        Stone950 => "outline-stone-950",
        /// ```css
        /// {
        ///     outline-color: #fef2f2;
        /// }
        /// ```
        Red50 => "outline-red-50",
        /// ```css
        /// {
        ///     outline-color: #fee2e2;
        /// }
        /// ```
        Red100 => "outline-red-100",
        /// ```css
        /// {
        ///     outline-color: #fecaca;
        /// }
        /// ```
        Red200 => "outline-red-200",
        /// ```css
        /// {
        ///     outline-color: #fca5a5;
        /// }
        /// ```
        Red300 => "outline-red-300",
        /// ```css
        /// {
        ///     outline-color: #f87171;
        /// }
        /// ```
        Red400 => "outline-red-400",
        /// ```css
        /// {
        ///     outline-color: #ef4444;
        /// }
        /// ```
        Red500 => "outline-red-500",
        /// ```css
        /// {
        ///     outline-color: #dc2626;
        /// }
        /// ```
        Red600 => "outline-red-600",
        /// ```css
        /// {
        ///     outline-color: #b91c1c;
        /// }
        /// ```
        Red700 => "outline-red-700",
        /// ```css
        /// {
        ///     outline-color: #991b1b;
        /// }
        /// ```
        Red800 => "outline-red-800",
        /// ```css
        /// {
        ///     outline-color: #7f1d1d;
        /// }
        /// ```
        Red900 => "outline-red-900",
        /// ```css
        /// {
        ///     outline-color: #450a0a;
        /// }
        /// ```
        Red950 => "outline-red-950",
        /// ```css
        /// {
        ///     outline-color: #fff7ed;
        /// }
        /// ```
        Orange50 => "outline-orange-50",
        /// ```css
        /// {
        ///     outline-color: #ffedd5;
        /// }
        /// ```
        Orange100 => "outline-orange-100",
        /// ```css
        /// {
        ///     outline-color: #fed7aa;
        /// }
        /// ```
        Orange200 => "outline-orange-200",
        /// ```css
        /// {
        ///     outline-color: #fdba74;
        /// }
        /// ```
        Orange300 => "outline-orange-300",
        /// ```css
        /// {
        ///     outline-color: #fb923c;
        /// }
        /// ```
        Orange400 => "outline-orange-400",
        /// ```css
        /// {
        ///     outline-color: #f97316;
        /// }
        /// ```
        Orange500 => "outline-orange-500",
        /// ```css
        /// {
        ///     outline-color: #ea580c;
        /// }
        /// ```
        Orange600 => "outline-orange-600",
        /// ```css
        /// {
        ///     outline-color: #c2410c;
        /// }
        /// ```
        Orange700 => "outline-orange-700",
        /// ```css
        /// {
        ///     outline-color: #9a3412;
        /// }
        /// ```
        Orange800 => "outline-orange-800",
        /// ```css
        /// {
        ///     outline-color: #7c2d12;
        /// }
        /// ```
        Orange900 => "outline-orange-900",
        /// ```css
        /// {
        ///     outline-color: #431407;
        /// }
        /// ```
        Orange950 => "outline-orange-950",
        /// ```css
        /// {
        ///     outline-color: #fffbeb;
        /// }
        /// ```
        Amber50 => "outline-amber-50",
        /// ```css
        /// {
        ///     outline-color: #fef3c7;
        /// }
        /// ```
        Amber100 => "outline-amber-100",
        /// ```css
        /// {
        ///     outline-color: #fde68a;
        /// }
        /// ```
        Amber200 => "outline-amber-200",
        /// ```css
        /// {
        ///     outline-color: #fcd34d;
        /// }
        /// ```
        Amber300 => "outline-amber-300",
        /// ```css
        /// {
        ///     outline-color: #fbbf24;
        /// }
        /// ```
        Amber400 => "outline-amber-400",
        /// ```css
        /// {
        ///     outline-color: #f59e0b;
        /// }
        /// ```
        Amber500 => "outline-amber-500",
        /// ```css
        /// {
        ///     outline-color: #d97706;
        /// }
        /// ```
        Amber600 => "outline-amber-600",
        /// ```css
        /// {
        ///     outline-color: #b45309;
        /// }
        /// ```
        Amber700 => "outline-amber-700",
        /// ```css
        /// {
        ///     outline-color: #92400e;
        /// }
        /// ```
        Amber800 => "outline-amber-800",
        /// ```css
        /// {
        ///     outline-color: #78350f;
        /// }
        /// ```
        Amber900 => "outline-amber-900",
        /// ```css
        /// {
        ///     outline-color: #451a03;
        /// }
        /// ```
        Amber950 => "outline-amber-950",
        /// ```css
        /// {
        ///     outline-color: #fefce8;
        /// }
        /// ```
        Yellow50 => "outline-yellow-50",
        /// ```css
        /// {
        ///     outline-color: #fef9c3;
        /// }
        /// ```
        Yellow100 => "outline-yellow-100",
        /// ```css
        /// {
        ///     outline-color: #fef08a;
        /// }
        /// ```
        Yellow200 => "outline-yellow-200",
        /// ```css
        /// {
        ///     outline-color: #fde047;
        /// }
        /// ```
        Yellow300 => "outline-yellow-300",
        /// ```css
        /// {
        ///     outline-color: #facc15;
        /// }
        /// ```
        Yellow400 => "outline-yellow-400",
        /// ```css
        /// {
        ///     outline-color: #eab308;
        /// }
        /// ```
        Yellow500 => "outline-yellow-500",
        /// ```css
        /// {
        ///     outline-color: #ca8a04;
        /// }
        /// ```
        Yellow600 => "outline-yellow-600",
        /// ```css
        /// {
        ///     outline-color: #a16207;
        /// }
        /// ```
        Yellow700 => "outline-yellow-700",
        /// ```css
        /// {
        ///     outline-color: #854d0e;
        /// }
        /// ```
        Yellow800 => "outline-yellow-800",
        /// ```css
        /// {
        ///     outline-color: #713f12;
        /// }
        /// ```
        Yellow900 => "outline-yellow-900",
        /// ```css
        /// {
        ///     outline-color: #422006;
        /// }
        /// ```
        Yellow950 => "outline-yellow-950",
        /// ```css
        /// {
        ///     outline-color: #f7fee7;
        /// }
        /// ```
        Lime50 => "outline-lime-50",
        /// ```css
        /// {
        ///     outline-color: #ecfccb;
        /// }
        /// ```
        Lime100 => "outline-lime-100",
        /// ```css
        /// {
        ///     outline-color: #d9f99d;
        /// }
        /// ```
        Lime200 => "outline-lime-200",
        /// ```css
        /// {
        ///     outline-color: #bef264;
        /// }
        /// ```
        Lime300 => "outline-lime-300",
        /// ```css
        /// {
        ///     outline-color: #a3e635;
        /// }
        /// ```
        Lime400 => "outline-lime-400",
        /// ```css
        /// {
        ///     outline-color: #84cc16;
        /// }
        /// ```
        Lime500 => "outline-lime-500",
        /// ```css
        /// {
        ///     outline-color: #65a30d;
        /// }
        /// ```
        Lime600 => "outline-lime-600",
        /// ```css
        /// {
        ///     outline-color: #4d7c0f;
        /// }
        /// ```
        Lime700 => "outline-lime-700",
        /// ```css
        /// {
        ///     outline-color: #3f6212;
        /// }
        /// ```
        Lime800 => "outline-lime-800",
        /// ```css
        /// {
        ///     outline-color: #365314;
        /// }
        /// ```
        Lime900 => "outline-lime-900",
        /// ```css
        /// {
        ///     outline-color: #1a2e05;
        /// }
        /// ```
        Lime950 => "outline-lime-950",
        /// ```css
        /// {
        ///     outline-color: #f0fdf4;
        /// }
        /// ```
        Green50 => "outline-green-50",
        /// ```css
        /// {
        ///     outline-color: #dcfce7;
        /// }
        /// ```
        Green100 => "outline-green-100",
        /// ```css
        /// {
        ///     outline-color: #bbf7d0;
        /// }
        /// ```
        Green200 => "outline-green-200",
        /// ```css
        /// {
        ///     outline-color: #86efac;
        /// }
        /// ```
        Green300 => "outline-green-300",
        /// ```css
        /// {
        ///     outline-color: #4ade80;
        /// }
        /// ```
        Green400 => "outline-green-400",
        /// ```css
        /// {
        ///     outline-color: #22c55e;
        /// }
        /// ```
        Green500 => "outline-green-500",
        /// ```css
        /// {
        ///     outline-color: #16a34a;
        /// }
        /// ```
        Green600 => "outline-green-600",
        /// ```css
        /// {
        ///     outline-color: #15803d;
        /// }
        /// ```
        Green700 => "outline-green-700",
        /// ```css
        /// {
        ///     outline-color: #166534;
        /// }
        /// ```
        Green800 => "outline-green-800",
        /// ```css
        /// {
        ///     outline-color: #14532d;
        /// }
        /// ```
        Green900 => "outline-green-900",
        /// ```css
        /// {
        ///     outline-color: #052e16;
        /// }
        /// ```
        Green950 => "outline-green-950",
        /// ```css
        /// {
        ///     outline-color: #ecfdf5;
        /// }
        /// ```
        Emerald50 => "outline-emerald-50",
        /// ```css
        /// {
        ///     outline-color: #d1fae5;
        /// }
        /// ```
        Emerald100 => "outline-emerald-100",
        /// ```css
        /// {
        ///     outline-color: #a7f3d0;
        /// }
        /// ```
        Emerald200 => "outline-emerald-200",
        /// ```css
        /// {
        ///     outline-color: #6ee7b7;
        /// }
        /// ```
        Emerald300 => "outline-emerald-300",
        /// ```css
        /// {
        ///     outline-color: #34d399;
        /// }
        /// ```
        Emerald400 => "outline-emerald-400",
        /// ```css
        /// {
        ///     outline-color: #10b981;
        /// }
        /// ```
        Emerald500 => "outline-emerald-500",
        /// ```css
        /// {
        ///     outline-color: #059669;
        /// }
        /// ```
        Emerald600 => "outline-emerald-600",
        /// ```css
        /// {
        ///     outline-color: #047857;
        /// }
        /// ```
        Emerald700 => "outline-emerald-700",
        /// ```css
        /// {
        ///     outline-color: #065f46;
        /// }
        /// ```
        Emerald800 => "outline-emerald-800",
        /// ```css
        /// {
        ///     outline-color: #064e3b;
        /// }
        /// ```
        Emerald900 => "outline-emerald-900",
        /// ```css
        /// {
        ///     outline-color: #022c22;
        /// }
        /// ```
        Emerald950 => "outline-emerald-950",
        /// ```css
        /// {
        ///     outline-color: #f0fdfa;
        /// }
        /// ```
        Teal50 => "outline-teal-50",
        /// ```css
        /// {
        ///     outline-color: #ccfbf1;
        /// }
        /// ```
        Teal100 => "outline-teal-100",
        /// ```css
        /// {
        ///     outline-color: #99f6e4;
        /// }
        /// ```
        Teal200 => "outline-teal-200",
        /// ```css
        /// {
        ///     outline-color: #5eead4;
        /// }
        /// ```
        Teal300 => "outline-teal-300",
        /// ```css
        /// {
        ///     outline-color: #2dd4bf;
        /// }
        /// ```
        Teal400 => "outline-teal-400",
        /// ```css
        /// {
        ///     outline-color: #14b8a6;
        /// }
        /// ```
        Teal500 => "outline-teal-500",
        /// ```css
        /// {
        ///     outline-color: #0d9488;
        /// }
        /// ```
        Teal600 => "outline-teal-600",
        /// ```css
        /// {
        ///     outline-color: #0f766e;
        /// }
        /// ```
        Teal700 => "outline-teal-700",
        /// ```css
        /// {
        ///     outline-color: #115e59;
        /// }
        /// ```
        Teal800 => "outline-teal-800",
        /// ```css
        /// {
        ///     outline-color: #134e4a;
        /// }
        /// ```
        Teal900 => "outline-teal-900",
        /// ```css
        /// {
        ///     outline-color: #042f2e;
        /// }
        /// ```
        Teal950 => "outline-teal-950",
        /// ```css
        /// {
        ///     outline-color: #ecfeff;
        /// }
        /// ```
        Cyan50 => "outline-cyan-50",
        /// ```css
        /// {
        ///     outline-color: #cffafe;
        /// }
        /// ```
        Cyan100 => "outline-cyan-100",
        /// ```css
        /// {
        ///     outline-color: #a5f3fc;
        /// }
        /// ```
        Cyan200 => "outline-cyan-200",
        /// ```css
        /// {
        ///     outline-color: #67e8f9;
        /// }
        /// ```
        Cyan300 => "outline-cyan-300",
        /// ```css
        /// {
        ///     outline-color: #22d3ee;
        /// }
        /// ```
        Cyan400 => "outline-cyan-400",
        /// ```css
        /// {
        ///     outline-color: #06b6d4;
        /// }
        /// ```
        Cyan500 => "outline-cyan-500",
        /// ```css
        /// {
        ///     outline-color: #0891b2;
        /// }
        /// ```
        Cyan600 => "outline-cyan-600",
        /// ```css
        /// {
        ///     outline-color: #0e7490;
        /// }
        /// ```
        Cyan700 => "outline-cyan-700",
        /// ```css
        /// {
        ///     outline-color: #155e75;
        /// }
        /// ```
        Cyan800 => "outline-cyan-800",
        /// ```css
        /// {
        ///     outline-color: #164e63;
        /// }
        /// ```
        Cyan900 => "outline-cyan-900",
        /// ```css
        /// {
        ///     outline-color: #083344;
        /// }
        /// ```
        Cyan950 => "outline-cyan-950",
        /// ```css
        /// {
        ///     outline-color: #f0f9ff;
        /// }
        /// ```
        Sky50 => "outline-sky-50",
        /// ```css
        /// {
        ///     outline-color: #e0f2fe;
        /// }
        /// ```
        Sky100 => "outline-sky-100",
        /// ```css
        /// {
        ///     outline-color: #bae6fd;
        /// }
        /// ```
        Sky200 => "outline-sky-200",
        /// ```css
        /// {
        ///     outline-color: #7dd3fc;
        /// }
        /// ```
        Sky300 => "outline-sky-300",
        /// ```css
        /// {
        ///     outline-color: #38bdf8;
        /// }
        /// ```
        Sky400 => "outline-sky-400",
        /// ```css
        /// {
        ///     outline-color: #0ea5e9;
        /// }
        /// ```
        Sky500 => "outline-sky-500",
        /// ```css
        /// {
        ///     outline-color: #0284c7;
        /// }
        /// ```
        Sky600 => "outline-sky-600",
        /// ```css
        /// {
        ///     outline-color: #0369a1;
        /// }
        /// ```
        Sky700 => "outline-sky-700",
        /// ```css
        /// {
        ///     outline-color: #075985;
        /// }
        /// ```
        Sky800 => "outline-sky-800",
        /// ```css
        /// {
        ///     outline-color: #0c4a6e;
        /// }
        /// ```
        Sky900 => "outline-sky-900",
        /// ```css
        /// {
        ///     outline-color: #082f49;
        /// }
        /// ```
        Sky950 => "outline-sky-950",
        /// ```css
        /// {
        ///     outline-color: #eff6ff;
        /// }
        /// ```
        Blue50 => "outline-blue-50",
        /// ```css
        /// {
        ///     outline-color: #dbeafe;
        /// }
        /// ```
        Blue100 => "outline-blue-100",
        /// ```css
        /// {
        ///     outline-color: #bfdbfe;
        /// }
        /// ```
        Blue200 => "outline-blue-200",
        /// ```css
        /// {
        ///     outline-color: #93c5fd;
        /// }
        /// ```
        Blue300 => "outline-blue-300",
        /// ```css
        /// {
        ///     outline-color: #60a5fa;
        /// }
        /// ```
        Blue400 => "outline-blue-400",
        /// ```css
        /// {
        ///     outline-color: #3b82f6;
        /// }
        /// ```
        Blue500 => "outline-blue-500",
        /// ```css
        /// {
        ///     outline-color: #2563eb;
        /// }
        /// ```
        Blue600 => "outline-blue-600",
        /// ```css
        /// {
        ///     outline-color: #1d4ed8;
        /// }
        /// ```
        Blue700 => "outline-blue-700",
        /// ```css
        /// {
        ///     outline-color: #1e40af;
        /// }
        /// ```
        Blue800 => "outline-blue-800",
        /// ```css
        /// {
        ///     outline-color: #1e3a8a;
        /// }
        /// ```
        Blue900 => "outline-blue-900",
        /// ```css
        /// {
        ///     outline-color: #172554;
        /// }
        /// ```
        Blue950 => "outline-blue-950",
        /// ```css
        /// {
        ///     outline-color: #eef2ff;
        /// }
        /// ```
        Indigo50 => "outline-indigo-50",
        /// ```css
        /// {
        ///     outline-color: #e0e7ff;
        /// }
        /// ```
        Indigo100 => "outline-indigo-100",
        /// ```css
        /// {
        ///     outline-color: #c7d2fe;
        /// }
        /// ```
        Indigo200 => "outline-indigo-200",
        /// ```css
        /// {
        ///     outline-color: #a5b4fc;
        /// }
        /// ```
        Indigo300 => "outline-indigo-300",
        /// ```css
        /// {
        ///     outline-color: #818cf8;
        /// }
        /// ```
        Indigo400 => "outline-indigo-400",
        /// ```css
        /// {
        ///     outline-color: #6366f1;
        /// }
        /// ```
        Indigo500 => "outline-indigo-500",
        /// ```css
        /// {
        ///     outline-color: #4f46e5;
        /// }
        /// ```
        Indigo600 => "outline-indigo-600",
        /// ```css
        /// {
        ///     outline-color: #4338ca;
        /// }
        /// ```
        Indigo700 => "outline-indigo-700",
        /// ```css
        /// {
        ///     outline-color: #3730a3;
        /// }
        /// ```
        Indigo800 => "outline-indigo-800",
        /// ```css
        /// {
        ///     outline-color: #312e81;
        /// }
        /// ```
        Indigo900 => "outline-indigo-900",
        /// ```css
        /// {
        ///     outline-color: #1e1b4b;
        /// }
        /// ```
        Indigo950 => "outline-indigo-950",
        /// ```css
        /// {
        ///     outline-color: #f5f3ff;
        /// }
        /// ```
        Violet50 => "outline-violet-50",
        /// ```css
        /// {
        ///     outline-color: #ede9fe;
        /// }
        /// ```
        Violet100 => "outline-violet-100",
        /// ```css
        /// {
        ///     outline-color: #ddd6fe;
        /// }
        /// ```
        Violet200 => "outline-violet-200",
        /// ```css
        /// {
        ///     outline-color: #c4b5fd;
        /// }
        /// ```
        Violet300 => "outline-violet-300",
        /// ```css
        /// {
        ///     outline-color: #a78bfa;
        /// }
        /// ```
        Violet400 => "outline-violet-400",
        /// ```css
        /// {
        ///     outline-color: #8b5cf6;
        /// }
        /// ```
        Violet500 => "outline-violet-500",
        /// ```css
        /// {
        ///     outline-color: #7c3aed;
        /// }
        /// ```
        Violet600 => "outline-violet-600",
        /// ```css
        /// {
        ///     outline-color: #6d28d9;
        /// }
        /// ```
        Violet700 => "outline-violet-700",
        /// ```css
        /// {
        ///     outline-color: #5b21b6;
        /// }
        /// ```
        Violet800 => "outline-violet-800",
        /// ```css
        /// {
        ///     outline-color: #4c1d95;
        /// }
        /// ```
        Violet900 => "outline-violet-900",
        /// ```css
        /// {
        ///     outline-color: #2e1065;
        /// }
        /// ```
        Violet950 => "outline-violet-950",
        /// ```css
        /// {
        ///     outline-color: #faf5ff;
        /// }
        /// ```
        Purple50 => "outline-purple-50",
        /// ```css
        /// {
        ///     outline-color: #f3e8ff;
        /// }
        /// ```
        Purple100 => "outline-purple-100",
        /// ```css
        /// {
        ///     outline-color: #e9d5ff;
        /// }
        /// ```
        Purple200 => "outline-purple-200",
        /// ```css
        /// {
        ///     outline-color: #d8b4fe;
        /// }
        /// ```
        Purple300 => "outline-purple-300",
        /// ```css
        /// {
        ///     outline-color: #c084fc;
        /// }
        /// ```
        Purple400 => "outline-purple-400",
        /// ```css
        /// {
        ///     outline-color: #a855f7;
        /// }
        /// ```
        Purple500 => "outline-purple-500",
        /// ```css
        /// {
        ///     outline-color: #9333ea;
        /// }
        /// ```
        Purple600 => "outline-purple-600",
        /// ```css
        /// {
        ///     outline-color: #7e22ce;
        /// }
        /// ```
        Purple700 => "outline-purple-700",
        /// ```css
        /// {
        ///     outline-color: #6b21a8;
        /// }
        /// ```
        Purple800 => "outline-purple-800",
        /// ```css
        /// {
        ///     outline-color: #581c87;
        /// }
        /// ```
        Purple900 => "outline-purple-900",
        /// ```css
        /// {
        ///     outline-color: #3b0764;
        /// }
        /// ```
        Purple950 => "outline-purple-950",
        /// ```css
        /// {
        ///     outline-color: #fdf4ff;
        /// }
        /// ```
        Fuchsia50 => "outline-fuchsia-50",
        /// ```css
        /// {
        ///     outline-color: #fae8ff;
        /// }
        /// ```
        Fuchsia100 => "outline-fuchsia-100",
        /// ```css
        /// {
        ///     outline-color: #f5d0fe;
        /// }
        /// ```
        Fuchsia200 => "outline-fuchsia-200",
        /// ```css
        /// {
        ///     outline-color: #f0abfc;
        /// }
        /// ```
        Fuchsia300 => "outline-fuchsia-300",
        /// ```css
        /// {
        ///     outline-color: #e879f9;
        /// }
        /// ```
        Fuchsia400 => "outline-fuchsia-400",
        /// ```css
        /// {
        ///     outline-color: #d946ef;
        /// }
        /// ```
        Fuchsia500 => "outline-fuchsia-500",
        /// ```css
        /// {
        ///     outline-color: #c026d3;
        /// }
        /// ```
        Fuchsia600 => "outline-fuchsia-600",
        /// ```css
        /// {
        ///     outline-color: #a21caf;
        /// }
        /// ```
        Fuchsia700 => "outline-fuchsia-700",
        /// ```css
        /// {
        ///     outline-color: #86198f;
        /// }
        /// ```
        Fuchsia800 => "outline-fuchsia-800",
        /// ```css
        /// {
        ///     outline-color: #701a75;
        /// }
        /// ```
        Fuchsia900 => "outline-fuchsia-900",
        /// ```css
        /// {
        ///     outline-color: #4a044e;
        /// }
        /// ```
        Fuchsia950 => "outline-fuchsia-950",
        /// ```css
        /// {
        ///     outline-color: #fdf2f8;
        /// }
        /// ```
        Pink50 => "outline-pink-50",
        /// ```css
        /// {
        ///     outline-color: #fce7f3;
        /// }
        /// ```
        Pink100 => "outline-pink-100",
        /// ```css
        /// {
        ///     outline-color: #fbcfe8;
        /// }
        /// ```
        Pink200 => "outline-pink-200",
        /// ```css
        /// {
        ///     outline-color: #f9a8d4;
        /// }
        /// ```
        Pink300 => "outline-pink-300",
        /// ```css
        /// {
        ///     outline-color: #f472b6;
        /// }
        /// ```
        Pink400 => "outline-pink-400",
        /// ```css
        /// {
        ///     outline-color: #ec4899;
        /// }
        /// ```
        Pink500 => "outline-pink-500",
        /// ```css
        /// {
        ///     outline-color: #db2777;
        /// }
        /// ```
        Pink600 => "outline-pink-600",
        /// ```css
        /// {
        ///     outline-color: #be185d;
        /// }
        /// ```
        Pink700 => "outline-pink-700",
        /// ```css
        /// {
        ///     outline-color: #9d174d;
        /// }
        /// ```
        Pink800 => "outline-pink-800",
        /// ```css
        /// {
        ///     outline-color: #831843;
        /// }
        /// ```
        Pink900 => "outline-pink-900",
        /// ```css
        /// {
        ///     outline-color: #500724;
        /// }
        /// ```
        Pink950 => "outline-pink-950",
        /// ```css
        /// {
        ///     outline-color: #fff1f2;
        /// }
        /// ```
        Rose50 => "outline-rose-50",
        /// ```css
        /// {
        ///     outline-color: #ffe4e6;
        /// }
        /// ```
        Rose100 => "outline-rose-100",
        /// ```css
        /// {
        ///     outline-color: #fecdd3;
        /// }
        /// ```
        Rose200 => "outline-rose-200",
        /// ```css
        /// {
        ///     outline-color: #fda4af;
        /// }
        /// ```
        Rose300 => "outline-rose-300",
        /// ```css
        /// {
        ///     outline-color: #fb7185;
        /// }
        /// ```
        Rose400 => "outline-rose-400",
        /// ```css
        /// {
        ///     outline-color: #f43f5e;
        /// }
        /// ```
        Rose500 => "outline-rose-500",
        /// ```css
        /// {
        ///     outline-color: #e11d48;
        /// }
        /// ```
        Rose600 => "outline-rose-600",
        /// ```css
        /// {
        ///     outline-color: #be123c;
        /// }
        /// ```
        Rose700 => "outline-rose-700",
        /// ```css
        /// {
        ///     outline-color: #9f1239;
        /// }
        /// ```
        Rose800 => "outline-rose-800",
        /// ```css
        /// {
        ///     outline-color: #881337;
        /// }
        /// ```
        Rose900 => "outline-rose-900",
        /// ```css
        /// {
        ///     outline-color: #4c0519;
        /// }
        /// ```
        Rose950 => "outline-rose-950",
    }
    /// Utilities for controlling the style of an element's outline.
    ///
    /// <https://tailwindcss.com/docs/outline-style>
    OutlineStyle {
        /// ```css
        /// {
        ///     outline: 2px solid transparent;
        ///     outline-offset: 2px;
        /// }
        /// ```
        None => "outline-none",
        /// ```css
        /// {
        ///     outline-style: solid;
        /// }
        /// ```
        Outline => "outline",
        /// ```css
        /// {
        ///     outline-style: dashed;
        /// }
        /// ```
        Dashed => "outline-dashed",
        /// ```css
        /// {
        ///     outline-style: dotted;
        /// }
        /// ```
        Dotted => "outline-dotted",
        /// ```css
        /// {
        ///     outline-style: double;
        /// }
        /// ```
        Double => "outline-double",
    }
    /// Utilities for controlling the offset of an element's outline.
    ///
    /// <https://tailwindcss.com/docs/outline-offset>
    OutlineOffset {
        /// ```css
        /// {
        ///     outline-offset: 0px;
        /// }
        /// ```
        _0 => "outline-offset-0",
        /// ```css
        /// {
        ///     outline-offset: 1px;
        /// }
        /// ```
        _1 => "outline-offset-1",
        /// ```css
        /// {
        ///     outline-offset: 2px;
        /// }
        /// ```
        _2 => "outline-offset-2",
        /// ```css
        /// {
        ///     outline-offset: 4px;
        /// }
        /// ```
        _4 => "outline-offset-4",
        /// ```css
        /// {
        ///     outline-offset: 8px;
        /// }
        /// ```
        _8 => "outline-offset-8",
    }
    /// Utilities for creating outline rings with box-shadows.
    ///
    /// <https://tailwindcss.com/docs/ring-width>
    RingWidth {
        /// ```css
        /// {
        ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color);
        /// }
        /// ```
        _0 => "ring-0",
        /// ```css
        /// {
        ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);
        /// }
        /// ```
        _1 => "ring-1",
        /// ```css
        /// {
        ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);
        /// }
        /// ```
        _2 => "ring-2",
        /// ```css
        /// {
        ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color);
        /// }
        /// ```
        Ring => "ring",
        /// ```css
        /// {
        ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color);
        /// }
        /// ```
        _4 => "ring-4",
        /// ```css
        /// {
        ///     box-shadow: var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color);
        /// }
        /// ```
        _8 => "ring-8",
        /// ```css
        /// {
        ///     --tw-ring-inset: inset;
        /// }
        /// ```
        Inset => "ring-inset",
    }
    /// Utilities for setting the color of outline rings.
    ///
    /// <https://tailwindcss.com/docs/ring-color>
    RingColor {
        /// ```css
        /// {
        ///     --tw-ring-color: inherit;
        /// }
        /// ```
        Inherit => "ring-inherit",
        /// ```css
        /// {
        ///     --tw-ring-color: currentColor;
        /// }
        /// ```
        Current => "ring-current",
        /// ```css
        /// {
        ///     --tw-ring-color: transparent;
        /// }
        /// ```
        Transparent => "ring-transparent",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(0 0 0);
        /// }
        /// ```
        Black => "ring-black",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(255 255 255);
        /// }
        /// ```
        White => "ring-white",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(248 250 252);
        /// }
        /// ```
        Slate50 => "ring-slate-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(241 245 249);
        /// }
        /// ```
        Slate100 => "ring-slate-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(226 232 240);
        /// }
        /// ```
        Slate200 => "ring-slate-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(203 213 225);
        /// }
        /// ```
        Slate300 => "ring-slate-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(148 163 184);
        /// }
        /// ```
        Slate400 => "ring-slate-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(100 116 139);
        /// }
        /// ```
        Slate500 => "ring-slate-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(71 85 105);
        /// }
        /// ```
        Slate600 => "ring-slate-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(51 65 85);
        /// }
        /// ```
        Slate700 => "ring-slate-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(30 41 59);
        /// }
        /// ```
        Slate800 => "ring-slate-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(15 23 42);
        /// }
        /// ```
        Slate900 => "ring-slate-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(2 6 23);
        /// }
        /// ```
        Slate950 => "ring-slate-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(249 250 251);
        /// }
        /// ```
        Gray50 => "ring-gray-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(243 244 246);
        /// }
        /// ```
        Gray100 => "ring-gray-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(229 231 235);
        /// }
        /// ```
        Gray200 => "ring-gray-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(209 213 219);
        /// }
        /// ```
        Gray300 => "ring-gray-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(156 163 175);
        /// }
        /// ```
        Gray400 => "ring-gray-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(107 114 128);
        /// }
        /// ```
        Gray500 => "ring-gray-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(75 85 99);
        /// }
        /// ```
        Gray600 => "ring-gray-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(55 65 81);
        /// }
        /// ```
        Gray700 => "ring-gray-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(31 41 55);
        /// }
        /// ```
        Gray800 => "ring-gray-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(17 24 39);
        /// }
        /// ```
        Gray900 => "ring-gray-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(3 7 18);
        /// }
        /// ```
        Gray950 => "ring-gray-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(250 250 250);
        /// }
        /// ```
        Zinc50 => "ring-zinc-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(244 244 245);
        /// }
        /// ```
        Zinc100 => "ring-zinc-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(228 228 231);
        /// }
        /// ```
        Zinc200 => "ring-zinc-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(212 212 216);
        /// }
        /// ```
        Zinc300 => "ring-zinc-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(161 161 170);
        /// }
        /// ```
        Zinc400 => "ring-zinc-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(113 113 122);
        /// }
        /// ```
        Zinc500 => "ring-zinc-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(82 82 91);
        /// }
        /// ```
        Zinc600 => "ring-zinc-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(63 63 70);
        /// }
        /// ```
        Zinc700 => "ring-zinc-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(39 39 42);
        /// }
        /// ```
        Zinc800 => "ring-zinc-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(24 24 27);
        /// }
        /// ```
        Zinc900 => "ring-zinc-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(9 9 11);
        /// }
        /// ```
        Zinc950 => "ring-zinc-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(250 250 250);
        /// }
        /// ```
        Neutral50 => "ring-neutral-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(245 245 245);
        /// }
        /// ```
        Neutral100 => "ring-neutral-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(229 229 229);
        /// }
        /// ```
        Neutral200 => "ring-neutral-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(212 212 212);
        /// }
        /// ```
        Neutral300 => "ring-neutral-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(163 163 163);
        /// }
        /// ```
        Neutral400 => "ring-neutral-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(115 115 115);
        /// }
        /// ```
        Neutral500 => "ring-neutral-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(82 82 82);
        /// }
        /// ```
        Neutral600 => "ring-neutral-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(64 64 64);
        /// }
        /// ```
        Neutral700 => "ring-neutral-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(38 38 38);
        /// }
        /// ```
        Neutral800 => "ring-neutral-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(23 23 23);
        /// }
        /// ```
        Neutral900 => "ring-neutral-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(10 10 10);
        /// }
        /// ```
        Neutral950 => "ring-neutral-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(250 250 249);
        /// }
        /// ```
        Stone50 => "ring-stone-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(245 245 244);
        /// }
        /// ```
        Stone100 => "ring-stone-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(231 229 228);
        /// }
        /// ```
        Stone200 => "ring-stone-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(214 211 209);
        /// }
        /// ```
        Stone300 => "ring-stone-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(168 162 158);
        /// }
        /// ```
        Stone400 => "ring-stone-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(120 113 108);
        /// }
        /// ```
        Stone500 => "ring-stone-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(87 83 78);
        /// }
        /// ```
        Stone600 => "ring-stone-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(68 64 60);
        /// }
        /// ```
        Stone700 => "ring-stone-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(41 37 36);
        /// }
        /// ```
        Stone800 => "ring-stone-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(28 25 23);
        /// }
        /// ```
        Stone900 => "ring-stone-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(12 10 9);
        /// }
        /// ```
        Stone950 => "ring-stone-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 242 242);
        /// }
        /// ```
        Red50 => "ring-red-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 226 226);
        /// }
        /// ```
        Red100 => "ring-red-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 202 202);
        /// }
        /// ```
        Red200 => "ring-red-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(252 165 165);
        /// }
        /// ```
        Red300 => "ring-red-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(248 113 113);
        /// }
        /// ```
        Red400 => "ring-red-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(239 68 68);
        /// }
        /// ```
        Red500 => "ring-red-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(220 38 38);
        /// }
        /// ```
        Red600 => "ring-red-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(185 28 28);
        /// }
        /// ```
        Red700 => "ring-red-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(153 27 27);
        /// }
        /// ```
        Red800 => "ring-red-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(127 29 29);
        /// }
        /// ```
        Red900 => "ring-red-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(69 10 10);
        /// }
        /// ```
        Red950 => "ring-red-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(255 247 237);
        /// }
        /// ```
        Orange50 => "ring-orange-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(255 237 213);
        /// }
        /// ```
        Orange100 => "ring-orange-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 215 170);
        /// }
        /// ```
        Orange200 => "ring-orange-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(253 186 116);
        /// }
        /// ```
        Orange300 => "ring-orange-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(251 146 60);
        /// }
        /// ```
        Orange400 => "ring-orange-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(249 115 22);
        /// }
        /// ```
        Orange500 => "ring-orange-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(234 88 12);
        /// }
        /// ```
        Orange600 => "ring-orange-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(194 65 12);
        /// }
        /// ```
        Orange700 => "ring-orange-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(154 52 18);
        /// }
        /// ```
        Orange800 => "ring-orange-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(124 45 18);
        /// }
        /// ```
        Orange900 => "ring-orange-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(67 20 7);
        /// }
        /// ```
        Orange950 => "ring-orange-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(255 251 235);
        /// }
        /// ```
        Amber50 => "ring-amber-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 243 199);
        /// }
        /// ```
        Amber100 => "ring-amber-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(253 230 138);
        /// }
        /// ```
        Amber200 => "ring-amber-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(252 211 77);
        /// }
        /// ```
        Amber300 => "ring-amber-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(251 191 36);
        /// }
        /// ```
        Amber400 => "ring-amber-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(245 158 11);
        /// }
        /// ```
        Amber500 => "ring-amber-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(217 119 6);
        /// }
        /// ```
        Amber600 => "ring-amber-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(180 83 9);
        /// }
        /// ```
        Amber700 => "ring-amber-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(146 64 14);
        /// }
        /// ```
        Amber800 => "ring-amber-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(120 53 15);
        /// }
        /// ```
        Amber900 => "ring-amber-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(69 26 3);
        /// }
        /// ```
        Amber950 => "ring-amber-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 252 232);
        /// }
        /// ```
        Yellow50 => "ring-yellow-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 249 195);
        /// }
        /// ```
        Yellow100 => "ring-yellow-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 240 138);
        /// }
        /// ```
        Yellow200 => "ring-yellow-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(253 224 71);
        /// }
        /// ```
        Yellow300 => "ring-yellow-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(250 204 21);
        /// }
        /// ```
        Yellow400 => "ring-yellow-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(234 179 8);
        /// }
        /// ```
        Yellow500 => "ring-yellow-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(202 138 4);
        /// }
        /// ```
        Yellow600 => "ring-yellow-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(161 98 7);
        /// }
        /// ```
        Yellow700 => "ring-yellow-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(133 77 14);
        /// }
        /// ```
        Yellow800 => "ring-yellow-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(113 63 18);
        /// }
        /// ```
        Yellow900 => "ring-yellow-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(66 32 6);
        /// }
        /// ```
        Yellow950 => "ring-yellow-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(247 254 231);
        /// }
        /// ```
        Lime50 => "ring-lime-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(236 252 203);
        /// }
        /// ```
        Lime100 => "ring-lime-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(217 249 157);
        /// }
        /// ```
        Lime200 => "ring-lime-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(190 242 100);
        /// }
        /// ```
        Lime300 => "ring-lime-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(163 230 53);
        /// }
        /// ```
        Lime400 => "ring-lime-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(132 204 22);
        /// }
        /// ```
        Lime500 => "ring-lime-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(101 163 13);
        /// }
        /// ```
        Lime600 => "ring-lime-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(77 124 15);
        /// }
        /// ```
        Lime700 => "ring-lime-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(63 98 18);
        /// }
        /// ```
        Lime800 => "ring-lime-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(54 83 20);
        /// }
        /// ```
        Lime900 => "ring-lime-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(26 46 5);
        /// }
        /// ```
        Lime950 => "ring-lime-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(240 253 244);
        /// }
        /// ```
        Green50 => "ring-green-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(220 252 231);
        /// }
        /// ```
        Green100 => "ring-green-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(187 247 208);
        /// }
        /// ```
        Green200 => "ring-green-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(134 239 172);
        /// }
        /// ```
        Green300 => "ring-green-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(74 222 128);
        /// }
        /// ```
        Green400 => "ring-green-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(34 197 94);
        /// }
        /// ```
        Green500 => "ring-green-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(22 163 74);
        /// }
        /// ```
        Green600 => "ring-green-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(21 128 61);
        /// }
        /// ```
        Green700 => "ring-green-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(22 101 52);
        /// }
        /// ```
        Green800 => "ring-green-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(20 83 45);
        /// }
        /// ```
        Green900 => "ring-green-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(5 46 22);
        /// }
        /// ```
        Green950 => "ring-green-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(236 253 245);
        /// }
        /// ```
        Emerald50 => "ring-emerald-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(209 250 229);
        /// }
        /// ```
        Emerald100 => "ring-emerald-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(167 243 208);
        /// }
        /// ```
        Emerald200 => "ring-emerald-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(110 231 183);
        /// }
        /// ```
        Emerald300 => "ring-emerald-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(52 211 153);
        /// }
        /// ```
        Emerald400 => "ring-emerald-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(16 185 129);
        /// }
        /// ```
        Emerald500 => "ring-emerald-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(5 150 105);
        /// }
        /// ```
        Emerald600 => "ring-emerald-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(4 120 87);
        /// }
        /// ```
        Emerald700 => "ring-emerald-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(6 95 70);
        /// }
        /// ```
        Emerald800 => "ring-emerald-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(6 78 59);
        /// }
        /// ```
        Emerald900 => "ring-emerald-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(2 44 34);
        /// }
        /// ```
        Emerald950 => "ring-emerald-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(240 253 250);
        /// }
        /// ```
        Teal50 => "ring-teal-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(204 251 241);
        /// }
        /// ```
        Teal100 => "ring-teal-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(153 246 228);
        /// }
        /// ```
        Teal200 => "ring-teal-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(94 234 212);
        /// }
        /// ```
        Teal300 => "ring-teal-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(45 212 191);
        /// }
        /// ```
        Teal400 => "ring-teal-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(20 184 166);
        /// }
        /// ```
        Teal500 => "ring-teal-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(13 148 136);
        /// }
        /// ```
        Teal600 => "ring-teal-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(15 118 110);
        /// }
        /// ```
        Teal700 => "ring-teal-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(17 94 89);
        /// }
        /// ```
        Teal800 => "ring-teal-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(19 78 74);
        /// }
        /// ```
        Teal900 => "ring-teal-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(4 47 46);
        /// }
        /// ```
        Teal950 => "ring-teal-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(236 254 255);
        /// }
        /// ```
        Cyan50 => "ring-cyan-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(207 250 254);
        /// }
        /// ```
        Cyan100 => "ring-cyan-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(165 243 252);
        /// }
        /// ```
        Cyan200 => "ring-cyan-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(103 232 249);
        /// }
        /// ```
        Cyan300 => "ring-cyan-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(34 211 238);
        /// }
        /// ```
        Cyan400 => "ring-cyan-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(6 182 212);
        /// }
        /// ```
        Cyan500 => "ring-cyan-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(8 145 178);
        /// }
        /// ```
        Cyan600 => "ring-cyan-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(14 116 144);
        /// }
        /// ```
        Cyan700 => "ring-cyan-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(21 94 117);
        /// }
        /// ```
        Cyan800 => "ring-cyan-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(22 78 99);
        /// }
        /// ```
        Cyan900 => "ring-cyan-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(8 51 68);
        /// }
        /// ```
        Cyan950 => "ring-cyan-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(240 249 255);
        /// }
        /// ```
        Sky50 => "ring-sky-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(224 242 254);
        /// }
        /// ```
        Sky100 => "ring-sky-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(186 230 253);
        /// }
        /// ```
        Sky200 => "ring-sky-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(125 211 252);
        /// }
        /// ```
        Sky300 => "ring-sky-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(56 189 248);
        /// }
        /// ```
        Sky400 => "ring-sky-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(14 165 233);
        /// }
        /// ```
        Sky500 => "ring-sky-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(2 132 199);
        /// }
        /// ```
        Sky600 => "ring-sky-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(3 105 161);
        /// }
        /// ```
        Sky700 => "ring-sky-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(7 89 133);
        /// }
        /// ```
        Sky800 => "ring-sky-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(12 74 110);
        /// }
        /// ```
        Sky900 => "ring-sky-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(8 47 73);
        /// }
        /// ```
        Sky950 => "ring-sky-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(239 246 255);
        /// }
        /// ```
        Blue50 => "ring-blue-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(219 234 254);
        /// }
        /// ```
        Blue100 => "ring-blue-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(191 219 254);
        /// }
        /// ```
        Blue200 => "ring-blue-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(147 197 253);
        /// }
        /// ```
        Blue300 => "ring-blue-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(96 165 250);
        /// }
        /// ```
        Blue400 => "ring-blue-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(59 130 246);
        /// }
        /// ```
        Blue500 => "ring-blue-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(37 99 235);
        /// }
        /// ```
        Blue600 => "ring-blue-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(29 78 216);
        /// }
        /// ```
        Blue700 => "ring-blue-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(30 64 175);
        /// }
        /// ```
        Blue800 => "ring-blue-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(30 58 138);
        /// }
        /// ```
        Blue900 => "ring-blue-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(23 37 84);
        /// }
        /// ```
        Blue950 => "ring-blue-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(238 242 255);
        /// }
        /// ```
        Indigo50 => "ring-indigo-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(224 231 255);
        /// }
        /// ```
        Indigo100 => "ring-indigo-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(199 210 254);
        /// }
        /// ```
        Indigo200 => "ring-indigo-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(165 180 252);
        /// }
        /// ```
        Indigo300 => "ring-indigo-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(129 140 248);
        /// }
        /// ```
        Indigo400 => "ring-indigo-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(99 102 241);
        /// }
        /// ```
        Indigo500 => "ring-indigo-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(79 70 229);
        /// }
        /// ```
        Indigo600 => "ring-indigo-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(67 56 202);
        /// }
        /// ```
        Indigo700 => "ring-indigo-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(55 48 163);
        /// }
        /// ```
        Indigo800 => "ring-indigo-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(49 46 129);
        /// }
        /// ```
        Indigo900 => "ring-indigo-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(30 27 75);
        /// }
        /// ```
        Indigo950 => "ring-indigo-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(245 243 255);
        /// }
        /// ```
        Violet50 => "ring-violet-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(237 233 254);
        /// }
        /// ```
        Violet100 => "ring-violet-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(221 214 254);
        /// }
        /// ```
        Violet200 => "ring-violet-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(196 181 253);
        /// }
        /// ```
        Violet300 => "ring-violet-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(167 139 250);
        /// }
        /// ```
        Violet400 => "ring-violet-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(139 92 246);
        /// }
        /// ```
        Violet500 => "ring-violet-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(124 58 237);
        /// }
        /// ```
        Violet600 => "ring-violet-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(109 40 217);
        /// }
        /// ```
        Violet700 => "ring-violet-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(91 33 182);
        /// }
        /// ```
        Violet800 => "ring-violet-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(76 29 149);
        /// }
        /// ```
        Violet900 => "ring-violet-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(46 16 101);
        /// }
        /// ```
        Violet950 => "ring-violet-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(250 245 255);
        /// }
        /// ```
        Purple50 => "ring-purple-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(243 232 255);
        /// }
        /// ```
        Purple100 => "ring-purple-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(233 213 255);
        /// }
        /// ```
        Purple200 => "ring-purple-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(216 180 254);
        /// }
        /// ```
        Purple300 => "ring-purple-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(192 132 252);
        /// }
        /// ```
        Purple400 => "ring-purple-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(168 85 247);
        /// }
        /// ```
        Purple500 => "ring-purple-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(147 51 234);
        /// }
        /// ```
        Purple600 => "ring-purple-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(126 34 206);
        /// }
        /// ```
        Purple700 => "ring-purple-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(107 33 168);
        /// }
        /// ```
        Purple800 => "ring-purple-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(88 28 135);
        /// }
        /// ```
        Purple900 => "ring-purple-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(59 7 100);
        /// }
        /// ```
        Purple950 => "ring-purple-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(253 244 255);
        /// }
        /// ```
        Fuchsia50 => "ring-fuchsia-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(250 232 255);
        /// }
        /// ```
        Fuchsia100 => "ring-fuchsia-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(245 208 254);
        /// }
        /// ```
        Fuchsia200 => "ring-fuchsia-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(240 171 252);
        /// }
        /// ```
        Fuchsia300 => "ring-fuchsia-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(232 121 249);
        /// }
        /// ```
        Fuchsia400 => "ring-fuchsia-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(217 70 239);
        /// }
        /// ```
        Fuchsia500 => "ring-fuchsia-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(192 38 211);
        /// }
        /// ```
        Fuchsia600 => "ring-fuchsia-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(162 28 175);
        /// }
        /// ```
        Fuchsia700 => "ring-fuchsia-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(134 25 143);
        /// }
        /// ```
        Fuchsia800 => "ring-fuchsia-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(112 26 117);
        /// }
        /// ```
        Fuchsia900 => "ring-fuchsia-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(74 4 78);
        /// }
        /// ```
        Fuchsia950 => "ring-fuchsia-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(253 242 248);
        /// }
        /// ```
        Pink50 => "ring-pink-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(252 231 243);
        /// }
        /// ```
        Pink100 => "ring-pink-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(251 207 232);
        /// }
        /// ```
        Pink200 => "ring-pink-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(249 168 212);
        /// }
        /// ```
        Pink300 => "ring-pink-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(244 114 182);
        /// }
        /// ```
        Pink400 => "ring-pink-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(236 72 153);
        /// }
        /// ```
        Pink500 => "ring-pink-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(219 39 119);
        /// }
        /// ```
        Pink600 => "ring-pink-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(190 24 93);
        /// }
        /// ```
        Pink700 => "ring-pink-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(157 23 77);
        /// }
        /// ```
        Pink800 => "ring-pink-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(131 24 67);
        /// }
        /// ```
        Pink900 => "ring-pink-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(80 7 36);
        /// }
        /// ```
        Pink950 => "ring-pink-950",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(255 241 242);
        /// }
        /// ```
        Rose50 => "ring-rose-50",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(255 228 230);
        /// }
        /// ```
        Rose100 => "ring-rose-100",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(254 205 211);
        /// }
        /// ```
        Rose200 => "ring-rose-200",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(253 164 175);
        /// }
        /// ```
        Rose300 => "ring-rose-300",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(251 113 133);
        /// }
        /// ```
        Rose400 => "ring-rose-400",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(244 63 94);
        /// }
        /// ```
        Rose500 => "ring-rose-500",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(225 29 72);
        /// }
        /// ```
        Rose600 => "ring-rose-600",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(190 18 60);
        /// }
        /// ```
        Rose700 => "ring-rose-700",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(159 18 57);
        /// }
        /// ```
        Rose800 => "ring-rose-800",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(136 19 55);
        /// }
        /// ```
        Rose900 => "ring-rose-900",
        /// ```css
        /// {
        ///     --tw-ring-color: rgb(76 5 25);
        /// }
        /// ```
        Rose950 => "ring-rose-950",
    }
    /// Utilities for simulating an offset when adding outline rings.
    ///
    /// <https://tailwindcss.com/docs/ring-offset-width>
    RingOffsetWidth {
        /// ```css
        /// {
        ///     --tw-ring-offset-width: 0px;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        _0 => "ring-offset-0",
        /// ```css
        /// {
        ///     --tw-ring-offset-width: 1px;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        _1 => "ring-offset-1",
        /// ```css
        /// {
        ///     --tw-ring-offset-width: 2px;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        _2 => "ring-offset-2",
        /// ```css
        /// {
        ///     --tw-ring-offset-width: 4px;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        _4 => "ring-offset-4",
        /// ```css
        /// {
        ///     --tw-ring-offset-width: 8px;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        _8 => "ring-offset-8",
    }
    /// Utilities for setting the color of outline ring offsets.
    ///
    /// <https://tailwindcss.com/docs/ring-offset-color>
    RingOffsetColor {
        /// ```css
        /// {
        ///     --tw-ring-offset-color: inherit;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Inherit => "ring-offset-inherit",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: currentColor;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Current => "ring-offset-current",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: transparent;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Transparent => "ring-offset-transparent",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #000;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Black => "ring-offset-black",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        White => "ring-offset-white",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f8fafc;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate50 => "ring-offset-slate-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f1f5f9;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate100 => "ring-offset-slate-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e2e8f0;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate200 => "ring-offset-slate-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #cbd5e1;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate300 => "ring-offset-slate-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #94a3b8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate400 => "ring-offset-slate-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #64748b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate500 => "ring-offset-slate-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #475569;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate600 => "ring-offset-slate-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #334155;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate700 => "ring-offset-slate-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #1e293b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate800 => "ring-offset-slate-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0f172a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate900 => "ring-offset-slate-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #020617;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Slate950 => "ring-offset-slate-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f9fafb;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray50 => "ring-offset-gray-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f3f4f6;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray100 => "ring-offset-gray-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e5e7eb;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray200 => "ring-offset-gray-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d1d5db;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray300 => "ring-offset-gray-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #9ca3af;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray400 => "ring-offset-gray-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #6b7280;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray500 => "ring-offset-gray-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #4b5563;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray600 => "ring-offset-gray-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #374151;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray700 => "ring-offset-gray-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #1f2937;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray800 => "ring-offset-gray-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #111827;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray900 => "ring-offset-gray-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #030712;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Gray950 => "ring-offset-gray-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fafafa;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc50 => "ring-offset-zinc-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f4f4f5;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc100 => "ring-offset-zinc-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e4e4e7;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc200 => "ring-offset-zinc-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d4d4d8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc300 => "ring-offset-zinc-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a1a1aa;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc400 => "ring-offset-zinc-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #71717a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc500 => "ring-offset-zinc-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #52525b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc600 => "ring-offset-zinc-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #3f3f46;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc700 => "ring-offset-zinc-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #27272a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc800 => "ring-offset-zinc-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #18181b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc900 => "ring-offset-zinc-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #09090b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Zinc950 => "ring-offset-zinc-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fafafa;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral50 => "ring-offset-neutral-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f5f5f5;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral100 => "ring-offset-neutral-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e5e5e5;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral200 => "ring-offset-neutral-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d4d4d4;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral300 => "ring-offset-neutral-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a3a3a3;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral400 => "ring-offset-neutral-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #737373;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral500 => "ring-offset-neutral-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #525252;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral600 => "ring-offset-neutral-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #404040;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral700 => "ring-offset-neutral-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #262626;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral800 => "ring-offset-neutral-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #171717;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral900 => "ring-offset-neutral-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0a0a0a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Neutral950 => "ring-offset-neutral-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fafaf9;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone50 => "ring-offset-stone-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f5f5f4;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone100 => "ring-offset-stone-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e7e5e4;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone200 => "ring-offset-stone-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d6d3d1;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone300 => "ring-offset-stone-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a8a29e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone400 => "ring-offset-stone-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #78716c;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone500 => "ring-offset-stone-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #57534e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone600 => "ring-offset-stone-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #44403c;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone700 => "ring-offset-stone-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #292524;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone800 => "ring-offset-stone-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #1c1917;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone900 => "ring-offset-stone-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0c0a09;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Stone950 => "ring-offset-stone-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fef2f2;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red50 => "ring-offset-red-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fee2e2;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red100 => "ring-offset-red-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fecaca;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red200 => "ring-offset-red-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fca5a5;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red300 => "ring-offset-red-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f87171;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red400 => "ring-offset-red-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ef4444;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red500 => "ring-offset-red-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #dc2626;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red600 => "ring-offset-red-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #b91c1c;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red700 => "ring-offset-red-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #991b1b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red800 => "ring-offset-red-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #7f1d1d;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red900 => "ring-offset-red-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #450a0a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Red950 => "ring-offset-red-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fff7ed;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange50 => "ring-offset-orange-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ffedd5;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange100 => "ring-offset-orange-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fed7aa;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange200 => "ring-offset-orange-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fdba74;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange300 => "ring-offset-orange-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fb923c;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange400 => "ring-offset-orange-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f97316;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange500 => "ring-offset-orange-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ea580c;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange600 => "ring-offset-orange-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #c2410c;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange700 => "ring-offset-orange-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #9a3412;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange800 => "ring-offset-orange-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #7c2d12;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange900 => "ring-offset-orange-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #431407;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Orange950 => "ring-offset-orange-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fffbeb;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber50 => "ring-offset-amber-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fef3c7;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber100 => "ring-offset-amber-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fde68a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber200 => "ring-offset-amber-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fcd34d;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber300 => "ring-offset-amber-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fbbf24;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber400 => "ring-offset-amber-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f59e0b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber500 => "ring-offset-amber-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d97706;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber600 => "ring-offset-amber-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #b45309;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber700 => "ring-offset-amber-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #92400e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber800 => "ring-offset-amber-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #78350f;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber900 => "ring-offset-amber-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #451a03;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Amber950 => "ring-offset-amber-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fefce8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow50 => "ring-offset-yellow-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fef9c3;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow100 => "ring-offset-yellow-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fef08a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow200 => "ring-offset-yellow-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fde047;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow300 => "ring-offset-yellow-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #facc15;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow400 => "ring-offset-yellow-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #eab308;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow500 => "ring-offset-yellow-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ca8a04;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow600 => "ring-offset-yellow-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a16207;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow700 => "ring-offset-yellow-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #854d0e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow800 => "ring-offset-yellow-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #713f12;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow900 => "ring-offset-yellow-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #422006;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Yellow950 => "ring-offset-yellow-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f7fee7;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime50 => "ring-offset-lime-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ecfccb;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime100 => "ring-offset-lime-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d9f99d;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime200 => "ring-offset-lime-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #bef264;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime300 => "ring-offset-lime-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a3e635;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime400 => "ring-offset-lime-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #84cc16;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime500 => "ring-offset-lime-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #65a30d;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime600 => "ring-offset-lime-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #4d7c0f;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime700 => "ring-offset-lime-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #3f6212;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime800 => "ring-offset-lime-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #365314;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime900 => "ring-offset-lime-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #1a2e05;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Lime950 => "ring-offset-lime-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f0fdf4;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green50 => "ring-offset-green-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #dcfce7;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green100 => "ring-offset-green-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #bbf7d0;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green200 => "ring-offset-green-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #86efac;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green300 => "ring-offset-green-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #4ade80;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green400 => "ring-offset-green-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #22c55e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green500 => "ring-offset-green-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #16a34a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green600 => "ring-offset-green-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #15803d;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green700 => "ring-offset-green-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #166534;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green800 => "ring-offset-green-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #14532d;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green900 => "ring-offset-green-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #052e16;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Green950 => "ring-offset-green-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ecfdf5;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald50 => "ring-offset-emerald-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d1fae5;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald100 => "ring-offset-emerald-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a7f3d0;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald200 => "ring-offset-emerald-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #6ee7b7;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald300 => "ring-offset-emerald-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #34d399;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald400 => "ring-offset-emerald-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #10b981;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald500 => "ring-offset-emerald-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #059669;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald600 => "ring-offset-emerald-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #047857;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald700 => "ring-offset-emerald-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #065f46;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald800 => "ring-offset-emerald-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #064e3b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald900 => "ring-offset-emerald-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #022c22;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Emerald950 => "ring-offset-emerald-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f0fdfa;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal50 => "ring-offset-teal-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ccfbf1;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal100 => "ring-offset-teal-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #99f6e4;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal200 => "ring-offset-teal-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #5eead4;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal300 => "ring-offset-teal-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #2dd4bf;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal400 => "ring-offset-teal-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #14b8a6;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal500 => "ring-offset-teal-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0d9488;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal600 => "ring-offset-teal-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0f766e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal700 => "ring-offset-teal-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #115e59;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal800 => "ring-offset-teal-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #134e4a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal900 => "ring-offset-teal-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #042f2e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Teal950 => "ring-offset-teal-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ecfeff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan50 => "ring-offset-cyan-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #cffafe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan100 => "ring-offset-cyan-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a5f3fc;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan200 => "ring-offset-cyan-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #67e8f9;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan300 => "ring-offset-cyan-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #22d3ee;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan400 => "ring-offset-cyan-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #06b6d4;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan500 => "ring-offset-cyan-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0891b2;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan600 => "ring-offset-cyan-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0e7490;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan700 => "ring-offset-cyan-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #155e75;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan800 => "ring-offset-cyan-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #164e63;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan900 => "ring-offset-cyan-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #083344;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Cyan950 => "ring-offset-cyan-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f0f9ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky50 => "ring-offset-sky-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e0f2fe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky100 => "ring-offset-sky-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #bae6fd;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky200 => "ring-offset-sky-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #7dd3fc;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky300 => "ring-offset-sky-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #38bdf8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky400 => "ring-offset-sky-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0ea5e9;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky500 => "ring-offset-sky-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0284c7;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky600 => "ring-offset-sky-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0369a1;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky700 => "ring-offset-sky-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #075985;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky800 => "ring-offset-sky-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #0c4a6e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky900 => "ring-offset-sky-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #082f49;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Sky950 => "ring-offset-sky-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #eff6ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue50 => "ring-offset-blue-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #dbeafe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue100 => "ring-offset-blue-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #bfdbfe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue200 => "ring-offset-blue-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #93c5fd;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue300 => "ring-offset-blue-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #60a5fa;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue400 => "ring-offset-blue-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #3b82f6;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue500 => "ring-offset-blue-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #2563eb;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue600 => "ring-offset-blue-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #1d4ed8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue700 => "ring-offset-blue-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #1e40af;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue800 => "ring-offset-blue-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #1e3a8a;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue900 => "ring-offset-blue-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #172554;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Blue950 => "ring-offset-blue-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #eef2ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo50 => "ring-offset-indigo-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e0e7ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo100 => "ring-offset-indigo-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #c7d2fe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo200 => "ring-offset-indigo-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a5b4fc;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo300 => "ring-offset-indigo-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #818cf8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo400 => "ring-offset-indigo-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #6366f1;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo500 => "ring-offset-indigo-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #4f46e5;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo600 => "ring-offset-indigo-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #4338ca;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo700 => "ring-offset-indigo-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #3730a3;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo800 => "ring-offset-indigo-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #312e81;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo900 => "ring-offset-indigo-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #1e1b4b;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Indigo950 => "ring-offset-indigo-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f5f3ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet50 => "ring-offset-violet-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ede9fe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet100 => "ring-offset-violet-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ddd6fe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet200 => "ring-offset-violet-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #c4b5fd;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet300 => "ring-offset-violet-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a78bfa;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet400 => "ring-offset-violet-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #8b5cf6;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet500 => "ring-offset-violet-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #7c3aed;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet600 => "ring-offset-violet-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #6d28d9;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet700 => "ring-offset-violet-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #5b21b6;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet800 => "ring-offset-violet-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #4c1d95;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet900 => "ring-offset-violet-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #2e1065;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Violet950 => "ring-offset-violet-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #faf5ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple50 => "ring-offset-purple-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f3e8ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple100 => "ring-offset-purple-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e9d5ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple200 => "ring-offset-purple-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d8b4fe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple300 => "ring-offset-purple-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #c084fc;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple400 => "ring-offset-purple-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a855f7;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple500 => "ring-offset-purple-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #9333ea;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple600 => "ring-offset-purple-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #7e22ce;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple700 => "ring-offset-purple-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #6b21a8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple800 => "ring-offset-purple-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #581c87;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple900 => "ring-offset-purple-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #3b0764;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Purple950 => "ring-offset-purple-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fdf4ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia50 => "ring-offset-fuchsia-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fae8ff;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia100 => "ring-offset-fuchsia-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f5d0fe;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia200 => "ring-offset-fuchsia-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f0abfc;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia300 => "ring-offset-fuchsia-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e879f9;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia400 => "ring-offset-fuchsia-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #d946ef;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia500 => "ring-offset-fuchsia-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #c026d3;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia600 => "ring-offset-fuchsia-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #a21caf;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia700 => "ring-offset-fuchsia-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #86198f;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia800 => "ring-offset-fuchsia-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #701a75;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia900 => "ring-offset-fuchsia-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #4a044e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Fuchsia950 => "ring-offset-fuchsia-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fdf2f8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink50 => "ring-offset-pink-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fce7f3;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink100 => "ring-offset-pink-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fbcfe8;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink200 => "ring-offset-pink-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f9a8d4;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink300 => "ring-offset-pink-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f472b6;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink400 => "ring-offset-pink-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ec4899;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink500 => "ring-offset-pink-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #db2777;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink600 => "ring-offset-pink-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #be185d;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink700 => "ring-offset-pink-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #9d174d;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink800 => "ring-offset-pink-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #831843;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink900 => "ring-offset-pink-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #500724;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Pink950 => "ring-offset-pink-950",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fff1f2;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose50 => "ring-offset-rose-50",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #ffe4e6;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose100 => "ring-offset-rose-100",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fecdd3;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose200 => "ring-offset-rose-200",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fda4af;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose300 => "ring-offset-rose-300",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #fb7185;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose400 => "ring-offset-rose-400",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #f43f5e;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose500 => "ring-offset-rose-500",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #e11d48;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose600 => "ring-offset-rose-600",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #be123c;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose700 => "ring-offset-rose-700",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #9f1239;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose800 => "ring-offset-rose-800",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #881337;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose900 => "ring-offset-rose-900",
        /// ```css
        /// {
        ///     --tw-ring-offset-color: #4c0519;
        ///     box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
        /// }
        /// ```
        Rose950 => "ring-offset-rose-950",
    }
);
