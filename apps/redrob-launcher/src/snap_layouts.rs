// ADDED-LINE-RATIONALE 5748cd9fbac70e72:1: Supports `explicitly` while preserving the nearby constraint: ! window, from its own UI thread.
// ADDED-LINE-RATIONALE e4a0eaaf9c4de36d:1: Supports `explicitly` while preserving the nearby constraint: ![allow(unsafe_code)].
// ADDED-LINE-RATIONALE c118fc0412c0be0d:1: Gives `explicitly` direct access to `std / sync / Mutex` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE d8f4b8e1faedadce:1: Gives `explicitly` direct access to `redrob_launcher_ui_egui / caption / OsState` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 7a785d70b2b06f0c:1: Gives `explicitly` direct access to `windows_sys / Win32 / Foundation` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 36bf407946b9f873:1: Gives `explicitly` direct access to `windows_sys / Win32 / Graphics` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 4e7c7cd2da77d832:1: Gives `explicitly` direct access to `windows_sys / Win32 / UI` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 493386d5684847a6:1: Gives `explicitly` direct access to `windows_sys / Win32 / UI` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 145750fbb6780ec5:1: Keeps `CallWindowProcW / EnableMenuItem / GWL_STYLE` in `explicitly`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 5ac6740c8f5f6b4d:1: Keeps `MF_ENABLED / MF_GRAYED / SC_CLOSE` in `explicitly`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE a7ac3c1b47665097:1: Keeps `SWP_NOACTIVATE / SWP_NOMOVE / SWP_NOSIZE` in `explicitly`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 166ba21ddae6a1a1:1: Keeps `TPM_RIGHTBUTTON / TrackPopupMenu / WM_MOUSEMOVE` in `explicitly`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 7432624305ed8b46:1: Keeps `WM_NCMOUSEMOVE / WM_STYLECHANGING / WM_SYSCOMMAND` in `explicitly`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 93d5aef770d8532c:1: Bounds the `explicitly` implementation here so `WM_NCMOUSEMOVE / WM_STYLECHANGING / WM_SYSCOMMAND` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b4d25fec240b5159:1: Declares the `cfg / target_pointer_width` contract on `explicitly` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 7014c9609a3ef028:1: Supports `explicitly` while preserving the nearby constraint: [cfg(target_pointer_width = "64")].
// ADDED-LINE-RATIONALE ad74e6273761a46c:1: Declares the `cfg / target_pointer_width` contract on `explicitly` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 2e136e79aeb6907a:1: Supports `explicitly` while preserving the nearby constraint: [cfg(target_pointer_width = "32")].
// ADDED-LINE-RATIONALE 4901df2ed59e08cb:1: Supports `explicitly` while preserving the nearby constraint: the bridge lossless on both 32- and 64-bit Windows. Zero means no window.
// ADDED-LINE-RATIONALE 766a59d855f9d592:1: Supports `explicitly` while preserving the nearby constraint: The window's own procedure (winit's), called for everything this one doesn't take.
// ADDED-LINE-RATIONALE 93fe6370cbcf23c5:1: Supports `explicitly` while preserving the nearby constraint: Whether a `WM_NCMOUSELEAVE` request is outstanding (asked once per stay, as WezTerm does).
// ADDED-LINE-RATIONALE 33dab4bf21b9043a:1: Supports `explicitly` while preserving the nearby constraint: Repaints the window when the OS's report on Maximize changes.
// ADDED-LINE-RATIONALE b4d25fec240b5159:2: Supports `explicitly` while preserving the nearby constraint: where the `W` functions carry the same 32-bit pointer bits.
// ADDED-LINE-RATIONALE af155cc4753ff2a9:1: Supports `get_window_long` while preserving the nearby constraint: [cfg(target_pointer_width = "64")].
// ADDED-LINE-RATIONALE 5a2368cfd82338e1:1: Supports `get_window_long` while preserving the nearby constraint: SAFETY: the caller supplies a live window and a valid GWL/GWLP index.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:1: Bounds the `get_window_long` implementation here so `unsafe / GetWindowLongPtrW / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ad74e6273761a46c:2: Declares the `cfg / target_pointer_width` contract on `get_window_long` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE af155cc4753ff2a9:2: Supports `get_window_long` while preserving the nearby constraint: [cfg(target_pointer_width = "32")].
// ADDED-LINE-RATIONALE 0f4bb08f3d800187:1: Supports `get_window_long` while preserving the nearby constraint: same width on this target.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:2: Bounds the `get_window_long` implementation here so `unsafe / GetWindowLongW / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b4d25fec240b5159:3: Declares the `cfg / target_pointer_width` contract on `get_window_long` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE baa6fc9a76b14a3e:1: Supports `set_window_long` while preserving the nearby constraint: [cfg(target_pointer_width = "64")].
// ADDED-LINE-RATIONALE 8e0412e4d7416be0:1: Supports `set_window_long` while preserving the nearby constraint: SAFETY: the caller supplies a live window, valid GWL/GWLP index, and compatible value.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:3: Bounds the `set_window_long` implementation here so `unsafe / SetWindowLongPtrW / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ad74e6273761a46c:3: Declares the `cfg / target_pointer_width` contract on `set_window_long` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE baa6fc9a76b14a3e:2: Supports `set_window_long` while preserving the nearby constraint: [cfg(target_pointer_width = "32")].
// ADDED-LINE-RATIONALE 7ce24529e043fba2:1: Supports `set_window_long` while preserving the nearby constraint: and isize have the same width on this target, so procedure pointers are not truncated.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:4: Bounds the `set_window_long` implementation here so `unsafe / SetWindowLongW / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 966c066e1c7402d3:1: Supports `install` while preserving the nearby constraint: then.
// ADDED-LINE-RATIONALE b276a36da2b81345:1: Freezes `if` for `install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c176218758ab21ef:1: Stops `install` at the `hwnd` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 752bdb12dfa3351f:1: Bounds the `install` implementation here so `WINDOW / load / Ordering` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE af804264b9351c71:1: Freezes `if` for `install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 72a673a3fea2a8f8:1: Delegates `install`'s `CONTEXT / lock / unwrap_or_else` work to `CONTEXT.lock` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c176218758ab21ef:2: Supports `install` while preserving the nearby constraint: CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = Some(ctx);.
// ADDED-LINE-RATIONALE 752bdb12dfa3351f:2: Bounds the `install` implementation here so `CONTEXT / lock / unwrap_or_else` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c04b4c3e425c888d:1: Freezes `window` for `install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 86053f7a3755a0ff:1: Freezes `procedure` for `install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2a39f875a4f17096:1: Supports `install` while preserving the nearby constraint: this call and storing the returned procedure below.
// ADDED-LINE-RATIONALE f8da8e9f17965e33:1: Freezes `if` for `install` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b3711603c2d5718b:1: Isolates `the` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE c176218758ab21ef:3: Stops `the` at the `log / warn / Snap` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 752bdb12dfa3351f:3: Bounds the `the` implementation here so `log / warn / Snap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1ae557dcb2e88b52:1: Delegates `the`'s `PREVIOUS / store / previous` work to `PREVIOUS.store` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 82d1063db76231d5:1: Delegates `the`'s `WINDOW / store / hwnd` work to `WINDOW.store` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 6338f8741c7a37f8:1: Delegates `the`'s `CONTEXT / lock / unwrap_or_else` work to `CONTEXT.lock` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9d1c8f33b2d74c07:1: Supports `the` while preserving the nearby constraint: CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = Some(ctx);.
// ADDED-LINE-RATIONALE f6f0e03322e64dc8:1: Delegates `the`'s `TRACKING / store / Ordering` work to `TRACKING.store` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 984cbfc151d3e58b:1: Supports `the` while preserving the nearby constraint: style without moving, sizing, reordering, or activating it. Alt+Space is replaced below.
// ADDED-LINE-RATIONALE 7035d88213a18923:1: Freezes `style` for `the` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE dc7e2c5c7b87cd7e:1: Delegates `the`'s `set_window_long / window / GWL_STYLE` work to `set_window_long` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ee6546e3b8a55927:1: Delegates `the`'s `SetWindowPos / window / std` work to `SetWindowPos` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 752bdb12dfa3351f:4: Bounds the `the` implementation here so `SetWindowPos / window / std` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:5: Bounds the `the` implementation here so `SetWindowPos / window / std` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0f666e50b35b990e:1: Supports `report` while preserving the nearby constraint: Reports a new state for Maximize and repaints if it changed.
// ADDED-LINE-RATIONALE 54b571da7eb75360:1: Prevents `report` from applying its side effect outside the `caption / set_os_state / state` state.
// ADDED-LINE-RATIONALE 74c81b672633b197:1: Delegates `report`'s `ctx / CONTEXT / lock` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 752bd912dfa331b9:1: Bounds the `report` implementation here so `ctx / CONTEXT / lock` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a65053439ce193b6:1: Delegates `report`'s `ctx / request_repaint` work to `ctx.request_repaint` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 752bdb12dfa3351f:5: Bounds the `report` implementation here so `ctx / request_repaint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:6: Bounds the `report` implementation here so `ctx / request_repaint` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c4261f4190c72c6c:1: Supports `client_point` while preserving the nearby constraint: The screen point in a mouse message's `lparam`, in `hwnd`'s client pixels.
// ADDED-LINE-RATIONALE 782ff6d29136b92c:1: Supports `client_point` while preserving the nearby constraint: GET_X_LPARAM / GET_Y_LPARAM: signed, so a second monitor left of or above the first works.
// ADDED-LINE-RATIONALE f0a0d02a308c5f23:1: Supports `client_point` while preserving the nearby constraint: SAFETY: `hwnd` is the window whose procedure is running, and `p` is a valid POINT.
// ADDED-LINE-RATIONALE 4275f8ca173b2810:1: Makes `p / x / as` an explicit part of `client_point` because the desktop shell integrates refreshed UI behavior without changing platform contracts.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:7: Bounds the `client_point` implementation here so `p / x / as` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0a569aec2decf3c0:1: Supports `set_system_command_enabled` while preserving the nearby constraint: Enables or grays one standard command in the window-owned system menu.
// ADDED-LINE-RATIONALE d345105c509bd4f1:1: Supports `set_system_command_enabled` while preserving the nearby constraint: SAFETY: `menu` is the live menu returned by GetSystemMenu; these are standard command IDs.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:8: Bounds the `set_system_command_enabled` implementation here so `unsafe / EnableMenuItem / menu` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0da7472c519dedcf:1: Supports `show_system_menu` while preserving the nearby constraint: Shows the standard Alt+Space menu that `WS_SYSMENU` would normally provide.
// ADDED-LINE-RATIONALE 984cbfc151d3e58b:2: Supports `show_system_menu` while preserving the nearby constraint: returned menu remains owned by the window, and TPM_RETURNCMD keeps dispatch under our control.
// ADDED-LINE-RATIONALE 42e2c6867bd5293c:1: Freezes `menu` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 750317829a766bfe:1: Prevents `show_system_menu` from applying its side effect outside the `menu / is_null` state.
// ADDED-LINE-RATIONALE bf66c8adb61995a7:1: Delegates `show_system_menu`'s `log / warn / Snap` work to `log::warn` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 36c2f8579d0accef:1: Stops `show_system_menu` at the `log / warn / Snap` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e5820d04bed5881f:1: Bounds the `show_system_menu` implementation here so `log / warn / Snap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6c87c76874969692:1: Freezes `style` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 012591371f7caaea:1: Freezes `minimized` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c85850b2347258eb:1: Freezes `maximized` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d7ffee808fffd3d8:1: Freezes `normal` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2ee779c679296476:1: Supports `show_system_menu` while preserving the nearby constraint: caption-box styles still describe which operations winit allows after WS_SYSMENU is off.
// ADDED-LINE-RATIONALE a33df44c6f958ddc:1: Delegates `show_system_menu`'s `set_system_command_enabled / menu / SC_MOVE` work to `set_system_command_enabled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8bd6a6e36a33c476:1: Freezes `set_system_command_enabled` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a2b1dfc564a6654e:1: Freezes `set_system_command_enabled` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c73e06902d887e4c:1: Freezes `set_system_command_enabled` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7fd5c4d09b2d9c72:1: Delegates `show_system_menu`'s `set_system_command_enabled / menu / SC_CLOSE` work to `set_system_command_enabled` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9ecac762eb568d48:1: Freezes `bounds` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d07390a86932c993:1: Freezes `if` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9a5b8b93e12b34b6:1: Delegates `show_system_menu`'s `log / warn / Snap` work to `log::warn` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 36c2f8579d0accef:2: Stops `show_system_menu` at the `log / warn / Snap` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e5820d04bed5881f:2: Bounds the `show_system_menu` implementation here so `log / warn / Snap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 97840d3f6de74350:1: Supports `show_system_menu` while preserving the nearby constraint: TrackPopupMenu requires its owner to be foreground for normal keyboard/click-away behavior.
// ADDED-LINE-RATIONALE 5767612a4599fac4:1: Freezes `command` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 050d994674d831e1:1: Freezes `if` for `show_system_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 49aeb7cb68af3409:1: Supports `show_system_menu` while preserving the nearby constraint: dispatching WM_SYSCOMMAND preserves the window procedure's normal command handling.
// ADDED-LINE-RATIONALE e5820d04bed5881f:3: Bounds the `show_system_menu` implementation here so `SendMessageW / hwnd / WM_SYSCOMMAND` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 752bdb12dfa3351f:6: Bounds the `show_system_menu` implementation here so `SendMessageW / hwnd / WM_SYSCOMMAND` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:9: Bounds the `show_system_menu` implementation here so `The / launcher / window` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 13b7b444aacfef53:1: Supports `procedure` while preserving the nearby constraint: The launcher window's procedure while subclassed.
// ADDED-LINE-RATIONALE cf9c34179093b3cd:1: Freezes `on_max` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a1b2e5edc654809d:1: Prevents `procedure` from applying its side effect outside the `msg` state.
// ADDED-LINE-RATIONALE f50855e5784bf963:1: Freezes `WM_NCHITTEST` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1cce0182c1873aea:1: Freezes `let` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 919e892cd7d71b7c:1: Prevents `procedure` from applying its side effect outside the `caption / on_maximize / x` state.
// ADDED-LINE-RATIONALE 4e80add38ebb2176:1: Stops `procedure` at the `HTMAXBUTTON / as / LRESULT` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 4266ac3a4e4a8b1f:1: Bounds the `procedure` implementation here so `HTMAXBUTTON / as / LRESULT` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e5820d04bed5881f:4: Bounds the `procedure` implementation here so `winit / recomputes / the` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b5c4107f45843c09:1: Supports `procedure` while preserving the nearby constraint: WS_SYSMENU off so the system caption buttons never come back.
// ADDED-LINE-RATIONALE 2294aec7364d403a:1: Supports `procedure` while preserving the nearby constraint: sender owns for the duration of this call.
// ADDED-LINE-RATIONALE e5820d04bed5881f:5: Bounds the `procedure` implementation here so `unsafe / lparam / as` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c65a85a541fb3ed2:1: Freezes `WM_SYSKEYDOWN` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 6772b0bb89aef70c:1: Supports `procedure` while preserving the nearby constraint: Bit 30 marks key-repeat. Consume repeats too, but open only once per Alt+Space press.
// ADDED-LINE-RATIONALE 192361d27ddb6ff2:1: Delegates `procedure`'s `show_system_menu / hwnd` work to `show_system_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4266ac3a4e4a8b1f:2: Bounds the `procedure` implementation here so `show_system_menu / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 1093d45e3283a28f:1: Stops `procedure` at the `show_system_menu / hwnd` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e5820d04bed5881f:6: Bounds the `procedure` implementation here so `WM_NCMOUSEMOVE` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ef0f0b2cdb4d322a:1: Freezes `WM_NCMOUSEMOVE` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ec054a6b8abe35ed:1: Supports `procedure` while preserving the nearby constraint: Ask once for WM_NCMOUSELEAVE (until it arrives), as WezTerm does.
// ADDED-LINE-RATIONALE db660224b6f9c560:1: Freezes `track` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 220e4afecec1efae:1: Keeps `cbSize / size_of / TRACKMOUSEEVENT` in `procedure`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 53bdee17564c8d5f:1: Keeps `dwFlags / TME_LEAVE / TME_NONCLIENT` in `procedure`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE e946c1765a2f14ed:1: Keeps `hwndTrack / hwnd` in `procedure`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 7217f10e93e54efe:1: Keeps `dwHoverTime` in `procedure`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 52c66fc2e328972c:1: Bounds the `procedure` implementation here so `dwHoverTime` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0802923175ce6729:1: Supports `procedure` while preserving the nearby constraint: SAFETY: `track` is a valid TRACKMOUSEEVENT for this window.
// ADDED-LINE-RATIONALE 4266ac3a4e4a8b1f:3: Bounds the `procedure` implementation here so `unsafe / TrackMouseEvent / track` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a297385b4b276282:1: Freezes `pressed` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a35faa249a4ae2b2:1: Delegates `procedure`'s `report / OsState / hovered` work to `report` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9889a85b54a0c0a0:1: Prevents `procedure` from applying its side effect outside the `on_max` state.
// ADDED-LINE-RATIONALE 91ad12fe6512b58f:1: Stops `procedure` at the `on_max` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 4266ac3a4e4a8b1f:4: Bounds the `procedure` implementation here so `on_max` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e5820d04bed5881f:7: Bounds the `procedure` implementation here so `WM_NCMOUSELEAVE` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE be762177f354450e:1: Freezes `WM_NCMOUSELEAVE` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 7f194820efdf63c8:1: Delegates `procedure`'s `TRACKING / store / Ordering` work to `TRACKING.store` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cc740012c518a6f9:1: Delegates `procedure`'s `report / OsState / default` work to `report` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e5820d04bed5881f:8: Bounds the `procedure` implementation here so `report / OsState / default` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f8c0d13af3bfeb48:1: Freezes `WM_MOUSEMOVE` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ffbae787d2dae503:1: Freezes `WM_NCLBUTTONDOWN` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE af72d982763b17c0:1: Delegates `procedure`'s `report / OsState / hovered` work to `report` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1093d45e3283a28f:2: Stops `procedure` at the `report / OsState / hovered` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e5820d04bed5881f:9: Bounds the `procedure` implementation here so `WM_NCLBUTTONUP / on_max` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 4b91cbcf756fdef0:1: Freezes `WM_NCLBUTTONUP` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 53ff4e7958b034b7:1: Prevents `procedure` from applying its side effect outside the `caption / os_state / pressed` state.
// ADDED-LINE-RATIONALE 1e37048c186f8b64:1: Supports `procedure` while preserving the nearby constraint: SAFETY: `hwnd` is the window whose procedure is running.
// ADDED-LINE-RATIONALE 4266ac3a4e4a8b1f:5: Bounds the `procedure` implementation here so `unsafe / ShowWindow / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 38d73c31713f14d9:1: Delegates `procedure`'s `report / OsState / hovered` work to `report` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 1093d45e3283a28f:3: Stops `procedure` at the `report / OsState / hovered` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e5820d04bed5881f:10: Bounds the `procedure` implementation here so `WM_NCDESTROY` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5cd70530697c36ae:1: Freezes `WM_NCDESTROY` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ca68b34f4e770d82:1: Freezes `previous` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a1f430a315b088e6:1: Delegates `procedure`'s `WINDOW / store / Ordering` work to `WINDOW.store` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7f194820efdf63c8:2: Delegates `procedure`'s `TRACKING / store / Ordering` work to `TRACKING.store` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d2e30bb289642207:1: Delegates `procedure`'s `caption / reset` work to `caption::reset` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7a203dc91a940cbc:1: Delegates `procedure`'s `CONTEXT / lock / unwrap_or_else` work to `CONTEXT.lock` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE be6ef64bae18630f:1: Supports `procedure` while preserving the nearby constraint: CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = None;.
// ADDED-LINE-RATIONALE 8c8b37e9900864e8:1: Supports `procedure` while preserving the nearby constraint: SAFETY: puts back the window's own procedure before it is destroyed.
// ADDED-LINE-RATIONALE 4266ac3a4e4a8b1f:6: Bounds the `procedure` implementation here so `unsafe / set_window_long / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE ab6b7eecbb3e99b2:1: Stops `procedure` at the `call_previous / previous / hwnd` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE e5820d04bed5881f:11: Bounds the `procedure` implementation here so `call_previous / previous / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 08b09c98323960ac:1: Freezes `_` for `procedure` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 752bdb12dfa3351f:7: Bounds the `procedure` implementation here so `_` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 93035dbb9d13bc0f:1: Delegates `procedure`'s `call_previous / PREVIOUS / load` work to `call_previous` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:10: Bounds the `procedure` implementation here so `call_previous / PREVIOUS / load` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 387e8d7f58687aac:1: Supports `call_previous` while preserving the nearby constraint: Hands a message to the window's own procedure.
// ADDED-LINE-RATIONALE 984cbfc151d3e58b:3: Supports `call_previous` while preserving the nearby constraint: this window, so it is its procedure (or 0, which CallWindowProcW treats as default handling).
// ADDED-LINE-RATIONALE 2d66141a10da814a:1: Freezes `previous` for `call_previous` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d9993b142b5b1628:1: Delegates `call_previous`'s `CallWindowProcW / previous / hwnd` work to `CallWindowProcW` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 752bdb12dfa3351f:8: Bounds the `call_previous` implementation here so `CallWindowProcW / previous / hwnd` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE c5b56a4020a0921f:11: Bounds the `call_previous` implementation here so `CallWindowProcW / previous / hwnd` cannot leak into the adjacent declaration or control path.
//! Windows 11 Snap Layouts on the drawn Maximize button.
//!
//! Windows opens Snap Layouts (the flyout of window layouts) when the pointer rests on a window's
//! maximize button. A custom-drawn button participates when the window answers `WM_NCHITTEST` with
//! `HTMAXBUTTON` over it. winit never does: its frameless windows only report the caption and the
//! resize edges. This module therefore subclasses the launcher's window and answers `HTMAXBUTTON`
//! over the place the UI last drew Maximize ([`redrob_launcher_ui_egui::caption`]). It also removes
//! `WS_SYSMENU`: otherwise Windows associates the flyout with winit's invisible standard caption
//! buttons instead of the drawn one. Because that style normally supplies Alt+Space behavior, the
//! subclass explicitly displays and dispatches the standard system menu from the keyboard.
//!
//! Over `HTMAXBUTTON` the pointer is in the frame, so egui no longer sees it there. The frame's
//! messages are relayed back instead: `WM_NCMOUSEMOVE` / `WM_NCMOUSELEAVE` give the hover, and
//! `WM_NCLBUTTONDOWN` / `WM_NCLBUTTONUP` the press and the click, which maximizes or restores the
//! window. Those messages stop here when they are about Maximize: passed on, the default handling
//! would hot-track or run the press loop of a classic caption button. Everything else goes to
//! winit untouched.
//!
//! This is the workspace's one use of `unsafe` (`AGENTS.md`): Win32 calls on the launcher's own
//! window, from its own UI thread.

