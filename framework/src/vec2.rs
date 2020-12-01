pub trait IntVec2: Sized {
    /// Gets the neighbors in the cardinal directions: N, E, S, W
    fn neighbors_cardinal(self) -> [Self; 4];
    /// Gets the neighbors in the ordinal directions: NE, SE, SW, NW
    fn neighbors_ordinal(self) -> [Self; 4];
    /// Gets the neighbors in both the ordinal and cardinal directions:
    /// N, NE, E, SE, S, SW, W, NW
    fn neighbors(self) -> [Self; 8];
}

impl IntVec2 for (i32, i32) {
    fn neighbors_cardinal(self) -> [Self; 4] {
        [
            (self.0, self.1 + 1),
            (self.0 + 1, self.1),
            (self.0, self.1 - 1),
            (self.0 - 1, self.1),
        ]
    }

    fn neighbors_ordinal(self) -> [Self; 4] {
        [
            (self.0 + 1, self.1 + 1),
            (self.0 + 1, self.1 - 1),
            (self.0 - 1, self.1 - 1),
            (self.0 - 1, self.1 + 1),
        ]
    }

    fn neighbors(self) -> [Self; 8] {
        [
            (self.0, self.1 + 1),
            (self.0 + 1, self.1 + 1),
            (self.0 + 1, self.1),
            (self.0 + 1, self.1 - 1),
            (self.0, self.1 - 1),
            (self.0 - 1, self.1 - 1),
            (self.0 - 1, self.1),
            (self.0 - 1, self.1 + 1),
        ]
    }
}
