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

        [line_type.polyester]
        diameter        = 0.233
        mass_per_length = 53.7
        axial_stiffness = 3.9e8

        [line_type.chain]
        diameter        = 0.127
        mass_per_length = 293.98
        axial_stiffness = 9.83e8
        """
    )
    with Path("test.toml").open("w") as fp:
        fp.write(contents)


@pytest.mark.usefixtures("with_toml_file")
def test_config_from_file() -> None:
    """Load a Config from a TOML file."""
    config = rusty_mooring.Config.from_file("test.toml")
    general = config.general
    assert general.units == "metric"
    assert general.gravity == 9.81
    assert general.water_density == 1025.9
    assert not hasattr(general, "extra_value")

    polyester = config.line_type["polyester"]
    assert polyester.diameter == 0.233
    assert polyester.mass_per_length == 53.7
    assert polyester.axial_stiffness == 3.9e8

    chain = config.line_type["chain"]
    assert chain.diameter == 0.127
    assert chain.mass_per_length == 293.98
    assert chain.axial_stiffness == 9.83e8


def test_config_from_file_missing_raises_error() -> None:
    """An exception is raised if the file doesn't exist."""
    with pytest.raises(FileNotFoundError) as exc_info:
        rusty_mooring.Config.from_file("test.toml")
    assert str(exc_info.value) == "File 'test.toml' not found"
