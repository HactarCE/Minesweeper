use ndarray::prelude::*;
use rand::Rng;
use std::iter;

pub type Pos = (usize, usize);

/// All information about a Minesweeper game
#[derive(Clone, Debug)]
pub struct Board {
    /// The size of the board (height, width)
    size: Pos,
    /// An array of mines and numbers
    tiles: Array2<Tile>,
    /// An array describing visibility, flags, and question marks.
    tilestates: Array2<TileState>,
    /// The number of flags remaining.
    flags_left: isize,
    /// The number of undiscovered safe squares.
    safe_squares_left: isize,
}

impl Board {
    pub fn get_size(&self) -> &Pos {
        &self.size
    }
    pub fn get_tiles(&self) -> &Array2<Tile> {
        &self.tiles
    }
    pub fn get_tilestates(&self) -> &Array2<TileState> {
        &self.tilestates
    }
    pub fn get_flags_left(&self) -> isize {
        self.flags_left
    }
    pub fn get_safe_squares_left(&self) -> isize {
        self.safe_squares_left
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Mine,
    Safe(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileState {
    Hidden,
    Flagged,
    QuestionMark,
    Uncovered,
}

impl Board {
    /// Reveal tiles adjacent to a discovered number tile it has the correct
    /// number of adjacent flags. Return a (possibly empty) vector of the
    /// coordinates of tiles that changed state.
    fn reveal_adjacent(&mut self, pos: Pos) -> Vec<Pos> {
        let mut result = vec![];
        // Ensure that the tile is safe.
        if let Tile::Safe(n) = self.tiles[pos] {
            // Check the number of neighboring flags.
            let mut flags = 0;
            for neighbor_pos in self.neighbor_coords(pos) {
                match self.tilestates[neighbor_pos] {
                    TileState::Flagged => flags += 1,
                    _ => (),
                }
            }
            if flags == n {
                // Reveal all non-flag neighbors.
                for neighbor_pos in self.neighbor_coords(pos) {
                    match self.tilestates[neighbor_pos] {
                        TileState::Flagged | TileState::Uncovered => (),
                        TileState::Hidden | TileState::QuestionMark => {
                            self.reveal(neighbor_pos);
                            result.push(neighbor_pos);
                        }
                    }
                }
            }
        }
        result
    }

    /// Reveal a single tile. Return a (possibly empty) vector of the
    /// coordinates of tiles that changed state.
    fn reveal(&mut self, pos: Pos) -> Vec<Pos> {
        match self.tilestates[pos] {
            TileState::Hidden | TileState::QuestionMark => {
                self.tilestates[pos] = TileState::Uncovered;
                self.safe_squares_left -= 1;
                // Cascade zeros (recursively).
                let mut result = vec![pos];
                if self.tiles[pos] == Tile::Safe(0) {
                    for neighbor_pos in self.neighbor_coords(pos) {
                        if self.tilestates[neighbor_pos] == TileState::Hidden {
                            result.append(&mut self.reveal(neighbor_pos));
                        }
                    }
                }
                result
            }
            TileState::Flagged | TileState::Uncovered => vec![],
        }
    }

    /// Cycle the flag/question mark on a single tile. Return a (possibly empty)
    /// vector of the coordinates of tiles that changed state (which should be
    /// one, at most).
    fn cycle_flag(&mut self, pos: Pos) -> Vec<Pos> {
        let mut result = vec![pos];
        self.tilestates[pos] = match self.tilestates[pos] {
            TileState::Hidden => {
                self.flags_left -= 1;
                TileState::Flagged
            }
            TileState::Flagged => {
                self.flags_left += 1;
                TileState::QuestionMark
            }
            TileState::QuestionMark => TileState::Hidden,
            other => {
                result.pop();
                other
            }
        };
        result
    }

    /// Handle a left click on a tile and return a (possibly empty) vector of
    /// coordinates that changed state as a result.
    pub fn left_click(&mut self, pos: Pos) -> Vec<Pos> {
        match self.tilestates[pos] {
            TileState::Hidden | TileState::QuestionMark => self.reveal(pos),
            TileState::Flagged => vec![],
            TileState::Uncovered => self.reveal_adjacent(pos),
        }
    }

    /// Handle a right click on a tile and return a (possibly empty) vector of
    /// coordinates that changed state as a result.
    pub fn right_click(&mut self, pos: Pos) -> Vec<Pos> {
        self.cycle_flag(pos)
    }

    /// Reveal all tiles
    pub fn reveal_all(&mut self) {
        self.tilestates.fill(TileState::Uncovered);
    }

    /// Make a new empty board with a given size.
    pub fn make_empty(size: (usize, usize)) -> Board {
        Board {
            size: size,
            tiles: Array2::from_elem(size, Tile::Safe(0)),
            tilestates: Array2::from_elem(size, TileState::Hidden),
            flags_left: 0,
            safe_squares_left: (size.0 * size.1) as isize,
        }
    }

    /// Make a new random board with a given size and number of mines.
    pub fn make_random(size: (usize, usize), mines: usize) -> Result<Board, &'static str> {
        if size.0 < 1 || size.1 < 7 {
            return Err("Board size must be at least 7x1");
        }
        if 50 < size.0 || 50 < size.1 {
            return Err("Board size may not be greater than 50x50");
        }
        if mines <= 0 || size.0 * size.1 / 2 < mines {
            return Err("Mine density must be greater than 0% and no more than 50%");
        }
        let mut board = Board::make_empty(size);
        for _ in 0..mines {
            board.place_mine();
        }
        Ok(board)
    }

    /// Ensure that a particular starting place on the board is safe. Assumes
    /// that there is sufficient empty space in the board to relocate mines.
    pub fn ensure_safe_start(&mut self, start: Pos) {
        // Relocating mines once does not actually _ensure_ safety, so just keep
        // doing it until the mines have all gone elsewhere.
        loop {
            match self.tiles[start] {
                // Safe start! Return.
                Tile::Safe(0) => return,
                // Remove mines from this square and adjacent ones.
                _ => {
                    for neighbor_pos in self.neighbor_coords(start) {
                        self.relocate_mine(neighbor_pos);
                    }
                }
            }
        }
    }

    /// Place a mine on a board. Assumes that there is sufficient empty space to
    /// place a mine.
    fn place_mine(&mut self) {
        let mut rng = rand::thread_rng();
        // Keep trying to place the mine until successful.
        let mut pos: Pos;
        loop {
            let (h, w) = self.size;
            let y = rng.gen_range(0, h);
            let x = rng.gen_range(0, w);
            pos = (y, x);
            if let Tile::Safe(_) = self.tiles[pos] {
                self.tiles[pos] = Tile::Mine;
                break;
            }
        }
        for neighbor in self.neighbor_slice_mut(pos) {
            if let Tile::Safe(n) = neighbor {
                *n += 1
            }
        }
        self.flags_left += 1;
        self.safe_squares_left -= 1;
    }

    /// Remove a mine from a specific location on the board. Return true if the
    /// mine was removed, and false if there was no mine initially.
    fn remove_mine(&mut self, pos: Pos) -> bool {
        // If this square is already safe, do nothing.
        if let Tile::Safe(_) = self.tiles[pos] {
            return false;
        }
        // Reset this square so that it is not counted in the number of mines.
        // Use the number 8 to prevent underflow (since this cell itself is
        // included in the neighbor iterator).
        self.tiles[pos] = Tile::Safe(8);
        let mut mine_count = 0;
        for neighbor in self.neighbor_slice_mut(pos) {
            match neighbor {
                // Decrement each adjacent square.
                Tile::Safe(n) => *n -= 1,
                // Increment the number of mines adjacent to this square.
                Tile::Mine => mine_count += 1,
            }
        }
        self.tiles[pos] = Tile::Safe(mine_count);
        self.flags_left -= 1;
        self.safe_squares_left += 1;
        true
    }

    /// Remove a mine and replace it somewhere on the board (could be the same
    /// spot!).
    fn relocate_mine(&mut self, pos: Pos) {
        if self.remove_mine(pos) {
            self.place_mine()
        }
    }

    /// Recompute a square's number indicating adjacent mines.
    fn recompute_number(&mut self, pos: Pos) {
        if let Tile::Safe(_) = self.tiles[pos] {
            let mut n = 0;
            for neighbor in self.neighbor_slice(pos) {
                if let Tile::Mine = neighbor {
                    n += 1;
                }
            }
            self.tiles[pos] = Tile::Safe(n)
        }
    }

    /// Return a slice of the 3x3 box surrounding a square.
    fn neighbor_slice(&self, (y, x): Pos) -> ndarray::ArrayView2<Tile> {
        self.tiles
            .slice(s![self.y_nieghbor_range(y), self.x_neighbor_range(x)])
    }

    /// Return a mutable slice of the 3x3 box surrounding a square.
    fn neighbor_slice_mut(&mut self, (y, x): Pos) -> ndarray::ArrayViewMut2<Tile> {
        self.tiles
            .slice_mut(s![self.y_nieghbor_range(y), self.x_neighbor_range(x)])
    }

    /// Return an iterator of the coordinates of the 3x3 box surrounding a
    /// square.
    pub fn neighbor_coords(&self, (y, x): Pos) -> impl Iterator<Item = Pos> {
        (self.y_nieghbor_range(y))
            .zip(iter::repeat(self.x_neighbor_range(x)))
            .flat_map(|(y_, x_range)| iter::repeat(y_).zip(x_range))
    }

    fn y_nieghbor_range(&self, y: usize) -> std::ops::Range<usize> {
        (if y == 0 { 0 } else { y - 1 })..std::cmp::min(y + 2, self.size.0)
    }

    fn x_neighbor_range(&self, x: usize) -> std::ops::Range<usize> {
        (if x == 0 { 0 } else { x - 1 })..std::cmp::min(x + 2, self.size.1)
    }
}

#[derive(Debug, PartialEq)]
pub struct Difficulty {
    pub size: (usize, usize),
    pub mines: usize,
}

impl Difficulty {
    pub fn beginner() -> Difficulty {
        Difficulty {
            size: (9, 9),
            mines: 10,
        }
    }
    pub fn intermediate() -> Difficulty {
        Difficulty {
            size: (16, 16),
            mines: 40,
        }
    }
    pub fn expert() -> Difficulty {
        Difficulty {
            size: (16, 30),
            mines: 99,
        }
    }

    /// Make a new random board with a preset size and number of mines based on
    /// the given difficulty.
    pub fn new_game(&self) -> Result<Board, &'static str> {
        Board::make_random(self.size, self.mines)
    }

    pub fn with_density((h, w): (usize, usize), density: f32) -> Difficulty {
        Difficulty {
            size: (h, w),
            mines: ((h * w) as f32 * density).round() as usize,
        }
    }
}
