def_types!(
    /// Utilities for controlling which CSS properties transition.
    ///
    /// <https://tailwindcss.com/docs/transition-property>
    TransitionProperty {
        /// ```css
        /// {
        ///     transition-property: none;
        /// }
        /// ```
        None => "transition-none",
        /// ```css
        /// {
        ///     transition-property: all;
        ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        ///     transition-duration: 150ms;
        /// }
        /// ```
        All => "transition-all",
        /// ```css
        /// {
        ///     transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
        ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        ///     transition-duration: 150ms;
        /// }
        /// ```
        Transition => "transition",
        /// ```css
        /// {
        ///     transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
        ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        ///     transition-duration: 150ms;
        /// }
        /// ```
        Colors => "transition-colors",
        /// ```css
        /// {
        ///     transition-property: opacity;
        ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        ///     transition-duration: 150ms;
        /// }
        /// ```
        Opacity => "transition-opacity",
        /// ```css
        /// {
        ///     transition-property: box-shadow;
        ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        ///     transition-duration: 150ms;
        /// }
        /// ```
        Shadow => "transition-shadow",
        /// ```css
        /// {
        ///     transition-property: transform;
        ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        ///     transition-duration: 150ms;
        /// }
        /// ```
        Transform => "transition-transform",
    }
    /// Utilities for controlling the duration of CSS transitions.
    ///
    /// <https://tailwindcss.com/docs/transition-duration>
    TransitionDuration {
        /// ```css
        /// {
        ///     transition-duration: 0s;
        /// }
        /// ```
        _0 => "duration-0",
        /// ```css
        /// {
        ///     transition-duration: 75ms;
        /// }
        /// ```
        _75 => "duration-75",
        /// ```css
        /// {
        ///     transition-duration: 100ms;
        /// }
        /// ```
        _100 => "duration-100",
        /// ```css
        /// {
        ///     transition-duration: 150ms;
        /// }
        /// ```
        _150 => "duration-150",
        /// ```css
        /// {
        ///     transition-duration: 200ms;
        /// }
        /// ```
        _200 => "duration-200",
        /// ```css
        /// {
        ///     transition-duration: 300ms;
        /// }
        /// ```
        _300 => "duration-300",
        /// ```css
        /// {
        ///     transition-duration: 500ms;
        /// }
        /// ```
        _500 => "duration-500",
        /// ```css
        /// {
        ///     transition-duration: 700ms;
        /// }
        /// ```
        _700 => "duration-700",
        /// ```css
        /// {
        ///     transition-duration: 1000ms;
        /// }
        /// ```
        _1000 => "duration-1000",
    }
    /// Utilities for controlling the easing of CSS transitions.
    ///
    /// <https://tailwindcss.com/docs/transition-timing-function>
    TransitionTimingFunction {
        /// ```css
        /// {
        ///     transition-timing-function: linear;
        /// }
        /// ```
        Linear => "ease-linear",
        /// ```css
        /// {
        ///     transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
        /// }
        /// ```
        In => "ease-in",
        /// ```css
        /// {
        ///     transition-timing-function: cubic-bezier(0, 0, 0.2, 1);
        /// }
        /// ```
        Out => "ease-out",
        /// ```css
        /// {
        ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        /// }
        /// ```
        InOut => "ease-in-out",
    }
    /// Utilities for controlling the delay of CSS transitions.
    ///
    /// <https://tailwindcss.com/docs/transition-delay>
    TransitionDelay {
        /// ```css
        /// {
        ///     transition-delay: 0s;
        /// }
        /// ```
        _0 => "delay-0",
        /// ```css
        /// {
        ///     transition-delay: 75ms;
        /// }
        /// ```
        _75 => "delay-75",
        /// ```css
        /// {
        ///     transition-delay: 100ms;
        /// }
        /// ```
        _100 => "delay-100",
        /// ```css
        /// {
        ///     transition-delay: 150ms;
        /// }
        /// ```
        _150 => "delay-150",
        /// ```css
        /// {
        ///     transition-delay: 200ms;
        /// }
        /// ```
        _200 => "delay-200",
        /// ```css
        /// {
        ///     transition-delay: 300ms;
        /// }
        /// ```
        _300 => "delay-300",
        /// ```css
        /// {
        ///     transition-delay: 500ms;
        /// }
        /// ```
        _500 => "delay-500",
        /// ```css
        /// {
        ///     transition-delay: 700ms;
        /// }
        /// ```
        _700 => "delay-700",
        /// ```css
        /// {
        ///     transition-delay: 1000ms;
        /// }
        /// ```
        _1000 => "delay-1000",
    }
    /// Utilities for animating elements with CSS animations.
    ///
    /// <https://tailwindcss.com/docs/animation>
    Animation {
        /// ```css
        /// {
        ///     animation: none;
        /// }
        /// ```
        None => "animate-none",
        /// ```css
        /// {
        ///     animation: spin 1s linear infinite;
        ///     
        ///     @keyframes spin {
        ///       from {
        ///         transform: rotate(0deg);
        ///       }
        ///       to {
        ///         transform: rotate(360deg);
        ///       }
        ///     }
        /// }
        /// ```
        Spin => "animate-spin",
        /// ```css
        /// {
        ///     animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
        ///     
        ///     @keyframes ping {
        ///       75%, 100% {
        ///         transform: scale(2);
        ///         opacity: 0;
        ///       }
        ///     }
        /// }
        /// ```
        Ping => "animate-ping",
        /// ```css
        /// {
        ///     animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
        ///     
        ///     @keyframes pulse {
        ///       0%, 100% {
        ///         opacity: 1;
        ///       }
        ///       50% {
        ///         opacity: .5;
        ///       }
        ///     }
        /// }
        /// ```
        Pulse => "animate-pulse",
        /// ```css
        /// {
        ///     animation: bounce 1s infinite;
        ///     
        ///     @keyframes bounce {
        ///       0%, 100% {
        ///         transform: translateY(-25%);
        ///         animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
        ///       }
        ///       50% {
        ///         transform: translateY(0);
        ///         animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
        ///       }
        ///     }
        /// }
        /// ```
        Bounce => "animate-bounce",
    }
);
