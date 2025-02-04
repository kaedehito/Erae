# Erae
Emacsライクの軽量エディター

## 概要
Eraeは、Emacsの操作性を踏襲しつつ、軽量かつシンプルなエディターです。Emacsの主要なキーバインドを保持しながら、一部の操作を独自に最適化しています。

## 特徴
- Emacsライクなキーバインドを採用
- 軽量で高速な動作
- ファイル編集に必要な基本機能を提供
- 直感的な操作性

## EmacsとEraeの異なるキーバインド

| 動作 | Emacs | Erae |
|:-----------|------------:|:------------:|
|   ファイルセーブ    |    C-x C-s     |     C-s     |
| 退出     |      C-x C-c |    C-q    |

基本的な操作はEmacsと変わりありません。

(C-o, C-f, C-b, C-d, C-n, C-p はすべてEmacsと同じ動作をします)

## インストール
```sh
# GitHubからクローンしてビルド
git clone https://github.com/kaedehito/Erae.git
cd Erae
cargo install --path .
```

## 使い方
```sh
erae ファイル名
```

## ライセンス
本ソフトウェアはMITライセンスのもとで提供されます。
