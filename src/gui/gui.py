
import flet as ft
import pprint

from gui.components.file_picker import FilePicker
from gui.models.image import ModelImage
from hashers.types import CombinedImageHash


weight_clrs = {
    ft.Colors.AMBER_800: 5,
    ft.Colors.BLUE_800: 10,
}

def toggle_delete(e: ft.Event[ft.Container], model: ModelImage):
    is_deleted = model.toggle_delete()
    if is_deleted:
        e.control.border = ft.border.all(2, ft.Colors.RED)
    else:
        e.control.border = None

    e.control.update()

def create_pair_image(matched_images: list[CombinedImageHash]) -> ft.Container:
    containers: list[ft.Control] = []

    for img in matched_images:
        model = ModelImage(hash=img)
        container = ft.Container(
            content=ft.Image(
                src=str(img.path),
                height=150,
                fit=ft.BoxFit.COVER,
                expand=1,
            ),
            on_click=lambda ev: toggle_delete(ev, model)
        )
        containers.append(container)

    img_row = ft.Row(
        controls=containers,
        alignment=ft.MainAxisAlignment.START,
        scroll=ft.ScrollMode.ADAPTIVE,
        width=float("inf"),
    )

    c = ft.Column(
        controls=[img_row]
    )
    return ft.Container(
        content=c,
        padding=10,
        border_radius=5,
        bgcolor=ft.Colors.with_opacity(0.1, ft.Colors.GREY_300),
    )


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

def flet_main(page: ft.Page):
    col = ft.Column(
        alignment=ft.MainAxisAlignment.SPACE_BETWEEN,
        controls=[
            create_pair_image([]),
            FilePicker()
        ],
    )

    c = ft.Container(
        alignment=ft.Alignment.CENTER,
        content=col,
        expand=True,
    )

    page.add(c)


