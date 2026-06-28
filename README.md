# tdd-tw-mahjong-rs

用 **Rust** + **TDD** 從零實作的台灣 16 張麻將規則引擎，學習筆記型 side project。

## 目標

1. 透過實作麻將遊戲學會 Rust（enum / pattern matching / ownership / trait / 迭代器）。
2. 完成一個可在終端機對戰的台灣麻將遊戲（類似明星三缺一的玩法核心）。

## 專案結構

```
tdd-tw-mahjong-rs/        # Cargo workspace（≈ .sln）
├── crates/
│   ├── engine/           # 純規則引擎，無 IO（≈ 類別庫）
│   └── cli/              # 文字介面，玩一局（≈ 主控台程式）
```

## 開發進度（roadmap）

- [ ] 階段 0：環境與專案骨架
- [ ] 階段 1：牌的模型（萬/筒/條/字/花、洗牌發牌）
- [ ] 階段 2：手牌與胡牌判斷
- [ ] 階段 3：遊戲流程引擎（摸打、吃碰槓、回合狀態機）
- [ ] 階段 4：台數計算與 CLI 對戰

## 常用指令

```bash
cargo build            # 編譯整個 workspace
cargo test             # 跑所有測試（TDD 主力）
cargo run -p mahjong-cli   # 執行 CLI
cargo clippy           # 靜態檢查（lint）
cargo fmt              # 自動排版
```

## 開發方式

採 TDD（red → green → refactor）。每條麻將規則先寫測試，再實作。
