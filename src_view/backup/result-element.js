import { LitElement, html, css } from "lit"

export class ResultElement extends LitElement { // ★export を追加★
    static properties = {
        result: {},
    }

    static styles = css`
        /* スタイルがあればここに記述 */
        pre {
            background-color: #333;
            padding: 10px;
            border-radius: 5px;
            overflow-x: auto;
            color: #eee;
        }
        h1 {
            color: #007bff; /* タイトル色を調整 */
            font-size: 1.2em;
            margin-bottom: 5px;
        }
        p {
            margin: 0;
            padding: 0;
        }
        nav div a {
            color: #87ceeb; /* リンク色を調整 */
            text-decoration: none;
        }
        nav div a:hover {
            text-decoration: underline;
        }
        /* エラー表示用のスタイル */
        .error-message {
            color: #ff4d4d; /* エラーメッセージを赤に */
            font-weight: bold;
        }
    `

    constructor() {
        super()
    }

    render() {
        if (this.result?.error) {
            const text = JSON.stringify(this.result.error, null, "  ")
            return html`
                <h1 class="error-message">エラー</h1>
                <pre>${text}</pre>
            `
        }
        if (this.result?.data) {
            const data = this.result.data

            let content
            if (data.operation === "get-title") {
                content = html`<p>${data.title}</p>`
            } else if (data.operation === "get-external-links") {
                content = html`
                    <nav>
                        ${data.links ? data.links.map(({ text, url }) => html`<div><a href=${url}>${text}</a></div>`) : html`<p>リンクなし</p>`}
                    </nav>
                `
            } else if (data.operation === "reset") { // reset操作の表示を追加
                content = html`<p>${data.message}</p>`
            }


            return html`
                <h1>${data.operation}</h1>
                ${content}
            `
        }
        return html`<p>操作してください</p>`
    }

    show(result) {
        this.result = result
    }
}

