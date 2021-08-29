use std::fmt::{Display, Formatter};

use inquire::{error::InquireResult, Select};

fn main() -> InquireResult<()> {
    let ans: Currency = Select::new("Currency:", Currency::VARIANTS.to_vec()).prompt()?;

    match ans {
        Currency::BRL | Currency::USD | Currency::CAD | Currency::EUR | Currency::GBP => {
            bank_transfer()
        }
        Currency::BTC | Currency::LTC => crypto_transfer(),
    }

    Ok(())
}

fn bank_transfer() {
    // ask for bank account
    // transfer funds
}

fn crypto_transfer() {
    // ask for wallet address
    // transfer funds
}

#[derive(Debug, Copy, Clone)]
enum Currency {
    BRL,
    USD,
    CAD,
    EUR,
    GBP,
    BTC,
    LTC,
}

impl Currency {
    // could be generated by macro
    const VARIANTS: &'static [Currency] = &[
        Self::BRL,
        Self::USD,
        Self::CAD,
        Self::EUR,
        Self::GBP,
        Self::BTC,
        Self::LTC,
    ];
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}