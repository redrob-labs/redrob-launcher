// ADDED-LINE-RATIONALE f9c37334b37fcab5:1: Supports `events` while preserving the nearby constraint: read sit on apps it shows: Design has no release yet ("Coming soon").
// ADDED-LINE-RATIONALE a2248ab5b49f6003:1: Isolates `harness` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE 67370f2d23b05397:1: Delegates `harness`'s `harness_at / theme / manage` work to `harness_at` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 53889f8f4b66ecda:1: Isolates `harness_at` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE cb414ba5dc1bdc7d:1: Delegates `harness_at`'s `harness_with / events / theme` work to `harness_with` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0d2bf5ddc39d2a81:1: Freezes `settings` for `harness_with` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 357ca776319de61d:1: Freezes `let` for `grid_shows_every_status_and_installs_on_click` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 93234d7a4a14e4cd:1: Supports `grid_shows_every_status_and_installs_on_click` while preserving the nearby constraint: chip draws them any more; each card carries its status for screen readers.
// ADDED-LINE-RATIONALE d018dc4fc59de133:1: Freezes `assert` for `grid_shows_every_status_and_installs_on_click` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8481fbd7872b95ac:1: Supports `grid_shows_every_status_and_installs_on_click` while preserving the nearby constraint: Named by app as well as status, so two cards in the same state never share a name.
// ADDED-LINE-RATIONALE 45c2a9dc9f49b7c6:1: Supports `grid_shows_every_status_and_installs_on_click` while preserving the nearby constraint: No links are inline; each visible card has a named menu trigger in addition to right-click.
// ADDED-LINE-RATIONALE aec724e2ae638378:1: Supports `grid_shows_every_status_and_installs_on_click` while preserving the nearby constraint: Derive the expectation from rendered app cards so this assertion follows fixture changes.
// ADDED-LINE-RATIONALE 4624a478fd4533da:1: Makes `grid_shows_every_status_and_installs_on_click` fail visibly when `assert_eq / h / query_all_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 5915795eb4ab993f:1: Supports `downloads_can_be_paused_resumed_and_discarded` while preserving the nearby constraint: installed: give one to Redrob Office too, which it shows.
// ADDED-LINE-RATIONALE c191c33aebff4472:1: Delegates `downloads_can_be_paused_resumed_and_discarded`'s `ev / push / Event` work to `ev.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d894bb9df6dd8362:1: Freezes `let` for `downloads_can_be_paused_resumed_and_discarded` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9468953c4b1adde8:1: Delegates `downloads_can_be_paused_resumed_and_discarded`'s `h / set_size / egui` work to `h.set_size` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 563666fd5d4cd5b6:1: Delegates `downloads_can_be_paused_resumed_and_discarded`'s `h / get_by_label_contains / Download` work to `h.get_by_label_contains` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 32025f3397f9b84f:1: Supports `downloads_can_be_paused_resumed_and_discarded` while preserving the nearby constraint: The badge, and in a narrow tile the progress caption too.
// ADDED-LINE-RATIONALE 357ca776319de61d:2: Freezes `let` for `a_kept_download_offers_retry_install_and_download_again` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 22caeacae0640ff6:1: Makes `a_kept_download_offers_retry_install_and_download_again` fail visibly when `assert / h / query_all_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 297be7705a3ffb15:1: Delegates `a_kept_download_offers_retry_install_and_download_again`'s `open_card_more_button / h / Redrob` work to `open_card_more_button` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ab2295c9fc7d6292:1: Delegates `a_kept_download_offers_retry_install_and_download_again`'s `h / get_by_label / Download` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bfa733e27560a070:1: Delegates `a_kept_download_offers_retry_install_and_download_again`'s `h / key_press / egui` work to `h.key_press` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 574ddb955177ad6e:6: Delegates `a_kept_download_offers_retry_install_and_download_again`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c2901c90f446f0f1:1: Supports `a_kept_download_offers_retry_install_and_download_again` while preserving the nearby constraint: neither opens the card's menu nor presses it.
// ADDED-LINE-RATIONALE 83431985deedbebb:1: Makes `a_kept_download_offers_retry_install_and_download_again` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE fe39d89e86eb70a6:1: Makes `a_kept_download_offers_retry_install_and_download_again` fail visibly when `assert / sent / borrow` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 7de68734fed54418:1: Delegates `a_kept_download_offers_retry_install_and_download_again`'s `open_card_menu / h / Redrob` work to `open_card_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 60e5730556e865da:5: Declares the `test` contract on `a_kept_download_offers_retry_install_and_download_again` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE d0335ade9696095f:1: Supports `an_uninstalled_card_has_keyboard_accessible_more_actions` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 870eb6e67c47052c:1: Freezes `let` for `an_uninstalled_card_has_keyboard_accessible_more_actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 242e659534c05db0:5: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 44f577bd91d2087b:3: Freezes `if` for `an_uninstalled_card_has_keyboard_accessible_more_actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1f6624771538c9d3:3: Supports `an_uninstalled_card_has_keyboard_accessible_more_actions` while preserving the nearby constraint: Limit the grid so the focused control belongs to an unequivocally uninstalled app.
// ADDED-LINE-RATIONALE f6d344c6c1887114:11: Bounds the `an_uninstalled_card_has_keyboard_accessible_more_actions` implementation here so `l / settings / filter` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 22bca913d096b0d4:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `h / get_all_by_label / Redrob` work to `h.get_all_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:6: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a745f7c4cce14dab:1: Freezes `name` for `an_uninstalled_card_has_keyboard_accessible_more_actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b7d0ba239e8c14de:1: Supports `an_uninstalled_card_has_keyboard_accessible_more_actions` while preserving the nearby constraint: Query by role as well as name to lock in the control's assistive-technology semantics.
// ADDED-LINE-RATIONALE f35fa9178d7a9fb5:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `query_all / egui_kittest / kittest` work to `query_all` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE aa5ee6eb1dcf42cc:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `filter / node` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ad052152bcb808c1:1: Freezes `rect` for `an_uninstalled_card_has_keyboard_accessible_more_actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE acb9a774fb8d0330:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `rect / left / left` work to `rect.left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f2a468afe3cbe797:1: Makes `rect / left / left` an explicit part of `an_uninstalled_card_has_keyboard_accessible_more_actions` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE 799664b61f58268e:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `min_by / a / b` work to `min_by` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9d06c01b261ef0d6:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `unwrap` work to `unwrap` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 36833dd6ae570368:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `menu / focus` work to `menu.focus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 352dde1f85790827:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `h / key_press / egui` work to `h.key_press` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5ff08c955659e92d:2: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ff594f3a5d8a5a32:1: Delegates `an_uninstalled_card_has_keyboard_accessible_more_actions`'s `h / get_by_label / Release` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:13: Bounds the `an_uninstalled_card_has_keyboard_accessible_more_actions` implementation here so `h / get_by_label / Release` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:6: Declares the `test` contract on `an_uninstalled_card_has_keyboard_accessible_more_actions` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 235407bdca19d83b:1: Supports `every_job_state_keeps_named_more_actions` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 5915795eb4ab993f:2: Freezes `ev` for `every_job_state_keeps_named_more_actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE bd6390755921a00d:1: Supports `every_job_state_keeps_named_more_actions` while preserving the nearby constraint: Paused, while these overrides cover the phases that replace all ordinary action buttons.
// ADDED-LINE-RATIONALE 861602c4c642c81c:1: Keeps `redrob / office / JobPhase` in `every_job_state_keeps_named_more_actions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 330b44964dbd9c76:1: Keeps `redrob / canvas / JobPhase` in `every_job_state_keeps_named_more_actions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE da2b659febf9c5b7:1: Keeps `redrob / shape / JobPhase` in `every_job_state_keeps_named_more_actions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 3f8dd886ffcd5a1c:1: Keeps `redrob / browser / JobPhase` in `every_job_state_keeps_named_more_actions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE bffc3a7a868a5e99:1: Keeps `redrob / code / JobPhase` in `every_job_state_keeps_named_more_actions`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 2c6c678279ea5b25:1: Makes `redrob / code / JobPhase` an explicit part of `every_job_state_keeps_named_more_actions` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE 1b735161d05690b7:1: Delegates `every_job_state_keeps_named_more_actions`'s `ev / push / Event` work to `ev.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f6d344c6c1887114:12: Bounds the `every_job_state_keeps_named_more_actions` implementation here so `ev / push / Event` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b261104ff869bd5f:1: Freezes `let` for `every_job_state_keeps_named_more_actions` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:3: Delegates `every_job_state_keeps_named_more_actions`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 45729bbe08f06c30:1: Supports `every_job_state_keeps_named_more_actions` while preserving the nearby constraint: to the card menu rather than relying on an undiscoverable secondary click.
// ADDED-LINE-RATIONALE 6f30f1e59ca4de96:1: Delegates `every_job_state_keeps_named_more_actions`'s `open_card_more_button / h / app` work to `open_card_more_button` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ec9a520642af1c62:1: Delegates `every_job_state_keeps_named_more_actions`'s `h / get_by_label / Release` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f92b573f77b335c0:1: Delegates `every_job_state_keeps_named_more_actions`'s `h / key_press / egui` work to `h.key_press` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a692321169d53abe:1: Delegates `every_job_state_keeps_named_more_actions`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f6d344c6c1887114:13: Bounds the `every_job_state_keeps_named_more_actions` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:14: Bounds the `every_job_state_keeps_named_more_actions` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b261104ff869bd5f:2: Freezes `let` for `a_dismissed_install_prompt_is_not_called_a_cancelled_download` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d894bb9df6dd8362:2: Freezes `let` for `a_kept_update_names_its_version_and_an_up_to_date_app_is_not_downloaded_` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ed5b7647613a2a23:1: Delegates `a_kept_update_names_its_version_and_an_up_to_date_app_is_not_downloaded_`'s `h / get_by_label / Retry` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:7: Delegates `a_kept_update_names_its_version_and_an_up_to_date_app_is_not_downloaded_`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d22809bcfda49e05:1: Delegates `a_kept_update_names_its_version_and_an_up_to_date_app_is_not_downloaded_`'s `snapshot / h / grid` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d1d7d84ac3be5eb0:1: Supports `a_kept_update_names_its_version_and_an_up_to_date_app_is_not_downloaded_` while preserving the nearby constraint: kept file).
// ADDED-LINE-RATIONALE 7fb19254176d8b24:1: Freezes `let` for `an_install_waiting_for_another_says_so_and_can_be_cancelled` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4992d86e320cf857:1: Supports `an_install_waiting_for_another_says_so_and_can_be_cancelled` while preserving the nearby constraint: the third shelf now: bring it on screen first.).
// ADDED-LINE-RATIONALE 242e659534c05db0:8: Delegates `an_install_waiting_for_another_says_so_and_can_be_cancelled`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d894bb9df6dd8362:3: Freezes `let` for `the_launcher_update_waiting_for_other_installs_can_be_cancelled` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b261104ff869bd5f:3: Freezes `let` for `each_cancellation_says_what_it_called_off` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 318d1634f3cf10dd:1: Supports `a_search_that_matches_nothing_says_what_to_relax_and_clears` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 59de9bb2e3407cdc:1: Freezes `let` for `a_search_that_matches_nothing_says_what_to_relax_and_clears` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:8: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 58aeae207dddfee1:1: Freezes `field` for `a_search_that_matches_nothing_says_what_to_relax_and_clears` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2ab102d4bbfd69a7:1: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `field / focus` work to `field.focus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 700301155da5c8f5:1: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `field / type_text / zzz` work to `field.type_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:9: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE bc2290a377db7ef8:1: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `snapshot / h / empty` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2355f9d69a1cad16:1: Supports `a_search_that_matches_nothing_says_what_to_relax_and_clears` while preserving the nearby constraint: A DS empty state: what happened, what to relax, and the way back.
// ADDED-LINE-RATIONALE 07a2f81cddab2cf6:1: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `h / get_by_label_contains / Nothing` work to `h.get_by_label_contains` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 134e57d3e7248508:1: Makes `a_search_that_matches_nothing_says_what_to_relax_and_clears` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 0cc7fdaf33501298:1: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `h / get_by_label / Clear` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:10: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 18f983796ec0660e:1: Makes `a_search_that_matches_nothing_says_what_to_relax_and_clears` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 20636db5306641a0:1: Delegates `a_search_that_matches_nothing_says_what_to_relax_and_clears`'s `h / get_by_label / Redrob` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f4ef9ae272afdffb:1: Supports `switching_filters` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 870eb6e67c47052c:2: Freezes `let` for `switching_filters` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 383491322e729c38:1: Supports `switching_filters` while preserving the nearby constraint: The grid is the only view: no Grid / Table control, no sort.
// ADDED-LINE-RATIONALE 1a8093361ad846f0:1: Makes `switching_filters` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE b8e6defa8d88c497:1: Delegates `switching_filters`'s `snapshot / h / grid` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0bf5f6dcd6b84bbd:1: Supports `from_account_menu` while preserving the nearby constraint: above itself.
// ADDED-LINE-RATIONALE 3994e54779f9a594:1: Freezes `row_rect` for `from_account_menu` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4ff41e034962f30d:1: Delegates `from_account_menu`'s `h / get_by_label / row` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:12: Delegates `from_account_menu`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f6d342c6c1886dae:1: Bounds the `from_account_menu` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5d00889072622055:1: Supports `from_account_menu` while preserving the nearby constraint: The menu's item: the nearest one above the row (a page title can share its label).
// ADDED-LINE-RATIONALE 4f21f36c7bec24fa:1: Delegates `from_account_menu`'s `get_all_by_label / item` work to `get_all_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4cf847c772cc57df:1: Delegates `from_account_menu`'s `filter / n / n` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 818374fda7b466dc:1: Delegates `from_account_menu`'s `max_by / a / b` work to `max_by` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9bdbaaa77852bbc6:1: Delegates `from_account_menu`'s `unwrap` work to `unwrap` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a83f3b7eb18a7cbd:1: Delegates `from_account_menu`'s `node / click` work to `node.click` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f6d344c6c1887114:17: Bounds the `from_account_menu` implementation here so `node / click` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 242e659534c05db0:13: Delegates `from_account_menu`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:22: Bounds the `from_account_menu` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5255d3ffefa77e2e:1: Supports `open_settings` while preserving the nearby constraint: backend) opens its menu, and the menu's Settings item opens the page.
// ADDED-LINE-RATIONALE 38d06d8177a0d5a5:1: Delegates `open_settings`'s `from_account_menu / h / Settings` work to `from_account_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:23: Bounds the `open_settings` implementation here so `from_account_menu / h / Settings` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6b6dfc56beca1fdf:1: Supports `open_card_menu` while preserving the nearby constraint: click). Every tile also has the named … button, including read-only and job states.
// ADDED-LINE-RATIONALE 8fde09899887d89e:1: Delegates `open_card_menu`'s `h / get_all_by_label / app` work to `h.get_all_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:14: Delegates `open_card_menu`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a801486921c9c85f:1: Delegates `open_card_menu`'s `h / get_all_by_label / app` work to `h.get_all_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5ff08c955659e92d:3: Delegates `open_card_menu`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:24: Bounds the `open_card_menu` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 10aa85d439a548d4:1: Supports `open_card_more_button` while preserving the nearby constraint: same tile.
// ADDED-LINE-RATIONALE 8fde09899887d89e:2: Delegates `open_card_more_button`'s `h / get_all_by_label / app` work to `h.get_all_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:15: Delegates `open_card_more_button`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f6d342c6c1886dae:2: Bounds the `open_card_more_button` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 232c5c32b2fe74d3:1: Freezes `name` for `open_card_more_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 33497e384ff1634e:1: Freezes `menu` for `open_card_more_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b64f6468839e8f99:1: Delegates `open_card_more_button`'s `get_all_by_label / More / actions` work to `get_all_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2e508c1d7dfe97b0:1: Delegates `open_card_more_button`'s `filter / n` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 78aa3489c11f38bd:1: Freezes `r` for `open_card_more_button` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 3fbdd1d17f654088:1: Delegates `open_card_more_button`'s `r / left / left` work to `r.left` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cb04887595f97487:1: Makes `r / left / left` an explicit part of `open_card_more_button` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE 90584161642f1a3e:1: Delegates `open_card_more_button`'s `min_by / a / b` work to `min_by` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9bdbaaa77852bbc6:2: Delegates `open_card_more_button`'s `unwrap` work to `unwrap` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f3018afbbb13e0b8:1: Delegates `open_card_more_button`'s `menu / click` work to `menu.click` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f6d344c6c1887114:18: Bounds the `open_card_more_button` implementation here so `menu / click` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 5ff08c955659e92d:4: Delegates `open_card_more_button`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:25: Bounds the `open_card_more_button` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:14: Declares the `test` contract on `open_card_more_button` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 1fee342d2e7bd3cb:1: Supports `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE ea398e46da41f23a:1: Gives `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` direct access to `redrob_launcher_ui_egui / settings / SortKey` instead of relying on an implicit platform facility.
// ADDED-LINE-RATIONALE 9a748ba77295813c:1: Supports `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` while preserving the nearby constraint: Saved by an earlier version, when there was a table and a sort.
// ADDED-LINE-RATIONALE 1705355caf4d75fa:1: Makes `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` fail visibly when `assert_eq / old / view` violates the behavior this path promises.
// ADDED-LINE-RATIONALE d1a3ed0a97035474:1: Freezes `backend` for `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4915d32ba3a801aa:2: Delegates `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it`'s `redrob_launcher_ui_egui / i18n / set_test_lang` work to `redrob_launcher_ui_egui::i18n::set_test_lang` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE eed9dfb33dd7afc5:2: Freezes `h` for `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 32d4dc1e4b83d3df:2: Makes `move / ui / state` an explicit part of `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE ad178ecd618da6e1:2: Prevents `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` from applying its side effect outside the `state / is_none` state.
// ADDED-LINE-RATIONALE 15c7ec016cd8459c:2: Delegates `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it`'s `b / backend / take` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 86686d0882678c8e:2: Bounds the `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` implementation here so `b / backend / take` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 420f02183eec3c8f:1: Delegates `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it`'s `state / LauncherUi / new` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 86686f0882678ff4:4: Supports `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` while preserving the nearby constraint: state = Some(LauncherUi::new(ui.ctx(), Box::new(b), old.clone()));.
// ADDED-LINE-RATIONALE a07eb68c80343947:2: Freezes `if` for `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 75131bcdf2e269b0:2: Delegates `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it`'s `l / logic / ui` work to `l.logic` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f29dc206299e294a:2: Delegates `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it`'s `l / ui / ui` work to `l.ui` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 86686f0882678ff4:5: Bounds the `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` implementation here so `l / ui / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f2a463afe3cbdf18:3: Keeps `l / ui / ui` in `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ff1b03ed74a59fd1:2: Keeps `h / run_steps` in `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f20484badb5742b9:3: Bounds the `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:10: Delegates `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 329f73e916915e43:1: Supports `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` while preserving the nearby constraint: The grid, on its three shelves in page order: the AI engine, everyday work, creative work.
// ADDED-LINE-RATIONALE 7bb8a719edf619ba:1: Freezes `work` for `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE ffcea1a0da4318b0:1: Freezes `creative` for `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE efd78e98ed87043d:1: Makes `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` fail visibly when `assert / engine / top` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 10f389d220c2f74e:1: Makes `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE b63cfc633884cfcf:1: Freezes `top` for `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE aba6c1bf1409f1d3:1: Makes `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` fail visibly when `assert / top / Redrob` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 07e4156afb9a42eb:1: Makes `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` fail visibly when `assert / top / Redrob` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 4b9e2128600318e7:1: Makes `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` fail visibly when `assert / top / Redrob` violates the behavior this path promises.
// ADDED-LINE-RATIONALE d7cad83484b2df9b:1: Supports `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` while preserving the nearby constraint: Query and Recall, not installed here, stay off the page.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:26: Bounds the `an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it` implementation here so `assert / h / query_all_by_label` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 39e022399fb881dc:1: Freezes `rects` for `below_icon_centre` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1743d8fe77ee67be:1: Freezes `left` for `below_icon_centre` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 350935cb38b4854a:1: Applies `below_icon_centre` consistently to every `l / r / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 005e6940aaee4c23:1: Makes `below_icon_centre` fail visibly when `assert / r / left` violates the behavior this path promises.
// ADDED-LINE-RATIONALE f6d344c6c1887114:19: Bounds the `below_icon_centre` implementation here so `assert / r / left` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3e6222815b203e0d:1: Freezes `text` for `below_icon_centre` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 224df29224599d9b:1: Makes `below_icon_centre` fail visibly when `assert / text / left` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 3b5dd96d75828336:1: Supports `name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 0d274c93df89bb6c:1: Freezes `let` for `name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 48ec40999a4f2d8c:1: Freezes `let` for `name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 92a4d7669df50baa:1: Delegates `name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it`'s `snapshot / h / format` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 735d6b8b31e37cf9:2: Freezes `off` for `name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 71edc07e5c3aa6f2:1: Makes `name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it` fail visibly when `assert / off / abs` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 60e5730556e865da:16: Declares the `test` contract on `name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE dc257c9c5a30a748:1: Supports `pages_lead_back_to_apps_by_the_back_button_and_escape` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 870eb6e67c47052c:3: Freezes `let` for `pages_lead_back_to_apps_by_the_back_button_and_escape` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:11: Delegates `pages_lead_back_to_apps_by_the_back_button_and_escape`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b33e32408a21b169:1: Freezes `on_settings` for `pages_lead_back_to_apps_by_the_back_button_and_escape` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 203f37954b9ef411:1: Supports `pages_lead_back_to_apps_by_the_back_button_and_escape` while preserving the nearby constraint: Settings is a page: no Done button, one primary at most (Website is secondary now).
// ADDED-LINE-RATIONALE ab396721bed60318:1: Makes `pages_lead_back_to_apps_by_the_back_button_and_escape` fail visibly when `assert / on_settings / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE f5cb485441e9bd74:1: Makes `pages_lead_back_to_apps_by_the_back_button_and_escape` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE cfca8612db6e42f5:1: Delegates `pages_lead_back_to_apps_by_the_back_button_and_escape`'s `h / get_by_label / Quit` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b13ea656b25060b5:1: Supports `pages_lead_back_to_apps_by_the_back_button_and_escape` while preserving the nearby constraint: The top bar's back button.
// ADDED-LINE-RATIONALE 242e659534c05db0:16: Delegates `pages_lead_back_to_apps_by_the_back_button_and_escape`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8d3b069b8cf158cd:1: Makes `pages_lead_back_to_apps_by_the_back_button_and_escape` fail visibly when `assert / on_settings / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE b4f961e79622166d:3: Delegates `pages_lead_back_to_apps_by_the_back_button_and_escape`'s `h / get_by_label / Update` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9abe72891143c0b1:1: Makes `pages_lead_back_to_apps_by_the_back_button_and_escape` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 203f37954b9ef411:2: Supports `pages_lead_back_to_apps_by_the_back_button_and_escape` while preserving the nearby constraint: Escape.
// ADDED-LINE-RATIONALE ab396721bed60318:2: Makes `pages_lead_back_to_apps_by_the_back_button_and_escape` fail visibly when `assert / on_settings / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE bfa733e27560a070:2: Delegates `pages_lead_back_to_apps_by_the_back_button_and_escape`'s `h / key_press / egui` work to `h.key_press` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:17: Delegates `pages_lead_back_to_apps_by_the_back_button_and_escape`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8d3b069b8cf158cd:2: Makes `pages_lead_back_to_apps_by_the_back_button_and_escape` fail visibly when `assert / on_settings / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE b4f961e79622166d:4: Delegates `pages_lead_back_to_apps_by_the_back_button_and_escape`'s `h / get_by_label / Update` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ee2dc7433034c236:1: Supports `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 870eb6e67c47052c:4: Freezes `let` for `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b33e32408a21b169:2: Freezes `on_settings` for `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 203f37954b9ef411:3: Supports `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` while preserving the nearby constraint: Keyboard focus on an ordinary button (as after Tab) doesn't swallow Escape.
// ADDED-LINE-RATIONALE ec033ef8f10b1352:1: Delegates `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list`'s `h / get_by_label / Quit` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 574ddb955177ad6e:11: Delegates `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 271686b35be7bc9d:1: Makes `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` fail visibly when `assert / h / ctx` violates the behavior this path promises.
// ADDED-LINE-RATIONALE bfa733e27560a070:3: Delegates `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list`'s `h / key_press / egui` work to `h.key_press` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:18: Delegates `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 132cec76164bab52:1: Makes `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` fail visibly when `assert / on_settings / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 203f37954b9ef411:4: Supports `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` while preserving the nearby constraint: An open list takes Escape for itself: it closes, and the page stays.
// ADDED-LINE-RATIONALE 1d9b8cfbb3487b11:1: Delegates `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list`'s `h / get_by_value / Automatically` work to `h.get_by_value` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:19: Delegates `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 70399d7d72f0153a:1: Makes `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` fail visibly when `assert / egui / Popup` violates the behavior this path promises.
// ADDED-LINE-RATIONALE bfa733e27560a070:4: Delegates `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list`'s `h / key_press / egui` work to `h.key_press` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:20: Delegates `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 310f0200849291f5:1: Makes `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` fail visibly when `assert / on_settings / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:30: Bounds the `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` implementation here so `assert / on_settings / h` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:18: Declares the `test` contract on `escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE dfb424a48d0613dc:1: Supports `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 6b5bb4f502d6e63c:1: Applies `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` consistently to every `width / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 829c7671460dd81b:1: Freezes `h` for `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 84c6161156ee9242:1: Delegates `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2451cf39a12d81f9:1: Freezes `options` for `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE babf91012130a24f:1: Makes `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` fail visibly when `assert_eq / options / len` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 42cc5149386bbd9e:1: Applies `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` consistently to every `r / in / options` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 7c5e3e918828dd06:1: Freezes `assert` for `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 72ced9a9e4dd28a4:1: Bounds the `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` implementation here so `assert / r / width` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 61249db9e251aae1:1: Supports `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` while preserving the nearby constraint: Neighbouring targets never overlap.
// ADDED-LINE-RATIONALE 7aeb4cf6f247cf2c:1: Freezes `assert` for `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 72ced9a9e4dd28a4:2: Bounds the `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` implementation here so `assert / a / intersect` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60ee750f60b476f9:1: Delegates `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme`'s `h / get_by_label / Light` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b00e63116f7040bd:1: Delegates `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a98fc924ff39f6c8:1: Makes `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` fail visibly when `assert_eq / h / state` violates the behavior this path promises.
// ADDED-LINE-RATIONALE f6d344c6c1887114:21: Bounds the `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` implementation here so `assert_eq / h / state` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:31: Bounds the `the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme` implementation here so `assert_eq / h / state` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a586c66e337ee1bd:1: Supports `harness_sized` while preserving the nearby constraint: The default harness at `width` x 900.
// ADDED-LINE-RATIONALE d1a3ed0a97035474:2: Freezes `backend` for `harness_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 4915d32ba3a801aa:3: Delegates `harness_sized`'s `redrob_launcher_ui_egui / i18n / set_test_lang` work to `redrob_launcher_ui_egui::i18n::set_test_lang` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 298a19ebeb91ffd7:1: Delegates `harness_sized`'s `Harness / builder / with_size` work to `Harness::builder` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 32d4dc1e4b83d3df:3: Makes `move / ui / state` an explicit part of `harness_sized` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE ad178ecd618da6e1:3: Prevents `harness_sized` from applying its side effect outside the `state / is_none` state.
// ADDED-LINE-RATIONALE 15c7ec016cd8459c:3: Delegates `harness_sized`'s `b / backend / take` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 86686d0882678c8e:3: Bounds the `harness_sized` implementation here so `b / backend / take` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 83168605afcf20f4:1: Delegates `harness_sized`'s `state / LauncherUi / new` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 86686f0882678ff4:6: Supports `harness_sized` while preserving the nearby constraint: state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));.
// ADDED-LINE-RATIONALE a07eb68c80343947:3: Freezes `if` for `harness_sized` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 75131bcdf2e269b0:3: Delegates `harness_sized`'s `l / logic / ui` work to `l.logic` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f29dc206299e294a:3: Delegates `harness_sized`'s `l / ui / ui` work to `l.ui` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 86686f0882678ff4:7: Bounds the `harness_sized` implementation here so `l / ui / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f2a463afe3cbdf18:4: Keeps `l / ui / ui` in `harness_sized`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE ff1b03ed74a59fd1:3: Keeps `l / ui / ui` in `harness_sized`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f6d370c6c188bbd8:1: Bounds the `harness_sized` implementation here so `test` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 870eb6e67c47052c:5: Freezes `let` for `settings_link_to_redrob` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 203f37954b9ef411:5: Delegates `settings_link_to_redrob`'s `open_settings / h` work to `open_settings` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 870eb6e67c47052c:6: Freezes `let` for `links_to_the_website_github_and_each_apps_pages` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b236210bd0194b32:1: Supports `links_to_the_website_github_and_each_apps_pages` while preserving the nearby constraint: The Apps page has no links: no footer, nothing on the cards.
// ADDED-LINE-RATIONALE a86f93b0464d350c:1: Makes `links_to_the_website_github_and_each_apps_pages` fail visibly when `assert_eq / h / query_all_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 3ce24e96313bcdd7:1: Supports `links_to_the_website_github_and_each_apps_pages` while preserving the nearby constraint: A card's menu has the app's GitHub repo and no website entry (no app has a web page yet).
// ADDED-LINE-RATIONALE 0e43a9cc697f3ee9:1: Makes `links_to_the_website_github_and_each_apps_pages` fail visibly when `assert_eq / h / query_all_by_label_contains` violates the behavior this path promises.
// ADDED-LINE-RATIONALE bfa733e27560a070:5: Supports `links_to_the_website_github_and_each_apps_pages` while preserving the nearby constraint: Redrob's own pages are in Settings.
// ADDED-LINE-RATIONALE 5ff08c955659e92d:5: Delegates `links_to_the_website_github_and_each_apps_pages`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 203f37954b9ef411:6: Delegates `links_to_the_website_github_and_each_apps_pages`'s `open_settings / h` work to `open_settings` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9578634e2d7f6639:1: Delegates `links_to_the_website_github_and_each_apps_pages`'s `h / get_by_label / Website` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c9bfb8dbe0b0d1af:1: Delegates `links_to_the_website_github_and_each_apps_pages`'s `h / get_by_label / GitHub` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:34: Bounds the `links_to_the_website_github_and_each_apps_pages` implementation here so `h / get_by_label / GitHub` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:21: Declares the `test` contract on `links_to_the_website_github_and_each_apps_pages` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE f67e0ea96dc789dc:1: Supports `the_account_menu_holds_settings_quit_and_the_version` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 6b5bb4f502d6e63c:2: Applies `the_account_menu_holds_settings_quit_and_the_version` consistently to every `width / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 829c7671460dd81b:2: Freezes `h` for `the_account_menu_holds_settings_quit_and_the_version` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 84c6161156ee9242:2: Delegates `the_account_menu_holds_settings_quit_and_the_version`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 56c6a4b2781646e7:1: Supports `the_account_menu_holds_settings_quit_and_the_version` while preserving the nearby constraint: No account backend: one row reading Settings, no Settings row of its own.
// ADDED-LINE-RATIONALE 5dfe2ea66ebb1606:1: Freezes `row` for `the_account_menu_holds_settings_quit_and_the_version` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e94cd9db4cd21922:1: Delegates `the_account_menu_holds_settings_quit_and_the_version`'s `h / get_by_label / Settings` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 72993c114c6520c0:1: Delegates `the_account_menu_holds_settings_quit_and_the_version`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 69a2e0619c30b9b4:1: Delegates `the_account_menu_holds_settings_quit_and_the_version`'s `snapshot / h / format` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 13865df8072ba33f:1: Freezes `version` for `the_account_menu_holds_settings_quit_and_the_version` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 8aabe5e754d4c90f:1: Freezes `quit` for `the_account_menu_holds_settings_quit_and_the_version` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE c50e4590d74fa119:1: Freezes `settings` for `the_account_menu_holds_settings_quit_and_the_version` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 485d98969ae5d37e:1: Supports `the_account_menu_holds_settings_quit_and_the_version` while preserving the nearby constraint: Above the row, in order: Settings, Quit, the version.
// ADDED-LINE-RATIONALE a1783476d3da26ad:1: Makes `the_account_menu_holds_settings_quit_and_the_version` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE f92b573f77b335c0:2: Supports `the_account_menu_holds_settings_quit_and_the_version` while preserving the nearby constraint: Escape closes it.
// ADDED-LINE-RATIONALE 72993c114c6520c0:2: Delegates `the_account_menu_holds_settings_quit_and_the_version`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 351360209e6ffd69:1: Makes `the_account_menu_holds_settings_quit_and_the_version` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE f6d344c6c1887114:22: Bounds the `the_account_menu_holds_settings_quit_and_the_version` implementation here so `assert / h / query_by_label` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 947cbe725cacf0f9:1: Freezes `h` for `the_account_menu_holds_settings_quit_and_the_version` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:15: Delegates `the_account_menu_holds_settings_quit_and_the_version`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 203f37954b9ef411:7: Delegates `the_account_menu_holds_settings_quit_and_the_version`'s `open_settings / h` work to `open_settings` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4a54c41187c73ff5:2: Delegates `the_account_menu_holds_settings_quit_and_the_version`'s `h / get_by_label / News` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:35: Bounds the `the_account_menu_holds_settings_quit_and_the_version` implementation here so `h / get_by_label / News` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:22: Declares the `test` contract on `the_account_menu_holds_settings_quit_and_the_version` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 1803eae29211a96c:1: Supports `the_logo_opens_console_redrob_ai` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 870eb6e67c47052c:7: Freezes `let` for `the_logo_opens_console_redrob_ai` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:16: Delegates `the_logo_opens_console_redrob_ai`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dcb7a7c7a4043ae1:1: Makes `the_logo_opens_console_redrob_ai` fail visibly when `assert_eq / click_opens / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:36: Bounds the `the_logo_opens_console_redrob_ai` implementation here so `assert_eq / click_opens / h` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:23: Declares the `test` contract on `the_logo_opens_console_redrob_ai` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE ea9563a31b2eaa73:1: Supports `typing_in_the_search_on_another_page_shows_the_apps` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 870eb6e67c47052c:8: Freezes `let` for `typing_in_the_search_on_another_page_shows_the_apps` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:17: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b33e32408a21b169:3: Freezes `on_settings` for `typing_in_the_search_on_another_page_shows_the_apps` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 203f37954b9ef411:8: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `open_settings / h` work to `open_settings` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE ab396721bed60318:3: Makes `typing_in_the_search_on_another_page_shows_the_apps` fail visibly when `assert / on_settings / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE b51a344ff553c88d:1: Supports `typing_in_the_search_on_another_page_shows_the_apps` while preserving the nearby constraint: The search stays in the title bar on every page.
// ADDED-LINE-RATIONALE b249f4905accc392:1: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `query_all / egui_kittest / kittest` work to `query_all` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 697968585f799ced:1: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `find / n / n` work to `find` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9d06c01b261ef0d6:3: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `unwrap` work to `unwrap` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 2ab102d4bbfd69a7:2: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `field / focus` work to `field.focus` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8ef220b20de09fa1:1: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `field / type_text / canvas` work to `field.type_text` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 242e659534c05db0:21: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7059a0b4c402b8c5:1: Makes `typing_in_the_search_on_another_page_shows_the_apps` fail visibly when `assert / on_settings / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 20636db5306641a0:3: Delegates `typing_in_the_search_on_another_page_shows_the_apps`'s `h / get_by_label / Redrob` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 136f99b6f510e706:1: Makes `typing_in_the_search_on_another_page_shows_the_apps` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 870eb6e67c47052c:9: Freezes `let` for `links_open_the_right_pages` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 43cd31cc39eef1d8:1: Supports `links_open_the_right_pages` while preserving the nearby constraint: Each app's pages, from its card's menu.
// ADDED-LINE-RATIONALE b237f3df4f1d0fdb:1: Makes `links_open_the_right_pages` fail visibly when `assert_eq / click_opens / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE c8d9d40fe515b0cf:1: Delegates `links_open_the_right_pages`'s `open_card_menu / h / Redrob` work to `open_card_menu` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 203f37954b9ef411:9: Supports `links_open_the_right_pages` while preserving the nearby constraint: Redrob's own, from Settings.
// ADDED-LINE-RATIONALE 378d07d1c44f89e1:1: Makes `links_open_the_right_pages` fail visibly when `assert_eq / click_opens / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 7766d3a2905bb695:1: Makes `links_open_the_right_pages` fail visibly when `assert_eq / click_opens / h` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 60e5730556e865da:26: Declares the `test` contract on `looks_again_at_installed_apps_every_few_minutes_not_every_frame` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE e1255b10dac16caa:1: Supports `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 59de9bb2e3407cdc:2: Freezes `let` for `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:19: Delegates `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE de3e24515b52ac96:1: Supports `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` while preserving the nearby constraint: Redrob Browser ends the everyday-work shelf's row of three, so it sits in the last column.
// ADDED-LINE-RATIONALE 389fa80c69c06e2c:1: Freezes `refresh` for `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0dc8c17f9f1e1597:1: Supports `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` while preserving the nearby constraint: The harness draws inside an 8 px margin of its own, so its "window" edge is at 1172.
// ADDED-LINE-RATIONALE ee0fb16069feb868:1: Freezes `assert` for `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1a52c29383513ca2:1: Supports `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` while preserving the nearby constraint: Hovering the page shows the thumb (for the snapshot: it is painted, not a node).
// ADDED-LINE-RATIONALE 34a7bf953dd76132:20: Delegates `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 69d8b560f45d4f71:1: Delegates `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it`'s `snapshot / h / grid` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:42: Bounds the `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` implementation here so `snapshot / h / grid` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:27: Declares the `test` contract on `the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE a920ed82da19f0a7:1: Supports `the_launcher_update_banner_is_one_slim_line` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 5915795eb4ab993f:7: Freezes `ev` for `the_launcher_update_banner_is_one_slim_line` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0798d3c5bc2a00d3:3: Delegates `the_launcher_update_banner_is_one_slim_line`'s `ev / push / Event` work to `ev.push` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7fe5cb84f99d7daf:1: Freezes `let` for `the_launcher_update_banner_is_one_slim_line` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:21: Delegates `the_launcher_update_banner_is_one_slim_line`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE efe436a59ec2b053:1: Supports `the_launcher_update_banner_is_one_slim_line` while preserving the nearby constraint: The prompt asks first; "Later" leaves the banner.
// ADDED-LINE-RATIONALE 242e659534c05db0:22: Delegates `the_launcher_update_banner_is_one_slim_line`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 83dbf4b4178a6811:1: Delegates `the_launcher_update_banner_is_one_slim_line`'s `snapshot / h / launcher` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 085860ab2403b477:1: Delegates `the_launcher_update_banner_is_one_slim_line`'s `h / get_by_label_contains / Redrob` work to `h.get_by_label_contains` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE eb64432ceacb0c07:1: Supports `the_launcher_update_banner_is_one_slim_line` while preserving the nearby constraint: 2 px taller fails (before this, a 66 px bar and a 14 px gap put the cells 92 px down).
// ADDED-LINE-RATIONALE e75699a652219176:1: Freezes `action` for `the_launcher_update_banner_is_one_slim_line` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d707f9eccee936d4:1: Delegates `the_launcher_update_banner_is_one_slim_line`'s `iter` work to `iter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 961aef7abbc4210a:1: Delegates `the_launcher_update_banner_is_one_slim_line`'s `flat_map / l / h` work to `flat_map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b505e4a4588c5a6b:1: Delegates `the_launcher_update_banner_is_one_slim_line`'s `find / r / r` work to `find` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9d06c01b261ef0d6:4: Delegates `the_launcher_update_banner_is_one_slim_line`'s `unwrap` work to `unwrap` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 5efe9280471b6927:1: Makes `the_launcher_update_banner_is_one_slim_line` fail visibly when `assert_eq / dismiss / height` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 1b35137c09e0e2f6:1: Supports `the_launcher_update_banner_is_one_slim_line` while preserving the nearby constraint: The filter row's "All" cell: the first thing under the banner.
// ADDED-LINE-RATIONALE dd38e779eeb55858:1: Delegates `the_launcher_update_banner_is_one_slim_line`'s `query_all_by_label_contains / All` work to `query_all_by_label_contains` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c74cc0248939c91f:2: Delegates `the_launcher_update_banner_is_one_slim_line`'s `map / n / n` work to `map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE cbb95df45b19bf82:1: Delegates `the_launcher_update_banner_is_one_slim_line`'s `filter / r / r` work to `filter` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE acbeff8e6277ead4:1: Delegates `the_launcher_update_banner_is_one_slim_line`'s `min_by / a / b` work to `min_by` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 9d06c01b261ef0d6:5: Delegates `the_launcher_update_banner_is_one_slim_line`'s `unwrap` work to `unwrap` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a0d5b073d174750a:1: Freezes `bar_top` for `the_launcher_update_banner_is_one_slim_line` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b1895a32a09cd921:1: Freezes `assert` for `the_launcher_update_banner_is_one_slim_line` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:43: Bounds the `the_launcher_update_banner_is_one_slim_line` implementation here so `assert / filters / top` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE b261104ff869bd5f:4: Freezes `let` for `launcher_update_prompts_to_relaunch_with_updated_version` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE caa59a487719b5bd:1: Delegates `launcher_update_prompts_to_relaunch_with_updated_version`'s `snapshot / h / launcher` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 870eb6e67c47052c:10: Supports `the_current_desktop_setting_is_only_offered_where_the_os_has_desktops` while preserving the nearby constraint: NoHost has no desktops (Windows, Linux, web): no setting.
// ADDED-LINE-RATIONALE 203f37954b9ef411:10: Delegates `the_current_desktop_setting_is_only_offered_where_the_os_has_desktops`'s `open_settings / h` work to `open_settings` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 432ab11a4ecc3585:1: Supports `settings_name_the_services_behind_what_is_on_screen` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 80d762a6cc2c0b9a:1: Freezes `status` for `settings_name_the_services_behind_what_is_on_screen` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 909293cad4f56e6d:1: Supports `settings_name_the_services_behind_what_is_on_screen` while preserving the nearby constraint: No footer on the Apps page any more.
// ADDED-LINE-RATIONALE 435ff2054aa61181:1: Delegates `settings_name_the_services_behind_what_is_on_screen`'s `open_settings / h` work to `open_settings` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE db3c279261c1934f:1: Makes `settings_name_the_services_behind_what_is_on_screen` fail visibly when `assert_eq / status / Vec` violates the behavior this path promises.
// ADDED-LINE-RATIONALE f040e8a57eee75d8:1: Makes `settings_name_the_services_behind_what_is_on_screen` fail visibly when `assert_eq / status / from_github` violates the behavior this path promises.
// ADDED-LINE-RATIONALE c6bbd10b139399fb:1: Freezes `let` for `web_build_offers_downloads` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE aec724e2ae638378:2: Supports `web_build_offers_downloads` while preserving the nearby constraint: Read-only web cards still need a discoverable route to alternate packages and links.
// ADDED-LINE-RATIONALE f439a25545ca549f:1: Makes `web_build_offers_downloads` fail visibly when `assert_eq / h / query_all_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE faaefc7c1d78050f:1: Supports `search_field_rect` while preserving the nearby constraint: The title bar's search field.
// ADDED-LINE-RATIONALE 706c611afafaad98:1: Delegates `search_field_rect`'s `h / query_all / egui_kittest` work to `h.query_all` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE c74cc0248939c91f:3: Delegates `search_field_rect`'s `map / n / n` work to `map` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 3383aaf45c04576e:1: Delegates `search_field_rect`'s `find / r / r` work to `find` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 7a5e9b530aae44e9:1: Delegates `search_field_rect`'s `unwrap` work to `unwrap` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:53: Bounds the `search_field_rect` implementation here so `unwrap` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:37: Declares the `test` contract on `search_field_rect` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 35e1a2a53ddd645f:1: Supports `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 357ca776319de61d:3: Supports `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` while preserving the nearby constraint: NoHost: the OS draws the title bar (or there is none, on the web).
// ADDED-LINE-RATIONALE 34a7bf953dd76132:26: Delegates `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 8dc25a2bd12c2c7a:1: Delegates `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page`'s `snapshot / h / grid` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 796a8fbcbf308bb8:1: Makes `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` fail visibly when `assert / h / query_by_label` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 67d91c521ddd9187:1: Supports `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` while preserving the nearby constraint: No title bar of ours: Update, the logo and the search sit in a 48 px row over the page, left to right.
// ADDED-LINE-RATIONALE e577ae42924b161e:1: Freezes `logo` for `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 549f838788c28087:1: Freezes `update` for `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9d4133968b6a3b1c:1: Applies `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` consistently to every `r / in / search` item rather than handling only the first match.
// ADDED-LINE-RATIONALE bd89645735fc23a5:1: Freezes `assert` for `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f6d344c6c1887114:27: Bounds the `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` implementation here so `assert / r / bottom` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3f2b2e227bbf6138:1: Freezes `assert` for `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b16da5441a3e1c72:1: Supports `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` while preserving the nearby constraint: The Apps page has no top bar of its own: no title over the grid, refresh at the filter row's end.
// ADDED-LINE-RATIONALE 3d78f4b542c0057a:1: Freezes `check` for `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 70f4f6ca1d6af8b9:1: Freezes `filters` for `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b226367569683f36:1: Makes `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` fail visibly when `assert / check / top` violates the behavior this path promises.
// ADDED-LINE-RATIONALE cf2d7b660be516e0:1: Makes `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` fail visibly when `assert / check / left` violates the behavior this path promises.
// ADDED-LINE-RATIONALE a705ddbe598bf745:5: Delegates `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page`'s `sent / borrow_mut / clear` work to `sent.borrow_mut` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 85bf8a434a11c692:1: Delegates `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page`'s `h / get_by_label / Check` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 574ddb955177ad6e:13: Delegates `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 52ff67b6ab16c59d:1: Makes `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` fail visibly when `assert / sent / borrow` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:54: Bounds the `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` implementation here so `assert / sent / borrow` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:38: Declares the `test` contract on `a_native_title_bar_puts_the_chrome_in_a_row_over_the_page` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 486944cfbc94f764:1: Supports `the_macos_title_bar_keeps_the_traffic_lights_clear` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 17dd26a8b09f2706:1: Isolates `MacHost` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE 5bf2e85764c0d0f8:1: Makes `Host / MacHost` an explicit part of `MacHost` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE 2850dec2d6ffd13c:1: Isolates `title_bar` as the owner of this behavior so callers can use and test it without duplicating policy.
// ADDED-LINE-RATIONALE a44d1ecd27c6dcd2:1: Makes `redrob_launcher_ui_egui / TitleBar / MacOs` an explicit part of `title_bar` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE 72ced9a9e4dd28a4:10: Bounds the `title_bar` implementation here so `redrob_launcher_ui_egui / TitleBar / MacOs` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f6d344c6c1887114:28: Bounds the `title_bar` implementation here so `width / in` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE e6fcb2b39ed6b1e9:1: Applies `title_bar` consistently to every `width / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 762e73d5376d5784:1: Freezes `backend` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 75300da55cf470b7:1: Freezes `h` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e9feed09ce5dbb0f:2: Makes `move / ui / state` an explicit part of `title_bar` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE cba388f6060a09d1:2: Prevents `title_bar` from applying its side effect outside the `state / is_none` state.
// ADDED-LINE-RATIONALE a1ce0f7cf3530d4c:2: Delegates `title_bar`'s `b / backend / take` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33596c30f672d29e:2: Bounds the `title_bar` implementation here so `b / backend / take` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 292e0982443c6044:2: Delegates `title_bar`'s `state / LauncherUi / new` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33596e30f672d604:3: Supports `title_bar` while preserving the nearby constraint: state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));.
// ADDED-LINE-RATIONALE ecb0b7120910b357:2: Freezes `if` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1c604d77412e2c02:1: Delegates `title_bar`'s `l / logic / ui` work to `l.logic` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE d888d519781a3e64:1: Delegates `title_bar`'s `l / ui / ui` work to `l.ui` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33596e30f672d604:4: Bounds the `title_bar` implementation here so `l / ui / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cb04837595f96c08:2: Keeps `l / ui / ui` in `title_bar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE aaa77f1080413041:2: Keeps `h / run_steps` in `title_bar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE f1f3f2afe3363369:2: Bounds the `title_bar` implementation here so `h / run_steps` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 84c6161156ee9242:4: Delegates `title_bar`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 198ff50f267e01ad:1: Freezes `if` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 1b8f047439c22d9f:1: Delegates `title_bar`'s `snapshot / h / grid` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 72ced9a9e4dd28a4:11: Bounds the `title_bar` implementation here so `snapshot / h / grid` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 6145026e4f9a4972:1: Supports `title_bar` while preserving the nearby constraint: The OS draws the traffic lights: no window buttons of ours.
// ADDED-LINE-RATIONALE 01fa986da359a697:1: Freezes `search` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e1466b88a809eace:1: Freezes `logo` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f933bc9a5d3ae7f7:1: Freezes `update` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b15a524cde2b47de:1: Supports `title_bar` while preserving the nearby constraint: Left to right: Update, the logo, the search, all in the 48 px bar and inside the window.
// ADDED-LINE-RATIONALE d1ed7ebe4dcfdd0c:1: Applies `title_bar` consistently to every `r / in / search` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 9d5f1fd134b28ad6:1: Freezes `assert` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 72ced9a9e4dd28a4:12: Bounds the `title_bar` implementation here so `assert / r / bottom` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 71c2c5aba206bd66:1: Supports `title_bar` while preserving the nearby constraint: The traffic lights' 72 px stay clear, with room to drag the window beside them.
// ADDED-LINE-RATIONALE f6d344c6c1887114:29: Bounds the `title_bar` implementation here so `assert / update / left` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:55: Bounds the `title_bar` implementation here so `assert / update / left` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 60e5730556e865da:39: Declares the `test` contract on `title_bar` so the compiler or runner applies it before execution.
// ADDED-LINE-RATIONALE 4c61a151d58a0a81:1: Supports `the_grid_shows_every_app_name_whole_at_the_default_window` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 870eb6e67c47052c:11: Freezes `let` for `the_grid_shows_every_app_name_whole_at_the_default_window` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 34a7bf953dd76132:27: Delegates `the_grid_shows_every_app_name_whole_at_the_default_window`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a5e98c77e55ca7cd:1: Supports `the_grid_shows_every_app_name_whole_at_the_default_window` while preserving the nearby constraint: lies inside the window, beside its icon.
// ADDED-LINE-RATIONALE ae7c855f6aee8282:1: Applies `the_grid_shows_every_app_name_whole_at_the_default_window` consistently to every `app / in / redrob_launcher_core` item rather than handling only the first match.
// ADDED-LINE-RATIONALE e66927ad1fce5dd8:1: Applies `the_grid_shows_every_app_name_whole_at_the_default_window` consistently to every `node / in / h` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 4d0ae258ba993101:1: Freezes `r` for `the_grid_shows_every_app_name_whole_at_the_default_window` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 5f9116ead89636c2:1: Prevents `the_grid_shows_every_app_name_whole_at_the_default_window` from applying its side effect outside the `r / top / r` state.
// ADDED-LINE-RATIONALE 4dd1039db2378c2c:1: Stops `the_grid_shows_every_app_name_whole_at_the_default_window` at the `continue / not / a` outcome so no later branch can overwrite that result.
// ADDED-LINE-RATIONALE 86686f0882678ff4:18: Bounds the `the_grid_shows_every_app_name_whole_at_the_default_window` implementation here so `continue / not / a` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cc9f6c165df221d3:1: Freezes `assert` for `the_grid_shows_every_app_name_whole_at_the_default_window` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 929cb0f73b3e33b7:1: Freezes `assert` for `the_grid_shows_every_app_name_whole_at_the_default_window` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE aa2b4ad0316a3842:1: Freezes `seen` for `the_grid_shows_every_app_name_whole_at_the_default_window` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 72ced9a9e4dd28a4:13: Bounds the `the_grid_shows_every_app_name_whole_at_the_default_window` implementation here so `seen` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f6d344c6c1887114:30: Bounds the `the_grid_shows_every_app_name_whole_at_the_default_window` implementation here so `assert / seen / the` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE a0d5737c9a2d4eef:1: Freezes `assert` for `the_grid_shows_every_app_name_whole_at_the_default_window` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 0260f6a515d6d3c4:56: Bounds the `the_grid_shows_every_app_name_whole_at_the_default_window` implementation here so `assert / seen / the` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 759d157f50a6621d:1: Freezes `frameless` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 762e73d5376d5784:2: Freezes `backend` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE b406ac24947466e7:1: Delegates `title_bar`'s `Harness / builder / with_size` work to `Harness::builder` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE e9feed09ce5dbb0f:3: Makes `move / ui / state` an explicit part of `title_bar` because the regression suite can pin the refreshed behavior before release.
// ADDED-LINE-RATIONALE cba388f6060a09d1:3: Prevents `title_bar` from applying its side effect outside the `state / is_none` state.
// ADDED-LINE-RATIONALE a1ce0f7cf3530d4c:3: Delegates `title_bar`'s `b / backend / take` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33596c30f672d29e:3: Bounds the `title_bar` implementation here so `b / backend / take` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 292e0982443c6044:3: Delegates `title_bar`'s `state / LauncherUi / new` work to `Some` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33596e30f672d604:5: Supports `title_bar` while preserving the nearby constraint: state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));.
// ADDED-LINE-RATIONALE ecb0b7120910b357:3: Freezes `if` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 2cde6b5ce85679ed:1: Delegates `title_bar`'s `l / logic / ui` work to `l.logic` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 148b17d745c3374b:1: Delegates `title_bar`'s `l / ui / ui` work to `l.ui` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 33596e30f672d604:6: Bounds the `title_bar` implementation here so `l / ui / ui` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE cb04837595f96c08:3: Keeps `l / ui / ui` in `title_bar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE aaa77f1080413041:3: Keeps `l / ui / ui` in `title_bar`'s API-defined order so positional values retain their intended meaning.
// ADDED-LINE-RATIONALE 72cea5a9e4dcd048:1: Bounds the `title_bar` implementation here so `Narrow / windows / shrink` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE f16f0cbadad852dd:3: Bounds the `title_bar` implementation here so `Narrow / windows / shrink` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 654c4f70efc652e9:1: Supports `title_bar` while preserving the nearby constraint: room to drag left of Update.
// ADDED-LINE-RATIONALE a6d9b401ff045aaf:1: Freezes `h` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 84c6161156ee9242:5: Delegates `title_bar`'s `h / run_steps` work to `h.run_steps` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE b9aa1bae629bcf3b:1: Delegates `title_bar`'s `snapshot / h / format` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 687a0bab75de327f:1: Freezes `minimize` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 01fa986da359a697:2: Freezes `search` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f933bc9a5d3ae7f7:2: Freezes `update` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 94c6382a70fa5dad:1: Freezes `assert` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE fb7bfbe33bda9a70:1: Freezes `assert` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f6d344c6c1887114:32: Bounds the `title_bar` implementation here so `assert / update / left` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 8b93a101bd594d25:1: Freezes `h` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 665317b85b4a8d2f:1: Supports `title_bar` while preserving the nearby constraint: The custom bar, right to left: window buttons, search, the group logo, Update.
// ADDED-LINE-RATIONALE 67d91c521ddd9187:2: Freezes `search` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e577ae42924b161e:2: Freezes `logo` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 549f838788c28087:2: Freezes `update` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 05219362125d448d:1: Freezes `assert` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 9d4133968b6a3b1c:2: Supports `title_bar` while preserving the nearby constraint: One row, inside the caption buttons' band (whose target starts under the 5 px resize edge).
// ADDED-LINE-RATIONALE 6e02bcac70f9fdd1:1: Freezes `assert` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f6d344c6c1887114:33: Bounds the `title_bar` implementation here so `assert / r / top` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE 3e888c3e376fbf42:1: Supports `title_bar` while preserving the nearby constraint: Check for updates moved down to the filter row.
// ADDED-LINE-RATIONALE e86db66b7f8c6c1c:1: Freezes `assert` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE e530f167e358e332:1: Freezes `restored_close` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE cf1ccd5720b857dd:1: Freezes `bar_top` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 42512005104aa2b7:1: Makes `title_bar` fail visibly when `assert / restored_close / top` violates the behavior this path promises.
// ADDED-LINE-RATIONALE b991b6ee2af22ccc:2: Supports `title_bar` while preserving the nearby constraint: right edge, so flinging the pointer into the corner lands on Close (Windows' own buttons).
// ADDED-LINE-RATIONALE 13557e3585e896c5:1: Freezes `h` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 37ebcf81b9632d8b:2: Delegates `title_bar`'s `h / step` work to `h.step` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f6d344c6c1887114:34: Bounds the `title_bar` implementation here so `h / step` cannot leak into the adjacent declaration or control path.
// ADDED-LINE-RATIONALE baedeeb6efefe10f:1: Freezes `close` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 902ceb6860396e37:1: Freezes `restore` for `title_bar` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE 09690a2e6ae150be:1: Makes `title_bar` fail visibly when `assert / close / top` violates the behavior this path promises.
// ADDED-LINE-RATIONALE ff90d832d77c4689:1: Makes `title_bar` fail visibly when `assert / restore / top` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 62ddd389308e0886:1: Delegates `title_bar`'s `h / get_by_label / Minimize` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE f171168a0935da1c:1: Delegates `title_bar`'s `h / get_by_label / Restore` work to `h.get_by_label` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE 4aec38ae6b1ea724:1: Supports `settings_say_when_github_can_be_asked_again_and_the_platform` while preserving the nearby constraint: [test].
// ADDED-LINE-RATIONALE 29f5a2730a46b66c:1: Applies `settings_say_when_github_can_be_asked_again_and_the_platform` consistently to every `width / in` item rather than handling only the first match.
// ADDED-LINE-RATIONALE 11d06fa99502e826:1: Supports `settings_say_when_github_can_be_asked_again_and_the_platform` while preserving the nearby constraint: The Apps page says none of it: the footer is gone.
// ADDED-LINE-RATIONALE 7fb0adef19c2c813:1: Makes `settings_say_when_github_can_be_asked_again_and_the_platform` fail visibly when `assert / h / query_by_label_contains` violates the behavior this path promises.
// ADDED-LINE-RATIONALE 435ff2054aa61181:2: Delegates `settings_say_when_github_can_be_asked_again_and_the_platform`'s `open_settings / h` work to `open_settings` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE a3c72fbee6f74d64:1: Delegates `settings_say_when_github_can_be_asked_again_and_the_platform`'s `snapshot / h / format` work to `snapshot` so existing state and error handling remain centralized.
// ADDED-LINE-RATIONALE dfb24a7ea0b25d96:1: Supports `settings_say_when_github_can_be_asked_again_and_the_platform` while preserving the nearby constraint: Whole, wrapped if it must, and inside the window.
// ADDED-LINE-RATIONALE 0ab1e400dc5e3a6b:1: Freezes `assert` for `settings_say_when_github_can_be_asked_again_and_the_platform` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE f0e7f440bb3c1655:1: Freezes `checked` for `settings_say_when_github_can_be_asked_again_and_the_platform` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE a4fc5c2b2bc9cd4a:1: Freezes `platform` for `settings_say_when_github_can_be_asked_again_and_the_platform` so every later calculation in this path observes the same input.
// ADDED-LINE-RATIONALE d51836b583ea244b:1: Supports `settings_say_when_github_can_be_asked_again_and_the_platform` while preserving the nearby constraint: Read top to bottom: the check, the limit, the platform.
//! Drives the launcher UI against a fake backend: what's shown for each status, and which
//! commands the buttons send. `REDROB_LAUNCHER_SNAPSHOTS=<dir>` also saves screenshots; run that
//! with `-- --test-threads=1`, as rendering on several threads at once can crash Vulkan loaders
//! older than 1.4.350 (KhronosGroup/Vulkan-Loader#1863).

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use redrob_launcher_core::protocol::{
    Backend, CANCELLED, Capabilities, Command, Event, INSTALL_CANCELLED, JobPhase, LatestRelease, Origin, PausedDownload, Source,
};
use redrob_launcher_core::release::{Asset, Release};
use redrob_launcher_core::status::InstallSource;
use redrob_launcher_core::{Arch, Installed, Os, Platform, SUITE, version};
use redrob_launcher_ui_egui::{Host, LauncherUi, Settings};

