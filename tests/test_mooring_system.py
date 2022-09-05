from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import pytest

from rusty_mooring import Config
from rusty_mooring import MooringSystem

if TYPE_CHECKING:
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