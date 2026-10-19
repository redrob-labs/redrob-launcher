// SPDX-License-Identifier: MIT OR Apache-2.0
// Resource identifiers shared between redrob_launcher_setup.rc and redrob_launcher_setup.cpp.
#pragma once

// RCDATA
#define IDR_PAYLOAD          101  // the release's windows zip (absent from the uninstaller)
#define IDR_FONT_SANS        104  // Pretendard SemiBold (assets/fonts, cut to the window's characters), when fontTools is present
#define IDR_FONT_BODY        108  // Pretendard Regular, the same way: body copy and labels
#define IDR_UNINSTALLER      106  // the payload-less build of this app, written into the install folder
#define IDR_SYMBOL_PNG       107  // assets/brand/symbol-white-192.png -- the white Redrob symbol, for the brand panel

#define IDI_APPICON          105  // assets/app-icon/redrob-launcher.ico

#define IDR_MANIFEST           1  // per-monitor-v2 DPI + common controls
