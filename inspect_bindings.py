from pprint import pprint
from types import ModuleType

import pyrust._bindings as bindings


def recursive_module_inspect(obj: object, depth: int = 2, max_depth: int = 5):
    """
    Recursively inspect modules and their contents.
    Returns a dictionary of the module's contents, recursing into any sub-modules.
    """
    if depth >= max_depth:
        return "Max depth reached"

    if not isinstance(obj, ModuleType):
        return obj

    result = {}
    for name, value in vars(obj).items():
        if isinstance(value, ModuleType):
            result[name] = recursive_module_inspect(value, depth + 1, max_depth)
        else:
            result[name] = value

    return result


pprint(recursive_module_inspect(bindings))
