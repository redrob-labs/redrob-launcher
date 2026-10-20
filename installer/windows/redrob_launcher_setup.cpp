// ADDED-LINE-RATIONALE e3ef39a27349bc04:1: Supports `NOMINMAX` while preserving the nearby constraint: include <dwrite.h>.
// ADDED-LINE-RATIONALE 9ca453fc3dc56456:1: Supports `NOMINMAX` while preserving the nearby constraint: include <propkey.h>.
// ADDED-LINE-RATIONALE 07a882bd17e427df:1: Supports `NOMINMAX` while preserving the nearby constraint: include <memory>.
// ADDED-LINE-RATIONALE 0bfb3f525ea26b89:1: Supports `NOMINMAX` while preserving the nearby constraint: pragma comment(lib, "propsys.lib").
// ADDED-LINE-RATIONALE 5122800b34498b7a:1: Supports `NOMINMAX` while preserving the nearby constraint: pragma comment(lib, "uiautomationcore.lib").
// ADDED-LINE-RATIONALE 3dad51ce1411fa8d:1: Supports `NOMINMAX` while preserving the nearby constraint: -- design-system colors (@redrob-labs/ui semantic roles) ---------.
// ADDED-LINE-RATIONALE a214739aa78dc927:1: Stops `NOMINMAX` at the `v / xFF / f` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:1: Bounds the `NOMINMAX` implementation here so `v / xFF / f` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f4955f5cf9362c9b:1: Supports `Palette` while preserving the nearby constraint: One set per theme. The window follows Windows' app theme (light or dark), like the launcher.
// ADDED-LINE-RATIONALE 5a43b560c0da2672:1: Makes `D2D1_COLOR_F / base / surface` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE f27a366692900f43:1: Makes `D2D1_COLOR_F / sunken / surface` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 78fb20e538e48816:1: Makes `D2D1_COLOR_F / ink / ink` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 0c39607fbf7a9674:1: Makes `D2D1_COLOR_F / secondary / ink` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE ef9bc79f4704e415:1: Makes `D2D1_COLOR_F / hairline / border` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 656ca612c328bd64:1: Delegates `Palette`'s `D2D1_COLOR_F / strong / border` work to `outlines` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f5b79fe94b2fabf2:1: Makes `D2D1_COLOR_F / blue / action` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE d0290fdac8ca7d85:1: Delegates `Palette`'s `D2D1_COLOR_F / blueHover / action` work to `action-primary-hover` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0dce2216a09781fc:1: Makes `D2D1_COLOR_F / focus / focus` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 47d2334d67d2eb83:1: Makes `D2D1_COLOR_F / danger / status` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE e3c45085d6744ea1:1: Makes `D2D1_COLOR_F / inkBrand / ink` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE d2ca515c8dbdc7f8:1: Makes `D2D1_COLOR_F / brandSubtle / surface` an explicit part of `Palette` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 583f446896282ed0:1: Bounds the `Palette` implementation here so `D2D1_COLOR_F / brandSubtle / surface` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 696b67ae6d489e53:1: Supports `Palette` while preserving the nearby constraint: Generated semantic roles and fixed brand-art exceptions keep native colours in lockstep.
// ADDED-LINE-RATIONALE 61d9261f4b143a13:1: Supports `Palette` while preserving the nearby constraint: include "generated_tokens.h".
// ADDED-LINE-RATIONALE d5c94ffef19cf8d0:1: Freezes `int` for `Palette` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fe75ea1c16b01483:1: Delegates `Palette`'s `inline / Palette / Pal` work to `Pal` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 68d57d1a3f5c8f42:1: Supports `Palette` while preserving the nearby constraint: a -> b by t (0..1): hover and pressed states blend between two roles.
// ADDED-LINE-RATIONALE 3a1a9848ace4269d:1: Stops `Palette` at the `a / r / b` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:2: Bounds the `Palette` implementation here so `a / r / b` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8c6b205810c8d438:1: Supports `SystemPrefersDark` while preserving the nearby constraint: Windows' app theme: HKCU\...\Themes\Personalize\AppsUseLightTheme (0 = dark). Read only. Light if unset.
// ADDED-LINE-RATIONALE 8c95ebb15e981c9e:1: Freezes `if` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 565211b11cac926e:1: Freezes `if` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f4085061a092ae6c:1: Freezes `DWORD` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d08a4f08a89dc557:1: Prevents `SystemPrefersDark` from applying its side effect outside the `RegGetValueW / HKEY_CURRENT_USER / L` state.
// ADDED-LINE-RATIONALE 21083f029b6793dc:1: Freezes `L` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a9006309a58ee8da:1: Stops `SystemPrefersDark` at the `L / AppsUseLightTheme / RRF_RT_REG_DWORD` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE b9305a9c5019562c:1: Freezes `return` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:3: Bounds the `SystemPrefersDark` implementation here so `value` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cceee262e1a4d07f:1: Supports `SystemPrefersDark` while preserving the nearby constraint: -- geometry (logical px at 96 DPI) --------------------.
// ADDED-LINE-RATIONALE 0dfb12932c6bc30d:1: Freezes `int` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 801a270ab6d5b203:1: Freezes `float` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3d2c03b3a1c7ad49:1: Freezes `float` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d485960e08fcfa90:1: Freezes `float` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 569993057ee45a55:1: Freezes `float` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fa1b3fcd6316a3aa:1: Freezes `UINT_PTR` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5d108ec78dc2981d:1: Freezes `float` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 039a1b9642edbb27:1: Freezes `float` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 53c0a50e602b12a7:1: Freezes `UINT_PTR` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE afcebac4ce84a730:1: Freezes `UINT_PTR` for `SystemPrefersDark` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 33479fad9bc60faf:1: Isolates `class` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE 7a150c73c405234a:1: Isolates `A11yProvider` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE 3586b971ed1c22fa:1: Isolates `A11yProvider` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE 164142345e6f4bff:1: Delegates `RaiseStructureChanged`'s `friend / RaiseStructureChanged / SetupApp` work to `RaiseStructureChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 09d01e5985d0974c:1: Delegates `RaiseToggleChanged`'s `friend / RaiseToggleChanged / SetupApp` work to `RaiseToggleChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6ec7ed628d50ad35:1: Delegates `RaiseFocusChanged`'s `friend / RaiseFocusChanged / SetupApp` work to `RaiseFocusChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 68cad51c8c38267d:1: Delegates `RaiseProgressChanged`'s `friend / RaiseProgressChanged / SetupApp` work to `RaiseProgressChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e4a9c4df028c578c:1: Delegates `ApplyHangulFont`'s `ApplyHangulFont / IDWriteTextLayout / layout` work to `ApplyHangulFont` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 73b5e2d2a510555b:1: Delegates `DrawTextLine`'s `DrawTextLine / std / wstring` work to `DrawTextLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5f397a6f898da092:1: Freezes `bool` for `Uninstalling` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9163642b8ef652b3:1: Makes `ComPtr / ID2D1Bitmap / symbolBitmap_` an explicit part of `ChooseLanguage` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 94eeae4d024382b9:1: Makes `ComPtr / IDWriteTextFormat / fmtTitle_` an explicit part of `ChooseLanguage` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 9efe6bd4dde9db72:1: Makes `ComPtr / IDWriteTextFormat / fmtBody_` an explicit part of `ChooseLanguage` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 511a06cccf2e0323:1: Makes `ComPtr / IDWriteTextFormat / fmtButton_` an explicit part of `ChooseLanguage` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE a3f9a669d9d7be53:1: Makes `ComPtr / IDWriteTextFormat / fmtLabel_` an explicit part of `ChooseLanguage` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 3dcf4eb5887a7b6f:1: Makes `ComPtr / IDWriteTextFormat / fmtStep_` an explicit part of `ChooseLanguage` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 5d92fc420d6c44b6:1: Makes `ComPtr / IDWriteTextFormat / fmtBrand_` an explicit part of `ChooseLanguage` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 973d09f575e3123a:1: Freezes `std` for `ChooseLanguage` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 26b5370bd0756220:1: Makes `std / wstring / hangulFamily_` an explicit part of `ChooseLanguage` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 7373dc2d1f27c6cb:1: Freezes `bool` for `ChooseLanguage` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ef34ee7ae90e552e:1: Supports `ChooseLanguage` while preserving the nearby constraint: drawing helpers (DrawScene).
// ADDED-LINE-RATIONALE b3cd36ee31ce748c:1: Delegates `ChooseLanguage`'s `float / DrawBlock / std` work to `DrawBlock` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8e6fa13c6b44001b:1: Delegates `DrawBrandPanel`'s `DrawBrandPanel / float / H` work to `DrawBrandPanel` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a7f292f9012b5248:1: Delegates `DrawStepper`'s `DrawStepper / float / x` work to `DrawStepper` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fe8cb3cb0a71a30d:1: Delegates `ApplyWindowTheme`'s `ApplyWindowTheme` work to `ApplyWindowTheme` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 58c949d695b6c8e8:1: Makes `RECT / rcBrand_ / rcStepper_` an explicit part of `ApplyWindowTheme` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE d9ca5e4a6a403c0c:1: Supports `Ctl` while preserving the nearby constraint: ErrClose = the secondary one (한국어 there), Check = the checkbox row.
// ADDED-LINE-RATIONALE 1c86ec1cd8517cd7:1: Freezes `int` for `Ctl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bfd7b50b6b35f115:1: Freezes `int` for `Ctl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fa9d8eb6f95c05aa:1: Freezes `int` for `Ctl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8b15c3e0b4076b51:1: Freezes `bool` for `Ctl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 04139626c8df2802:1: Freezes `bool` for `Ctl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 74d1d4bab994fdac:1: Freezes `bool` for `Ctl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b15b976d2a63a71d:1: Freezes `float` for `Ctl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 19c208de942892ab:1: Freezes `ULONGLONG` for `Ctl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2c0fbf2156942caf:1: Delegates `HitTest`'s `int / HitTest / POINT` work to `HitTest` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 53482539684b98d3:1: Delegates `Activate`'s `Activate / int / ctl` work to `Activate` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9aefc2de89138f51:1: Delegates `UpdateHover`'s `UpdateHover` work to `UpdateHover` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3117733728652599:1: Delegates `StartAnim`'s `StartAnim` work to `StartAnim` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 36cd4c35220b2f9f:1: Delegates `AnimStep`'s `bool / AnimStep / advance` work to `AnimStep` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c57bac300270a24d:1: Delegates `AnimStep`'s `std / vector / int` work to `Focusables` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5c7863d28c551bbd:1: Delegates `IsFocusable`'s `bool / IsFocusable / int` work to `IsFocusable` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 903df012d1a7a0f8:1: Delegates `WindowHasKeyboardFocus`'s `bool / WindowHasKeyboardFocus` work to `WindowHasKeyboardFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e53c6cd80027e6da:1: Delegates `SetLogicalFocus`'s `SetLogicalFocus / int / ctl` work to `SetLogicalFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6a425f94d0f3efb5:1: Delegates `SetLogicalFocus`'s `float / Eased / int` work to `Eased` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bcbafe3e61e2b69b:1: Delegates `SetLogicalFocus`'s `float / StageFade` work to `StageFade` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8903f6648f58ee8c:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Language` state.
// ADDED-LINE-RATIONALE db2bee5604969a1e:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Preparing` state.
// ADDED-LINE-RATIONALE 62fa7a0184067480:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Installing` state.
// ADDED-LINE-RATIONALE e5152019f2dbcc3e:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Opening` state.
// ADDED-LINE-RATIONALE fb0dd8f85921ef60:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Error` state.
// ADDED-LINE-RATIONALE ef2981de6c13b6dd:1: Delegates `SetLogicalFocus`'s `Tr / L / Redrob` work to `Tr` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bb393eb7c1466794:1: Delegates `SetLogicalFocus`'s `Tr / L / Setup` work to `Tr` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a2c4bf0907d63a4a:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Removing` state.
// ADDED-LINE-RATIONALE f192af6135e6b224:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Removed` state.
// ADDED-LINE-RATIONALE ca8b19362946c214:1: Supports `SetLogicalFocus` while preserving the nearby constraint: The line under each screen's title. The error screen's is the worker's result (ResultSentence).
// ADDED-LINE-RATIONALE 7ce09727dcce6599:2: Prevents `SetLogicalFocus` from applying its side effect outside the `switch / s` state.
// ADDED-LINE-RATIONALE 795c3eca65f0fca6:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Language` state.
// ADDED-LINE-RATIONALE de18110b4717e43a:1: Stops `SetLogicalFocus` at the `L / Setup / Redrob` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE bba9720d0174f68e:1: Makes `L / Redrob / Launcher` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 53dc0c9ca0b884ac:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Welcome` state.
// ADDED-LINE-RATIONALE 0b6e54bcbbc2f176:1: Stops `SetLogicalFocus` at the `Tr / L / It` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 3ad6e35f42b110ba:1: Makes `L / Windows / Redrob` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 2e45629235af9383:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Preparing` state.
// ADDED-LINE-RATIONALE 5b42dc0ca28d4d62:1: Stops `SetLogicalFocus` at the `Tr / L / Redrob` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 9023b5e850841f1c:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Opening` state.
// ADDED-LINE-RATIONALE 50ecb7e615d928c7:1: Stops `SetLogicalFocus` at the `Tr / L / Redrob` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE be3a495443e54f8a:1: Makes `L / Redrob / Launcher` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 1d3e2ebf2dd2bf48:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Confirm` state.
// ADDED-LINE-RATIONALE b59128349a0ae443:1: Stops `SetLogicalFocus` at the `Tr / L / This` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e846377485ad9d40:1: Makes `L / PC / Redrob` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 6df937de3690db79:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Removing` state.
// ADDED-LINE-RATIONALE e58aa5fde6c50bcb:1: Stops `SetLogicalFocus` at the `Tr / L / The` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 65f787e03986454c:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Removed` state.
// ADDED-LINE-RATIONALE b38012f81105f387:1: Stops `SetLogicalFocus` at the `deleteData / Tr / L` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d373775163633d3f:1: Makes `L` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 7916c7bf25cc0acc:1: Keeps `Tr / L / The` in `SetLogicalFocus`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 97cf992c2993e5a7:1: Makes `L / PC` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE bfd5b5ac035cadd7:1: Prevents `SetLogicalFocus` from applying its side effect outside the `case / Stage / Error` state.
// ADDED-LINE-RATIONALE 5bf4e5e686bb802a:1: Makes `default / L` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE a46cedb895787deb:3: Bounds the `SetLogicalFocus` implementation here so `default / L` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:5: Bounds the `SetLogicalFocus` implementation here so `default / L` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 493f126dd454e62c:1: Stops `SetLogicalFocus` at the `Tr / L / The` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 2c287b67da1b1453:1: Stops `SetLogicalFocus` at the `Tr / L / This` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e4ffb63517a0e109:1: Makes `L / Redrob / Launcher` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 414e244d26814083:1: Freezes `return` for `SetLogicalFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 22f619c69d7bc164:1: Makes `L / Windows / error` an explicit part of `SetLogicalFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE dd450b372f9f590d:1: Supports `A11yProvider` while preserving the nearby constraint: its nodes use the same bounds and actions as painting, hit testing, keyboard focus, and stage state.
// ADDED-LINE-RATIONALE c1185f95d48597fa:1: Keeps `public / IRawElementProviderFragmentRoot / public` in `A11yProvider`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE b0aeb3aba85fa2a3:1: Makes `public / IToggleProvider / public` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 736692b74dd3f345:3: Makes `public` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 1a8aa3fe3934aa2f:1: Delegates `A11yProvider`'s `A11yProvider / SetupApp / app` work to `A11yProvider` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 11d123777500b76f:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / QueryInterface / REFIID` work to `QueryInterface` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0908cd1e644d1fe7:1: Prevents `A11yProvider` from applying its side effect outside the `out / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE a34bd8ed6175b385:1: Makes `out / nullptr` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE f58593b89914dd88:1: Supports `A11yProvider` while preserving the nearby constraint: out = nullptr;.
// ADDED-LINE-RATIONALE a6311b4a7652226d:1: Freezes `else` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9281ba01372ab2fc:1: Freezes `else` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 336d2506edd37e38:1: Freezes `else` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4ec0738f2cc11f63:1: Freezes `else` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 28121159d2a5d829:1: Freezes `else` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6b3bc3a3162dba42:1: Prevents `A11yProvider` from applying its side effect outside the `out / E_NOINTERFACE` state.
// ADDED-LINE-RATIONALE 3db5ec971815baa8:1: Delegates `A11yProvider`'s `AddRef` work to `AddRef` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:1: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:4: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a69a82b98c79f623:1: Delegates `A11yProvider`'s `IFACEMETHODIMP_ / ULONG / AddRef` work to `IFACEMETHODIMP_` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cc08f2afdf5d8e4f:1: Delegates `A11yProvider`'s `IFACEMETHODIMP_ / ULONG / Release` work to `IFACEMETHODIMP_` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cb957c11be41e1ca:1: Freezes `LONG` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4c9b20447ab5f2af:1: Prevents `A11yProvider` from applying its side effect outside the `refs / delete` state.
// ADDED-LINE-RATIONALE 33e98d91b764f077:1: Stops `A11yProvider` at the `static_cast / ULONG / refs` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:5: Bounds the `A11yProvider` implementation here so `static_cast / ULONG / refs` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 043f04029e675e60:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / get_ProviderOptions / ProviderOptions` work to `get_ProviderOptions` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7d0b5df742f3d241:1: Prevents `A11yProvider` from applying its side effect outside the `options / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE 5d731b855477c973:1: Makes `options / ProviderOptions_ServerSideProvider` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:2: Supports `A11yProvider` while preserving the nearby constraint: options = ProviderOptions_ServerSideProvider;.
// ADDED-LINE-RATIONALE a46cedb895787deb:6: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2c23eab619c469f8:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / GetPatternProvider / PATTERNID` work to `GetPatternProvider` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0908cd1e644d1fe7:2: Prevents `A11yProvider` from applying its side effect outside the `out / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE a34bd8ed6175b385:2: Makes `out / nullptr` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE c98680ac40949c27:1: Supports `A11yProvider` while preserving the nearby constraint: out = nullptr;.
// ADDED-LINE-RATIONALE 72f3fc681ed6a4a0:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 957f7fdb6506725a:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:3: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:7: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b1547e63047cc436:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / GetPropertyValue / PROPERTYID` work to `GetPropertyValue` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e4beadd7ee48caae:1: Prevents `A11yProvider` from applying its side effect outside the `value / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE d9b8ec7f91412e3a:1: Delegates `A11yProvider`'s `VariantInit / value` work to `VariantInit` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d24910e17e7c3908:1: Prevents `A11yProvider` from applying its side effect outside the `Available / UIA_E_ELEMENTNOTAVAILABLE` state.
// ADDED-LINE-RATIONALE 3bbc95bd226ee6e6:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1345edfe7e7d824a:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f171549f24460bdd:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 690a102431f98b3f:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a1f24963eafdefcf:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 45cdd8b987a913c7:1: Freezes `value` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dcc993ebbdbf0cb7:1: Delegates `A11yProvider`'s `property / UIA_IsControlElementPropertyId / property` work to `if` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f45080da26968c12:1: Freezes `value` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 58047fdc04efeb33:1: Delegates `A11yProvider`'s `property / UIA_IsEnabledPropertyId` work to `if` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d068fcdec0755e18:1: Freezes `value` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0d982af39072b201:1: Delegates `A11yProvider`'s `property / UIA_IsKeyboardFocusablePropertyId` work to `if` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d8f08929097fda82:1: Freezes `value` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4b13fcaed7c7483d:1: Delegates `A11yProvider`'s `property / UIA_HasKeyboardFocusPropertyId` work to `if` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4bd9dd368eb6b48b:1: Freezes `value` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE af3db68ef2412ee8:1: Delegates `A11yProvider`'s `property / UIA_LiveSettingPropertyId` work to `if` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f624bf5f1e9df492:1: Makes `id_ / A11yId / Title` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 1c94a5f0f32350a5:1: Freezes `value` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 86a5916f9a042504:1: Delegates `A11yProvider`'s `property / UIA_ItemStatusPropertyId / id_` work to `if` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 87f724589765ef48:1: Freezes `app_` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fa21cf0252072b10:1: Freezes `bool` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 30eb71315fed5c5f:1: Prevents `A11yProvider` from applying its side effect outside the `selected / SetString / value` state.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:1: Bounds the `A11yProvider` implementation here so `selected / SetString / value` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:4: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:8: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE df3870e3d4d94d3f:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / get_HostRawElementProvider / IRawElementProviderSimple` work to `get_HostRawElementProvider` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0908cd1e644d1fe7:3: Prevents `A11yProvider` from applying its side effect outside the `out / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE a34bd8ed6175b385:3: Makes `out / nullptr` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE e266376c4cba8a82:1: Supports `A11yProvider` while preserving the nearby constraint: out = nullptr;.
// ADDED-LINE-RATIONALE a46cedb895787deb:9: Bounds the `A11yProvider` implementation here so `id_ / A11yId / Root` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e6801befc1377095:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / Navigate / NavigateDirection` work to `Navigate` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0908cd1e644d1fe7:4: Prevents `A11yProvider` from applying its side effect outside the `out / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE a34bd8ed6175b385:4: Makes `out / nullptr` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE d24910e17e7c3908:2: Supports `A11yProvider` while preserving the nearby constraint: out = nullptr;.
// ADDED-LINE-RATIONALE 20110f709657dfb3:1: Freezes `auto` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0c4390cd583f6b72:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 37eda9b1009454aa:1: Prevents `A11yProvider` from applying its side effect outside the `children / empty / S_OK` state.
// ADDED-LINE-RATIONALE 290a1fdc9fa9c601:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6a9ec3fe6a031683:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f7ac19c51a5bb953:1: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:2: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 54e9df74f2caad4d:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c5d9784cdee668fc:1: Freezes `auto` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0746d73ffd31389f:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 45fb6a2a9e25b059:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5515d9cac5841e9d:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:5: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:10: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f7ebf8de10dbeefe:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / GetRuntimeId / SAFEARRAY` work to `GetRuntimeId` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0908cd1e644d1fe7:5: Prevents `A11yProvider` from applying its side effect outside the `out / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE a34bd8ed6175b385:5: Makes `out / nullptr` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 081d977348a76f56:1: Supports `A11yProvider` while preserving the nearby constraint: out = nullptr;.
// ADDED-LINE-RATIONALE 209981e374c06c00:1: Freezes `SAFEARRAY` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e0373b764eebd156:1: Prevents `A11yProvider` from applying its side effect outside the `ids / E_OUTOFMEMORY` state.
// ADDED-LINE-RATIONALE ad72904975c8e53a:1: Freezes `LONG` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 059d84422e6f69ce:1: Delegates `A11yProvider`'s `SafeArrayPutElement / ids / index` work to `SafeArrayPutElement` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3d38b239a4b548de:1: Delegates `A11yProvider`'s `index / value / static_cast` work to `SafeArrayPutElement` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f4695947cd9e7908:1: Makes `out / ids` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:6: Supports `A11yProvider` while preserving the nearby constraint: out = ids;.
// ADDED-LINE-RATIONALE a46cedb895787deb:11: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 17d55bca03f18e4d:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / get_BoundingRectangle / UiaRect` work to `get_BoundingRectangle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7f5dfbc0a7d6e36e:1: Prevents `A11yProvider` from applying its side effect outside the `bounds / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE e21c09c04bad95f9:1: Makes `bounds` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE d24910e17e7c3908:3: Supports `A11yProvider` while preserving the nearby constraint: bounds = {};.
// ADDED-LINE-RATIONALE 9dbd27149bc8e913:1: Freezes `RECT` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0e12c68f1c0add9a:1: Makes `POINT / top / rect` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 8c99e68e172a90cf:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 271e6ab05cd266b0:1: Freezes `bounds` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:7: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:12: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 54bfee8016c04b7a:1: Freezes `IFACEMETHODIMP` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0eb9abd0980ded37:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / SetFocus / override` work to `SetFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 20fe473b79c22ae8:1: Prevents `A11yProvider` from applying its side effect outside the `Available / KeyboardFocusable / UIA_E_NOTSUPPORTED` state.
// ADDED-LINE-RATIONALE 8936d1eeec1c93c0:1: Delegates `A11yProvider`'s `PostMessageW / app_ / hwnd_` work to `PostMessageW` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:8: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:13: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 20151be395365ca8:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / get_FragmentRoot / IRawElementProviderFragmentRoot` work to `get_FragmentRoot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0908cd1e644d1fe7:6: Prevents `A11yProvider` from applying its side effect outside the `out / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE b76647bc9b630682:1: Delegates `A11yProvider`'s `out / new / std` work to `new` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6888bb4bdcc5913c:1: Supports `A11yProvider` while preserving the nearby constraint: out = new (std::nothrow) A11yProvider(app_, A11yId::Root);.
// ADDED-LINE-RATIONALE a46cedb895787deb:14: Bounds the `A11yProvider` implementation here so `out / S_OK / E_OUTOFMEMORY` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 246c4fcfad300071:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / ElementProviderFromPoint / double` work to `ElementProviderFromPoint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0908cd1e644d1fe7:7: Prevents `A11yProvider` from applying its side effect outside the `out / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE 14e510a781b0a7c8:1: Makes `POINT / point / static_cast` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE ba232e29d2fe626b:1: Delegates `A11yProvider`'s `ScreenToClient / app_ / hwnd_` work to `ScreenToClient` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 20110f709657dfb3:2: Freezes `auto` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fadd162a477e0fc7:1: Freezes `for` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0914c74c2c144bfb:1: Stops `A11yProvider` at the `New / A11yId / Root` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:15: Bounds the `A11yProvider` implementation here so `New / A11yId / Root` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 899f7e43cc8bc2a5:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / GetFocus / IRawElementProviderFragment` work to `GetFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0908cd1e644d1fe7:8: Prevents `A11yProvider` from applying its side effect outside the `out / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE a34bd8ed6175b385:6: Makes `out / nullptr` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE d24910e17e7c3908:4: Supports `A11yProvider` while preserving the nearby constraint: out = nullptr;.
// ADDED-LINE-RATIONALE fad50c6a491194a7:1: Supports `A11yProvider` while preserving the nearby constraint: Virtual focus is meaningful only while this top-level HWND owns the thread's real keyboard focus.
// ADDED-LINE-RATIONALE 0fcf3ba14ffdf4f4:1: Stops `A11yProvider` at the `New / IdForControl / app_` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:16: Bounds the `A11yProvider` implementation here so `New / IdForControl / app_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE bb43af6ac7ca2aa7:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / Invoke / override` work to `Invoke` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d24910e17e7c3908:5: Prevents `A11yProvider` from applying its side effect outside the `Available / UIA_E_ELEMENTNOTAVAILABLE` state.
// ADDED-LINE-RATIONALE 6363952e51770786:1: Prevents `A11yProvider` from applying its side effect outside the `IsButton / UIA_E_NOTSUPPORTED` state.
// ADDED-LINE-RATIONALE 7ff5a7f26d8ebbec:1: Prevents `A11yProvider` from applying its side effect outside the `Enabled / UIA_E_ELEMENTNOTENABLED` state.
// ADDED-LINE-RATIONALE fd715044e1f9a548:1: Delegates `A11yProvider`'s `PostMessageW / app_ / hwnd_` work to `PostMessageW` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:9: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:17: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 93f0b9586977fe6f:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / Toggle / override` work to `Toggle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 20715cb93d42c9bc:1: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 53f56b794c7c7037:1: Delegates `A11yProvider`'s `PostMessageW / app_ / hwnd_` work to `PostMessageW` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:10: Stops `A11yProvider` at the `S_OK` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:18: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE dc6f0f16b858372f:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / get_ToggleState / ToggleState` work to `get_ToggleState` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a381210d2f72772a:1: Prevents `A11yProvider` from applying its side effect outside the `state / E_INVALIDARG` state.
// ADDED-LINE-RATIONALE 20715cb93d42c9bc:2: Freezes `if` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8b86423dcb9325bd:1: Makes `state / app_ / stage_` an explicit part of `A11yProvider` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 8628d8f4d4f5f333:11: Supports `A11yProvider` while preserving the nearby constraint: state = (app_->stage_ == Stage::Welcome ? app_->desktopShortcut_ : app_->deleteData_) ? ToggleState_On : ToggleState_Off;.
// ADDED-LINE-RATIONALE a46cedb895787deb:19: Bounds the `A11yProvider` implementation here so `S_OK` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 9fb906a27b631682:1: Delegates `A11yProvider`'s `IFACEMETHODIMP / SetValue / double` work to `SetValue` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c2ad490bbd389d7b:1: Freezes `IFACEMETHODIMP` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fb80554b436289d6:1: Freezes `IFACEMETHODIMP` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9e03bae24bc1b05a:1: Freezes `IFACEMETHODIMP` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1304dba676d7069f:1: Freezes `IFACEMETHODIMP` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8b64febd9e330b24:1: Freezes `IFACEMETHODIMP` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e04c9f60acaef00c:1: Freezes `IFACEMETHODIMP` for `A11yProvider` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 933d3471144bf6ea:1: Delegates `IdForControl`'s `A11yId / IdForControl / int` work to `IdForControl` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0098598448475b7c:1: Freezes `if` for `IdForControl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ee8844d26822a546:1: Freezes `if` for `IdForControl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2e4315fb0a7b9dde:1: Freezes `if` for `IdForControl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 788143a984bb6b51:1: Freezes `if` for `IdForControl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8988753fe3f8c15f:1: Freezes `if` for `IdForControl` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0007ed9d7fd9ff28:1: Stops `IdForControl` at the `A11yId / Root` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:20: Bounds the `IdForControl` implementation here so `A11yId / Root` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 255ccbcb9c2031b7:3: Makes `private` an explicit part of `IdForControl` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 781e7121ee5374ba:1: Delegates `Available`'s `bool / Available` work to `Available` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 74731ae3fd2eb4e4:1: Prevents `Available` from applying its side effect outside the `app_ / app_ / hwnd_` state.
// ADDED-LINE-RATIONALE 9962e38b665d469a:1: Freezes `if` for `Available` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1bbf860efdc0ba40:1: Freezes `auto` for `Available` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 876562cc635053ec:1: Freezes `return` for `Available` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:21: Bounds the `Available` implementation here so `std / find / list` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2f5bf40a29459cfb:1: Freezes `bool` for `IsButton` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1de497ca95758372:1: Freezes `bool` for `IsInteractive` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 27d998de9c19b17c:1: Freezes `bool` for `Enabled` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d0a39bdba55bcdde:1: Delegates `KeyboardFocusable`'s `bool / KeyboardFocusable` work to `KeyboardFocusable` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3280eb7a6535b0ba:1: Prevents `KeyboardFocusable` from applying its side effect outside the `IsInteractive` state.
// ADDED-LINE-RATIONALE cd438306cf8c92eb:1: Freezes `auto` for `KeyboardFocusable` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4c937fceaf80691a:1: Freezes `return` for `KeyboardFocusable` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:22: Bounds the `KeyboardFocusable` implementation here so `std / find / order` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d71becf5897c18b1:1: Delegates `Control`'s `int / Control` work to `Control` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 464e43a7642ed886:1: Freezes `if` for `Control` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 96d8c972dfa0c761:1: Freezes `if` for `Control` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f6751e973f605060:1: Freezes `return` for `Control` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:23: Bounds the `Control` implementation here so `id_ / A11yId / Checkbox` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 9c57fa6ce29b2d8d:1: Delegates `HasFocus`'s `bool / HasFocus` work to `HasFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 88db20d4c5a74ad4:1: Freezes `return` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:24: Bounds the `HasFocus` implementation here so `app_ / WindowHasKeyboardFocus / id_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 70f2cc019606d9d8:1: Delegates `HasFocus`'s `CONTROLTYPEID / ControlType` work to `ControlType` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8d0b734111e19c10:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 45159de5a3dd0c47:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c3351a68db05a276:1: Stops `HasFocus` at the `UIA_TextControlTypeId` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:25: Bounds the `HasFocus` implementation here so `UIA_TextControlTypeId` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 977cb8614e2836cb:1: Delegates `HasFocus`'s `std / wstring / Name` work to `Name` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE fc52f5d8e6646ace:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b050fb5b65c16ed5:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8acb4a3487e01ec6:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2436d8a17eca3832:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5d10576df4017aca:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bfdc77f5c4522df6:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c83452ee20a112ab:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e6a4a3c80e3a7fdd:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b8793de077b3379c:1: Freezes `int` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c8fa274173a8072b:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a8230f76f29a3a8d:1: Freezes `else` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f6de5895b07e6cbb:1: Freezes `else` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 532e13436fd1217a:1: Freezes `return` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:3: Bounds the `HasFocus` implementation here so `g_lang / Lang / Ko` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2ab9d46cf34084a0:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 555aa9e02286b6af:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f888afea3f76037a:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 23bf807d82e284e6:1: Stops `HasFocus` at the `Tr / L / Retry` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:4: Bounds the `HasFocus` implementation here so `Tr / L / Retry` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c0162e8b58eba7c5:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5f387502e991838c:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:26: Bounds the `HasFocus` implementation here so `app_ / stage_ / Stage` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 698e92a8517732ea:1: Delegates `HasFocus`'s `std / wstring / AutomationId` work to `AutomationId` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c2ce2c5c4823b383:1: Freezes `static` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dad71421646a89a8:1: Stops `HasFocus` at the `ids / static_cast / size_t` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:27: Bounds the `HasFocus` implementation here so `ids / static_cast / size_t` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 49116664a7840cba:1: Delegates `HasFocus`'s `RECT / RectFor / A11yId` work to `RectFor` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 529ba6b4b88ee0d9:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 442afe3d2a0e0acc:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 49996b56c4b2b830:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 91474a3975d9bd60:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3a90017dd3fe1ab9:1: Freezes `return` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:28: Bounds the `HasFocus` implementation here so `id / A11yId / Secondary` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5b28985b0733d55f:1: Delegates `HasFocus`'s `std / vector / A11yId` work to `Children` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7cc2f79c091387c7:1: Makes `std / vector / A11yId` an explicit part of `HasFocus` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 2b77a7edf83f303c:1: Freezes `if` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8f0fae45133928f2:1: Freezes `else` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE db9f0de082838413:1: Freezes `else` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e2233480c4f33a7b:1: Freezes `else` for `HasFocus` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f25218a4f252fb00:1: Delegates `HasFocus`'s `result / push_back / A11yId` work to `result.push_back` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a46cedb895787deb:29: Bounds the `HasFocus` implementation here so `result / push_back / A11yId` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 394571a466181604:1: Delegates `New`'s `HRESULT / New / A11yId` work to `New` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3321302443578af5:1: Freezes `if` for `New` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:30: Bounds the `New` implementation here so `out / E_INVALIDARG / out` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ae5b5696990ed6e5:1: Delegates `SetString`'s `HRESULT / SetString / VARIANT` work to `SetString` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f8eb1c959458dd90:1: Freezes `value` for `SetString` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:31: Bounds the `SetString` implementation here so `value / vt / VT_BSTR` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 77f3fa8baa97d2cd:1: Freezes `LONG` for `SetString` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 583f446896282ed0:4: Bounds the `SetString` implementation here so `LONG / refs_ / SetupApp` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c93a8fc74fef43e6:1: Delegates `RaiseStructureChanged`'s `RaiseStructureChanged / SetupApp / app` work to `RaiseStructureChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d56a1e0de096e3fc:1: Prevents `RaiseStructureChanged` from applying its side effect outside the `app / app / hwnd_` state.
// ADDED-LINE-RATIONALE f102b51924c63614:1: Freezes `auto` for `RaiseStructureChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8d9739b8d7245543:1: Prevents `RaiseStructureChanged` from applying its side effect outside the `root / UiaRaiseStructureChangedEvent / root` state.
// ADDED-LINE-RATIONALE 879f0f2dcf10a00b:1: Supports `RaiseStructureChanged` while preserving the nearby constraint: when the worker-written text and the Body provider's Name expose the same current value.
// ADDED-LINE-RATIONALE 1159ac815bd03ccc:1: Freezes `auto` for `RaiseStructureChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3c11af02ccd94e1c:1: Prevents `RaiseStructureChanged` from applying its side effect outside the `live` state.
// ADDED-LINE-RATIONALE d83692dc65f425d7:1: Delegates `RaiseStructureChanged`'s `UiaRaiseAutomationEvent / live / UIA_LiveRegionChangedEventId` work to `UiaRaiseAutomationEvent` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ec9a8df4f1037e15:1: Freezes `if` for `RaiseStructureChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7aafa804a56eaeaf:1: Freezes `BSTR` for `RaiseStructureChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 734bb2883d73f902:1: Freezes `BSTR` for `RaiseStructureChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 68f2cca2b938bf89:1: Prevents `RaiseStructureChanged` from applying its side effect outside the `display / activity` state.
// ADDED-LINE-RATIONALE 745d66a4ed18dc7b:1: Keeps `UiaRaiseNotificationEvent / live / NotificationKind_ActionAborted` in `RaiseStructureChanged`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 62e78a05b3173098:1: Makes `NotificationProcessing_ImportantMostRecent / display / activity` an explicit part of `RaiseStructureChanged` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 17faac00cc7b51db:1: Delegates `RaiseStructureChanged`'s `SysFreeString / activity` work to `SysFreeString` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 13e972f27768b798:1: Delegates `RaiseStructureChanged`'s `SysFreeString / display` work to `SysFreeString` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a46cedb895787deb:32: Bounds the `RaiseStructureChanged` implementation here so `SysFreeString / display` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60094e7ac222167e:1: Delegates `RaiseStructureChanged`'s `live / Release` work to `Release` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:7: Bounds the `RaiseStructureChanged` implementation here so `live / Release` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1c71ef20201aab06:1: Delegates `RaiseToggleChanged`'s `RaiseToggleChanged / SetupApp / app` work to `RaiseToggleChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 41e05e0fc8e4ac4a:1: Freezes `if` for `RaiseToggleChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 357591ebddc34ff1:1: Freezes `auto` for `RaiseToggleChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4639fc798bfa1247:1: Freezes `VARIANT` for `RaiseToggleChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7cb193cfd1772ec9:1: Delegates `RaiseToggleChanged`'s `UiaRaiseAutomationPropertyChangedEvent / option / UIA_ToggleToggleStatePropertyId` work to `UiaRaiseAutomationPropertyChangedEvent` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:8: Bounds the `RaiseToggleChanged` implementation here so `UiaRaiseAutomationPropertyChangedEvent / option / UIA_ToggleToggleStatePropertyId` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3d83717cd39b6a37:1: Delegates `RaiseFocusChanged`'s `RaiseFocusChanged / SetupApp / app` work to `RaiseFocusChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b6dca6327cac00f1:1: Freezes `if` for `RaiseFocusChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE adf957c11e763cf4:1: Prevents `RaiseFocusChanged` from applying its side effect outside the `node / UiaRaiseAutomationEvent / node` state.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:9: Bounds the `RaiseFocusChanged` implementation here so `node / UiaRaiseAutomationEvent / node` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e3f755d91e8a355d:1: Delegates `RaiseProgressChanged`'s `RaiseProgressChanged / SetupApp / app` work to `RaiseProgressChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 74f076fe41ec4209:1: Freezes `int` for `RaiseProgressChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 789c8a8fce30724e:1: Freezes `if` for `RaiseProgressChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fa0adb3a877c911f:1: Freezes `auto` for `RaiseProgressChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 691fa79dcd6d7a96:1: Freezes `VARIANT` for `RaiseProgressChanged` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 81df5e6ce372d383:1: Delegates `RaiseProgressChanged`'s `UiaRaiseAutomationPropertyChangedEvent / node / UIA_RangeValueValuePropertyId` work to `UiaRaiseAutomationPropertyChangedEvent` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:10: Bounds the `RaiseProgressChanged` implementation here so `UiaRaiseAutomationPropertyChangedEvent / node / UIA_RangeValueValuePropertyId` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 12fb4afae9d761a7:1: Freezes `g_dark` for `DefaultLanguage` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a1b673de7f678c77:1: Delegates `DefaultLanguage`'s `ApplyWindowTheme` work to `ApplyWindowTheme` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a804b7a050d922bd:1: Freezes `BOOL` for `DefaultLanguage` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f173a0e0516e07c4:1: Freezes `if` for `DefaultLanguage` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cdce94e135fde5d3:1: Isolates `REDROB_REQUIRE_BUNDLED_FONTS` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE cce1a7241a5496a2:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: ifdef REDROB_REQUIRE_BUNDLED_FONTS.
// ADDED-LINE-RATIONALE 5d0fc304617b4823:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Production never silently substitutes fonts because that changes Korean shaping and metrics.
// ADDED-LINE-RATIONALE cb21fd002ad50e81:1: Keeps `L` in `REDROB_REQUIRE_BUNDLED_FONTS`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 9af128d120f5f07c:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `Tr / L / Redrob` work to `Tr` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 01dc7dd2aff8e762:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `DestroyWindow / hwnd_` work to `DestroyWindow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4edea240ed046be2:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `fontColl_ / reset / wicFactory_` work to `fontColl_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f2b8eac842d795e0:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `ERROR_RESOURCE_DATA_NOT_FOUND` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:33: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `ERROR_RESOURCE_DATA_NOT_FOUND` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 025d4ae15d9fc9fd:3: Keeps `REDROB_REQUIRE_BUNDLED_FONTS` on the `endif` compile-time path required by the supported native targets.
// ADDED-LINE-RATIONALE db020f5d93d7987f:1: Freezes `errorMessage_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 27fc9a1364185883:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Raise after first paint so provider bounds are ready when a client handles WindowOpened.
// ADDED-LINE-RATIONALE c34c69facf63b5ed:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `openedProvider / UiaRaiseAutomationEvent / openedProvider` state.
// ADDED-LINE-RATIONALE 011f9f681cb48033:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `fmtTitle_ / reset` work to `fmtTitle_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 91345bca8071bc73:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `fmtBody_ / reset` work to `fmtBody_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e29e507ab22b3e63:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `fmtLabel_ / reset` work to `fmtLabel_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 484c00454ffba325:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `fmtStep_ / reset` work to `fmtStep_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c0ba429a8e6bf7ec:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `fmtBrand_ / reset` work to `fmtBrand_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f1d9bfea135a7ca1:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: The DWM frame follows the theme too (immersive dark mode only for the dark theme).
// ADDED-LINE-RATIONALE 8fdad656143fa55a:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `hwnd_` state.
// ADDED-LINE-RATIONALE 12695a415ae9e034:1: Freezes `BOOL` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 66ede89415628a7e:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `DwmSetWindowAttribute / hwnd_ / DWMWA_USE_IMMERSIVE_DARK_MODE` work to `DwmSetWindowAttribute` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:15: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `DwmSetWindowAttribute / hwnd_ / DWMWA_USE_IMMERSIVE_DARK_MODE` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e4523a8cdd24d4e0:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: One private collection keeps deterministic Pretendard and Wanted Sans resources together.
// ADDED-LINE-RATIONALE 164633106a7e6798:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `SUCCEEDED / dwFactory_ / QueryInterface` state.
// ADDED-LINE-RATIONALE 74ba90cd8fe7016d:1: Makes `ComPtr / IDWriteInMemoryFontFileLoader / loader` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 3aae16a69abb1d34:1: Makes `ComPtr / IDWriteFontSetBuilder1 / builder` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 679bb332f5286d8c:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `SUCCEEDED / f5 / CreateInMemoryFontFileLoader` state.
// ADDED-LINE-RATIONALE 5d77674a91eb3768:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SUCCEEDED / f5 / CreateFontSetBuilder` work to `SUCCEEDED` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 82cf34130c44ef75:1: Freezes `int` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c762a2f78fd3bdc7:1: Applies `REDROB_REQUIRE_BUNDLED_FONTS` consistently to every `int / id / IDR_FONT_SEMIBOLD` item rather than handling only the first match.
// ADDED-LINE-RATIONALE f26b463b34dad3b4:1: Freezes `HRSRC` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8aa154c08e000977:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `resource / continue` state.
// ADDED-LINE-RATIONALE ed6a2f6af3f76b8a:1: Freezes `HGLOBAL` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 62f5e4cb706af4db:1: Freezes `void` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9dddd2912a682076:1: Freezes `DWORD` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 644bad73ca447ba3:1: Makes `ComPtr / IDWriteFontFile / file` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE a38960c1fa829532:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `data / size / SUCCEEDED` state.
// ADDED-LINE-RATIONALE 290fa00fe540feab:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SUCCEEDED / builder / AddFontFile` work to `SUCCEEDED` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3013a1266148ee2b:1: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `SUCCEEDED / builder / AddFontFile` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d03fbaff668ae1bb:1: Makes `ComPtr / IDWriteFontSet / set` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE c42d0a0dd9bd2a56:1: Makes `ComPtr / IDWriteFontCollection1 / collection` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE c2f16493ff91cc52:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1df7e8b2b31ea4c3:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SUCCEEDED / f5 / CreateFontCollectionFromFontSet` work to `SUCCEEDED` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 90a278de45169fa2:1: Freezes `fontColl_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 28f94912e608756d:1: Freezes `collection` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3013a1266148ee2b:2: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `collection / p / nullptr` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6d429f740474dc99:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `fontColl_` state.
// ADDED-LINE-RATIONALE 4da0d8ee9223aa86:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Family identity is independent of file order; both Pretendard weights normally share one family.
// ADDED-LINE-RATIONALE 42d123c7119ba092:1: Makes `ComPtr / IDWriteFontFamily / family` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 6c6d35eda2a91072:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `FAILED / fontColl_ / GetFontFamily` state.
// ADDED-LINE-RATIONALE 5bb967a0a591a94a:1: Freezes `UINT32` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3edcd7b837e1b2c1:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a97077d1aa4f3c1a:1: Freezes `UINT32` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9d3955628a740eaf:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `std / wstring / length` work to `name` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE edfc31d00bc3e5bc:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0460d2a5ebbd3435:1: Freezes `else` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:6: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `find / L / Wanted` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 69041328e74506d5:1: Freezes `bundledFontsReady_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 61cd9d50ad83074a:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `bundledFontsReady_ / fontColl_ / reset` state.
// ADDED-LINE-RATIONALE 8180a1759bcaf462:1: Freezes `auto` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a3c1e110ea99b659:1: Makes `float / lineHeight / DWRITE_TEXT_ALIGNMENT` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 85c1efa6d9b791c3:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `out / reset` work to `out.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1a1c007c3a17de3e:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `FAILED / dwFactory_ / CreateTextFormat` state.
// ADDED-LINE-RATIONALE 16802a3355b53ac7:1: Makes `DWRITE_FONT_STRETCH_NORMAL / size / L` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE f286ccd4f00d145d:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `out / SetTextAlignment / align` work to `SetTextAlignment` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f06dfe747d95e395:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `lineHeight / out / SetLineSpacing` state.
// ADDED-LINE-RATIONALE dd95c0a5fbbdc870:1: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `lineHeight / out / SetLineSpacing` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1e8b0a157a562269:1: Freezes `auto` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cf073653e44d094b:1: Freezes `auto` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 146e2ce3014fc31a:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `make / fmtTitle_ / fontFamily_` work to `make` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bf56cd49beaa19e1:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `make / fmtBody_ / bodyFamily_` work to `make` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE efd7bf446d03aab0:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `make / fmtButton_ / fontFamily_` work to `make` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE af0824386e95f5a4:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `make / fmtLabel_ / bodyFamily_` work to `make` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1f2670dc52ea511b:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `make / fmtStep_ / fontFamily_` work to `make` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3f45d8bbcccebce2:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `make / fmtBrand_ / fontFamily_` work to `make` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bb76d4cca4a9cf01:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Production readiness includes format creation, not merely finding three resource blobs.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:16: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `bundledFontsReady_ / bundledFontsReady_ / fmtTitle_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 61d5ceb63bbd0d8e:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetupApp / ApplyHangulFont / IDWriteTextLayout` work to `SetupApp::ApplyHangulFont` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3868866574e2c52a:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `layout / hangulFamily_ / empty` state.
// ADDED-LINE-RATIONALE afd7c3f4202da256:1: Freezes `auto` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 691465df6502ccaa:1: Freezes `for` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5156827069888647:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `isHangul / text / start` state.
// ADDED-LINE-RATIONALE 7a708061bed374b6:1: Freezes `UINT32` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6428b38e4ee9b14e:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Range-level assignment preserves Pretendard for Latin product names embedded in Korean copy.
// ADDED-LINE-RATIONALE 0de2732fdc1b890a:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `layout / SetFontWeight / DWRITE_FONT_WEIGHT_REGULAR` work to `SetFontWeight` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 93afd2775724c39b:1: Freezes `start` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1a0e133f4b5d4a17:1: Keeps `SetupApp / DrawTextLine / std` in `REDROB_REQUIRE_BUNDLED_FONTS`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0de116688c406680:1: Makes `D2D1_RECT_F / rect / ID2D1Brush` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 08a5535ad8ea1032:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `format / text / empty` state.
// ADDED-LINE-RATIONALE 570183aa3eb53ced:1: Makes `ComPtr / IDWriteTextLayout / layout` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE ff6ab7c6e4a08133:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `FAILED / dwFactory_ / CreateTextLayout` state.
// ADDED-LINE-RATIONALE 34fc9637e58df1cd:1: Makes `rect / right / rect` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE e3d952ecb315136b:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `ApplyHangulFont / layout / p` work to `ApplyHangulFont` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c6100def6b379161:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `rt_ / DrawTextLayout / D2D1` work to `DrawTextLayout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:18: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `rt_ / DrawTextLayout / D2D1` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5f2018a60f4a1672:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_GETOBJECT` state.
// ADDED-LINE-RATIONALE e303737631553673:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 98e7ca4951b9f2a4:1: Freezes `auto` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6ca617d8ae8d689d:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `provider` state.
// ADDED-LINE-RATIONALE de6fa2431e146c36:1: Freezes `LRESULT` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c50b12e9829fafb5:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `provider / Release` work to `Release` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 55beaed16b317a24:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `result` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 3013a1266148ee2b:3: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `result` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 67d66cbce22c77b5:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `DefWindowProc / hwnd_ / msg` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d41794c63c426c27:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / kA11yInvokeMsg / Activate` state.
// ADDED-LINE-RATIONALE e2e7065b4c42677f:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / kA11yFocusMsg` state.
// ADDED-LINE-RATIONALE 0249a65b53b9eb6c:1: Freezes `int` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8627c541a58a843b:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `IsFocusable / target / the` state.
// ADDED-LINE-RATIONALE a537bb9dc45b26ba:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Set the virtual target first: a synchronous WM_SETFOCUS then announces the right fragment.
// ADDED-LINE-RATIONALE cf1f5088d35a2204:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetFocus / hwnd_` work to `SetFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bd14f8f9ecfe5ec3:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `SetFocus / hwnd_` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:7: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `case / WM_SETFOCUS` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b80dc2de03e27e75:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_SETFOCUS` state.
// ADDED-LINE-RATIONALE 4a366927ae06123b:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2e9cc8f51c1ef480:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `RaiseFocusChanged / A11yProvider / IdForControl` work to `RaiseFocusChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bd14f8f9ecfe5ec3:2: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `RaiseFocusChanged / A11yProvider / IdForControl` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE ee48274624e018ef:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_KILLFOCUS` state.
// ADDED-LINE-RATIONALE 4a366927ae06123b:2: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Retain logical focus for activation restoration, but painting/properties are gated by real HWND focus.
// ADDED-LINE-RATIONALE bd14f8f9ecfe5ec3:3: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `InvalidateRect / hwnd_ / nullptr` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 48431fb42f94455f:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6d51b803aa8bfd21:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `AnimStep / KillTimer / hwnd_` state.
// ADDED-LINE-RATIONALE 1cf9fa108fa4345b:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3013a1266148ee2b:4: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `InvalidateRect / hwnd_ / nullptr` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 431b3bb207716cd7:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_SETTINGCHANGE` state.
// ADDED-LINE-RATIONALE 5134d831e32058b7:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Windows switched between light and dark: follow it.
// ADDED-LINE-RATIONALE 0f8c4f467bd81b87:1: Freezes `g_dark` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 887b5d23844a8797:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `ApplyWindowTheme` work to `ApplyWindowTheme` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1cf9fa108fa4345b:2: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bd14f8f9ecfe5ec3:9: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `InvalidateRect / hwnd_ / nullptr` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 1d4a9a05f2122af9:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_MOUSEMOVE` state.
// ADDED-LINE-RATIONALE 34b5a0def34f8623:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `trackingMouse_` state.
// ADDED-LINE-RATIONALE 8c2255a53f6b9943:1: Freezes `TRACKMOUSEEVENT` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e1ebfc602b6d7a25:1: Freezes `trackingMouse_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e4473a7a6dfc14eb:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `UpdateHover` work to `UpdateHover` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bd14f8f9ecfe5ec3:10: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `UpdateHover` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 07f44bcaf46afbab:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_MOUSELEAVE` state.
// ADDED-LINE-RATIONALE d95ff1350e568d52:1: Freezes `trackingMouse_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c2333909e4e7e24b:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bd14f8f9ecfe5ec3:11: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `hover_ / kNone / hover_` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e8a418bc6c181259:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_SETCURSOR` state.
// ADDED-LINE-RATIONALE ec32efc57b17c82a:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 89d5330e1e7af153:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `POINT / pt / GetCursorPos` work to `GetCursorPos` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a147f9d32b659268:1: Freezes `SetCursor` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2113556ba2b868eb:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `TRUE` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 67d66cbce22c77b5:2: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `DefWindowProc / hwnd_ / msg` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 9942746e60441c7a:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_LBUTTONDOWN` state.
// ADDED-LINE-RATIONALE d2ce14ce5beb21f6:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `POINT / pt / GET_X_LPARAM` work to `GET_X_LPARAM` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 927db3cd13e2d141:1: Freezes `focusVisible_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cc4b40765886bfe3:1: Freezes `int` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 45a2a853a485f405:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e4aa6fd8f7fd7f1f:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `IsFocusable / hit` state.
// ADDED-LINE-RATIONALE b06e61fd9f2e4ebf:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Publish the pointer's virtual target before SetFocus can synchronously send WM_SETFOCUS.
// ADDED-LINE-RATIONALE c67cae0bc014b7c4:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetFocus / hwnd_` work to `SetFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c0d019fc8c87914b:1: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `SetFocus / hwnd_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE dde3d6269d0b83aa:1: Freezes `pressed_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 71855026f4185666:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetCapture / hwnd_` work to `SetCapture` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5de71478239dc75b:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `StartAnim` work to `StartAnim` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b5cf314c7f83438b:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Anywhere else on the caption strip or the brand panel drags the window.
// ADDED-LINE-RATIONALE 4a366927ae06123b:4: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9f6d8ea77590546d:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_LBUTTONUP` state.
// ADDED-LINE-RATIONALE d2ce14ce5beb21f6:2: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `POINT / pt / GET_X_LPARAM` work to `GET_X_LPARAM` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE af540c7b9864f4ac:1: Freezes `int` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 30de9e942dd5fab4:1: Freezes `pressed_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 23e0082e538ab868:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7aa98d434768aabb:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `StartAnim` work to `StartAnim` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 40be706b1c221991:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: The action runs on release over the control it went down on, like a native button.
// ADDED-LINE-RATIONALE bd14f8f9ecfe5ec3:13: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `was / kNone / HitTest` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:12: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `was / kNone / HitTest` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d5c21d0d74c94ce3:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_KEYDOWN` state.
// ADDED-LINE-RATIONALE 48803d3b9e1c1729:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 205a90ae4a373ce2:1: Freezes `std` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 69cb1ca403a4d743:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `order / empty` state.
// ADDED-LINE-RATIONALE 0b0b6ad4ed997f6c:1: Freezes `bool` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ab9942ff631e2c86:1: Freezes `auto` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 019ae1533122e524:1: Freezes `int` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 71645cdb1054405a:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `int / at / order` work to `order.begin` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9b25412f626ee960:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetLogicalFocus / order / i` work to `SetLogicalFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE af8197b6c4145ce3:2: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `SetLogicalFocus / order / i` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 3013a1266148ee2b:12: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `Logical / focus / drives` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e61ddfebaccbb9ee:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Logical focus drives activation; keyboard modality only decides whether its ring is painted.
// ADDED-LINE-RATIONALE 205a90ae4a373ce2:2: Freezes `std` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d6f7e439815a090f:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0b72451ac1e7c1fd:1: Freezes `focusVisible_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 207d5e23ffc354fb:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 02fc4b59a20b662e:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `Activate / focus_` work to `Activate` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e35c684fd14d1b83:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `Activate / focus_` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE c0d019fc8c87914b:2: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `Activate / focus_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3013a1266148ee2b:13: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `stage_ / Stage / Language` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a748db98b5939947:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 05afeb2bbdc00983:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:13: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `stage_ / Stage / Removed` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 667e75bc08cb60dc:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / WM_DESTROY` state.
// ADDED-LINE-RATIONALE 886d40998e917954:1: Freezes `auto` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE eba9a2a129b1d0e1:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `closedProvider / UiaRaiseAutomationEvent / closedProvider` state.
// ADDED-LINE-RATIONALE c20110d49aa03734:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `KillTimer / hwnd_ / kTickTimer` work to `KillTimer` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:14: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `KillTimer / hwnd_ / kTickTimer` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6a9d893ea95bc351:1: Keeps `L / Redrob / Launcher` in `REDROB_REQUIRE_BUNDLED_FONTS`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 646dc881af4cf6b3:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: What a click (on release) or a focused Enter/Space does. The same actions the screens always had.
// ADDED-LINE-RATIONALE fe682d0665577deb:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8275a743d08f7711:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9a2538e1a7429980:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 00347d3fd3d54c4d:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: The two buttons ARE the choice: picking one moves on, in that language.
// ADDED-LINE-RATIONALE 8035e1146da0db7f:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:43: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `ctl / kErrClose / ChooseLanguage` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 86ec3c0db43a918e:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5651095fad1c7425:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7cb1d051778f94ac:1: Freezes `bool` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 61ca3239616205f9:1: Freezes `deleteData_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4a366927ae06123b:5: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 522663f19ffacc0d:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `RaiseToggleChanged / before / deleteData_` work to `RaiseToggleChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c98aec4071288263:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `RaiseToggleChanged / before / deleteData_` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:15: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `ctl / kRetry / StartUninstall` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 866626c4cd3ee377:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cd0bebf1d3ec10eb:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:44: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `ctl / kErrClose / RequestClose` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 49616ed5788ceafa:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5651095fad1c7425:2: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f503ee9ea2288c73:1: Freezes `bool` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 174e2a0949f0fd95:1: Freezes `desktopShortcut_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4a366927ae06123b:6: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0905e687689b5630:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `RaiseToggleChanged / before / desktopShortcut_` work to `RaiseToggleChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c98aec4071288263:2: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `RaiseToggleChanged / before / desktopShortcut_` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:16: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `ctl / kRetry / StartInstall` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4c4fa52549525609:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78a35af8b4c6068d:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:45: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `ctl / kErrClose / SetStage` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 23c70f6f723eba27:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d4631f81284a72c3:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4a8e3e474c7903bf:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `errorMessage_ / clear` work to `errorMessage_.clear` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a557499304477224:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetStage / Stage / Confirm` work to `SetStage` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5a4fc9342670f443:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `SetStage / Stage / Confirm` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:46: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `stage_ / Stage / Error` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d766b642c2e77b34:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6fefee418f5b5c3a:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:22: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `stage_ / Stage / Error` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 32069b812d533a8f:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: The control under `pt` (device px). Only the controls the current screen drew have a rect.
// ADDED-LINE-RATIONALE 37098d8bcf6f62aa:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `PtInRect / rcClose_ / pt` state.
// ADDED-LINE-RATIONALE 85fd2fa544aa5be2:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `PtInRect / rcMin_ / pt` state.
// ADDED-LINE-RATIONALE 35d43171394aac3e:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `PtInRect / rcRetry_ / pt` state.
// ADDED-LINE-RATIONALE e740e8dc39ee9126:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `PtInRect / rcErrClose_ / pt` state.
// ADDED-LINE-RATIONALE c709fd38611c7d8a:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `PtInRect / rcCheck_ / pt` state.
// ADDED-LINE-RATIONALE adec84e076fa75fe:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `kNone` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:23: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `kNone` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a23661c27be6ce6d:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetupApp / UpdateHover` work to `SetupApp::UpdateHover` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8fdad656143fa55a:2: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `hwnd_` state.
// ADDED-LINE-RATIONALE 7f1d28d52e88fc33:1: Makes `POINT / pt` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE b7bed90705909162:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `GetCursorPos / pt` work to `GetCursorPos` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ee9674d6ea06b370:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `ScreenToClient / hwnd_ / pt` work to `ScreenToClient` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6090f45c81bdca22:1: Makes `RECT / rc` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE f2ef55ce058f7f6e:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `GetClientRect / hwnd_ / rc` work to `GetClientRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 709be6f2f4b1b1f2:1: Freezes `int` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1a2af18e3094c5e5:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:24: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `hit / hover_ / hover_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3d171486c304774d:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Tab order, left to right as drawn: the checkbox, then the secondary button, then the primary.
// ADDED-LINE-RATIONALE baac44c763bad09b:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `switch / stage_` state.
// ADDED-LINE-RATIONALE e05912b2b1ec6fd6:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / Stage / Language` state.
// ADDED-LINE-RATIONALE b6b08aa6884b7d55:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / Stage / Welcome` state.
// ADDED-LINE-RATIONALE b56755496b4a5b82:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / Stage / Error` state.
// ADDED-LINE-RATIONALE 7454e89a3641229e:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `case / Stage / Removed` state.
// ADDED-LINE-RATIONALE cecc5d3de8817c74:1: Makes `default` an explicit part of `REDROB_REQUIRE_BUNDLED_FONTS` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE a46cedb895787deb:47: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `default` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:25: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `default` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 02604c852a8a1cdd:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `bool / SetupApp / IsFocusable` work to `SetupApp::IsFocusable` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5c5f1a492c7d5e42:1: Freezes `std` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 30dbbcd5b9304347:1: Freezes `return` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:26: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `std / find / order` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 057ee52427844c37:1: Freezes `bool` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8f05ba17d5785be8:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetupApp / SetLogicalFocus / int` work to `SetupApp::SetLogicalFocus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 27c9fa057e7d8b68:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d417c61b374c6273:1: Freezes `bool` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 366054d1fcff2624:1: Freezes `focus_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e6d1c5e67152fcae:1: Freezes `focusVisible_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 75e379dae758cfcb:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `hwnd_ / InvalidateRect / hwnd_` state.
// ADDED-LINE-RATIONALE 806f771e539ac934:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `changed / WindowHasKeyboardFocus` state.
// ADDED-LINE-RATIONALE 180efa7548dd6aa0:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `RaiseFocusChanged / A11yProvider / IdForControl` work to `RaiseFocusChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:27: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `RaiseFocusChanged / A11yProvider / IdForControl` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE fbb3d6a4a604d7ad:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetupApp / StartAnim` work to `SetupApp::StartAnim` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8fdad656143fa55a:3: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `hwnd_` state.
// ADDED-LINE-RATIONALE d946c2ca65682180:1: Prevents `REDROB_REQUIRE_BUNDLED_FONTS` from applying its side effect outside the `reducedMotion_` state.
// ADDED-LINE-RATIONALE 05c386007f5465a5:1: Freezes `for` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 195038391fe34a9b:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5a4fc9342670f443:2: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `InvalidateRect / hwnd_ / nullptr` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:48: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `animLast_ / GetTickCount64` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5e133689ed20dfe3:1: Freezes `animLast_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9322c39b4eadd685:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `SetTimer / hwnd_ / kAnimTimer` work to `SetTimer` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ef24fd670984167b:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:28: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `InvalidateRect / hwnd_ / nullptr` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0542df8c7a27d811:1: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `bool / SetupApp / AnimStep` work to `SetupApp::AnimStep` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6f4598f4de43e7b5:1: Freezes `ULONGLONG` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1b34805b6dfdc81e:1: Freezes `float` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7ea5d58932f996ee:1: Freezes `animLast_` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 886583e96a3bacdb:1: Freezes `bool` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0cd21f5489c3fccd:1: Freezes `for` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5819a2c3f44ba1e5:1: Freezes `float` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 52ee29a90b1d0315:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 05c6d91c0ebe05e4:1: Freezes `else` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cc2d203f3e52bd6f:1: Freezes `if` for `REDROB_REQUIRE_BUNDLED_FONTS` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:49: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `hoverT_ / i / target` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 20e9088ca088bfd9:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `moving / StageFade / f` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:29: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `moving / StageFade / f` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 41ad4b87e2032045:1: Supports `StandardEase` while preserving the nearby constraint: DS standard easing, cubic-bezier(0.2, 0, 0, 1), solved for x by bisection.
// ADDED-LINE-RATIONALE a4bc52f039c67b3c:1: Freezes `if` for `StandardEase` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 52f3ded69a72bdb2:1: Freezes `if` for `StandardEase` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c4d4b7127d595643:1: Freezes `auto` for `StandardEase` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ffd15a62fd8b89a6:1: Freezes `float` for `StandardEase` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4ec26f1002c0bc51:1: Freezes `for` for `StandardEase` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5eadf05d34baf5fb:1: Stops `StandardEase` at the `bez / f / f` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:30: Bounds the `StandardEase` implementation here so `bez / f / f` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 65e2f232e54f173f:1: Delegates `StandardEase`'s `float / SetupApp / Eased` work to `SetupApp::Eased` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 19d301ff4705d5e3:1: Supports `StandardEase` while preserving the nearby constraint: The current screen's content opacity: 0 -> 1 over 220 ms after the screen changes; 1 under reduced motion.
// ADDED-LINE-RATIONALE efbe4d2b3193238a:1: Prevents `StandardEase` from applying its side effect outside the `reducedMotion_ / hwnd_ / f` state.
// ADDED-LINE-RATIONALE 8508b5243ca2f0de:1: Stops `StandardEase` at the `StandardEase / std / min` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:31: Bounds the `StandardEase` implementation here so `StandardEase / std / min` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1aa970c533c65f27:1: Supports `StandardEase` while preserving the nearby constraint: A new screen starts with no virtual child focused (Enter keeps its primary-action meaning).
// ADDED-LINE-RATIONALE 339b070e91e36766:1: Freezes `focus_` for `StandardEase` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3b8bb717ac005154:1: Freezes `focusVisible_` for `StandardEase` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 82d6a75c1fee9674:1: Freezes `pressed_` for `StandardEase` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f459f6a8c8bd6088:1: Prevents `StandardEase` from applying its side effect outside the `hwnd_ / UpdateHover / StartAnim` state.
// ADDED-LINE-RATIONALE f96f56e78d0d9c2d:1: Prevents `StandardEase` from applying its side effect outside the `hwnd_` state.
// ADDED-LINE-RATIONALE 195038391fe34a9b:2: Delegates `StandardEase`'s `InvalidateRect / hwnd_ / nullptr` work to `InvalidateRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1434a37c45949829:1: Supports `StandardEase` while preserving the nearby constraint: Paint synchronously so a stage-change event never exposes stale fragment bounds.
// ADDED-LINE-RATIONALE a46cedb895787deb:50: Bounds the `StandardEase` implementation here so `UpdateWindow / hwnd_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6f07243f5990e701:1: Delegates `StandardEase`'s `RaiseStructureChanged` work to `RaiseStructureChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 19b5eda185369043:1: Supports `StandardEase` while preserving the nearby constraint: Structure comes first so clients cannot resolve the old child when focus moves to the fragment root.
// ADDED-LINE-RATIONALE 3249de016efa60d2:1: Supports `EaseOut` while preserving the nearby constraint: Ease displayed progress toward the target (motion < 1/3 s per step).
// ADDED-LINE-RATIONALE 537029ad782064b7:1: Delegates `EaseOut`'s `RaiseProgressChanged / previousProgress / progress_` work to `RaiseProgressChanged` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d32c54cf0da8ee73:1: Freezes `symbolBitmap_` for `EaseOut` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 68b2bf7261447b1f:1: Delegates `EaseOut`'s `symbolBitmap_ / reset` work to `symbolBitmap_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a57804c8a8984bfc:1: Supports `StepState` while preserving the nearby constraint: Where a screen sits among the three steps the stepper shows.
// ADDED-LINE-RATIONALE 30b4b6413603cc93:1: Delegates `StepOf`'s `StepState / StepOf / Stage` work to `StepOf` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 39d394af8b2eac2b:1: Freezes `if` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bb51fedcc4c37939:1: Prevents `StepOf` from applying its side effect outside the `switch / s` state.
// ADDED-LINE-RATIONALE cc3aed84e5857ebe:1: Prevents `StepOf` from applying its side effect outside the `case / Stage / Removing` state.
// ADDED-LINE-RATIONALE 5c4f1852e65e19b7:1: Prevents `StepOf` from applying its side effect outside the `case / Stage / Removed` state.
// ADDED-LINE-RATIONALE 4b22c2258a169fb7:1: Makes `default` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:20: Bounds the `StepOf` implementation here so `default` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a46cedb895787deb:53: Bounds the `StepOf` implementation here so `switch / s` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 7ce09727dcce6599:3: Prevents `StepOf` from applying its side effect outside the `switch / s` state.
// ADDED-LINE-RATIONALE 734ba1fdfa79d082:1: Prevents `StepOf` from applying its side effect outside the `case / Stage / Language` state.
// ADDED-LINE-RATIONALE f9593fa9c7a1d5c2:1: Prevents `StepOf` from applying its side effect outside the `case / Stage / Opening` state.
// ADDED-LINE-RATIONALE 8c26e2d3512f2d59:1: Makes `default / Welcome / Preparing` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE a46cedb895787deb:54: Bounds the `StepOf` implementation here so `default / Welcome / Preparing` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:40: Bounds the `StepOf` implementation here so `default / Welcome / Preparing` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE be00cc4c112425ee:1: Supports `StepOf` while preserving the nearby constraint: between any two Hangul syllables, so a word joiner (U+2060, zero width) is put inside each word.
// ADDED-LINE-RATIONALE 5aa0747d43b72f3b:1: Freezes `auto` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dbd772e936e0075c:2: Makes `std / wstring / out` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 83bff7e63277f681:1: Delegates `StepOf`'s `out / reserve / s` work to `out.reserve` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE aa3670e66fab2ff6:1: Freezes `for` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3a7d3b4fa7895c95:1: Freezes `out` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b132c1923f5d98e9:1: Prevents `StepOf` from applying its side effect outside the `i / s / size` state.
// ADDED-LINE-RATIONALE 2c1c789686994a88:1: Freezes `out` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:55: Bounds the `StepOf` implementation here so `out / L / u2060` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d4d77feafb8a847b:4: Stops `StepOf` at the `out` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:41: Bounds the `StepOf` implementation here so `out` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e54a2cdcec31a9ec:1: Delegates `StepOf`'s `float / SetupApp / TextWidth` work to `SetupApp::TextWidth` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 570183aa3eb53ced:2: Makes `ComPtr / IDWriteTextLayout / layout` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE f4826ba6ca2178da:1: Prevents `StepOf` from applying its side effect outside the `f / FAILED / dwFactory_` state.
// ADDED-LINE-RATIONALE cb96df67d99df1b9:1: Delegates `StepOf`'s `ApplyHangulFont / layout / p` work to `ApplyHangulFont` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4ec8f3cb473d87f9:1: Makes `DWRITE_TEXT_METRICS / tm` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 2cdfd4bc3abce4b5:1: Stops `StepOf` at the `SUCCEEDED / layout / GetMetrics` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:42: Bounds the `StepOf` implementation here so `SUCCEEDED / layout / GetMetrics` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8444b17849c49546:1: Supports `StepOf` while preserving the nearby constraint: Left-aligned, wrapped text at (x, y), `w` wide. Returns the height it took.
// ADDED-LINE-RATIONALE c6c63bc1af689bf7:1: Prevents `StepOf` from applying its side effect outside the `f / s / empty` state.
// ADDED-LINE-RATIONALE dbdcd5302f8d5a6f:1: Freezes `std` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 570183aa3eb53ced:3: Makes `ComPtr / IDWriteTextLayout / layout` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 446d2664936872cb:1: Prevents `StepOf` from applying its side effect outside the `FAILED / dwFactory_ / CreateTextLayout` state.
// ADDED-LINE-RATIONALE e3d952ecb315136b:2: Delegates `StepOf`'s `ApplyHangulFont / layout / p` work to `ApplyHangulFont` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 24b9e52ccf35dba5:1: Freezes `layout` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1fa97360d1c0525e:1: Delegates `StepOf`'s `rt_ / DrawTextLayout / D2D1` work to `DrawTextLayout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4ec8f3cb473d87f9:2: Makes `DWRITE_TEXT_METRICS / tm` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 152f538b6e25f21a:1: Stops `StepOf` at the `SUCCEEDED / layout / GetMetrics` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:43: Bounds the `StepOf` implementation here so `SUCCEEDED / layout / GetMetrics` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 01d4eb3bc663c983:1: Supports `StepOf` while preserving the nearby constraint: symbol is the design system's file, drawn as is: never tinted, shadowed or redrawn.
// ADDED-LINE-RATIONALE 6c85892725603da6:1: Freezes `D2D1_RECT_F` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cd2d5580a7e1cad1:1: Freezes `auto` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 96adff98431f9ae0:1: Makes `ComPtr / ID2D1GradientStopCollection / coll` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 673617a8fc600629:1: Prevents `StepOf` from applying its side effect outside the `FAILED / rt_ / CreateGradientStopCollection` state.
// ADDED-LINE-RATIONALE 7959349e66be4d67:1: Makes `ComPtr / ID2D1RadialGradientBrush / brush` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 8c47eb3cc8a82855:1: Prevents `StepOf` from applying its side effect outside the `FAILED / rt_ / CreateRadialGradientBrush` state.
// ADDED-LINE-RATIONALE 8290f2efe63b0b2a:1: Delegates `StepOf`'s `D2D1 / RadialGradientBrushProperties / c` work to `D2D1::RadialGradientBrushProperties` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 367283a9a71db542:1: Delegates `StepOf`'s `rt_ / FillRectangle / r` work to `FillRectangle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dd95c0a5fbbdc870:2: Bounds the `StepOf` implementation here so `rt_ / FillRectangle / r` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 13efd05f5d857dcf:1: Freezes `D2D1_GRADIENT_STOP` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7d876a2b7fe206be:1: Delegates `StepOf`'s `radial / D2D1 / Point2F` work to `radial` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b85a7f97536464f6:1: Freezes `D2D1_COLOR_F` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1fe3718a8e877f42:1: Freezes `D2D1_GRADIENT_STOP` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fadc4f8fb9a31311:1: Delegates `StepOf`'s `radial / D2D1 / Point2F` work to `radial` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 61bf50782de7b3cb:1: Freezes `D2D1_COLOR_F` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 50eeb90a4c92a4cd:1: Freezes `v0` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 45cfd07850bdc962:1: Freezes `D2D1_GRADIENT_STOP` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c5eccae86b67a2ed:1: Delegates `StepOf`'s `radial / D2D1 / Point2F` work to `radial` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cea7efa1c309f725:1: Prevents `StepOf` from applying its side effect outside the `symbolBitmap_` state.
// ADDED-LINE-RATIONALE d2d27e89695c3ccb:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c985ed85cb2f3f58:1: Keeps `rt_ / DrawBitmap / symbolBitmap_` in `StepOf`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 70d320c7e3c46dc3:1: Makes `ComPtr / ID2D1SolidColorBrush / white` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 96f0ac2f0e328f57:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / kWhite` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1a73621932351087:1: Prevents `StepOf` from applying its side effect outside the `white / DrawBlock / L` state.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:44: Bounds the `StepOf` implementation here so `white / DrawBlock / L` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 207d6ca8953d2efa:1: Supports `StepOf` while preserving the nearby constraint: halo, next = outline), each with its label, joined by a hairline (blue once passed).
// ADDED-LINE-RATIONALE b410295e85ef12fc:1: Freezes `Palette` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2819c042edf27628:1: Makes `ComPtr / ID2D1SolidColorBrush / ink` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 482e66e8f89cdc9a:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1cc1033ecea6c752:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4fd3b9cc4cf73c8a:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE db26730821e58894:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0324fe5922dafb2a:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8af3afcfaac51a84:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f9169627858e7bb4:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dc9c410d81af2bda:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE da738ba21de1d43a:1: Prevents `StepOf` from applying its side effect outside the `ink / white` state.
// ADDED-LINE-RATIONALE f73affd542e66c62:1: Freezes `StepState` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f6bb82e976190c26:1: Makes `wchar_t / labels` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 650ebcffb076b3f4:3: Freezes `if` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3f621174da3f7e01:1: Freezes `labels` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bb9a564027981125:2: Makes `labels / Tr / L` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE abe947eb75da8074:1: Freezes `labels` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cedb895787deb:57: Bounds the `StepOf` implementation here so `labels / Tr / L` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE fd851d32502d8b71:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6551faef38a9898b:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 66d0c6d0840355d9:1: Freezes `for` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 15defdce9157e210:1: Freezes `bool` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f3f20efc67717a57:1: Freezes `bool` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d7234f50cbbba8c9:1: Freezes `D2D1_POINT_2F` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6367f02b5d92184f:1: Prevents `StepOf` from applying its side effect outside the `cur` state.
// ADDED-LINE-RATIONALE 5c1d7e6e2f3c7600:1: Delegates `StepOf`'s `rt_ / FillEllipse / D2D1` work to `FillEllipse` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7a7b182aff4a23ff:1: Delegates `StepOf`'s `rt_ / FillEllipse / D2D1` work to `FillEllipse` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 42918f351002f3fc:1: Delegates `StepOf`'s `rt_ / DrawEllipse / D2D1` work to `DrawEllipse` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 50d5f1a76526bb5f:1: Delegates `StepOf`'s `done` work to `if` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a7c16d01616894de:1: Delegates `StepOf`'s `rt_ / FillEllipse / D2D1` work to `FillEllipse` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a72028ce495965bc:1: Delegates `StepOf`'s `rt_ / DrawLine / D2D1` work to `DrawLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e8b97df833dbd885:1: Delegates `StepOf`'s `rt_ / DrawLine / D2D1` work to `DrawLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b62de026336335c5:1: Makes `rt_ / DrawLine / D2D1` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 7a7b182aff4a23ff:2: Delegates `StepOf`'s `rt_ / FillEllipse / D2D1` work to `FillEllipse` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 58bab698d072db93:1: Delegates `StepOf`'s `rt_ / DrawEllipse / D2D1` work to `DrawEllipse` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:21: Bounds the `StepOf` implementation here so `rt_ / DrawEllipse / D2D1` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 2262cf0139b6b34e:1: Prevents `StepOf` from applying its side effect outside the `done / fmtStep_` state.
// ADDED-LINE-RATIONALE 7be022db17ad31e0:1: Freezes `wchar_t` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 97ec22fd33844e2d:1: Makes `ComPtr / IDWriteTextLayout / layout` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 911b5b3dc8fd04cb:1: Prevents `StepOf` from applying its side effect outside the `SUCCEEDED / dwFactory_ / CreateTextLayout` state.
// ADDED-LINE-RATIONALE 5abde0ee32cceb70:1: Delegates `StepOf`'s `layout / SetTextAlignment / DWRITE_TEXT_ALIGNMENT_CENTER` work to `SetTextAlignment` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 473858b8b040d223:1: Delegates `StepOf`'s `rt_ / DrawTextLayout / D2D1` work to `DrawTextLayout` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3013a1266148ee2b:15: Bounds the `StepOf` implementation here so `rt_ / DrawTextLayout / D2D1` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:22: Bounds the `StepOf` implementation here so `float / lw / TextWidth` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 45eea8c0d41ff44a:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 360d79f907a98369:1: Prevents `StepOf` from applying its side effect outside the `fmtStep_` state.
// ADDED-LINE-RATIONALE 0ca0d9107842eacd:1: Keeps `DrawTextLine / labels / i` in `StepOf`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2747f81d0f181370:1: Makes `done / cur / ink` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:23: Bounds the `StepOf` implementation here so `done / cur / ink` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d0da7cb201cbc15f:1: Freezes `cx` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6d9d5207e2d64112:1: Prevents `StepOf` from applying its side effect outside the `i` state.
// ADDED-LINE-RATIONALE 03af186d7a08ca36:1: Keeps `rt_ / DrawLine / D2D1` in `StepOf`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE df39bd428aa8a486:1: Makes `i / st / current` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 97fd159e7340178c:1: Freezes `cx` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:24: Bounds the `StepOf` implementation here so `cx` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a46cedb895787deb:58: Bounds the `StepOf` implementation here so `cx` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE d88568f2b46d8dcb:45: Bounds the `StepOf` implementation here so `HRESULT / SetupApp / DrawScene` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 01718b9045f57ed5:1: Delegates `StepOf`'s `HRESULT / SetupApp / DrawScene` work to `SetupApp::DrawScene` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b410295e85ef12fc:2: Freezes `Palette` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ae87d4ede9e71e69:1: Delegates `StepOf`'s `rt_ / BeginDraw` work to `BeginDraw` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 70a90ca691380944:1: Delegates `StepOf`'s `rt_ / Clear / p` work to `Clear` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4f0ce2c3b71954f4:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE acae4e568c9661a5:1: Freezes `rcBrand_` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2236a72640216e37:1: Delegates `StepOf`'s `DrawBrandPanel / H` work to `DrawBrandPanel` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f53808ee0d4cc2a3:1: Makes `ComPtr / ID2D1SolidColorBrush / ink` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 482e66e8f89cdc9a:2: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1cc1033ecea6c752:2: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4fd3b9cc4cf73c8a:2: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b0d9987f5c64eb66:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE db26730821e58894:2: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8af3afcfaac51a84:2: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 96f0ac2f0e328f57:3: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / kWhite` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 064e7505573cc2a0:1: Prevents `StepOf` from applying its side effect outside the `ink / white / rt_` state.
// ADDED-LINE-RATIONALE 6b570cafdacbd9c3:1: Supports `StepOf` while preserving the nearby constraint: -- caption buttons (minimise, close), 46 x 40 each, top-right ------.
// ADDED-LINE-RATIONALE bd9104d9ee5088b7:1: Freezes `rcMin_` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a46cf3b89578881d:1: Bounds the `StepOf` implementation here so `rcMin_ / ToDevice / W` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 212ab2d48e5fc71f:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 647ee7758e65c95b:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3c60bbb88c0c85d6:1: Supports `StepOf` while preserving the nearby constraint: Hover: 8% ink behind minimise; status-danger behind close, with a white glyph.
// ADDED-LINE-RATIONALE abca0fbe3c3e0ee4:1: Prevents `StepOf` from applying its side effect outside the `hm` state.
// ADDED-LINE-RATIONALE 8c5f8696473e88ed:1: Makes `ComPtr / ID2D1SolidColorBrush / fill` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 3dec1097b6f1f6e0:1: Freezes `D2D1_COLOR_F` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 711788612d99a61a:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / c` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 971e6414790fa391:1: Prevents `StepOf` from applying its side effect outside the `fill / rt_ / FillRectangle` state.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:25: Bounds the `StepOf` implementation here so `fill / rt_ / FillRectangle` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8a70ff41213e8be4:1: Makes `ComPtr / ID2D1SolidColorBrush / closeGlyph` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 8299e8f6b8874b9a:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / Mix` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cda5b0597e8f4d9a:1: Prevents `StepOf` from applying its side effect outside the `hc` state.
// ADDED-LINE-RATIONALE 8c5f8696473e88ed:2: Makes `ComPtr / ID2D1SolidColorBrush / fill` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE f74bf11427e887a1:1: Freezes `D2D1_COLOR_F` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 711788612d99a61a:2: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / c` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 44eef909a0e5198a:1: Prevents `StepOf` from applying its side effect outside the `fill / rt_ / FillRectangle` state.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:26: Bounds the `StepOf` implementation here so `fill / rt_ / FillRectangle` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 955a5ae300d059b1:1: Delegates `StepOf`'s `rt_ / DrawLine / D2D1` work to `DrawLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8d052e74ba059805:1: Delegates `StepOf`'s `rt_ / DrawLine / D2D1` work to `DrawLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c9da5514c540065f:1: Delegates `StepOf`'s `rt_ / DrawLine / D2D1` work to `DrawLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3213ef419c9d721e:1: Supports `StepOf` while preserving the nearby constraint: The content column fades in with each new screen (not the progress bar, which carries on).
// ADDED-LINE-RATIONALE 42ee147b5919e80c:1: Makes `ComPtr / ID2D1Layer / layer` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE e2170229972d0794:1: Prevents `StepOf` from applying its side effect outside the `fade / f / SUCCEEDED` state.
// ADDED-LINE-RATIONALE a54e6f992337cfb0:1: Keeps `rt_ / PushLayer / D2D1` in `StepOf`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 9d3aac769120c0c9:1: Delegates `StepOf`'s `D2D1 / IdentityMatrix / fade` work to `D2D1::IdentityMatrix` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d54efe4f9b1cf1a6:1: Supports `StepOf` while preserving the nearby constraint: -- content column: stepper, title, body, controls; left aligned ------.
// ADDED-LINE-RATIONALE ffa9a3fbe41da461:1: Freezes `rcStepper_` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3b512e12f48d6a7c:1: Delegates `StepOf`'s `DrawStepper / x / f` work to `DrawStepper` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6a964ab9c57c2370:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8fa56d098796c1c7:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8c347489e6aba03c:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bb03cb2958b9cdfa:1: Freezes `rcTitle_` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 28612e7fe36d0390:1: Freezes `y` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d681e01c145adf47:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d76f692438e9fc3c:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 556b54e7a4cafa84:1: Freezes `rcBody_` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 57b2e267fc935cc9:1: Freezes `y` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 01f950df9755ea07:1: Supports `StepOf` while preserving the nearby constraint: 1 px border-strong outline on the ground. Laid out from the right edge; returns its left edge.
// ADDED-LINE-RATIONALE 5647fb3200f6dd45:1: Supports `StepOf` while preserving the nearby constraint: DS focus ring: 2 px focus-ring, 2 px outside the control, following its radius. Keyboard focus only.
// ADDED-LINE-RATIONALE f17b50f19c58e123:1: Supports `StepOf` while preserving the nearby constraint: Offline stage fixtures have no HWND; interactive rings additionally require real window focus.
// ADDED-LINE-RATIONALE bcac4cbbb938f2fc:1: Makes `ComPtr / ID2D1SolidColorBrush / ring` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE c57f546b0e06c8f8:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / p` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 051c067e1f92d8b1:1: Prevents `StepOf` from applying its side effect outside the `ring / rt_ / DrawRoundedRectangle` state.
// ADDED-LINE-RATIONALE 9b2af98a47974415:1: Makes `radius / radius / ring` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 349f3a2824093489:1: Supports `StepOf` while preserving the nearby constraint: Pressed reads as full hover (DS: action-primary-hover is hover AND pressed).
// ADDED-LINE-RATIONALE bbfd05d5343c8860:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4a1908789f93cc6b:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 58f91fcbc3326bb7:1: Freezes `D2D1_RECT_F` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 816db4a20f39d07f:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6504e987efd4dfcd:1: Makes `ComPtr / ID2D1SolidColorBrush / fill` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 92a429635732f29f:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / primary` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5b16946734914df6:1: Prevents `StepOf` from applying its side effect outside the `fill / rt_ / FillRoundedRectangle` state.
// ADDED-LINE-RATIONALE 37b07f1d0565de93:1: Prevents `StepOf` from applying its side effect outside the `primary` state.
// ADDED-LINE-RATIONALE 2c37eaa1973ca04a:1: Keeps `rt_ / DrawRoundedRectangle / D2D1` in `StepOf`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ad8e63cdafb0e5f0:1: Makes `strong / p / f` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE e45bf20ac947a64f:1: Delegates `StepOf`'s `focusRing / ctl / br` work to `focusRing` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 48baf30f619e70b1:1: Delegates `StepOf`'s `DrawTextLine / label / fmtButton_` work to `DrawTextLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3a40cf76edf8d749:1: Freezes `rc` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 697c6852165d4057:1: Stops `StepOf` at the `l` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE dd95c0a5fbbdc870:4: Bounds the `StepOf` implementation here so `l` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 839f67783b4694f7:1: Freezes `auto` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f81c3e5ab53df92b:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a7e155d557c4dc03:1: Delegates `StepOf`'s `button / rcErrClose_ / kErrClose` work to `button` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dd95c0a5fbbdc870:5: Bounds the `StepOf` implementation here so `button / rcErrClose_ / kErrClose` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ae3fbf0591ca1d34:1: Supports `StepOf` while preserving the nearby constraint: label) is the hit target; hovering it fills an empty box with sunken, a checked one with the hover blue.
// ADDED-LINE-RATIONALE b7805ea39d850174:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0978c0123843c83d:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 57b4a457d6e2541d:1: Freezes `D2D1_RECT_F` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f670c0ce54de73c0:1: Freezes `D2D1_ROUNDED_RECT` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6504e987efd4dfcd:2: Makes `ComPtr / ID2D1SolidColorBrush / fill` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 19077f1e2c793b70:1: Delegates `StepOf`'s `rt_ / CreateSolidColorBrush / checked` work to `CreateSolidColorBrush` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b90c7e97463eee98:1: Prevents `StepOf` from applying its side effect outside the `fill / rt_ / FillRoundedRectangle` state.
// ADDED-LINE-RATIONALE c9fbea133b35e93e:1: Delegates `StepOf`'s `rt_ / DrawLine / D2D1` work to `DrawLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9c29c83e49343332:1: Delegates `StepOf`'s `rt_ / DrawLine / D2D1` work to `DrawLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3b35f8eec8129ac4:1: Keeps `rt_ / DrawRoundedRectangle / D2D1` in `StepOf`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 62fe48d593a1dd0f:1: Makes `f / f / strong` an explicit part of `StepOf` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE 592b17db1b6116b4:1: Delegates `StepOf`'s `focusRing / kCheck / boxR` work to `focusRing` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 29e5363d533a4d68:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6c6d66fa98f38733:1: Delegates `StepOf`'s `DrawTextLine / label / fmtLabel_` work to `DrawTextLine` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ea6fcee8873d0e38:1: Freezes `rcCheck_` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c3bc83aec69daa0d:1: Freezes `rcRetry_` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 396ff3b69cac4e8d:1: Supports `StepOf` while preserving the nearby constraint: one (a previous install's answer, or the Windows UI language) is drawn as the primary.
// ADDED-LINE-RATIONALE c3dafe964b8cd1f7:1: Freezes `button` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a96325644a1e2091:1: Delegates `StepOf`'s `checkRow / y / f` work to `checkRow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4159dc365b6adb9f:1: Delegates `StepOf`'s `twoButtons / Tr / L` work to `twoButtons` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3cf11b8ca849ff19:1: Delegates `StepOf`'s `checkRow / y / f` work to `checkRow` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c67ac3b31ac19c63:1: Delegates `StepOf`'s `twoButtons / Tr / L` work to `twoButtons` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6a0fffc7825acf99:1: Delegates `StepOf`'s `button / rcRetry_ / kRetry` work to `button` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 853c1f4a2c19a9cc:1: Delegates `StepOf`'s `stage_ / Stage / Error` work to `if` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 25e73c3186229dc4:1: Delegates `StepOf`'s `twoButtons / Tr / L` work to `twoButtons` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c2eb8d9ec14ab725:1: Prevents `StepOf` from applying its side effect outside the `layer / rt_ / PopLayer` state.
// ADDED-LINE-RATIONALE 1370fd6a13a60f3d:1: Supports `StepOf` while preserving the nearby constraint: -- DS Progress: 8 px pill track (sunken), blue bar ------------.
// ADDED-LINE-RATIONALE 22d993d80ed14044:1: Freezes `rcProgress_` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 36eaf3adcbd626fb:1: Delegates `StepOf`'s `rt_ / FillRoundedRectangle / D2D1` work to `FillRoundedRectangle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e31683e2f5efd9d3:1: Freezes `float` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1989d32aaaf516a0:1: Prevents `StepOf` from applying its side effect outside the `fillW / f` state.
// ADDED-LINE-RATIONALE 391b04c869d8302d:1: Delegates `StepOf`'s `rt_ / FillRoundedRectangle` work to `FillRoundedRectangle` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b320580aae5da815:1: Delegates `StepOf`'s `D2D1 / RoundedRect / D2D1` work to `D2D1::RoundedRect` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:28: Bounds the `StepOf` implementation here so `D2D1 / RoundedRect / D2D1` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ac9c5c86a1967915:1: Prevents `StepOf` from applying its side effect outside the `layer / rt_ / PopLayer` state.
// ADDED-LINE-RATIONALE 49a741a1cc12ae98:1: Supports `StepOf` while preserving the nearby constraint: Render automation accepts a fresh nested destination rather than failing each encoder opaquely.
// ADDED-LINE-RATIONALE 42c7ccb07a0ecb27:1: Freezes `if` for `StepOf` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cdce94e135fde5d3:2: Isolates `REDROB_REQUIRE_BUNDLED_FONTS` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE cce1a7241a5496a2:2: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: ifdef REDROB_REQUIRE_BUNDLED_FONTS.
// ADDED-LINE-RATIONALE 69c9133d0256415d:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: Headless validation reports a font-resource failure distinctly and releases COM in-order.
// ADDED-LINE-RATIONALE 4edea240ed046be2:2: Delegates `REDROB_REQUIRE_BUNDLED_FONTS`'s `fontColl_ / reset / wicFactory_` work to `fontColl_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6a44e85217079764:1: Stops `REDROB_REQUIRE_BUNDLED_FONTS` at the `fontColl_ / reset / wicFactory_` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE a46cedb895787deb:61: Bounds the `REDROB_REQUIRE_BUNDLED_FONTS` implementation here so `endif` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 025d4ae15d9fc9fd:4: Keeps `REDROB_REQUIRE_BUNDLED_FONTS` on the `endif` compile-time path required by the supported native targets.
// ADDED-LINE-RATIONALE 7b9b469a060782a7:1: Supports `REDROB_REQUIRE_BUNDLED_FONTS` while preserving the nearby constraint: endif.
// ADDED-LINE-RATIONALE 6bb9d61fb0a51623:1: Isolates `Shot` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE fd3d8ca9c3690990:1: Supports `Shot` while preserving the nearby constraint: states 1: pointer over Back, keyboard focus on Install; 2: pointer over close, focus on the checkbox.
// ADDED-LINE-RATIONALE 38f9bda5214d728b:1: Keeps `Stage / Welcome / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3ceb8e7789d01fb8:1: Keeps `Stage / Language / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2282a5dc4778d1ce:1: Keeps `Stage / Language / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 6a231972da8f2854:1: Keeps `Stage / Welcome / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 814fb674c97ae5bc:1: Keeps `Stage / Welcome / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0ba5ffb506daa4ac:1: Keeps `Stage / Welcome / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 4f0fb8d3943f72e6:1: Keeps `Stage / Preparing / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a34b8ed511d51f24:1: Keeps `Stage / Preparing / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 8ad7b2964c85cade:1: Keeps `Stage / Installing / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 4638ba88768d994c:1: Keeps `Stage / Installing / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 9e5eb2eed42b2d96:1: Keeps `Stage / Opening / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 21bff9fd1ec9e274:1: Keeps `Stage / Opening / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 04942639fd470cda:1: Keeps `Stage / Error / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE fca780ef06401958:1: Keeps `Stage / Error / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5214fafd35f0d095:1: Keeps `Stage / Error / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 9d3fdf600ceb3ffd:1: Keeps `Stage / Error / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 8a6d3f64714bd8e6:1: Keeps `Stage / Confirm / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 1175d4b310514124:1: Keeps `Stage / Confirm / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 86ba560bef20e768:1: Keeps `Stage / Removing / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a79c9e61b8e37c9e:1: Keeps `Stage / Removing / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3659158598e4886e:1: Keeps `Stage / Removed / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 879c4250d37cc77c:1: Keeps `Stage / Removed / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 0345ba3d9498a0b3:1: Keeps `Stage / Removed / L` in `Shot`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE cf9f97f1d1654d13:1: Freezes `symbolBitmap_` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c126213e19d05250:1: Freezes `mode_` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 841809a3b8104bd6:1: Freezes `desktopShortcut_` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1ad7e8ef0c6fa802:1: Freezes `deleteData_` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2a4be94fd587086b:1: Freezes `for` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c9621980c4c0ec3e:1: Freezes `hover_` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 52de6a4c372678a4:1: Freezes `focusVisible_` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fcf86c045a755f45:1: Freezes `if` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 50d20d2284d0121e:1: Freezes `if` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 05ad59a3f633c8d3:1: Freezes `if` for `Shot` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 011f9f681cb48033:2: Delegates `Shot`'s `fmtTitle_ / reset` work to `fmtTitle_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 91345bca8071bc73:2: Delegates `Shot`'s `fmtBody_ / reset` work to `fmtBody_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e29e507ab22b3e63:2: Delegates `Shot`'s `fmtLabel_ / reset` work to `fmtLabel_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 484c00454ffba325:2: Delegates `Shot`'s `fmtStep_ / reset` work to `fmtStep_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c0ba429a8e6bf7ec:2: Delegates `Shot`'s `fmtBrand_ / reset` work to `fmtBrand_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 919fa3833475813f:2: Delegates `Shot`'s `fontColl_ / reset` work to `fontColl_.reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b6b1588efbcd59d9:1: Delegates `MakeShortcut`'s `link / SetDescription / Tr` work to `SetDescription` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1637b411c2294cee:2: Supports `WriteArpEntry` while preserving the nearby constraint: Silent updates preserve the interactive choice instead of recreating a declined desktop shortcut.
// ADDED-LINE-RATIONALE 64fdda51530bb1b9:2: Freezes `if` for `WriteArpEntry` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cb749eb5b3c51c4a:1: Freezes `else` for `WriteArpEntry` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cd6e2b7e5d45f04e:1: Makes `i` an explicit part of `WriteArpEntry` because the native Windows installer remains accessible, deterministic, and warning-clean.
// ADDED-LINE-RATIONALE e73e58f6944e5f62:1: Freezes `g_themeFlag` for `WriteArpEntry` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 78f0ea09c01e5b0b:33: Bounds the `WriteArpEntry` implementation here so `g_themeFlag / lstrcmpiW / argv` cannot leak into the adjacent declaration or control path.
// SPDX-License-Identifier: MIT OR Apache-2.0
// redrob_launcher_setup -- the branded Windows setup and uninstaller for Redrob Launcher.
//
// Adapted from the Redrob Canvas setup (redrob-labs/redrob-canvas, installer/windows/), itself adapted from
// the Redrob Browser setup window; all our own code. The window and the design-system rules are the same.
// What differs: the first screen asks the language (English or Korean) and every later screen follows
// it; there is no Redrob Code opt-in (the launcher installs Redrob Code itself); a "desktop shortcut" box
// replaces it; and the install records what the launcher reads back.
//
// What it is: a native Win32 app, no dependencies beyond the Windows SDK. It renders a small borderless,
// rounded, DWM-shadowed 720 x 440 window: a Redrob Blue brand panel on the left, and a left-aligned content
// column (step dots, a title, one line under it, the screen's controls, buttons at the bottom right).
//
// Design system (@redrob-labs/ui):
//   * Follows Windows' app theme (AppsUseLightTheme): light = surface-base #FFFFFF, ink #0A0B0C; dark =
//     #0A0B0C, ink #F8F9FB. Every colour is a semantic role (Palette, below).
//   * Brand panel: radial gradient #2E7EFF -> #2B52FF -> #2746EE with a white and a #8B82FF glint, drawn in
//     Direct2D, with the design system's white symbol (assets/brand/symbol-white-192.png) as is.
//   * "Blue acts": the primary button, the checked box and the progress bar are action-primary #2B52FF.
//   * Controls are the DS Button (radius 8, h40), Checkbox (18 px, radius 4), Progress (8 px pill) and a
//     small Stepper, with hover, pressed and keyboard focus (2 px focus ring) states.
//   * Type is Pretendard for Latin and Wanted Sans for Hangul, embedded as a private DirectWrite
//     collection. Production builds fail if deterministic subsets cannot be embedded or loaded.
//   * Motion: 140 ms hover and 220 ms screen fades, DS standard easing; none when Windows' client-area
//     animation is off.
//
// Language: the first screen is bilingual and offers English / 한국어. The choice is stored under
// HKCU\Software\Redrob\Redrob Launcher\Language (en, ko); the launcher adopts it on its first run and
// publishes it for every Redrob app (docs/shared-language.md). The screen defaults to the value a
// previous install recorded, else Korean on a Korean Windows, else English; --lang en|ko overrides.
//
// Install (per user, no administrator rights):
//   1. The payload is the release's own signed portable zip (redrob-launcher.exe and portable.txt at the
//      top level), embedded as RCDATA. It is written to %TEMP% and unpacked with Windows' own tar.exe
//      (bsdtar, in System32 since Windows 10 1803, reads zip) into %LOCALAPPDATA%\Programs\Redrob
//      Launcher.new; portable.txt is dropped, since its presence is what makes the launcher run portable.
//   2. The old install, if any, is renamed aside and the new one renamed into place, so a failed update
//      never leaves half of two versions. A running launcher keeps its files locked; the setup asks the
//      person to close it (the launcher's own self-update closes itself first and runs this --silent).
//   3. The uninstaller (this same app built without a payload) is written into the install folder; the
//      Start menu shortcut (with the AppUserModelID ai.redrob.launcher, which Windows needs to show toast
//      notifications from an unpackaged app) and, if chosen, the desktop shortcut are made; the
//      Settings > Apps entry, the App Paths entry (Win+R "redrob-launcher") and the launcher's own key
//      (InstallDir, Language, DesktopShortcutChoice; read by self_update.rs and language.rs) are written
//      under HKCU.
//   4. Redrob Launcher is opened (not with --silent).
// The steps report no progress of their own, so the bar is a time-based eased ramp capped at ~90% until
// they finish.
//
// Uninstall (Settings > Apps, or the uninstaller directly): confirm, with "Also delete settings" off by
// default; remove the shortcuts, the registry entries and the install folder. The running uninstaller
// cannot delete itself, so a hidden PowerShell removes it and the folder once it exits. The shared account
// and language files (%APPDATA%\Redrob\Shared) belong to every Redrob app and are never touched.
//
// Flags:
//   --lang en|ko         start in that language (skips nothing: the language screen still shows, preselected).
//   --silent             install (or, with --uninstall, remove) with no window; the exit code is the result.
//   --delete-data        with --uninstall --silent: also delete the launcher's settings and cache.
//   --demo               run the whole install sequence on a timer with NO payload.
//   --demo-error         show the error state immediately.
//   --uninstall          remove Redrob Launcher (what Settings > Apps runs).
//   --demo-uninstall     the uninstall sequence on a timer, removing nothing.
//   --render-stages DIR  draw each screen to a PNG in DIR, no window.
//   (no flag)            install the embedded payload for real.

