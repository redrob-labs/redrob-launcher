# dmgbuild settings for the launcher's disk image (packaging/macos/package.sh).
# Run as: dmgbuild -s dmg-settings.py -D app=<path to .app> -D background=<png> "<volume>" <out.dmg>
# The window is a drag-to-Applications layout like Chrome's or Notion's: the app on the left, the
# Applications link on the right, an arrow between them (dmg-background.svg; the @2x PNG is
# picked up next to the 1x one).
import os.path

app = defines["app"]  # noqa: F821 (dmgbuild provides `defines`)
app_name = os.path.basename(app)

format = "UDZO"
compression_level = 9
files = [app]
symlinks = {"Applications": "/Applications"}
hide_extension = [app_name]

background = defines["background"]  # noqa: F821
window_rect = ((200, 120), (640, 400))
default_view = "icon-view"
show_status_bar = False
show_tab_view = False
show_toolbar = False
show_pathbar = False
show_sidebar = False
icon_size = 128
text_size = 13
icon_locations = {app_name: (170, 190), "Applications": (470, 190)}
