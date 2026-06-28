// 這是 binary crate 的進入點，`main` 函式就是程式起點 ≈ C# 的 Main(string[] args)。
//
// 架構上 cli 是「driving adapter」：它依賴 engine（domain），engine 反過來不認得 cli。
// 依賴方向：cli ──► engine（由外向內）。

// 從引擎引用我們需要的型別與函式。crate 名 mahjong-engine 在程式碼中連字號變底線 → mahjong_engine。
// 用大括號一次引入同一路徑下的多個項目，類似 C# 的 using static / 多個 using。
use mahjong_engine::domain::tile::{Dragon, Flower, Suit, Tile, Wind, full_wall};

fn main() {
    println!("台灣麻將引擎 — 階段 1：牌的模型 demo");

    // 發一副完整的牌，印出總數。
    let wall = full_wall();
    println!("一副牌共 {} 張", wall.len());

    // 每種類別各取一張當示範，用 Display 印出牌面。
    let samples = [
        Tile::Suited(Suit::Characters, 1),
        Tile::Suited(Suit::Dots, 5),
        Tile::Suited(Suit::Bamboo, 9),
        Tile::Wind(Wind::East),
        Tile::Dragon(Dragon::Red),
        Tile::Flower(Flower::Spring),
    ];

    // iter().map(...).collect()：把每張牌轉成字串再收集成 Vec<String> ≈ C# 的 Select(...).ToList()。
    // to_string() 來自 Display：有了 Display 就自動有 ToString 能力。
    let faces: Vec<String> = samples.iter().map(|t| t.to_string()).collect();
    println!("牌面示範：{}", faces.join(" "));
}
