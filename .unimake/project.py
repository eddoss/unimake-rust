from umk import cli, project


@project.init
def _(p: project.Info):
    p.name = "gl"
    p.version = "0.1.0"
    p.title = "OpenGL"
    p.description = "Open Graphics Library for C"
    p.contributors = [
        project.Contributor("John Doe", ["jonh.doe@gmail.com"]),
    ]


@cli.cmd("bin")
@cli.opt(str, "src", default="src/name")
@cli.opt(str, "dst", default="dst/name")
def _(src: str, dst: str):
    print(f"[python] Copy file from src={src} to dst={dst}")

# def foo(output: str):
#     print(f"[python] build binary in {output}")


# foo = cli.cmd("foo")(cli.opt)(str, "output", "o")(foo)

# cli.cmd("foo")           -> cmd.inner
# cmd.inner(cli.opt)       -> cli.opt
# cli.opt(str, "out", "o") -> opt.inner
# opt.inner(foo)           -> foo
