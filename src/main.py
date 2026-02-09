import multiprocessing
import flet as ft

from gui import flet_config, flet_main

if __name__ == "__main__":
    multiprocessing.set_start_method("spawn", force=True)
    multiprocessing.freeze_support()

    _ = ft.run( # pyright: ignore[reportUnknownVariableType, reportUnknownMemberType]
        before_main=flet_config, 
        main=flet_main,
        assets_dir="img"
    ) 

