//! # domain 層 — 純領域模型
//!
//! 依架構規範（Hexagonal / Clean Architecture），這一層放**實體與值物件**，
//! 不碰任何 IO / 框架。麻將的「牌」「手牌」「胡牌規則」都會落在這裡。

// 宣告 tile 子模組 → 對應檔案 src/domain/tile.rs。
pub mod tile;
