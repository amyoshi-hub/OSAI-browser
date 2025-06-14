import { LitElement, html, css } from "lit"

export class AppElement extends LitElement {
    static properties = {
        currentUrl: { type: String }, // URL入力欄とiframeのURLを同期させるためのプロパティ
        initialUrl: { type: String } // ページリセット用の初期URL
    }

    static styles = css`
        :host {
            display: flex;
            flex-direction: column;
            height: 100vh;
            width: 100vw;
            background-color: black; /* 全体の背景色 */
            color: white;
            box-sizing: border-box;
            overflow: hidden;
        }
        .address-bar {
            display: flex;
            padding: 10px;
            background-color: #222;
            border-bottom: 1px solid #444;
            flex-shrink: 0;
        }
        #urlInput {
            flex-grow: 1;
            padding: 8px;
            border: 1px solid #555;
            border-radius: 5px;
            background-color: #333;
            color: white;
            font-size: 1em;
            outline: none;
        }
        #goButton {
            padding: 8px 15px;
            background-color: #007bff;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            margin-left: 10px;
            font-size: 1em;
        }
        #goButton:hover {
            background-color: #0056b3;
        }
        #contentFrame {
            flex-grow: 1; /* 残りのスペースを埋める */
            width: 100%;
            border: none;
            background-color: white; /* iframeのデフォルト背景色 (コンテンツで上書きされる) */
        }
        /* コントロールと結果表示のエリア */
        .control-and-result-area {
            display: flex;
            flex-direction: column;
            gap: 10px;
            padding: 10px; /* パディングを追加して見やすく */
            flex-shrink: 0;
        }
    `;

    constructor() {
        super();
        this.initialUrl = 'https://www.google.com'; // 初期URL
        this.currentUrl = this.initialUrl; // 現在表示しているURL
    }

    render() {
        return html`
            <div class="address-bar">
                <input
                    type="text"
                    id="urlInput"
                    .value="${this.currentUrl}"
                    @keydown="${this._handleKeyDown}"
                >
                <button id="goButton" @click="${this._handleGoClick}">Go</button>
            </div>
            <div class="control-and-result-area">
                <control-element id="ctrl" @ctrl=${this.onctrl}></control-element>
                <hr/>
                <result-element id="result"></result-element>
            </div>
            <hr/>
            <iframe id="contentFrame" .src="${this.currentUrl}" @load="${this._handleFrameLoad}"></iframe>
        `;
    }

    firstUpdated() {
        // 初回ロード時にiframeのURLを設定
        this.shadowRoot.getElementById('contentFrame').src = this.currentUrl;
    }

    _handleGoClick() {
        const inputElement = this.shadowRoot.getElementById('urlInput');
        let url = inputElement.value;
        // ユーザーがhttp://やhttps://を省略した場合に補完
        if (!url.startsWith('http://') && !url.startsWith('https://')) {
            url = 'https://' + url;
        }
        this.currentUrl = url; // プロパティを更新
        this.shadowRoot.getElementById('contentFrame').src = this.currentUrl; // iframeのsrcを直接更新
    }

    _handleKeyDown(event) {
        if (event.key === 'Enter') {
            this._handleGoClick();
        }
    }

    _handleFrameLoad() {
        const contentFrame = this.shadowRoot.getElementById('contentFrame');
        try {
            // クロスオリジンポリシーにより、外部サイトのiframe内部のURL取得はブロックされます。
            // 成功するのは、同じオリジンからのコンテンツ（例: app://）を読み込んだ場合のみです。
            const iframeUrl = contentFrame.contentWindow.location.href;
            this.currentUrl = iframeUrl; // アドレスバーを更新
        } catch (e) {
            console.warn("Could not get iframe URL due to cross-origin restrictions:", e);
            // 外部サイトの場合、iframeのsrc属性なら取得可能ですが、
            // 実際にリダイレクトなどが発生した最終URLとは異なる場合があります。
            // this.currentUrl = contentFrame.src;
        }
    }

    async onctrl(event) {
        const { operation, options } = event.detail;
        const resultElement = this.shadowRoot.getElementById("result");

        try {
            let resData;
            if (operation === "reset") {
                // 'reset' 操作はJavaScriptで直接 iframe のURLをリセットする
                this.currentUrl = this.initialUrl; // 初期URLにリセット
                this.shadowRoot.getElementById('contentFrame').src = this.currentUrl;
                resData = { operation: "reset", message: "ページを初期URLにリセットしました。" };
            } else {
                // その他の操作はRustコマンドを呼び出す
                // 注意: Rust側でWebView.eval()を使ってiframe内の外部サイトの情報を取得しようとすると、
                // 同一オリジンポリシーによりブロックされます。これはブラウザのセキュリティ機能です。
                // よって、get-titleやget-external-linksはメインの（app://）WebViewの情報を返します。
                resData = await window.__TAURI__.invoke(operation, options);
            }
            resultElement.show({ data: resData });
        } catch (error) {
            console.error("操作中にエラーが発生しました:", operation, error);
            resultElement.show({ error: { message: error.toString(), operation } });
        }
    }
}

