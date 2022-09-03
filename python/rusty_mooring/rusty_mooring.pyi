class GeneralConfig:
    units: str
    gravity: float
    water_density: float

    @classmethod
    def from_file(cls, filename: str) -> GeneralConfig: ...
    def __init__(self, units: str, gravity: float, water_density: float): ...
