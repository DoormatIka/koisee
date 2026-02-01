
import pprint
from typing import Protocol
import tracemalloc

class BlankLogger(Protocol):
    def info(self, s: str) -> None: ...
    def warn(self, s: str) -> None: ...
    def match(self, s: str) -> None: ...
    def point(self) -> None: ...

class MatchLogger:
    def info(self, s: str) -> None: # pyright: ignore[reportUnusedParameter]
        pass
    def warn(self, s: str) -> None: # pyright: ignore[reportUnusedParameter]
        pass
    def match(self, s: str) -> None:
        print(f"[MATCH] - {s}")

class Logger: # color code per log level: info, warn, match
    previous_snapshot: tracemalloc.Snapshot | None = None
    def __init__(self):
        tracemalloc.start()

    def info(self, s: str) -> None:
        print(f"[INFO] - {s}")
    def warn(self, s: str) -> None:
        print(f"[WARN] - {s}")
    def match(self, s: str) -> None:
        print(f"[MATCH] - {s}")
    def point(self) -> None:
        snapshot = tracemalloc.take_snapshot()
        if self.previous_snapshot != None:
            pprint.pprint(self.previous_snapshot.compare_to(snapshot, key_type="lineno"))

        self.previous_snapshot = snapshot


