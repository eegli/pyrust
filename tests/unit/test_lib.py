from typing import assert_type

import pyrust
from pyrust.cpu import num_cpus


def test_num_cpus() -> None:
    num_cpus_result = num_cpus()
    assert_type(num_cpus_result, int)
    assert num_cpus_result > 0


def test_constants() -> None:
    assert pyrust.backend == "Rust!"
