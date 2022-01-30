import pytest

import rustyter


@pytest.mark.parametrize("a,b", [(1, 2), (20, 3), (109, 4)])
def test_sum_as_string(a, b):
    got = rustyter.sum_as_string(a, b)
    expected = str(a + b)

    assert got == expected


@pytest.mark.parametrize("it", [list(range(100)), list(), list(range(23000)), ["a", 1, -.4], ["ab"], [list(), dict(), set()]])
def test_length(it):
    got = rustyter.length(it)
    expected = len(it)

    assert got == expected
