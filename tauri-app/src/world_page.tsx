import React, { useEffect, useState } from 'react';
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core"; // invoke をインポート
import "./App.css";

// RustのWorldEntry構造体と対応する型定義
interface WorldEntry {
    id: string; // RustのWorldEntryに合わせてidを追加
    name: string;
    entry_point_path: string; // Rustのentry_point_pathに合わせる
}

// RustのWorldList構造体（内部にworlds配列を持つ）と対応する型定義
interface WorldListResponse {
    worlds: WorldEntry[];
}

const WorldPage: React.FC = () => {
    // データを保持するためのState。型を WorldEntry[] に変更
    const [worldList, setWorldList] = useState<WorldEntry[]>([]);
    // エラーメッセージを保持するためのState
    const [error, setError] = useState<string | null>(null);
    // ローディング状態を保持するためのState
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const navigate = useNavigate();

    useEffect(() => {
        const loadWorldList = async () => {
            try {
                setIsLoading(true); // ロード開始
                setError(null);    // エラーをリセット

                // Rustコマンドを呼び出してワールドリストを取得
                // invokeの結果は直接 WorldListResponse 型のデータ
                const response: WorldListResponse = await invoke("get_world_list");
                
                // Rustから受け取ったデータを直接セット
                // ここで response.worlds をセットします
                setWorldList(response.worlds);

            } catch (err) {
                console.error('ワールドリストの読み込みエラー:', err);
                if (err instanceof Error) {
                    setError(`データの読み込みに失敗しました: ${err.message}`);
                } else if (typeof err === 'string') {
                    // Rustから返されるエラーは通常文字列なのでこれを考慮
                    setError(`データの読み込みに失敗しました: ${err}`);
                } else {
                    setError('データの読み込みに失敗しました。不明なエラー。');
                }
            } finally {
                setIsLoading(false); // ロード終了
            }
        };

        loadWorldList();
    }, []); 

    const loadWorldSearch = () => {
        navigate("/world_search");
    };
    const WasmLoader = () => {
        navigate("/wasm_loader");
    };

    // ワールドをクリックしたときのハンドラ
        const handleWorldClick = async (worldId: string, worldName: string, entryPointPath: string) => {
        console.log(`ワールド起動試行: ID=${worldId}, Name=${worldName}, Entry=${entryPointPath}`);
        try {
            await invoke('open_world', {
                entryPointPath: entryPointPath,
                worldName: worldName,
            });
            console.log("ゲームウィンドウが正常に開かれました。");
            // navigate('/'); // 例: メインメニューに戻る
        } catch (error) {
            console.error("ゲーム起動に失敗しました:", error);
            // エラー表示などのフィードバック
            if (error instanceof Error) {
                alert(`ゲームの起動に失敗しました: ${error.message}`);
            } else if (typeof error === 'string') {
                alert(`ゲームの起動に失敗しました: ${error}`);
            } else {
                alert('ゲームの起動に失敗しました。不明なエラー。');
            }
        }
    };

    return (
        <div>
            {/* 戻るリンク */}
            <a href="/">戻る</a> {/* メニューに戻る */}
            <button onClick={loadWorldSearch}>WORLD_Search</button>
            <button onClick={WasmLoader}>WORLD_IMPORT</button>

            <h2>ワールドリスト</h2>
            <div id="content">
                {isLoading && <p>データを読み込み中...</p>}
                {error && <p style={{ color: 'red' }}>{error}</p>}
                
                {/* データが表示される部分 */}
                {!isLoading && !error && (
                    worldList.length > 0 ? (
                        worldList.map((item) => (
                            <button
                                key={item.id} // keyはWorldEntryのidプロパティを使用
                                onClick={() => handleWorldClick(item.id, item.name, item.entry_point_path)}
                                style={{ display: 'block', margin: '10px 0', padding: '10px', border: '1px solid #ccc', borderRadius: '5px', cursor: 'pointer', backgroundColor: '#f9f9f9', width: 'fit-content' }}
                            >
                                {item.name}
                            </button>
                        ))
                    ) : (
                        <p>表示するデータがありません。</p>
                    )
                )}

            </div>
        </div>
    );
};

export default WorldPage;
