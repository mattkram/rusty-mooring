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
    youngs_modulus: float

class LineSegment:
    line_type: str
    length: float
    num_elements: int

class Line:
    top_position: list[float]
    bottom_position: list[float]
    segments: list[LineSegment]

class Node:
    tension: float
    declination_angle: float
    arc_length: float
    x_corr: float
    y_corr: float

class MooringSystem:
    config: Config

    @classmethod
    def from_file(cls, filename: str) -> MooringSystem: ...
    def __init__(self, config: Config): ...
    def solve_static(self) -> dict[str, list[Node]]: ...
    def get_line_coordinates(self) -> dict[str, list[tuple[float, float, float]]]: ...
