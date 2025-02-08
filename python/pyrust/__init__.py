import importlib.metadata

from pyrust._bindings import backend

try:
    __version__ = importlib.metadata.version("pyrust")
except Exception:
    __version__ = "Unknown"

# example of a global export
__all__ = ["backend"]