#![allow(unsafe_code)]

use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::{Mutex, PoisonError};

use redrob_launcher_ui_egui::caption::{self, OsState};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{TME_LEAVE, TME_NONCLIENT, TRACKMOUSEEVENT, TrackMouseEvent, VK_SPACE};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, EnableMenuItem, GWL_STYLE, GWLP_WNDPROC, GetSystemMenu, GetWindowRect, HTMAXBUTTON, IsIconic, IsZoomed, MF_BYCOMMAND,
    MF_ENABLED, MF_GRAYED, SC_CLOSE, SC_MAXIMIZE, SC_MINIMIZE, SC_MOVE, SC_RESTORE, SC_SIZE, STYLESTRUCT, SW_MAXIMIZE, SW_RESTORE, SWP_FRAMECHANGED,
    SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SendMessageW, SetForegroundWindow, SetWindowPos, ShowWindow, TPM_RETURNCMD,
    TPM_RIGHTBUTTON, TrackPopupMenu, WM_MOUSEMOVE, WM_NCDESTROY, WM_NCHITTEST, WM_NCLBUTTONDBLCLK, WM_NCLBUTTONDOWN, WM_NCLBUTTONUP, WM_NCMOUSELEAVE,
    WM_NCMOUSEMOVE, WM_STYLECHANGING, WM_SYSCOMMAND, WM_SYSKEYDOWN, WNDPROC, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_SYSMENU, WS_THICKFRAME,
};
#[cfg(target_pointer_width = "64")]
use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW};
#[cfg(target_pointer_width = "32")]
use windows_sys::Win32::UI::WindowsAndMessaging::{GetWindowLongW, SetWindowLongW};

