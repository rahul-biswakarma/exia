# Exia Design System - Quick Reference

## Component Usage Patterns

### Basic Setup

```rust
use dioxus::prelude::*;
use exia::components::atoms::*;
use exia::contexts::theme::NeonThemeProvider;

#[component]
fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "assets/styles/themes/neon-evangelion.css" }
        NeonThemeProvider {
            // Your components here
        }
    }
}
```

## Component Quick Reference

### Buttons

```rust
// Basic buttons
Button { class: "primary", "Primary Action" }
Button { class: "secondary", "Secondary Action" }
Button { class: "outline", "Outline Button" }
Button { class: "ghost", "Ghost Button" }

// Terminal theme variants
Button { class: "terminal-primary", "Neural Interface" }
Button { class: "terminal-success", "System Online" }
Button { class: "terminal-warning", "Caution" }
Button { class: "terminal-danger", "Emergency" }
Button { class: "terminal-outline", "Terminal Mode" }

// With effects
Button { class: "primary glow", "Glowing Button" }
Button { class: "secondary small", "Small Button" }
Button { class: "outline large", "Large Button" }

// With state
let disabled = use_signal(|| false);
let loading = use_signal(|| false);
Button {
    class: "primary",
    disabled: disabled.into(),
    loading: loading.into(),
    onclick: move |_| println!("Clicked!"),
    "Interactive Button"
}
```

### Cards

```rust
// Basic cards
Card { class: "terminal", "Simple terminal card" }
Card { class: "decorated", "Card with corner decorations" }
Card { class: "glow hoverable", "Interactive glowing card" }

// Color variants
Card { class: "terminal-green", "Success themed card" }
Card { class: "terminal-yellow", "Warning themed card" }
Card { class: "terminal-cyan", "Info themed card" }
```

### Form Controls

```rust
// Toggle
let toggled = use_signal(|| false);
Toggle {
    class: "terminal glow",
    pressed: Some(toggled),
}

// Progress
Progress {
    class: "terminal",
    value: 75.0,
    max: 100.0,
}
```

### Terminal Components

```rust
// Terminal Panel
TerminalPanel {
    title: "System Status",
    color_theme: Some("orange".to_string()),

    // Panel content
    DataRow { label: "CPU", value: "23%", value_type: Some("success".to_string()) }
    DataRow { label: "Memory", value: "67%", value_type: Some("warning".to_string()) }
    DataRow { label: "Disk", value: "89%", value_type: Some("error".to_string()) }
}

// Terminal Grid Layout
TerminalGrid {
    columns: "1fr 1fr 1fr".to_string(),
    gap: "1rem".to_string(),

    TerminalPanel { title: "Panel 1", "Content 1" }
    TerminalPanel { title: "Panel 2", "Content 2" }
    TerminalPanel { title: "Panel 3", "Content 3" }
}

// Status Indicators
StatusIndicator { status: "Online", status_type: "online" }
StatusIndicator { status: "Warning", status_type: "warning" }
StatusIndicator { status: "Error", status_type: "error" }
StatusIndicator { status: "Offline", status_type: "offline" }
StatusIndicator { status: "Info", status_type: "info" }
```

## CSS Class Combinations

### Button Classes

| Base | Variants | Effects | Sizes |
|------|----------|---------|-------|
| `btn` | `primary` | `glow` | `small` |
|       | `secondary` |        | `large` |
|       | `outline` |          |         |
|       | `ghost` |            |         |
|       | `terminal-primary` | |         |
|       | `terminal-success` | |         |
|       | `terminal-warning` | |         |
|       | `terminal-danger` |  |         |
|       | `terminal-outline` | |         |

### Card Classes

| Base | Themes | Effects | Interactions |
|------|--------|---------|--------------|
| `card` | `terminal` | `glow` | `hoverable` |
|        | `terminal-green` | `decorated` | |
|        | `terminal-yellow` | | |
|        | `terminal-cyan` | | |

### Data Value Classes

| Base | Types |
|------|-------|
| `data-value` | `highlight` |
|              | `success` |
|              | `warning` |
|              | `error` |

## Common Patterns

### Dashboard Layout

```rust
TerminalGrid {
    columns: "300px 1fr".to_string(),

    // Sidebar
    div {
        TerminalPanel { title: "Navigation",
            Button { class: "terminal-outline", style: "width: 100%; margin-bottom: 0.5rem;", "Dashboard" }
            Button { class: "terminal-outline", style: "width: 100%; margin-bottom: 0.5rem;", "Analytics" }
            Button { class: "terminal-outline", style: "width: 100%; margin-bottom: 0.5rem;", "Settings" }
        }
    }

    // Main content
    div {
        TerminalGrid {
            columns: "1fr 1fr".to_string(),

            TerminalPanel { title: "System Status",
                StatusIndicator { status: "All Systems Operational", status_type: "online" }
                Progress { class: "terminal", value: 85.0, max: 100.0 }
            }

            TerminalPanel { title: "Performance",
                DataRow { label: "CPU", value: "23%", value_type: Some("success".to_string()) }
                DataRow { label: "Memory", value: "67%", value_type: Some("warning".to_string()) }
                DataRow { label: "Network", value: "12 Mbps", value_type: Some("highlight".to_string()) }
            }
        }
    }
}
```