const NOW: i64 = 1_791_500_000;

struct Fake {
    events: Vec<Event>,
    sent: Rc<RefCell<Vec<Command>>>,
    manage: bool,
    clock: Rc<Cell<i64>>,
}

impl Backend for Fake {
    fn send(&mut self, command: Command) {
        self.sent.borrow_mut().push(command);
    }
    fn poll(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }
    fn capabilities(&self) -> Capabilities {
        Capabilities { manage_apps: self.manage, reveal_label: "Show in Finder", self_update: self.manage }
    }
    fn platform(&self) -> Option<Platform> {
        Some(Platform { os: Os::MacOs, arch: Arch::Arm64 })
    }
    fn now(&self) -> i64 {
        self.clock.get()
    }
}

struct NoHost;
impl Host for NoHost {}

fn release(id: &str, v: &str, at: i64) -> Release {
    let name = format!("{id}-{v}-macos-universal.dmg");
    Release {
        tag: format!("v{v}"),
        version: version::parse(v),
        name: format!("v{v}"),
        published_at: Some(at),
        html_url: format!("https://github.com/storytold/{id}/releases/tag/v{v}"),
        notes: "## What's Changed\n* Faster things by @a in https://github.com/storytold/x/pull/1".into(),
        prerelease: false,
        assets: vec![Asset {
            name: name.clone(),
            size: 150 << 20,
            url: format!("https://github.com/storytold/{id}/releases/download/v{v}/{name}"),
            sha256: None,
        }],
        manifest: None,
    }
}

