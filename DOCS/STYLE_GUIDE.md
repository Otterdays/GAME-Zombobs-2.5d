# Style Guide

## Code Conventions

### Naming
- **Rust**: `snake_case` for functions/variables, `PascalCase` for types/structs
- **JavaScript**: `camelCase` for functions/variables, `PascalCase` for classes
- **CSS**: `kebab-case` for classes and IDs

### Line Limits
- **Max Line Length**: 100 characters
- **Max Function Length**: 50 lines
- **Max File Length**: 400 lines (refactor if exceeded)

### Comments
- **WHY over WHAT**: Explain reasoning, not obvious code
- **Prefixes**: `TODO:`, `FIXME:`, `NOTE:`
- **Trace Tags**: `// [TRACE: filename.md]` to link code to docs

### Types Over Comments
Let TypeScript/Rust types document intent. Avoid redundant comments.

## Documentation Traces

Use trace tags to link code to documentation:

```rust
// [TRACE: ROADMAP.md]
fn spawn_follow_shadow(world: &mut World, parent: hecs::Entity) {
    // Implementation...
}
```

## Secrets
Never commit secrets. Use `.env` files (gitignored).

## Testing
- **Unit Tests**: Many (80% coverage on business logic)
- **Integration Tests**: Moderate
- **E2E Tests**: Few (critical paths only)
- **Tools**: Vitest/Jest (unit), Playwright (E2E)

## UI Design System

### Colors
```css
--color-primary: #00ff00;      /* Matrix Green */
--color-danger: #ff0000;       /* Red */
--color-warning: #ffaa00;      /* Orange */
--color-background: #0a0c10;   /* Dark Background */
```

### Typography
```css
font-family: 'Courier New', 'Consolas', monospace;
--font-size-small: 12px;
--font-size-normal: 16px;
--font-size-large: 24px;
--font-size-xlarge: 32px;
```

### Animations
- Use CSS transforms (GPU-accelerated)
- Keep animations under 500ms
- Use `ease-out` for most transitions

## Commit Messages
Follow conventional commits:
- `feat:` New feature
- `fix:` Bug fix
- `refactor:` Code restructure
- `docs:` Documentation only
- `style:` Formatting only
- `test:` Test additions

Example: `feat: add Taurus G2C weapon with ammo system`
