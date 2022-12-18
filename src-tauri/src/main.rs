use proj::Proj;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let from = "EPSG:2230";
    let to = "EPSG:26946";
    let ft_to_m = Proj::new_known_crs(&from, &to, None).unwrap();
    let result = ft_to_m
        .convert((4760096.421921f64, 3744293.729449f64))
        .unwrap();
    format!("Hello, {:?}! You've been greeted from Rust!", result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