#ifndef UNICODE
#define UNICODE
#endif
#ifndef NOMINMAX
#define NOMINMAX  // keep windows.h's min/max macros from breaking std::min/std::max
#endif

#include <windows.h>
#include <windowsx.h>
#include <dwmapi.h>
#include <d2d1.h>
#include <d2d1helper.h>
#include <dwrite.h>
#include <dwrite_3.h>  // IDWriteFactory5: the in-memory font collection
#include <wincodec.h>
#include <shellapi.h>
#include <shlobj.h>
#include <shobjidl.h>
#include <shlguid.h>
#include <propsys.h>
#include <propvarutil.h>
#include <initguid.h>   // so propkey.h DEFINES the PKEY_* keys in this one translation unit
#include <propkey.h>
#include <UIAutomation.h>  // real fragment providers expose the custom-painted UI to assistive technology
#include <tlhelp32.h>
#include <string>
#include <algorithm>
#include <cctype>
#include <memory>
#include <new>
#include <vector>

#include "resource.h"
#include "build_defs.h"  // generated by build.py: version, resource paths

#pragma comment(lib, "dwmapi.lib")
#pragma comment(lib, "d2d1.lib")
#pragma comment(lib, "dwrite.lib")
#pragma comment(lib, "windowscodecs.lib")
#pragma comment(lib, "shell32.lib")
#pragma comment(lib, "shlwapi.lib")
#pragma comment(lib, "ole32.lib")
#pragma comment(lib, "user32.lib")
#pragma comment(lib, "gdi32.lib")
#pragma comment(lib, "advapi32.lib")
#pragma comment(lib, "propsys.lib")
#pragma comment(lib, "uiautomationcore.lib")
#pragma comment(lib, "oleaut32.lib")

