//! # 台灣麻將規則引擎 (mahjong-engine)
//!
//! 這個 crate 只負責「規則與狀態」的純邏輯，**完全不碰畫面或輸入輸出**。
//! 好處：可以用單元測試把每條規則測到飽，CLI / GUI / Web 之後都共用這顆引擎。
//!
//! `//!` 是「模組層級文件註解」(會被 `cargo doc` 收進文件)；
//! `//` 則是一般註解。對照 C#：`///` XML 文件註解 vs `//` 一般註解。

// `pub mod domain;` 宣告一個公開的子模組，對應到檔案 `src/domain.rs`（及其下的子模組）。
// 對照 C#：模組 ≈ namespace + 檔案結構；`pub` ≈ public。Rust 預設是「私有」，要對外公開才加 pub。
pub mod domain;
