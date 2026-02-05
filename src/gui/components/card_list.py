
from pathlib import Path
from typing import Any
import flet as ft

from gui.router.observer import Observer
from gui.components.card_images import ImageCardRow
from wrappers import clusterer


class FileCardList(ft.Container):
    content: ft.Control | None
    _column: ft.Column
    _observer: Observer
    def __init__(
        self, 
        observer: Observer,
        width: float | None = None,
        height: float | None = None,
        expand: bool | None = None,
        scroll: ft.ScrollMode | None = None,
        **kwargs: Any # pyright: ignore[reportAny, reportExplicitAny]
    ):
        super().__init__(
            width=width, 
            height=height,
            expand=expand,
            **kwargs # pyright: ignore[reportAny]
        )
        observer.subscribe("directory", self.create_matches)

        self._column = ft.Column(
            scroll=scroll,
            controls=[]
        )
        self.content = self._column
        self._observer = observer

    async def create_matches(self, directory: object):
        self._column.controls.clear()

        if isinstance(directory, str):
            image_matches = await clusterer(Path(directory))
            for pair in image_matches:
                row = ImageCardRow(pair)
                self._column.controls.append(row)

        self.update()

