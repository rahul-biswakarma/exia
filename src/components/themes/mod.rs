// Declare modules
pub mod context;
pub mod wrappers;

// Re-export from context
pub use context::{
    use_theme, Theme, ThemeAnimationEffects, ThemeAnimations, ThemeBorderEffects, ThemeBorders,
    ThemeBoxShadows, ThemeColors, ThemeComponentEffects, ThemeDecorative, ThemeEffects,
    ThemeFontEffects, ThemeFonts, ThemeGeneralEffects, ThemeGlassEffects, ThemeGlowEffects,
    ThemeGradientEffects, ThemeGridEffects, ThemeHoverEffects, ThemeLayout, ThemeLightingEffects,
    ThemeMotionEffects, ThemeNeonEvangelion, ThemeNoiseEffects, ThemeOtherEffects,
    ThemePatternEffects, ThemeScrollbarEffects, ThemeShadowEffects, ThemeSpacing, ThemeTypography,
    ThemeVariant, ThemeProvider, ThemeSwitcher, ComponentTexts, LoaderStyles,
    NeonEvangelionTheme, // This is a type alias for Theme
    use_neon_theme, // This is a function alias for use_theme
    NeonThemeProvider, // This is a component alias for ThemeProvider
    CURRENT_THEME, // This is a GlobalSignal<Theme>
    switch_theme, // This is a function
};

// Re-export from wrappers
pub use wrappers::{
    ThemedButton, ThemedButtonProps,
    ThemedCard, ThemedCardProps,
    ThemedLoader, ThemedLoaderProps, LoaderContext,
    ThemedPageLoader, ThemedPageLoaderProps,
    ThemedInlineLoader, ThemedInlineLoaderProps,
};
