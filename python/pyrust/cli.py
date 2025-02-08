import sys

from pyrust._bindings import cli


# example of Python making use of a Rust function
def launch_cli():
    # we could also have clap parse the args directly. this is just to
    # show that we could pass in different initializers.
    return cli.PyCli(sys.argv).run()


__all__ = ["launch_cli"]