namespace {

// ---- design-system colors (@redrob-labs/ui semantic roles) ------------------
constexpr D2D1_COLOR_F Hex(unsigned v, float a = 1.0f) {
  return {((v >> 16) & 0xFF) / 255.0f, ((v >> 8) & 0xFF) / 255.0f, (v & 0xFF) / 255.0f, a};
}

// One set per theme. The window follows Windows' app theme (light or dark), like the launcher.
struct Palette {
  D2D1_COLOR_F base;         // surface-base: the content column's ground
  D2D1_COLOR_F sunken;       // surface-sunken: progress track, secondary button hover
  D2D1_COLOR_F ink;          // ink-primary: titles, labels
  D2D1_COLOR_F secondary;    // ink-secondary: body copy, caption glyphs
  D2D1_COLOR_F hairline;     // border-subtle: step connectors
  D2D1_COLOR_F strong;       // border-strong: control outlines (secondary button, checkbox)
  D2D1_COLOR_F blue;         // action-primary
  D2D1_COLOR_F blueHover;    // action-primary-hover (hover and pressed)
  D2D1_COLOR_F focus;        // focus-ring
  D2D1_COLOR_F danger;       // status-danger: close-button hover
  D2D1_COLOR_F inkBrand;     // ink-brand: the current step's number
  D2D1_COLOR_F brandSubtle;  // surface-brand-subtle: the current step's halo
};
// Generated semantic roles and fixed brand-art exceptions keep native colours in lockstep.
#include "generated_tokens.h"

bool g_dark = false;  // set from Windows' AppsUseLightTheme at start (or --theme)
int g_themeFlag = 0;  // --theme: 0 follow Windows, 1 light, 2 dark (a dev aid, mainly for --render-stages)
inline const Palette& Pal() { return g_dark ? kDark : kLight; }
// a -> b by t (0..1): hover and pressed states blend between two roles.
inline D2D1_COLOR_F Mix(const D2D1_COLOR_F& a, const D2D1_COLOR_F& b, float t) {
  return {a.r + (b.r - a.r) * t, a.g + (b.g - a.g) * t, a.b + (b.b - a.b) * t, a.a + (b.a - a.a) * t};
}

// Windows' app theme: HKCU\...\Themes\Personalize\AppsUseLightTheme (0 = dark). Read only. Light if unset.
static bool SystemPrefersDark() {
  if (g_themeFlag == 1) return false;
  if (g_themeFlag == 2) return true;
  DWORD value = 1, size = sizeof(value);
  if (RegGetValueW(HKEY_CURRENT_USER, L"Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
                   L"AppsUseLightTheme", RRF_RT_REG_DWORD, nullptr, &value, &size) != ERROR_SUCCESS)
    return false;
  return value == 0;
}

// ---- geometry (logical px at 96 DPI) ---------------------------------------
constexpr int   kBaseW   = 720;
constexpr int   kBaseH   = 440;
constexpr float kCaption = 40.0f;   // top drag strip height; the caption buttons are this tall
constexpr float kPanelW  = 240.0f;  // the brand panel on the left
constexpr float kPad     = 32.0f;   // the content column's padding
constexpr float kCapBtnW = 46.0f;   // each caption button

// window messages / timers
constexpr UINT_PTR kTickTimer = 1;
constexpr UINT     kTickMs     = 16;   // ~60fps for the eased bar / fades
constexpr UINT_PTR kAnimTimer = 2;     // hover and stage fades; runs only while something moves
constexpr float    kHoverMs = 140.0f;  // DS motion "fast", standard easing
constexpr float    kStageMs = 220.0f;  // DS motion "base": a new screen's content fades in
constexpr UINT_PTR kPayloadMsgDone = (WM_APP + 1);
constexpr UINT_PTR kUninstallMsgDone = (WM_APP + 2);
constexpr UINT_PTR kStageMsg = (WM_APP + 3);  // the worker moved on: wParam is the Stage
constexpr UINT_PTR kA11yInvokeMsg = (WM_APP + 4);  // UIA calls marshal actions onto the window thread
constexpr UINT_PTR kA11yFocusMsg = (WM_APP + 5);   // UIA focus uses the same state as keyboard focus

// Results the install and uninstall workers post. 0 is success; anything else is a Win32 error code, except
// these, which have their own sentences.
constexpr DWORD kResultAppRunning = 0xE0000001;   // Redrob Launcher is open from the install folder
constexpr DWORD kResultUnpackFailed = 0xE0000002; // tar.exe could not unpack the payload
constexpr DWORD kResultNoExe = 0xE0000003;        // the unpacked payload has no redrob-launcher.exe

// Where everything goes. Per user, like every other app that installs without administrator rights. The
// same folder and registry key the MSI used through 0.3.0, so an update over an MSI install lands in
// place and the launcher's own detection (self_update.rs) keeps working.
constexpr wchar_t kInstallSubdir[] = L"\\Programs\\Redrob Launcher";  // under %LOCALAPPDATA%
constexpr wchar_t kAppExeRel[] = L"\\redrob-launcher.exe";             // the portable zip's own layout
constexpr wchar_t kPortableMarker[] = L"\\portable.txt";                // present in the zip, dropped here
constexpr wchar_t kUninstallerName[] = L"\\Uninstall Redrob Launcher.exe";
constexpr wchar_t kShortcutName[] = L"\\Redrob Launcher.lnk";
constexpr wchar_t kAppUserModelId[] = L"ai.redrob.launcher";
// Settings > Apps. Its own key.
constexpr wchar_t kArpKey[] = L"Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\RedrobLauncher";
// The launcher's own key: InstallDir (self_update.rs), Language (language.rs), DesktopShortcutChoice.
constexpr wchar_t kLauncherKey[] = L"Software\\Redrob\\Redrob Launcher";
constexpr wchar_t kAppPathsKey[] = L"Software\\Microsoft\\Windows\\CurrentVersion\\App Paths\\redrob-launcher.exe";
// The redrob:// URL scheme (docs/deep-links.md): web pages open the launcher on an app. The launcher
// registers it on every start as well (url_scheme.rs), so a portable copy gets it too.
constexpr wchar_t kUrlSchemeKey[] = L"Software\\Classes\\redrob";

enum class Stage { Preparing, Installing, Opening, Error, Confirm, Removing, Removed, Welcome, Language };
enum class Mode { Install, Uninstall };
enum class Lang { En, Ko };
enum class A11yId { Root, Brand, Stepper, Title, Body, Progress, Minimize, Close, Primary, Secondary, Checkbox };
class A11yProvider;

// The language every screen after the first draws in. Set by the language screen (or --lang, or the
// value a previous install recorded, or the Windows UI language), read by Tr() at every string.
Lang g_lang = Lang::En;
inline const wchar_t* Tr(const wchar_t* en, const wchar_t* ko) { return g_lang == Lang::Ko ? ko : en; }

// A tiny COM pointer so cleanup is not a maze of gotos. Move-only: a plain
// memberwise copy would double-Release (and a function returning one by value
// would hand back an already-freed interface).
template <class T> struct ComPtr {
  T* p = nullptr;
  ComPtr() = default;
  ComPtr(const ComPtr&) = delete;
  ComPtr& operator=(const ComPtr&) = delete;
  ComPtr(ComPtr&& o) noexcept : p(o.p) { o.p = nullptr; }
  ComPtr& operator=(ComPtr&& o) noexcept {
    if (std::addressof(o) != this) { if (p) p->Release(); p = o.p; o.p = nullptr; }
    return *this;
  }
  ~ComPtr() { if (p) p->Release(); }
  T** operator&() { return &p; }
  T* operator->() const { return p; }
  explicit operator bool() const { return p != nullptr; }
  void reset() { if (p) { p->Release(); p = nullptr; } }
};

// ---------------------------------------------------------------------------
// The app object. One window, Direct2D render target, DirectWrite text, WIC for
// the PNG resources, and a worker thread that runs the payload.
// ---------------------------------------------------------------------------
class SetupApp {
  friend class A11yProvider;
  friend void RaiseStructureChanged(SetupApp*);
  friend void RaiseToggleChanged(SetupApp*, bool, bool);
  friend void RaiseFocusChanged(SetupApp*, A11yId);
  friend void RaiseProgressChanged(SetupApp*, double, double);
 public:
  SetupApp(bool demo, bool demoError, Mode mode) : demo_(demo), demoError_(demoError), mode_(mode) {}