/// A realistic mix: updates, up-to-date apps, a download in progress, one app with no release.
fn events() -> Vec<Event> {
    let mut ev = Vec::new();
    let versions = [
        ("redrob-canvas", "0.5.0"),
        ("redrob-shape", "0.7.0"),
        ("redrob-film", "0.4.0"),
        ("redrob-office", "0.4.0"),
        ("redrob-browser", "0.6.0"),
        ("redrob-code", "0.3.0"),
        ("redrob-cowork", "0.3.0"),
        ("redrob-cad", "0.3.0"),
    ];
    for (i, (id, v)) in versions.iter().enumerate() {
        ev.push(Event::Latest {
            app: (*id).into(),
            latest: LatestRelease::Found(release(id, v, NOW - 86_400 - i as i64 * 600)),
            from: Origin::RedrobApi,
        });
    }
    ev.push(Event::Latest { app: "redrob-design".into(), latest: LatestRelease::None, from: Origin::RedrobApi });
    let installed = [("redrob-canvas", "0.4.1"), ("redrob-code", "0.3.0"), ("redrob-browser", "0.5.0")];
    for app in &SUITE {
        let inst = installed.iter().find(|(id, _)| *id == app.id).map(|(_, v)| Installed {
            version: version::parse(v),
            source: InstallSource::MacApp,
            location: format!("/Applications/{}.app", app.name),
            package: None,
        });
        ev.push(Event::Detected { app: app.id.into(), installed: inst });
    }
    ev.push(Event::Progress { app: "redrob-film".into(), phase: JobPhase::Downloading { done: 48 << 20, total: 131 << 20 } });
    ev.push(Event::Paused { app: "redrob-cowork".into(), download: Some(PausedDownload { done: 30 << 20, total: 120 << 20 }) });
    ev.push(Event::Progress { app: "redrob-office".into(), phase: JobPhase::Reconnecting { done: 10 << 20, total: 90 << 20 } });
    ev.push(Event::ReadyToInstall { app: "redrob-cad".into(), ready: true });
    ev.push(Event::CheckFinished { at: NOW - 180, source: Source::GitHubApi, rate_limited_until: None });
    ev
}

