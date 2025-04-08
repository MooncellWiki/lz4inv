import pathlib
import lz4inv

def test_decompress():
    # Get the directory of the current test file
    current_dir = pathlib.Path(__file__).parent.absolute()

    # Construct paths to the test data files
    compressed_file_path = current_dir / "compressed_data.bin"
    expected_file_path = current_dir / "decompressed_data.bin"

    with open(compressed_file_path, "rb") as f:
        compressed_data = f.read()
        uncompressed = lz4inv.decompress(compressed_data, 131072)
        # Read the expected decompressed data
        with open(expected_file_path, "rb") as expected_file:
            expected_data = expected_file.read()

        # Compare the uncompressed data with the expected data
        assert uncompressed == expected_data, (
            "Decompressed data does not match expected data"
        )