  int Run(HINSTANCE hinst);
  // --silent: the same work with no window, for the launcher's own self-update and scripted installs.
  // The process exit code is the worker's result (0 = success). Never opens the launcher afterwards.
  int RunSilent(HINSTANCE hinst, bool deleteData);

 private:
  static LRESULT CALLBACK WndProcThunk(HWND, UINT, WPARAM, LPARAM);
  LRESULT WndProc(UINT, WPARAM, LPARAM);

  void CreateDeviceResources();
  void DiscardDeviceResources();
  void OnPaint();
  void OnTick();
  void LayoutText();
  void ApplyHangulFont(IDWriteTextLayout* layout, const std::wstring& text);
  void DrawTextLine(const std::wstring& text, IDWriteTextFormat* format, const D2D1_RECT_F& rect, ID2D1Brush* brush);
  RECT ToDevice(float x, float y, float w, float h);

  ComPtr<ID2D1Bitmap> LoadPngResource(int resId, ID2D1RenderTarget* rt);
  HRESULT DrawScene();  // BeginDraw..EndDraw on rt_, whatever kind of target it is
 public:
  // --render-stages <dir>: draw each screen into a PNG with no window, so the layout can be checked
  // on a machine reached over ssh, where no window can be shown. Returns 0 when every file was written.
  int RenderStages(HINSTANCE hinst, const std::wstring& dir);
  bool Uninstalling() const { return mode_ == Mode::Uninstall; }
 private:

  // payload
  static DWORD WINAPI PayloadThunk(LPVOID self);
  DWORD RunPayload();
  DWORD InstallLauncher();
  bool ExtractPayload(std::wstring& outPath);
  bool InstallUninstaller(const std::wstring& dir);
  void RecordChoices(const std::wstring& dir);

  void StartInstall();

  // uninstall
  static DWORD WINAPI UninstallThunk(LPVOID self);
  DWORD RunUninstall();
  void StartUninstall();
  void ScheduleSelfCleanup();

  void SetStage(Stage s);
  void RequestClose(bool force);
  void ChooseLanguage(Lang l);

  HINSTANCE hinst_ = nullptr;
  HWND hwnd_ = nullptr;
  UINT dpi_ = 96;
  bool demo_ = false;
  bool demoError_ = false;
  Mode mode_ = Mode::Install;
  bool deleteData_ = false;       // the confirm screen's "also delete settings"
  bool silent_ = false;
  bool desktopShortcut_ = true;   // the welcome screen's "create a desktop shortcut"; a previous choice is kept

  // Direct2D / DirectWrite / WIC
  ComPtr<ID2D1Factory>        d2dFactory_;
  ComPtr<IDWriteFactory>      dwFactory_;
  ComPtr<IWICImagingFactory>  wicFactory_;
  ComPtr<ID2D1RenderTarget> rt_;
  ComPtr<ID2D1Bitmap>         symbolBitmap_;  // assets/brand/symbol-white-192.png, verbatim
  ComPtr<IDWriteTextFormat>   fmtTitle_;      // 21/600, the screen's one title
  ComPtr<IDWriteTextFormat>   fmtBody_;       // 14/22, ink-secondary, under the title
  ComPtr<IDWriteTextFormat>   fmtButton_;     // 14/600, centred in a button
  ComPtr<IDWriteTextFormat>   fmtLabel_;      // 14, a checkbox label
  ComPtr<IDWriteTextFormat>   fmtStep_;       // 12/600, the step labels and numbers
  ComPtr<IDWriteTextFormat>   fmtBrand_;      // 15/600, the product name on the brand panel
  ComPtr<IDWriteFontCollection> fontColl_;
  std::wstring                fontFamily_ = L"Segoe UI Variable Display";
  std::wstring                bodyFamily_ = L"Segoe UI Variable Text";
  std::wstring                hangulFamily_;
  bool                        bundledFontsReady_ = false;

  // drawing helpers (DrawScene)
  float TextWidth(const std::wstring& s, IDWriteTextFormat* f);
  float DrawBlock(const std::wstring& s, IDWriteTextFormat* f, float x, float y, float w, ID2D1Brush* b);
  void DrawBrandPanel(float H);
  void DrawStepper(float x, float y);
  void ApplyWindowTheme();

  // state
  Stage  stage_ = Stage::Preparing;
  double progress_ = 0.0;      // 0..1 displayed
  double progressTarget_ = 0.0;
  ULONGLONG startTick_ = 0;
  ULONGLONG openedAt_ = 0;
  ULONGLONG stageTick_ = 0;    // when the current stage began; the install ramps from here
  bool payloadDone_ = false;
  DWORD payloadExit_ = 0;
  HANDLE payloadThread_ = nullptr;
  std::wstring errorMessage_;

  // Painting, hit testing, and UIA share these exact device-pixel bounds, avoiding proxy controls.
  RECT rcMin_{}, rcClose_{}, rcRetry_{}, rcErrClose_{}, rcCheck_{};
  RECT rcBrand_{}, rcStepper_{}, rcTitle_{}, rcBody_{}, rcProgress_{};

