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
    Spring, // 春
    Summer, // 夏
    Autumn, // 秋
    Winter, // 冬
    Plum,   // 梅
    Orchid, // 蘭
    Bamboo, // 竹
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
}
