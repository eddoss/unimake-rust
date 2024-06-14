from asyncio.tasks import Task
from umk import project
# print(dir(project.Contributor))

print(project.Contributor())
# joe = project.Contributor("Joe", emails=["joe@email.com"])
# print(joe.name)

# print(dir(umk))
# print("project" in dir(umk))
# print(dir(umk.project))
# print(type(project))
# print(dir(project))
# from umk import project as pro
#
# p = pro.project()
# p.show()
#
#
# @pro.empty
# def _():
#     print(__file__)


# @pro.empty
# def _(s: umk.Project):
#     s.info.id = "sample"
#     s.info.name = "Sample Project"
#     s.info.description = "Sample project detailed description"
#     s.info.version = "v0.2.0"
#     s.info.contrib("John Doe", "john.doe@mail.com")
