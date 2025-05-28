import pathlib
import typing as t


class Layout:
    root: pathlib.Path = ...  # Project root directory
    unimake: pathlib.Path = ...  # .unimake directory


def get() -> t.Any: ...


Initializer = t.Callable[[Layout], t.Any]


def init(func: Initializer): ...