fn harness(theme: egui::Theme, manage: bool) -> (Harness<'static, Option<LauncherUi>>, Rc<RefCell<Vec<Command>>>) {
    harness_at(theme, manage, 1.0)
}

fn harness_at(theme: egui::Theme, manage: bool, scale: f32) -> (Harness<'static, Option<LauncherUi>>, Rc<RefCell<Vec<Command>>>) {
    harness_with(events(), theme, manage, scale)
}

fn harness_with(
    events: Vec<Event>,
    theme: egui::Theme,
    manage: bool,
    scale: f32,
) -> (Harness<'static, Option<LauncherUi>>, Rc<RefCell<Vec<Command>>>) {
    // Integration tests compile without cfg!(test), so system_lang() picks up the
    // developer's locale. Force English so labels match the assertions.
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let sent = Rc::new(RefCell::new(Vec::new()));
    let sent2 = sent.clone();
    let mut backend = Some(Fake { events, sent: sent2, manage, clock: Rc::new(Cell::new(NOW)) });
    let settings = Settings { notify: false, ..Settings::default() };
    // No `.wgpu()`: it gives every test its own GPU device, and those segfault on parallel test threads (see above).
    // The default renderer creates one only when a test renders.
    let harness = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).with_pixels_per_point(scale).with_theme(theme).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                let mut s = settings.clone();
                s.theme = if theme == egui::Theme::Dark {
                    redrob_launcher_ui_egui::theme::ThemeChoice::Dark
                } else {
                    redrob_launcher_ui_egui::theme::ThemeChoice::Light
                };
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), s));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    (harness, sent)
}

