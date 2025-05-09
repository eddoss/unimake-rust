from umk import project

p = project.Info(
    name="gl",
    version="0.1.0",
    title="Open Graphics Library",
    description="Open Graphics Library for C",
)
p.contributors = [
    project.Contributor("John Doe", ["jd@google.com"], {"twitter": "https://twitter.com/jd"})
]

print(p)
print(p.contributors)

# @pro.empty
# def _(s: umk.Project):
#     s.info.id = "sample"
#     s.info.name = "Sample Project"
#     s.info.description = "Sample project detailed description"
#     s.info.version = "v0.2.0"
#     s.info.contrib("John Doe", "john.doe@mail.com")
