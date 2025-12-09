/// Configuration for SVG outputs.
///
/// Used by [`Mana::as_svg`][crate::Mana::as_svg]
/// and [`Manas::as_svg`][crate::Manas::as_svg].
///
/// For default options, use [`SVGConfig::default`].

#[derive(Debug, Clone)]
pub struct SVGConfig {
    /// Whether to draw a circular shadow.
    pub shadow: bool,

    /// How large should the shadow be offset from the main circle.
    /// Even if the shadow is not drawn, this will affect the size of the margin
    /// around the main circle.
    pub shadow_offset: f64,
}

impl Default for SVGConfig {
    fn default() -> Self {
        Self { shadow: true, shadow_offset: 1.5 }
    }
}
