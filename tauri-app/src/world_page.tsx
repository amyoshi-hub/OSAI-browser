import React, { useEffect, useState } from 'react';
import { useNavigate } from "react-router-dom";
import "./App.css";


interface WorldItem {
    name: string;
    url: string;
}

const WorldPage: React.FC = () => {
    // データを保持するためのState
    const [worldList, setWorldList] = useState<WorldItem[]>([]);
    // エラーメッセージを保持するためのState
    const [error, setError] = useState<string | null>(null);
    // ローディング状態を保持するためのState
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const navigate = useNavigate();

    useEffect(() => {
        const loadJsonFile = async () => {
            try {
                setIsLoading(true); // ロード開始
                setError(null);    // エラーをリセット

                // Tauri環境でビルドする場合、`public` フォルダ直下に配置されているJSONファイルは
                // そのままパスでアクセスできることが多いですが、環境によってはパスの調整が必要です。
                const response = await fetch('world_list.json'); // 同一階層にある場合

                if (!response.ok) {
                    throw new Error(`HTTPエラー: ${response.status} ${response.statusText}`);
                }

                const jsonData: WorldItem[] = await response.json();
                setWorldList(jsonData); // データをStateにセット
            } catch (err) {
                console.error('エラー:', err);
                if (err instanceof Error) {
                    setError(`データの読み込みに失敗しました: ${err.message}`);
                } else {
                    setError('データの読み込みに失敗しました。不明なエラー。');
                }
            } finally {
                setIsLoading(false); // ロード終了
            }
        };

        loadJsonFile();
    }, []); 

      const loadWorldSearch = () => {
          navigate("/world_search");
      };
      const WasmLoader = () => {
          navigate("/wasm_loader");
      };


    return (
        <div>
            {/* 戻るリンク */}
            <a href="index.html">戻る</a> {/* 必要に応じてReact RouterのLinkなどに変更 */}
	    <button onClick={loadWorldSearch}>WORLD_Search</button>
	    <button onClick={WasmLoader}>WORLD_IMPORT</button>

            <h2>ワールドリスト</h2>
            <div id="content">
                {isLoading && <p>データを読み込み中...</p>}
                {error && <p style={{ color: 'red' }}>{error}</p>}
                
                {/* データが表示される部分 */}
                {!isLoading && !error && (
                    worldList.length > 0 ? (
                        worldList.map((item, index) => (
                            <a
                                key={index} // keyは必須です。実際のデータにユニークなidがあればそちらを使うべきです。
                                href={item.url}
                                style={{ display: 'block', margin: '10px 0' }}
                            >
                                {item.name}
                            </a>
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
