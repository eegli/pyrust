# PyRust

**A minimal skeleton for building Rust applications with a Python frontend.**

Building native, Rust-based Python modules is not very straightforward, especially since most inter-dependencies are barely explained in the `maturin` tutorial. A full walkthrough for this template is available at TODO.

This template includes:

- Rust as a foundation with support for [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- A generated Python package
- A CLI implemented in Rust, exposed to Python
- Stubs to enable Python type hints
- Test setup, including GitHub actions
- Linting and packaging
- A Makefile for convenience

Further reading and sources:

- https://www.maturin.rs/
- https://pyo3.rs/v0.23.4/

Some design choices are inspired by how [Polars](https://github.com/pola-rs/polars) and [uv](https://github.com/astral-sh/uv) create Rust bindings for Python.
