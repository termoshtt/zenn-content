#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/error_handling.md")]
pub mod error_handling {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/num_traits.md")]
pub mod num_traits {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/ndarray_linalg.md")]
pub mod ndarray_linalg {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/rand.md")]
pub mod rand {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/simd.md")]
pub mod simd {

    // translated from
    // <https://github.com/Matherunner/bin2hex-sse/blob/master/base16_sse4.cpp>
    #[target_feature(enable = "sse4.1")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub unsafe fn hex_encode_sse41(mut src: &[u8], dst: &mut [u8]) {
        #[cfg(target_arch = "x86")]
        use std::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;

        let ascii_zero = _mm_set1_epi8(b'0' as i8);
        let nines = _mm_set1_epi8(9);
        let ascii_a = _mm_set1_epi8((b'a' - 9 - 1) as i8);
        let and4bits = _mm_set1_epi8(0xf);

        let mut i = 0_isize;
        while src.len() >= 16 {
            let invec = _mm_loadu_si128(src.as_ptr() as *const _);

            let masked1 = _mm_and_si128(invec, and4bits);
            let masked2 = _mm_and_si128(_mm_srli_epi64(invec, 4), and4bits);

            // return 0xff corresponding to the elements > 9, or 0x00 otherwise
            let cmpmask1 = _mm_cmpgt_epi8(masked1, nines);
            let cmpmask2 = _mm_cmpgt_epi8(masked2, nines);

            // add '0' or the offset depending on the masks
            let masked1 = _mm_add_epi8(masked1, _mm_blendv_epi8(ascii_zero, ascii_a, cmpmask1));
            let masked2 = _mm_add_epi8(masked2, _mm_blendv_epi8(ascii_zero, ascii_a, cmpmask2));

            // interleave masked1 and masked2 bytes
            let res1 = _mm_unpacklo_epi8(masked2, masked1);
            let res2 = _mm_unpackhi_epi8(masked2, masked1);

            _mm_storeu_si128(dst.as_mut_ptr().offset(i * 2) as *mut _, res1);
            _mm_storeu_si128(dst.as_mut_ptr().offset(i * 2 + 16) as *mut _, res2);
            src = &src[16..];
            i += 16;
        }

        let i = i as usize;
        hex_encode_fallback(src, &mut dst[i * 2..]);
    }

    pub fn hex_encode_fallback(src: &[u8], dst: &mut [u8]) {
        fn hex(byte: u8) -> u8 {
            static TABLE: &[u8] = b"0123456789abcdef";
            TABLE[byte as usize]
        }

        for (byte, slots) in src.iter().zip(dst.chunks_mut(2)) {
            slots[0] = hex((*byte >> 4) & 0xf);
            slots[1] = hex(*byte & 0xf);
        }
    }
}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/thread.md")]
pub mod thread {}

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/rayon.md")]
pub mod rayon {}

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

    #[cfg_attr(doc, aquamarine::aquamarine)]
    /// Test mermaid.js
    ///
    /// ```mermaid
    /// graph TD;
    ///     A-->B;
    ///     A-->C;
    ///     B-->D;
    ///     C-->D;
    /// ```
    ///
    pub fn mermaid_test() {}

    #[cfg_attr(doc, p5doc::p5doc)]
    /// Test p5.js
    ///
    /// ```p5doc:200x100
    /// background(220);
    /// ellipse(50,50,80,80);
    /// ```
    ///
    pub fn p5doc_test() {}
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
