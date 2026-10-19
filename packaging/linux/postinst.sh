#!/bin/sh
# deb/rpm post-install and post-remove: refresh the desktop and icon caches. The desktop
# database is what makes the entry's MimeType=x-scheme-handler/redrob (redrob:// links) known.
# Every tool is optional; missing ones are skipped.
if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database -q /usr/share/applications || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi
exit 0
