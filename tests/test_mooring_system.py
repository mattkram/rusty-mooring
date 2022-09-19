from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import pytest

from rusty_mooring import Config
from rusty_mooring import MooringSystem

if TYPE_CHECKING:
    from _pytest.capture import CaptureFixture
    from _pytest.fixtures import SubRequest


@pytest.fixture()
def config(config_file: Path) -> Config:
    """The configuration, after being loaded from the TOML file."""
    config = Config.from_file(config_file.as_posix())
    return config


@pytest.fixture(params=["from_file", "from_obj"])
def mooring_system(request: SubRequest, config_file: Path, config: Config) -> MooringSystem:
    response_map = {
        "from_file": MooringSystem.from_file(config_file.as_posix()),
        "from_obj": MooringSystem(config),
    }
    return response_map[request.param]


def test_load_mooring_system_from_file(mooring_system: MooringSystem) -> None:
    """We can load the whole system config from a file or Config object via the MooringSystem class."""
    assert mooring_system.config.line_types["chain"].diameter == 0.127


def test_solve_static(mooring_system: MooringSystem, capsys: CaptureFixture) -> None:
    """Solve statics and check coordinate output."""
    # TODO: This is a regression test that is sensitive and should be updated after refactoring
    results = mooring_system.solve_static()
    assert results["Line1"][0].tension == pytest.approx(323018.156)
    assert results["Line2"][0].tension == pytest.approx(323018.156)
    assert results["Line1"][-1].coords.x == 30.0
    assert results["Line1"][-1].coords.y == 30.0
    assert results["Line1"][-1].coords.z == -25.0
