# EXIA Neon Evangelion Theme

A futuristic, cyberpunk-inspired design system for Dioxus applications, featuring neon colors, glowing effects, and animated decorative elements.

## Theme Features

### Color Palette
- **Primary**: Electric Red (`#ff3366`) - Main action color with intense neon glow
- **Secondary**: Neon Cyan (`#00ffcc`) - Complementary accent for secondary actions
- **Accent**: Electric Yellow (`#ffcc00`) - Highlight color for important elements
- **Background**: Deep Void (`#0a0a0f`) - Dark background for maximum contrast
- **Surface**: Dark Purple (`#1a1a2e`) - Card and component backgrounds

### Typography
- **Font Family**: JetBrains Mono, Fira Code (monospace for that terminal feel)
- **Text Colors**: Pure white primary, light gray secondary
- **Font Weights**: 400 (normal), 500 (medium), 700 (bold)

### Decorative Effects
- ✨ **Scan Lines**: Animated horizontal lines that move across the screen
- 🌟 **Noise Overlay**: Subtle texture that flickers for visual depth
- 💫 **Floating Particles**: Glowing dots that drift across the interface
- ⚡ **Neon Glow**: CSS box-shadow based glow effects on interactive elements
- 📺 **Terminal Cursor**: Blinking underscore for text elements

## Components

### Button
```rust
Button {
    variant: ButtonVariant::Primary,
    with_glow: true,
    "Activate System"
}
```

**Variants:**
- `Primary`: Gradient red background with intense glow
- `Secondary`: Gradient cyan background with neon effects
- `Outline`: Transparent with animated fill on hover
- `Ghost`: Subtle background with glow on hover

### Card
```rust
Card {
    with_decorations: true,
    with_glow: true,

    CardHeader { h3 { "System Status" } }
    CardContent { /* content */ }
    CardFooter { /* actions */ }
}
```

**Features:**
- Corner decorations (L-shaped borders)
- Glow effects with hover animations
- Backdrop blur for depth

### Toggle
```rust
Toggle {
    checked: state(),
    with_glow: true,
    onchange: move |value| set_state(value),
}
```

### Progress Bar
```rust
Progress {
    value: 75.0,
    max: 100.0,
}
```

**Features:**
- Gradient fill from primary to accent
- Animated shine effect
- Neon glow shadow

## Usage

### Basic Setup
```rust
use exia::contexts::theme::{NeonThemeProvider, use_neon_theme};

#[component]
fn App() -> Element {
    rsx! {
        // Include CSS
        link { rel: "stylesheet", href: "assets/styles/fonts.css" }
        link { rel: "stylesheet", href: "assets/styles/design-system.css" }

        // Wrap with theme provider
        NeonThemeProvider {
            div { class: "floating-particles" }
            YourAppContent {}
        }
    }
}
```

### Accessing Theme Data
```rust
#[component]
fn MyComponent() -> Element {
    let theme = use_neon_theme();

    rsx! {
        div {
            style: "color: {theme.colors.primary}",
            "This text is neon red!"
        }
    }
}
```

## CSS Classes

### Utility Classes
- `.glow` - Adds pulsing glow animation
- `.text-glow` - Adds text shadow glow
- `.neon-border` - Neon border with glow effect
- `.terminal-cursor` - Blinking cursor effect

### Component Classes
- `.btn`, `.btn-primary`, `.btn-secondary`, etc.
- `.card`, `.card-decorated`, `.card-glow`
- `.input` - Styled input fields
- `.toggle` - Toggle switch styling
- `.progress`, `.progress-bar` - Progress indicators

## Accessibility

The theme includes:
- Respects `prefers-reduced-motion` for animations
- High contrast mode support
- Keyboard navigation focus styles
- ARIA-friendly component structure

## Browser Compatibility

- Modern browsers with CSS Grid support
- CSS custom properties (CSS variables)
- CSS backdrop-filter support for blur effects

## Performance

- Hardware-accelerated animations using `transform` and `opacity`
- Efficient CSS selectors
- Minimal JavaScript for theme switching
- Optimized asset loading

## Example

Run the demo:
```bash
cargo run --example neon_theme_demo
```

This showcases all components with interactive examples of the Neon Evangelion theme in action.
