// src/server_signal.rs

use rand::Rng; // session_id 生成用
// build_packet は client モジュールにあると仮定。
// client モジュールがなければ、このファイルを client.rs に統合するか、共通の util.rs などに移動。
use crate::client::build_packet; // 例: crate::client::build_packet のようにパスを修正

// サーバーアナウンスメントパケットを構築する関数
// 構築したパケットのバイト数 (len) を返す
pub fn build_server_announce_packet(
    buffer: &mut [u8],
    src_port: u16,
    dst_port: u16,
    // session_id はこの関数内で生成する
    // data_vec, data もこのシグナルに特化した内容にする
) -> usize {
    let mut rng = rand::thread_rng();
    let session_id: [u8; 16] = rng.gen(); // ランダムなセッションID
    let chunk: [u8; 8] = [0; 8];          // シグナル用なので全て0でOK
    let format_signal: [u8; 2] = [0xFF, 0xFF]; // サーバー生存シグナル
    let data_vec: [u8; 14] = [0; 14];     // シグナル用なので全て0でOK
    let data: &[u8] = b"OSAI Server Online"; // 簡潔なメッセージ (UTF-8)

    // build_packet を呼び出してパケットを構築し、その長さを返す
    // build_packet 関数がパケットの長さを返すように設計されている前提
    // もし build_packet が長さを返さないなら、自分で計算するか、build_packet の定義を見直す
    build_packet(
        buffer,
        src_port,
        dst_port,
        &session_id,
        &chunk,
        &format_signal,
        &data_vec,
        data
    )
}
