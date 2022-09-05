from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import pytest

if TYPE_CHECKING:
    from _pytest.monkeypatch import MonkeyPatch


@pytest.fixture(autouse=True)
def tmp_cwd(tmp_path: Path, monkeypatch: MonkeyPatch) -> None:
    """Ensure we always run from a blank, temporary directory."""
    monkeypatch.chdir(tmp_path)
