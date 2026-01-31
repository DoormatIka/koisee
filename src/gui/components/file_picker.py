
import flet as ft
import pprint

def file_picker_component():
    async def get_dir():
        dir_path = await ft.FilePicker().get_directory_path(dialog_title="pick folder to scan")
        pprint.pprint(dir_path)
        return dir_path

    btn = ft.Button(
        content="Pick folder",
        icon=ft.Icons.UPLOAD_FILE,
        on_click=get_dir,
        style=ft.ButtonStyle(
            padding=ft.Padding.only(left=20, right=20)
        )
    )

    return ft.Container(
        content=btn,
    )
