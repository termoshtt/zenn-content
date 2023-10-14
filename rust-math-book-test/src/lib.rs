#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/num_traits.md")]
pub mod num_traits {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/ndarray_linalg.md")]
pub mod ndarray_linalg {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/rand.md")]
pub mod rand {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/simd.md")]
pub mod simd {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/thread.md")]
pub mod thread {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/data_format.md")]
pub mod data_format {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/document.md")]
pub mod document {
    use ndarray::Array2;

    /// ```rust
    /// use rust_math_book_test::document::add;
    /// assert_eq!(add(1, 2), 3);
    /// ```
    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }

    /// Test of $\KaTeX$ document
    ///
    /// $$
    /// A = LU
    /// $$
    ///
    /// where $A \in R^{n \times n}$ is input matrix,
    /// and lower triangular matrix $L \in R^{n \times n}$ and upper triangular matrix $U \in R^{n \times n}$ will be returned.
    ///
    /// <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css" integrity="sha384-n8MVd4RsNIU0tAv4ct0nTaAbDJwPJzDEaqSD1odI+WdtXRGWt2kTvGFasHpSy3SV" crossorigin="anonymous">
    /// <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js" integrity="sha384-XjKyOOlGwcjNTAIQHIpgOno0Hl1YQqzUOEleOLALmuqehneUG+vnGctmUb0ZY0l8" crossorigin="anonymous"></script>
    /// <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js" integrity="sha384-+VBxd3r6XgURycqtZ117nYw44OOcIax56Z4dCRWbxyPt0Koah1uHoK0o4+/RRE05" crossorigin="anonymous"></script>
    /// <script>
    ///     document.addEventListener("DOMContentLoaded", function() {
    ///         renderMathInElement(document.body, {
    ///           // customised options
    ///           // • auto-render specific keys, e.g.:
    ///           delimiters: [
    ///               {left: '$$', right: '$$', display: true},
    ///               {left: '$', right: '$', display: false},
    ///               {left: '\\(', right: '\\)', display: false},
    ///               {left: '\\[', right: '\\]', display: true}
    ///           ],
    ///           // • rendering keys, e.g.:
    ///           throwOnError : false
    ///         });
    ///     });
    /// </script>
    ///
    pub fn lu(a: Array2<f64>) -> (Array2<f64>, Array2<f64>) {
        todo!()
    }

    #[cfg_attr(doc, katexit::katexit)]
    /// Test of $\KaTeX$ document
    ///
    /// $$
    /// A = LU
    /// $$
    ///
    /// where $A \in R^{n \times n}$ is input matrix,
    /// and lower triangular matrix $L \in R^{n \times n}$ and upper triangular matrix $U \in R^{n \times n}$ will be returned.
    pub fn lu_(a: Array2<f64>) -> (Array2<f64>, Array2<f64>) {
        todo!()
    }
}

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
