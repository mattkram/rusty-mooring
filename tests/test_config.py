from __future__ import annotations

from pathlib import Path
from textwrap import dedent

import pytest

import rusty_mooring


@pytest.fixture()
def with_toml_file() -> None:
    """Write a TOML file into the current directory."""
    contents = dedent(
        """\
        [general]
        units = "metric"
        gravity = 9.81
        water_density = 1025.9
        extra_value = 10.0
        """
    )
    with Path("test.toml").open("w") as fp:
        fp.write(contents)


def test_config_init() -> None:
    """Initialize the Config like a normal object, read & write attributes."""
    general = rusty_mooring.GeneralConfig(units="metric", gravity=9.81, water_density=1025.9)

    assert general.units == "metric"
    assert general.gravity == 9.81
    assert general.water_density == 1025.9

    general.units = "english"
    general.gravity = 32.2
    general.water_density = 1.94

    assert general.units == "english"
    assert general.gravity == 32.2
    assert general.water_density == 1.94


@pytest.mark.usefixtures("with_toml_file")
def test_config_from_file() -> None:
    """Load a Config from a TOML file."""
    general = rusty_mooring.GeneralConfig.from_file("test.toml")
    assert general.units == "metric"
    assert general.gravity == 9.81
    assert general.water_density == 1025.9
    assert not hasattr(general, "extra_value")


def test_config_from_file_missing_raises_error() -> None:
    """An exception is raised if the file doesn't exist."""
    with pytest.raises(FileNotFoundError) as exc_info:
        rusty_mooring.GeneralConfig.from_file("test.toml")
    assert str(exc_info.value) == "File 'test.toml' not found"
