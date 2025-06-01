// Declare modules
pub mod context;
pub mod wrappers;

// Re-export from context
pub use context::{
    switch_theme,   // This is a function
    use_neon_theme, // This is a function alias for use_theme
    use_theme,
    ComponentTexts,
    LoaderStyles,
    NeonEvangelionTheme, // This is a type alias for Theme
    NeonThemeProvider,   // This is a component alias for ThemeProvider
    Theme,
    ThemeColors,
    ThemeDecorative,
    ThemeEffects,
    ThemeProvider,
    ThemeSpacing,
    ThemeSwitcher,
    ThemeTypography,
    ThemeVariant,
    CURRENT_THEME, // This is a GlobalSignal<Theme>
};

// Re-export from wrappers
pub use wrappers::{
    LoaderContext, ThemedButton, ThemedButtonProps, ThemedCard, ThemedCardProps,
    ThemedInlineLoader, ThemedInlineLoaderProps, ThemedLoader, ThemedLoaderProps, ThemedPageLoader,
    ThemedPageLoaderProps,
};
