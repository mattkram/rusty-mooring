from enum import Enum

class Config:
    general: GeneralConfig
    line_types: dict[str, LineType]
    lines: dict[str, Line]

    @classmethod
    def from_file(cls, filename: str) -> Config: ...

class Units(Enum):
    METRIC = ...
    ENGLISH = ...

class GeneralConfig:
    units: str
    gravity: float
    water_density: float

class LineType:
    diameter: float
    mass_per_length: float
    axial_stiffness: float

class LineSegment:
    line_type: str
    length: float
    num_elements: int

class Line:
    top_position: list[float]
    bottom_position: list[float]
    segments: list[LineSegment]

class MooringSystem:
    config: Config

    @classmethod
    def from_file(cls, filename: str) -> MooringSystem: ...
    def __init__(self, config: Config): ...
