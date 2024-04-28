[![Build](https://github.com/pirsonxyz/sauss-cramer/actions/workflows/rust.yml/badge.svg)](https://github.com/pirsonxyz/sauss-cramer/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/sauss-cramer.svg)](https://crates.io/crates/sauss-cramer)
[![Documentation](https://docs.rs/sauss-cramer/badge.svg)](https://docs.rs/sauss-cramer)
[![License](https://img.shields.io/crates/l/sauss-cramer.svg)](https://crates.io/crates/sauss-cramer)

# Sauss-Cramer, una librería para resolver sistemas de ecuaciones lineales.

## Esta es una librería hecha en Rust para resolver sistemas de ecuaciones lineales usando el método de [Cramer](https://es.wikipedia.org/wiki/Regla_de_Cramer).

### Ejemplo resolviendo un sistema 3x3.

```rust
use sauss_cramer::{solve_3_variables, print_3_variables_result};

fn main() {
    let result = solve_3_variables(
        5.0, 2.0, 1.0, -2.0, 5.0, -4.0, 1.0, -2.0, 3.0, 24.0, -14.0, 26.0,
    );

    print_3_variables_result(result); // result: x = 3, y = 2, z = 5.
}
```