fn snapshot(h: &mut Harness<'_, Option<LauncherUi>>, name: &str) {
    let Some(dir) = std::env::var_os("REDROB_LAUNCHER_SNAPSHOTS") else { return };
    let img = h.render().unwrap();
    img.save(std::path::Path::new(&dir).join(format!("{name}.png"))).unwrap();
}

#[test]
fn keeps_working_with_no_window_then_draws_in_a_new_one() {
    let sent = Rc::new(RefCell::new(Vec::new()));
    let backend = Fake { events: events(), sent: sent.clone(), manage: true, clock: Rc::new(Cell::new(NOW)) };
    let windowless = egui::Context::default();
    let mut ui = LauncherUi::new(&windowless, Box::new(backend), Settings { notify: false, ..Settings::default() });
    // No window: the backend's news is still taken in.
    ui.logic(&windowless, &mut NoHost);
    assert!(sent.borrow().contains(&Command::CheckForUpdates { force: false }));
    assert_eq!(ui.update_count(), 2);
    assert_eq!(ui.settings.last_check, Some(NOW - 180));

    let mut attached = false;
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |egui_ui, state: &mut LauncherUi| {
            if !attached {
                attached = true;
                state.attach(egui_ui.ctx());
            }
            state.logic(&egui_ui.ctx().clone(), &mut NoHost);
            state.ui(egui_ui, &mut NoHost);
        },
        ui,
    );
    h.run_steps(4);
    h.get_by_label("Update to 0.5.0");
    h.get_by_label("Update to 0.6.0");
}

#[test]
fn grid_shows_every_status_and_installs_on_click() {
    let (mut h, sent) = harness(egui::Theme::Dark, true);
    h.run_steps(8);
    snapshot(&mut h, "grid-dark");
    // Statuses the user asked for: available, installed and up to date, update with version. No
    // chip draws them any more; each card carries its status for screen readers.
    assert!(h.query_all_by_label_contains(": Available to install").count() >= 2);
    assert!(h.query_all_by_label_contains(": Installed · up to date").count() >= 1);
    // Named by app as well as status, so two cards in the same state never share a name.
    h.get_by_label("Redrob Shape: Available to install");
    // No links are inline; each visible card has a named menu trigger in addition to right-click.
    assert!(h.query_by_label("GitHub").is_none() && h.query_by_label("Website").is_none(), "cards carry no links");
    // Derive the expectation from rendered app cards so this assertion follows fixture changes.
    let visible_cards = SUITE.iter().filter(|app| h.query_by_label_contains(&format!("{}:", app.name)).is_some()).count();
    assert_eq!(h.query_all_by_label("More actions").count(), visible_cards, "one … button on every visible app card");
    h.get_by_label("Update to 0.5.0");
    h.get_by_label("Update to 0.6.0");
    h.get_by_label_contains("0.4.1 installed → 0.5.0 released");
    h.get_by_label("Graphics");
    // The window has no title bar of its own here (the web build, and the tests).
    assert!(h.query_by_label("Close").is_none());
    // No upstream community links, social icons or trademark notice.
    assert!(h.query_all_by_label_contains("Discord").next().is_none());
    assert!(h.query_all_by_label_contains("ArtCraft").next().is_none());
    assert!(h.query_all_by_label("Coming soon").count() >= 1);
    h.get_by_label_contains("48 MB of 131 MB");
    h.get_by_label("Update all (2)").click();
    h.run_steps(2);
    let installs: Vec<_> = sent.borrow().iter().filter_map(|c| if let Command::Install { app } = c { Some(app.clone()) } else { None }).collect();
    assert_eq!(installs.len(), 2, "{installs:?}");
    assert!(installs.contains(&"redrob-canvas".to_string()) && installs.contains(&"redrob-browser".to_string()));
}

#[test]
fn downloads_can_be_paused_resumed_and_discarded() {
    // The fixture's reconnecting download is Redrob Office's.
    let (mut h, sent) = harness_with(events(), egui::Theme::Dark, true, 1.0);
    h.run_steps(4);
    // Bring the paused and reconnecting apps on screen (the sidebar leaves the grid three
    // columns, so the third row needs a taller window).
    if let Some(l) = h.state_mut().as_mut() {
        l.settings.filter = redrob_launcher_ui_egui::settings::Filter::NotInstalled;
    }
    h.set_size(egui::vec2(1180.0, 1100.0));
    h.run_steps(6);
    snapshot(&mut h, "grid-downloads-dark");
    h.get_by_label_contains(": Download paused");
    // The badge, and in a narrow tile the progress caption too.
    assert!(h.query_all_by_label("Reconnecting…").count() >= 1);
    h.get_by_label_contains("· 25%").click();
    h.run_steps(2);
    assert!(sent.borrow().contains(&Command::Install { app: "redrob-cowork".into() }));
    h.get_by_label("Discard download").click();
    h.run_steps(2);
    assert!(sent.borrow().contains(&Command::Cancel { app: "redrob-cowork".into() }));
    h.get_all_by_label("Pause download").next().unwrap().click();
    h.run_steps(2);
    assert!(sent.borrow().iter().any(|c| matches!(c, Command::Pause { .. })));
}

#[test]
fn a_kept_download_offers_retry_install_and_download_again() {
    let (mut h, sent) = harness(egui::Theme::Dark, true);
    h.run_steps(4);
    if let Some(l) = h.state_mut().as_mut() {
        l.settings.filter = redrob_launcher_ui_egui::settings::Filter::NotInstalled;
    }
    h.run_steps(6);
    // Below the fold in the restyled grid: bring the card on screen first.
    h.get_by_label("Retry install").scroll_to_me();
    h.run_steps(4);
    snapshot(&mut h, "grid-retry-install-dark");
    sent.borrow_mut().clear();
    let retry = h.get_by_label("Retry install");
    let row = retry.rect();
    retry.click();
    h.run_steps(2);
    assert_eq!(*sent.borrow(), [Command::Install { app: "redrob-cad".into() }]);
    sent.borrow_mut().clear();
    // The named button beside this uninstalled app exposes the same command as its context menu.
    let beside = |r: egui::Rect| row.y_range().contains(r.center().y) && (row.right()..row.right() + 60.0).contains(&r.left());
    assert!(h.query_all_by_label("More actions").any(|n| beside(n.rect())), "… beside Retry install");
    open_card_more_button(&mut h, "Redrob CAD");
    h.get_by_label("Download again");
    h.key_press(egui::Key::Escape);
    h.run_steps(2);
    // The card's own controls sit over the card and keep the pointer: right-clicking the button
    // neither opens the card's menu nor presses it.
    h.get_by_label("Retry install").click_secondary();
    h.run_steps(3);
    assert!(h.query_by_label("Download again").is_none(), "a right-click on a button is the button's");
    assert!(sent.borrow().is_empty(), "{:?}", sent.borrow());
    open_card_menu(&mut h, "Redrob CAD");
    snapshot(&mut h, "grid-retry-install-menu-dark");
    h.get_by_label("Download again").click();
    h.run_steps(2);
    assert_eq!(*sent.borrow(), [Command::Cancel { app: "redrob-cad".into() }, Command::Install { app: "redrob-cad".into() }]);
    // Apps without a kept download don't offer it.
    assert_eq!(h.query_all_by_label("Retry install").count(), 1);
}

#[test]
fn an_uninstalled_card_has_keyboard_accessible_more_actions() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(4);
    if let Some(l) = h.state_mut().as_mut() {
        // Limit the grid so the focused control belongs to an unequivocally uninstalled app.
        l.settings.filter = redrob_launcher_ui_egui::settings::Filter::NotInstalled;
    }
    h.get_all_by_label("Redrob Shape").next().unwrap().scroll_to_me();
    h.run_steps(4);
    let name = h.get_all_by_label("Redrob Shape").next().unwrap().rect();
    // Query by role as well as name to lock in the control's assistive-technology semantics.
    let menu = h
        .query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::Button).label("More actions"))
        .filter(|node| {
            let rect = node.rect();
            rect.left() > name.left() && rect.top() > name.bottom() && rect.top() < name.top() + 180.0
        })
        .min_by(|a, b| a.rect().left().total_cmp(&b.rect().left()))
        .unwrap();
    menu.focus();
    h.key_press(egui::Key::Enter);
    h.run_steps(3);
    h.get_by_label("Release notes");
}

