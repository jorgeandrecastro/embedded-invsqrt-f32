Markdown

# embedded-invsqrt-f32

[![Crates.io](https://img.shields.io/crates/v/embedded-invsqrt-f32)](https://crates.io/crates/embedded-invsqrt-f32)
[![Téléchargements](https://img.shields.io/crates/d/embedded-invsqrt-f32)](https://crates.io/crates/embedded-invsqrt-f32)
[![Documentation](https://docs.rs/embedded-invsqrt-f32/badge.svg)](https://docs.rs/embedded-invsqrt-f32)

Approximation rapide de l'inverse de la racine carrée pour `f32` (`1 / sqrt(x)`) dans un contexte `no_std`, optimisée pour les systèmes embarqués.

## Version 0.1.0

- Gestion robuste des cas `NaN` et `Infinity`
- Sécurité renforcée avec `#![forbid(unsafe_code)]`
- Optimisation de performance via `opt-level = 3`

## Points forts

- Zero dependency : aucune dépendance sur `libm`, `micromath` ou une bibliothèque C.
- Sécurité maximale : code sans `unsafe` et gestion explicitée avec `Result`.
- Portabilité élevée :
  - Cibles sans FPU (Cortex-M0+, RISC-V) : implémentation logicielle déterministe.
  - Cibles avec FPU (Cortex-M4F, M33, M7) : utilisation native des unités flottantes.
- Précision nette : raffinement par Newton-Raphson pour réduire l'erreur relative.

## Installation

Ajoutez la dépendance dans votre `Cargo.toml` :

```toml
[dependencies]
embedded-invsqrt-f32 = "0.1"
```

## Exemple d'utilisation

```rust
use embedded_invsqrt_f32::inv_sqrt;

if let Ok(result) = inv_sqrt(4.0) {
    // résultat attendu : 0.5
    println!("{:?}", result);
}

match inv_sqrt(-1.0) {
    Ok(val) => {
        println!("valeur : {}", val);
    }
    Err(e) => {
        eprintln!("erreur : {}", e);
    }
}
```

## Algorithme

L'implémentation repose sur une variante de la méthode Fast Inverse Square Root, complétée par des itérations de Newton-Raphson.

1. Estimation initiale : manipulation directe des bits IEEE 754 via une constante magique.
2. Raffinement : 2 itérations de la formule `y = y * (1.5 - 0.5 * x * y * y)`.
3. Résultat : approximation très rapide et suffisamment précise pour `f32` en environnement embarqué.

## Licence

GPL-2.0-or-later

Copyright (C) 2026 Jorge Andre Castro.
