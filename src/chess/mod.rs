use chess::{Board, BoardStatus, ChessMove, Color, File, MoveGen, Piece, Rank, Square};

#[derive(Clone, Debug)]
pub struct GameState {
    pub board: Board,
}

impl GameState {
    pub fn new() -> Self {
        Self { board: Board::default() }
    }

    pub fn side_to_move(&self) -> Color {
        self.board.side_to_move()
    }

    pub fn is_terminal(&self) -> bool {
        !matches!(self.board.status(), BoardStatus::Ongoing)
    }

    pub fn legal_moves(&self) -> Vec<ChessMove> {
        MoveGen::new_legal(&self.board).collect()
    }

    pub fn apply_move(&mut self, mv: ChessMove) {
        self.board = self.board.make_move_new(mv);
    }

    pub fn board_string(&self) -> String {
        // Render ASCII board with coordinates
        let mut s = String::new();
        s.push_str("  a b c d e f g h\n");
        let ranks = [
            Rank::Eighth,
            Rank::Seventh,
            Rank::Sixth,
            Rank::Fifth,
            Rank::Fourth,
            Rank::Third,
            Rank::Second,
            Rank::First,
        ];
        let files = [
            File::A, File::B, File::C, File::D,
            File::E, File::F, File::G, File::H,
        ];
        for (i, rank) in ranks.iter().enumerate() {
            let rank_label = 8 - i;
            s.push_str(&format!("{rank_label} "));
            for file in files.iter() {
                let sq = Square::make_square(*rank, *file);
                let ch = if let Some(piece) = self.board.piece_on(sq) {
                    let mut c = match piece {
                        Piece::Pawn => 'p',
                        Piece::Knight => 'n',
                        Piece::Bishop => 'b',
                        Piece::Rook => 'r',
                        Piece::Queen => 'q',
                        Piece::King => 'k',
                    };
                    if let Some(color) = self.board.color_on(sq) {
                        if color == Color::White { c = c.to_ascii_uppercase(); }
                    }
                    c
                } else {
                    '.'
                };
                s.push(ch);
                s.push(' ');
            }
            s.push_str(&format!("{rank_label}\n"));
        }
        s.push_str("  a b c d e f g h\n");
        s
    }

    // --- Parsing UCI minimale (e2e4, e7e8q per promozione) ---
    pub fn parse_uci_move(mv: &str) -> Option<ChessMove> {
        if mv.len() < 4 { return None; }
        let from = &mv[0..2];
        let to = &mv[2..4];
        let from_sq = Self::uci_square(from)?;
        let to_sq = Self::uci_square(to)?;
        let promo = if mv.len() == 5 { Self::uci_promo_piece(mv.chars().nth(4)?) } else { None };
        Some(ChessMove::new(from_sq, to_sq, promo))
    }

    fn uci_square(s: &str) -> Option<Square> {
        if s.len() != 2 { return None; }
        let mut it = s.chars();
        let f = Self::uci_file(it.next()?)?;
        let r = Self::uci_rank(it.next()?)?;
        Some(Square::make_square(r, f))
    }

    fn uci_file(c: char) -> Option<File> {
        match c.to_ascii_lowercase() {
            'a' => Some(File::A),
            'b' => Some(File::B),
            'c' => Some(File::C),
            'd' => Some(File::D),
            'e' => Some(File::E),
            'f' => Some(File::F),
            'g' => Some(File::G),
            'h' => Some(File::H),
            _ => None,
        }
    }

    fn uci_rank(c: char) -> Option<Rank> {
        match c {
            '1' => Some(Rank::First),
            '2' => Some(Rank::Second),
            '3' => Some(Rank::Third),
            '4' => Some(Rank::Fourth),
            '5' => Some(Rank::Fifth),
            '6' => Some(Rank::Sixth),
            '7' => Some(Rank::Seventh),
            '8' => Some(Rank::Eighth),
            _ => None,
        }
    }

    fn uci_promo_piece(c: char) -> Option<Piece> {
        match c.to_ascii_lowercase() {
            'q' => Some(Piece::Queen),
            'r' => Some(Piece::Rook),
            'b' => Some(Piece::Bishop),
            'n' => Some(Piece::Knight),
            _ => None,
        }
    }
}


