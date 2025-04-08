import pytest
import lz4inv


def test_sum_as_string():
    assert lz4inv.sum_as_string(1, 1) == "2"
