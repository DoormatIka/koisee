
from typing import Protocol


class BlankLogger(Protocol):
    def info(self, s: str) -> None: ...
    def warn(self, s: str) -> None: ...
    def match(self, s: str) -> None: ...

class MatchLogger:
    def info(self, s: str) -> None: # pyright: ignore[reportUnusedParameter]
        pass
    def warn(self, s: str) -> None: # pyright: ignore[reportUnusedParameter]
        pass
    def match(self, s: str) -> None:
        print(f"[MATCH] - {s}")

class Logger: # color code per log level: info, warn, match
    def info(self, s: str) -> None:
        print(f"[INFO] - {s}")
    def warn(self, s: str) -> None:
        print(f"[WARN] - {s}")
    def match(self, s: str) -> None:
        print(f"[MATCH] - {s}")

