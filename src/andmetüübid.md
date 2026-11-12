## Erinevad andmetüübid (Rust)

### Täisarvud (Integer Types)

| Tüüp | Kirjeldus | Näide |
|---|---|---|
| `i8` | -128 kuni 127 | `-100`, `0`, `100` |
| `i16` | 16-bit täisarv | `-30000`, `30000` |
| `i32` | 32-bit täisarv | `-2000000`, `2000000` |
| `i64` | 64-bit täisarv | `-9000000000`, `9000000000` |
| `i128` | 128-bit täisarv | `-1000000000000` |
| `isize` | Arhitektuurist sõltuv täisarv | `-1024`, `1024` |

### Positiivsed täisarvud (Unsigned Types)

| Tüüp | Kirjeldus | Näide |
|---|---|---|
| `u8` | 0 kuni 255 | `0`, `100`, `255` |
| `u16` | 0 kuni 65 535 | `0`, `32000`, `65535` |
| `u32` | 32-bit positiivne täisarv | `1000000` |
| `u64` | 64-bit positiivne täisarv | `100000000` |
| `u128` | 128-bit positiivne täisarv | `1000000000000` |
| `usize` | Arhitektuurist sõltuv täisarv | `0`, `100`, `1024` |

### Komaarvud (Floating-Point Types)

| Tüüp | Kirjeldus | Näide |
|---|---|---|
| `f32` | 32-bit komaarv | `3.14`, `10.5`, `-25.75` |
| `f64` | 64-bit komaarv | `3.1415926535`, `100.99` |

### Muud andmetüübid (Other Types)

| Tüüp | Kirjeldus | Näide |
|---|---|---|
| `bool` | Tõeväärtus | `true`, `false` |
| `char` | Üks Unicode märk | `'A'`, `'õ'`, `'€'` |
| `str` | Tekstiviide | `"Tere maailm"` |
| `String` | Dünaamiline tekst | `String::from("Rust")` |

### Lühidalt

- `i` = positiivsed ja negatiivsed täisarvud.
- `u` = null ja positiivsed täisarvud.
- `f` = komaarvud.
- `bool` = tõene või väär.
- `char` = üks märk.
- `String` = tekst.