def cmd(
    name: str,
    *,
    help: str = ...
): ...


def opt(
    klass: type,
    name: str,
    *,
    short: str = ...,
    default=None,
    var: str | None = None,
    help: str = ...,
): ...


def arg(
    klass: type,
    name: str,
    *,
    default=None,
    var: str | None = None,
    help: str = ...,
): ...
