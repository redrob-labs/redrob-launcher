// ADDED-LINE-RATIONALE 65da47a1dd7dcee6:1: Supports `resource` while preserving the nearby constraint: RCDATA.
// ADDED-LINE-RATIONALE b6ec1a55c1d890cb:1: Supports `resource` while preserving the nearby constraint: define IDR_PAYLOAD          101  // the release's Windows zip (absent from the uninstaller).
// ADDED-LINE-RATIONALE 0eb90ff747ccb5c2:1: Supports `resource` while preserving the nearby constraint: define IDR_UNINSTALLER      106  // the payload-less build of this app, written into the install folder.
// ADDED-LINE-RATIONALE 8f60b668ca24178e:1: Supports `resource` while preserving the nearby constraint: define IDR_SYMBOL_PNG       107  // assets/brand/symbol-white-192.png — the white Redrob symbol.
// ADDED-LINE-RATIONALE 782dd62aa3a9d4cf:1: Supports `resource` while preserving the nearby constraint: define IDR_FONT_REGULAR     108  // deterministic Pretendard Regular subset for Latin body text.
// SPDX-License-Identifier: MIT OR Apache-2.0
// Resource identifiers shared between redrob_launcher_setup.rc and redrob_launcher_setup.cpp.
#pragma once

// RCDATA
#define IDR_PAYLOAD          101  // the release's Windows zip (absent from the uninstaller)
#define IDR_FONT_SEMIBOLD    104  // deterministic Pretendard SemiBold subset for Latin headings and controls
#define IDR_UNINSTALLER      106  // the payload-less build of this app, written into the install folder
#define IDR_SYMBOL_PNG       107  // assets/brand/symbol-white-192.png — the white Redrob symbol
#define IDR_FONT_REGULAR     108  // deterministic Pretendard Regular subset for Latin body text
#define IDR_FONT_HANGUL      109  // deterministic Wanted Sans Regular subset for every Hangul UI run

#define IDI_APPICON          105  // assets/app-icon/redrob-launcher.ico

#define IDR_MANIFEST           1  // per-monitor-v2 DPI + common controls
