// src-tauri/src/main.rs

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tauri::Manager;
//use tauri::path::app_data_dir;
//use uuid::Uuid;

// world_list.json のエントリの構造を定義
#[derive(Debug, Serialize, Deserialize)]
struct WorldEntry {
    id: String,
    name: String,
    // GodotゲームのエントリポイントHTMLへの相対パスを保存
    // 例: "my_godot_game_folder/index.html"
    entry_point_path: String,
    // 必要に応じて、WASMパスやその他のメタデータも追加可能
}

// world_list.json ファイル全体の構造
#[derive(Debug, Serialize, Deserialize)]
struct WorldList {
    worlds: Vec<WorldEntry>,
}

// ZIPファイルを解凍するヘルパー関数
// dest_dir: 解凍先のディレクトリパス
// zip_file_path: ZIPファイルのパス
fn unzip_file(zip_file_path: &Path, dest_dir: &Path) -> Result<(), String> {
    println!("Rust: Unzipping {} to {}", zip_file_path.display(), dest_dir.display());
    let file = fs::File::open(zip_file_path)
        .map_err(|e| format!("Failed to open zip file {}: {}", zip_file_path.display(), e))?;

    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to create zip archive: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to get file from zip archive at index {}: {}", i, e))?;
        let outpath = match file.enclosed_name() {
            Some(path) => dest_dir.join(path),
            None => continue,
        };

        if (&*file.name()).ends_with('/') {
            // ディレクトリの場合
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory {}: {}", outpath.display(), e))?;
        } else {
            // ファイルの場合
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("Failed to create parent directory {}: {}", p.display(), e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath)
                .map_err(|e| format!("Failed to create file {}: {}", outpath.display(), e))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to copy file content to {}: {}", outpath.display(), e))?;
        }
        // Unixパーミッションをセット（オプション）
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))
                    .map_err(|e| format!("Failed to set permissions for {}: {}", outpath.display(), e))?;
            }
        }
    }
    Ok(())
}


// ファイルのコピーとworld_list.jsonの更新を行うコマンド
// source_path: ドロップされたファイルの絶対パス (フロントエンドから渡される)
// world_name: ワールドの名前 (ユーザー入力やファイル名から決定)
// 戻り値: 追加されたワールドのエントリポイントパス (worldディレクトリからの相対パス)
#[tauri::command]
pub async fn process_and_add_world(source_path: String, world_name: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    println!("Rust: Processing and adding world...");

    let source_path_buf = PathBuf::from(&source_path);
    let file_stem = source_path_buf.file_stem() // 拡張子なしのファイル名 (例: "my_game")
                                   .and_then(|s| s.to_str())
                                   .ok_or_else(|| "Invalid source path: missing file stem".to_string())?
                                   .to_string();
    let extension = source_path_buf.extension() // 拡張子 (例: "zip", "html", "wasm")
                                   .and_then(|s| s.to_str())
                                   .unwrap_or_default();

    // アプリケーションのデータディレクトリを取得
    let app_data_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let worlds_root_dir = app_data_dir.join("worlds"); // 例: .../your_app_name/worlds
    let world_destination_dir = worlds_root_dir.join(&file_stem); // 例: .../worlds/my_game

    // worlds ルートディレクトリが存在しない場合は作成
    if !worlds_root_dir.exists() {
        fs::create_dir_all(&worlds_root_dir)
            .map_err(|e| format!("Failed to create worlds root directory: {}", e))?;
    }

    let mut entry_point_relative_path = String::new(); // world_list.jsonに保存するパス

    if extension == "zip" {
        // ZIPファイルの場合、解凍
        if world_destination_dir.exists() {
             // 既存のディレクトリがある場合は削除またはエラーにする
            fs::remove_dir_all(&world_destination_dir)
                .map_err(|e| format!("Failed to remove existing world directory {}: {}", world_destination_dir.display(), e))?;
        }
        fs::create_dir_all(&world_destination_dir)
            .map_err(|e| format!("Failed to create world destination directory for zip: {}", e))?;

        unzip_file(&source_path_buf, &world_destination_dir)?;

        // Godot Webエクスポートの場合、通常はルートに index.html がある
        let html_file_name = "index.html"; // Godotのデフォルト
        let godot_html_path = world_destination_dir.join(html_file_name);
        if !godot_html_path.exists() {
            // index.html が見つからない場合はエラー、または別のデフォルトを探す
            return Err(format!("Unzipped content does not contain {}. Please ensure it's a valid Godot Web Export.", html_file_name));
        }
        entry_point_relative_path = PathBuf::from(file_stem).join(html_file_name).to_str().unwrap().to_string();

    } else {
        // ZIPファイル以外の場合（単一のHTML/WASMなど）は、world_destination_dirの下に直接コピー
        if world_destination_dir.exists() {
             // 既に同名のフォルダがある場合は削除またはエラーにする
            fs::remove_dir_all(&world_destination_dir)
                .map_err(|e| format!("Failed to remove existing world directory {}: {}", world_destination_dir.display(), e))?;
        }
        fs::create_dir_all(&world_destination_dir)
            .map_err(|e| format!("Failed to create world destination directory for file: {}", e))?;
        
        let dest_file_path = world_destination_dir.join(source_path_buf.file_name().unwrap());
        fs::copy(&source_path_buf, &dest_file_path)
            .map_err(|e| format!("Failed to copy file from {} to {}: {}", source_path, dest_file_path.display(), e))?;

        // この場合のエントリポイントはコピーしたファイル自体
        entry_point_relative_path = PathBuf::from(file_stem).join(source_path_buf.file_name().unwrap()).to_str().unwrap().to_string();
    }

    // world_list.json を読み込むか、新規作成する
    let world_list_path = worlds_root_dir.join("world_list.json");
    let mut world_list: WorldList = if world_list_path.exists() {
        let json_str = fs::read_to_string(&world_list_path)
            .map_err(|e| format!("Failed to read world_list.json: {}", e))?;
        serde_json::from_str(&json_str)
            .map_err(|e| format!("Failed to parse world_list.json: {}", e))?
    } else {
        WorldList { worlds: Vec::new() }
    };

    // 新しいワールドエントリを追加 (既存の同名ワールドは削除)
    world_list.worlds.retain(|w| w.name != world_name); // 同じ名前のワールドがあれば削除
    let new_world_id = uuid::Uuid::new_v4().to_string(); // ユニークなIDを生成
    let new_entry = WorldEntry {
        id: new_world_id,
        name: world_name.clone(), // フロントから渡された名前を使用
        entry_point_path: entry_point_relative_path.clone(),
    };
    world_list.worlds.push(new_entry);

    // 更新されたワールドリストをJSONとして保存
    let updated_json = serde_json::to_string_pretty(&world_list)
        .map_err(|e| format!("Failed to serialize updated world_list.json: {}", e))?;
    fs::write(&world_list_path, updated_json)
        .map_err(|e| format!("Failed to write updated world_list.json: {}", e))?;

    println!("Rust: World '{}' added successfully. Entry point: {}", world_name, entry_point_relative_path);
    Ok(entry_point_relative_path) // 成功したエントリポイントパスを返す
}