#[test]
fn every_job_state_keeps_named_more_actions() {
    let mut ev = events();
    // Put every active phase on a visible app; the base fixture already supplies Downloading and
    // Paused, while these overrides cover the phases that replace all ordinary action buttons.
    for (app, phase) in [
        ("redrob-office", JobPhase::Reconnecting { done: 10 << 20, total: 90 << 20 }),
        ("redrob-canvas", JobPhase::Verifying),
        ("redrob-shape", JobPhase::Waiting),
        ("redrob-browser", JobPhase::Installing),
        ("redrob-code", JobPhase::Removing),
    ] {
        ev.push(Event::Progress { app: app.into(), phase });
    }
    let (mut h, _) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    // Opening each button proves progress-only, cancellable, and paused rows retain a named path
    // to the card menu rather than relying on an undiscoverable secondary click.
    for app in ["Redrob Film", "Redrob Office", "Redrob Canvas", "Redrob Shape", "Redrob Browser", "Redrob Code", "Redrob Cowork"] {
        open_card_more_button(&mut h, app);
        h.get_by_label("Release notes");
        h.key_press(egui::Key::Escape);
        h.run_steps(2);
    }
}

#[test]
fn a_dismissed_install_prompt_is_not_called_a_cancelled_download() {
    let finished = |app: &str, why: &str| Event::JobFinished { app: app.into(), result: Err(why.into()) };
    // The password prompt was dismissed with the download kept (redrob-cad) and without (redrob-office);
    // a download was cancelled (redrob-shape).
    let mut ev = events();
    ev.push(finished("redrob-cad", INSTALL_CANCELLED));
    ev.push(finished("redrob-office", INSTALL_CANCELLED));
    ev.push(finished("redrob-shape", CANCELLED));
    let (mut h, _) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-install-cancelled-dark");
    for toast in ["Install cancelled. The download is kept for the next try.", "Install cancelled", "Download cancelled"] {
        assert_eq!(h.query_all_by_label(toast).count(), 1, "{toast}");
    }
}

#[test]
fn a_kept_update_names_its_version_and_an_up_to_date_app_is_not_downloaded_again() {
    let mut ev = events();
    for app in ["redrob-browser", "redrob-code"] {
        ev.push(Event::ReadyToInstall { app: app.into(), ready: true });
    }
    let (mut h, sent) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(8);
    h.get_by_label("Retry update to 0.6.0").scroll_to_me();
    h.run_steps(4);
    snapshot(&mut h, "grid-retry-update-dark");
    sent.borrow_mut().clear();
    assert!(h.query_by_label("Update to 0.6.0").is_none());
    h.get_by_label("Retry update to 0.6.0").click();
    h.run_steps(2);
    assert_eq!(*sent.borrow(), [Command::Install { app: "redrob-browser".into() }]);
    // Redrob Code is up to date: its … button opens the menu, which offers Reinstall (it uses the
    // kept file).
    open_card_more_button(&mut h, "Redrob Code");
    h.get_by_label("Reinstall");
    assert!(h.query_by_label("Download again").is_none());
}

#[test]
fn an_install_waiting_for_another_says_so_and_can_be_cancelled() {
    let mut events = events();
    // Two updates reached the system's installer together: one runs, the other waits its turn.
    events.push(Event::Progress { app: "redrob-canvas".into(), phase: JobPhase::Installing });
    events.push(Event::Progress { app: "redrob-shape".into(), phase: JobPhase::Waiting });
    let (mut h, sent) = harness_with(events, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-waiting-dark");
    h.get_by_label("Installing…");
    h.get_by_label("Waiting for another app…");
    // Only the waiting one: the install that is running can't be called off. (Redrob Shape is on
    // the third shelf now: bring it on screen first.)
    h.get_by_label("Cancel").scroll_to_me();
    h.run_steps(4);
    h.get_by_label("Cancel").click();
    h.run_steps(2);
    let cancels: Vec<_> = sent.borrow().iter().filter(|c| matches!(c, Command::Cancel { .. })).cloned().collect();
    assert_eq!(cancels, [Command::Cancel { app: "redrob-shape".into() }]);
}

#[test]
fn the_launcher_update_waiting_for_other_installs_can_be_cancelled() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    ev.push(Event::Progress { app: "redrob-launcher".into(), phase: JobPhase::Waiting });
    let (mut h, sent) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-launcher-waiting-dark");
    h.get_by_label("Waiting for other installs…");
    sent.borrow_mut().clear();
    h.get_by_label("Cancel update").click();
    h.run_steps(2);
    assert_eq!(*sent.borrow(), [Command::Cancel { app: "redrob-launcher".into() }]);
}

#[test]
fn each_cancellation_says_what_it_called_off() {
    let finished = |app: &str, why: &str| Event::JobFinished { app: app.into(), result: Err(why.into()) };
    let mut ev = events();
    // An uninstall called off while it waited for the installer.
    ev.push(Event::Progress { app: "redrob-film".into(), phase: JobPhase::Removing });
    ev.push(Event::Progress { app: "redrob-film".into(), phase: JobPhase::Waiting });
    ev.push(finished("redrob-film", CANCELLED));
    // The launcher's own update, cancelled while it waited.
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    ev.push(Event::Progress { app: "redrob-launcher".into(), phase: JobPhase::Waiting });
    ev.push(finished("redrob-launcher", CANCELLED));
    // A download cancelled mid-way.
    ev.push(finished("redrob-film", CANCELLED));
    let (mut h, _) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    for toast in ["Uninstall cancelled", "Update cancelled", "Download cancelled"] {
        assert_eq!(h.query_all_by_label(toast).count(), 1, "{toast}");
    }
}

#[test]
fn a_search_that_matches_nothing_says_what_to_relax_and_clears() {
    let (mut h, _) = harness(egui::Theme::Light, true);
    h.run_steps(6);
    let field = h.query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::TextInput).label("Search apps")).next().unwrap();
    field.focus();
    field.type_text("zzz");
    h.run_steps(4);
    snapshot(&mut h, "empty-search-light");
    // A DS empty state: what happened, what to relax, and the way back.
    h.get_by_label("No apps match");
    h.get_by_label_contains("Nothing matches");
    assert!(h.query_by_label("Redrob Canvas").is_none());
    h.get_by_label("Clear search").click();
    h.run_steps(4);
    assert!(h.query_by_label("No apps match").is_none());
    h.get_by_label("Redrob Canvas");
}

#[test]
fn switching_filters() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    // The grid is the only view: no Grid / Table control, no sort.
    assert!(h.query_by_label("Table").is_none() && h.query_by_label("Grid").is_none());
    assert!(h.query_by_label("Sort").is_none());
    h.get_by_label_contains("Updates").click();
    h.run_steps(4);
    snapshot(&mut h, "grid-updates-dark");
    assert!(h.query_by_label("Redrob Code").is_none());
    h.get_by_label("Redrob Canvas");
}

/// Clicks the sidebar's account row labelled `row`, then the item `item` of the menu it opens
/// above itself.
fn from_account_menu(h: &mut Harness<'_, Option<LauncherUi>>, row: &str, item: &str) {
    let row_rect = h.get_by_label(row).rect();
    h.get_by_label(row).click();
    h.run_steps(4);
    {
        // The menu's item: the nearest one above the row (a page title can share its label).
        let node = h
            .get_all_by_label(item)
            .filter(|n| n.rect().bottom() <= row_rect.top() + 0.5)
            .max_by(|a, b| a.rect().bottom().total_cmp(&b.rect().bottom()))
            .unwrap();
        node.click();
    }
    h.run_steps(4);
}

/// Opens the Settings page: the sidebar's account row (it reads "Settings" with no account
/// backend) opens its menu, and the menu's Settings item opens the page.
fn open_settings(h: &mut Harness<'_, Option<LauncherUi>>) {
    from_account_menu(h, "Settings", "Settings");
}

/// Opens `app`'s menu by right-clicking its tile (on its name: the card under the label takes the
/// click). Every tile also has the named … button, including read-only and job states.
fn open_card_menu(h: &mut Harness<'_, Option<LauncherUi>>, app: &str) {
    h.get_all_by_label(app).next().unwrap().scroll_to_me();
    h.run_steps(4);
    h.get_all_by_label(app).next().unwrap().click_secondary();
    h.run_steps(3);
}

/// Opens `app`'s menu with its … button: the nearest one right of its name and under it, in the
/// same tile.
fn open_card_more_button(h: &mut Harness<'_, Option<LauncherUi>>, app: &str) {
    h.get_all_by_label(app).next().unwrap().scroll_to_me();
    h.run_steps(4);
    {
        let name = h.get_all_by_label(app).next().unwrap().rect();
        let menu = h
            .get_all_by_label("More actions")
            .filter(|n| {
                let r = n.rect();
                r.left() > name.left() && r.top() > name.bottom() && r.top() < name.top() + 180.0
            })
            .min_by(|a, b| a.rect().left().total_cmp(&b.rect().left()))
            .unwrap();
        menu.click();
    }
    h.run_steps(3);
}

#[test]
fn an_old_table_or_sort_setting_still_loads_and_the_grid_ignores_it() {
    use redrob_launcher_ui_egui::settings::{SortKey, ViewMode};
    // Saved by an earlier version, when there was a table and a sort.
    let old: Settings = serde_json::from_str(r#"{"view":"Table","sort":"Name","sort_descending":true,"notify":false}"#).unwrap();
    assert_eq!((old.view, old.sort, old.sort_descending), (ViewMode::Table, SortKey::Name, true));
    let mut backend = Some(Fake { events: events(), sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), old.clone()));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    h.run_steps(6);
    // The grid, on its three shelves in page order: the AI engine, everyday work, creative work.
    let engine = h.get_by_label("For your AI engine").rect();
    let work = h.get_by_label("For your everyday work").rect();
    let creative = h.get_by_label("For your creative work").rect();
    assert!(engine.top() < work.top() && work.top() < creative.top());
    assert!(h.query_by_label("Kind").is_none(), "no table header");
    let top = |name: &str| h.get_all_by_label(name).next().unwrap().rect().top();
    assert!(top("Redrob Code") > engine.bottom() && top("Redrob Code") < work.top());
    assert!(top("Redrob Cowork") > work.bottom() && top("Redrob Browser") < creative.top());
    assert!(top("Redrob Design") > creative.bottom());
}

/// How far the middle of these lines of text is below the middle of the app icon beside them, in
/// points. The lines also share one left edge, just right of the icon.
fn below_icon_centre(h: &Harness<'_, Option<LauncherUi>>, lines: &[&str]) -> f32 {
    let rects: Vec<egui::Rect> = lines.iter().map(|l| h.get_by_label(l).rect()).collect();
    let left = rects[0].left();
    for (l, r) in lines.iter().zip(&rects) {
        assert!((r.left() - left).abs() < 0.5, "{l} is not left-aligned with {}: {} vs {left}", lines[0], r.left());
    }
    let text = rects.iter().copied().reduce(|a, b| a.union(b)).unwrap();
    let icon = h
        .query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::Image))
        .map(|n| n.rect())
        .filter(|r| r.right() <= text.left() && r.top() < text.bottom() && r.bottom() > text.top())
        .max_by(|a, b| a.right().total_cmp(&b.right()))
        .unwrap();
    assert!(text.left() - icon.right() < 24.0, "no icon beside {lines:?}");
    assert!(text.left() > icon.right(), "{lines:?} runs into the icon");
    text.center().y - icon.center().y
}

#[test]
fn name_and_tagline_are_left_aligned_beside_the_icon_and_centred_on_it() {
    for (scale, suffix) in [(1.0, "-1x"), (1.5, "-1.5x"), (2.0, "-2x")] {
        let (mut h, _) = harness_at(egui::Theme::Dark, true, scale);
        h.run_steps(8);
        snapshot(&mut h, &format!("grid-dark{suffix}"));
        let off = below_icon_centre(&h, &["Redrob Canvas", "Paint and edit images with an agent", "Graphics"]);
        assert!(off.abs() < 0.25, "grid at {scale}x: {off}");
        let (mut h, _) = harness_at(egui::Theme::Light, true, scale);
        h.run_steps(8);
        snapshot(&mut h, &format!("grid-light{suffix}"));
        let off = below_icon_centre(&h, &["Redrob Canvas", "Paint and edit images with an agent", "Graphics"]);
        assert!(off.abs() < 0.25, "grid (light) at {scale}x: {off}");
    }
}

#[test]
fn pages_lead_back_to_apps_by_the_back_button_and_escape() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    let on_settings = |h: &Harness<'_, Option<LauncherUi>>| h.query_by_label("News and source code for every Redrob app").is_some();
    // Settings is a page: no Done button, one primary at most (Website is secondary now).
    open_settings(&mut h);
    assert!(on_settings(&h));
    assert!(h.query_by_label("Done").is_none());
    h.get_by_label("Quit Redrob Launcher");
    // The top bar's back button.
    h.get_by_label("Back to apps").click();
    h.run_steps(4);
    assert!(!on_settings(&h));
    h.get_by_label("Update to 0.5.0");
    assert!(h.query_by_label("Back to apps").is_none(), "Apps is where back leads; it has no back button");
    // Escape.
    open_settings(&mut h);
    assert!(on_settings(&h));
    h.key_press(egui::Key::Escape);
    h.run_steps(4);
    assert!(!on_settings(&h));
    h.get_by_label("Update to 0.5.0");
}

#[test]
fn escape_leaves_a_page_from_a_focused_button_but_not_from_an_open_list() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    let on_settings = |h: &Harness<'_, Option<LauncherUi>>| h.query_by_label("News and source code for every Redrob app").is_some();
    // Keyboard focus on an ordinary button (as after Tab) doesn't swallow Escape.
    open_settings(&mut h);
    h.get_by_label("Quit Redrob Launcher").focus();
    h.run_steps(2);
    assert!(h.ctx.memory(|m| m.focused().is_some()), "the button holds keyboard focus");
    h.key_press(egui::Key::Escape);
    h.run_steps(4);
    assert!(!on_settings(&h), "Escape from a focused button returns to Apps");
    // An open list takes Escape for itself: it closes, and the page stays.
    open_settings(&mut h);
    h.get_by_value("Automatically").click();
    h.run_steps(4);
    assert!(egui::Popup::is_any_open(&h.ctx), "the check-for-updates list is open");
    h.key_press(egui::Key::Escape);
    h.run_steps(4);
    assert!(on_settings(&h), "Escape closed the list, not the page");
}

