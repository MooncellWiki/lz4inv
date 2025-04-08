use pyo3::prelude::*;
use std::fmt::Display;

#[derive(Debug)]
pub enum DecompressError {
    /// The provided output is too small
    OutputTooSmall {
        /// Minimum expected output size
        expected: usize,
        /// Actual size of output
        actual: usize,
    },
    /// Literal is out of bounds of the input
    LiteralOutOfBounds,
    /// Deduplication offset out of bounds (not in buffer).
    OffsetOutOfBounds,
}
impl Display for DecompressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecompressError::OutputTooSmall { expected, actual } => {
                write!(
                    f,
                    "Output too small: expected {}, actual {}",
                    expected, actual
                )
            }
            DecompressError::LiteralOutOfBounds => write!(f, "Literal out of bounds"),
            DecompressError::OffsetOutOfBounds => write!(f, "Offset out of bounds"),
        }
    }
}
impl From<DecompressError> for PyErr {
    fn from(err: DecompressError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(err.to_string())
    }
}
pub fn decompress_impl(src: &[u8], dst: &mut [u8]) -> Result<usize, DecompressError> {
    let mut src_pos = 0;
    let mut dst_pos = 0;

    while src_pos < src.len() && dst_pos < dst.len() {
        // Get token
        let (mut literal_length, mut match_length) = get_literal_token(src, &mut src_pos);

        // Copy literal chunk
        literal_length = get_length(literal_length, src, &mut src_pos);
        if literal_length > src.len() - src_pos {
            return Err(DecompressError::LiteralOutOfBounds);
        }
        if literal_length > dst.len() - dst_pos {
            return Err(DecompressError::OutputTooSmall {
                expected: dst_pos + literal_length,
                actual: dst.len(),
            });
        }

        dst[dst_pos..dst_pos + literal_length]
            .copy_from_slice(&src[src_pos..src_pos + literal_length]);

        src_pos += literal_length;
        dst_pos += literal_length;

        if src_pos >= src.len() {
            break;
        }

        // Copy compressed chunk
        let offset = get_chunk_end(src, &mut src_pos);

        match_length = get_length(match_length, src, &mut src_pos) + 4;

        // 复制的源开始的地方
        let (enc_pos, did_overflow) = dst_pos.overflowing_sub(offset);
        if did_overflow {
            return Err(DecompressError::OffsetOutOfBounds);
        }
        if dst_pos + match_length > dst.len() {
            return Err(DecompressError::OutputTooSmall {
                expected: dst_pos + match_length,
                actual: dst.len(),
            });
        }

        if match_length <= offset {
            dst.copy_within(enc_pos..enc_pos + match_length, dst_pos);
            dst_pos += match_length;
        } else {
            // overlapping
            let mut match_length_remain = match_length;
            let mut curr_enc_pos = enc_pos;
            let mut curr_dst_pos = dst_pos;

            while match_length_remain > 0 {
                dst[curr_dst_pos] = dst[curr_enc_pos];
                curr_enc_pos += 1;
                curr_dst_pos += 1;
                match_length_remain -= 1;
            }

            dst_pos = curr_dst_pos;
        }
    }

    Ok(dst_pos)
}

// |literal|match|
// |0000|0000|
// 正常来说应该是高四位的是*不需要*解压的长度(literal)，低四位的是*需要*解压的长度(match)，但是这里反过来了
fn get_literal_token(src: &[u8], src_pos: &mut usize) -> (usize, usize) {
    let token = src[*src_pos];
    *src_pos += 1;
    ((token & 0xf) as usize, ((token >> 4) & 0xf) as usize)
}

// 正常应该是个读一个le16 但是这里要读一个be16
fn get_chunk_end(src: &[u8], src_pos: &mut usize) -> usize {
    let high = src[*src_pos] as usize;
    *src_pos += 1;
    let low = src[*src_pos] as usize;
    *src_pos += 1;
    (high << 8) | low
}

// 读*不需要*解压的数据的长度
fn get_length(mut length: usize, src: &[u8], src_pos: &mut usize) -> usize {
    if length == 0xf {
        let mut sum;
        loop {
            sum = src[*src_pos] as usize;
            *src_pos += 1;
            length += sum;
            if sum != 0xff {
                break;
            }
        }
    }
    length
}

/// Decompresses LZ4 compressed data
#[pyfunction]
fn decompress(compressed: &[u8], decompressed_size: usize) -> Result<Vec<u8>, DecompressError> {
    let mut decompressed = vec![0u8; decompressed_size];
    decompress_impl(compressed, &mut decompressed)?;
    Ok(decompressed)
}

/// A Python module implemented in Rust.
#[pymodule]
fn lz4inv(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decompress, m)?)?;
    Ok(())
}
