def_types!(
    % for enum in enums:
    /// ${enum.docs}
    ///
    /// <${enum.url}>
    ${enum.name} {
        % for enum_variant in enum.variants:
        /// ```css
        /// {
        % for line in enum_variant.docs.split('\n'):
        ///     ${line}
        % endfor
        /// }
        /// ```
        ${enum_variant.key} => "${enum_variant.value}",
        % endfor
    }
    % endfor
);
