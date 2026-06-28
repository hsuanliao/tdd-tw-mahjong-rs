//! 牌 (Tile) 的領域模型。

// #[derive(...)] 請編譯器「自動產生」這些 trait 的實作，省去手寫樣板。
//   - Debug      → 能用 {:?} 印出（assert_eq! 失敗時要靠它顯示值）≈ C# 預設 ToString/除錯顯示
//   - Clone/Copy → 可複製。Copy 代表「廉價的位元複製」，賦值時用「複製」而非「移動」(ownership)
//   - PartialEq/Eq → 能用 == / != 比較 ≈ C# 覆寫 Equals / IEquatable
//   - Hash       → 之後要把牌放進 HashMap/HashSet 計數時需要
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Characters, // 萬
    Dots,       // 筒
    Bamboo,     // 索（條）
}

/// 風牌：東 南 西 北
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wind {
    East,  // 東
    South, // 南
    West,  // 西
    North, // 北
}

/// 三元牌：中 發 白
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dragon {
    Red,   // 中
    Green, // 發
    White, // 白
}

/// 花牌：台灣麻將共 8 張（春夏秋冬 + 梅蘭竹菊）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flower {
    Spring,        // 春
    Summer,        // 夏
    Autumn,        // 秋
    Winter,        // 冬
    Plum,          // 梅
    Orchid,        // 蘭
    Bamboo,        // 竹
    Chrysanthemum, // 菊
}

/// 一張麻將牌。這是「帶資料的 enum」：每個 variant 可攜帶不同型別的資料。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    /// 數牌：花色 + 點數（1..=9）。
    Suited(Suit, u8),
    /// 風牌。
    Wind(Wind),
    /// 三元牌。
    Dragon(Dragon),
    /// 花牌。
    Flower(Flower),
}

// 引入標準函式庫的 fmt 模組，才能寫 fmt::Display / fmt::Formatter / fmt::Result。
use std::fmt;

// `impl Trait for Type { ... }` 是「為某型別實作某 trait」≈ C# class : IInterface 後實作其方法。
// Display 規定你要提供一個 fmt 方法，把自己寫進傳入的 formatter。
impl fmt::Display for Tile {
    // &self：唯讀借用自己（不奪走所有權）≈ C# 的 this（但這裡明確是借用）。
    // f 是輸出緩衝；回傳 fmt::Result 表示寫入成功/失敗。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // match self 拆解 enum；編譯器會強制窮舉每個 variant，漏一個就編不過。
        match self {
            Tile::Suited(suit, rank) => {
                // 點數 → 國字。match 對 u8 必須窮舉，所以用 `_` 收尾接住 1~9 以外的值
                //（呼應先前提到的「Suited 目前允許非法點數」破綻，這裡先給個保底）。
                let n = match rank {
                    1 => "一",
                    2 => "二",
                    3 => "三",
                    4 => "四",
                    5 => "五",
                    6 => "六",
                    7 => "七",
                    8 => "八",
                    9 => "九",
                    _ => "?",
                };
                let s = match suit {
                    Suit::Characters => "萬",
                    Suit::Dots => "筒",
                    Suit::Bamboo => "索",
                };
                // write! 把格式化結果寫進 f；`{n}{s}` 是把變數直接內嵌進字串（Rust 2021+ 語法）。
                write!(f, "{n}{s}")
            }
            Tile::Wind(wind) => {
                let s = match wind {
                    Wind::East => "東",
                    Wind::South => "南",
                    Wind::West => "西",
                    Wind::North => "北",
                };
                write!(f, "{s}")
            }
            Tile::Dragon(dragon) => {
                let s = match dragon {
                    Dragon::Red => "中",
                    Dragon::Green => "發",
                    Dragon::White => "白",
                };
                write!(f, "{s}")
            }
            Tile::Flower(flower) => {
                let s = match flower {
                    Flower::Spring => "春",
                    Flower::Summer => "夏",
                    Flower::Autumn => "秋",
                    Flower::Winter => "冬",
                    Flower::Plum => "梅",
                    Flower::Orchid => "蘭",
                    Flower::Bamboo => "竹",
                    Flower::Chrysanthemum => "菊",
                };
                write!(f, "{s}")
            }
        }
    }
}

