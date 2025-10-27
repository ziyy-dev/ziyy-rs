# Style

| STYLE              | NO OF BITS | Byte Index |
| ------------------ | ---------- | ---------- |
| Previous Intensity | 2          | 0          |
| Intensity          | 3          | 0          |
| Italics/Fractur    | 3          | 0          |

| STYLE     | NO OF BITS | Byte Index |
| --------- | ---------- | ---------- |
| Negative  | 2          | 1          |
| Underline | 3          | 1          |
| Blink     | 3          | 1          |

| STYLE                   | NO OF BITS | Byte Index |
| ----------------------- | ---------- | ---------- |
| Fg color (1 Bit)        | 1          | 2          |
| Bg color (1 Bit)        | 1          | 2          |
| Underline color (1 Bit) | 1          | 2          |
| Reset                   | 1          | 2          |
| Hidden                  | 2          | 2          |
| Deleted                 | 2          | 2          |

| STYLE           | NO OF BITS | Byte Index |
| --------------- | ---------- | ---------- |
| Fg color        | 24         | 3..6       |
| Bg color        | 24         | 6..9       |
| Underline color | 24         | 9..12      |

| STYLE                | NO OF BITS | Byte Index |
| -------------------- | ---------- | ---------- |
| Proportional spacing | 2          | 12         |
| Framed/Encircled     | 3          | 12         |
| Ideogram             | 3          | 12         |

| STYLE      | NO OF BITS | Byte Index |
| ---------- | ---------- | ---------- |
| Reserved 1 | 1          | 13         |
| Reserved 2 | 1          | 13         |
| Overline   | 2          | 13         |
| Font       | 4          | 13         |
