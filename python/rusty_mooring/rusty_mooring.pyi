class Config:
    general: GeneralConfig
    line_type: dict[str, LineType]
    lines: dict[str, Line]

    @classmethod
    def from_file(cls, filename: str) -> Config: ...

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
