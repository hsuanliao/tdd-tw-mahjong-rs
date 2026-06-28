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

/// 暫時的佔位函式，純粹用來確認測試框架可以跑。
/// 下一階段（牌的模型）我們會把它換成真正的麻將邏輯。
///
/// 注意 Rust 函式簽名：參數 `left: u64` 是「名字: 型別」順序，
/// 回傳型別寫在 `->` 後面。`u64` 是 64 位元無號整數（≈ C# 的 `ulong`）。
pub fn add(left: u64, right: u64) -> u64 {
    // 函式最後一個「沒有分號」的運算式，就是回傳值（不需要寫 return）。
    left + right
}

// #[cfg(test)] 告訴編譯器：這個模組只有在「跑測試時」才編譯，
// 正式 build 不會包含，所以測試碼不會進到最終產物。
#[cfg(test)]
mod tests {
    // `use super::*;` 把外層模組（這個檔案）的所有東西引進來，才能呼叫 add。
    use super::*;

    // #[test] 標記這是一個測試函式 ≈ C# 的 [Fact] / [Test]。
    #[test]
    fn it_adds_two_numbers() {
        // assert_eq! 是巨集（macro，名稱後有 `!`），斷言兩值相等。
        assert_eq!(add(2, 2), 4);
    }
}
