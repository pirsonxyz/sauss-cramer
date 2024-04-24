# Sauss-Cramer, una libreria para resolver sistemas de ecuaciones lineales.

## Esta es una libreria hecha en Rust para resolver sistemas de ecuaciones usando el metodo de [Cramer](https://es.wikipedia.org/wiki/Regla_de_Cramer).

### Ejemplo resolviendo un sistema 3x3.

```rust
use sauss_cramer::solve_3_determinants;

fn main() {
    let result = solve_3_determinants(
        5.0, 2.0, 1.0, -2.0, 5.0, -4.0, 1.0, -2.0, 3.0, 24.0, -14.0, 26.0,
    );

    println!("{}", result.0); // 3
    println!("{}", result.1); // -2
    println!("{}", result.2); // 5
}
```
