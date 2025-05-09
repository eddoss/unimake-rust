import socket
from asyncio.tasks import Task
from umk import project
# print(dir(project.Contributor))

p = project.Contributor(
    name="John Doe",
    emails=["jd@google.com"],
    socials={"twitter": "https://twitter.com/jd"},
)
# p = project.Contributor(
    # name="John Doe",
    # socials={
    #     "twitter": "https://twitter.com/jd",
    #     "github": 2,
    # }
# )
# p.name = "John Doe"
# print(f"Contributor", dir(p))
p.emails = ["hello@mail.ru", "my@gmail.net"]
p.emails.append("job@apple.com")
print(p.emails)
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