#[test]
fn the_theme_switch_options_are_big_enough_to_hit_and_switch_the_theme() {
    for width in [1180.0, 600.0] {
        let mut h = harness_sized(width);
        h.run_steps(6);
        let options: Vec<egui::Rect> = h.query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::RadioButton)).map(|n| n.rect()).collect();
        assert_eq!(options.len(), 3, "{width}: System, Light, Dark");
        for r in &options {
            assert!(r.width() >= 32.0 && r.height() >= 32.0, "{width}: a theme option's target is only {r:?}");
        }
        // Neighbouring targets never overlap.
        for (a, b) in options.iter().zip(options.iter().skip(1)) {
            assert!(a.intersect(*b).area() <= 0.5, "{width}: {a:?} overlaps {b:?}");
        }
        h.get_by_label("Light").click();
        h.run_steps(3);
        assert_eq!(h.state().as_ref().unwrap().settings.theme, redrob_launcher_ui_egui::theme::ThemeChoice::Light, "{width}");
    }
}

/// The default harness at `width` x 900.
fn harness_sized(width: f32) -> Harness<'static, Option<LauncherUi>> {
    let mut backend = Some(Fake { events: events(), sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
    redrob_launcher_ui_egui::i18n::set_test_lang(redrob_launcher_ui_egui::i18n::Lang::EN);
    Harness::builder().with_size(egui::vec2(width, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    )
}

#[test]
fn settings_link_to_redrob() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    open_settings(&mut h);
    snapshot(&mut h, "settings-dark");
    h.get_by_label("News and source code for every Redrob app");
    assert!(h.query_all_by_label_contains("Discord").next().is_none());
}

#[test]
fn links_to_the_website_github_and_each_apps_pages() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    // The Apps page has no links: no footer, nothing on the cards.
    assert_eq!(h.query_all_by_label("GitHub").count(), 0);
    assert_eq!(h.query_all_by_label("Website").count(), 0);
    // A card's menu has the app's GitHub repo and no website entry (no app has a web page yet).
    open_card_menu(&mut h, "Redrob Code");
    h.get_by_label("GitHub repository");
    assert_eq!(h.query_all_by_label_contains("Website").count(), 0, "no app has a website page yet");
    // Redrob's own pages are in Settings.
    h.key_press(egui::Key::Escape);
    h.run_steps(3);
    open_settings(&mut h);
    h.get_by_label("Website");
    h.get_by_label("GitHub");
}

#[test]
fn the_account_menu_holds_settings_quit_and_the_version() {
    for width in [1180.0, 600.0] {
        let mut h = harness_sized(width);
        h.run_steps(6);
        // No account backend: one row reading Settings, no Settings row of its own.
        assert_eq!(h.query_all_by_label("Settings").count(), 1, "{width}");
        let row = h.get_by_label("Settings").rect();
        h.get_by_label("Settings").click();
        h.run_steps(4);
        snapshot(&mut h, &format!("account-menu-no-account-{width}"));
        let version = h.get_by_label(&format!("Redrob Launcher {}", env!("CARGO_PKG_VERSION"))).rect();
        let quit = h.get_by_label("Quit Redrob Launcher").rect();
        let settings = h.get_all_by_label("Settings").map(|n| n.rect()).find(|r| r.bottom() <= row.top() + 0.5).unwrap();
        // Above the row, in order: Settings, Quit, the version.
        assert!(settings.top() < quit.top() && quit.top() < version.top() && version.bottom() <= row.top() + 0.5, "{width}");
        assert!(h.query_by_label("Log in or sign up").is_none(), "{width}: no account item without an account backend");
        // Escape closes it.
        h.key_press(egui::Key::Escape);
        h.run_steps(4);
        assert!(h.query_by_label("Quit Redrob Launcher").is_none(), "{width}");
    }
    let mut h = harness_sized(1180.0);
    h.run_steps(6);
    open_settings(&mut h);
    h.get_by_label("News and source code for every Redrob app");
}

#[test]
fn the_logo_opens_console_redrob_ai() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    assert_eq!(click_opens(&mut h, "Go to console.redrob.ai").as_deref(), Some("https://console.redrob.ai"));
}

#[test]
fn typing_in_the_search_on_another_page_shows_the_apps() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    let on_settings = |h: &Harness<'_, Option<LauncherUi>>| h.query_by_label("News and source code for every Redrob app").is_some();
    open_settings(&mut h);
    assert!(on_settings(&h));
    // The search stays in the title bar on every page.
    let field = h
        .query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::TextInput).label("Search apps"))
        .find(|n| n.rect().bottom() <= 48.0)
        .unwrap();
    field.focus();
    field.type_text("canvas");
    h.run_steps(4);
    assert!(!on_settings(&h), "typing switched to the apps");
    h.get_by_label("Redrob Canvas");
    assert!(h.query_by_label("Redrob Shape").is_none(), "and the grid shows what it finds");
}

/// Clicks the `n`th widget labelled `label` and returns the URL the app asked to open.
fn click_nth_opens(h: &mut Harness<'_, Option<LauncherUi>>, label: &str, n: usize) -> Option<String> {
    h.get_all_by_label(label).nth(n)?.click();
    for _ in 0..3 {
        h.step();
        let url = h.output().platform_output.commands.iter().find_map(|c| match c {
            egui::OutputCommand::OpenUrl(o) => Some(o.url.clone()),
            _ => None,
        });
        if url.is_some() {
            return url;
        }
    }
    None
}

fn click_opens(h: &mut Harness<'_, Option<LauncherUi>>, label: &str) -> Option<String> {
    click_nth_opens(h, label, 0)
}

#[test]
fn links_open_the_right_pages() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    // Each app's pages, from its card's menu.
    open_card_menu(&mut h, "Redrob Canvas");
    assert_eq!(click_opens(&mut h, "GitHub repository").as_deref(), Some("https://github.com/redrob-labs/redrob-canvas"));
    open_card_menu(&mut h, "Redrob Shape");
    assert_eq!(click_opens(&mut h, "All releases").as_deref(), Some("https://github.com/redrob-labs/redrob-shape/releases"));
    // Redrob's own, from Settings.
    open_settings(&mut h);
    assert_eq!(click_opens(&mut h, "Website").as_deref(), Some("https://redrob.ai"));
    assert_eq!(click_opens(&mut h, "GitHub").as_deref(), Some("https://github.com/redrob-labs"));
}

#[test]
fn looks_again_at_installed_apps_every_few_minutes_not_every_frame() {
    let sent = Rc::new(RefCell::new(Vec::new()));
    let clock = Rc::new(Cell::new(NOW));
    let mut backend = Some(Fake { events: events(), sent: sent.clone(), manage: true, clock: clock.clone() });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    let detects = |sent: &Rc<RefCell<Vec<Command>>>| sent.borrow().iter().filter(|c| **c == Command::Detect).count();
    h.run_steps(10);
    assert_eq!(detects(&sent), 0, "the backend's own startup detection is enough");
    clock.set(NOW + 4 * 60);
    h.run_steps(10);
    assert_eq!(detects(&sent), 0, "not before five minutes");
    clock.set(NOW + 5 * 60);
    h.run_steps(10);
    assert_eq!(detects(&sent), 1, "once at five minutes, not once per frame");
}

#[test]
fn the_page_ends_24_px_from_the_edge_with_its_scroll_bar_beyond_it() {
    let (mut h, _) = harness(egui::Theme::Light, true);
    h.run_steps(6);
    // The cards (each carries its status as its name) and the refresh above them share the page's
    // right edge, 24 px in: the scroll bar sits in that padding, at the window's edge.
    // Redrob Browser ends the everyday-work shelf's row of three, so it sits in the last column.
    let right = h.query_all_by_label_contains("Redrob Browser: ").map(|n| n.rect().right()).fold(0.0, f32::max);
    let refresh = h.get_by_label("Check for updates").rect().right();
    // The harness draws inside an 8 px margin of its own, so its "window" edge is at 1172.
    assert!((right - (1180.0 - 8.0 - 24.0)).abs() <= 1.0, "cards end at {right}");
    assert!((refresh - right).abs() <= 1.0, "refresh {refresh} vs cards {right}");
    // Hovering the page shows the thumb (for the snapshot: it is painted, not a node).
    h.hover_at(egui::pos2(700.0, 500.0));
    h.run_steps(6);
    snapshot(&mut h, "grid-scroll-hover-light");
}

#[test]
fn the_launcher_update_banner_is_one_slim_line() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let (mut h, _) = harness_with(ev, egui::Theme::Light, true, 1.0);
    h.run_steps(6);
    // The prompt asks first; "Later" leaves the banner.
    h.get_by_label("Later").click();
    h.run_steps(4);
    snapshot(&mut h, "launcher-update-banner-light");
    h.get_by_label_contains("Redrob Launcher 0.2.0 is available");
    // One line: its controls are Sm (32), so the bar (12 / 6 padding) is 44 px. Under it come the
    // 12 px gap and the filter row's own 12 px inset (its 40 px row around the 30 px cells, plus
    // the pill's padding), so the cells start 68 px under the bar's top: 44 + 12 + 12. A bar even
    // 2 px taller fails (before this, a 66 px bar and a 14 px gap put the cells 92 px down).
    let dismiss = h.get_by_label("Dismiss").rect();
    let action = ["Update and restart", "Download"]
        .iter()
        .flat_map(|l| h.query_all_by_label(l).map(|n| n.rect()).collect::<Vec<_>>())
        .find(|r| (r.center().y - dismiss.center().y).abs() < 1.0)
        .unwrap();
    assert_eq!((dismiss.height(), action.height()), (32.0, 32.0));
    // The filter row's "All" cell: the first thing under the banner.
    let filters = h
        .query_all_by_label_contains("All")
        .map(|n| n.rect())
        .filter(|r| r.top() > dismiss.bottom())
        .min_by(|a, b| a.top().total_cmp(&b.top()))
        .unwrap();
    let bar_top = dismiss.top() - 6.0;
    assert!(filters.top() - bar_top <= 44.0 + 12.0 + 12.0 + 1.0, "banner from {bar_top}, filters at {filters:?}");
}

#[test]
fn the_launcher_updates_itself_and_quits_to_restart() {
    struct QuitHost(Rc<Cell<bool>>);
    impl Host for QuitHost {
        fn quit(&mut self) {
            self.0.set(true);
        }
    }
    let sent = Rc::new(RefCell::new(Vec::new()));
    let mut events = events();
    events.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let shared_events = Rc::new(RefCell::new(Vec::new()));
    struct Scripted {
        inner: Fake,
        later: Rc<RefCell<Vec<Event>>>,
    }
    impl Backend for Scripted {
        fn send(&mut self, c: Command) {
            self.inner.send(c);
        }
        fn poll(&mut self) -> Vec<Event> {
            let mut out = self.inner.poll();
            out.extend(self.later.borrow_mut().drain(..));
            out
        }
        fn capabilities(&self) -> Capabilities {
            self.inner.capabilities()
        }
        fn platform(&self) -> Option<Platform> {
            self.inner.platform()
        }
        fn now(&self) -> i64 {
            self.inner.now()
        }
    }
    let quit = Rc::new(Cell::new(false));
    let mut host = QuitHost(quit.clone());
    let mut backend =
        Some(Scripted { inner: Fake { events, sent: sent.clone(), manage: true, clock: Rc::new(Cell::new(NOW)) }, later: shared_events.clone() });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut host);
                l.ui(ui, &mut host);
            }
        },
        None,
    );
    h.run_steps(6);
    h.get_by_label_contains("Redrob Launcher 0.2.0 is available");
    h.get_by_label("Update and restart").click();
    h.run_steps(2);
    assert!(sent.borrow().contains(&Command::UpdateLauncher));
    // Downloading: the banner shows progress instead of the button.
    shared_events
        .borrow_mut()
        .push(Event::Progress { app: "redrob-launcher".into(), phase: JobPhase::Downloading { done: 5 << 20, total: 10 << 20 } });
    h.run_steps(3);
    assert!(h.query_by_label("Update and restart").is_none());
    assert!(!quit.get());
    // In place: the launcher quits so the helper can start the new one.
    shared_events.borrow_mut().push(Event::LauncherRestarting);
    h.run_steps(3);
    assert!(quit.get());
}

#[test]
fn launcher_update_prompts_to_relaunch_with_updated_version() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let (mut h, _) = harness_with(ev, egui::Theme::Dark, true, 1.0);
    h.run_steps(6);
    // Dialog pops up on launch with the update prompt
    h.get_by_label_contains("Redrob Launcher 0.2.0 is available");
    h.get_by_label("A new version is available. Would you like to relaunch with the updated version?");
    h.get_by_label("What's new");
    snapshot(&mut h, "launcher-update-dialog-dark");
    // Clicking "Later" dismisses the modal dialog
    h.get_by_label("Later").click();
    h.run_steps(3);
    assert!(h.query_by_label("A new version is available. Would you like to relaunch with the updated version?").is_none());
    // Toolbar update banner is now visible with Update and restart
    h.get_by_label("Update and restart");
}

#[test]
fn the_current_desktop_setting_is_only_offered_where_the_os_has_desktops() {
    // NoHost has no desktops (Windows, Linux, web): no setting.
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    open_settings(&mut h);
    assert!(h.query_by_label("Show the window on the current desktop").is_none());
    // On by default for everyone; macOS shows it in Settings (DesktopHost::has_desktops).
    assert!(Settings::default().show_on_current_desktop);
}

#[test]
fn feed_announcements_and_alerts_show_until_dismissed() {
    use redrob_launcher_core::feed::Notice;
    let notice = |id: &str, title: &str, link: Option<&str>| Notice {
        id: id.into(),
        title: title.into(),
        body: format!("{title} body"),
        link: link.map(str::to_string),
        created_at: None,
    };
    let sent = Rc::new(RefCell::new(Vec::new()));
    let mut evs = events();
    evs.push(Event::Notices {
        announcements: vec![notice("a1", "Redrob CAD 1.0 is out", Some("https://redrob.ai/news"))],
        alerts: vec![notice("x1", "Security update", Some("https://redrob.ai/security"))],
    });
    let mut backend = Some(Fake { events: evs, sent, manage: true, clock: Rc::new(Cell::new(NOW)) });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    h.run_steps(6);
    // The alert pops up first; OK acknowledges it for good.
    h.get_by_label("Security update");
    h.get_by_label("Open link");
    h.get_by_label("OK").click();
    h.run_steps(4);
    assert!(h.query_by_label("Security update").is_none());
    assert!(h.state().as_ref().unwrap().settings.dismissed_notices.contains("x1"));
    // The announcement stays inline until dismissed.
    h.get_by_label("Redrob CAD 1.0 is out");
    h.get_by_label("Learn more");
    h.get_all_by_label("Dismiss").next().unwrap().click();
    h.run_steps(4);
    assert!(h.query_by_label("Redrob CAD 1.0 is out").is_none());
    assert!(h.state().as_ref().unwrap().settings.dismissed_notices.contains("a1"));
}

