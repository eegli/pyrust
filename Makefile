.PHONY: all
all: install bindings lint check_types test build_release

.PHONY: install
install:
	@uv sync --all-packages
	
.PHONY: bindings
bindings: install
	@uv run maturin develop --uv

.PHONY: build_release
build_release: build_py_release build_rs_release

.PHONY: build_py_release
build_py_release: install
	@uv run maturin build --release

.PHONY: build_rs_release
build_rs_release:
	@cargo build --release

.PHONY: test
test: test_unit test_cli

.PHONY: test_unit
test_unit:
	@uv run pytest

.PHONY: test_cli
test_cli:
	@bash tests/test_cli.sh

.PHONY: check_types
check_types:
	uv run mypy python/pyrust tests

.PHONY: lint
lint:
	@cargo clippy --all-targets --all-features -- -D warnings
	@uv run ruff check python/pyrust tests

.PHONY: format
format:
	@cargo fmt --all -- --check
	@uv run ruff format python/pyrust tests

.PHONY: clean_cache
clean_cache:
	@find . | grep -E "(__pycache__|\.pyc$$)" | xargs rm -rf
	@find . -type f -name "*.pyd" -delete

.PHONY: clean_venv
clean_venv:
	@rm -rf .venv

.PHONY: clean
clean: clean_cache clean_venv