  // Pointer and keyboard state of the controls. Retry = the primary button (English on the language screen),
  // ErrClose = the secondary one (한국어 there), Check = the checkbox row.
  enum Ctl { kNone = 0, kMin, kClose, kRetry, kErrClose, kCheck, kCtlCount };
  int hover_ = kNone;          // under the pointer
  int pressed_ = kNone;        // mouse went down here; the action runs on release over it
  int focus_ = kNone;          // virtual logical focus, synchronized with UIA while the HWND is focused
  bool focusVisible_ = false;  // input modality: keyboard shows the ring; pointer input hides it
  bool trackingMouse_ = false;
  bool reducedMotion_ = false; // SPI_GETCLIENTAREAANIMATION off: no fades
  float hoverT_[kCtlCount] = {};  // 0..1 per control, eased toward hover/pressed
  ULONGLONG animLast_ = 0;
  int HitTest(POINT pt) const;
  void Activate(int ctl);
  void UpdateHover();
  void StartAnim();
  bool AnimStep();  // advance; true while anything still moves
  std::vector<int> Focusables() const;
  bool IsFocusable(int ctl) const;
  bool WindowHasKeyboardFocus() const;
  void SetLogicalFocus(int ctl, bool showRing);
  float Eased(int ctl) const;
  float StageFade() const;
};

SetupApp* g_app = nullptr;

std::wstring StatusText(Stage s) {
  switch (s) {
    case Stage::Language:   return L"Choose your language\n언어를 선택하세요";
    case Stage::Welcome:    return Tr(L"Install Redrob Launcher", L"Redrob Launcher 설치");
    case Stage::Preparing:  return Tr(L"Preparing to install", L"설치를 준비하고 있어요");
    case Stage::Installing: return Tr(L"Installing Redrob Launcher", L"Redrob Launcher를 설치하고 있어요");
    case Stage::Opening:    return Tr(L"Opening Redrob Launcher", L"Redrob Launcher를 열고 있어요");
    case Stage::Error:      return g_app && g_app->Uninstalling()
                                   ? Tr(L"Redrob Launcher was not removed", L"Redrob Launcher를 제거하지 못했어요")
                                   : Tr(L"Setup did not finish", L"설치를 마치지 못했어요");
    case Stage::Confirm:    return Tr(L"Uninstall Redrob Launcher?", L"Redrob Launcher를 제거할까요?");
    case Stage::Removing:   return Tr(L"Removing Redrob Launcher", L"Redrob Launcher를 제거하고 있어요");
    case Stage::Removed:    return Tr(L"Redrob Launcher is removed", L"Redrob Launcher를 제거했어요");
  }
  return L"";
}

// The line under each screen's title. The error screen's is the worker's result (ResultSentence).
std::wstring BodyText(Stage s, bool deleteData, const std::wstring& error) {
  switch (s) {
    case Stage::Language:
      return L"Setup, Redrob Launcher and the Redrob apps use this language. You can change it later in Redrob Launcher's settings.\n"
             L"설치 화면과 Redrob Launcher, Redrob 앱에 이 언어가 적용돼요. 나중에 Redrob Launcher 설정에서 바꿀 수 있어요.";
    case Stage::Welcome:
      return Tr(L"It installs for this Windows account only, so no administrator rights are needed. Redrob Launcher opens when setup is done.",
                L"이 Windows 계정에만 설치해서 관리자 권한이 필요 없어요. 설치가 끝나면 Redrob Launcher가 열려요.");
    case Stage::Preparing: case Stage::Installing:
      return Tr(L"Redrob Launcher opens when it is done.", L"끝나면 Redrob Launcher가 열려요.");
    case Stage::Opening:
      return Tr(L"Redrob Launcher is installed. This window closes by itself.",
                L"Redrob Launcher를 설치했어요. 이 창은 자동으로 닫혀요.");
    case Stage::Confirm:
      return Tr(L"This removes Redrob Launcher from this PC. The apps it installed and your files stay where they are.",
                L"이 PC에서 Redrob Launcher를 제거해요. 런처로 설치한 앱과 내 파일은 그대로 남아요.");
    case Stage::Removing:
      return Tr(L"The apps it installed and your files stay where they are.", L"런처로 설치한 앱과 내 파일은 그대로 남아요.");
    case Stage::Removed:
      return deleteData ? Tr(L"Its settings and downloads are deleted too. The apps it installed and your files stay where they are.",
                             L"설정과 다운로드한 파일도 삭제했어요. 런처로 설치한 앱과 내 파일은 그대로 남아요.")
                        : Tr(L"The apps it installed and your files stay where they are. Its settings stay on this PC for a later install.",
                             L"런처로 설치한 앱과 내 파일은 그대로 남아요. 설정은 다시 설치할 때 쓸 수 있도록 이 PC에 남겨 뒀어요.");
    case Stage::Error: return error;
    default: return L"";
  }
}

// What the error screen says for a worker's result.
std::wstring ResultSentence(DWORD code, bool uninstall) {
  if (code == kResultAppRunning)
    return uninstall ? Tr(L"Redrob Launcher is open. Quit it from its tray icon, then press Retry.",
                          L"Redrob Launcher가 실행 중이에요. 트레이 아이콘에서 종료한 뒤 다시 시도를 누르세요.")
                     : Tr(L"Redrob Launcher is open. Quit it from its tray icon, then press Retry to update it.",
                          L"Redrob Launcher가 실행 중이에요. 트레이 아이콘에서 종료한 뒤 다시 시도를 눌러 업데이트하세요.");
  if (code == kResultUnpackFailed)
    return Tr(L"The app files could not be unpacked. Press Retry to try again.", L"앱 파일의 압축을 풀지 못했어요. 다시 시도를 눌러 주세요.");
  if (code == kResultNoExe)
    return Tr(L"This setup file is damaged. Please download it again.", L"이 설치 파일이 손상됐어요. 다시 다운로드해 주세요.");
  if (code == ERROR_DISK_FULL || code == ERROR_HANDLE_DISK_FULL)
    return Tr(L"There is not enough free disk space. Free some space, then press Retry.",
              L"디스크 공간이 부족해요. 공간을 비운 뒤 다시 시도를 누르세요.");
  if (code == ERROR_ACCESS_DENIED || code == ERROR_SHARING_VIOLATION)
    return Tr(L"A file is in use. Close Redrob Launcher and anything opened from its folder, then press Retry.",
              L"파일이 사용 중이에요. Redrob Launcher와 그 폴더에서 연 파일을 모두 닫은 뒤 다시 시도를 누르세요.");
  const std::wstring n = std::to_wstring(code);
  return g_lang == Lang::Ko ? L"Windows 오류 코드 " + n + L". 다시 시도를 눌러 주세요."
                            : L"Windows error code " + n + L". Press Retry to try again.";
}

// ---------------------------------------------------------------------------
// A virtual UI Automation fragment is the maintainable native contract for this custom-painted window:
// its nodes use the same bounds and actions as painting, hit testing, keyboard focus, and stage state.
class A11yProvider final : public IRawElementProviderSimple, public IRawElementProviderFragment,
                           public IRawElementProviderFragmentRoot, public IInvokeProvider,
                           public IToggleProvider, public IRangeValueProvider {
 public:
  A11yProvider(SetupApp* app, A11yId id) : app_(app), id_(id) {}
  IFACEMETHODIMP QueryInterface(REFIID iid, void** out) override {
    if (!out) return E_INVALIDARG;
    *out = nullptr;
    if (iid == __uuidof(IUnknown) || iid == __uuidof(IRawElementProviderSimple)) *out = static_cast<IRawElementProviderSimple*>(this);
    else if (iid == __uuidof(IRawElementProviderFragment)) *out = static_cast<IRawElementProviderFragment*>(this);
    else if (iid == __uuidof(IRawElementProviderFragmentRoot) && id_ == A11yId::Root) *out = static_cast<IRawElementProviderFragmentRoot*>(this);
    else if (iid == __uuidof(IInvokeProvider) && IsButton()) *out = static_cast<IInvokeProvider*>(this);
    else if (iid == __uuidof(IToggleProvider) && id_ == A11yId::Checkbox) *out = static_cast<IToggleProvider*>(this);
    else if (iid == __uuidof(IRangeValueProvider) && id_ == A11yId::Progress) *out = static_cast<IRangeValueProvider*>(this);
    if (!*out) return E_NOINTERFACE;
    AddRef();
    return S_OK;
  }
  IFACEMETHODIMP_(ULONG) AddRef() override { return static_cast<ULONG>(InterlockedIncrement(&refs_)); }
  IFACEMETHODIMP_(ULONG) Release() override {
    const LONG refs = InterlockedDecrement(&refs_);
    if (!refs) delete this;
    return static_cast<ULONG>(refs);
  }
  IFACEMETHODIMP get_ProviderOptions(ProviderOptions* options) override {
    if (!options) return E_INVALIDARG;
    *options = ProviderOptions_ServerSideProvider;
    return S_OK;
  }
  IFACEMETHODIMP GetPatternProvider(PATTERNID pattern, IUnknown** out) override {
    if (!out) return E_INVALIDARG;
    *out = nullptr;
    if (pattern == UIA_InvokePatternId && IsButton()) return QueryInterface(__uuidof(IInvokeProvider), reinterpret_cast<void**>(out));
    if (pattern == UIA_TogglePatternId && id_ == A11yId::Checkbox) return QueryInterface(__uuidof(IToggleProvider), reinterpret_cast<void**>(out));
    if (pattern == UIA_RangeValuePatternId && id_ == A11yId::Progress) return QueryInterface(__uuidof(IRangeValueProvider), reinterpret_cast<void**>(out));
    return S_OK;
  }
  IFACEMETHODIMP GetPropertyValue(PROPERTYID property, VARIANT* value) override {
    if (!value) return E_INVALIDARG;
    VariantInit(value);
    if (!Available()) return UIA_E_ELEMENTNOTAVAILABLE;
    if (property == UIA_NamePropertyId) return SetString(value, Name());
    if (property == UIA_AutomationIdPropertyId) return SetString(value, AutomationId());
    if (property == UIA_FrameworkIdPropertyId) return SetString(value, L"Win32");
    if (property == UIA_ClassNamePropertyId) return SetString(value, L"RedrobLauncherSetup");
    if (property == UIA_ControlTypePropertyId) {
      value->vt = VT_I4; value->lVal = ControlType();
    } else if (property == UIA_IsControlElementPropertyId || property == UIA_IsContentElementPropertyId) {
      value->vt = VT_BOOL; value->boolVal = VARIANT_TRUE;
    } else if (property == UIA_IsEnabledPropertyId) {
      value->vt = VT_BOOL; value->boolVal = Enabled() ? VARIANT_TRUE : VARIANT_FALSE;
    } else if (property == UIA_IsKeyboardFocusablePropertyId) {
      value->vt = VT_BOOL; value->boolVal = KeyboardFocusable() ? VARIANT_TRUE : VARIANT_FALSE;
    } else if (property == UIA_HasKeyboardFocusPropertyId) {
      value->vt = VT_BOOL; value->boolVal = HasFocus() ? VARIANT_TRUE : VARIANT_FALSE;
    } else if (property == UIA_LiveSettingPropertyId &&
               (id_ == A11yId::Title || (id_ == A11yId::Body && app_->stage_ == Stage::Error))) {
      value->vt = VT_I4; value->lVal = Assertive;
    } else if (property == UIA_ItemStatusPropertyId && (id_ == A11yId::Primary || id_ == A11yId::Secondary) &&
               app_->stage_ == Stage::Language) {
      const bool selected = (id_ == A11yId::Primary) == (g_lang == Lang::En);
      if (selected) return SetString(value, Tr(L"Selected", L"선택됨"));
    }
    return S_OK;
  }
  IFACEMETHODIMP get_HostRawElementProvider(IRawElementProviderSimple** out) override {
    if (!out) return E_INVALIDARG;
    *out = nullptr;
    return id_ == A11yId::Root ? UiaHostProviderFromHwnd(app_->hwnd_, out) : S_OK;
  }
  IFACEMETHODIMP Navigate(NavigateDirection direction, IRawElementProviderFragment** out) override {
    if (!out) return E_INVALIDARG;
    *out = nullptr;
    if (!Available()) return UIA_E_ELEMENTNOTAVAILABLE;
    const auto children = Children();
    if (id_ == A11yId::Root) {
      if (children.empty()) return S_OK;
      if (direction == NavigateDirection_FirstChild) return New(children.front(), out);
      if (direction == NavigateDirection_LastChild) return New(children.back(), out);
      return S_OK;
    }
    if (direction == NavigateDirection_Parent) return New(A11yId::Root, out);
    const auto at = std::find(children.begin(), children.end(), id_);
    if (at == children.end()) return UIA_E_ELEMENTNOTAVAILABLE;
    if (direction == NavigateDirection_NextSibling && at + 1 != children.end()) return New(*(at + 1), out);
    if (direction == NavigateDirection_PreviousSibling && at != children.begin()) return New(*(at - 1), out);
    return S_OK;
  }
  IFACEMETHODIMP GetRuntimeId(SAFEARRAY** out) override {
    if (!out) return E_INVALIDARG;
    *out = nullptr;
    if (id_ == A11yId::Root) return S_OK;
    SAFEARRAY* ids = SafeArrayCreateVector(VT_I4, 0, 2);
    if (!ids) return E_OUTOFMEMORY;
    LONG index = 0; int value = UiaAppendRuntimeId;
    SafeArrayPutElement(ids, &index, &value);
    ++index; value = static_cast<int>(id_); SafeArrayPutElement(ids, &index, &value);
    *out = ids;
    return S_OK;
  }
  IFACEMETHODIMP get_BoundingRectangle(UiaRect* bounds) override {
    if (!bounds) return E_INVALIDARG;
    *bounds = {};
    if (!Available()) return UIA_E_ELEMENTNOTAVAILABLE;
    const RECT rect = RectFor(id_);
    POINT top{rect.left, rect.top}, bottom{rect.right, rect.bottom};
    if (id_ != A11yId::Root) { ClientToScreen(app_->hwnd_, &top); ClientToScreen(app_->hwnd_, &bottom); }
    bounds->left = top.x; bounds->top = top.y; bounds->width = bottom.x - top.x; bounds->height = bottom.y - top.y;
    return S_OK;
  }
  IFACEMETHODIMP GetEmbeddedFragmentRoots(SAFEARRAY** out) override { if (!out) return E_INVALIDARG; *out = nullptr; return S_OK; }
  IFACEMETHODIMP SetFocus() override {
    if (!Available() || !KeyboardFocusable()) return UIA_E_NOTSUPPORTED;
    PostMessageW(app_->hwnd_, kA11yFocusMsg, static_cast<WPARAM>(Control()), 0);
    return S_OK;
  }
  IFACEMETHODIMP get_FragmentRoot(IRawElementProviderFragmentRoot** out) override {
    if (!out) return E_INVALIDARG;
    *out = new (std::nothrow) A11yProvider(app_, A11yId::Root);
    return *out ? S_OK : E_OUTOFMEMORY;
  }
  IFACEMETHODIMP ElementProviderFromPoint(double x, double y, IRawElementProviderFragment** out) override {
    if (!out) return E_INVALIDARG;
    POINT point{static_cast<LONG>(x), static_cast<LONG>(y)};
    ScreenToClient(app_->hwnd_, &point);
    const auto children = Children();
    for (auto it = children.rbegin(); it != children.rend(); ++it) { RECT rect = RectFor(*it); if (PtInRect(&rect, point)) return New(*it, out); }
    return New(A11yId::Root, out);
  }
  IFACEMETHODIMP GetFocus(IRawElementProviderFragment** out) override {
    if (!out) return E_INVALIDARG;
    *out = nullptr;
    if (!Available()) return UIA_E_ELEMENTNOTAVAILABLE;
    // Virtual focus is meaningful only while this top-level HWND owns the thread's real keyboard focus.
    if (!app_->WindowHasKeyboardFocus()) return S_OK;
    return New(IdForControl(app_->focus_), out);
  }
  IFACEMETHODIMP Invoke() override {
    if (!Available()) return UIA_E_ELEMENTNOTAVAILABLE;
    if (!IsButton()) return UIA_E_NOTSUPPORTED;
    if (!Enabled()) return UIA_E_ELEMENTNOTENABLED;
    PostMessageW(app_->hwnd_, kA11yInvokeMsg, static_cast<WPARAM>(Control()), 0);
    return S_OK;
  }
  IFACEMETHODIMP Toggle() override {
    if (!Available() || id_ != A11yId::Checkbox) return UIA_E_ELEMENTNOTAVAILABLE;
    PostMessageW(app_->hwnd_, kA11yInvokeMsg, static_cast<WPARAM>(SetupApp::kCheck), 0);
    return S_OK;
  }
  IFACEMETHODIMP get_ToggleState(ToggleState* state) override {
    if (!state) return E_INVALIDARG;
    if (!Available() || id_ != A11yId::Checkbox) return UIA_E_ELEMENTNOTAVAILABLE;
    *state = (app_->stage_ == Stage::Welcome ? app_->desktopShortcut_ : app_->deleteData_) ? ToggleState_On : ToggleState_Off;
    return S_OK;
  }
  IFACEMETHODIMP SetValue(double) override { return UIA_E_NOTSUPPORTED; }
  IFACEMETHODIMP get_Value(double* value) override { if (!value) return E_INVALIDARG; *value = app_->progress_ * 100.0; return Available() ? S_OK : UIA_E_ELEMENTNOTAVAILABLE; }
  IFACEMETHODIMP get_IsReadOnly(BOOL* value) override { if (!value) return E_INVALIDARG; *value = TRUE; return S_OK; }
  IFACEMETHODIMP get_Maximum(double* value) override { if (!value) return E_INVALIDARG; *value = 100.0; return S_OK; }
  IFACEMETHODIMP get_Minimum(double* value) override { if (!value) return E_INVALIDARG; *value = 0.0; return S_OK; }
  IFACEMETHODIMP get_LargeChange(double* value) override { if (!value) return E_INVALIDARG; *value = 0.0; return S_OK; }
  IFACEMETHODIMP get_SmallChange(double* value) override { if (!value) return E_INVALIDARG; *value = 0.0; return S_OK; }

  static A11yId IdForControl(int control) {
    if (control == SetupApp::kMin) return A11yId::Minimize;
    if (control == SetupApp::kClose) return A11yId::Close;
    if (control == SetupApp::kRetry) return A11yId::Primary;
    if (control == SetupApp::kErrClose) return A11yId::Secondary;
    if (control == SetupApp::kCheck) return A11yId::Checkbox;
    return A11yId::Root;
  }
 private:
  bool Available() const {
    if (!app_ || !app_->hwnd_ || !IsWindow(app_->hwnd_)) return false;
    if (id_ == A11yId::Root) return true;
    const auto list = Children();
    return std::find(list.begin(), list.end(), id_) != list.end();
  }
  bool IsButton() const { return id_ == A11yId::Minimize || id_ == A11yId::Close || id_ == A11yId::Primary || id_ == A11yId::Secondary; }
  bool IsInteractive() const { return IsButton() || id_ == A11yId::Checkbox; }
  bool Enabled() const { return !(id_ == A11yId::Close && app_->stage_ == Stage::Removing); }
  bool KeyboardFocusable() const {
    if (!IsInteractive()) return false;
    const auto order = app_->Focusables();
    return std::find(order.begin(), order.end(), Control()) != order.end();
  }
  int Control() const {
    if (id_ == A11yId::Minimize) return SetupApp::kMin; if (id_ == A11yId::Close) return SetupApp::kClose;
    if (id_ == A11yId::Primary) return SetupApp::kRetry; if (id_ == A11yId::Secondary) return SetupApp::kErrClose;
    return id_ == A11yId::Checkbox ? SetupApp::kCheck : SetupApp::kNone;
  }
  bool HasFocus() const {
    return app_->WindowHasKeyboardFocus() && id_ == IdForControl(app_->focus_);
  }
  CONTROLTYPEID ControlType() const {
    if (id_ == A11yId::Root) return UIA_WindowControlTypeId; if (IsButton()) return UIA_ButtonControlTypeId;
    if (id_ == A11yId::Checkbox) return UIA_CheckBoxControlTypeId; if (id_ == A11yId::Progress) return UIA_ProgressBarControlTypeId;
    return UIA_TextControlTypeId;
  }
  std::wstring Name() const {
    if (id_ == A11yId::Root) { wchar_t title[128]{}; GetWindowTextW(app_->hwnd_, title, ARRAYSIZE(title)); return title; }
    if (id_ == A11yId::Brand) return L"Redrob Launcher";
    if (id_ == A11yId::Title) return StatusText(app_->stage_);
    if (id_ == A11yId::Body) return BodyText(app_->stage_, app_->deleteData_, app_->errorMessage_);
    if (id_ == A11yId::Minimize) return Tr(L"Minimize", L"최소화"); if (id_ == A11yId::Close) return Tr(L"Close", L"닫기");
    if (id_ == A11yId::Progress) return app_->mode_ == Mode::Uninstall ? Tr(L"Removal progress", L"제거 진행률") : Tr(L"Installation progress", L"설치 진행률");
    if (id_ == A11yId::Checkbox) return app_->stage_ == Stage::Welcome ? Tr(L"Create a desktop shortcut", L"바탕 화면에 바로 가기 만들기") : Tr(L"Also delete its settings and downloads", L"설정과 다운로드한 파일도 삭제");
    if (id_ == A11yId::Stepper) {
      int step = 1; const wchar_t* label = app_->mode_ == Mode::Uninstall ? Tr(L"Confirm", L"확인") : Tr(L"Language", L"언어");
      if (app_->stage_ == Stage::Removing || app_->stage_ == Stage::Error) { step = 2; label = app_->mode_ == Mode::Uninstall ? Tr(L"Remove", L"제거") : Tr(L"Install", L"설치"); }
      else if (app_->stage_ == Stage::Removed || app_->stage_ == Stage::Opening) { step = 3; label = app_->mode_ == Mode::Uninstall ? Tr(L"Done", L"완료") : Tr(L"Open", L"열기"); }
      else if (app_->mode_ == Mode::Install && app_->stage_ != Stage::Language) { step = 2; label = Tr(L"Install", L"설치"); }
      return g_lang == Lang::Ko ? L"3단계 중 " + std::to_wstring(step) + L"단계: " + label : L"Step " + std::to_wstring(step) + L" of 3: " + label;
    }
    if (id_ == A11yId::Primary) {
      if (app_->stage_ == Stage::Language) return L"English"; if (app_->stage_ == Stage::Welcome) return Tr(L"Install", L"설치");
      if (app_->stage_ == Stage::Confirm) return Tr(L"Uninstall", L"제거"); if (app_->stage_ == Stage::Removed) return Tr(L"Close", L"닫기");
      return Tr(L"Retry", L"다시 시도");
    }
    if (app_->stage_ == Stage::Language) return L"한국어"; if (app_->stage_ == Stage::Welcome) return Tr(L"Back", L"뒤로");
    if (app_->stage_ == Stage::Confirm) return Tr(L"Cancel", L"취소"); return Tr(L"Close", L"닫기");
  }
  std::wstring AutomationId() const {
    static const wchar_t* ids[] = {L"window",L"brand",L"stepper",L"status",L"description",L"progress",L"minimize",L"close",L"primary",L"secondary",L"option"};
    return ids[static_cast<size_t>(id_)];
  }
  RECT RectFor(A11yId id) const {
    if (id == A11yId::Root) { RECT r{}; GetWindowRect(app_->hwnd_, &r); return r; }
    if (id == A11yId::Brand) return app_->rcBrand_; if (id == A11yId::Stepper) return app_->rcStepper_;
    if (id == A11yId::Title) return app_->rcTitle_; if (id == A11yId::Body) return app_->rcBody_; if (id == A11yId::Progress) return app_->rcProgress_;
    if (id == A11yId::Minimize) return app_->rcMin_; if (id == A11yId::Close) return app_->rcClose_; if (id == A11yId::Primary) return app_->rcRetry_;
    return id == A11yId::Secondary ? app_->rcErrClose_ : app_->rcCheck_;
  }
  std::vector<A11yId> Children() const {
    std::vector<A11yId> result{A11yId::Brand,A11yId::Stepper,A11yId::Title,A11yId::Body};
    if (app_->stage_ == Stage::Language) { result.push_back(A11yId::Primary); result.push_back(A11yId::Secondary); }
    else if (app_->stage_ == Stage::Welcome || app_->stage_ == Stage::Confirm) { result.push_back(A11yId::Checkbox); result.push_back(A11yId::Secondary); result.push_back(A11yId::Primary); }
    else if (app_->stage_ == Stage::Error) { result.push_back(A11yId::Secondary); result.push_back(A11yId::Primary); }
    else if (app_->stage_ == Stage::Removed) result.push_back(A11yId::Primary); else result.push_back(A11yId::Progress);
    result.push_back(A11yId::Minimize); result.push_back(A11yId::Close); return result;
  }
  HRESULT New(A11yId id, IRawElementProviderFragment** out) const {
    if (!out) return E_INVALIDARG; *out = new (std::nothrow) A11yProvider(app_, id); return *out ? S_OK : E_OUTOFMEMORY;
  }
  static HRESULT SetString(VARIANT* value, const std::wstring& text) {
    value->vt = VT_BSTR; value->bstrVal = SysAllocStringLen(text.data(), static_cast<UINT>(text.size())); return value->bstrVal || text.empty() ? S_OK : E_OUTOFMEMORY;
  }
  LONG refs_ = 1; SetupApp* app_; A11yId id_;
};

void RaiseStructureChanged(SetupApp* app) {
  if (!app || !app->hwnd_ || !IsWindowVisible(app->hwnd_)) return;
  auto* root = new (std::nothrow) A11yProvider(app, A11yId::Root);
  if (root) { UiaRaiseStructureChangedEvent(root, StructureChangeType_ChildrenInvalidated, nullptr, 0); root->Release(); }

  // Error details are the actionable status. Announce them only after SetStage's synchronous paint,
  // when the worker-written text and the Body provider's Name expose the same current value.
  const A11yId liveId = app->stage_ == Stage::Error ? A11yId::Body : A11yId::Title;
  auto* live = new (std::nothrow) A11yProvider(app, liveId);
  if (!live) return;
  UiaRaiseAutomationEvent(live, UIA_LiveRegionChangedEventId);
  if (app->stage_ == Stage::Error && !app->errorMessage_.empty()) {
    BSTR display = SysAllocString(app->errorMessage_.c_str());
    BSTR activity = SysAllocString(L"redrob-launcher-setup-error");
    if (display && activity)
      UiaRaiseNotificationEvent(live, NotificationKind_ActionAborted,
                                NotificationProcessing_ImportantMostRecent, display, activity);
    SysFreeString(activity);
    SysFreeString(display);
  }
  live->Release();
}
void RaiseToggleChanged(SetupApp* app, bool before, bool after) {
  if (!app || before == after || !IsWindowVisible(app->hwnd_)) return;
  auto* option = new (std::nothrow) A11yProvider(app, A11yId::Checkbox); if (!option) return;
  VARIANT oldValue{}, newValue{}; oldValue.vt = newValue.vt = VT_I4; oldValue.lVal = before ? ToggleState_On : ToggleState_Off; newValue.lVal = after ? ToggleState_On : ToggleState_Off;
  UiaRaiseAutomationPropertyChangedEvent(option, UIA_ToggleToggleStatePropertyId, oldValue, newValue); option->Release();
}
void RaiseFocusChanged(SetupApp* app, A11yId id) {
  if (!app || !IsWindowVisible(app->hwnd_)) return; auto* node = new (std::nothrow) A11yProvider(app, id);
  if (node) { UiaRaiseAutomationEvent(node, UIA_AutomationFocusChangedEventId); node->Release(); }
}
void RaiseProgressChanged(SetupApp* app, double before, double after) {
  const int oldPercent = static_cast<int>(before * 100.0), newPercent = static_cast<int>(after * 100.0);
  if (!app || oldPercent == newPercent || !IsWindowVisible(app->hwnd_)) return;
  auto* node = new (std::nothrow) A11yProvider(app, A11yId::Progress); if (!node) return;
  VARIANT oldValue{}, newValue{}; oldValue.vt = newValue.vt = VT_R8; oldValue.dblVal = oldPercent; newValue.dblVal = newPercent;
  UiaRaiseAutomationPropertyChangedEvent(node, UIA_RangeValueValuePropertyId, oldValue, newValue); node->Release();
}

// The language a previous install recorded (en, ko), or nothing.
static std::wstring RecordedLanguage() {
  HKEY key = nullptr;
  if (RegOpenKeyExW(HKEY_CURRENT_USER, kLauncherKey, 0, KEY_READ, &key) != ERROR_SUCCESS) return L"";
  wchar_t buf[16] = {};
  DWORD size = sizeof(buf), type = 0;
  std::wstring out;
  if (RegQueryValueExW(key, L"Language", nullptr, &type, reinterpret_cast<BYTE*>(buf), &size) == ERROR_SUCCESS && type == REG_SZ)
    out = buf;
  RegCloseKey(key);
  return out;
}

// The desktop-shortcut choice a previous install recorded: 1, 0, or -1 for none.
static int RecordedDesktopShortcut() {
  HKEY key = nullptr;
  if (RegOpenKeyExW(HKEY_CURRENT_USER, kLauncherKey, 0, KEY_READ, &key) != ERROR_SUCCESS) return -1;
  wchar_t buf[8] = {};
  DWORD size = sizeof(buf), type = 0;
  int out = -1;
  if (RegQueryValueExW(key, L"DesktopShortcutChoice", nullptr, &type, reinterpret_cast<BYTE*>(buf), &size) == ERROR_SUCCESS && type == REG_SZ)
    out = (buf[0] == L'1') ? 1 : 0;
  RegCloseKey(key);
  return out;
}

// The language to preselect: --lang, else the recorded one, else Korean on a Korean Windows, else English.
static Lang DefaultLanguage(const std::wstring& flag) {
  if (lstrcmpiW(flag.c_str(), L"ko") == 0) return Lang::Ko;
  if (lstrcmpiW(flag.c_str(), L"en") == 0) return Lang::En;
  const std::wstring recorded = RecordedLanguage();
  if (lstrcmpiW(recorded.c_str(), L"ko") == 0) return Lang::Ko;
  if (lstrcmpiW(recorded.c_str(), L"en") == 0) return Lang::En;
  return PRIMARYLANGID(GetUserDefaultUILanguage()) == LANG_KOREAN ? Lang::Ko : Lang::En;
}

int SetupApp::Run(HINSTANCE hinst) {
  hinst_ = hinst;
  g_app = this;

  SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
  HRESULT hr = CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);
  (void)hr;

  D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, &d2dFactory_);
  DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, __uuidof(IDWriteFactory),
                      reinterpret_cast<IUnknown**>(&dwFactory_));
  CoCreateInstance(CLSID_WICImagingFactory, nullptr, CLSCTX_INPROC_SERVER,
                   IID_PPV_ARGS(&wicFactory_));

  WNDCLASSEX wc = {sizeof(wc)};
  wc.style = CS_HREDRAW | CS_VREDRAW;
  wc.lpfnWndProc = &SetupApp::WndProcThunk;
  wc.hInstance = hinst;
  wc.hCursor = LoadCursor(nullptr, IDC_ARROW);
  wc.lpszClassName = L"RedrobLauncherSetupWindow";
  wc.hIcon = LoadIconW(hinst, MAKEINTRESOURCEW(IDI_APPICON));
  wc.hIconSm = wc.hIcon;
  RegisterClassEx(&wc);

  // Center on the primary monitor's work area.
  dpi_ = GetDpiForSystem();
  int w = MulDiv(kBaseW, dpi_, 96);
  int h = MulDiv(kBaseH, dpi_, 96);
  RECT wa{};
  SystemParametersInfo(SPI_GETWORKAREA, 0, &wa, 0);
  int x = wa.left + ((wa.right - wa.left) - w) / 2;
  int y = wa.top + ((wa.bottom - wa.top) - h) / 2;

  // Borderless (WS_POPUP). WS_THICKFRAME is kept out; a DWM shadow is attached
  // separately so a popup still casts one.
  hwnd_ = CreateWindowExW(WS_EX_APPWINDOW, wc.lpszClassName,
                          mode_ == Mode::Uninstall ? Tr(L"Uninstall Redrob Launcher", L"Redrob Launcher 제거")
                                                   : Tr(L"Redrob Launcher Setup", L"Redrob Launcher 설치"),
                          WS_POPUP, x, y, w, h, nullptr, nullptr, hinst, this);
  if (!hwnd_) return 2;

  // DWM: a real shadow on a borderless window + rounded corners (Win11).
  MARGINS m = {1, 1, 1, 1};
  DwmExtendFrameIntoClientArea(hwnd_, &m);
  DWM_WINDOW_CORNER_PREFERENCE corner = DWMWCP_ROUND;
  DwmSetWindowAttribute(hwnd_, DWMWA_WINDOW_CORNER_PREFERENCE, &corner, sizeof(corner));
  g_dark = SystemPrefersDark();
  ApplyWindowTheme();
  BOOL animate = TRUE;
  if (SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION, 0, &animate, 0)) reducedMotion_ = !animate;

  LayoutText();
