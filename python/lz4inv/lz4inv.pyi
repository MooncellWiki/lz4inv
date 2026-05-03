__all__ = ["decompress", "decompress_buffer"]

def decompress(decompress_bytes: bytes, uncompressed_size: int) -> bytes: ...

def decompress_buffer(
    decompress_bytes: bytes | bytearray | memoryview, uncompressed_size: int
) -> bytes: ...
