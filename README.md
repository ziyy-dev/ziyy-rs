# Ziyy - Simple terminal styling.

Ziyy is a markup language that allows you to style your terminal using HTML-like syntax.

For information on using Ziyy, see the [Ziyy website](https://ziyy-dev.github.io).

## Usage

### Command line

```bash
ziyy -c "<b>Lorem
    <d> dolor sit
        <b>amet consectetur
            <d>adipiscing elit</d>
            quisque
        </b>faucibus ex sapien."

```

## As a Library

```rust
use ziyy::style;

let styled = style("<b>Lorem
    <d> dolor sit
        <b>amet consectetur
            <d>adipiscing elit</d>
            quisque
        </b>faucibus ex sapien.");
```
