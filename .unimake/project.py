from umk import project


@project.init
class Project:
    def __init__(self):
        print("from class")


@project.init
def _():
    print("from function")
    # c = project.Contributor("John Doe")
    # c.emails.append("jd@google.com")
    # c.socials["twitter"] = "https://twitter.com/jd"
    # p = project.Info(
    #     name="gl",
    #     version="0.1.0",
    #     title="Open Graphics Library",
    #     description="Open Graphics Library for C",
    # )
    # p.contributors = [c]
    # p.contribute("", "", twiter="")
