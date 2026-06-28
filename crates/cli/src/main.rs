// 這是 binary crate 的進入點，`main` 函式就是程式起點 ≈ C# 的 Main(string[] args)。

// 從我們的引擎 crate 引用 add 函式。crate 名稱 mahjong-engine 在程式碼中
// 連字號會自動變底線 → mahjong_engine。
use mahjong_engine::add;

fn main() {
    // println! 是巨集，把字串印到 stdout。{} 是佔位符（類似 C# 的字串插值）。
    println!("台灣麻將引擎 CLI — 待開發");
    println!("引擎自我測試：2 + 2 = {}", add(2, 2));
}
