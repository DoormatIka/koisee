
from typing import cast
import flet as ft

from gui.components.card_list import FileCardList
from gui.components.file_picker import FilePicker

from gui.router.observer import AppState, EventBus
from gui.payload_types import DirectoryResult, SelectedImageResult

"""
Hello! This code uses an event bus to pass data around the UI.
"""

def entry_page(page: ft.Page, bus: EventBus):
    manage_app_errors = make_manage_errors(page)

    bus.subscribe("directory", manage_directory)
    bus.subscribe("selected_images", manage_selected_images)
    bus.subscribe("SEVERE_APP_ERROR", manage_app_errors)

    col = ft.Column(
        expand=True,
        alignment=ft.MainAxisAlignment.SPACE_BETWEEN,
        controls=[
            FileCardList(
                expand=True,
                bus=bus
            ),
            FilePicker(bus=bus),
        ],
    )

    return ft.Container(
        content=col,
        expand=True,
    )

def make_manage_errors(page: ft.Page):
    snack_bar = ft.SnackBar(
        content=ft.Text("Error: "),
        action="Close",
        on_action=lambda _: page.pop_dialog(),
        bgcolor=ft.Colors.RED_100
    )

    def manage_app_errors(_: AppState, payload: object):
        snack_bar.content = ft.Text(f"Error: {payload}")
        if not snack_bar.open:
            page.show_dialog(snack_bar)
        page.update() # pyright: ignore[reportUnknownMemberType]

    return manage_app_errors

def manage_directory(state: AppState, payload: object):
    if isinstance(payload, DirectoryResult):
        state.directory = payload

def manage_selected_images(state: AppState, payload: object):
    cmd, model_image = cast(SelectedImageResult, payload)
    if cmd == "add":
        state.selected_images.add(model_image)
    if cmd == "delete":
        state.selected_images.discard(model_image)


