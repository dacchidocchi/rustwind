def_types!(
    /// Utilities for applying blur filters to an element.
    ///
    /// <https://tailwindcss.com/docs/blur>
    Blur {
        /// ```css
        /// {
        ///     filter:  ;
        /// }
        /// ```
        None => "blur-none",
        /// ```css
        /// {
        ///     filter: blur(4px);
        /// }
        /// ```
        Sm => "blur-sm",
        /// ```css
        /// {
        ///     filter: blur(8px);
        /// }
        /// ```
        Blur => "blur",
        /// ```css
        /// {
        ///     filter: blur(12px);
        /// }
        /// ```
        Md => "blur-md",
        /// ```css
        /// {
        ///     filter: blur(16px);
        /// }
        /// ```
        Lg => "blur-lg",
        /// ```css
        /// {
        ///     filter: blur(24px);
        /// }
        /// ```
        Xl => "blur-xl",
        /// ```css
        /// {
        ///     filter: blur(40px);
        /// }
        /// ```
        _2xl => "blur-2xl",
        /// ```css
        /// {
        ///     filter: blur(64px);
        /// }
        /// ```
        _3xl => "blur-3xl",
    }
    /// Utilities for applying brightness filters to an element.
    ///
    /// <https://tailwindcss.com/docs/brightness>
    Brightness {
        /// ```css
        /// {
        ///     filter: brightness(0);
        /// }
        /// ```
        _0 => "brightness-0",
        /// ```css
        /// {
        ///     filter: brightness(.5);
        /// }
        /// ```
        _50 => "brightness-50",
        /// ```css
        /// {
        ///     filter: brightness(.75);
        /// }
        /// ```
        _75 => "brightness-75",
        /// ```css
        /// {
        ///     filter: brightness(.9);
        /// }
        /// ```
        _90 => "brightness-90",
        /// ```css
        /// {
        ///     filter: brightness(.95);
        /// }
        /// ```
        _95 => "brightness-95",
        /// ```css
        /// {
        ///     filter: brightness(1);
        /// }
        /// ```
        _100 => "brightness-100",
        /// ```css
        /// {
        ///     filter: brightness(1.05);
        /// }
        /// ```
        _105 => "brightness-105",
        /// ```css
        /// {
        ///     filter: brightness(1.1);
        /// }
        /// ```
        _110 => "brightness-110",
        /// ```css
        /// {
        ///     filter: brightness(1.25);
        /// }
        /// ```
        _125 => "brightness-125",
        /// ```css
        /// {
        ///     filter: brightness(1.5);
        /// }
        /// ```
        _150 => "brightness-150",
        /// ```css
        /// {
        ///     filter: brightness(2);
        /// }
        /// ```
        _200 => "brightness-200",
    }
    /// Utilities for applying contrast filters to an element.
    ///
    /// <https://tailwindcss.com/docs/contrast>
    Contrast {
        /// ```css
        /// {
        ///     filter: contrast(0);
        /// }
        /// ```
        _0 => "contrast-0",
        /// ```css
        /// {
        ///     filter: contrast(.5);
        /// }
        /// ```
        _50 => "contrast-50",
        /// ```css
        /// {
        ///     filter: contrast(.75);
        /// }
        /// ```
        _75 => "contrast-75",
        /// ```css
        /// {
        ///     filter: contrast(1);
        /// }
        /// ```
        _100 => "contrast-100",
        /// ```css
        /// {
        ///     filter: contrast(1.25);
        /// }
        /// ```
        _125 => "contrast-125",
        /// ```css
        /// {
        ///     filter: contrast(1.5);
        /// }
        /// ```
        _150 => "contrast-150",
        /// ```css
        /// {
        ///     filter: contrast(2);
        /// }
        /// ```
        _200 => "contrast-200",
    }
    /// Utilities for applying drop-shadow filters to an element.
    ///
    /// <https://tailwindcss.com/docs/drop-shadow>
    DropShadow {
        /// ```css
        /// {
        ///     filter: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));
        /// }
        /// ```
        Sm => "drop-shadow-sm",
        /// ```css
        /// {
        ///     filter: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));
        /// }
        /// ```
        DropShadow => "drop-shadow",
        /// ```css
        /// {
        ///     filter: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));
        /// }
        /// ```
        Md => "drop-shadow-md",
        /// ```css
        /// {
        ///     filter: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));
        /// }
        /// ```
        Lg => "drop-shadow-lg",
        /// ```css
        /// {
        ///     filter: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));
        /// }
        /// ```
        Xl => "drop-shadow-xl",
        /// ```css
        /// {
        ///     filter: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));
        /// }
        /// ```
        _2xl => "drop-shadow-2xl",
        /// ```css
        /// {
        ///     filter: drop-shadow(0 0 #0000);
        /// }
        /// ```
        None => "drop-shadow-none",
    }
    /// Utilities for applying grayscale filters to an element.
    ///
    /// <https://tailwindcss.com/docs/grayscale>
    Grayscale {
        /// ```css
        /// {
        ///     filter: grayscale(0);
        /// }
        /// ```
        _0 => "grayscale-0",
        /// ```css
        /// {
        ///     filter: grayscale(100%);
        /// }
        /// ```
        Grayscale => "grayscale",
    }
    /// Utilities for applying hue-rotate filters to an element.
    ///
    /// <https://tailwindcss.com/docs/hue-rotate>
    HueRotate {
        /// ```css
        /// {
        ///     filter: hue-rotate(0deg);
        /// }
        /// ```
        _0 => "hue-rotate-0",
        /// ```css
        /// {
        ///     filter: hue-rotate(15deg);
        /// }
        /// ```
        _15 => "hue-rotate-15",
        /// ```css
        /// {
        ///     filter: hue-rotate(30deg);
        /// }
        /// ```
        _30 => "hue-rotate-30",
        /// ```css
        /// {
        ///     filter: hue-rotate(60deg);
        /// }
        /// ```
        _60 => "hue-rotate-60",
        /// ```css
        /// {
        ///     filter: hue-rotate(90deg);
        /// }
        /// ```
        _90 => "hue-rotate-90",
        /// ```css
        /// {
        ///     filter: hue-rotate(180deg);
        /// }
        /// ```
        _180 => "hue-rotate-180",
    }
    /// Utilities for applying invert filters to an element.
    ///
    /// <https://tailwindcss.com/docs/invert>
    Invert {
        /// ```css
        /// {
        ///     filter: invert(0);
        /// }
        /// ```
        _0 => "invert-0",
        /// ```css
        /// {
        ///     filter: invert(100%);
        /// }
        /// ```
        Invert => "invert",
    }
    /// Utilities for applying saturation filters to an element.
    ///
    /// <https://tailwindcss.com/docs/saturate>
    Saturate {
        /// ```css
        /// {
        ///     filter: saturate(0);
        /// }
        /// ```
        _0 => "saturate-0",
        /// ```css
        /// {
        ///     filter: saturate(.5);
        /// }
        /// ```
        _50 => "saturate-50",
        /// ```css
        /// {
        ///     filter: saturate(1);
        /// }
        /// ```
        _100 => "saturate-100",
        /// ```css
        /// {
        ///     filter: saturate(1.5);
        /// }
        /// ```
        _150 => "saturate-150",
        /// ```css
        /// {
        ///     filter: saturate(2);
        /// }
        /// ```
        _200 => "saturate-200",
    }
    /// Utilities for applying sepia filters to an element.
    ///
    /// <https://tailwindcss.com/docs/sepia>
    Sepia {
        /// ```css
        /// {
        ///     filter: sepia(0);
        /// }
        /// ```
        _0 => "sepia-0",
        /// ```css
        /// {
        ///     filter: sepia(100%);
        /// }
        /// ```
        Sepia => "sepia",
    }
    /// Utilities for applying backdrop blur filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-blur>
    BackdropBlur {
        /// ```css
        /// {
        ///     backdrop-filter:  ;
        /// }
        /// ```
        None => "backdrop-blur-none",
        /// ```css
        /// {
        ///     backdrop-filter: blur(4px);
        /// }
        /// ```
        Sm => "backdrop-blur-sm",
        /// ```css
        /// {
        ///     backdrop-filter: blur(8px);
        /// }
        /// ```
        BackdropBlur => "backdrop-blur",
        /// ```css
        /// {
        ///     backdrop-filter: blur(12px);
        /// }
        /// ```
        Md => "backdrop-blur-md",
        /// ```css
        /// {
        ///     backdrop-filter: blur(16px);
        /// }
        /// ```
        Lg => "backdrop-blur-lg",
        /// ```css
        /// {
        ///     backdrop-filter: blur(24px);
        /// }
        /// ```
        Xl => "backdrop-blur-xl",
        /// ```css
        /// {
        ///     backdrop-filter: blur(40px);
        /// }
        /// ```
        _2xl => "backdrop-blur-2xl",
        /// ```css
        /// {
        ///     backdrop-filter: blur(64px);
        /// }
        /// ```
        _3xl => "backdrop-blur-3xl",
    }
    /// Utilities for applying backdrop brightness filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-brightness>
    BackdropBrightness {
        /// ```css
        /// {
        ///     backdrop-filter: brightness(0);
        /// }
        /// ```
        _0 => "backdrop-brightness-0",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(.5);
        /// }
        /// ```
        _50 => "backdrop-brightness-50",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(.75);
        /// }
        /// ```
        _75 => "backdrop-brightness-75",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(.9);
        /// }
        /// ```
        _90 => "backdrop-brightness-90",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(.95);
        /// }
        /// ```
        _95 => "backdrop-brightness-95",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(1);
        /// }
        /// ```
        _100 => "backdrop-brightness-100",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(1.05);
        /// }
        /// ```
        _105 => "backdrop-brightness-105",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(1.1);
        /// }
        /// ```
        _110 => "backdrop-brightness-110",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(1.25);
        /// }
        /// ```
        _125 => "backdrop-brightness-125",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(1.5);
        /// }
        /// ```
        _150 => "backdrop-brightness-150",
        /// ```css
        /// {
        ///     backdrop-filter: brightness(2);
        /// }
        /// ```
        _200 => "backdrop-brightness-200",
    }
    /// Utilities for applying backdrop contrast filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-contrast>
    BackdropContrast {
        /// ```css
        /// {
        ///     backdrop-filter: contrast(0);
        /// }
        /// ```
        _0 => "backdrop-contrast-0",
        /// ```css
        /// {
        ///     backdrop-filter: contrast(.5);
        /// }
        /// ```
        _50 => "backdrop-contrast-50",
        /// ```css
        /// {
        ///     backdrop-filter: contrast(.75);
        /// }
        /// ```
        _75 => "backdrop-contrast-75",
        /// ```css
        /// {
        ///     backdrop-filter: contrast(1);
        /// }
        /// ```
        _100 => "backdrop-contrast-100",
        /// ```css
        /// {
        ///     backdrop-filter: contrast(1.25);
        /// }
        /// ```
        _125 => "backdrop-contrast-125",
        /// ```css
        /// {
        ///     backdrop-filter: contrast(1.5);
        /// }
        /// ```
        _150 => "backdrop-contrast-150",
        /// ```css
        /// {
        ///     backdrop-filter: contrast(2);
        /// }
        /// ```
        _200 => "backdrop-contrast-200",
    }
    /// Utilities for applying backdrop grayscale filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-grayscale>
    BackdropGrayscale {
        /// ```css
        /// {
        ///     backdrop-filter: grayscale(0);
        /// }
        /// ```
        _0 => "backdrop-grayscale-0",
        /// ```css
        /// {
        ///     backdrop-filter: grayscale(100%);
        /// }
        /// ```
        BackdropGrayscale => "backdrop-grayscale",
    }
    /// Utilities for applying backdrop hue-rotate filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-hue-rotate>
    BackdropHueRotate {
        /// ```css
        /// {
        ///     backdrop-filter: hue-rotate(0deg);
        /// }
        /// ```
        _0 => "backdrop-hue-rotate-0",
        /// ```css
        /// {
        ///     backdrop-filter: hue-rotate(15deg);
        /// }
        /// ```
        _15 => "backdrop-hue-rotate-15",
        /// ```css
        /// {
        ///     backdrop-filter: hue-rotate(30deg);
        /// }
        /// ```
        _30 => "backdrop-hue-rotate-30",
        /// ```css
        /// {
        ///     backdrop-filter: hue-rotate(60deg);
        /// }
        /// ```
        _60 => "backdrop-hue-rotate-60",
        /// ```css
        /// {
        ///     backdrop-filter: hue-rotate(90deg);
        /// }
        /// ```
        _90 => "backdrop-hue-rotate-90",
        /// ```css
        /// {
        ///     backdrop-filter: hue-rotate(180deg);
        /// }
        /// ```
        _180 => "backdrop-hue-rotate-180",
    }
    /// Utilities for applying backdrop invert filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-invert>
    BackdropInvert {
        /// ```css
        /// {
        ///     backdrop-filter: invert(0);
        /// }
        /// ```
        _0 => "backdrop-invert-0",
        /// ```css
        /// {
        ///     backdrop-filter: invert(100%);
        /// }
        /// ```
        BackdropInvert => "backdrop-invert",
    }
    /// Utilities for applying backdrop opacity filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-opacity>
    BackdropOpacity {
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0);
        /// }
        /// ```
        _0 => "backdrop-opacity-0",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.05);
        /// }
        /// ```
        _5 => "backdrop-opacity-5",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.1);
        /// }
        /// ```
        _10 => "backdrop-opacity-10",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.15);
        /// }
        /// ```
        _15 => "backdrop-opacity-15",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.2);
        /// }
        /// ```
        _20 => "backdrop-opacity-20",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.25);
        /// }
        /// ```
        _25 => "backdrop-opacity-25",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.3);
        /// }
        /// ```
        _30 => "backdrop-opacity-30",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.35);
        /// }
        /// ```
        _35 => "backdrop-opacity-35",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.4);
        /// }
        /// ```
        _40 => "backdrop-opacity-40",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.45);
        /// }
        /// ```
        _45 => "backdrop-opacity-45",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.5);
        /// }
        /// ```
        _50 => "backdrop-opacity-50",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.55);
        /// }
        /// ```
        _55 => "backdrop-opacity-55",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.6);
        /// }
        /// ```
        _60 => "backdrop-opacity-60",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.65);
        /// }
        /// ```
        _65 => "backdrop-opacity-65",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.7);
        /// }
        /// ```
        _70 => "backdrop-opacity-70",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.75);
        /// }
        /// ```
        _75 => "backdrop-opacity-75",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.8);
        /// }
        /// ```
        _80 => "backdrop-opacity-80",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.85);
        /// }
        /// ```
        _85 => "backdrop-opacity-85",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.9);
        /// }
        /// ```
        _90 => "backdrop-opacity-90",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(0.95);
        /// }
        /// ```
        _95 => "backdrop-opacity-95",
        /// ```css
        /// {
        ///     backdrop-filter: opacity(1);
        /// }
        /// ```
        _100 => "backdrop-opacity-100",
    }
    /// Utilities for applying backdrop saturation filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-saturate>
    BackdropSaturate {
        /// ```css
        /// {
        ///     backdrop-filter: saturate(0);
        /// }
        /// ```
        _0 => "backdrop-saturate-0",
        /// ```css
        /// {
        ///     backdrop-filter: saturate(.5);
        /// }
        /// ```
        _50 => "backdrop-saturate-50",
        /// ```css
        /// {
        ///     backdrop-filter: saturate(1);
        /// }
        /// ```
        _100 => "backdrop-saturate-100",
        /// ```css
        /// {
        ///     backdrop-filter: saturate(1.5);
        /// }
        /// ```
        _150 => "backdrop-saturate-150",
        /// ```css
        /// {
        ///     backdrop-filter: saturate(2);
        /// }
        /// ```
        _200 => "backdrop-saturate-200",
    }
    /// Utilities for applying backdrop sepia filters to an element.
    ///
    /// <https://tailwindcss.com/docs/backdrop-sepia>
    BackdropSepia {
        /// ```css
        /// {
        ///     backdrop-filter: sepia(0);
        /// }
        /// ```
        _0 => "backdrop-sepia-0",
        /// ```css
        /// {
        ///     backdrop-filter: sepia(100%);
        /// }
        /// ```
        BackdropSepia => "backdrop-sepia",
    }
);
