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
@cli.opt(str, "src", default="src/name", help="Source file")
@cli.opt(str, "dst", default="dst/name", help="Destination file path")
def foo(src: str, dst: list[str]):
    print(f"[python] Copy file from src={src} to dst={dst}")