/// The subclassed window as a pointer-sized integer (`HWND` is not `Sync`); using `isize` keeps
/// the bridge lossless on both 32- and 64-bit Windows. Zero means no window.
static WINDOW: AtomicIsize = AtomicIsize::new(0);
/// The window's own procedure (winit's), called for everything this one doesn't take.
static PREVIOUS: AtomicIsize = AtomicIsize::new(0);
/// Whether a `WM_NCMOUSELEAVE` request is outstanding (asked once per stay, as WezTerm does).
static TRACKING: AtomicBool = AtomicBool::new(false);
/// Repaints the window when the OS's report on Maximize changes.
static CONTEXT: Mutex<Option<egui::Context>> = Mutex::new(None);

/// Pointer-width Win32 accessors. The `Ptr` symbols do not exist in windows-sys on 32-bit Windows,
/// where the `W` functions carry the same 32-bit pointer bits.
#[cfg(target_pointer_width = "64")]
unsafe fn get_window_long(hwnd: HWND, index: i32) -> isize {
    // SAFETY: the caller supplies a live window and a valid GWL/GWLP index.
    unsafe { GetWindowLongPtrW(hwnd, index) }
}

#[cfg(target_pointer_width = "32")]
unsafe fn get_window_long(hwnd: HWND, index: i32) -> isize {
    // SAFETY: the caller supplies a live window and a valid GWL/GWLP index; i32 and isize have the
    // same width on this target.
    unsafe { GetWindowLongW(hwnd, index) as isize }
}