/// 發一副完整的台灣麻將牌（未洗牌）：數牌 108 + 字牌 28 + 花牌 8 = 144 張。
///
/// 回傳 `Vec<Tile>`：Vec 是「可成長的陣列」≈ C# 的 `List<T>`，資料配置在 heap 上。
/// 注意回傳型別是 `Vec<Tile>`（擁有所有權的值）——呼叫端會「拿到」這個 Vec 的所有權。
pub fn full_wall() -> Vec<Tile> {
    // `let mut` 才能修改。Rust 變數預設「不可變 (immutable)」，要可變必須明寫 mut。
    // 對照 C#：C# 變數預設可變，Rust 反過來，預設不可變更安全。
    let mut wall = Vec::new();

    // 數牌：三門 × 1~9 點，每種 4 張。
    // `for x in [...]` 直接走訪陣列；因為 Suit 是 Copy，迴圈裡拿到的是複製，原陣列不受影響。
    for suit in [Suit::Characters, Suit::Dots, Suit::Bamboo] {
        // `1..=9` 是「含尾端」的範圍 (1 到 9)。若寫 `1..9` 則不含 9。
        for rank in 1..=9u8 {
            for _ in 0..4 {
                // `_` 是「我不在乎這個值」的慣例命名（這裡只想重複 4 次）。
                wall.push(Tile::Suited(suit, rank));
            }
        }
    }

    // 風牌：4 種，每種 4 張。
    for wind in [Wind::East, Wind::South, Wind::West, Wind::North] {
        for _ in 0..4 {
            wall.push(Tile::Wind(wind));
        }
    }

    // 三元牌：3 種，每種 4 張。
    for dragon in [Dragon::Red, Dragon::Green, Dragon::White] {
        for _ in 0..4 {
            wall.push(Tile::Dragon(dragon));
        }
    }

    // 花牌：8 種，每種只有 1 張。
    for flower in [
        Flower::Spring,
        Flower::Summer,
        Flower::Autumn,
        Flower::Winter,
        Flower::Plum,
        Flower::Orchid,
        Flower::Bamboo,
        Flower::Chrysanthemum,
    ] {
        wall.push(Tile::Flower(flower));
    }

    // 最後一行沒有分號 → 這就是回傳值（把 wall 的所有權交給呼叫端）。
    wall
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 相同的牌相等_不同的牌不相等() {
        let 一萬_a = Tile::Suited(Suit::Characters, 1);
        let 一萬_b = Tile::Suited(Suit::Characters, 1);
        let 二萬 = Tile::Suited(Suit::Characters, 2);

        assert_eq!(一萬_a, 一萬_b); // 同樣是一萬 → 相等
        assert_ne!(一萬_a, 二萬); // 一萬 ≠ 二萬
    }

    #[test]
    fn 一副牌共有_144_張() {
        let wall = full_wall();
        assert_eq!(wall.len(), 144);
    }

    #[test]
    fn 每種數牌各有_4_張() {
        let wall = full_wall();

        // .iter() 借用 wall 產生迭代器（不奪走所有權）≈ C# 的 foreach/LINQ。
        // .filter(...) 收 closure（閉包，≈ C# lambda）；|&t| 用模式把 &Tile 解開成 Tile（Copy 才能這樣）。
        // .count() 算出符合的數量。
        let 一萬數量 = wall
            .iter()
            .filter(|&&t| t == Tile::Suited(Suit::Characters, 1))
            .count();

        assert_eq!(一萬數量, 4);
    }

    #[test]
    fn 花牌剛好_8_張() {
        let wall = full_wall();

        // matches! 巨集：判斷某值是否符合某個模式，回傳 bool。
        // Tile::Flower(_) 的 `_` 代表「是花牌就好，不在乎是哪一張」。
        let 花牌數量 = wall.iter().filter(|t| matches!(t, Tile::Flower(_))).count();

        assert_eq!(花牌數量, 8);
    }

    #[test]
    fn 牌可印成人類可讀字串() {
        // format!("{}", x) 會用到 Display；下面每一行都驗證一種 variant 的呈現。
        assert_eq!(format!("{}", Tile::Suited(Suit::Characters, 1)), "一萬");
        assert_eq!(format!("{}", Tile::Suited(Suit::Dots, 5)), "五筒");
        assert_eq!(format!("{}", Tile::Suited(Suit::Bamboo, 9)), "九索");
        assert_eq!(format!("{}", Tile::Wind(Wind::East)), "東");
        assert_eq!(format!("{}", Tile::Dragon(Dragon::Red)), "中");
        assert_eq!(format!("{}", Tile::Dragon(Dragon::White)), "白");
        assert_eq!(format!("{}", Tile::Flower(Flower::Spring)), "春");
        assert_eq!(format!("{}", Tile::Flower(Flower::Chrysanthemum)), "菊");
    }
}
