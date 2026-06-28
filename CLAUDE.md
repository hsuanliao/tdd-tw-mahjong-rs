# tdd-tw-mahjong-rs — 專案說明 (給 Claude 的常駐脈絡)

## 這是什麼

使用者的 **Rust 學習型 side project**：用 **TDD** 從零實作台灣 16 張麻將規則引擎。
主要目的是「透過做麻將學會 Rust」，不只是把功能做完。

## 使用者背景與偏好

- 有經驗的 **.NET 後端工程師**（.NET Framework / Core / Web / 後端），但 **Rust 新手**。
- 教學方式：**邊做邊講解 Rust 概念**，並**對照 C#/.NET**（crate≈專案、Cargo.toml≈.csproj、Option≈no null、Result≈比 exception 明確…）。
- 開發節奏：**TDD red-green-refactor**，每條規則先寫測試再實作；**一次走一個階段**，確認懂了再前進；每階段結束 commit 一次。
- 回應一律用**繁體中文**。

## 環境

- Rust **GNU 工具鏈**（x86_64-pc-windows-gnu；沒裝 VS C++ build tools）。
- PowerShell session 需把 `%USERPROFILE%\.cargo\bin` 加進 PATH 才找得到 cargo。
- IDE：JetBrains RustRover。
- git：repo 層級身分為 hsuanlch0628@gmail.com（**勿動全域**，全域是公司信箱）。
- GitHub：https://github.com/hsuanliao/tdd-tw-mahjong-rs (public, SSH remote)。

## 專案結構

Cargo workspace：
- `crates/engine` — 純規則引擎 (lib)，**無 IO**，所有規則邏輯與測試在這。
- `crates/cli` — 文字介面 (bin `mahjong`)，依賴 engine。

## Roadmap（5 階段）

- [x] 階段 0：環境與專案骨架
- [ ] 階段 1：牌的模型（enum/struct/derive/Vec/ownership）
- [ ] 階段 2：手牌與胡牌判斷（match/遞迴/Option/借用）
- [ ] 階段 3：遊戲流程引擎（enum 狀態機/Result/錯誤處理）
- [ ] 階段 4：台數計算與 CLI 對戰（trait/迭代器/closure）

## 常用指令

```bash
cargo test                 # TDD 主力
cargo run -p mahjong-cli   # 執行 CLI
cargo clippy               # lint
cargo fmt                  # 排版
```