#[test]
fn automatic_checks_run_every_five_minutes() {
    use redrob_launcher_ui_egui::settings::CheckEvery;
    assert_eq!(Settings::default().check_every.seconds(), Some(300));
    // Older saved choices still load, and mean "automatically" now.
    assert_eq!(CheckEvery::Day.seconds(), Some(300));
    assert_eq!(CheckEvery::Never.seconds(), None);
}

#[test]
fn automatic_updates_queue_installs_for_outdated_apps_and_launcher() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let sent = Rc::new(RefCell::new(Vec::new()));
    let sent2 = sent.clone();
    let mut backend = Some(Fake { events: ev, sent: sent2, manage: true, clock: Rc::new(Cell::new(NOW)) });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { auto_update: true, notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    h.run_steps(4);
    let commands = sent.borrow().clone();
    assert!(
        commands.contains(&Command::Install { app: "redrob-canvas".into() }),
        "outdated redrob-canvas should be queued for install: {commands:?}"
    );
    assert!(
        commands.contains(&Command::Install { app: "redrob-browser".into() }),
        "outdated redrob-browser should be queued for install: {commands:?}"
    );
    assert!(commands.contains(&Command::UpdateLauncher), "launcher update should be queued: {commands:?}");
}

#[test]
fn automatic_updates_are_disabled_by_default() {
    let mut ev = events();
    ev.push(Event::LauncherUpdate(release("redrob-launcher", "0.2.0", NOW)));
    let sent = Rc::new(RefCell::new(Vec::new()));
    let sent2 = sent.clone();
    let mut backend = Some(Fake { events: ev, sent: sent2, manage: true, clock: Rc::new(Cell::new(NOW)) });
    let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none()
                && let Some(b) = backend.take()
            {
                *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { auto_update: false, notify: false, ..Settings::default() }));
            }
            if let Some(l) = state.as_mut() {
                l.logic(&ui.ctx().clone(), &mut NoHost);
                l.ui(ui, &mut NoHost);
            }
        },
        None,
    );
    h.run_steps(4);
    let commands = sent.borrow().clone();
    assert!(
        !commands.contains(&Command::Install { app: "redrob-canvas".into() }),
        "redrob-canvas should not be auto-installed when auto_update is false"
    );
    assert!(!commands.contains(&Command::UpdateLauncher), "launcher should not be auto-updated when auto_update is false");
}

#[test]
fn settings_name_the_services_behind_what_is_on_screen() {
    let status = |extra: Vec<Event>| {
        let mut evs = events();
        evs.extend(extra);
        let sent = Rc::new(RefCell::new(Vec::new()));
        let mut backend = Some(Fake { events: evs, sent, manage: true, clock: Rc::new(Cell::new(NOW)) });
        let mut h = Harness::builder().with_size(egui::vec2(1180.0, 900.0)).build_ui_state(
            move |ui, state: &mut Option<LauncherUi>| {
                if state.is_none()
                    && let Some(b) = backend.take()
                {
                    *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
                }
                if let Some(l) = state.as_mut() {
                    l.logic(&ui.ctx().clone(), &mut NoHost);
                    l.ui(ui, &mut NoHost);
                }
            },
            None,
        );
        h.run_steps(6);
        // No footer on the Apps page any more.
        assert!(h.query_by_label_contains("Checked ").is_none(), "the Apps page has no footer");
        open_settings(&mut h);
        let label =
            h.get_by_label_contains("Checked ").accesskit_node().label().or_else(|| h.get_by_label_contains("Checked ").accesskit_node().value());
        label.unwrap_or_default()
    };
    assert_eq!(status(Vec::new()), "Checked 3 min ago · using Redrob API");
    let from_github =
        vec![Event::Latest { app: "redrob-code".into(), latest: LatestRelease::Found(release("redrob-code", "0.3.1", NOW)), from: Origin::GitHub }];
    assert_eq!(status(from_github), "Checked 3 min ago · using Redrob API and GitHub");
}

#[test]
fn web_build_offers_downloads() {
    let (mut h, _) = harness(egui::Theme::Dark, false);
    h.run_steps(8);
    snapshot(&mut h, "web-grid-dark");
    assert!(h.query_all_by_label("Download for macOS").count() >= 6);
    // Read-only web cards still need a discoverable route to alternate packages and links.
    let visible_cards = SUITE.iter().filter(|app| h.query_by_label_contains(&format!("{}:", app.name)).is_some()).count();
    assert_eq!(h.query_all_by_label("More actions").count(), visible_cards);
    assert!(h.query_by_label("Install").is_none());
}

/// The title bar's search field.
fn search_field_rect(h: &Harness<'_, Option<LauncherUi>>) -> egui::Rect {
    h.query_all(egui_kittest::kittest::by().role(egui::accesskit::Role::TextInput).label("Search apps"))
        .map(|n| n.rect())
        .find(|r| r.bottom() <= 48.0)
        .unwrap()
}

#[test]
fn a_native_title_bar_puts_the_chrome_in_a_row_over_the_page() {
    // NoHost: the OS draws the title bar (or there is none, on the web).
    let (mut h, sent) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    snapshot(&mut h, "grid-native-dark");
    assert!(h.query_by_label("Minimize").is_none());
    // No title bar of ours: Update, the logo and the search sit in a 48 px row over the page, left to right.
    let search = search_field_rect(&h);
    let logo = h.get_by_label("Go to console.redrob.ai").rect();
    let update = h.get_by_label("Update all (2)").rect();
    for r in [search, logo, update] {
        assert!(r.bottom() <= 48.0, "the chrome row is 48 px: {r:?}");
    }
    assert!(update.right() <= logo.left() && logo.right() <= search.left(), "{update:?} {logo:?} {search:?}");
    // The Apps page has no top bar of its own: no title over the grid, refresh at the filter row's end.
    assert_eq!(h.query_all_by_label("All apps").count(), 1, "only the sidebar's row");
    let check = h.get_by_label("Check for updates").rect();
    let filters = h.get_by_label_contains("Not installed ").rect();
    assert!(check.top() > 48.0 && (check.center().y - filters.center().y).abs() < 4.0, "{check:?} vs {filters:?}");
    assert!(check.left() > filters.right(), "refresh at the right end of the filter row");
    sent.borrow_mut().clear();
    h.get_by_label("Check for updates").click();
    h.run_steps(2);
    assert!(sent.borrow().contains(&Command::CheckForUpdates { force: true }));
}

#[test]
fn the_macos_title_bar_keeps_the_traffic_lights_clear() {
    struct MacHost;
    impl Host for MacHost {
        fn title_bar(&self) -> redrob_launcher_ui_egui::TitleBar {
            redrob_launcher_ui_egui::TitleBar::MacOs
        }
    }
    for width in [600.0, 960.0, 1180.0] {
        let mut backend = Some(Fake { events: events(), sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
        let mut h = Harness::builder().with_size(egui::vec2(width, 900.0)).build_ui_state(
            move |ui, state: &mut Option<LauncherUi>| {
                if state.is_none()
                    && let Some(b) = backend.take()
                {
                    *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
                }
                if let Some(l) = state.as_mut() {
                    l.logic(&ui.ctx().clone(), &mut MacHost);
                    l.ui(ui, &mut MacHost);
                }
            },
            None,
        );
        h.run_steps(6);
        if width == 1180.0 {
            snapshot(&mut h, "grid-title-bar-macos-dark");
        }
        // The OS draws the traffic lights: no window buttons of ours.
        assert!(h.query_by_label("Minimize").is_none(), "{width}");
        let search = search_field_rect(&h);
        let logo = h.get_by_label("Go to console.redrob.ai").rect();
        let update = h.get_by_label("Update all (2)").rect();
        // Left to right: Update, the logo, the search, all in the 48 px bar and inside the window.
        assert!(update.right() <= logo.left() && logo.right() <= search.left() && search.right() <= width, "{width}: {update:?} {logo:?} {search:?}");
        for r in [search, logo, update] {
            assert!(r.bottom() <= 48.0, "{width}: {r:?}");
        }
        // The traffic lights' 72 px stay clear, with room to drag the window beside them.
        assert!(update.left() >= 72.0 + 24.0, "{width}: {update:?}");
    }
}

#[test]
fn the_grid_shows_every_app_name_whole_at_the_default_window() {
    let (mut h, _) = harness(egui::Theme::Dark, true);
    h.run_steps(6);
    // Each card's name is one unwrapped line (a wrapped wordmark would be two lines tall) and
    // lies inside the window, beside its icon.
    let mut seen = 0;
    for app in redrob_launcher_core::SUITE {
        for node in h.query_all_by_label(app.name) {
            let r = node.rect();
            if r.top() < 48.0 || r.height() > 30.0 && r.width() > 300.0 {
                continue; // not a card's name (e.g. a whole card or the title bar)
            }
            assert!(r.height() <= 30.0, "{}: wrapped onto a second line: {r:?}", app.name);
            assert!(r.right() <= 1180.0 - 24.0, "{}: runs past the content edge: {r:?}", app.name);
            seen += 1;
        }
    }
    assert!(seen >= 5, "the first section's names were checked ({seen})");
}

#[test]
fn custom_title_bar_has_window_buttons() {
    struct FramelessHost;
    impl Host for FramelessHost {
        fn title_bar(&self) -> redrob_launcher_ui_egui::TitleBar {
            redrob_launcher_ui_egui::TitleBar::Custom
        }
    }
    let frameless = |width: f32| {
        let mut backend = Some(Fake { events: events(), sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
        Harness::builder().with_size(egui::vec2(width, 900.0)).build_ui_state(
            move |ui, state: &mut Option<LauncherUi>| {
                if state.is_none()
                    && let Some(b) = backend.take()
                {
                    *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
                }
                if let Some(l) = state.as_mut() {
                    l.logic(&ui.ctx().clone(), &mut FramelessHost);
                    l.ui(ui, &mut FramelessHost);
                }
            },
            None,
        )
    };
    // Narrow windows shrink the search first, never under the caption buttons, and leave the bar
    // room to drag left of Update.
    for width in [600.0, 960.0] {
        let mut h = frameless(width);
        h.run_steps(6);
        snapshot(&mut h, &format!("grid-title-bar-dark-{width}"));
        let minimize = h.get_by_label("Minimize").rect();
        let search = search_field_rect(&h);
        let update = h.get_by_label("Update all (2)").rect();
        assert!(search.right() <= minimize.left() && search.width() >= 159.5, "{width}: {search:?}");
        assert!(update.left() >= 24.0, "{width}: {update:?}");
    }
    let mut h = frameless(1180.0);
    h.run_steps(6);
    snapshot(&mut h, "grid-title-bar-dark");
    // The custom bar, right to left: window buttons, search, the group logo, Update.
    let minimize = h.get_by_label("Minimize").rect();
    let search = search_field_rect(&h);
    let logo = h.get_by_label("Go to console.redrob.ai").rect();
    let update = h.get_by_label("Update all (2)").rect();
    assert!(update.right() <= logo.left() && logo.right() <= search.left() && search.right() <= minimize.left());
    // One row, inside the caption buttons' band (whose target starts under the 5 px resize edge).
    for r in [search, logo, update] {
        assert!(r.top() >= minimize.top() - 5.0 && r.bottom() <= minimize.bottom(), "{r:?} vs {minimize:?}");
    }
    // Check for updates moved down to the filter row.
    assert!(h.get_by_label("Check for updates").rect().top() > 48.0);
    assert!(h.get_all_by_label("All apps").all(|n| n.rect().top() >= 48.0), "the sidebar and page start under the title bar");
    // Restored, the top 5 px of the caption buttons and close's right 5 px stay the resize band.
    // (The harness insets the bar: its top is minimize's minus that band.)
    h.get_by_label("Maximize");
    let restored_close = h.get_by_label("Close").rect();
    let bar_top = minimize.top() - 5.0;
    assert!((restored_close.top() - (bar_top + 5.0)).abs() < 0.5, "{restored_close:?}");
    // Maximized there is no resize band: the buttons run to the top of the screen and close to its
    // right edge, so flinging the pointer into the corner lands on Close (Windows' own buttons).
    for _ in 0..3 {
        h.input_mut().viewports.entry(egui::ViewportId::ROOT).or_default().maximized = Some(true);
        h.step();
    }
    let close = h.get_by_label("Close").rect();
    let restore = h.get_by_label("Restore").rect();
    assert!((close.top() - bar_top).abs() < 0.5 && (close.right() - (restored_close.right() + 5.0)).abs() < 0.5, "{close:?}");
    assert!((restore.top() - bar_top).abs() < 0.5 && (restore.right() - close.left()).abs() < 0.5, "{restore:?}");
    h.get_by_label("Minimize");
    h.get_by_label("Restore");
    h.get_by_label("Close").click();
    h.step();
    let closes = h.output().viewport_output.values().any(|v| v.commands.contains(&egui::ViewportCommand::Close));
    assert!(closes, "the close button asks the window to close (the shell decides whether that hides it to the tray)");
}

#[test]
fn settings_say_when_github_can_be_asked_again_and_the_platform() {
    for width in [600.0, 1180.0] {
        let mut evs = events();
        evs.push(Event::CheckFinished { at: NOW - 180, source: Source::GitHubApi, rate_limited_until: Some(NOW + 1800) });
        let mut backend = Some(Fake { events: evs, sent: Rc::new(RefCell::new(Vec::new())), manage: true, clock: Rc::new(Cell::new(NOW)) });
        let mut h = Harness::builder().with_size(egui::vec2(width, 900.0)).build_ui_state(
            move |ui, state: &mut Option<LauncherUi>| {
                if state.is_none()
                    && let Some(b) = backend.take()
                {
                    *state = Some(LauncherUi::new(ui.ctx(), Box::new(b), Settings { notify: false, ..Settings::default() }));
                }
                if let Some(l) = state.as_mut() {
                    l.logic(&ui.ctx().clone(), &mut NoHost);
                    l.ui(ui, &mut NoHost);
                }
            },
            None,
        );
        h.run_steps(6);
        // The Apps page says none of it: the footer is gone.
        for gone in ["in 30 min", "Checked ", "Installing for"] {
            assert!(h.query_by_label_contains(gone).is_none(), "{width}: {gone} is still on the Apps page");
        }
        open_settings(&mut h);
        snapshot(&mut h, &format!("settings-rate-limited-{width}"));
        // Whole, wrapped if it must, and inside the window.
        let notice = h.get_by_label_contains("full details return in 30 min").rect();
        assert!(notice.right() <= width, "{width}: notice {notice:?}");
        let checked = h.get_by_label_contains("Checked ").rect();
        let platform = h.get_by_label_contains("Installing for").rect();
        // Read top to bottom: the check, the limit, the platform.
        assert!(checked.bottom() <= notice.top() && notice.bottom() <= platform.top(), "{width}: {checked:?} {notice:?} {platform:?}");
    }
}
