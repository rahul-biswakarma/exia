# Theme Creation Guide for Exia Design System

## Overview

This guide explains how to create new themes for the Exia design system, specifically tailored for use with Large Language Models (LLMs) to generate consistent, comprehensive themes.

## Architecture

Exia uses a **headless component architecture** where:
- **Atomic components** are completely design-agnostic
- **Themes** are pure CSS that style components through class selectors
- **Theme providers** inject CSS variables globally

## Quick Start

1. **Create theme CSS file**: `assets/styles/themes/your-theme-name.css`
2. **Update theme context**: Add new theme struct in `src/contexts/theme.rs`
3. **Apply CSS classes**: Use theme-specific classes in your components

## LLM Prompt for Theme Creation

Use this comprehensive prompt with any LLM to generate new themes:

```
Create a complete CSS theme for the Exia design system based on [YOUR_THEME_DESCRIPTION].

REQUIREMENTS:
1. File should be named: assets/styles/themes/[theme-name].css
2. Include ALL component categories listed below
3. Use consistent color palette and design language
4. Include responsive design and accessibility features
5. Follow the exact CSS class structure provided

TARGET COMPONENTS:
- Buttons (primary, secondary, outline, ghost, terminal variants)
- Form Controls (toggle, switch, checkbox, radio, select, slider)
- Cards and Containers (terminal panels, decorated cards)
- Progress and Data Display (progress bars, status indicators, data rows)
- Navigation (tabs, menubar, dropdown menus, context menus)
- Overlays (dialogs, tooltips, hover cards, toasts)
- Layout (accordion, collapsible, separator, scroll area)
- Misc (avatar, label, calendar, system stats)

COLOR PALETTE STRUCTURE:
- Primary: [main brand color]
- Secondary: [secondary action color]
- Accent: [highlight/warning color]
- Background: [main background]
- Surface: [card/panel background]
- Text: [primary text color]
- Text Secondary: [muted text color]
- Border: [border color]
- Success: [success state color]
- Warning: [warning state color]
- Error: [error state color]
- Info: [info state color]

DESIGN PATTERNS TO INCLUDE:
- Hover states with color transitions
- Focus states for accessibility
- Active/pressed states
- Disabled states
- Loading states
- Glow/shadow effects (optional)
- Border animations (optional)
- Responsive breakpoints
- High contrast mode support
- Reduced motion support

EXAMPLE THEME AESTHETIC: [Describe your desired theme - e.g., "cyberpunk neon", "minimalist dark", "retro terminal", etc.]
```

## Component CSS Classes Reference

### Buttons
```css
.btn.primary { /* Main action button */ }
.btn.secondary { /* Secondary action */ }
.btn.outline { /* Outlined button */ }
.btn.ghost { /* Subtle button */ }
.btn.terminal-primary { /* Terminal style primary */ }
.btn.terminal-success { /* Terminal style success */ }
.btn.terminal-warning { /* Terminal style warning */ }
.btn.terminal-danger { /* Terminal style danger */ }
.btn.terminal-outline { /* Terminal style outline */ }
.btn.small { /* Small size variant */ }
.btn.large { /* Large size variant */ }
.btn.glow { /* Glowing effect */ }
```

### Cards
```css
.card.terminal { /* Terminal style card */ }
.card.terminal-green { /* Green border variant */ }
.card.terminal-yellow { /* Yellow border variant */ }
.card.terminal-cyan { /* Cyan border variant */ }
.card.decorated { /* Corner decorations */ }
.card.glow { /* Glowing effect */ }
.card.hoverable { /* Hover animations */ }
```

### Form Controls
```css
.toggle.terminal { /* Terminal style toggle */ }
.toggle.glow { /* Glowing toggle */ }
.switch { /* Switch component */ }
.switch[data-state="checked"] { /* Checked switch */ }
.switch-thumb { /* Switch thumb */ }
.checkbox { /* Checkbox input */ }
.checkbox[data-state="checked"] { /* Checked checkbox */ }
.radio-item { /* Radio button */ }
.radio-item[data-state="checked"] { /* Selected radio */ }
.radio-indicator { /* Radio indicator dot */ }
```

### Progress & Sliders
```css
.progress.terminal { /* Terminal style progress */ }
.progress.glow { /* Glowing progress */ }
.progress-bar { /* Progress bar fill */ }
.slider-track { /* Slider track */ }
.slider-range { /* Slider filled range */ }
.slider-thumb { /* Slider handle */ }
```

### Navigation
```css
.tabs-list { /* Tab container */ }
.tabs-trigger { /* Individual tab */ }
.tabs-trigger[data-state="active"] { /* Active tab */ }
.tabs-content { /* Tab content area */ }
.menubar { /* Menu bar container */ }
.menubar-trigger { /* Menu trigger */ }
.menubar-content { /* Menu dropdown */ }
.dropdown-content { /* Dropdown menu */ }
.dropdown-item { /* Menu item */ }
.context-menu-content { /* Context menu */ }
.context-menu-item { /* Context menu item */ }
```

