class Config:
    general: GeneralConfig
    line_type: dict[str, LineType]

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
