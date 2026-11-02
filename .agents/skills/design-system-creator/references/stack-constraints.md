# Stack-Specific Design Constraints Reference

When establishing `DESIGN.md`, the `design-system-creator` skill adapts implementation rules based on the project's technology stack.

---

## Stack Patterns

### 1. Next.js + Tailwind CSS + shadcn/ui
- **Tokens**: Map directly to `globals.css` CSS variables (`--background`, `--foreground`, `--primary`, etc.) and `@theme` / `tailwind.config.js`.
- **Primitives**: Leverage shadcn components located in `@/components/ui`.
- **Icons**: Standardize on `lucide-react`.
- **Rule**: Do not create custom button or card components if a shadcn primitive already exists. Extend via cva (class-variance-authority) variants.

### 2. React Native / Expo
- **Tokens**: Use JavaScript theme objects or `StyleSheet` constants (or NativeWind tokens).
- **Layout Constraints**: Flexbox column defaults, no web-specific CSS grid.
- **Touch & Accessibility**: Minimum 44x44pt touch targets, handle safe areas (`react-native-safe-area-context`).
- **Typography**: Native font families or custom fonts loaded via `expo-font`.

### 3. Shopify Liquid & Polaris
- **Tokens**: Merchant-configurable theme settings (`settings_schema.json`) mapped to CSS custom properties.
- **Component Architecture**: Liquid snippets and sections (`sections/`, `snippets/`).
- **Styling**: Shopify Polaris CSS tokens or BEM-styled Vanilla CSS.
- **Rule**: Respect Shopify store theme editor constraints and merchant customizable color controls.

### 4. Vanilla HTML / Javascript / CSS
- **Tokens**: Standard CSS Custom Properties (`:root { --color-primary: ... }`) in `index.css` or `styles.css`.
- **Layout**: Native CSS Grid and Flexbox layouts.
- **Typography**: `@import` or `<link>` from Google Fonts with standard CSS font stacks.
- **Rule**: Modular CSS architecture (BEM or semantic CSS variables) without heavy build tool dependencies.
