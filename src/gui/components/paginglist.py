
from pathlib import Path
from typing import Any
import flet as ft

from src.gui.components.card_row import ImageCardRow
from src.gui.events import Directory, ImageUpdate
from src.gui.components.card_list import FileCardList
from src.gui.infra.app_bus import AppEventBus, AppState

class PagingList(ft.Container):
    content: ft.Control | None
    expand: bool | int | None

    _bus: AppEventBus
    _list: list[ft.Control]
    def __init__(
        self, 
        bus: AppEventBus,
        width: float | None = None,
        height: float | None = None,
        expand: bool | int | None = None,
        **kwargs: Any # pyright: ignore[reportAny, reportExplicitAny]
    ):
        super().__init__(
            width=width, 
            height=height,
            expand=expand,
            **kwargs # pyright: ignore[reportAny]
        )
        bus.subscribe(Directory, self.create_matches)

        self._bus = bus
        self._list = []
        self.expand = expand

    async def create_matches(self, state: AppState, obj: Directory):
        self._list.clear()

        if obj.directory is None:
            raise ValueError("Directory is null!")

        image_hashes = await state.finder.create_hashes_from_directory(Path(obj.directory))
        similar_images = state.finder.get_similar_objects(image_hashes)
        for pair in similar_images:
            row = ImageCardRow(self._bus, pair)
            self._list.append(row)

        await self._bus.notify(ImageUpdate(total=len(similar_images)))

        self.content = FileCardList(self._bus, self._list)
        self.update()

