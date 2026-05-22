# Rust × AIでサーボを動かそう！ RP2040ハンズオン

## 機能

ボリュームつまみの位置に合わせ、1) R(赤)LED光量 2) サーボモータの回転角 が変わります

## 配線図

<img src="doc/wire.svg" alt="配線" />

## 使い方

開発（コード編集）は**ホスト上で普通に行い**、**ビルドだけを Docker で**行う。
Dev Container は使わなくてよい（必要なら `.devcontainer/` はそのまま残してある）。

ビルド成果物の `.uf2` ファイルは **OS に依存しない**ので、コンテナ内で作った `.uf2` を
ホストの `RPI-RP2` ドライブにコピーすれば、macOS / Linux / Windows のどれでも書き込める。

### 前提

- [Docker](https://docs.docker.com/get-docker/) が使えること（Docker Desktop / Docker Engine いずれでも可）。
- このリポジトリのルートで作業する。

### 1. ビルド用イメージを作る（最初の一度だけ）

`.devcontainer/Dockerfile` をそのままビルドイメージとして使う。Rust ツールチェイン・
`thumbv6m-none-eabi` ターゲット・`elf2uf2-rs` が入る。

```bash
docker build -t iotlt-build -f .devcontainer/Dockerfile .
```

### 2. UF2 をビルドする

カレントディレクトリ（リポジトリ）を `/workspace` にマウントしてコンテナ内でビルドし、
`elf2uf2-rs` で UF2 に変換する。`**-d` は付けない**（コンテナからホストの USB ドライブは見えないため）。

```bash
docker run --rm -v "$PWD":/workspace -w /workspace iotlt-build \
  bash -c "cargo build --release && \
    elf2uf2-rs target/thumbv6m-none-eabi/release/iotlt_flag_servo iotlt_flag_servo.uf2"
```

完了すると、リポジトリ直下（ホスト側）に `**iotlt_flag_servo.uf2**` が出来る。

> **Linux で成果物の所有者がずれるとき**: コンテナ内ユーザー（`vscode`, uid 1000）でファイルが
> 作られる。ホストの uid が 1000 以外だと `target/` や `.uf2` の所有者が合わないことがある。
> 気になる場合は `sudo chown -R "$(id -u):$(id -g)" target iotlt_flag_servo.uf2` で直す。
> macOS / Windows の Docker Desktop では自動でホストユーザー所有になるため通常は不要。

### 3. ファームウェアの書き込み（UF2）

1. PCとRP2040をUSB接続
2. XIAO RP2040 の **BOOT(B)** ボタンを押しながら RESET(B) ボタンを押す。USB コネクタ横の赤と RGB の LED が全部点灯すれば成功（点滅している場合はやり直す）。
3. 手順 2 で出来た `iotlt_flag_servo.uf2` を `**RPI-RP2` ドライブにコピー**する。
  コピーが終わるとドライブが自動的にマウント解除され、書き込み完了し、書き込んだプログラムが起動します。

### ドキュメント・リンク

- [Seeed Studio XIAO RP2040 の Wiki](https://wiki.seeedstudio.com/XIAO-RP2040/)
- Rust 向け BSP: [seeeduino-xiao-rp2040（docs.rs）](https://docs.rs/seeeduino-xiao-rp2040/)
- [Docker Documentation](https://docs.docker.com/)


###　他には？

- LEDは、R, G, B 3色の各USER_LEDと [フルカラーLED(NeoPixel WS2812)](https://akizukidenshi.com/goodsaffix/WS2812B_20200225.pdf)一つがあり、自由に使用することができます。
- 起動しているかどうかわかりにくいので、起動時にLEDを点滅するとかすればいいかも。
- 今は R LEDの輝度調節しているだけですが、フルカラーLEDを使うと色々できます。
- Rustの他、C言語、micro python が利用可能です
