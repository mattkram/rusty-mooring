class GeneralConfig:
    units: str
    gravity: float
    water_density: float

    def __init__(self, units: str, gravity: float, water_density: float): ...

class Config:
    general: GeneralConfig

    @classmethod
    def from_file(cls, filename: str) -> Config: ...
