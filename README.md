# RP2040

[Seeed Studio XIAO RP2040 の Wiki](https://wiki.seeedstudio.com/XIAO-RP2040/)

## 使い方（日本語）

### 前提

- 配線・ピン割り当てはリポジトリ内の `AGENTS.md` を参照する。
- 本リポジトリは **Dev Container（Docker）** で Rust ツールチェーンと `thumbv6m-none-eabi` ターゲットを揃える想定である。
- ファームウェアのピン割り当て（要約）: サーボ **D4 / GPIO6**、ツマミ **D0 / GPIO26**、赤 LED は
  基板 **USER_LED_R / GPIO17**（PWM キャリア約 **1 kHz**）。**I2C は使用しない**。

### Dev Container で開発する

1. [Dev Containers](https://containers.dev/) 拡張機能が使えるエディタ（VS Code / Cursor など）でこのフォルダを開く。
2. コマンドパレットから **「Dev Containers: Reopen in Container」**（コンテナーで再度開く）を実行する。
3. コンテナ作成後に `postCreateCommand` で `cargo build` が走る。手元でビルドする場合はコンテナ内のターミナルで次を実行する。

   ```bash
   cargo build
   ```

4. **`/workspace` にファイルが無い**ように見えるときは、エクスプローラーで開いているパスがコンテナ内の
   `/workspace` か確認する。それでも空ならコマンドパレットから **「Dev Containers: Rebuild
   Container」** で再ビルドする（設定の `workspaceMount` が反映される）。ホスト側では必ず
   **このリポジトリのルート**をフォルダとして開くこと。

### Dev Container 内では `elf2uf2-rs -d` が使えない

コンテナの中からは、Mac / Docker Desktop などでは **ホストに出ている BOOTSEL 用 USB
ストレージ（`RPI-RP2`）が見えない**ことがほとんどです。  
そのため **`Unable to find mounted pico`** は、**想定どおりの失敗**（`-d` がドライブを
見つけられない）です。Docker まわりの説明は [Docker
Documentation](https://docs.docker.com/desktop/) を参照。

#### 推奨手順（ビルドはコンテナ、書き込みはホスト）

1. コンテナ内のターミナルで **UF2 ファイルだけ**作る（**`-d` は付けない**）。

   ```bash
   cargo build --release
   elf2uf2-rs target/thumbv6m-none-eabi/release/iotlt_flag_servo iotlt_flag_servo.uf2
   ```

2. `iotlt_flag_servo.uf2` は `/workspace` 経由で **ホストのプロジェクトフォルダ**にも出る。
   **Finder** でそのファイルを **`RPI-RP2` にドラッグ＆ドロップ**する（または **ホストの**
   ターミナルで `cp` する）。

`cargo run` / `cargo run --release` は `.cargo/config.toml` で  
`runner` が `elf2uf2-rs -d` のため、コンテナ内では書き込みまで行かない。  
**ビルドだけ**なら `cargo build` を使う。

### ファームウェアの書き込み（UF2）

1. XIAO RP2040 の **BOOT(B)** ボタンを押しながら USB で PC に接続し、USB ストレージ（多くの場合
   **`RPI-RP2`**）として認識させる。USBコネクタ横の赤とRGBのLEDが全部点灯すれば成功です。(点滅している場合は、やり直してください)
2. ビルドしてから UF2 を書き込む。**Dev Container 内では上記「`-d` が使えない」節の手順**を使う。

   **`elf2uf2-rs -d` が使えない環境**（コンテナ内、Cursor 内の一部環境など）では、`-d` は RP2040 用
   ボリュームを見つけられない。**方法 B** か、**ホストの通常ターミナル**で `-d` または `cp` を使う。

   **方法 A: 自動デプロイ（ホストで USB ストレージが見えるときだけ）**

   ```bash
   cargo build --release
   elf2uf2-rs target/thumbv6m-none-eabi/release/iotlt_flag_servo -d
   ```

   または `cargo run --release`（runner が同じく `elf2uf2-rs -d`）。

   **方法 B: UF2 をファイルに出してからコピー（確実・待たない）**

   ボリューム名は Finder やホストで `ls /Volumes` して確認する（例: `RPI-RP2`）。  
   **`cp` はホストのターミナルで実行する**（コンテナ内の `/Volumes` は Mac のドライブと別）。

   ```bash
   cargo build --release
   elf2uf2-rs target/thumbv6m-none-eabi/release/iotlt_flag_servo iotlt_flag_servo.uf2
   cp iotlt_flag_servo.uf2 /Volumes/RPI-RP2/
   ```

   書き込みが終わるとドライブが自動でマウント解除されることが多い。

### macOS で Docker を使う場合の注意

Docker Desktop 経由では **ホストの USB デバイスにコンテナからアクセスできない** ことが多い。その場合は次のいずれかになる。

- ホスト（macOS）側に `elf2uf2-rs` を入れ、ビルド成果物（ELF または生成した UF2）をホストで書き込む。
- または Linux 実機 / WSL2 など、USB パススルーが成立する環境でコンテナを使う。

### ドキュメント・リンク

- ボード情報: [XIAO RP2040 Wiki](https://wiki.seeedstudio.com/XIAO-RP2040/)
- Rust 向け BSP: [seeeduino-xiao-rp2040（docs.rs）](https://docs.rs/seeeduino-xiao-rp2040/)
