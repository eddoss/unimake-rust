import typing as t


class Contributor:
    name: str = ...
    emails: list[str] = ...
    socials: dict[str, str] = ...

    def __init__(
            self,
            name: str = ...,
            emails: list[str] = ...,
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


class Project:
    info: Info = ...


def get() -> Project: ...


Initializer = t.Callable[..., t.Any] | t.Callable[[Project], t.Any]


def init(func: Initializer): ...
