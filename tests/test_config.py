from __future__ import annotations

from pathlib import Path
from textwrap import dedent

import pytest

from rusty_mooring import Config


@pytest.fixture()
def config() -> Config:
    """Write a TOML file into the current directory."""
    contents = dedent(
        """\
        [general]
        units = "metric"
        gravity = 9.81
        water_density = 1025.9
        extra_value = 10.0

        [line_types.polyester]
        diameter        = 0.233
        mass_per_length = 53.7
        axial_stiffness = 3.9e8

        [line_types.chain]
        diameter        = 0.127
        mass_per_length = 293.98
        axial_stiffness = 9.83e8

        [lines.Line1]
        top_position    = [ 34, 34, -26.2 ]
        bottom_position = [ 1700, 10, -1961.74 ]
        segments = [
            {line_type="chain", length=1000, num_elements=200},
            {line_type="polyester", length=1800, num_elements=360},
        ]

        [lines.Line2]
        top_position    = [ 34, 34, -26.2 ]
        bottom_position = [ 1700, 10, -1961.74 ]
        segments = [
            {line_type="chain", length=1000, num_elements=200},
            {line_type="polyester", length=1800, num_elements=360},
        ]
        """
    )
    filename = "test.toml"
    with Path(filename).open("w") as fp:
        fp.write(contents)

    config = Config.from_file(filename)
    return config


def test_load_general_config_from_file(config: Config) -> None:
    """Load a Config from a TOML file."""
    general = config.general
    assert general.units == "metric"
    assert general.gravity == 9.81
    assert general.water_density == 1025.9
    assert not hasattr(general, "extra_value")


def test_load_line_types_from_file(config: Config) -> None:
    """Load line types from TOML file."""
    polyester = config.line_types["polyester"]
    assert polyester.diameter == 0.233
    assert polyester.mass_per_length == 53.7
    assert polyester.axial_stiffness == 3.9e8

    chain = config.line_types["chain"]
    assert chain.diameter == 0.127
    assert chain.mass_per_length == 293.98
    assert chain.axial_stiffness == 9.83e8


def test_load_lines_from_file(config: Config) -> None:
    """We can load any number of lines with a rich schema from the file."""
    assert len(config.lines) == 2

    line = config.lines["Line1"]
    assert line.top_position == [34, 34, -26.2]
    assert line.bottom_position == [1700, 10, -1961.74]

    assert line.segments[0].line_type == "chain"
    assert line.segments[0].length == 1000.0
    assert line.segments[0].num_elements == 200

    assert line.segments[1].line_type == "polyester"
    assert line.segments[1].length == 1800.0
    assert line.segments[1].num_elements == 360


def test_config_from_file_missing_raises_error() -> None:
    """An exception is raised if the file doesn't exist."""
    with pytest.raises(FileNotFoundError) as exc_info:
        Config.from_file("test.toml")
    assert str(exc_info.value) == "File 'test.toml' not found"
