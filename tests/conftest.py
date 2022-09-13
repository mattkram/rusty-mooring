from __future__ import annotations

from pathlib import Path
from textwrap import dedent
from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from _pytest.monkeypatch import MonkeyPatch


@pytest.fixture(autouse=True)
def tmp_cwd(tmp_path: Path, monkeypatch: MonkeyPatch) -> None:
    """Ensure we always run from a blank, temporary directory."""
    monkeypatch.chdir(tmp_path)


@pytest.fixture()
def config_file() -> Path:
    """Write a TOML file into the current directory."""
    contents = dedent(
        """\
        [general]
        units = "metric"
        gravity = 9.81
        water_density = 1025.9
        water_depth = 2000.0
        extra_value = 10.0

        [line_types.polyester]
        diameter        = 0.233
        mass_per_length = 53.7
        youngs_modulus = 9.15e9
        internal_diameter = 0.0
        internal_contents_density = 0.0

        [line_types.chain]
        diameter        = 0.127
        mass_per_length = 293.98
        youngs_modulus = 7.76e10
        internal_diameter = 0.0
        internal_contents_density = 0.0

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
    config_file = Path("test.toml")
    with config_file.open("w") as fp:
        fp.write(contents)

    return config_file
