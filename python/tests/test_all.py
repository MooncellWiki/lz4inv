import pytest
import lz4inv


def test_decompress():
    assert lz4inv.decompress(1, 1) == "2"
