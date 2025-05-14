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

    def with_email(self, email: str) -> 'Contributor': ...

    def with_social(self, email: str) -> 'Contributor': ...


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

    def contribute(
            self,
            name: str,
            email: str | list[str],
            **socials: str,
    ): ...


class Layout:
    root: pathlib.Path = ...  # Project root directory
    unimake: pathlib.Path = ...  # .unimake directory


class Project:
    info: Info = ...
    layout: Layout = ...


ProjectBuilder = t.Callable[..., t.Any] | t.Callable[[Project], t.Any]


def get() -> Project: ...


def init(func: ProjectBuilder): ...