### Overlays
```css
.dialog-overlay { /* Dialog backdrop */ }
.dialog-content { /* Dialog container */ }
.dialog-title { /* Dialog title */ }
.dialog-close { /* Close button */ }
.tooltip-content { /* Tooltip container */ }
.hover-card-content { /* Hover card */ }
.toast { /* Toast notification */ }
.toast.success { /* Success toast */ }
.toast.warning { /* Warning toast */ }
.toast.error { /* Error toast */ }
```

### Layout
```css
.accordion-item { /* Accordion section */ }
.accordion-trigger { /* Accordion header */ }
.accordion-content { /* Accordion body */ }
.collapsible-trigger { /* Collapsible trigger */ }
.separator { /* Divider line */ }
.scroll-area-scrollbar { /* Scrollbar track */ }
.scroll-area-thumb { /* Scrollbar thumb */ }
```

### Data Display
```css
.status-indicator { /* Status with colored dot */ }
.status-online { /* Online status color */ }
.status-warning { /* Warning status color */ }
.status-error { /* Error status color */ }
.status-info { /* Info status color */ }
.status-offline { /* Offline status color */ }
.data-row { /* Key-value row */ }
.data-label { /* Data label */ }
.data-value { /* Data value */ }
.data-value.highlight { /* Highlighted value */ }
.data-value.success { /* Success value */ }
.data-value.warning { /* Warning value */ }
.data-value.error { /* Error value */ }
.system-stats { /* Stats grid */ }
.stat-item { /* Individual stat */ }
.stat-value { /* Stat number */ }
.stat-label { /* Stat description */ }
```

### Special Components
```css
.terminal-grid { /* Terminal layout grid */ }
.terminal-panel { /* Terminal style panel */ }
.terminal-panel-header { /* Panel header */ }
.terminal-panel-content { /* Panel content */ }
.avatar { /* User avatar */ }
.avatar-fallback { /* Avatar placeholder */ }
.label { /* Form label */ }
.label.required { /* Required field label */ }
.select-trigger { /* Select dropdown trigger */ }
.select-content { /* Select dropdown */ }
.select-item { /* Select option */ }
```

## Required CSS Structure Template

Use this template as a starting point for any new theme:

```css
/* [Theme Name] - Complete Atomic Component Coverage */

/* ============================================================================
   THEME VARIABLES & BASE
   ============================================================================ */

:root {
  /* Define your color palette */
  --theme-primary: #yourcolor;
  --theme-secondary: #yourcolor;
  --theme-accent: #yourcolor;
  --theme-background: #yourcolor;
  --theme-surface: #yourcolor;
  --theme-text: #yourcolor;
  --theme-text-secondary: #yourcolor;
  --theme-border: #yourcolor;
  --theme-success: #yourcolor;
  --theme-warning: #yourcolor;
  --theme-error: #yourcolor;
  --theme-info: #yourcolor;
}

/* ============================================================================
   BUTTONS
   ============================================================================ */

/* Primary button styles */
.btn.primary { }
.btn.primary:hover { }
.btn.primary:focus { }
.btn.primary:disabled { }

/* Secondary button styles */
.btn.secondary { }
/* ... continue for all button variants ... */

/* ============================================================================
   FORM CONTROLS
   ============================================================================ */

/* Toggle/Switch styles */
.toggle { }
.toggle[data-state="on"] { }
/* ... continue for all form controls ... */

/* ============================================================================
   [Continue for all component categories]
   ============================================================================ */

/* ============================================================================
   ANIMATIONS
   ============================================================================ */

@keyframes yourAnimation {
  /* Define theme-specific animations */
}

/* ============================================================================
   RESPONSIVE DESIGN
   ============================================================================ */

@media (max-width: 768px) {
  /* Mobile styles */
}

/* ============================================================================
   ACCESSIBILITY
   ============================================================================ */

@media (prefers-reduced-motion: reduce) {
  /* Disable animations for users who prefer reduced motion */
}

@media (prefers-contrast: high) {
  /* High contrast adjustments */
}
```

## Theme Context Integration

After creating your CSS theme, update the theme context:

### 1. Add Theme Struct

```rust
// In src/contexts/theme.rs

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YourThemeName {
    pub name: String,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub typography: ThemeTypography,
    pub borders: ThemeBorders,
    pub effects: ThemeEffects,
    pub animations: ThemeAnimations,
    pub decorative: ThemeDecorative,
}

impl YourThemeName {
    pub fn new() -> Self {
        Self {
            name: "Your Theme Name".to_string(),
            colors: ThemeColors {
                primary: "#yourcolor".to_string(),
                secondary: "#yourcolor".to_string(),
                // ... define all colors
            },
            // ... define all other properties
        }
    }
}
```

