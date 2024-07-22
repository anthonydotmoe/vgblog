use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Upc(u64);

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("invalid UPC: {0}")]
pub struct UpcError(u64);

impl Upc {
    const MAX_UPC: u64 = 999_999_999_999;

    /// Creates a new `Upc` if the provided number is valid.
    /// Returns a `UpcError` if the number is invalid.
    pub fn new(number: u64) -> Result<Self, UpcError> {
        if number > Self::MAX_UPC {
            return Err(UpcError(number));
        }

        if Self::is_valid_upc(number) {
            Ok(Upc(number))
        } else {
            Err(UpcError(number))
        }
    }
    
    /// Validates the UPC-A number using the check digit algorithm.
    fn is_valid_upc(number: u64) -> bool {
        let mut d: [i32;12] = [0; 12];
        let mut num = number;

        for i in (0..=11).rev() {
            d[i] = (num % 10) as i32; 
            num /= 10;
        }

        let check_sum = 
            3 * (d[0] + d[2] + d[4] + d[6] + d[8] + d[10]) +
            (d[1] + d[3] + d[5] + d[7] + d[9] + d[11]);
    
        check_sum % 10 == 0
    }
}

impl Display for Upc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u64> for Upc {
    type Error = UpcError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Upc::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_upc() {
        let test_upcs = [
            883929729258,               // LEGO Star Wars: The Skywalker Saga (PS5)
            083717160090,               // Double Dribble: The Playoff Edition (Genesis)
            805529909632,               // Outrun 2 (Xbox)
            790145134981,               // Inside COM, Dale Rogerson (Microsoft Press)
        ];

        for upc in test_upcs {
            assert!(Upc::new(upc).is_ok(), "{upc} is not a valid UPC")
        }
    }

    #[test]
    fn invalid_upc() {
        let test_upcs = [   // Same as above, +1
            883929729259,
            083717160091,
            805529909633,
            790145134982,
        ];

        for upc in test_upcs {
            assert!(Upc::new(upc).is_err(), "{upc} is a valid UPC")
        }
    }
}
