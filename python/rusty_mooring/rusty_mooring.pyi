def sum_as_string(a: int, b: int) -> str: ...


class Config:
    ip: str
    port: int

    @classmethod
    def from_file(cls, filename: str) -> Config: ...

    def __init__(self, ip: str, port: int): ...
