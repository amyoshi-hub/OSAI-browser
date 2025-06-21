# OSAI-browser
OSAI-browser: A P2P Browser with World Support

OSAI-browser is a new type of browser, designed with peer-to-peer (P2P) functionality at its core.

Key Features:

    P2P Functionality: OSAI-browser is built around a P2P network, allowing for decentralized content sharing and access.
    World Support: Additional content is managed as "Worlds."
    Easy World Import: Add new "Worlds" by simply dragging and dropping files into the "World Import" area.
    iframe Mode: Seamlessly switch between local "Worlds" and regular websites (like Google) using the iframe mode. Local files can be accessed without disabling iframes.

Technical Details:

    Technology Stack: rust, tauri, (in p2o):pnet
    Installation: exe file only

rust:
    in root: cargo tauri build

Current Status:

    P2P functionality is implemented.
    (TODO: later)

Important Notes (Windows):

    On Windows, a separate packet monitoring driver may be required.
    Currently, P2P functionality requires administrator privileges.
p2p function:
    ->need sudo permission<-
file drag install:
    please include "index.html"

$comment:
    currently app lunguege is japanease sorry!

p2pの機能を持ったブラウザである
<p>
追加コンテンツはworldとして扱う
<p>
worldにはworld importにファイルをドラッグすることで追加することができる
<p>
iframe mode:
<p>
iframeからno iframeにすることでurlからgoogleなどに飛べる。ローカルのファイルなら切らなくて良い
<p>
TODO:動機、技術スタック、install方法などを書く
<p>
windowsは別途のパケット監視ドライバを入れないと動かないかも
<p>
今のところp2p機能は管理者権限がいる
