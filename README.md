# Ziyy - Terminal Styling using HTML-Like Tags

<p align="center">
  <img src="https://raw.githubusercontent.com/alMukaafih/ziyy/main/logo.svg" width="250" alt="Ziyy"s Logo">
</p>

## Overview

Style your Terminal using HTML-like tags `<b>..</b>`, making it easy to apply styles such as bold, italics, and colors directly in your terminal output. For example, `<b c="red">Hello` (where `c` stands for color) will render "Hello" in bold and red.

## Example

```html
<b u i c="rgb(5, 165, 104)">This is a bold green underlined text in italics</b>
```

You can omit the closing tag or use `</>` instead.

### Tags

| Tags                                           | Description                                                                     |
| ---------------------------------------------- | ------------------------------------------------------------------------------- |
| `<>..</>`                                      | Empty tag                                                                       |
| `<z> \| <ziyy>`                                | Normalize whitespace.                                                           |
| `<div>`                                        | Used to group related content. Inserts a newline if not at the start of a line. |
| `<pre>`                                        | Preserved Whitespace.                                                           |
| `<p>`                                          | Inserts a newline if not at the start of a line.                                |
| `<a>`                                          | Creates a hyperlink. For example: `<a href="https://example.com">Example</a>`.  |
| `<b> \| <strong>`                              | Causes text to be bold.                                                         |
| `<br/>`                                        | Produces a line break in text (carriage-return).                                |
| `<d> \| <dim>`                                 | Causes text to be dim.                                                          |
| `<h> \| <hidden> \| <hide> \| <invisible>`     | Causes text to be hidden.                                                       |
| `<k> \| <blink>`                               | Causes text to blink.                                                           |
| `<r> \| <invert> \| <reverse> \| <negative>`   | Reverse foreground and background colors of text.                               |
| `<i> \| <em> \| <italics>`                     | Causes text to be italicized.                                                   |
| `<s> \| <del> \| <strike> \| <strike-through>` | Strikes through text.                                                           |
| `<u> \| <ins>`                                 | Underlines text.                                                                |
| `<uu> \| <double-under> \| <double-underline>` | Double Underlines text.                                                         |
| `<c> \| <fg>`                                  | Sets foreground color.                                                          |
| `<x> \| <bg>`                                  | Sets background color.                                                          |
| `<let/>`                                       | Declares new custom tag.                                                        |

## Attributes

| Property                                 | Description                                                        |
| ---------------------------------------- | ------------------------------------------------------------------ |
| `b \| bold \| strong`                    | Causes text to be bold.                                            |
| `d \| dim`                               | Causes text to be dim.                                             |
| `h \| hidden \| hide \| invisible`       | Causes text to be hidden.                                          |
| `k \| blink`                             | Causes text to blink.                                              |
| `r \| invert \| reverse \| negative`     | Reverse foreground and background colors of text.                  |
| `i \| em \| italics`                     | Causes text to be italicized.                                      |
| `s \| del \| strike \| strike-through`   | Strike through text.                                               |
| `u \| ins \| under \|  underline`        | Underlines text.                                                   |
| `uu \| double-under \| double-underline` | Underlines text using double lines.                                |
| `double`                                 | Underlines text using double lines (`<u>` only).                   |
| `c="COLOR" \| fg="COLOR"`                | Sets foreground color.                                             |
| `x="COLOR" \| bg="COLOR"`                | Sets background color.                                             |
| `ul="COLOR"`                             | Sets Underline color.                                              |
| `fixed="0-255" \| 0-255`                 | ANSI 256 color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only). |
| `rgb="0-255, 0-255, 0-255"`              | Rgb colors (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).     |
| `black \| black="light"`                 | Black color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).    |
| `red \| red="light"`                     | Red color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).      |
| `green \| green="light"`                 | Green color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).    |
| `yellow \| yellow="light"`               | Yellow color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).   |
| `blue \| blue="light"`                   | Blue color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).     |
| `magenta \| magenta="light"`             | Magenta color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).  |
| `cyan \| cyan="light"`                   | Cyan color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).     |
| `white \| white="light"`                 | White color (`<c> \| <fg> \| <x> \| <bg> \| <u> \| <uu>` only).    |
| `id="..."`                               | Name of custom tag declared by `<let/>`.                           |
| `class="..."`                            | A space-separated list of tags to inherit styles from.             |
| `indent="0-255"`                         | indent a `<p>` with _n_ spaces.                                    |
| `href="..."`                             | url that `<a>` points to.                                          |
| `n="0-255"`                              | number of line breaks `<br/>` should insert. Default is 1.         |

> COLOR is any of `fixed(0-255) | rgb(0-255, 0-255, 0-255) | #RRGGBB | #RGB | black | red | green | yellow | blue | magenta | cyan | white`
