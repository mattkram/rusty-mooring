from pathlib import Path

import pytest

from rusty_mooring import Config
from rusty_mooring import MooringSystem


@pytest.fixture()
def config(config_file: Path) -> Config:
    """The configuration, after being loaded from the TOML file."""
    config = Config.from_file(config_file.as_posix())
    return config


def test_load_mooring_system_from_file(config_file: Path) -> None:
    """We can load the whole system config from a file via the MooringSystem class."""
    system = MooringSystem.from_file(config_file.as_posix())
    assert system.config.line_types["chain"].diameter == 0.127


def test_load_mooring_system_from_config(config: Config) -> None:
    """We can instantiate a Mooring system directly from the config."""
    system = MooringSystem(config)
    assert system.config.line_types["chain"].diameter == 0.127