### 2. Create Theme Provider

```rust
#[component]
pub fn YourThemeProvider(children: Element) -> Element {
    let theme = use_signal(|| YourThemeName::new());

    rsx! {
        style {
            ":root {{ {theme.read().to_css_variables()} }}"
        }
        div {
            class: "your-theme-name",
            "data-theme": "yourthemename",
            {children}
        }
    }
}
```

## Testing Your Theme

Create a comprehensive demo to test all components:

```rust
// In examples/your_theme_demo.rs

use dioxus::prelude::*;
use exia::components::atoms::*;
use exia::contexts::theme::YourThemeProvider;

#[component]
fn ThemeDemo() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "assets/styles/themes/your-theme.css" }

        YourThemeProvider {
            // Test all component variants
            Button { class: "primary", "Primary Button" }
            Button { class: "secondary", "Secondary Button" }
            Card { class: "decorated glow", "Theme Demo Card" }
            // ... test all components
        }
    }
}
```

## Best Practices

### Color Theory
- Use consistent color relationships (complementary, triadic, analogous)
- Ensure sufficient contrast ratios (WCAG 2.1 AA: 4.5:1 for normal text)
- Test colors in different lighting conditions

### Accessibility
- Always include focus states
- Support high contrast mode
- Respect reduced motion preferences
- Use semantic color meanings (red for error, green for success)

### Performance
- Use CSS custom properties for easy theming
- Minimize complex animations
- Optimize for different screen sizes

### Consistency
- Maintain consistent spacing scale
- Use harmonious border radius values
- Keep animation timing consistent

## Example Themes

### Cyberpunk Neon Theme
```css
:root {
  --theme-primary: #ff0080;      /* Hot pink */
  --theme-secondary: #00ffff;    /* Cyan */
  --theme-accent: #ffff00;       /* Neon yellow */
  --theme-background: #0a0a0a;   /* Deep black */
  --theme-surface: #1a1a2e;      /* Dark purple */
  --theme-text: #ffffff;         /* Pure white */
  --theme-border: #ff0080;       /* Hot pink borders */
}
```

### Minimalist Light Theme
```css
:root {
  --theme-primary: #2563eb;      /* Blue 600 */
  --theme-secondary: #64748b;    /* Slate 500 */
  --theme-accent: #f59e0b;       /* Amber 500 */
  --theme-background: #ffffff;   /* Pure white */
  --theme-surface: #f8fafc;      /* Slate 50 */
  --theme-text: #0f172a;         /* Slate 900 */
  --theme-border: #e2e8f0;       /* Slate 200 */
}
```

### Retro Terminal Theme
```css
:root {
  --theme-primary: #00ff00;      /* Matrix green */
  --theme-secondary: #ffff00;    /* Terminal yellow */
  --theme-accent: #ff8000;       /* Amber */
  --theme-background: #000000;   /* Black */
  --theme-surface: #001100;      /* Dark green tint */
  --theme-text: #00ff00;         /* Green text */
  --theme-border: #00ff00;       /* Green borders */
}
```

## LLM Theme Generation Checklist

When asking an LLM to generate a theme, ensure it includes:

- [ ] Complete color palette (12+ colors)
- [ ] All button variants (8+ styles)
- [ ] All form control states
- [ ] Navigation component styles
- [ ] Overlay/modal styles
- [ ] Data display components
- [ ] Animation keyframes
- [ ] Responsive breakpoints
- [ ] Accessibility features
- [ ] Consistent spacing/sizing
- [ ] Hover/focus/active states
- [ ] Disabled states
- [ ] Loading states

## Troubleshooting

### Common Issues

**Colors not applying:**
- Check CSS file is imported correctly
- Verify CSS selectors match component classes
- Ensure CSS specificity is correct

**Animations not working:**
- Check `prefers-reduced-motion` isn't disabling them
- Verify keyframe names are unique
- Test browser compatibility

**Components look broken:**
- Verify all required CSS classes are styled
- Check for typos in class names
- Test responsive breakpoints

### Debug Mode

Add this to any theme for debugging:

```css
/* DEBUG MODE - Remove in production */
[data-theme="yourtheme"] * {
  outline: 1px solid rgba(255, 0, 0, 0.1) !important;
}
```

## Contributing Themes

To contribute a new theme to Exia:

1. Create theme following this guide
2. Add comprehensive tests
3. Include documentation and screenshots
4. Submit PR with theme files and integration

## Resources

- [WCAG Contrast Checker](https://webaim.org/resources/contrastchecker/)
- [Color Palette Generators](https://coolors.co/)
- [CSS Animation Resources](https://animista.net/)
- [Accessibility Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)

---

This guide provides everything needed to create comprehensive, accessible themes for the Exia design system using LLM assistance. Each theme should be a complete design language that covers all atomic components with consistent styling.
