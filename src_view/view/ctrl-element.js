import { LitElement, html, css } from "lit"

// CtrlElement クラスをエクスポート
export class CtrlElement extends LitElement { // ★export を追加★
    static properties = {
    }

    static styles = css`
        .container {
            display: flex;
            gap: 10px;
        }
    `

    constructor() {
        super()
    }

    render() {
        return html`
            <div class="container">
                <button @click=${this.getTitle}>タイトル取得</button>
                <button @click=${this.getExternalLinks}>外部リンク取得</button>
                <button @click=${this.reset}>ページリセット</button>
            </div>
        `
    }

    sendEvent(operation, options) {
        this.dispatchEvent(new CustomEvent("ctrl", { detail: { operation, options } }))
    }

    getTitle() {
        this.sendEvent("get-title")
    }

    getExternalLinks() {
        this.sendEvent("get-external-links")
    }

    reset() {
        this.sendEvent("reset")
    }
}

// ResultElement クラスもエクスポート
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
    `

    constructor() {
        super()
    }

    render() {
        if (this.result?.error) {
            const text = JSON.stringify(this.result.error, null, "  ")
            return html`
                <h1>エラー</h1>
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
                        ${data.links.map(({ text, url }) => html`<div><a href=${url}>${text}</a></div>`)}
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