#ifdef REDROB_REQUIRE_BUNDLED_FONTS
  if (!bundledFontsReady_) {
    // Production never silently substitutes fonts because that changes Korean shaping and metrics.
    MessageBoxW(hwnd_, Tr(L"Setup could not load its required fonts. Please download the setup again.",
                          L"설치에 필요한 글꼴을 불러오지 못했어요. 설치 파일을 다시 다운로드해 주세요."),
                Tr(L"Redrob Launcher Setup", L"Redrob Launcher 설치"), MB_OK | MB_ICONERROR);
    DestroyWindow(hwnd_);
    fontColl_.reset(); wicFactory_.reset(); dwFactory_.reset(); d2dFactory_.reset(); CoUninitialize();
    return ERROR_RESOURCE_DATA_NOT_FOUND;
  }
#endif

  startTick_ = GetTickCount64();
  if (mode_ == Mode::Uninstall) {
    SetStage(Stage::Confirm);
  } else if (demoError_) {
    errorMessage_ = ResultSentence(kResultUnpackFailed, false);
    SetStage(Stage::Error);
  } else {
    // Nothing is installed until the person presses Install. The language comes first: every screen
    // after it, the launcher and then every Redrob app follow the choice.
    const int previous = RecordedDesktopShortcut();
    if (previous >= 0) desktopShortcut_ = previous == 1;
    SetStage(Stage::Language);
  }

  ShowWindow(hwnd_, SW_SHOW);
  UpdateWindow(hwnd_);
  // Raise after first paint so provider bounds are ready when a client handles WindowOpened.
  auto* openedProvider = new (std::nothrow) A11yProvider(this, A11yId::Root);
  if (openedProvider) { UiaRaiseAutomationEvent(openedProvider, UIA_Window_WindowOpenedEventId); openedProvider->Release(); }

  MSG msg;
  while (GetMessage(&msg, nullptr, 0, 0) > 0) {
    TranslateMessage(&msg);
    DispatchMessage(&msg);
  }
  if (payloadThread_) {
    WaitForSingleObject(payloadThread_, 5000);
    CloseHandle(payloadThread_);
  }
  // Every COM object goes BEFORE CoUninitialize. Left to the destructor, the WIC factory was released after
  // COM was torn down, and the process died with an access violation on its way out of a successful install.
  DiscardDeviceResources();
  fmtTitle_.reset();
  fmtBody_.reset();
  fmtButton_.reset();
  fmtLabel_.reset();
  fmtStep_.reset();
  fmtBrand_.reset();
  fontColl_.reset();
  wicFactory_.reset();
  dwFactory_.reset();
  d2dFactory_.reset();
  CoUninitialize();
  return (int)msg.wParam;
}

// The DWM frame follows the theme too (immersive dark mode only for the dark theme).
void SetupApp::ApplyWindowTheme() {
  if (!hwnd_) return;
  BOOL dark = g_dark ? TRUE : FALSE;
  DwmSetWindowAttribute(hwnd_, DWMWA_USE_IMMERSIVE_DARK_MODE, &dark, sizeof(dark));
}

void SetupApp::LayoutText() {
  // One private collection keeps deterministic Pretendard and Wanted Sans resources together.
  ComPtr<IDWriteFactory5> f5;
  if (SUCCEEDED(dwFactory_->QueryInterface(__uuidof(IDWriteFactory5), reinterpret_cast<void**>(&f5)))) {
    ComPtr<IDWriteInMemoryFontFileLoader> loader;
    ComPtr<IDWriteFontSetBuilder1> builder;
    if (SUCCEEDED(f5->CreateInMemoryFontFileLoader(&loader)) && SUCCEEDED(f5->RegisterFontFileLoader(loader.p)) &&
        SUCCEEDED(f5->CreateFontSetBuilder(&builder))) {
      int added = 0;
      for (int id : {IDR_FONT_SEMIBOLD, IDR_FONT_REGULAR, IDR_FONT_HANGUL}) {
        HRSRC resource = FindResourceW(hinst_, MAKEINTRESOURCEW(id), RT_RCDATA);
        if (!resource) continue;
        HGLOBAL loaded = LoadResource(hinst_, resource);
        const void* data = loaded ? LockResource(loaded) : nullptr;
        const DWORD size = SizeofResource(hinst_, resource);
        ComPtr<IDWriteFontFile> file;
        if (data && size && SUCCEEDED(loader->CreateInMemoryFontFileReference(f5.p, data, size, nullptr, &file)) &&
            SUCCEEDED(builder->AddFontFile(file.p))) ++added;
      }
      ComPtr<IDWriteFontSet> set;
      ComPtr<IDWriteFontCollection1> collection;
      if (added == 3 && SUCCEEDED(builder->CreateFontSet(&set)) &&
          SUCCEEDED(f5->CreateFontCollectionFromFontSet(set.p, &collection))) {
        fontColl_.p = collection.p;
        collection.p = nullptr;
      }
    }
  }
  if (fontColl_) {
    // Family identity is independent of file order; both Pretendard weights normally share one family.
    for (UINT32 i = 0; i < fontColl_->GetFontFamilyCount(); ++i) {
      ComPtr<IDWriteFontFamily> family; ComPtr<IDWriteLocalizedStrings> names;
      if (FAILED(fontColl_->GetFontFamily(i, &family)) || FAILED(family->GetFamilyNames(&names)) || !names->GetCount()) continue;
      UINT32 index = 0; BOOL found = FALSE;
      if (FAILED(names->FindLocaleName(L"en-us", &index, &found)) || !found) index = 0;
      UINT32 length = 0; if (FAILED(names->GetStringLength(index, &length))) continue;
      std::wstring name(length + 1, L'\0'); if (FAILED(names->GetString(index, &name[0], length + 1))) continue; name.resize(length);
      if (name.find(L"Pretendard") != std::wstring::npos) fontFamily_ = bodyFamily_ = name;
      else if (name.find(L"Wanted") != std::wstring::npos) hangulFamily_ = name;
    }
    bundledFontsReady_ = fontFamily_.find(L"Pretendard") != std::wstring::npos && !hangulFamily_.empty();
    if (!bundledFontsReady_) fontColl_.reset();
  }
  auto make = [&](ComPtr<IDWriteTextFormat>& out, const std::wstring& family, DWRITE_FONT_WEIGHT weight, float size,
                  float lineHeight, DWRITE_TEXT_ALIGNMENT align, DWRITE_PARAGRAPH_ALIGNMENT para) {
    out.reset();
    if (FAILED(dwFactory_->CreateTextFormat(family.c_str(), fontColl_.p, weight, DWRITE_FONT_STYLE_NORMAL,
        DWRITE_FONT_STRETCH_NORMAL, size, L"en-us", &out)) || !out) return;
    out->SetTextAlignment(align); out->SetParagraphAlignment(para); out->SetWordWrapping(DWRITE_WORD_WRAPPING_WRAP);
    if (lineHeight > 0) out->SetLineSpacing(DWRITE_LINE_SPACING_METHOD_UNIFORM, lineHeight, lineHeight * 0.78f);
  };
  const auto leading = DWRITE_TEXT_ALIGNMENT_LEADING, center = DWRITE_TEXT_ALIGNMENT_CENTER;
  const auto near = DWRITE_PARAGRAPH_ALIGNMENT_NEAR, middle = DWRITE_PARAGRAPH_ALIGNMENT_CENTER;
  make(fmtTitle_, fontFamily_, DWRITE_FONT_WEIGHT_SEMI_BOLD, 21.0f, 28.0f, leading, near);
  make(fmtBody_, bodyFamily_, DWRITE_FONT_WEIGHT_REGULAR, 14.0f, 22.0f, leading, near);
  make(fmtButton_, fontFamily_, DWRITE_FONT_WEIGHT_SEMI_BOLD, 14.0f, 0.0f, center, middle);
  make(fmtLabel_, bodyFamily_, DWRITE_FONT_WEIGHT_REGULAR, 14.0f, 20.0f, leading, middle);
  make(fmtStep_, fontFamily_, DWRITE_FONT_WEIGHT_SEMI_BOLD, 12.0f, 0.0f, leading, middle);
  make(fmtBrand_, fontFamily_, DWRITE_FONT_WEIGHT_SEMI_BOLD, 15.0f, 20.0f, leading, near);
  // Production readiness includes format creation, not merely finding three resource blobs.
  bundledFontsReady_ = bundledFontsReady_ && fmtTitle_ && fmtBody_ && fmtButton_ && fmtLabel_ && fmtStep_ && fmtBrand_;
}

void SetupApp::ApplyHangulFont(IDWriteTextLayout* layout, const std::wstring& text) {
  if (!layout || hangulFamily_.empty()) return;
  auto isHangul = [](wchar_t c) { return (c >= 0x1100 && c <= 0x11FF) || (c >= 0x3130 && c <= 0x318F) || (c >= 0xAC00 && c <= 0xD7A3); };
  for (UINT32 start = 0; start < text.size();) {
    if (!isHangul(text[start])) { ++start; continue; }
    UINT32 end = start + 1; while (end < text.size() && isHangul(text[end])) ++end;
    // Range-level assignment preserves Pretendard for Latin product names embedded in Korean copy.
    layout->SetFontFamilyName(hangulFamily_.c_str(), DWRITE_TEXT_RANGE{start, end - start});
    layout->SetFontWeight(DWRITE_FONT_WEIGHT_REGULAR, DWRITE_TEXT_RANGE{start, end - start});
    start = end;
  }
}

void SetupApp::DrawTextLine(const std::wstring& text, IDWriteTextFormat* format,
                            const D2D1_RECT_F& rect, ID2D1Brush* brush) {
  if (!format || text.empty()) return;
  ComPtr<IDWriteTextLayout> layout;
  if (FAILED(dwFactory_->CreateTextLayout(text.c_str(), static_cast<UINT32>(text.size()), format,
      rect.right - rect.left, rect.bottom - rect.top, &layout))) return;
  ApplyHangulFont(layout.p, text);
  rt_->DrawTextLayout(D2D1::Point2F(rect.left, rect.top), layout.p, brush);
}

LRESULT CALLBACK SetupApp::WndProcThunk(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
  if (msg == WM_NCCREATE) {
    auto* cs = reinterpret_cast<CREATESTRUCT*>(lp);
    SetWindowLongPtr(hwnd, GWLP_USERDATA, (LONG_PTR)cs->lpCreateParams);
  }
  auto* self = reinterpret_cast<SetupApp*>(GetWindowLongPtr(hwnd, GWLP_USERDATA));
  if (self) { self->hwnd_ = hwnd; return self->WndProc(msg, wp, lp); }
  return DefWindowProc(hwnd, msg, wp, lp);
}

LRESULT SetupApp::WndProc(UINT msg, WPARAM wp, LPARAM lp) {
  switch (msg) {
    case WM_PAINT: OnPaint(); return 0;
    case WM_ERASEBKGND: return 1;
    case WM_GETOBJECT:
      if (static_cast<LONG>(lp) == UiaRootObjectId) {
        auto* provider = new (std::nothrow) A11yProvider(this, A11yId::Root);
        if (!provider) return 0;
        const LRESULT result = UiaReturnRawElementProvider(hwnd_, wp, lp, provider);
        provider->Release();
        return result;
      }
      return DefWindowProc(hwnd_, msg, wp, lp);
    case kA11yInvokeMsg: Activate(static_cast<int>(wp)); return 0;
    case kA11yFocusMsg: {
      const int target = static_cast<int>(wp);
      if (!IsFocusable(target)) return 0;  // the provider can become stale while this message is queued
      // Set the virtual target first: a synchronous WM_SETFOCUS then announces the right fragment.
      SetLogicalFocus(target, true);
      SetFocus(hwnd_);
      return 0;
    }
    case WM_SETFOCUS:
      InvalidateRect(hwnd_, nullptr, FALSE);
      RaiseFocusChanged(this, A11yProvider::IdForControl(focus_));
      return 0;
    case WM_KILLFOCUS:
      // Retain logical focus for activation restoration, but painting/properties are gated by real HWND focus.
      InvalidateRect(hwnd_, nullptr, FALSE);
      return 0;

    case WM_TIMER:
      if (wp == kTickTimer) OnTick();
      if (wp == kAnimTimer) {
        if (!AnimStep()) KillTimer(hwnd_, kAnimTimer);
        InvalidateRect(hwnd_, nullptr, FALSE);
      }
      return 0;

    case kStageMsg:
      if (stage_ != Stage::Error) SetStage((Stage)wp);
      return 0;

    case kPayloadMsgDone: {
      payloadDone_ = true;
      payloadExit_ = (DWORD)wp;
      if (payloadExit_ == 0) {
        progressTarget_ = 1.0;
        SetStage(Stage::Opening);
      } else {
        errorMessage_ = ResultSentence(payloadExit_, false);
        SetStage(Stage::Error);
      }
      return 0;
    }

    case kUninstallMsgDone: {
      const DWORD code = (DWORD)wp;
      if (code == 0) {
        progressTarget_ = 1.0;
        progress_ = 1.0;
        SetStage(Stage::Removed);
        if (!demo_) ScheduleSelfCleanup();
      } else {
        errorMessage_ = ResultSentence(code, true);
        SetStage(Stage::Error);
      }
      return 0;
    }

    case WM_GETDPISCALEDSIZE: return TRUE;
    case WM_DPICHANGED: {
      dpi_ = HIWORD(wp);
      RECT* r = reinterpret_cast<RECT*>(lp);
      SetWindowPos(hwnd_, nullptr, r->left, r->top, r->right - r->left, r->bottom - r->top,
                   SWP_NOZORDER | SWP_NOACTIVATE);
      DiscardDeviceResources();
      InvalidateRect(hwnd_, nullptr, FALSE);
      return 0;
    }

    case WM_SIZE: DiscardDeviceResources(); return 0;

    case WM_SETTINGCHANGE:
      // Windows switched between light and dark: follow it.
      if (lp && lstrcmpiW(reinterpret_cast<LPCWSTR>(lp), L"ImmersiveColorSet") == 0) {
        g_dark = SystemPrefersDark();
        ApplyWindowTheme();
        InvalidateRect(hwnd_, nullptr, FALSE);
      }
      return 0;

    case WM_MOUSEMOVE:
      if (!trackingMouse_) {
        TRACKMOUSEEVENT tme = {sizeof(tme), TME_LEAVE, hwnd_, 0};
        trackingMouse_ = TrackMouseEvent(&tme) != FALSE;
      }
      UpdateHover();
      return 0;

    case WM_MOUSELEAVE:
      trackingMouse_ = false;
      if (hover_ != kNone) { hover_ = kNone; StartAnim(); }
      return 0;

    case WM_SETCURSOR:
      if (LOWORD(lp) == HTCLIENT) {
        POINT pt; GetCursorPos(&pt); ScreenToClient(hwnd_, &pt);
        SetCursor(LoadCursor(nullptr, HitTest(pt) != kNone ? IDC_HAND : IDC_ARROW));
        return TRUE;
      }
      return DefWindowProc(hwnd_, msg, wp, lp);

    case WM_LBUTTONDOWN: {
      POINT pt{GET_X_LPARAM(lp), GET_Y_LPARAM(lp)};
      focusVisible_ = false;  // input modality controls the ring independently of logical focus
      const int hit = HitTest(pt);
      if (hit != kNone) {
        if (IsFocusable(hit)) {
          // Publish the pointer's virtual target before SetFocus can synchronously send WM_SETFOCUS.
          SetLogicalFocus(hit, false);
          SetFocus(hwnd_);
        }
        pressed_ = hit;
        SetCapture(hwnd_);
        StartAnim();
        return 0;
      }
      // Anywhere else on the caption strip or the brand panel drags the window.
      if (pt.y < MulDiv((int)kCaption, dpi_, 96) || pt.x < MulDiv((int)kPanelW, dpi_, 96)) {
        ReleaseCapture();
        SendMessage(hwnd_, WM_NCLBUTTONDOWN, HTCAPTION, 0);
      }
      InvalidateRect(hwnd_, nullptr, FALSE);
      return 0;
    }

    case WM_LBUTTONUP: {
      POINT pt{GET_X_LPARAM(lp), GET_Y_LPARAM(lp)};
      const int was = pressed_;
      pressed_ = kNone;
      if (GetCapture() == hwnd_) ReleaseCapture();
      StartAnim();
      // The action runs on release over the control it went down on, like a native button.
      if (was != kNone && HitTest(pt) == was) Activate(was);
      return 0;
    }

    case WM_KEYDOWN: {
      if (wp == VK_ESCAPE) { RequestClose(false); return 0; }
      if (wp == VK_TAB) {
        const std::vector<int> order = Focusables();
        if (order.empty()) return 0;
        const bool back = (GetKeyState(VK_SHIFT) & 0x8000) != 0;
        auto at = std::find(order.begin(), order.end(), focus_);
        int i = at == order.end() ? (back ? (int)order.size() - 1 : 0)
                                  : ((int)(at - order.begin()) + (back ? -1 : 1) + (int)order.size()) % (int)order.size();
        SetLogicalFocus(order[i], true);
        return 0;
      }
      // Logical focus drives activation; keyboard modality only decides whether its ring is painted.
      if (focus_ != kNone && (wp == VK_RETURN || wp == VK_SPACE)) {
        const std::vector<int> order = Focusables();
        if (std::find(order.begin(), order.end(), focus_) != order.end()) {
          focusVisible_ = true;
          InvalidateRect(hwnd_, nullptr, FALSE);
          Activate(focus_);
          return 0;
        }
      }
      if (stage_ == Stage::Language) {
        // Left/Right or E/K pick; Enter keeps the preselected language.
        if (wp == VK_LEFT || wp == 'E') { ChooseLanguage(Lang::En); return 0; }
        if (wp == VK_RIGHT || wp == 'K') { ChooseLanguage(Lang::Ko); return 0; }
        if (wp == VK_RETURN) { ChooseLanguage(g_lang); return 0; }
        return 0;
      }
      if (stage_ == Stage::Confirm && wp == VK_SPACE) { Activate(kCheck); return 0; }
      if (stage_ == Stage::Confirm && wp == VK_RETURN) { StartUninstall(); return 0; }
      if (stage_ == Stage::Welcome && wp == VK_SPACE) { Activate(kCheck); return 0; }
      if (stage_ == Stage::Welcome && wp == VK_RETURN) { StartInstall(); return 0; }
      if (stage_ == Stage::Welcome && wp == VK_BACK) { SetStage(Stage::Language); return 0; }
      if (stage_ == Stage::Removed && wp == VK_RETURN) { RequestClose(true); return 0; }
      return 0;
    }

    case WM_SYSKEYDOWN:
      if (wp == VK_F4) { RequestClose(false); return 0; }  // Alt+F4
      return DefWindowProc(hwnd_, msg, wp, lp);

    case WM_CLOSE:
      RequestClose(false);
      return 0;

    case WM_DESTROY: {
      auto* closedProvider = new (std::nothrow) A11yProvider(this, A11yId::Root);
      if (closedProvider) { UiaRaiseAutomationEvent(closedProvider, UIA_Window_WindowClosedEventId); closedProvider->Release(); }
      KillTimer(hwnd_, kTickTimer); DiscardDeviceResources(); PostQuitMessage(0); return 0;
    }
  }
  return DefWindowProc(hwnd_, msg, wp, lp);
}

void SetupApp::RequestClose(bool force) {
  // While installing, Esc / close ask for confirmation first (do not abandon an
  // install silently). In demo and error states, close immediately.
  bool installing = (stage_ == Stage::Installing || stage_ == Stage::Preparing) && !demo_ && !force &&
                    mode_ == Mode::Install;
  // A removal cannot be safely abandoned halfway; closing waits for it to finish.
  if (stage_ == Stage::Removing && !force) return;
  if (installing) {
    int r = MessageBoxW(hwnd_,
        Tr(L"Redrob Launcher is still installing. Stop the installation and quit?",
           L"Redrob Launcher를 아직 설치하고 있어요. 설치를 중단하고 닫을까요?"),
        Tr(L"Redrob Launcher Setup", L"Redrob Launcher 설치"), MB_YESNO | MB_ICONWARNING | MB_DEFBUTTON2);
    if (r != IDYES) return;
  }
  DestroyWindow(hwnd_);
}

// What a click (on release) or a focused Enter/Space does. The same actions the screens always had.
void SetupApp::Activate(int ctl) {
  if (ctl == kClose) { RequestClose(false); return; }
  if (ctl == kMin) { ShowWindow(hwnd_, SW_MINIMIZE); return; }
  if (stage_ == Stage::Language) {
    // The two buttons ARE the choice: picking one moves on, in that language.
    if (ctl == kRetry) { ChooseLanguage(Lang::En); return; }
    if (ctl == kErrClose) { ChooseLanguage(Lang::Ko); return; }
  }
  if (stage_ == Stage::Confirm) {
    if (ctl == kCheck) {
      const bool before = deleteData_;
      deleteData_ = !deleteData_;
      InvalidateRect(hwnd_, nullptr, FALSE);
      RaiseToggleChanged(this, before, deleteData_);
      return;
    }
    if (ctl == kRetry) { StartUninstall(); return; }       // "Uninstall"
    if (ctl == kErrClose) { RequestClose(true); return; }  // "Cancel"
  }
  if (stage_ == Stage::Welcome) {
    if (ctl == kCheck) {
      const bool before = desktopShortcut_;
      desktopShortcut_ = !desktopShortcut_;
      InvalidateRect(hwnd_, nullptr, FALSE);
      RaiseToggleChanged(this, before, desktopShortcut_);
      return;
    }
    if (ctl == kRetry) { StartInstall(); return; }               // "Install"
    if (ctl == kErrClose) { SetStage(Stage::Language); return; } // "Back"
  }
  if (stage_ == Stage::Removed && ctl == kRetry) { RequestClose(true); return; }  // "Close"
  if (stage_ == Stage::Error && mode_ == Mode::Uninstall && ctl == kRetry) {
    errorMessage_.clear();
    SetStage(Stage::Confirm);  // back to the question, so a retry is still a choice
    return;
  }
  if (stage_ == Stage::Error && ctl == kRetry) { errorMessage_.clear(); StartInstall(); return; }
  if (stage_ == Stage::Error && ctl == kErrClose) { RequestClose(true); return; }
}

// The control under `pt` (device px). Only the controls the current screen drew have a rect.
int SetupApp::HitTest(POINT pt) const {
  if (PtInRect(&rcClose_, pt)) return kClose;
  if (PtInRect(&rcMin_, pt)) return kMin;
  if (PtInRect(&rcRetry_, pt)) return kRetry;
  if (PtInRect(&rcErrClose_, pt)) return kErrClose;
  if (PtInRect(&rcCheck_, pt)) return kCheck;
  return kNone;
}

void SetupApp::UpdateHover() {
  if (!hwnd_) return;
  POINT pt;
  GetCursorPos(&pt);
  ScreenToClient(hwnd_, &pt);
  RECT rc;
  GetClientRect(hwnd_, &rc);
  const int hit = PtInRect(&rc, pt) ? HitTest(pt) : kNone;
  if (hit != hover_) { hover_ = hit; StartAnim(); }
}

// Tab order, left to right as drawn: the checkbox, then the secondary button, then the primary.
std::vector<int> SetupApp::Focusables() const {
  switch (stage_) {
    case Stage::Language: return {kRetry, kErrClose};  // English, 한국어
    case Stage::Welcome: case Stage::Confirm: return {kCheck, kErrClose, kRetry};
    case Stage::Error: return {kErrClose, kRetry};
    case Stage::Removed: return {kRetry};
    default: return {};
  }
}

bool SetupApp::IsFocusable(int ctl) const {
  const std::vector<int> order = Focusables();
  return std::find(order.begin(), order.end(), ctl) != order.end();
}

bool SetupApp::WindowHasKeyboardFocus() const { return hwnd_ && ::GetFocus() == hwnd_; }

void SetupApp::SetLogicalFocus(int ctl, bool showRing) {
  if (ctl != kNone && !IsFocusable(ctl)) return;
  const bool changed = focus_ != ctl;
  focus_ = ctl;
  focusVisible_ = ctl != kNone && showRing;
  if (hwnd_) InvalidateRect(hwnd_, nullptr, FALSE);
  if (changed && WindowHasKeyboardFocus())
    RaiseFocusChanged(this, A11yProvider::IdForControl(focus_));
}

void SetupApp::StartAnim() {
  if (!hwnd_) return;
  if (reducedMotion_) {
    for (int i = 0; i < kCtlCount; ++i) hoverT_[i] = (i == hover_ || i == pressed_) ? 1.0f : 0.0f;
    InvalidateRect(hwnd_, nullptr, FALSE);
    return;
  }
  animLast_ = GetTickCount64();
  SetTimer(hwnd_, kAnimTimer, kTickMs, nullptr);
  InvalidateRect(hwnd_, nullptr, FALSE);
}

bool SetupApp::AnimStep() {
  const ULONGLONG now = GetTickCount64();
  const float step = (float)(now - animLast_) / kHoverMs;
  animLast_ = now;
  bool moving = false;
  for (int i = 0; i < kCtlCount; ++i) {
    const float target = (i == hover_ || i == pressed_) ? 1.0f : 0.0f;
    if (hoverT_[i] < target) hoverT_[i] = std::min(target, hoverT_[i] + step);
    else if (hoverT_[i] > target) hoverT_[i] = std::max(target, hoverT_[i] - step);
    if (hoverT_[i] != target) moving = true;
  }
  return moving || StageFade() < 1.0f;
}

// DS standard easing, cubic-bezier(0.2, 0, 0, 1), solved for x by bisection.
static float StandardEase(float t) {
  if (t <= 0) return 0;
  if (t >= 1) return 1;
  auto bez = [](float a, float b, float u) { return 3 * a * u * (1 - u) * (1 - u) + 3 * b * u * u * (1 - u) + u * u * u; };
  float lo = 0, hi = 1, u = t;
  for (int i = 0; i < 20; ++i) { u = (lo + hi) / 2; if (bez(0.2f, 0.0f, u) < t) lo = u; else hi = u; }
  return bez(0.0f, 1.0f, u);
}

float SetupApp::Eased(int ctl) const { return StandardEase(hoverT_[ctl]); }

// The current screen's content opacity: 0 -> 1 over 220 ms after the screen changes; 1 under reduced motion.
float SetupApp::StageFade() const {
  if (reducedMotion_ || !hwnd_) return 1.0f;
  return StandardEase(std::min(1.0f, (float)(GetTickCount64() - stageTick_) / kStageMs));
}

void SetupApp::SetStage(Stage s) {
  if (s != stage_) stageTick_ = GetTickCount64();
  stage_ = s;
  if (s == Stage::Opening) openedAt_ = GetTickCount64();
  if (s == Stage::Error || s == Stage::Confirm || s == Stage::Removed || s == Stage::Welcome || s == Stage::Language)
    KillTimer(hwnd_, kTickTimer);
  // A new screen starts with no virtual child focused (Enter keeps its primary-action meaning).
  const bool movedFocusToRoot = focus_ != kNone && WindowHasKeyboardFocus();
  focus_ = kNone;
  focusVisible_ = false;
  pressed_ = kNone;
  if (hwnd_) { UpdateHover(); StartAnim(); }
  if (hwnd_) {
    InvalidateRect(hwnd_, nullptr, FALSE);
    // Paint synchronously so a stage-change event never exposes stale fragment bounds.
    UpdateWindow(hwnd_);
  }
  RaiseStructureChanged(this);
  // Structure comes first so clients cannot resolve the old child when focus moves to the fragment root.
  if (movedFocusToRoot) RaiseFocusChanged(this, A11yId::Root);
}

// The language screen's answer: from here on every string, including the window title, is in `l`.
void SetupApp::ChooseLanguage(Lang l) {
  g_lang = l;
  if (hwnd_) SetWindowTextW(hwnd_, mode_ == Mode::Uninstall ? Tr(L"Uninstall Redrob Launcher", L"Redrob Launcher 제거")
                                                            : Tr(L"Redrob Launcher Setup", L"Redrob Launcher 설치"));
  SetStage(Stage::Welcome);
}

// The Install button (and Retry).
void SetupApp::StartInstall() {
  progress_ = progressTarget_ = 0.0;
  startTick_ = GetTickCount64();
  payloadDone_ = false;
  SetStage(Stage::Preparing);
  SetTimer(hwnd_, kTickTimer, kTickMs, nullptr);
  if (!demo_) {
    if (payloadThread_) { WaitForSingleObject(payloadThread_, 5000); CloseHandle(payloadThread_); }
    payloadThread_ = CreateThread(nullptr, 0, &SetupApp::PayloadThunk, this, 0, nullptr);
  }
}

static double EaseOut(double t) { return 1 - (1 - t) * (1 - t); }

