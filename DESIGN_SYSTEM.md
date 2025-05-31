# EXIA Design System

A powerful, theme-driven design system for Dioxus applications featuring advanced decorative elements and multiple aesthetic themes.

## Features

- 🎨 **Multiple Themes**: Neon Evangelion, Eva/Gundam, Modern Minimal
- ✨ **Advanced Decoratives**: CSS pseudo-elements for scan lines, particles, glows
- 🎛️ **Context-Based**: Easy theme switching with React-like context
- 🧩 **Modular Components**: Reusable atomic components
- 🎯 **Accessibility**: WCAG compliant with reduced motion support
- 📱 **Responsive**: Mobile-first design principles

## Quick Start

### 1. Add CSS Files

Include the design system CSS in your app:

```rust
rsx! {
    link { rel: "stylesheet", href: "assets/styles/fonts.css" }
    link { rel: "stylesheet", href: "assets/styles/design-system.css" }

    ThemeProvider {
        // Your app content
    }
}
```

### 2. Wrap with ThemeProvider

```rust
use exia::contexts::theme::ThemeProvider;

#[component]
fn App() -> Element {
    rsx! {
        ThemeProvider {
            // Your app components
        }
    }
}
```

### 3. Use Components

```rust
use exia::components::atoms::button::{Button, ButtonVariant};
use exia::components::atoms::card::{Card, CardHeader, CardContent};

rsx! {
    Card {
        with_decorations: true,
        with_glow: true,

        CardHeader {
            h3 { "Welcome" }
        }

        CardContent {
            Button {
                variant: ButtonVariant::Primary,
                "Click me!"
            }
        }
    }
}
```

## Themes