#[cfg(target_pointer_width = "64")]
unsafe fn set_window_long(hwnd: HWND, index: i32, value: isize) -> isize {
    // SAFETY: the caller supplies a live window, valid GWL/GWLP index, and compatible value.
    unsafe { SetWindowLongPtrW(hwnd, index, value) }
}

#[cfg(target_pointer_width = "32")]
unsafe fn set_window_long(hwnd: HWND, index: i32, value: isize) -> isize {
    // SAFETY: the caller supplies a live window, valid GWL/GWLP index, and compatible value; i32
    // and isize have the same width on this target, so procedure pointers are not truncated.
    unsafe { SetWindowLongW(hwnd, index, value as i32) as isize }
}

/// Subclasses `hwnd`, the launcher's window, and suppresses its invisible system caption buttons.
/// A window opened later (after one closed to the tray) replaces the last: the old one is gone by
/// then.
pub fn install(hwnd: isize, ctx: egui::Context) {
    if hwnd == 0 {
        return;
    }
    if WINDOW.load(Ordering::Relaxed) == hwnd {
        *CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = Some(ctx);
        return;
    }

    let window = hwnd as HWND;
    let procedure: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT = procedure;
    // SAFETY: `hwnd` is the live window eframe just created on this thread, and `procedure` has
    // the window-procedure signature. This thread cannot dispatch another window message between
    // this call and storing the returned procedure below.
    let previous = unsafe { set_window_long(window, GWLP_WNDPROC, procedure as usize as isize) };
    if previous == 0 {
        log::warn!("Snap Layouts: couldn't subclass the window");
        return;
    }

    PREVIOUS.store(previous, Ordering::Relaxed);
    WINDOW.store(hwnd, Ordering::Relaxed);
    *CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = Some(ctx);
    caption::reset();
    TRACKING.store(false, Ordering::Relaxed);

    // SAFETY: style calls target the same live window. `SWP_FRAMECHANGED` makes Windows re-read the
    // style without moving, sizing, reordering, or activating it. Alt+Space is replaced below.
    unsafe {
        let style = get_window_long(window, GWL_STYLE);
        set_window_long(window, GWL_STYLE, style & !(WS_SYSMENU as isize));
        SetWindowPos(window, std::ptr::null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED);
    }
}

