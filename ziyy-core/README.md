# Ziyy's core library

Ziyy is a markup language that allows you to style your terminal using HTML-like syntax.

For information on using Ziyy, see the [Ziyy website](https://ziyy-dev.github.io).

## Usage

```rust
use ziyy_core::style;

let styled = style("<b>Lorem
    <d> dolor sit
        <b>amet consectetur
            <d>adipiscing elit</d>
            quisque
        </b>faucibus ex sapien.");
```

## Feature flags

- `unstable` Can break or be removed at any time without warning.
- `parallel` Enables parallel parsing.