### Neon Evangelion
- **Aesthetic**: Cyberpunk, neon colors
- **Features**: Scan lines, particle effects, neon glows
- **Colors**: Electric red (#ff3366), cyan (#00ffcc), yellow (#ffcc00)
- **Font**: JetBrains Mono

### Eva/Gundam
- **Aesthetic**: Mecha-inspired, angular design
- **Features**: Corner decorations, hexagonal elements
- **Colors**: Orange-red (#ff6b35), deep blue (#004e89), yellow (#ffd23f)
- **Font**: Orbitron

### Modern Minimal
- **Aesthetic**: Clean, accessible, professional
- **Features**: Subtle shadows, smooth animations
- **Colors**: Blue (#2563eb), slate (#64748b), purple (#8b5cf6)
- **Font**: Inter

## Components

### Button

```rust
Button {
    variant: ButtonVariant::Primary, // Primary | Secondary | Outline | Ghost
    size: ButtonSize::Medium,        // Small | Medium | Large
    with_glow: true,                 // Enables glow effect
    loading: loading_state,          // Shows theme-specific loader
    disabled: false,
    onclick: |_| { /* handler */ },
    "Button Text"
}
```

### Card

```rust
Card {
    with_decorations: true,  // Enables theme decorations
    with_glow: false,        // Adds glow effect
    hoverable: true,         // Lift on hover

    CardHeader {
        h3 { "Card Title" }
    }

    CardContent {
        p { "Card content goes here" }
    }

    CardFooter {
        Button { "Action" }
    }
}
```

### Toggle

```rust
let mut toggle_state = use_signal(|| false);

Toggle {
    pressed: Some(toggle_state),
    on_pressed_change: move |pressed| toggle_state.set(pressed),
}
```

### Progress

```rust
Progress {
    value: 65.0,
    max: 100.0,
    with_glow: true,
}
```

## Theme Switching

### Using Theme Context

```rust
use exia::contexts::theme::{use_theme, ThemeVariant};

#[component]
fn ThemeControls() -> Element {
    let theme_context = use_theme();

    rsx! {
        Button {
            onclick: |_| theme_context.switch_theme(ThemeVariant::NeonEvangelion),
            "Neon Theme"
        }

        Button {
            onclick: |_| theme_context.switch_theme(ThemeVariant::EvaGundam),
            "Eva Theme"
        }

        Button {
            onclick: |_| theme_context.switch_theme(ThemeVariant::ModernMinimal),
            "Minimal Theme"
        }
    }
}
```

### Theme Switcher Component

```rust
use exia::contexts::theme::ThemeSwitcher;

rsx! {
    ThemeSwitcher {}  // Automatic dropdown for theme selection
}
```

## Decorative Features

Each theme has specific decorative elements that can be toggled:

### Neon Evangelion
- **Scan Lines**: Animated horizontal lines across the screen
- **Noise Overlay**: Subtle texture for authenticity
- **Floating Particles**: Animated radial gradients in cards
- **Glow Effects**: Neon-style box shadows

### Eva/Gundam
- **Corner Decorations**: Angular borders on cards
- **Hexagonal Elements**: Geometric button backgrounds
- **Mechanical Aesthetics**: Industrial-style borders

### Modern Minimal
- **Clean Design**: No decorative elements by default
- **Subtle Shadows**: Professional depth
- **Smooth Animations**: Refined interactions

## CSS Variables

The system uses CSS custom properties for theming:

```css
:root {
  --color-primary: #2563eb;
  --color-secondary: #64748b;
  --color-accent: #8b5cf6;
  --color-background: #ffffff;
  --color-surface: #f8fafc;
  --color-text: #1e293b;
  --spacing-md: 1rem;
  --border-radius-md: 0.5rem;
  /* ... many more */
}
```

## Utility Classes

### Glow Effects
```css
.glow { box-shadow: var(--glow-primary); }
.glow-accent { box-shadow: var(--glow-accent); }
```

### Size Variants
```css
.btn-sm { /* Small button */ }
.btn-lg { /* Large button */ }
```

### Disable Decorations
```css
.no-decorations * { position: static !important; }
.no-decorations *::before,
.no-decorations *::after { display: none !important; }
```

## Accessibility

- **Reduced Motion**: Respects `prefers-reduced-motion`
- **High Contrast**: Adapts to `prefers-contrast: high`
- **Screen Readers**: Proper ARIA attributes
- **Keyboard Navigation**: Full keyboard support

## Creating Custom Themes

```rust
use exia::contexts::theme::*;

let custom_theme = Theme {
    variant: ThemeVariant::Custom,
    name: "Cyberpunk".to_string(),
    colors: ThemeColors {
        primary: "#ff0080".to_string(),
        secondary: "#00ff80".to_string(),
        // ... more colors
    },
    decorative: ThemeDecorative {
        corner_decorations: true,
        glow_effects: true,
        scan_lines: true,
        // ... more features
    },
    // ... rest of theme
};
```

## Advanced Usage

### Theme-Specific Components

```rust
#[component]
fn NeonCard(children: Element) -> Element {
    let theme = use_theme();
    let is_neon = matches!(theme.current_theme.read().variant, ThemeVariant::NeonEvangelion);

    rsx! {
        Card {
            with_decorations: is_neon,
            with_glow: is_neon,
            {children}
        }
    }
}
```

### Conditional Decorations

```rust
#[component]
fn AdaptiveButton() -> Element {
    let theme = use_theme();
    let current = theme.current_theme.read();

    rsx! {
        Button {
            with_glow: current.decorative.glow_effects,
            variant: match current.variant {
                ThemeVariant::NeonEvangelion => ButtonVariant::Primary,
                ThemeVariant::EvaGundam => ButtonVariant::Secondary,
                _ => ButtonVariant::Outline,
            },
            "Adaptive Button"
        }
    }
}
```

## Performance

- **CSS Variables**: Instant theme switching
- **Pseudo Elements**: No DOM manipulation for decorations
- **Efficient Animations**: Hardware-accelerated transforms
- **Lazy Loading**: Components only render needed features

## Browser Support

- **Modern Browsers**: Full feature support
- **Fallbacks**: Graceful degradation for older browsers
- **CSS Grid/Flexbox**: Modern layout techniques

## Examples

See `examples/design_system_usage.rs` for complete integration examples.

## Contributing

1. Add new themes to `src/contexts/theme.rs`
2. Create new components in `src/components/atoms/`
3. Add corresponding CSS in `assets/styles/design-system.css`
4. Update documentation

## License

This design system is part of the EXIA project.
