"""Stub files for all bindings"""

backend: str

def num_cpus() -> int: ...

# cli is defined as a submodule:
# https://pyo3.rs/v0.23.4/module.html#python-submodules. note that
# this does not define a package. we can provide stubs via class
# properties.
class cli:
    class PyCli:
        def __init__(self, launch_args: list[str]) -> None: ...
        def run(self) -> None: ...
