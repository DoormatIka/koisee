
from collections.abc import Awaitable, Callable, Collection
from dataclasses import dataclass, field
import inspect
from typing import Literal

from gui.models.image import ModelImage

@dataclass
class AppState:
    directory: str | None = None
    images: Collection[ModelImage] = field(default_factory=list)

StateKey = Literal["directory", "images"]
# add AppState to Callable soon.
ObserverFn = Callable[[object], None | Awaitable[None]]

class Observer:
    state: AppState
    _fns: dict[StateKey, list[ObserverFn]]
    def __init__(self, state: AppState):
        self.state = state
        self._fns = {}
    def subscribe(self, key: StateKey, on_key: ObserverFn):
        if key not in self._fns:
            self._fns[key] = []
        self._fns[key].append(on_key)

    async def notify(self, key: StateKey, payload: object):
        setattr(self.state, key, payload)
        
        if key in self._fns:
            for fn in self._fns[key]:
                if inspect.iscoroutinefunction(fn):
                    await fn(payload)
                else:
                    _ = fn(payload)
