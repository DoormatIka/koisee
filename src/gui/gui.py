
import flet as ft

from gui.router.router import Router
from gui.views.entry import entry_page


async def flet_main(page: ft.Page):
    router = Router(page=page)
    router.add_route(route="/", container=entry_page())

    # works properly.
    page.views.append(ft.View(route="/", controls=[ft.Text(value="hi")]))
    # check if self.page.views and page.views are in the same memory.

    # print(page.route)
    await page.push_route(page.route)

