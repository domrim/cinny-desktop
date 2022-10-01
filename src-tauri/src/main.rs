#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[cfg(target_os = "macos")]
mod menu;

fn main() {
  let builder = tauri::Builder::default();

  #[cfg(target_os = "macos")]
  let builder = builder.menu(menu::menu());

  let builder = builder
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  #[cfg(not(target_os = "macos"))]
  builder
    .run();

  #[cfg(target_os = "macos")]
  builder
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      }
      _ => {}
    });
}