void SetupApp::OnTick() {
  ULONGLONG elapsed = GetTickCount64() - startTick_;

  if (mode_ == Mode::Uninstall) {
    // Removal reports no progress either: the same eased ramp, over ~12s, capped until it finishes.
    if (stage_ == Stage::Removing) {
      double t = std::min(1.0, (GetTickCount64() - startTick_) / 12000.0);
      progressTarget_ = 0.90 * (1 - (1 - t) * (1 - t));
    }
  } else if (demo_ && stage_ != Stage::Error) {
    // Scripted demo, same shape as a real run: Preparing (0-0.8s) -> Installing (0.8-5s, bar to ~0.9)
    // -> Opening (5-6.5s) -> close.
    if (elapsed < 800) { if (stage_ != Stage::Preparing) SetStage(Stage::Preparing); progressTarget_ = 0.04; }
    else if (elapsed < 5000) { if (stage_ != Stage::Installing) SetStage(Stage::Installing);
      progressTarget_ = 0.90 * EaseOut((elapsed - 800) / 4200.0); }
    else if (elapsed < 6500) { if (stage_ != Stage::Opening) SetStage(Stage::Opening); progressTarget_ = 1.0; }
    else { DestroyWindow(hwnd_); return; }
  } else if (!demo_) {
    // Real run. Preparing turns into Installing after 0.6s; the bar is a time-based eased ramp, capped
    // until the worker finishes.
    if (stage_ == Stage::Preparing && elapsed > 600) SetStage(Stage::Installing);
    const double since = (double)(GetTickCount64() - stageTick_);
    if (stage_ == Stage::Installing && !payloadDone_) {
      progressTarget_ = 0.90 * EaseOut(std::min(1.0, since / 30000.0));  // approach over ~30s
    }
    if (stage_ == Stage::Opening) {
      progressTarget_ = 1.0;
      // Close on a fixed delay after Opening began: the launcher has been started (InstallLauncher), so
      // there is nothing left for this window to do but get out of the way.
      if (GetTickCount64() - openedAt_ > 1400) { DestroyWindow(hwnd_); return; }
    }
  }

  // Ease displayed progress toward the target (motion < 1/3 s per step).
  const double previousProgress = progress_;
  progress_ += (progressTarget_ - progress_) * 0.12;
  if (progress_ > 0.9999) progress_ = 1.0;
  RaiseProgressChanged(this, previousProgress, progress_);
  InvalidateRect(hwnd_, nullptr, FALSE);
}

// ---- rendering -------------------------------------------------------------

void SetupApp::CreateDeviceResources() {
  if (rt_) return;
  RECT rc; GetClientRect(hwnd_, &rc);
  D2D1_SIZE_U size = D2D1::SizeU(rc.right - rc.left, rc.bottom - rc.top);
  D2D1_RENDER_TARGET_PROPERTIES props = D2D1::RenderTargetProperties(
      D2D1_RENDER_TARGET_TYPE_DEFAULT,
      D2D1::PixelFormat(DXGI_FORMAT_B8G8R8A8_UNORM, D2D1_ALPHA_MODE_PREMULTIPLIED));
  ComPtr<ID2D1HwndRenderTarget> hwndRt;
  HRESULT hr = d2dFactory_->CreateHwndRenderTarget(
      props, D2D1::HwndRenderTargetProperties(hwnd_, size, D2D1_PRESENT_OPTIONS_NONE), &hwndRt);
  (void)hr;
  if (!hwndRt) return;
  rt_.p = hwndRt.p;  // ID2D1HwndRenderTarget is an ID2D1RenderTarget; rt_ takes the reference
  hwndRt.p = nullptr;
  rt_->SetDpi((float)dpi_, (float)dpi_);
  symbolBitmap_ = LoadPngResource(IDR_SYMBOL_PNG, rt_.p);
}

void SetupApp::DiscardDeviceResources() {
  symbolBitmap_.reset();
  rt_.reset();
}

ComPtr<ID2D1Bitmap> SetupApp::LoadPngResource(int resId, ID2D1RenderTarget* rt) {
  ComPtr<ID2D1Bitmap> out;
  HRSRC res = FindResourceW(hinst_, MAKEINTRESOURCEW(resId), RT_RCDATA);
  if (!res) return out;
  HGLOBAL h = LoadResource(hinst_, res);
  void* data = LockResource(h);
  DWORD sz = SizeofResource(hinst_, res);
  if (!data || !sz) return out;

  ComPtr<IWICStream> stream;
  wicFactory_->CreateStream(&stream);
  stream->InitializeFromMemory((BYTE*)data, sz);
  ComPtr<IWICBitmapDecoder> dec;
  if (FAILED(wicFactory_->CreateDecoderFromStream(stream.p, nullptr,
          WICDecodeMetadataCacheOnLoad, &dec))) return out;
  ComPtr<IWICBitmapFrameDecode> frame;
  dec->GetFrame(0, &frame);
  ComPtr<IWICFormatConverter> conv;
  wicFactory_->CreateFormatConverter(&conv);
  conv->Initialize(frame.p, GUID_WICPixelFormat32bppPBGRA, WICBitmapDitherTypeNone,
                   nullptr, 0.0, WICBitmapPaletteTypeMedianCut);
  rt->CreateBitmapFromWicBitmap(conv.p, nullptr, &out);
  return out;
}

void SetupApp::OnPaint() {
  CreateDeviceResources();
  PAINTSTRUCT ps; BeginPaint(hwnd_, &ps);
  if (!rt_) { EndPaint(hwnd_, &ps); return; }
  if (DrawScene() == D2DERR_RECREATE_TARGET) DiscardDeviceResources();
  EndPaint(hwnd_, &ps);
}

// Where a screen sits among the three steps the stepper shows.
struct StepState { int current; bool allDone; };
static StepState StepOf(Stage s, Mode m) {
  if (m == Mode::Uninstall) {
    switch (s) {
      case Stage::Removing: case Stage::Error: return {1, false};
      case Stage::Removed: return {2, true};
      default: return {0, false};
    }
  }
  switch (s) {
    case Stage::Language: return {0, false};
    case Stage::Opening: return {2, false};
    default: return {1, false};  // Welcome, Preparing, Installing, Error
  }
}

// Korean breaks between words, never inside one (DS 35-korean: keep-all). DirectWrite allows a break
// between any two Hangul syllables, so a word joiner (U+2060, zero width) is put inside each word.
static std::wstring KeepAll(const std::wstring& s) {
  auto hangul = [](wchar_t c) { return (c >= 0xAC00 && c <= 0xD7A3) || (c >= 0x3130 && c <= 0x318F); };
  std::wstring out;
  out.reserve(s.size() * 2);
  for (size_t i = 0; i < s.size(); ++i) {
    out += s[i];
    if (i + 1 < s.size() && (hangul(s[i]) || hangul(s[i + 1])) && !iswspace(s[i]) && !iswspace(s[i + 1]))
      out += L'\u2060';
  }
  return out;
}

float SetupApp::TextWidth(const std::wstring& s, IDWriteTextFormat* f) {
  ComPtr<IDWriteTextLayout> layout;
  if (!f || FAILED(dwFactory_->CreateTextLayout(s.c_str(), (UINT32)s.size(), f, 4096.0f, 64.0f, &layout))) return 0;
  ApplyHangulFont(layout.p, s);
  DWRITE_TEXT_METRICS tm{};
  return SUCCEEDED(layout->GetMetrics(&tm)) ? tm.widthIncludingTrailingWhitespace : 0;
}

// Left-aligned, wrapped text at (x, y), `w` wide. Returns the height it took.
float SetupApp::DrawBlock(const std::wstring& s, IDWriteTextFormat* f, float x, float y, float w, ID2D1Brush* b) {
  if (!f || s.empty()) return 0;
  const std::wstring text = KeepAll(s);
  ComPtr<IDWriteTextLayout> layout;
  if (FAILED(dwFactory_->CreateTextLayout(text.c_str(), (UINT32)text.size(), f, w, 400.0f, &layout))) return 0;
  ApplyHangulFont(layout.p, text);
  layout->SetLocaleName(g_lang == Lang::Ko ? L"ko-kr" : L"en-us", DWRITE_TEXT_RANGE{0, (UINT32)text.size()});
  rt_->DrawTextLayout(D2D1::Point2F(x, y), layout.p, b);
  DWRITE_TEXT_METRICS tm{};
  return SUCCEEDED(layout->GetMetrics(&tm)) ? tm.height : 0;
}

// The brand panel: Redrob Blue radial gradient (#2E7EFF -> #2B52FF -> #2746EE), a soft white glint top-left
// and a violet one bottom-right, the white Redrob symbol near the top and the product name at the foot. The
// symbol is the design system's file, drawn as is: never tinted, shadowed or redrawn.
void SetupApp::DrawBrandPanel(float H) {
  const D2D1_RECT_F r = D2D1::RectF(0, 0, kPanelW, H);
  auto radial = [&](D2D1_POINT_2F c, float rx, float ry, const D2D1_GRADIENT_STOP* stops, UINT n) {
    ComPtr<ID2D1GradientStopCollection> coll;
    if (FAILED(rt_->CreateGradientStopCollection(stops, n, &coll))) return;
    ComPtr<ID2D1RadialGradientBrush> brush;
    if (FAILED(rt_->CreateRadialGradientBrush(
            D2D1::RadialGradientBrushProperties(c, D2D1::Point2F(0, 0), rx, ry), coll.p, &brush))) return;
    rt_->FillRectangle(r, brush.p);
  };
  const D2D1_GRADIENT_STOP bed[] = {{0.0f, kPanelCore}, {0.55f, kPanelMid}, {1.0f, kPanelEdge}};
  radial(D2D1::Point2F(kPanelW * 0.30f, H * 0.34f), kPanelW * 1.25f, H * 0.85f, bed, 3);
  const D2D1_COLOR_F w0 = {1, 1, 1, 0.30f}, w1 = {1, 1, 1, 0.0f};
  const D2D1_GRADIENT_STOP glint[] = {{0.0f, w0}, {1.0f, w1}};
  radial(D2D1::Point2F(kPanelW * 0.06f, H * 0.02f), 150.0f, 150.0f, glint, 2);
  D2D1_COLOR_F v0 = kPanelViolet, v1 = kPanelViolet, v2 = kPanelViolet;
  v0.a = 0.70f; v1.a = 0.22f; v2.a = 0.0f;
  const D2D1_GRADIENT_STOP violet[] = {{0.0f, v0}, {0.5f, v1}, {1.0f, v2}};
  radial(D2D1::Point2F(kPanelW, H), 230.0f, 230.0f, violet, 3);

  if (symbolBitmap_) {
    const float s = 40.0f;  // >= 18 px minimum (10-logo.md); drawn down from the 192 px file
    rt_->DrawBitmap(symbolBitmap_.p, D2D1::RectF(kPad, 44.0f, kPad + s, 44.0f + s), 1.0f,
                    D2D1_BITMAP_INTERPOLATION_MODE_LINEAR);
  }
  ComPtr<ID2D1SolidColorBrush> white;
  rt_->CreateSolidColorBrush(kWhite, &white);
  if (white) DrawBlock(L"Redrob Launcher", fmtBrand_.p, kPad, H - kPad - 20.0f, kPanelW - 2 * kPad, white.p);
}

// The DS Stepper, small: three round markers (done = blue with a white tick, current = blue ring with a
// halo, next = outline), each with its label, joined by a hairline (blue once passed).
void SetupApp::DrawStepper(float x, float y) {
  const Palette& p = Pal();
  ComPtr<ID2D1SolidColorBrush> ink, secondary, blue, strong, hairline, base, halo, brand, white;
  rt_->CreateSolidColorBrush(p.ink, &ink);
  rt_->CreateSolidColorBrush(p.secondary, &secondary);
  rt_->CreateSolidColorBrush(p.blue, &blue);
  rt_->CreateSolidColorBrush(p.strong, &strong);
  rt_->CreateSolidColorBrush(p.hairline, &hairline);
  rt_->CreateSolidColorBrush(p.base, &base);
  rt_->CreateSolidColorBrush(p.brandSubtle, &halo);
  rt_->CreateSolidColorBrush(p.inkBrand, &brand);
  rt_->CreateSolidColorBrush(kWhite, &white);
  if (!ink || !white) return;
  const StepState st = StepOf(stage_, mode_);
  const wchar_t* labels[3];
  if (mode_ == Mode::Uninstall) {
    labels[0] = Tr(L"Confirm", L"확인"); labels[1] = Tr(L"Remove", L"제거"); labels[2] = Tr(L"Done", L"완료");
  } else {
    labels[0] = Tr(L"Language", L"언어"); labels[1] = Tr(L"Install", L"설치"); labels[2] = Tr(L"Open", L"열기");
  }
  const float d = 20.0f, rr = d / 2;
  float cx = x;
  for (int i = 0; i < 3; ++i) {
    const bool done = i < st.current || st.allDone;
    const bool cur = i == st.current && !st.allDone;
    const D2D1_POINT_2F c = D2D1::Point2F(cx + rr, y + rr);
    if (cur) {
      rt_->FillEllipse(D2D1::Ellipse(c, rr + 3, rr + 3), halo.p);
      rt_->FillEllipse(D2D1::Ellipse(c, rr, rr), base.p);
      rt_->DrawEllipse(D2D1::Ellipse(c, rr - 0.5f, rr - 0.5f), blue.p, 1.0f);
    } else if (done) {
      rt_->FillEllipse(D2D1::Ellipse(c, rr, rr), blue.p);
      rt_->DrawLine(D2D1::Point2F(c.x - 4.0f, c.y + 0.2f), D2D1::Point2F(c.x - 1.2f, c.y + 3.0f), white.p, 1.6f);
      rt_->DrawLine(D2D1::Point2F(c.x - 1.2f, c.y + 3.0f), D2D1::Point2F(c.x + 4.2f, c.y - 3.2f), white.p, 1.6f);
    } else {
      rt_->FillEllipse(D2D1::Ellipse(c, rr, rr), base.p);
      rt_->DrawEllipse(D2D1::Ellipse(c, rr - 0.5f, rr - 0.5f), strong.p, 1.0f);
    }
    if (!done && fmtStep_) {
      const wchar_t num[2] = {(wchar_t)(L'1' + i), 0};
      ComPtr<IDWriteTextLayout> layout;
      if (SUCCEEDED(dwFactory_->CreateTextLayout(num, 1, fmtStep_.p, d, d, &layout))) {
        layout->SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER);
        rt_->DrawTextLayout(D2D1::Point2F(cx, y), layout.p, cur ? brand.p : secondary.p);
      }
    }
    const float lw = TextWidth(labels[i], fmtStep_.p);
    if (fmtStep_) {
      DrawTextLine(labels[i], fmtStep_.p, D2D1::RectF(cx + d + 8, y, cx + d + 8 + lw + 2, y + d),
                   (done || cur) ? ink.p : secondary.p);
    }
    cx += d + 8 + lw;
    if (i < 2) {
      rt_->DrawLine(D2D1::Point2F(cx + 10, y + rr), D2D1::Point2F(cx + 34, y + rr),
                    (i < st.current || st.allDone) ? blue.p : hairline.p, 1.0f);
      cx += 44;
    }
  }
}

HRESULT SetupApp::DrawScene() {
  const Palette& p = Pal();
  rt_->BeginDraw();
  rt_->Clear(p.base);

  const float W = (float)kBaseW, H = (float)kBaseH;
  rcBrand_ = ToDevice(0, 0, kPanelW, H);
  DrawBrandPanel(H);

  ComPtr<ID2D1SolidColorBrush> ink, secondary, blue, sunken, strong, base, white;
  rt_->CreateSolidColorBrush(p.ink, &ink);
  rt_->CreateSolidColorBrush(p.secondary, &secondary);
  rt_->CreateSolidColorBrush(p.blue, &blue);
  rt_->CreateSolidColorBrush(p.sunken, &sunken);
  rt_->CreateSolidColorBrush(p.strong, &strong);
  rt_->CreateSolidColorBrush(p.base, &base);
  rt_->CreateSolidColorBrush(kWhite, &white);
  if (!ink || !white) return rt_->EndDraw();

  // ---- caption buttons (minimise, close), 46 x 40 each, top-right ------------
  rcClose_ = ToDevice(W - kCapBtnW, 0, kCapBtnW, kCaption);
  rcMin_ = ToDevice(W - 2 * kCapBtnW, 0, kCapBtnW, kCaption);
  {
    const float cy = kCaption / 2, g = 5.0f;  // 10 dip glyphs
    const float closeX = W - kCapBtnW / 2, minX = W - kCapBtnW * 1.5f;
    // Hover: 8% ink behind minimise; status-danger behind close, with a white glyph.
    const float hm = Eased(kMin), hc = Eased(kClose);
    if (hm > 0) {
      ComPtr<ID2D1SolidColorBrush> fill;
      D2D1_COLOR_F c = p.ink; c.a = 0.08f * hm;
      rt_->CreateSolidColorBrush(c, &fill);
      if (fill) rt_->FillRectangle(D2D1::RectF(W - 2 * kCapBtnW, 0, W - kCapBtnW, kCaption), fill.p);
    }
    ComPtr<ID2D1SolidColorBrush> closeGlyph;
    rt_->CreateSolidColorBrush(Mix(p.secondary, kWhite, hc), &closeGlyph);
    if (hc > 0) {
      ComPtr<ID2D1SolidColorBrush> fill;
      D2D1_COLOR_F c = p.danger; c.a = hc;
      rt_->CreateSolidColorBrush(c, &fill);
      if (fill) rt_->FillRectangle(D2D1::RectF(W - kCapBtnW, 0, W, kCaption), fill.p);
    }
    rt_->DrawLine(D2D1::Point2F(closeX - g, cy - g), D2D1::Point2F(closeX + g, cy + g), closeGlyph.p, 1.0f);
    rt_->DrawLine(D2D1::Point2F(closeX + g, cy - g), D2D1::Point2F(closeX - g, cy + g), closeGlyph.p, 1.0f);
    rt_->DrawLine(D2D1::Point2F(minX - g, cy), D2D1::Point2F(minX + g, cy), secondary.p, 1.0f);
  }

  // The content column fades in with each new screen (not the progress bar, which carries on).
  const float fade = StageFade();
  ComPtr<ID2D1Layer> layer;
  if (fade < 1.0f && SUCCEEDED(rt_->CreateLayer(&layer)))
    rt_->PushLayer(D2D1::LayerParameters(D2D1::InfiniteRect(), nullptr, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
                                         D2D1::IdentityMatrix(), fade), layer.p);

  // ---- content column: stepper, title, body, controls; left aligned -----------
  const float x = kPanelW + kPad, cw = W - x - kPad;
  rcStepper_ = ToDevice(x, 48.0f, cw, 28.0f);
  DrawStepper(x, 52.0f);
  float y = 104.0f;
  const float titleY = y;
  const float titleHeight = DrawBlock(StatusText(stage_), fmtTitle_.p, x, y, cw, ink.p);
  rcTitle_ = ToDevice(x, titleY, cw, titleHeight);
  y += titleHeight + 8.0f;
  const float bodyY = y;
  const float bodyHeight = DrawBlock(BodyText(stage_, deleteData_, errorMessage_), fmtBody_.p, x, y, cw, secondary.p);
  rcBody_ = ToDevice(x, bodyY, cw, bodyHeight);
  y += bodyHeight;

  // DS Button: radius 8, 40 tall, 20 side padding (min 96); primary Redrob Blue with white type, secondary a
  // 1 px border-strong outline on the ground. Laid out from the right edge; returns its left edge.
  const float bh = 40.0f, by = H - kPad - bh;
  // DS focus ring: 2 px focus-ring, 2 px outside the control, following its radius. Keyboard focus only.
  auto focusRing = [&](int ctl, D2D1_RECT_F r, float radius) {
    // Offline stage fixtures have no HWND; interactive rings additionally require real window focus.
    if (!focusVisible_ || (hwnd_ && !WindowHasKeyboardFocus()) || focus_ != ctl) return;
    ComPtr<ID2D1SolidColorBrush> ring;
    rt_->CreateSolidColorBrush(p.focus, &ring);
    if (ring) rt_->DrawRoundedRectangle(D2D1::RoundedRect(D2D1::RectF(r.left - 3, r.top - 3, r.right + 3, r.bottom + 3),
                                                          radius + 3, radius + 3), ring.p, 2.0f);
  };
  // Pressed reads as full hover (DS: action-primary-hover is hover AND pressed).
  auto button = [&](RECT& rc, int ctl, float right, const wchar_t* label, bool primary) -> float {
    const float w = std::max(96.0f, TextWidth(label, fmtButton_.p) + 40.0f);
    const float l = right - w;
    const D2D1_RECT_F br = D2D1::RectF(l, by, right, by + bh);
    const float t = pressed_ == ctl ? 1.0f : Eased(ctl);
    ComPtr<ID2D1SolidColorBrush> fill;
    rt_->CreateSolidColorBrush(primary ? Mix(p.blue, p.blueHover, t) : Mix(p.base, p.sunken, t), &fill);
    if (fill) rt_->FillRoundedRectangle(D2D1::RoundedRect(br, 8, 8), fill.p);
    if (!primary)
      rt_->DrawRoundedRectangle(D2D1::RoundedRect(D2D1::RectF(l + 0.5f, by + 0.5f, right - 0.5f, by + bh - 0.5f), 7.5f, 7.5f),
                                strong.p, 1.0f);
    focusRing(ctl, br, 8.0f);
    DrawTextLine(label, fmtButton_.p, br, primary ? white.p : ink.p);
    rc = ToDevice(l, by, w, bh);
    return l;
  };
  auto twoButtons = [&](const wchar_t* primary, const wchar_t* secondaryLabel) {
    const float l = button(rcRetry_, kRetry, W - kPad, primary, true);
    button(rcErrClose_, kErrClose, l - 8.0f, secondaryLabel, false);
  };
  // DS Checkbox: 18 px box, radius 4, 1 px border-strong; checked = blue with a white tick. The row (box and
  // label) is the hit target; hovering it fills an empty box with sunken, a checked one with the hover blue.
  auto checkRow = [&](float rowY, const std::wstring& label, bool checked) {
    const float box = 18.0f, bx = x, byy = rowY + 1.0f;
    const float t = pressed_ == kCheck ? 1.0f : Eased(kCheck);
    const D2D1_RECT_F boxR = D2D1::RectF(bx, byy, bx + box, byy + box);
    const D2D1_ROUNDED_RECT boxRR = D2D1::RoundedRect(boxR, 4.0f, 4.0f);
    ComPtr<ID2D1SolidColorBrush> fill;
    rt_->CreateSolidColorBrush(checked ? Mix(p.blue, p.blueHover, t) : Mix(p.base, p.sunken, t), &fill);
    if (fill) rt_->FillRoundedRectangle(boxRR, fill.p);
    if (checked) {
      rt_->DrawLine(D2D1::Point2F(bx + 4.5f, byy + 9.5f), D2D1::Point2F(bx + 7.75f, byy + 12.5f), white.p, 1.8f);
      rt_->DrawLine(D2D1::Point2F(bx + 7.75f, byy + 12.5f), D2D1::Point2F(bx + 13.5f, byy + 6.0f), white.p, 1.8f);
    } else {
      rt_->DrawRoundedRectangle(D2D1::RoundedRect(D2D1::RectF(bx + 0.5f, byy + 0.5f, bx + box - 0.5f, byy + box - 0.5f),
                                                  3.5f, 3.5f), strong.p, 1.0f);
    }
    focusRing(kCheck, boxR, 4.0f);
    const float lw = TextWidth(label, fmtLabel_.p);
    DrawTextLine(label, fmtLabel_.p, D2D1::RectF(bx + box + 8, rowY, bx + box + 8 + lw + 2, rowY + 20), ink.p);
    rcCheck_ = ToDevice(bx - 4, rowY - 4, box + 8 + lw + 8, 28);
  };

  rcRetry_ = rcErrClose_ = rcCheck_ = rcProgress_ = RECT{};
  if (stage_ == Stage::Language) {
    // Bilingual by nature: it comes before the choice. The two buttons are the choice; the preselected
    // one (a previous install's answer, or the Windows UI language) is drawn as the primary.
    const float l = button(rcErrClose_, kErrClose, W - kPad, L"한국어", g_lang == Lang::Ko);
    button(rcRetry_, kRetry, l - 8.0f, L"English", g_lang == Lang::En);
  } else if (stage_ == Stage::Welcome) {
    checkRow(y + 20.0f, Tr(L"Create a desktop shortcut", L"바탕 화면에 바로 가기 만들기"), desktopShortcut_);
    twoButtons(Tr(L"Install", L"설치"), Tr(L"Back", L"뒤로"));
  } else if (stage_ == Stage::Confirm) {
    checkRow(y + 20.0f, Tr(L"Also delete its settings and downloads", L"설정과 다운로드한 파일도 삭제"), deleteData_);
    twoButtons(Tr(L"Uninstall", L"제거"), Tr(L"Cancel", L"취소"));
  } else if (stage_ == Stage::Removed) {
    button(rcRetry_, kRetry, W - kPad, Tr(L"Close", L"닫기"), true);
  } else if (stage_ == Stage::Error) {
    twoButtons(Tr(L"Retry", L"다시 시도"), Tr(L"Close", L"닫기"));
  } else {
    if (layer) { rt_->PopLayer(); layer.reset(); }
    // ---- DS Progress: 8 px pill track (sunken), blue bar ------------------------
    const float barY = y + 24.0f, barH = 8.0f;
    rcProgress_ = ToDevice(x, barY, cw, barH);
    rt_->FillRoundedRectangle(D2D1::RoundedRect(D2D1::RectF(x, barY, x + cw, barY + barH), 4, 4), sunken.p);
    const float fillW = (float)(cw * progress_);
    if (fillW > 0.5f) {
      rt_->FillRoundedRectangle(
          D2D1::RoundedRect(D2D1::RectF(x, barY, x + std::max(fillW, barH), barY + barH), 4, 4), blue.p);
    }
  }
  if (layer) rt_->PopLayer();

  return rt_->EndDraw();
}

int SetupApp::RenderStages(HINSTANCE hinst, const std::wstring& dir) {
  hinst_ = hinst;
  g_app = this;
  // Render automation accepts a fresh nested destination rather than failing each encoder opaquely.
  const int directoryResult = SHCreateDirectoryExW(nullptr, dir.c_str(), nullptr);
  if (directoryResult != ERROR_SUCCESS && directoryResult != ERROR_ALREADY_EXISTS && directoryResult != ERROR_FILE_EXISTS) return 2;
  CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);
  D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, &d2dFactory_);
  DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, __uuidof(IDWriteFactory),
                      reinterpret_cast<IUnknown**>(&dwFactory_));
  CoCreateInstance(CLSID_WICImagingFactory, nullptr, CLSCTX_INPROC_SERVER, IID_PPV_ARGS(&wicFactory_));
  if (!d2dFactory_ || !dwFactory_ || !wicFactory_) return 2;
  LayoutText();
#ifdef REDROB_REQUIRE_BUNDLED_FONTS
  if (!bundledFontsReady_) {
    // Headless validation reports a font-resource failure distinctly and releases COM in-order.
    fmtTitle_.reset(); fmtBody_.reset(); fmtButton_.reset(); fmtLabel_.reset(); fmtStep_.reset(); fmtBrand_.reset();
    fontColl_.reset(); wicFactory_.reset(); dwFactory_.reset(); d2dFactory_.reset(); CoUninitialize();
    return 3;
  }
#endif
  g_dark = SystemPrefersDark();  // --theme light|dark picks one; otherwise Windows' own
  dpi_ = 192;  // 200%: the sharpest the art is drawn for
  const UINT pw = MulDiv(kBaseW, dpi_, 96), ph = MulDiv(kBaseH, dpi_, 96);
  int failures = 0;
  struct Shot { Stage stage; double progress; const wchar_t* name; Lang lang; bool flag; int states = 0; bool uninstall = false; };
  const Shot shots[] = {
    // states 1: pointer over Back, keyboard focus on Install; 2: pointer over close, focus on the checkbox.
    {Stage::Welcome, 0.0, L"8-states-buttons.png", Lang::En, true, 1},
    {Stage::Welcome, 0.0, L"8-states-checkbox.png", Lang::Ko, false, 2},
    {Stage::Language, 0.0, L"0-language.png", Lang::En, false},
    {Stage::Language, 0.0, L"0-language-ko.png", Lang::Ko, false},
    {Stage::Welcome, 0.0, L"1-welcome.png", Lang::En, true},
    {Stage::Welcome, 0.0, L"1-welcome-ko.png", Lang::Ko, true},
    {Stage::Welcome, 0.0, L"1-welcome-unchecked.png", Lang::En, false},
    {Stage::Preparing, 0.04, L"2-preparing.png", Lang::En, false},
    {Stage::Preparing, 0.04, L"2-preparing-ko.png", Lang::Ko, false},
    {Stage::Installing, 0.55, L"2-installing.png", Lang::En, false},
    {Stage::Installing, 0.55, L"2-installing-ko.png", Lang::Ko, false},
    {Stage::Opening, 1.0, L"3-opening.png", Lang::En, false},
    {Stage::Opening, 1.0, L"3-opening-ko.png", Lang::Ko, false},
    {Stage::Error, 0.0, L"4-error.png", Lang::En, false},
    {Stage::Error, 0.0, L"4-error-ko.png", Lang::Ko, false},
    {Stage::Error, 0.0, L"4-uninstall-error.png", Lang::En, false, 0, true},
    {Stage::Error, 0.0, L"4-uninstall-error-ko.png", Lang::Ko, false, 0, true},
    {Stage::Confirm, 0.0, L"5-uninstall-confirm.png", Lang::En, false},
    {Stage::Confirm, 0.0, L"5-uninstall-confirm-ko.png", Lang::Ko, false},
    {Stage::Removing, 0.4, L"6-removing.png", Lang::En, false},
    {Stage::Removing, 0.4, L"6-removing-ko.png", Lang::Ko, false},
    {Stage::Removed, 1.0, L"7-removed.png", Lang::En, false},
    {Stage::Removed, 1.0, L"7-removed-ko.png", Lang::Ko, false},
    {Stage::Removed, 1.0, L"7-removed-deleted-ko.png", Lang::Ko, true},
  };
  for (const Shot& s : shots) {
    ComPtr<IWICBitmap> bitmap;
    if (FAILED(wicFactory_->CreateBitmap(pw, ph, GUID_WICPixelFormat32bppPBGRA, WICBitmapCacheOnLoad, &bitmap))) {
      ++failures; continue;
    }
    D2D1_RENDER_TARGET_PROPERTIES props = D2D1::RenderTargetProperties(
        D2D1_RENDER_TARGET_TYPE_DEFAULT,
        D2D1::PixelFormat(DXGI_FORMAT_B8G8R8A8_UNORM, D2D1_ALPHA_MODE_PREMULTIPLIED), (float)dpi_, (float)dpi_);
    DiscardDeviceResources();
    if (FAILED(d2dFactory_->CreateWicBitmapRenderTarget(bitmap.p, props, &rt_))) { ++failures; continue; }
    symbolBitmap_ = LoadPngResource(IDR_SYMBOL_PNG, rt_.p);
    mode_ = s.uninstall || s.stage == Stage::Confirm || s.stage == Stage::Removing || s.stage == Stage::Removed
                ? Mode::Uninstall : Mode::Install;
    stage_ = s.stage;
    progress_ = s.progress;
    g_lang = s.lang;
    desktopShortcut_ = s.flag;  // Welcome: the box ticked or not
    deleteData_ = s.flag;       // Removed: settings deleted too, or kept
    for (float& t : hoverT_) t = 0.0f;
    hover_ = focus_ = kNone;
    focusVisible_ = s.states != 0;
    if (s.states == 1) { hover_ = kErrClose; hoverT_[kErrClose] = 1.0f; focus_ = kRetry; }
    if (s.states == 2) { hover_ = kClose; hoverT_[kClose] = 1.0f; focus_ = kCheck; }
    if (s.stage == Stage::Error) errorMessage_ = ResultSentence(kResultAppRunning, mode_ == Mode::Uninstall);
    if (FAILED(DrawScene())) { ++failures; continue; }

    const std::wstring path = dir + L"\\" + s.name;
    ComPtr<IWICStream> stream;
    ComPtr<IWICBitmapEncoder> encoder;
    ComPtr<IWICBitmapFrameEncode> frame;
    bool ok = SUCCEEDED(wicFactory_->CreateStream(&stream)) &&
              SUCCEEDED(stream->InitializeFromFilename(path.c_str(), GENERIC_WRITE)) &&
              SUCCEEDED(wicFactory_->CreateEncoder(GUID_ContainerFormatPng, nullptr, &encoder)) &&
              SUCCEEDED(encoder->Initialize(stream.p, WICBitmapEncoderNoCache)) &&
              SUCCEEDED(encoder->CreateNewFrame(&frame, nullptr)) &&
              SUCCEEDED(frame->Initialize(nullptr)) &&
              SUCCEEDED(frame->WriteSource(bitmap.p, nullptr)) &&
              SUCCEEDED(frame->Commit()) && SUCCEEDED(encoder->Commit());
    if (!ok) ++failures;
  }
  DiscardDeviceResources();
  fmtTitle_.reset();
  fmtBody_.reset();
  fmtButton_.reset();
  fmtLabel_.reset();
  fmtStep_.reset();
  fmtBrand_.reset();
  fontColl_.reset();
  wicFactory_.reset();
  dwFactory_.reset();
  d2dFactory_.reset();
  CoUninitialize();
  return failures;
}

