// Copyright (C) 2026 Jorge Andre Castro
//
// Ce programme est un logiciel libre : vous pouvez le redistribuer et/ou le modifier
// selon les termes de la Licence Publique Générale GNU telle que publiée par la
// Free Software Foundation, soit la version 2 de la licence, soit (à votre convention)
// n'importe quelle version ultérieure.

//! # embedded-invsqrt-f32
//!
//! Inverse de racine carrée rapide `f32` (`1 / sqrt(x)`) pour systèmes embarqués `no_std`.
//!
//! Sans dépendance, sans `unsafe`, sans FPU requise (mais optimisé si présente).
//!
//! ```rust
//! use embedded_invsqrt_f32::inv_sqrt;
//!
//! assert!((inv_sqrt(4.0).unwrap() - 0.5).abs() < 1e-5);
//! assert!((inv_sqrt(1.0).unwrap() - 1.0).abs() < 1e-5);
//! assert!(inv_sqrt(-1.0).is_err());
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Erreur retournée pour une entrée négative ou nulle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NegativeOrZeroInput;

/// Inverse de racine carrée `f32` par méthode itérative.
///
/// Estimation initiale via la constante magique IEEE 754 (style Fast Inverse Square Root),
/// puis itérations de Newton-Raphson pour converger vers une précision élevée.
///
/// Retourne `Err(NegativeOrZeroInput)` si `x <= 0.0`.
pub fn inv_sqrt(x: f32) -> Result<f32, NegativeOrZeroInput> {
    if x.is_nan() {
        return Ok(f32::NAN);
    }
    if x.is_infinite() {
        return Ok(if x.is_sign_negative() { f32::NAN } else { 0.0 });
    }
    if x <= 0.0 {
        return Err(NegativeOrZeroInput);
    }

    // Constante magique pour f32 (variante de la Fast Inverse Square Root)
    let i = 0x5F37_59DF - (x.to_bits() >> 1);
    let mut y = f32::from_bits(i);

    // Itération de Raffinement de Newton-Raphson : y = y * (1.5 - 0.5 * x * y * y)
    // 2 itérations suffisent généralement amplement après la constante magique.
    let xhalf = 0.5 * x;
    y = y * (1.5 - xhalf * y * y);
    y = y * (1.5 - xhalf * y * y);

    Ok(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_values() {
        let expected = 1.0 / (4.0_f32).sqrt();
        assert!((inv_sqrt(4.0).unwrap() - expected).abs() < 1e-5);

        let expected_two = 1.0 / (2.0_f32).sqrt();
        assert!((inv_sqrt(2.0).unwrap() - expected_two).abs() < 1e-5);
    }

    #[test]
    fn negative_or_zero() {
        assert_eq!(inv_sqrt(0.0), Err(NegativeOrZeroInput));
        assert_eq!(inv_sqrt(-1.0), Err(NegativeOrZeroInput));
    }
}