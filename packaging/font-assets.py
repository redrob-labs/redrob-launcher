#!/usr/bin/env python3
# ADDED-LINE-RATIONALE 5dc05bdca25cba93:1: Supports `font-assets` while preserving the nearby constraint: !/usr/bin/env python3.
# ADDED-LINE-RATIONALE b26d723ede15bf35:1: Makes `Requires / fontTools / only` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE fa889be5af877eea:1: Makes `builds / embed / the` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 5fbfc0df9cbfb955:1: Makes `builds / embed / the` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE d4a9180c9fef9973:1: Gives `font-assets` direct access to `__future__ / annotations` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 84f4c935893fb787:1: Gives `font-assets` direct access to `argparse` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 0c18217d5382db6b:1: Gives `font-assets` direct access to `hashlib` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE c5a31039ea92b3a7:1: Gives `font-assets` direct access to `importlib / metadata` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 23a4be87b99767bb:1: Gives `font-assets` direct access to `shutil` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE a19230434670f95c:1: Gives `font-assets` direct access to `tempfile` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE d7322d5526c44d4b:1: Gives `font-assets` direct access to `urllib / request` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 12fa48e3aff2cf0b:1: Gives `font-assets` direct access to `pathlib / Path` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 0fe927429d183c70:1: Freezes `FONTTOOLS_VERSION` for `font-assets` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 2e46cc609d153ceb:1: Freezes `PRETENDARD_VERSION` for `font-assets` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 107803310c62c8d1:1: Freezes `ROOT` for `font-assets` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE b0e1435ea9a9a360:1: Freezes `FONT_DIR` for `font-assets` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 4a35644e139a5c1f:1: Freezes `SOURCE_BASE` for `font-assets` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 2a78c60b571f6e02:1: Makes `https / raw / githubusercontent` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 05d228b5b71a38ab:1: Makes `f / v / PRETENDARD_VERSION` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 573db492d2be8314:1: Bounds the `font-assets` implementation here so `f / v / PRETENDARD_VERSION` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE 360f4676b68ca6bf:1: Freezes `SOURCES` for `font-assets` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE edcf4c620e57e724:1: Keeps `Pretendard / Regular / ttf` in `font-assets`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE b8258c66a1d5d070:1: Keeps `Pretendard / Medium / ttf` in `font-assets`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE 3244f945c737d159:1: Keeps `Pretendard / SemiBold / ttf` in `font-assets`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE 573de092d2becdd8:1: Bounds the `font-assets` implementation here so `Pretendard / SemiBold / ttf` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE 393f384e6dfc2745:1: Freezes `EXPECTED_WEIGHTS` for `font-assets` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 2e798948c636dae3:1: Keeps `Pretendard / Regular / ttf` in `font-assets`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE a3c85cc4fcf6e6a9:1: Keeps `Pretendard / Medium / ttf` in `font-assets`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE d9d6cfb8bef2e2bc:1: Keeps `Pretendard / SemiBold / ttf` in `font-assets`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE 573de092d2becdd8:2: Bounds the `font-assets` implementation here so `Pretendard / SemiBold / ttf` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE 7f8bd543cdf4ef4b:1: Supports `font-assets` while preserving the nearby constraint: Hangul cmap. Shared punctuation stays in Pretendard so mixed Latin text is visually coherent.
# ADDED-LINE-RATIONALE 847eddbf648686d1:1: Makes `x0020 / x02FF / Basic` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 2d6e82e8fd72b94c:1: Makes `x0300 / x036F / Combining` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 3ee674e57fbc7fd1:1: Makes `x1D00 / x1DFF / Phonetic` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 80c9e78c2b85b6f7:1: Makes `x1E00 / x1EFF / Latin` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE f4069635d7caf1f0:1: Makes `x2000 / x218F / Punctuation` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 57ff1c22875574e8:1: Makes `x2460 / x24FF / Enclosed` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE be4b7348bcd9d30d:1: Makes `x2C60 / x2C7F / Latin` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE e6f717df2e70de22:1: Makes `x2E00 / x2E7F / Supplemental` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE e912fe36083a5039:1: Makes `xA720 / xA7FF / Latin` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE c88bcd37e393254f:1: Makes `xAB30 / xAB6F / Latin` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 8671c045abbb9fdb:1: Delegates `font-assets`'s `xFF01 / xFF5E / Fullwidth` work to `forms` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE d4fdfe97dbeaa2b5:1: Makes `x10780 / x107BF / Latin` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 7d421bf067cb80c4:1: Makes `x1DF00 / x1DFFF / Latin` an explicit part of `font-assets` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 573db492d2be8314:2: Bounds the `font-assets` implementation here so `x1DF00 / x1DFFF / Latin` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE e58283f2098949be:1: Freezes `HANGUL_RANGES` for `font-assets` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 0fb6d3020ea38237:1: Isolates `sha256` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE 0435717e379b476e:1: Stops `sha256` at the `hashlib / sha256 / data` outcome so no later branch can overwrite that result.
# ADDED-LINE-RATIONALE 2eb37c1aad6bb5f9:1: Isolates `codepoints` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE cd8a9be8b6c8de47:1: Stops `codepoints` at the `value / start / end` outcome so no later branch can overwrite that result.
# ADDED-LINE-RATIONALE acea9ac144b53406:1: Isolates `fetch_sources` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE 2bcbeb4492cbf6e5:1: Delegates `fetch_sources`'s `destination / mkdir` work to `destination.mkdir` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE a1f37109364f82b2:1: Applies `fetch_sources` consistently to every `expected_hash / in / SOURCES` item rather than handling only the first match.
# ADDED-LINE-RATIONALE c471ad716bf93925:1: Freezes `request` for `fetch_sources` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 002e59ab3a7f2a4e:1: Keeps `f / SOURCE_BASE` in `fetch_sources`'s API-defined order so positional values retain their intended meaning.
# ADDED-LINE-RATIONALE 1ac801d40103f59e:1: Freezes `headers` for `fetch_sources` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE ebc49c4586ffabb4:1: Bounds the `fetch_sources` implementation here so `headers / User / Agent` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE ce32f139586c19a4:1: Delegates `fetch_sources`'s `urllib / request / urlopen` work to `urllib.request.urlopen` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE b1eee037113359b6:1: Freezes `data` for `fetch_sources` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 9494ec08a042eeb5:1: Freezes `actual_hash` for `fetch_sources` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE f3a01a5ede66d5e6:1: Freezes `if` for `fetch_sources` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 31dc2d4135acd1a5:1: Makes `fetch_sources` fail visibly when `raise / RuntimeError` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 2aed84b3e0c7ed0d:1: Makes `f / source / SHA` an explicit part of `fetch_sources` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE d75088a4335b6024:1: Bounds the `fetch_sources` implementation here so `f / source / SHA` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE 52d24868b015fcc8:1: Delegates `fetch_sources`'s `destination / write_bytes / data` work to `write_bytes` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 4fa0cd9a552c6ac0:1: Isolates `generate` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE 4dc2eac3c37bd8e0:1: Gives `generate` direct access to `fontTools / subset / type` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 8d790c31cabd29ee:1: Gives `generate` direct access to `fontTools / ttLib / TTFont` instead of relying on an implicit platform facility.
# ADDED-LINE-RATIONALE 2bcbeb4492cbf6e5:2: Delegates `generate`'s `destination / mkdir` work to `destination.mkdir` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE cdc4d4947a976d6e:1: Freezes `allowed` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 6273f6b0fef73394:1: Freezes `forbidden` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE ac9d81ff4a613314:1: Applies `generate` consistently to every `expected_weight / in / EXPECTED_WEIGHTS` item rather than handling only the first match.
# ADDED-LINE-RATIONALE a7b6f96d61606216:1: Freezes `source_path` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 2dfeaa31f5c6d7ad:1: Freezes `source` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE bbddffabe2c31c81:1: Freezes `source_cmap` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 8e4b2e016d66a8da:1: Freezes `weight` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE a106b2c221905de1:1: Delegates `generate`'s `source / close` work to `source.close` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 0335a71ca30d9431:1: Freezes `if` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 31dc2d4135acd1a5:2: Makes `generate` fail visibly when `raise / RuntimeError` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 0b993e0ad3111746:1: Makes `f / source / weight` an explicit part of `generate` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE d75088a4335b6024:2: Bounds the `generate` implementation here so `f / source / weight` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE 7b37760ba047e8d9:1: Freezes `retained` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE da745e2ce9579c34:1: Prevents `generate` from applying its side effect outside the `not / set / range` state.
# ADDED-LINE-RATIONALE 50fcff72c7c4f424:1: Makes `generate` fail visibly when `raise / RuntimeError / f` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 8257a227966217e7:1: Freezes `options` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE e7d5ad68b70672a8:1: Freezes `options` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 0aa22ad41c699180:1: Freezes `options` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 76ce443390286048:1: Freezes `options` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE fd7c4f553a5c3c3d:1: Freezes `options` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 21a313e09bf000ca:1: Freezes `options` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 7a1a359de6b5aa17:1: Freezes `options` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE a8a5fde269c5854b:1: Freezes `font` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 7973f0de9aa6ff97:1: Freezes `subsetter` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 5aff821b699a1670:1: Freezes `subsetter` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 4fcaf515ff1d0a6a:1: Delegates `generate`'s `subsetter / subset / font` work to `subsetter.subset` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE ba41a2a197551827:1: Freezes `output_path` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 60ec64a5b22f17df:1: Delegates `generate`'s `subset / save_font / font` work to `subset.save_font` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 962573c84bd4b528:1: Freezes `check` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 219565ffa0fc3212:1: Freezes `output_cmap` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE e7e2f281cc8c084b:1: Freezes `output_weight` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE c9eec95e862796e6:1: Delegates `generate`'s `check / close` work to `check.close` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 390a9dc137123fe5:1: Freezes `missing` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 991c1d49cf28c757:1: Freezes `claimed_hangul` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 1ccbcc2e3fa53c44:1: Prevents `generate` from applying its side effect outside the `missing` state.
# ADDED-LINE-RATIONALE 2835ebbca83a90db:1: Makes `generate` fail visibly when `raise / RuntimeError / f` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 1b8c86f44bc3be11:1: Prevents `generate` from applying its side effect outside the `claimed_hangul` state.
# ADDED-LINE-RATIONALE 31dc2d4135acd1a5:3: Makes `generate` fail visibly when `raise / RuntimeError` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 70c428441cd9cb60:1: Makes `f / generated / face` an explicit part of `generate` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE d75088a4335b6024:3: Bounds the `generate` implementation here so `f / generated / face` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE ee6306ecced5829f:1: Freezes `if` for `generate` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 31dc2d4135acd1a5:4: Makes `generate` fail visibly when `raise / RuntimeError` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 261a4767516a75e0:1: Makes `f / generated / weight` an explicit part of `generate` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE d75088a4335b6024:4: Bounds the `generate` implementation here so `f / generated / weight` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE b84f9de9e884ff41:1: Isolates `compare` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE 39494934273a682e:1: Applies `compare` consistently to every `in / SOURCES` item rather than handling only the first match.
# ADDED-LINE-RATIONALE e4ca91047f5f14aa:1: Freezes `left_bytes` for `compare` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 81cbe425d17b182c:1: Freezes `right_bytes` for `compare` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE af92e304f29f87dd:1: Freezes `if` for `compare` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 3ba3d43c3748d6d7:1: Makes `compare` fail visibly when `raise / RuntimeError / f` violates the behavior this path promises.
# ADDED-LINE-RATIONALE d8608270d243e9a5:1: Isolates `main` as the owner of this behavior so callers can use and test it without duplicating policy.
# ADDED-LINE-RATIONALE 7ad995db48fdd656:1: Freezes `parser` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 3f2abb353925668d:1: Delegates `main`'s `parser / add_argument` work to `parser.add_argument` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 47093e97b8e5eaf1:1: Makes `check / action / store_true` an explicit part of `main` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE 066e13d250c84584:1: Bounds the `main` implementation here so `check / action / store_true` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE d7579cd110221a72:1: Freezes `args` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 21bb4ef59528effe:1: Freezes `installed` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 86c65ea1a5103edd:1: Freezes `if` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 89d3a53b85ee4d75:1: Makes `main` fail visibly when `raise / RuntimeError` violates the behavior this path promises.
# ADDED-LINE-RATIONALE 79d3b6f339ff4450:1: Makes `f / fontTools / FONTTOOLS_VERSION` an explicit part of `main` because packaging produces reproducible artifacts without platform-dependent fallbacks.
# ADDED-LINE-RATIONALE ebc49c4586ffabb4:2: Bounds the `main` implementation here so `f / fontTools / FONTTOOLS_VERSION` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE 8074c48d42ac0456:1: Freezes `with` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 8f36d53ea5c3443f:1: Freezes `temp` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 5947f9bed6787c01:1: Freezes `source_dir` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE a521c71997e80bdf:1: Freezes `first` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE af20a0c3eda46a02:1: Delegates `main`'s `fetch_sources / source_dir` work to `fetch_sources` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 53691cbb6ede6706:1: Delegates `main`'s `generate / source_dir / first` work to `generate` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 5a330d06cf8e05f9:1: Prevents `main` from applying its side effect outside the `args / check` state.
# ADDED-LINE-RATIONALE 80c893a37a6eebef:1: Freezes `second` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 01e8e240e1e8e4ea:1: Delegates `main`'s `generate / source_dir / second` work to `generate` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 0ec2e190c7f31a7e:1: Delegates `main`'s `compare / first / second` work to `compare` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 9d1d4cadb2a53d34:1: Delegates `main`'s `compare / first / FONT_DIR` work to `compare` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 274ab0a48a091a2c:1: Delegates `main`'s `print / Pretendard / subsets` work to `print` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 7ac5f6a9c5caa74e:1: Prevents `main` from applying its side effect outside the `print / Pretendard / subsets` state.
# ADDED-LINE-RATIONALE ba905613ea5f288e:1: Applies `main` consistently to every `in / SOURCES` item rather than handling only the first match.
# ADDED-LINE-RATIONALE 2623cf89e229109d:1: Delegates `main`'s `shutil / copyfile / first` work to `shutil.copyfile` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 781bd00947dffb5d:1: Freezes `data` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE ecb31ab078b99930:1: Delegates `main`'s `print` work to `print` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 0e9287153d0f053f:1: Delegates `main`'s `f / wrote / assets` work to `len` so existing state and error handling remain centralized.
# ADDED-LINE-RATIONALE 5df00fda6048ced4:1: Bounds the `main` implementation here so `f / wrote / assets` cannot leak into the adjacent declaration or control path.
# ADDED-LINE-RATIONALE 9269fc1ea45dc012:1: Freezes `if` for `main` so every later calculation in this path observes the same input.
# ADDED-LINE-RATIONALE 8c06c079e198556b:1: Delegates `main`'s `main` work to `main` so existing state and error handling remain centralized.
"""Generate the committed Latin-only Pretendard faces used by egui.

Requires fontTools 4.67.0 only when regenerating or checking assets; normal Rust
builds embed the committed outputs and have no generator dependency.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.metadata
import shutil
import tempfile
import urllib.request
from pathlib import Path

FONTTOOLS_VERSION = "4.67.0"
PRETENDARD_VERSION = "1.3.9"
ROOT = Path(__file__).resolve().parents[1]
FONT_DIR = ROOT / "assets" / "fonts"
SOURCE_BASE = (
    "https://raw.githubusercontent.com/orioncactus/pretendard/"
    f"v{PRETENDARD_VERSION}/packages/pretendard/dist/public/static/alternative"
)
SOURCES = {
    "Pretendard-Regular.ttf": "6d0af5258997aec7354a6e340fc2325ba321c410ca48b3af858c8c3d6e92a324",
    "Pretendard-Medium.ttf": "3bae579377eb8e9ac412cb4809ebc3de1d956ed75995c1e346f0c1311053f4e2",
    "Pretendard-SemiBold.ttf": "5e1c548732af70873103066c16e1369b9a8a871f0b38c321a1d5bc73e43cea2d",
}
EXPECTED_WEIGHTS = {
    "Pretendard-Regular.ttf": 400,
    "Pretendard-Medium.ttf": 500,
    "Pretendard-SemiBold.ttf": 600,
}
# Keep broad Latin language coverage and shared typography without retaining the source fonts'
# Hangul cmap. Shared punctuation stays in Pretendard so mixed Latin text is visually coherent.
LATIN_RANGES = (
    (0x0020, 0x02FF),  # Basic Latin through spacing modifier letters
    (0x0300, 0x036F),  # Combining diacritical marks
    (0x1D00, 0x1DFF),  # Phonetic extensions and combining supplement
    (0x1E00, 0x1EFF),  # Latin Extended Additional
    (0x2000, 0x218F),  # Punctuation, currency, letterlike symbols, and number forms
    (0x2460, 0x24FF),  # Enclosed alphanumerics
    (0x2C60, 0x2C7F),  # Latin Extended-C
    (0x2E00, 0x2E7F),  # Supplemental punctuation
    (0xA720, 0xA7FF),  # Latin Extended-D
    (0xAB30, 0xAB6F),  # Latin Extended-E
    (0xFF01, 0xFF5E),  # Fullwidth ASCII forms (not halfwidth Hangul)
    (0x10780, 0x107BF),  # Latin Extended-F
    (0x1DF00, 0x1DFFF),  # Latin Extended-G
)
HANGUL_RANGES = ((0x1100, 0x11FF), (0x3130, 0x318F), (0xAC00, 0xD7A3))


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def codepoints(ranges: tuple[tuple[int, int], ...]) -> set[int]:
    return {value for start, end in ranges for value in range(start, end + 1)}


def fetch_sources(destination: Path) -> None:
    destination.mkdir()
    for name, expected_hash in SOURCES.items():
        request = urllib.request.Request(
            f"{SOURCE_BASE}/{name}",
            headers={"User-Agent": "Redrob-Launcher/font-assets"},
        )
        with urllib.request.urlopen(request) as response:
            data = response.read()
        actual_hash = sha256(data)
        if actual_hash != expected_hash:
            raise RuntimeError(
                f"{name}: source SHA-256 is {actual_hash}, expected {expected_hash}"
            )
        (destination / name).write_bytes(data)


def generate(source_dir: Path, destination: Path) -> None:
    from fontTools import subset  # type: ignore[import-untyped]
    from fontTools.ttLib import TTFont  # type: ignore[import-untyped]

    destination.mkdir()
    allowed = codepoints(LATIN_RANGES)
    forbidden = codepoints(HANGUL_RANGES)
    for name, expected_weight in EXPECTED_WEIGHTS.items():
        source_path = source_dir / name
        source = TTFont(source_path, lazy=True)
        source_cmap = source.getBestCmap() or {}
        weight = source["OS/2"].usWeightClass
        source.close()
        if weight != expected_weight:
            raise RuntimeError(
                f"{name}: source weight is {weight}, expected {expected_weight}"
            )
        retained = allowed.intersection(source_cmap)
        if not set(range(0x20, 0x7F)).issubset(retained):
            raise RuntimeError(f"{name}: source does not contain printable ASCII")

        options = subset.Options()
        options.flavor = None
        options.layout_features = ["*"]
        options.name_IDs = ["*"]
        options.notdef_outline = True
        options.recalc_timestamp = False
        options.canonical_order = True
        font = subset.load_font(str(source_path), options)
        subsetter = subset.Subsetter(options)
        subsetter.populate(unicodes=retained)
        subsetter.subset(font)
        output_path = destination / name
        subset.save_font(font, str(output_path), options)

        check = TTFont(output_path, lazy=True, recalcTimestamp=False)
        output_cmap = check.getBestCmap() or {}
        output_weight = check["OS/2"].usWeightClass
        check.close()
        missing = sorted(retained.difference(output_cmap))
        claimed_hangul = sorted(forbidden.intersection(output_cmap))
        if missing:
            raise RuntimeError(f"{name}: generated face lost U+{missing[0]:04X}")
        if claimed_hangul:
            raise RuntimeError(
                f"{name}: generated face still claims U+{claimed_hangul[0]:04X}"
            )
        if output_weight != expected_weight:
            raise RuntimeError(
                f"{name}: generated weight is {output_weight}, expected {expected_weight}"
            )


def compare(left: Path, right: Path, description: str) -> None:
    for name in SOURCES:
        left_bytes = (left / name).read_bytes()
        right_bytes = (right / name).read_bytes()
        if left_bytes != right_bytes:
            raise RuntimeError(f"{name}: {description}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check", action="store_true", help="verify committed outputs and idempotence"
    )
    args = parser.parse_args()

    installed = importlib.metadata.version("fonttools")
    if installed != FONTTOOLS_VERSION:
        raise RuntimeError(
            f"fontTools {FONTTOOLS_VERSION} is required for deterministic output; found {installed}"
        )

    with tempfile.TemporaryDirectory(prefix="redrob-fonts-") as temporary:
        temp = Path(temporary)
        source_dir = temp / "source"
        first = temp / "first"
        fetch_sources(source_dir)
        generate(source_dir, first)
        if args.check:
            second = temp / "second"
            generate(source_dir, second)
            compare(first, second, "two generations differ")
            compare(first, FONT_DIR, "committed output is stale")
            print("Pretendard subsets are current and byte-for-byte reproducible")
        else:
            for name in SOURCES:
                shutil.copyfile(first / name, FONT_DIR / name)
                data = (FONT_DIR / name).read_bytes()
                print(
                    f"wrote assets/fonts/{name} ({len(data)} bytes, sha256 {sha256(data)})"
                )


if __name__ == "__main__":
    main()