// device-pixel rect from logical (dip) coordinates
RECT SetupApp::ToDevice(float x, float y, float w, float h) {
  float s = dpi_ / 96.0f;
  RECT r;
  r.left = (LONG)(x * s); r.top = (LONG)(y * s);
  r.right = (LONG)((x + w) * s); r.bottom = (LONG)((y + h) * s);
  return r;
}

// ---- payload ---------------------------------------------------------------

DWORD WINAPI SetupApp::PayloadThunk(LPVOID self) {
  return reinterpret_cast<SetupApp*>(self)->RunPayload();
}

// %LOCALAPPDATA%, or empty.
static std::wstring LocalAppData() {
  wchar_t local[MAX_PATH];
  if (FAILED(SHGetFolderPathW(nullptr, CSIDL_LOCAL_APPDATA, nullptr, 0, local))) return L"";
  return local;
}

static std::wstring KnownFolder(REFKNOWNFOLDERID id) {
  PWSTR p = nullptr;
  std::wstring out;
  if (SUCCEEDED(SHGetKnownFolderPath(id, 0, nullptr, &p))) out = p;
  CoTaskMemFree(p);
  return out;
}

static std::wstring InstallDir() {
  const std::wstring local = LocalAppData();
  return local.empty() ? L"" : local + kInstallSubdir;
}

static bool Exists(const std::wstring& p) { return GetFileAttributesW(p.c_str()) != INVALID_FILE_ATTRIBUTES; }

// The one subfolder of `dir` that holds `rel` (a path starting with a backslash), or empty.
static std::wstring SingleSubfolderWith(const std::wstring& dir, const wchar_t* rel) {
  std::wstring found;
  WIN32_FIND_DATAW fd;
  HANDLE find = FindFirstFileW((dir + L"\\*").c_str(), &fd);
  if (find == INVALID_HANDLE_VALUE) return found;
  do {
    const std::wstring name = fd.cFileName;
    if (name == L"." || name == L".." || !(fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY)) continue;
    const std::wstring sub = dir + L"\\" + name;
    if (Exists(sub + rel)) { found = sub; break; }
  } while (FindNextFileW(find, &fd));
  FindClose(find);
  return found;
}

// Write one RCDATA resource to `path`. False if the resource is missing or the write was short.
static bool WriteResource(HINSTANCE hinst, int id, const std::wstring& path) {
  HRSRC res = FindResourceW(hinst, MAKEINTRESOURCEW(id), RT_RCDATA);
  if (!res) return false;
  HGLOBAL h = LoadResource(hinst, res);
  const void* data = LockResource(h);
  const DWORD sz = SizeofResource(hinst, res);
  if (!data || !sz) return false;
  HANDLE f = CreateFileW(path.c_str(), GENERIC_WRITE, 0, nullptr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, nullptr);
  if (f == INVALID_HANDLE_VALUE) return false;
  DWORD written = 0;
  const BOOL ok = WriteFile(f, data, sz, &written, nullptr);
  CloseHandle(f);
  if (!ok || written != sz) { DeleteFileW(path.c_str()); return false; }
  return true;
}

bool SetupApp::ExtractPayload(std::wstring& outPath) {
  wchar_t tmp[MAX_PATH];
  GetTempPathW(MAX_PATH, tmp);
  wchar_t name[MAX_PATH];
  if (!GetTempFileNameW(tmp, L"rcs", 0, name)) return false;
  const std::wstring zip = std::wstring(name) + L".zip";
  DeleteFileW(name);  // the 0-byte file GetTempFileName created
  if (!WriteResource(hinst_, IDR_PAYLOAD, zip)) return false;
  outPath = zip;
  return true;
}

// Delete a directory tree. Paths are checked by the callers to be inside the install folder or our own
// data folders; this never runs on anything it was not given. `keep` (a full path) is left in place.
static DWORD DeleteTree(const std::wstring& dir, const std::wstring& keep = L"") {
  DWORD firstError = 0;
  WIN32_FIND_DATAW fd;
  HANDLE find = FindFirstFileW((dir + L"\\*").c_str(), &fd);
  if (find != INVALID_HANDLE_VALUE) {
    do {
      const std::wstring name = fd.cFileName;
      if (name == L"." || name == L"..") continue;
      const std::wstring path = dir + L"\\" + name;
      if (!keep.empty() && lstrcmpiW(path.c_str(), keep.c_str()) == 0) continue;
      DWORD err = 0;
      if (fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
        // A junction is removed as a link; its target is not followed.
        if (fd.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT) err = RemoveDirectoryW(path.c_str()) ? 0 : GetLastError();
        else err = DeleteTree(path, keep);
      } else {
        if (fd.dwFileAttributes & FILE_ATTRIBUTE_READONLY)
          SetFileAttributesW(path.c_str(), fd.dwFileAttributes & ~FILE_ATTRIBUTE_READONLY);
        err = DeleteFileW(path.c_str()) ? 0 : GetLastError();
      }
      if (err && !firstError) firstError = err;
    } while (FindNextFileW(find, &fd));
    FindClose(find);
  }
  if (!RemoveDirectoryW(dir.c_str())) {
    const DWORD err = GetLastError();
    // The kept file, or a missing folder, is not a failure.
    if (err != ERROR_FILE_NOT_FOUND && err != ERROR_PATH_NOT_FOUND && err != ERROR_DIR_NOT_EMPTY && !firstError)
      firstError = err;
    if (err == ERROR_DIR_NOT_EMPTY && keep.empty() && !firstError) firstError = err;
  }
  return firstError;
}

// Whether a process is running from inside `dir` (Redrob Launcher itself).
static bool AnythingRunningFrom(const std::wstring& dir) {
  HANDLE snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
  if (snap == INVALID_HANDLE_VALUE) return false;
  const std::wstring prefix = dir + L"\\";
  const DWORD self = GetCurrentProcessId();
  bool found = false;
  PROCESSENTRY32W pe = {sizeof(pe)};
  for (BOOL ok = Process32FirstW(snap, &pe); ok && !found; ok = Process32NextW(snap, &pe)) {
    if (pe.th32ProcessID == self) continue;
    HANDLE h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pe.th32ProcessID);
    if (!h) continue;
    wchar_t path[MAX_PATH];
    DWORD len = MAX_PATH;
    if (QueryFullProcessImageNameW(h, 0, path, &len) && _wcsnicmp(path, prefix.c_str(), prefix.size()) == 0)
      found = true;
    CloseHandle(h);
  }
  CloseHandle(snap);
  return found;
}

// Run a command hidden and wait. Returns its exit code, or the CreateProcess error.
static DWORD RunHidden(std::wstring cmd, const wchar_t* cwd = nullptr) {
  STARTUPINFOW si = {sizeof(si)};
  si.dwFlags = STARTF_USESHOWWINDOW;
  si.wShowWindow = SW_HIDE;
  PROCESS_INFORMATION pi = {};
  if (!CreateProcessW(nullptr, &cmd[0], nullptr, nullptr, FALSE, CREATE_NO_WINDOW, nullptr, cwd, &si, &pi))
    return GetLastError();
  WaitForSingleObject(pi.hProcess, INFINITE);
  DWORD exitCode = 1;
  GetExitCodeProcess(pi.hProcess, &exitCode);
  CloseHandle(pi.hThread);
  CloseHandle(pi.hProcess);
  return exitCode;
}

// A shortcut to the launcher. It carries the AppUserModelID ai.redrob.launcher: Windows needs it to show
// toast notifications from an unpackaged app and to group the taskbar button (the MSI's ShortcutProperty
// for the same thing failed with "Warning 1946" on some machines; IPropertyStore is the direct way).
static bool MakeShortcut(const std::wstring& lnk, const std::wstring& target, const std::wstring& workDir) {
  ComPtr<IShellLinkW> link;
  if (FAILED(CoCreateInstance(CLSID_ShellLink, nullptr, CLSCTX_INPROC_SERVER, IID_PPV_ARGS(&link)))) return false;
  link->SetPath(target.c_str());
  link->SetWorkingDirectory(workDir.c_str());
  link->SetIconLocation(target.c_str(), 0);
  link->SetDescription(Tr(L"Install and update the Redrob apps", L"Redrob 앱 설치 및 업데이트"));
  ComPtr<IPropertyStore> store;
  if (SUCCEEDED(link->QueryInterface(IID_PPV_ARGS(&store)))) {
    PROPVARIANT pv;
    if (SUCCEEDED(InitPropVariantFromString(kAppUserModelId, &pv))) {
      store->SetValue(PKEY_AppUserModel_ID, pv);
      store->Commit();
      PropVariantClear(&pv);
    }
  }
  ComPtr<IPersistFile> file;
  if (FAILED(link->QueryInterface(IID_PPV_ARGS(&file)))) return false;
  return SUCCEEDED(file->Save(lnk.c_str(), TRUE));
}

static bool SetArpString(HKEY key, const wchar_t* name, const std::wstring& value) {
  return RegSetValueExW(key, name, 0, REG_SZ, reinterpret_cast<const BYTE*>(value.c_str()),
                        (DWORD)((value.size() + 1) * sizeof(wchar_t))) == ERROR_SUCCESS;
}

static ULONGLONG TreeSizeBytes(const std::wstring& dir) {
  ULONGLONG total = 0;
  WIN32_FIND_DATAW fd;
  HANDLE find = FindFirstFileW((dir + L"\\*").c_str(), &fd);
  if (find == INVALID_HANDLE_VALUE) return 0;
  do {
    const std::wstring name = fd.cFileName;
    if (name == L"." || name == L"..") continue;
    if (fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
      if (!(fd.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT)) total += TreeSizeBytes(dir + L"\\" + name);
    } else {
      total += ((ULONGLONG)fd.nFileSizeHigh << 32) | fd.nFileSizeLow;
    }
  } while (FindNextFileW(find, &fd));
  FindClose(find);
  return total;
}

// The Settings > Apps entry. Everything Windows shows there: name, version, publisher, icon, size.
static bool WriteArpEntry(const std::wstring& dir) {
  HKEY key = nullptr;
  if (RegCreateKeyExW(HKEY_CURRENT_USER, kArpKey, 0, nullptr, 0, KEY_WRITE, nullptr, &key, nullptr) != ERROR_SUCCESS)
    return false;
  const std::wstring exe = dir + kAppExeRel;
  const std::wstring uninstaller = dir + kUninstallerName;
  bool ok = SetArpString(key, L"DisplayName", L"Redrob Launcher") &&
            SetArpString(key, L"DisplayVersion", REDROB_VER_WSTR) &&
            SetArpString(key, L"Publisher", L"Redrob") &&
            SetArpString(key, L"DisplayIcon", L"\"" + exe + L"\",0") &&
            SetArpString(key, L"InstallLocation", dir) &&
            SetArpString(key, L"URLInfoAbout", L"https://redrob.ai") &&
            SetArpString(key, L"HelpLink", L"https://github.com/redrob-labs/redrob-launcher") &&
            SetArpString(key, L"URLUpdateInfo", L"https://github.com/redrob-labs/redrob-launcher/releases") &&
            SetArpString(key, L"UninstallString", L"\"" + uninstaller + L"\" --uninstall");
  const DWORD one = 1;
  const DWORD sizeKb = (DWORD)std::min<ULONGLONG>(TreeSizeBytes(dir) / 1024, 0xFFFFFFFFull);
  RegSetValueExW(key, L"NoModify", 0, REG_DWORD, reinterpret_cast<const BYTE*>(&one), sizeof(one));
  RegSetValueExW(key, L"NoRepair", 0, REG_DWORD, reinterpret_cast<const BYTE*>(&one), sizeof(one));
  RegSetValueExW(key, L"EstimatedSize", 0, REG_DWORD, reinterpret_cast<const BYTE*>(&sizeKb), sizeof(sizeKb));
  RegCloseKey(key);
  return ok;
}

// The payload-less build of this app (IDR_UNINSTALLER) goes into the install folder; Settings > Apps runs it.
bool SetupApp::InstallUninstaller(const std::wstring& dir) {
  return WriteResource(hinst_, IDR_UNINSTALLER, dir + kUninstallerName);
}

static std::wstring StartMenuShortcut() {
  const std::wstring programs = KnownFolder(FOLDERID_Programs);
  return programs.empty() ? L"" : programs + kShortcutName;
}

static std::wstring DesktopShortcut() {
  const std::wstring desktop = KnownFolder(FOLDERID_Desktop);
  return desktop.empty() ? L"" : desktop + kShortcutName;
}

// HKCU\Software\Classes\redrob: "URL Protocol", and shell\open\command starts the launcher with the
// link as its one argument. Per-user, so no admin rights. Must match url_scheme.rs.
static void RegisterUrlScheme(const std::wstring& exe) {
  HKEY key = nullptr;
  if (RegCreateKeyExW(HKEY_CURRENT_USER, kUrlSchemeKey, 0, nullptr, 0, KEY_WRITE, nullptr, &key, nullptr) == ERROR_SUCCESS) {
    SetArpString(key, nullptr, L"URL:Redrob Launcher");
    SetArpString(key, L"URL Protocol", L"");
    RegCloseKey(key);
  }
  const std::wstring icon = std::wstring(kUrlSchemeKey) + L"\\DefaultIcon";
  if (RegCreateKeyExW(HKEY_CURRENT_USER, icon.c_str(), 0, nullptr, 0, KEY_WRITE, nullptr, &key, nullptr) == ERROR_SUCCESS) {
    SetArpString(key, nullptr, L"\"" + exe + L"\",0");
    RegCloseKey(key);
  }
  const std::wstring command = std::wstring(kUrlSchemeKey) + L"\\shell\\open\\command";
  if (RegCreateKeyExW(HKEY_CURRENT_USER, command.c_str(), 0, nullptr, 0, KEY_WRITE, nullptr, &key, nullptr) == ERROR_SUCCESS) {
    SetArpString(key, nullptr, L"\"" + exe + L"\" \"%1\"");
    RegCloseKey(key);
  }
}

// What the launcher reads back (self_update.rs, language.rs) and the next setup preselects: the install
// folder, the language, the desktop-shortcut choice; and the App Paths entry for Win+R "redrob-launcher".
void SetupApp::RecordChoices(const std::wstring& dir) {
  HKEY key = nullptr;
  if (RegCreateKeyExW(HKEY_CURRENT_USER, kLauncherKey, 0, nullptr, 0, KEY_WRITE, nullptr, &key, nullptr) == ERROR_SUCCESS) {
    SetArpString(key, L"InstallDir", dir);
    SetArpString(key, L"Language", g_lang == Lang::Ko ? L"ko" : L"en");
    SetArpString(key, L"DesktopShortcutChoice", desktopShortcut_ ? L"1" : L"0");
    RegCloseKey(key);
  }
  if (RegCreateKeyExW(HKEY_CURRENT_USER, kAppPathsKey, 0, nullptr, 0, KEY_WRITE, nullptr, &key, nullptr) == ERROR_SUCCESS) {
    SetArpString(key, nullptr, dir + kAppExeRel);
    RegCloseKey(key);
  }
  RegisterUrlScheme(dir + kAppExeRel);
}

// The install proper. Unpack beside the old install, then swap, so neither a failed unpack nor a locked
// file leaves a half-updated folder behind.
DWORD SetupApp::InstallLauncher() {
  const std::wstring dir = InstallDir();
  if (dir.empty()) return ERROR_PATH_NOT_FOUND;
  if (AnythingRunningFrom(dir)) return kResultAppRunning;

  std::wstring zip;
  if (!ExtractPayload(zip)) return ERROR_RESOURCE_DATA_NOT_FOUND;

  const std::wstring parent = LocalAppData() + L"\\Programs";
  CreateDirectoryW(parent.c_str(), nullptr);
  const std::wstring staging = dir + L".new";
  const std::wstring aside = dir + L".old";
  DeleteTree(staging);  // leftovers of an interrupted earlier run
  DeleteTree(aside);
  if (!CreateDirectoryW(staging.c_str(), nullptr)) { DeleteFileW(zip.c_str()); return GetLastError(); }

  // System32's tar is bsdtar, which reads zip. Its full path, so a tar.exe elsewhere on PATH is never used.
  wchar_t sys[MAX_PATH];
  GetSystemDirectoryW(sys, MAX_PATH);
  const DWORD tarExit = RunHidden(L"\"" + std::wstring(sys) + L"\\tar.exe\" -x -f \"" + zip + L"\" -C \"" +
                                  staging + L"\"");
  DeleteFileW(zip.c_str());
  if (tarExit != 0) { DeleteTree(staging); return kResultUnpackFailed; }
  // The portable zip wraps its files in one folder (redrob-launcher-<version>-windows-<arch>-portable\).
  // Lift that folder's contents up so the exe sits in the install folder itself.
  if (!Exists(staging + kAppExeRel)) {
    const std::wstring inner = SingleSubfolderWith(staging, kAppExeRel);
    if (inner.empty()) { DeleteTree(staging); return kResultNoExe; }
    const std::wstring lifted = dir + L".lift";
    DeleteTree(lifted);
    if (!MoveFileExW(inner.c_str(), lifted.c_str(), 0)) { const DWORD err = GetLastError(); DeleteTree(staging); return err; }
    DeleteTree(staging);
    if (!MoveFileExW(lifted.c_str(), staging.c_str(), 0)) { const DWORD err = GetLastError(); DeleteTree(lifted); return err; }
  }
  // The zip is the portable build; the marker beside the exe is what makes the launcher run portable
  // (settings next to itself, no self-update). An installed copy must not have it.
  DeleteFileW((staging + kPortableMarker).c_str());

  // Swap. The rename fails, rather than half-copying, if anything still holds a file in the old folder.
  if (Exists(dir)) {
    if (!MoveFileExW(dir.c_str(), aside.c_str(), 0)) {
      const DWORD err = GetLastError();
      DeleteTree(staging);
      return err == ERROR_ACCESS_DENIED || err == ERROR_SHARING_VIOLATION ? kResultAppRunning : err;
    }
  }
  if (!MoveFileExW(staging.c_str(), dir.c_str(), 0)) {
    const DWORD err = GetLastError();
    if (Exists(aside)) MoveFileExW(aside.c_str(), dir.c_str(), 0);  // put the old install back
    DeleteTree(staging);
    return err;
  }
  DeleteTree(aside);

  // The rest is best effort: the launcher is installed and runs without any of it.
  InstallUninstaller(dir);
  const std::wstring exe = dir + kAppExeRel;
  const std::wstring start = StartMenuShortcut();
  if (!start.empty()) MakeShortcut(start, exe, dir);
  const std::wstring desktop = DesktopShortcut();
  if (!desktop.empty()) {
    if (desktopShortcut_) MakeShortcut(desktop, exe, dir);
    else DeleteFileW(desktop.c_str());  // an update where the box was unticked removes the old one
  }
  RecordChoices(dir);
  WriteArpEntry(dir);
  SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, nullptr, nullptr);  // refresh shortcut icons

  if (!silent_) ShellExecuteW(nullptr, L"open", exe.c_str(), nullptr, dir.c_str(), SW_SHOWNORMAL);
  return 0;
}

DWORD WINAPI SetupApp::UninstallThunk(LPVOID self) {
  return reinterpret_cast<SetupApp*>(self)->RunUninstall();
}

void SetupApp::StartUninstall() {
  progress_ = progressTarget_ = 0.0;
  startTick_ = GetTickCount64();
  SetStage(Stage::Removing);
  SetTimer(hwnd_, kTickTimer, kTickMs, nullptr);
  if (payloadThread_) { WaitForSingleObject(payloadThread_, 5000); CloseHandle(payloadThread_); }
  payloadThread_ = CreateThread(nullptr, 0, &SetupApp::UninstallThunk, this, 0, nullptr);
}

// Remove what InstallLauncher made. The uninstaller itself is still running from the install folder, so it
// is kept here and removed by ScheduleSelfCleanup once this window closes.
DWORD SetupApp::RunUninstall() {
  if (demo_) {
    Sleep(3000);
    PostMessage(hwnd_, kUninstallMsgDone, 0, 0);
    return 0;
  }
  const std::wstring dir = InstallDir();
  if (dir.empty()) { PostMessage(hwnd_, kUninstallMsgDone, (WPARAM)ERROR_PATH_NOT_FOUND, 0); return 1; }
  if (AnythingRunningFrom(dir)) { PostMessage(hwnd_, kUninstallMsgDone, (WPARAM)kResultAppRunning, 0); return 1; }

  const std::wstring start = StartMenuShortcut();
  if (!start.empty()) DeleteFileW(start.c_str());
  const std::wstring desktop = DesktopShortcut();
  if (!desktop.empty()) DeleteFileW(desktop.c_str());
  RegDeleteTreeW(HKEY_CURRENT_USER, kArpKey);
  RegDeleteTreeW(HKEY_CURRENT_USER, kAppPathsKey);
  RegDeleteTreeW(HKEY_CURRENT_USER, kUrlSchemeKey);
  // The folder is gone, so the launcher's record of it goes too; the language and the shortcut choice
  // stay so the next install preselects them (and the launcher's shared language file is untouched).
  HKEY key = nullptr;
  if (RegOpenKeyExW(HKEY_CURRENT_USER, kLauncherKey, 0, KEY_WRITE, &key) == ERROR_SUCCESS) {
    RegDeleteValueW(key, L"InstallDir");
    RegCloseKey(key);
  }

  wchar_t self[MAX_PATH];
  GetModuleFileNameW(nullptr, self, MAX_PATH);
  DWORD err = Exists(dir) ? DeleteTree(dir, self) : 0;

  if (deleteData_) {
    // Where the launcher keeps its state on Windows: %APPDATA%\ai.redrob.launcher (its settings,
    // eframe's storage dir for the app id) and %LOCALAPPDATA%\Redrob Launcher (downloads, release
    // cache, the engine). Its registry key goes too. %APPDATA%\Redrob\Shared (the sign-in and the
    // language every Redrob app reads) and the shared models folder belong to the other apps and stay.
    const std::wstring roaming = KnownFolder(FOLDERID_RoamingAppData);
    if (!roaming.empty()) DeleteTree(roaming + L"\\ai.redrob.launcher");
    const std::wstring local = LocalAppData();
    if (!local.empty()) DeleteTree(local + L"\\Redrob Launcher");
    RegDeleteTreeW(HKEY_CURRENT_USER, kLauncherKey);
  }
  SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, nullptr, nullptr);
  PostMessage(hwnd_, kUninstallMsgDone, (WPARAM)err, 0);
  return err;
}

// Once removed, this exe is the last thing left. A running exe cannot delete itself, so a hidden
// PowerShell waits for this process to exit, then removes it and the install folder. Only when this copy
// runs from the install folder: a copy started anywhere else is never deleted.
void SetupApp::ScheduleSelfCleanup() {
  wchar_t self[MAX_PATH];
  if (!GetModuleFileNameW(nullptr, self, MAX_PATH)) return;
  const std::wstring dir = InstallDir();
  const std::wstring me = self;
  if (dir.size() < 4 || _wcsnicmp(me.c_str(), (dir + L"\\").c_str(), dir.size() + 1) != 0) return;
  std::wstring cmd = L"powershell.exe -NoProfile -NonInteractive -WindowStyle Hidden -Command \"Wait-Process -Id " +
                     std::to_wstring(GetCurrentProcessId()) +
                     L" -ErrorAction SilentlyContinue; Remove-Item -LiteralPath '" + me +
                     L"' -Force -ErrorAction SilentlyContinue; Remove-Item -LiteralPath '" + dir +
                     L"' -Recurse -Force -ErrorAction SilentlyContinue\"";
  STARTUPINFOW si = {sizeof(si)};
  si.dwFlags = STARTF_USESHOWWINDOW;
  si.wShowWindow = SW_HIDE;
  PROCESS_INFORMATION pi = {};
  // From the parent folder: a process whose working directory is the install folder would hold it open.
  const std::wstring parent = LocalAppData() + L"\\Programs";
  if (CreateProcessW(nullptr, &cmd[0], nullptr, nullptr, FALSE, CREATE_NO_WINDOW, nullptr, parent.c_str(), &si, &pi)) {
    CloseHandle(pi.hThread);
    CloseHandle(pi.hProcess);
  }
}

int SetupApp::RunSilent(HINSTANCE hinst, bool deleteData) {
  hinst_ = hinst;
  silent_ = true;
  deleteData_ = deleteData;
  CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);  // the shortcut writer is COM
  DWORD result;
  if (mode_ == Mode::Uninstall) {
    result = RunUninstall();  // hwnd_ is null: its PostMessage lands on this thread's queue, unread
    if (result == 0) ScheduleSelfCleanup();
  } else {
    // Silent updates preserve the interactive choice instead of recreating a declined desktop shortcut.
    const int previous = RecordedDesktopShortcut();
    if (previous >= 0) desktopShortcut_ = previous == 1;
    result = InstallLauncher();
  }
  CoUninitialize();
  return (int)result;
}

DWORD SetupApp::RunPayload() {
  // This is a worker thread: COM is per thread, and without it the shortcut writer's CoCreateInstance
  // fails, so the install finished with no Start menu entry and Windows search never found the launcher.
  const HRESULT com = CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED);
  PostMessage(hwnd_, kStageMsg, (WPARAM)Stage::Installing, 0);
  const DWORD result = InstallLauncher();
  if (SUCCEEDED(com)) CoUninitialize();
  PostMessage(hwnd_, kPayloadMsgDone, (WPARAM)result, 0);
  return result;
}

}  // namespace

// ---- entry point -----------------------------------------------------------

int WINAPI wWinMain(HINSTANCE hinst, HINSTANCE, PWSTR, int) {
  bool demo = false, demoError = false;
  bool silent = false, deleteData = false, explicitInstall = false;
  std::wstring renderDir, lang;
  Mode mode = Mode::Install;
  int argc = 0;
  LPWSTR* argv = CommandLineToArgvW(GetCommandLineW(), &argc);
  for (int i = 1; i < argc; ++i) {
    if (lstrcmpiW(argv[i], L"--demo") == 0) { demo = true; explicitInstall = true; }
    else if (lstrcmpiW(argv[i], L"--demo-error") == 0) { demo = true; demoError = true; explicitInstall = true; }
    else if (lstrcmpiW(argv[i], L"--uninstall") == 0) mode = Mode::Uninstall;
    else if (lstrcmpiW(argv[i], L"--demo-uninstall") == 0) { demo = true; mode = Mode::Uninstall; }
    else if (lstrcmpiW(argv[i], L"--silent") == 0) silent = true;
    else if (lstrcmpiW(argv[i], L"--delete-data") == 0) deleteData = true;
    else if (lstrcmpiW(argv[i], L"--lang") == 0 && i + 1 < argc) lang = argv[++i];
    else if (lstrcmpiW(argv[i], L"--render-stages") == 0 && i + 1 < argc) renderDir = argv[++i];
    else if (lstrcmpiW(argv[i], L"--theme") == 0 && i + 1 < argc) {
      ++i;
      g_themeFlag = lstrcmpiW(argv[i], L"light") == 0 ? 1 : lstrcmpiW(argv[i], L"dark") == 0 ? 2 : 0;
    }
  }
  if (argv) LocalFree(argv);
  g_lang = DefaultLanguage(lang);

#ifdef REDROB_NO_PAYLOAD
  // The uninstaller carries nothing to install, so opened by a double-click it offers removal, not an
  // Install button that could only fail.
  if (!explicitInstall) mode = Mode::Uninstall;
#endif

  SetupApp app(demo, demoError, mode);
  if (!renderDir.empty()) return app.RenderStages(hinst, renderDir);
  if (silent && !demo) return app.RunSilent(hinst, deleteData);
  return app.Run(hinst);
}
