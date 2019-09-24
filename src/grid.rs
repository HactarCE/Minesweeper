use ndarray::prelude::*;
use rand::Rng;
use std::iter;

type Pos = (usize, usize);

/// All information about a Minesweeper game
#[derive(Clone, Debug)]
pub struct Board {
    /// The size of the board (height, width)
    size: Pos,
    /// An array of mines and numbers
    tiles: Array2<Tile>,
    /// An array describing visibility, flags, and question marks.
    tilestates: Array2<TileState>,
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
}

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Mine,
    Safe(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum TileState {
    Hidden,
    Flagged,
    QuestionMark,
    Uncovered,
}

impl Board {
    pub fn reveal_all(&mut self) {
        self.tilestates.fill(TileState::Uncovered);
    }

    /// Make a new empty board with a given size.
    pub fn make_empty(size: (usize, usize)) -> Board {
        Board {
            size: size,
            tiles: Array2::from_elem(size, Tile::Safe(0)),
            tilestates: Array2::from_elem(size, TileState::Hidden),
        }
    }

    /// Make a new random board with a given size and number of mines.
    pub fn make_random(size: (usize, usize), mines: usize) -> Result<Board, &'static str> {
        if mines > size.0 * size.1 / 2 {
            return Err("Mine density cannot be greater than 50%");
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
        // Relocating mines does not actually _ensure_ safety, so just keep
        // doing it until it works.
        loop {
            match self.tiles[start] {
                // Safe start! Return.
                Tile::Safe(0) => return,
                // Remove mines from this square and adjacent ones.
                _ => {
                    for neighbor_pos in self.neighbors_coords(start) {
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
    }

    /// Remove a mine from a specific location on the board. Return true if the
    /// mine was removed, and false if there was no mine initially.
    fn remove_mine(&mut self, pos: Pos) -> bool {
        // If this square is already safe, do nothing.
        if let Tile::Safe(_) = self.tiles[pos] {
            return false;
        }
        // Reset this square so that it is not counted in the number of mines.
        self.tiles[pos] = Tile::Safe(0);
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
    pub fn neighbors_coords(&self, (y, x): Pos) -> impl Iterator<Item = Pos> {
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
