// src/WorldPage.tsx

import React, { useEffect, useState } from 'react';

// JSONデータの型定義 (world_list.json の構造に合わせて調整してください)
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

    useEffect(() => {
        const loadJsonFile = async () => {
            try {
                setIsLoading(true); // ロード開始
                setError(null);    // エラーをリセット

                // Tauri環境でビルドする場合、`public` フォルダ直下に配置されているJSONファイルは
                // そのままパスでアクセスできることが多いですが、環境によってはパスの調整が必要です。
                // 例: const response = await fetch('/world_list.json');
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
    }, []); // 空の依存配列で、コンポーネントのマウント時に一度だけ実行

    return (
        <div>
            {/* 戻るリンク */}
            <a href="index.html">戻る</a> {/* 必要に応じてReact RouterのLinkなどに変更 */}

            <h2>ワールドリスト</h2>
            <div id="content">
                {isLoading && <p>データを読み込み中...</p>}
                {error && <p style={{ color: 'red' }}>{error}</p>}
                
                {/* データが表示される部分 */}
                {!isLoading && !error && (
                    worldList.length > 0 ? (
                        worldList.map((item, index) => (
                            <a
                                key={index} // keyは必須です。実際のデータにユニークなIDがあればそちらを使うべきです。
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
