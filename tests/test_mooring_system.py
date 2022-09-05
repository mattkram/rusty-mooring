from pathlib import Path

from rusty_mooring import MooringSystem


def test_load_mooring_system_from_file(config_file: Path) -> None:
    """We can load the whole system config from a file via the MooringSystem class."""
    system = MooringSystem.from_file(config_file.as_posix())
    # Just check one deeply nested attribute. The rest are covered in test_config.py
    assert system.config.line_types["chain"].diameter == 0.127
