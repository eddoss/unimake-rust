import pathlib
import typing as t


class Contributor:
    name: str = ...
    emails: list[str] = ...
    socials: dict[str, str] = ...

    def __init__(
            self,
            name: str = "",
            emails: list[str] = "",
            socials: dict[str, str] = ...
    ): ...


class Info:
    name: str = ...
    version: str = ...
    title: str = ...
    description: str = ...
    contributors: list[Contributor] = ...

    def __init__(
            self,
            name: str = "",
            version: str = "",
            title: str = "",
            description: str = "",
    ): ...

    def contrib(
            self,
            name: str,
            email: str | list[str],
            socials: dict[str, str] = None
    ): ...


class Layout:
    root: pathlib.Path = ...  # Project root directory
    unimake: pathlib.Path = ...  # .unimake directory


class Project:
    info: Info = ...
    layout: Layout = ...


ProjectBuilder = t.Callable[..., t.Any] | t.Callable[[Project], t.Any]


def get() -> Project: ...


def empty(func: ProjectBuilder): ...
