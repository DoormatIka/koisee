
from typing import Any
import flet as ft
import pprint

class FilePicker(ft.Container):
    content: ft.Control | None
    def __init__(
        self, 
        width: float | None = None,
        height: float | None = None,
        expand: bool | None = None,
        **kwargs: Any # pyright: ignore[reportAny, reportExplicitAny]
    ):
        super().__init__(
            width=width, 
            height=height,
            expand=expand,
            **kwargs # pyright: ignore[reportAny]
        )
        btn = ft.Button(
            content="Pick folder",
            icon=ft.Icons.UPLOAD_FILE,
            on_click=self.get_dir,
            style=ft.ButtonStyle(
                padding=ft.Padding.only(left=20, right=20)
            )
        )
        self.content = btn

    async def get_dir(self):
        dir_path = await ft.FilePicker().get_directory_path(dialog_title="pick folder to scan")
        pprint.pprint(dir_path)
        return dir_path