/// Reports a new state for Maximize and repaints if it changed.
fn report(state: OsState) {
    if caption::set_os_state(state)
        && let Some(ctx) = CONTEXT.lock().unwrap_or_else(PoisonError::into_inner).as_ref()
    {
        ctx.request_repaint();
    }
}

/// The screen point in a mouse message's `lparam`, in `hwnd`'s client pixels.
fn client_point(hwnd: HWND, lparam: LPARAM) -> (f32, f32) {
    // GET_X_LPARAM / GET_Y_LPARAM: signed, so a second monitor left of or above the first works.
    let mut p = POINT { x: i32::from((lparam & 0xFFFF) as u16 as i16), y: i32::from(((lparam >> 16) & 0xFFFF) as u16 as i16) };
    // SAFETY: `hwnd` is the window whose procedure is running, and `p` is a valid POINT.
    unsafe { ScreenToClient(hwnd, &mut p) };
    (p.x as f32, p.y as f32)
}

/// Enables or grays one standard command in the window-owned system menu.
fn set_system_command_enabled(menu: windows_sys::Win32::UI::WindowsAndMessaging::HMENU, command: u32, enabled: bool) {
    // SAFETY: `menu` is the live menu returned by GetSystemMenu; these are standard command IDs.
    unsafe { EnableMenuItem(menu, command, MF_BYCOMMAND | if enabled { MF_ENABLED } else { MF_GRAYED }) };
}

