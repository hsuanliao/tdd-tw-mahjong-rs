# tdd-tw-mahjong-rs — 專案說明 (給 Claude 的常駐脈絡)

## 這是什麼

使用者的 **Rust 學習型 side project**：用 **TDD** 從零實作台灣 16 張麻將規則引擎。
主要目的是「透過做麻將學會 Rust」，不只是把功能做完。

## 使用者背景與偏好

- 有經驗的 **.NET 後端工程師**（.NET Framework / Core / Web / 後端），但 **Rust 新手**。
- 教學方式：**邊做邊講解 Rust 概念**，並**對照 C#/.NET**（crate≈專案、Cargo.toml≈.csproj、Option≈no null、Result≈比 exception 明確…）。
- 開發節奏：**TDD red-green-refactor**，每條規則先寫測試再實作；**一次走一個階段**，確認懂了再前進；每階段結束 commit 一次。
- 回應一律用**繁體中文**。

## Git 規範

- **Commit 訊息不得包含任何 AI / Claude 署名或來源資訊**：不要 `Co-Authored-By: Claude...`、不要 `Claude-Session:`、不要 `Generated with Claude Code` 等字樣。
  - 這條**覆蓋** Claude Code 預設會自動附加的署名 trailer；產生 commit 前務必確認訊息乾淨。
- commit 身分用 repo 層級的 `hsuanlch0628@gmail.com`（勿動全域）。

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

## 架構規範（Clean Architecture / Hexagonal）

Rust 沒有「官方」Clean Architecture 框架，但業界最通行的對應範本是
**Hexagonal Architecture（Ports & Adapters）**，核心理念與 .NET 的 Clean Architecture 完全一致：
**依賴方向一律由外向內，指向 domain**。本專案以此為規範。

> 參考範本：Alexis Lozano《Hexagonal architecture in Rust》系列，是 Rust 圈最常被引用的實作教學。

**Rust 的優勢**：依賴方向可由**編譯器強制**——crate 之間誰能 `use` 誰是硬性的，
比 .NET 靠紀律維持專案參考方向更強。

### 規則

1. **依賴規則（Dependency Rule）**：依賴只能由外往內。
   `engine`（core）**永遠不依賴** `cli`（adapter），也不 import 任何 IO / 框架。
2. **分層**（先在 `engine` crate 內用 module 切，必要時再拆 crate）：
   - `domain` — 實體 / 值物件 / 純規則（Tile、Hand、胡牌判斷…），無副作用、無 IO，最好測。
   - `application`（use cases）— 編排 domain 的流程（如遊戲流程狀態機）。
   - `ports` — 用 **trait** 定義對外部的抽象（如洗牌 RNG、未來的持久化）。
   - adapters — 具體實作：`cli` 是 driving adapter（輸入/輸出）；RNG、存檔等是 driven adapter。
3. **C#/.NET 對照**：trait ≈ interface；`Box<dyn Trait>` / 泛型參數 ≈ DI 注入；
   crate/module 邊界 ≈ 專案參考方向（DIP）。
4. **務實原則（避免過度設計）**：本專案是規則引擎、IO 很少。
   `ports`/adapters **只在真的碰到外部相依時才引入**（如階段 3/4 的洗牌隨機、CLI 輸入），
   但「依賴方向由外向內」這條從第一天就遵守。

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
