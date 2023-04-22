#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/num_traits.md")]
pub mod num_traits {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/ndarray_linalg.md")]
pub mod ndarray_linalg {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/rand.md")]
pub mod rand {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/simd.md")]
pub mod simd {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/data_format.md")]
pub mod data_format {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/criterion.md")]
pub mod criterion {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/nom.md")]
pub mod nom {
    use ::nom::character::complete::digit1;
    use num_traits::Unsigned;
    use std::str::FromStr;
    pub fn uint<I: Unsigned + FromStr>(input: &str) -> Result<I> {
        let (residual, digits) = digit1(input)?;
        let num: I = digits
            .parse()
            .map_err(|_| failure(input, "unsigned integer"))?;
        Ok((residual, num))
    }
    use nom::error::{VerboseError, VerboseErrorKind};
    pub type Result<'input, T> = nom::IResult<&'input str, T, VerboseError<&'input str>>;
    fn failure<'input>(
        input: &'input str,
        msg: &'static str,
    ) -> nom::Err<VerboseError<&'input str>> {
        nom::Err::Failure(VerboseError {
            errors: vec![(input, VerboseErrorKind::Context(msg))],
        })
    }
}
