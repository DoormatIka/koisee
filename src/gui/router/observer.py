
from collections.abc import Awaitable, Callable
from dataclasses import dataclass, field
import inspect
from typing import Any, TypeVar

from gui.payload_types import Event, SelectedPayload, SevereAppError



@dataclass
class AppState:
    directory: str | None = None
    total_images: int = field(default_factory=int)
    selected_images: dict[str, SelectedPayload] = field(default_factory=dict)


EventT = TypeVar("EventT", bound=Event)
Observer = Callable[[AppState, EventT], None | Awaitable[None]]
RuntimeObserver = Callable[[AppState, Any], Any] # pyright: ignore[reportExplicitAny]

class EventBus:
    state: AppState
    _fns: dict[type[Event], list[RuntimeObserver]]
    def __init__(self, state: AppState):
        self.state = state
        self._fns = {}
    def subscribe(self, event: type[EventT], handler: Observer[EventT]) -> None:
        if event not in self._fns:
            self._fns[event] = []
        self._fns[event].append(handler)

    async def notify(self, event: Event):
        handlers = list(self._fns.get(type(event), []))
        if not handlers:
            return

        for fn in handlers:
            try:
                if inspect.iscoroutinefunction(fn):
                    await fn(self.state, event)
                else:
                    _ = fn(self.state, event) # pyright: ignore[reportAny]
            except Exception as e:
                if isinstance(event, SevereAppError):
                    print(f"Recursed SEVERE_APP_ERROR handler exception: ", e)
                else:
                    await self.notify(SevereAppError(e))
