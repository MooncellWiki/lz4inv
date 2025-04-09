# Lz4Inv

Lz4Inv is a Rust-backed Python module for decompressing data compressed with the modified LZ4 algorithm for an anime game. It leverages [PyO3](https://pyo3.rs/) to provide a high-performance decompression function directly from Python.

## Installation

### Prerequisites

- Rust (edition 2021)
- Python (version 3.9 or higher)
- [maturin](https://github.com/PyO3/maturin) for building the extension module

### Build and Install

1. Clone the repository:

   ```powershell
   git clone https://github.com/MooncellWiki/lz4inv.git
   cd lz4inv
   ```

2. Build the module using maturin:

   ```powershell
   maturin develop --release
   ```

3. Install the package (if not using `maturin develop`):

   ```powershell
   pip install .
   ```

## Usage

Import the module in your Python code to decompress LZ4 compressed data.

```python
import lz4inv

with open("compressed_data.bin", "rb") as f:
    compressed_data = f.read()

decompressed_size = 131072
decompressed_data = lz4inv.decompress(compressed_data, decompressed_size)
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [PyO3](https://github.com/PyO3/pyo3) for making Python/Rust interop simple.
- The LZ4 compression library for inspiration.