### Form Layout

```rust
Card { class: "terminal decorated",
    h2 { style: "color: #ff6b35; margin-bottom: 1rem;", "System Configuration" }

    div { style: "display: flex; flex-direction: column; gap: 1rem;",
        div { style: "display: flex; justify-content: space-between; align-items: center;",
            span { "Enable Neural Interface" }
            Toggle { class: "terminal glow", pressed: Some(neural_enabled) }
        }

        div { style: "display: flex; justify-content: space-between; align-items: center;",
            span { "Learning Mode" }
            Toggle { class: "terminal", pressed: Some(learning_mode) }
        }

        div { style: "margin-top: 1rem;",
            Button { class: "terminal-primary", style: "margin-right: 0.5rem;", "Apply Changes" }
            Button { class: "terminal-outline", "Reset to Default" }
        }
    }
}
```

### Status Dashboard

```rust
TerminalGrid {
    columns: "repeat(auto-fit, minmax(200px, 1fr))".to_string(),

    TerminalPanel { title: "Network Status", color_theme: Some("green".to_string()),
        StatusIndicator { status: "Connected", status_type: "online" }
        DataRow { label: "Latency", value: "23ms", value_type: Some("success".to_string()) }
        DataRow { label: "Bandwidth", value: "100 Mbps", value_type: Some("highlight".to_string()) }
    }

    TerminalPanel { title: "System Health", color_theme: Some("orange".to_string()),
        StatusIndicator { status: "Optimal", status_type: "online" }
        Progress { class: "terminal", value: 92.0, max: 100.0 }
    }

    TerminalPanel { title: "Security", color_theme: Some("cyan".to_string()),
        StatusIndicator { status: "Secure", status_type: "online" }
        DataRow { label: "Threat Level", value: "Low", value_type: Some("success".to_string()) }
    }
}
```

## Color Reference

### Neon Evangelion Theme

| Color | Hex | Usage |
|-------|-----|-------|
| Primary | `#ff6b35` | Terminal orange, main actions |
| Secondary | `#00ff66` | Success states, positive feedback |
| Accent | `#ffcc00` | Warnings, highlights |
| Background | `#0a0a0f` | Main background |
| Surface | `#1a1a2e` | Card backgrounds |
| Text | `#ffffff` | Primary text |
| Text Secondary | `#cccccc` | Muted text |
| Border | `#ff6b35` | Borders, dividers |
| Success | `#00ff66` | Success states |
| Warning | `#ffcc00` | Warning states |
| Error | `#ff3366` | Error states |
| Info | `#00ffcc` | Info states |

## Animation Classes

| Class | Effect |
|-------|--------|
| `glow` | Pulsing glow effect |
| `hoverable` | Hover lift animation |
| `terminal` | Scanning line animations |

## Responsive Breakpoints

| Breakpoint | Screen Size | Notes |
|------------|-------------|-------|
| Mobile | `< 768px` | Reduced padding, smaller fonts |
| Tablet | `768px - 1024px` | Adjusted grid layouts |
| Desktop | `> 1024px` | Full feature set |

## Accessibility

### Focus States
All interactive components automatically include focus states for keyboard navigation.

### Color Contrast
All color combinations meet WCAG 2.1 AA standards (4.5:1 contrast ratio).

### Motion Preferences
Animations are disabled when `prefers-reduced-motion: reduce` is set.

### High Contrast
Additional styles are applied for `prefers-contrast: high`.

## Tips & Best Practices

1. **Always include theme CSS**: Don't forget to import your theme CSS file
2. **Use semantic color classes**: `success`, `warning`, `error`, `info` for meaningful colors
3. **Combine classes thoughtfully**: `terminal glow` works well, `primary secondary` doesn't
4. **Test responsive behavior**: Check your layouts on different screen sizes
5. **Consider accessibility**: Test with keyboard navigation and screen readers

## Troubleshooting

**Components look unstyled:**
- Check that theme CSS is imported
- Verify CSS file path is correct
- Ensure you're using correct class names

**Animations not working:**
- Check browser supports CSS animations
- Verify `prefers-reduced-motion` isn't disabling them
- Test in different browsers

**Colors look wrong:**
- Confirm you're using the right theme CSS file
- Check for CSS specificity conflicts
- Verify color class names are correct