/// Shows the standard Alt+Space menu that `WS_SYSMENU` would normally provide.
fn show_system_menu(hwnd: HWND) {
    // SAFETY: every handle used here belongs to the live window whose procedure is running. The
    // returned menu remains owned by the window, and TPM_RETURNCMD keeps dispatch under our control.
    unsafe {
        let menu = GetSystemMenu(hwnd, 0);
        if menu.is_null() {
            log::warn!("Snap Layouts: couldn't get the system menu");
            return;
        }

        let style = get_window_long(hwnd, GWL_STYLE);
        let minimized = IsIconic(hwnd) != 0;
        let maximized = IsZoomed(hwnd) != 0;
        let normal = !minimized && !maximized;
        // Mirror the default system menu: window state controls Restore/Move, while the remaining
        // caption-box styles still describe which operations winit allows after WS_SYSMENU is off.
        set_system_command_enabled(menu, SC_RESTORE, minimized || maximized);
        set_system_command_enabled(menu, SC_MOVE, normal);
        set_system_command_enabled(menu, SC_SIZE, normal && style & WS_THICKFRAME as isize != 0);
        set_system_command_enabled(menu, SC_MINIMIZE, !minimized && style & WS_MINIMIZEBOX as isize != 0);
        set_system_command_enabled(menu, SC_MAXIMIZE, !maximized && style & WS_MAXIMIZEBOX as isize != 0);
        set_system_command_enabled(menu, SC_CLOSE, true);

        let mut bounds = RECT::default();
        if GetWindowRect(hwnd, &mut bounds) == 0 {
            log::warn!("Snap Layouts: couldn't position the system menu");
            return;
        }

        // TrackPopupMenu requires its owner to be foreground for normal keyboard/click-away behavior.
        SetForegroundWindow(hwnd);
        let command = TrackPopupMenu(menu, TPM_RETURNCMD | TPM_RIGHTBUTTON, bounds.left, bounds.top, 0, hwnd, std::ptr::null());
        if command != 0 {
            // SAFETY: command is one of the menu's pointer-width-safe, predefined SC_* identifiers;
            // dispatching WM_SYSCOMMAND preserves the window procedure's normal command handling.
            SendMessageW(hwnd, WM_SYSCOMMAND, command as WPARAM, 0);
        }
    }
}

