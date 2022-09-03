from __future__ import annotations

from pathlib import Path
from textwrap import dedent
from typing import TYPE_CHECKING

import pytest

import rusty_mooring

if TYPE_CHECKING:
    from _pytest.monkeypatch import MonkeyPatch


@pytest.fixture(autouse=True)
def tmp_cwd(tmp_path: Path, monkeypatch: MonkeyPatch) -> None:
    """Ensure we always run from a blank, temporary directory."""
    monkeypatch.chdir(tmp_path)


@pytest.fixture()
def with_toml_file() -> None:
    """Write a TOML file into the current directory."""
    contents = dedent(
        """\
        [config]
        ip = "42.69.42.0"
        port = 42
        """
    )
    with Path("test.toml").open("w") as fp:
        fp.write(contents)


def test_config_init() -> None:
    """Initialize the Config like a normal object, read & write attributes."""
    config = rusty_mooring.Config(ip="1.2.3.4", port=8000)

    assert config.ip == "1.2.3.4"
    assert config.port == 8000

    config.ip = "4.3.2.1"
    assert config.ip == "4.3.2.1"


@pytest.mark.usefixtures("with_toml_file")
def test_config_from_file() -> None:
    """Load a Config from a TOML file."""
    config = rusty_mooring.Config.from_file("test.toml")
    assert config.ip == "42.69.42.0"
    assert config.port == 42


def test_config_from_file_missing_raises_error() -> None:
    """An exception is raised if the file doesn't exist."""
    with pytest.raises(FileNotFoundError) as exc_info:
        rusty_mooring.Config.from_file("test.toml")
    assert str(exc_info.value) == "File 'test.toml' not found"
