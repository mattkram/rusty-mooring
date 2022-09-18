import nox
from nox import Session


@nox.session()
def mypy(session: Session) -> None:
    session.install("mypy")
    session.run("mypy", ".")


@nox.session(python=["3.9"])
def pytest(session: Session) -> None:
    session.install("pytest", "tests")
    session.run("pytest")
