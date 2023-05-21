# mrtd

[![Crate](https://img.shields.io/crates/v/mrtd.svg)](https://crates.io/crates/mrtd)
[![Documentation](https://docs.rs/mrtd/badge.svg)](https://docs.rs/mrtd)
![Build Status](https://github.com/asmarques/mrtd/workflows/CI/badge.svg)

A Rust parser for the machine-readable zone (MRZ) of machine-readable travel documents (MRTD) as defined by ICAO Document 9303.

Supported travel documents:

- Passport
- Identity Card

## Example

```rust
use mrtd::{parse, Document};

fn main() {
    let mrz = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
            L898902C36UTO7408122X1204159ZE184226B<<<<<10";
    let Document::Passport(passport) = parse(mrz).unwrap();
    assert_eq!(passport.passport_number, "L898902C3");
    println!("{:?}", passport);

    let id_card_mrz = "C<ITACA00000AA4<<<<<<<<<<<<<<<\
        6412308F2212304ITA<<<<<<<<<<<0\
        ROSSI<<BIANCA<<<<<<<<<<<<<<<<<";
    let Document::IdentityCard(identity_card) = parse(id_card_mrz).unwrap();
    assert_eq!(identity_card.document_number, "CA00000AA");
    println!("{:?}", identity_card);
}
```
