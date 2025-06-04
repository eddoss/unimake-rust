from umk import cli, project


@project.init
class Project(project.Info):
    def __init__(self):
        self.name = "gl"
        self.version = "0.1.0"
        self.title = "OpenGL"
        self.description = "Open Graphics Library for C"
        self.contributors = [
            project.Contributor("John Doe", ["jonh.doe@gmail.com"]),
        ]


@cli.cmd("bin")
@cli.opt(str, "src", default="src/name", help="Source file")
@cli.opt(str, "dst", default="dst/name", help="Destination file path")
def foo(src: str, dst: str):
    print(f"[python] Copy file from src={src} to dst={dst}")
