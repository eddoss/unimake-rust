from umk import project, cli


@project.init
def _(p: project.Info):
    p.name = "gl"
    p.version = "0.1.0"
    p.title = "OpenGL"
    p.description = "Open Graphics Library for C"
    p.contributors = [
        project.Contributor("John Doe", ["jonh.doe@gmail.com"]),
    ]


@cli.cmd
def _():
    print("hello from cmd")