/// The launcher window's procedure while subclassed.
unsafe extern "system" fn procedure(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let on_max = wparam == HTMAXBUTTON as WPARAM;
    match msg {
        WM_NCHITTEST => {
            let (x, y) = client_point(hwnd, lparam);
            if caption::on_maximize(x, y) {
                return HTMAXBUTTON as LRESULT;
            }
        }
        // winit recomputes the whole style on some state changes (maximize, resizable, ...): keep
        // WS_SYSMENU off so the system caption buttons never come back.
        WM_STYLECHANGING if wparam == GWL_STYLE as isize as WPARAM && lparam != 0 => {
            // SAFETY: for WM_STYLECHANGING with GWL_STYLE, lparam points to a STYLESTRUCT the
            // sender owns for the duration of this call.
            unsafe { (*(lparam as *mut STYLESTRUCT)).styleNew &= !WS_SYSMENU };
        }
        WM_SYSKEYDOWN if wparam == VK_SPACE as WPARAM && lparam & (1 << 29) != 0 => {
            // Bit 30 marks key-repeat. Consume repeats too, but open only once per Alt+Space press.
            if lparam & (1 << 30) == 0 {
                show_system_menu(hwnd);
            }
            return 0;
        }
        WM_NCMOUSEMOVE => {
            // Ask once for WM_NCMOUSELEAVE (until it arrives), as WezTerm does.
            if !TRACKING.swap(true, Ordering::Relaxed) {
                let mut track = TRACKMOUSEEVENT {
                    cbSize: size_of::<TRACKMOUSEEVENT>() as u32,
                    dwFlags: TME_LEAVE | TME_NONCLIENT,
                    hwndTrack: hwnd,
                    dwHoverTime: 0,
                };
                // SAFETY: `track` is a valid TRACKMOUSEEVENT for this window.
                unsafe { TrackMouseEvent(&mut track) };
            }
            let pressed = on_max && caption::os_state().pressed;
            report(OsState { hovered: on_max, pressed });
            if on_max {
                return 0;
            }
        }
        WM_NCMOUSELEAVE => {
            TRACKING.store(false, Ordering::Relaxed);
            report(OsState::default());
        }
        WM_MOUSEMOVE => report(OsState::default()),
        WM_NCLBUTTONDOWN | WM_NCLBUTTONDBLCLK if on_max => {
            report(OsState { hovered: true, pressed: true });
            return 0;
        }
        WM_NCLBUTTONUP if on_max => {
            if caption::os_state().pressed {
                // SAFETY: `hwnd` is the window whose procedure is running.
                unsafe { ShowWindow(hwnd, if IsZoomed(hwnd) != 0 { SW_RESTORE } else { SW_MAXIMIZE }) };
            }
            report(OsState { hovered: true, pressed: false });
            return 0;
        }
        WM_NCDESTROY => {
            let previous = PREVIOUS.swap(0, Ordering::Relaxed);
            WINDOW.store(0, Ordering::Relaxed);
            TRACKING.store(false, Ordering::Relaxed);
            caption::reset();
            *CONTEXT.lock().unwrap_or_else(PoisonError::into_inner) = None;
            if previous != 0 {
                // SAFETY: puts back the window's own procedure before it is destroyed.
                unsafe { set_window_long(hwnd, GWLP_WNDPROC, previous) };
            }
            return call_previous(previous, hwnd, msg, wparam, lparam);
        }
        _ => {}
    }
    call_previous(PREVIOUS.load(Ordering::Relaxed), hwnd, msg, wparam, lparam)
}

/// Hands a message to the window's own procedure.
fn call_previous(previous: isize, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // SAFETY: `previous` came from the pointer-width window-long wrapper for GWLP_WNDPROC on
    // this window, so it is its procedure (or 0, which CallWindowProcW treats as default handling).
    unsafe {
        let previous = std::mem::transmute::<isize, WNDPROC>(previous);
        CallWindowProcW(previous, hwnd, msg, wparam, lparam)
    }
}
