use std::{pin, process::id};

#[derive(Copy, Clone)]
pub struct Board {
    pub boards: [u64; 12],
    pub w_enpesant: u64,
    pub b_enpesant: u64,
    pub w_l_rook_moved: bool,
    pub w_r_rook_moved: bool,
    pub w_k_moved: bool,
    pub b_l_rook_moved: bool,
    pub b_r_rook_moved: bool,
    pub b_k_moved: bool,
    pub white_turn: bool,
}

impl Board {
    pub const W_PAWNS: usize = 0;
    pub const W_ROOKS: usize = 1;
    pub const W_KNIGHTS: usize = 2;
    pub const W_BISHOPS: usize = 3;
    pub const W_KINGS: usize = 4;
    pub const W_QUEENS: usize = 5;

    pub const B_PAWNS: usize = 6;
    pub const B_ROOKS: usize = 7;
    pub const B_KNIGHTS: usize = 8;
    pub const B_BISHOPS: usize = 9;
    pub const B_KINGS: usize = 10;
    pub const B_QUEENS: usize = 11;

    pub fn test_mate(&mut self) {
        println!("White tests:");

        println!("2 rooks");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::W_KINGS, 2);
        Board::add_piece(self, Board::B_BISHOPS, 16);
        Board::add_piece(self, Board::B_BISHOPS, 8);
        Board::add_piece(self, Board::B_BISHOPS, 24);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[2]);
        println!("Is mate: {}", Board::is_mate_white(self));
        Board::print_divider();
    }

    pub fn test_pawn(&mut self) {
        println!("White tests:");

        println!("moving forward");

        println!("Capturing");

        println!("En pesant");

        println!("Promoting");
    }

    pub fn test_castle(&mut self) {
        /**/
        println!("White tests:");

        println!("Blocked by white piece");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::W_KINGS, 4);
        Board::add_piece(self, Board::W_ROOKS, 0);
        Board::add_piece(self, Board::W_ROOKS, 2);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4]);
        Board::print_divider();

        self.white_turn = true;
        Board::move_piece(self, 4, 2, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("Not blocked");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::W_KINGS, 4);
        Board::add_piece(self, Board::W_ROOKS, 0);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4]);
        Board::print_divider();

        self.white_turn = true;
        Board::move_piece(self, 4, 2, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("Blocked by attacker");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::W_KINGS, 4);
        Board::add_piece(self, Board::W_ROOKS, 0);
        Board::add_piece(self, Board::B_ROOKS, 3 + 2 * 8);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4]);
        Board::print_divider();

        self.white_turn = true;
        Board::move_piece(self, 4, 2, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("King in check");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::W_KINGS, 4);
        Board::add_piece(self, Board::W_ROOKS, 0);
        Board::add_piece(self, Board::B_ROOKS, 4 + 2 * 8);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4]);
        Board::print_divider();

        self.white_turn = true;
        Board::move_piece(self, 4, 2, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("King has moved");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::W_KINGS, 4);
        Board::add_piece(self, Board::W_ROOKS, 0);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4]);
        Board::print_divider();

        self.white_turn = true;
        Board::move_piece(self, 4, 5, None);
        Board::print_board(*self);
        Board::print_divider();

        self.white_turn = true;
        Board::move_piece(self, 5, 4, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4]);
        Board::print_divider();

        self.white_turn = true;
        Board::move_piece(self, 4, 2, None);
        Board::print_board(*self);
        Board::print_divider();

        //------------------------------------------------------------------

        println!("Black tests:");

        self.b_k_moved = false;
        self.b_l_rook_moved = false;
        self.b_r_rook_moved = false;

        println!("Blocked by white piece");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::B_KINGS, 4 + 7 * 8);
        Board::add_piece(self, Board::B_ROOKS, 0 + 7 * 8);
        Board::add_piece(self, Board::B_ROOKS, 2 + 7 * 8);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4 + 7 * 8]);
        Board::print_divider();

        self.white_turn = false;
        Board::move_piece(self, 4 + 7 * 8, 2 + 7 * 8, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("Not blocked");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::B_KINGS, 4 + 7 * 8);
        Board::add_piece(self, Board::B_ROOKS, 0 + 7 * 8);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4 + 7 * 8]);
        Board::print_divider();

        self.white_turn = false;
        Board::move_piece(self, 4 + 7 * 8, 2 + 7 * 8, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("Blocked by attacker");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::B_KINGS, 4 + 7 * 8);
        Board::add_piece(self, Board::B_ROOKS, 0 + 7 * 8);
        Board::add_piece(self, Board::W_ROOKS, 3 + 2 * 8);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4 + 7 * 8]);
        Board::print_divider();

        self.white_turn = false;
        Board::move_piece(self, 4 + 7 * 8, 2 + 7 * 8, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("King in check");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::B_KINGS, 4 + 7 * 8);
        Board::add_piece(self, Board::B_ROOKS, 0 + 7 * 8);
        Board::add_piece(self, Board::W_ROOKS, 4 + 2 * 8);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4 + 7 * 8]);
        Board::print_divider();

        self.white_turn = false;
        Board::move_piece(self, 4 + 7 * 8, 2 + 7 * 8, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("King has moved");

        Board::clear_board(self);
        Board::print_divider();

        Board::add_piece(self, Board::B_KINGS, 4 + 7 * 8);
        Board::add_piece(self, Board::B_ROOKS, 0 + 7 * 8);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4 + 7 * 8]);
        Board::print_divider();

        self.white_turn = false;
        Board::move_piece(self, 4 + 7 * 8, 5 + 7 * 8, None);
        Board::print_board(*self);
        Board::print_divider();

        self.white_turn = false;
        Board::move_piece(self, 5 + 7 * 8, 4 + 7 * 8, None);
        Board::print_board(*self);
        Board::print_divider();

        println!("Legal moves of king");
        Board::print_bitboard(Board::get_all_legal_moves(&self)[4 + 7 * 8]);
        Board::print_divider();

        self.white_turn = false;
        Board::move_piece(self, 4 + 7 * 8, 2 + 7 * 8, None);
        Board::print_board(*self);
        Board::print_divider();
        /**/
    }

    pub fn move_piece(
        &mut self,
        start_position: usize,
        end_position: u64,
        promotion: Option<usize>,
    ) -> bool {
        let piece_type: isize = Board::piece_type_on_position(self, start_position);

        if piece_type == -1 {
            println!("Not a piece");
            return false;
        }

        if Board::board_is_black(piece_type as usize) && self.white_turn {
            println!("Whites turn");
            return false;
        }
        if Board::board_is_white(piece_type as usize) && !self.white_turn {
            println!("Black turn");
            return false;
        }

        self.white_turn = !self.white_turn;

        if Board::piece_type_on_position(self, 0) != Board::W_ROOKS as isize {
            self.w_l_rook_moved = true;
        }
        if Board::piece_type_on_position(self, 7) != Board::W_ROOKS as isize {
            self.w_r_rook_moved = true;
        }
        if Board::piece_type_on_position(self, 0 + 7 * 8) != Board::B_ROOKS as isize {
            self.b_l_rook_moved = true;
        }
        if Board::piece_type_on_position(self, 7 + 7 * 8) != Board::B_ROOKS as isize {
            self.b_r_rook_moved = true;
        }
        if Board::piece_type_on_position(self, 4) != Board::W_KINGS as isize {
            self.w_k_moved = true;
        }
        if Board::piece_type_on_position(self, 4 + 7 * 8) != Board::B_KINGS as isize {
            self.b_k_moved = true;
        }

        if Board::board_is_black(piece_type as usize) {
            self.b_enpesant = 0;
        }
        if Board::board_is_white(piece_type as usize) {
            self.w_enpesant = 0;
        }

        if piece_type == Board::W_PAWNS as isize {
            if !Board::board_is_white(promotion.unwrap_or(Board::W_QUEENS))
                || promotion.unwrap_or(Board::W_QUEENS) == Board::W_PAWNS
                || promotion.unwrap_or(Board::W_QUEENS) == Board::W_KINGS
            {
                println!("Illegal promotion");
                return false;
            }
        }
        if piece_type == Board::B_PAWNS as isize {
            if !Board::board_is_black(promotion.unwrap_or(Board::B_QUEENS))
                || promotion.unwrap_or(Board::B_QUEENS) == Board::B_PAWNS
                || promotion.unwrap_or(Board::B_QUEENS) == Board::B_KINGS
            {
                println!("Illegal promotion");
                return false;
            }
        }

        let legal_moves = Board::get_all_legal_moves(self);
        let start_board: u64 = 1 << start_position;
        let end_board: u64 = 1 << end_position;

        if legal_moves[start_position] & end_board > 0 {
            println!("Moving piece");
            self.boards[piece_type as usize] += end_board;
            self.boards[piece_type as usize] = self.boards[piece_type as usize] & !start_board;

            for i in 0..self.boards.len() {
                if i != piece_type as usize {
                    self.boards[i] = self.boards[i] & !end_board;
                }
            }

            if end_position as isize == start_position as isize + 2 {
                if piece_type == Board::W_KINGS as isize {
                    self.boards[Board::W_ROOKS] += end_board >> 1;
                    self.boards[Board::W_ROOKS] = self.boards[Board::W_ROOKS] & !(end_board << 1);
                }
                if piece_type == Board::B_KINGS as isize {
                    self.boards[Board::B_ROOKS] += end_board >> 1;
                    self.boards[Board::B_ROOKS] = self.boards[Board::B_ROOKS] & !(end_board << 1);
                }
            }
            if end_position as isize == start_position as isize - 2 {
                if piece_type == Board::W_KINGS as isize {
                    self.boards[Board::W_ROOKS] += end_board << 1;
                    self.boards[Board::W_ROOKS] = self.boards[Board::W_ROOKS] & !(end_board >> 2);
                }
                if piece_type == Board::B_KINGS as isize {
                    self.boards[Board::B_ROOKS] += end_board << 1;
                    self.boards[Board::B_ROOKS] = self.boards[Board::B_ROOKS] & !(end_board >> 2);
                }
            }
            if end_position >= 8 {
                if (self.w_enpesant >> end_position - 8) & 1 == 1 {
                    self.w_enpesant -= 1 << end_position - 8;
                }
            }

            if piece_type == Board::W_PAWNS as isize {
                if end_position / 8 == 7 {
                    self.boards[Board::W_PAWNS] = self.boards[Board::W_PAWNS] & !end_board;
                    Board::add_piece(self, promotion.unwrap_or(Board::W_QUEENS), end_position);
                }
                if start_position / 8 == 1 && end_position / 8 == 3 {
                    self.w_enpesant += 1 << 2 * 8 + (start_position % 8);
                }
                if start_position / 8 == 3
                    && (self.w_enpesant >> 2 * 8 + start_position % 8) & 1 == 1
                {
                    self.w_enpesant -= 1 << 2 * 8 + (start_position % 8);
                }
                if (self.b_enpesant >> end_position) & 1 == 1 {
                    self.boards[Board::B_PAWNS] -= 1 << end_position - 8;
                }
            }

            if end_position <= 55 {
                if (self.b_enpesant >> end_position + 8) & 1 == 1 {
                    self.b_enpesant -= 1 << end_position + 8;
                }
            }

            if piece_type == Board::B_PAWNS as isize {
                if end_position / 8 == 0 {
                    self.boards[Board::B_PAWNS] = self.boards[Board::B_PAWNS] & !end_board;
                    Board::add_piece(self, promotion.unwrap_or(Board::B_QUEENS), end_position);
                }
                if start_position / 8 == 6 && end_position / 8 == 4 {
                    self.w_enpesant += 1 << 5 * 8 + (start_position % 8);
                }
                if start_position / 8 == 4
                    && (self.w_enpesant >> 5 * 8 + start_position % 8) & 1 == 1
                {
                    self.w_enpesant -= 1 << 5 * 8 + (start_position % 8);
                }
                if (self.w_enpesant >> end_position) & 1 == 1 {
                    self.boards[Board::W_PAWNS] -= 1 << end_position + 8;
                }
            }

            return true;
        } else {
            println!("{} to {} is an illegal move", start_position, end_position);
            return false;
        }
    }

    pub fn is_mate_white(board: &Board) -> bool {
        let all_moves: [u64; 64] = Board::get_all_legal_moves(board);

        let mut white_moves: u64 = 0;
        let mut black_moves: u64 = 0;

        let mut king_board: u64 = 0;

        for i in 0..64 {
            let piece_type = Board::piece_type_on_position(board, i);
            if piece_type == -1 {
                continue;
            }
            if Board::board_is_white(piece_type as usize) {
                white_moves = white_moves | all_moves[i];
            }
            if Board::board_is_black(piece_type as usize) {
                black_moves = black_moves | all_moves[i];
            }
            if piece_type == Board::W_KINGS as isize {
                king_board = 1 << i;
            }
        }

        println!("Black moves:");
        Board::print_bitboard(black_moves);
        println!("White moves:");
        Board::print_bitboard(white_moves);
        println!("King board: ");
        Board::print_bitboard(king_board);

        if white_moves == 0 && king_board & black_moves > 0 {
            return true;
        }
        return false;
    }

    pub fn is_mate_black(board: &Board) -> bool {
        let all_moves: [u64; 64] = Board::get_all_legal_moves(board);

        let mut white_moves: u64 = 0;
        let mut black_moves: u64 = 0;

        let mut king_board: u64 = 0;

        for i in 0..64 {
            let piece_type = Board::piece_type_on_position(board, i);
            if piece_type == -1 {
                continue;
            }
            if Board::board_is_white(piece_type as usize) {
                white_moves = white_moves | all_moves[i];
            }
            if Board::board_is_black(piece_type as usize) {
                black_moves = black_moves | all_moves[i];
            }
            if piece_type == Board::W_KINGS as isize {
                king_board = 1 << i;
            }
        }

        println!("Black moves:");
        Board::print_bitboard(black_moves);
        println!("White moves:");
        Board::print_bitboard(white_moves);
        println!("King board: ");
        Board::print_bitboard(king_board);

        if black_moves == 0 && king_board & white_moves > 0 {
            return true;
        }
        return false;
    }

    pub fn print_divider() {
        println!("------------------------------");
    }

    pub fn clear_board(&mut self) {
        self.boards = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    }

    pub fn set_standard_board(&mut self) {
        let mut standard_board: [u64; 12] = [0; 12];

        // row 2
        // 0xff = hexadecimal ff
        // hexadecimal f = 15 = 1111
        // ff = 1111 1111
        //W Pawns
        standard_board[0] = 0xff_u64 << (1 * 8);
        //W Rooks
        standard_board[1] = 1 << (0) | 1 << (7);
        //W Knights
        standard_board[2] = 1 << (1) | 1 << (6);
        //W Bishops
        standard_board[3] = 1 << (2) | 1 << (5);
        //W Kings
        standard_board[4] = 1 << (3);
        //W Queens
        standard_board[5] = 1 << (4);

        //B Pawns
        standard_board[6] = 0xff << (6 * 8);
        //B Rooks
        standard_board[7] = 1 << (7 * 8) | 1 << (7 + 7 * 8);
        //B Knights
        standard_board[8] = 1 << (1 + 7 * 8) | 1 << (6 + 7 * 8);
        //B Bishops
        standard_board[9] = 1 << (2 + 7 * 8) | 1 << (5 + 7 * 8);
        //B Kings
        standard_board[10] = 1 << (3 + 7 * 8);
        //B Queens
        standard_board[11] = 1 << (4 + 7 * 8);

        self.boards = standard_board;
    }

    pub fn add_piece(&mut self, piece_type: usize, index: u64) {
        if piece_type > 11 || piece_type < 0 {
            return;
        }
        if self.boards[piece_type] >> index & 1 == 0 {
            self.boards[piece_type] += 1 << index;
        }
    }

    pub fn print_bitboards(boards: [u64; 64]) {
        for i in 0..64 {
            println!("-- Board {} --", i);
            Board::print_bitboard(boards[i]);
        }
    }

    pub fn print_bitboard(board: u64) {
        for i in 0..64 {
            if i % 8 == 7 {
                println!("{} ", ((board >> i) & 1));
            } else {
                print!("{} ", ((board >> i) & 1));
            }
        }
    }

    pub fn board_is_white(board: usize) -> bool {
        if board < Board::B_PAWNS {
            return true;
        }
        return false;
    }

    pub fn board_is_black(board: usize) -> bool {
        if board >= Board::B_PAWNS {
            return true;
        }
        return false;
    }

    //Returns -1 if no returns int of board if yes
    pub fn piece_type_on_position(boards: &Board, position: usize) -> isize {
        for i in 0..boards.boards.len() {
            let board: u64 = boards.boards[i];
            if (board >> position) & 1 == 1 {
                return i as isize;
            }
        }
        return -1;
    }

    pub fn print_board(board: Board) {
        let mut board_list: [&str; 64] = [" . "; 64];

        for i in 0..board.boards.len() {
            let bin_board = String::from(format!("{:b})", board.boards[i]));
            let mut symbol: &str = "";
            match i {
                Board::W_PAWNS => symbol = "wP ",
                Board::W_ROOKS => symbol = "wR ",
                Board::W_KNIGHTS => symbol = "wKn",
                Board::W_BISHOPS => symbol = "wB ",
                Board::W_KINGS => symbol = "wK ",
                Board::W_QUEENS => symbol = "wQ ",

                Board::B_PAWNS => symbol = "bP ",
                Board::B_ROOKS => symbol = "bR ",
                Board::B_KNIGHTS => symbol = "bKn",
                Board::B_BISHOPS => symbol = "bB ",
                Board::B_KINGS => symbol = "bK ",
                Board::B_QUEENS => symbol = "bQ ",

                _ => println!("How did we get here"),
            }
            for j in 0..bin_board.len() {
                if bin_board.chars().nth(j).unwrap() == '1' {
                    board_list[bin_board.len() - j - 2] = symbol;
                }
            }
        }

        let mut board_row: String = String::from("");
        for i in 0..board_list.len() {
            board_row += board_list[i];
            board_row += " ";
            //println!("Board row: {}", board_row);
            if (i) % 8 == 7 {
                println!("{}", board_row);
                board_row = String::from("");
            }
        }
    }

    pub fn get_all_legal_moves(boards: &Board) -> [u64; 64] {
        //Varje plats är en bitboard.
        //Varje 1a i bitboarden säger att pjäsen på det indexet i arrayen kan gå till 1an
        //Gör först möjliga drag sedan lagliga drag genom att ta bort pinnade pjäser
        let mut moves: [u64; 64] = [0; 64];
        let mut attacks: [u64; 64] = [0; 64];

        for i in 0..64 as u64 {
            for j in 0..boards.boards.len() {
                let current_board = boards.boards[j];

                if (current_board >> i) & 1 == 1 {
                    moves[i as usize] = Board::get_possible_moves(boards, i, j);
                    attacks[i as usize] = Board::get_possible_attacks(boards, i, j);
                }
            }
        }

        moves = Board::limit_kings(boards, moves, attacks);

        moves = Board::check(boards, attacks, moves);
        return moves;
    }

    pub fn is_different_color(piece_1_type: isize, piece_2_type: isize) -> bool {
        if piece_1_type == -1 || piece_2_type == -1 {
            return false;
        }
        if piece_1_type >= Board::B_PAWNS as isize && piece_2_type < Board::B_PAWNS as isize {
            return true;
        } else if piece_1_type < Board::B_PAWNS as isize && piece_2_type >= Board::B_PAWNS as isize
        {
            return true;
        }
        return false;
    }
    // Separera attack och move bitboard
    pub fn get_possible_moves(boards: &Board, piece_position: u64, piece_type: usize) -> u64 {
        //Lista med möjliga drag
        let mut possible_moves: u64 = 0;

        match piece_type {
            Board::W_PAWNS | Board::B_PAWNS => {
                if piece_type == Board::W_PAWNS && piece_position + 8 <= 63 {
                    // blockerad på raden framför
                    if Board::piece_type_on_position(boards, piece_position as usize + 8) == -1 {
                        possible_moves += 1 << (piece_position + 8);
                        if Board::piece_type_on_position(boards, piece_position as usize + 16) == -1
                            && piece_position / 8 == 1
                        {
                            possible_moves += 1 << (piece_position + 16);
                        }
                    }

                    // pjäs på raden framför till vänster och byter inte rad

                    if (piece_position as isize % 8) - 1 == (piece_position as isize - 1) % 8
                        && ((Board::piece_type_on_position(boards, piece_position as usize + 7)
                            != -1
                            && Board::is_different_color(
                                piece_type as isize,
                                Board::piece_type_on_position(boards, piece_position as usize + 7),
                            ))
                            || (boards.b_enpesant >> piece_position + 7) & 1 == 1)
                    {
                        possible_moves += 1 << (piece_position + 7)
                    }
                    if (piece_position % 8) + 1 == (piece_position + 1) % 8
                        && (Board::piece_type_on_position(boards, piece_position as usize + 9)
                            != -1
                            && Board::is_different_color(
                                piece_type as isize,
                                Board::piece_type_on_position(boards, piece_position as usize + 9),
                            )
                            || (boards.b_enpesant >> piece_position + 9) & 1 == 1)
                    {
                        possible_moves += 1 << (piece_position + 9)
                    }
                }
                if piece_type == Board::B_PAWNS && piece_position / 8 != 0 {
                    if Board::piece_type_on_position(boards, piece_position as usize - 8) == -1 {
                        possible_moves += 1 << (piece_position - 8);
                        if Board::piece_type_on_position(boards, piece_position as usize - 16) == -1
                            && piece_position / 8 == 6
                        {
                            possible_moves += 1 << (piece_position - 16);
                        }
                    }

                    if (piece_position as isize % 8) - 1 == (piece_position as isize - 1) % 8
                        && (Board::piece_type_on_position(boards, piece_position as usize - 9)
                            != -1
                            && Board::is_different_color(
                                piece_type as isize,
                                Board::piece_type_on_position(boards, piece_position as usize - 9),
                            )
                            || (boards.w_enpesant >> piece_position - 9) & 1 == 1)
                    {
                        possible_moves += 1 << (piece_position - 9)
                    }
                    if (piece_position % 8) + 1 == (piece_position + 1) % 8
                        && (Board::piece_type_on_position(boards, piece_position as usize - 7)
                            != -1
                            && Board::is_different_color(
                                piece_type as isize,
                                Board::piece_type_on_position(boards, piece_position as usize - 7),
                            )
                            || (boards.w_enpesant >> piece_position + 7) & 1 == 1)
                    {
                        possible_moves += 1 << (piece_position - 7)
                    }
                }
            }
            Board::W_ROOKS | Board::B_ROOKS => {
                //från rook ut åt höger. Kan bli error vid 8..8?
                for i in 1..=(7 - (piece_position % 8)) {
                    let looked_at_position: u64 = piece_position + i;
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);

                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_ROOKS
                            && Board::board_is_white(piece_type_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_ROOKS
                            && Board::board_is_black(piece_type_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_ROOKS
                            && Board::board_is_black(piece_type_at_position as usize)
                            && piece_type_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_ROOKS
                            && Board::board_is_white(piece_type_at_position as usize)
                            && piece_type_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }
                //Från rook neråt
                //1 kan vara större än piece_position/8
                for i in 1..=(piece_position / 8) {
                    let looked_at_position: u64 = piece_position - (8 * i);
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);

                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_ROOKS
                            && Board::board_is_white(piece_type_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_ROOKS
                            && Board::board_is_black(piece_type_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_ROOKS
                            && Board::board_is_black(piece_type_at_position as usize)
                            && piece_type_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_ROOKS
                            && Board::board_is_white(piece_type_at_position as usize)
                            && piece_type_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }

                //Från rook till vänster
                for i in 1..=(piece_position % 8) {
                    let looked_at_position: u64 = piece_position - i;
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_ROOKS
                            && Board::board_is_white(piece_type_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_ROOKS
                            && Board::board_is_black(piece_type_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_ROOKS
                            && Board::board_is_black(piece_type_at_position as usize)
                            && piece_type_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_ROOKS
                            && Board::board_is_white(piece_type_at_position as usize)
                            && piece_type_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }

                //Från rook uppåt
                //1 kan vara större än piece_position/8
                for i in 1..=(7 - (piece_position / 8)) {
                    let looked_at_position: u64 = piece_position + (8 * i);
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_ROOKS
                            && Board::board_is_white(piece_type_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_ROOKS
                            && Board::board_is_black(piece_type_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_ROOKS
                            && Board::board_is_black(piece_type_at_position as usize)
                            && piece_type_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_ROOKS
                            && Board::board_is_white(piece_type_at_position as usize)
                            && piece_type_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }
            }
            Board::W_KNIGHTS | Board::B_KNIGHTS => {
                let mut looked_at_position: i32 = 0;
                for i in 0..8 {
                    match i {
                        0 => {
                            //plats upp till vänster
                            looked_at_position = piece_position as i32 + (2 * 8) - 1;
                            if looked_at_position % 8 != (piece_position as i32 % 8) - 1 {
                                continue;
                            }
                        }
                        1 => {
                            //plats upp till höger
                            looked_at_position = piece_position as i32 + (2 * 8) + 1;
                            if looked_at_position % 8 != (piece_position as i32 % 8) + 1 {
                                continue;
                            }
                        }
                        2 => {
                            //plats höger upp
                            looked_at_position = piece_position as i32 + (1 * 8) + 2;
                            if looked_at_position % 8 != (piece_position as i32 % 8) + 2 {
                                continue;
                            }
                        }
                        3 => {
                            //plats höger ner
                            looked_at_position = piece_position as i32 + (-1 * 8) + 2;
                            if looked_at_position % 8 != (piece_position as i32 % 8) + 2 {
                                continue;
                            }
                        }
                        4 => {
                            //plats ner höger
                            looked_at_position = piece_position as i32 + (-2 * 8) + 1;
                            if looked_at_position % 8 != (piece_position as i32 % 8) + 1 {
                                continue;
                            }
                        }
                        5 => {
                            //ner vänster
                            looked_at_position = piece_position as i32 + (-2 * 8) - 1;
                            if looked_at_position % 8 != (piece_position as i32 % 8) - 1 {
                                continue;
                            }
                        }
                        6 => {
                            //vänster ner
                            looked_at_position = piece_position as i32 + (-1 * 8) - 2;
                            if looked_at_position % 8 != (piece_position as i32 % 8) - 2 {
                                continue;
                            }
                        }
                        7 => {
                            //vänster upp
                            looked_at_position = piece_position as i32 + (1 * 8) - 2;
                            if looked_at_position % 8 != (piece_position as i32 % 8) - 2 {
                                continue;
                            }
                        }
                        _ => {}
                    }
                    if looked_at_position <= 63 && looked_at_position >= 0 {
                        let piece_type_at_position: isize =
                            Board::piece_type_on_position(boards, looked_at_position as usize);
                        if piece_type_at_position >= 0 {
                            if piece_type == Board::W_KNIGHTS
                                && piece_type_at_position >= Board::B_PAWNS as isize
                            {
                                possible_moves += 1 << looked_at_position as u32;
                                //possible_moves.push((piece_position, looked_at_position as u32));
                            } else if piece_type == Board::B_KNIGHTS
                                && piece_type_at_position < Board::B_PAWNS as isize
                            {
                                possible_moves += 1 << looked_at_position as u32;
                                //possible_moves.push((piece_position, looked_at_position as u32));
                            }
                        } else {
                            possible_moves += 1 << looked_at_position as u32;
                            //possible_moves.push((piece_position, looked_at_position as u32));
                        }
                    }
                }
            }
            Board::B_BISHOPS | Board::W_BISHOPS => {
                //Upp höger
                //brädets bredd - pjäs.x - 1
                //Brädets höjd - pjäs.y - 1
                for i in 1..=u64::min(7 - (piece_position % 8), 7 - (piece_position / 8)) {
                    let looked_at_position: u64 = piece_position + (8 * i) + i;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_BISHOPS
                            && Board::board_is_white(piece_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_BISHOPS
                            && Board::board_is_black(piece_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_BISHOPS
                            && Board::board_is_black(piece_at_position as usize)
                            && piece_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_BISHOPS
                            && Board::board_is_white(piece_at_position as usize)
                            && piece_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }
                //Ner höger
                //Brädets bredd - pjäs.x - 1
                //Pjäs.y
                for i in 1..=u64::min(7 - (piece_position % 8), piece_position / 8) {
                    let looked_at_position: u64 = (piece_position - (8 * i)) + i;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_BISHOPS
                            && Board::board_is_white(piece_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_BISHOPS
                            && Board::board_is_black(piece_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_BISHOPS
                            && Board::board_is_black(piece_at_position as usize)
                            && piece_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_BISHOPS
                            && Board::board_is_white(piece_at_position as usize)
                            && piece_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }
                //Ner vänster
                //Brädets bredd - pjäs.x
                //Pjäs.y
                for i in 1..=u64::min(piece_position % 8, piece_position / 8) {
                    let looked_at_position: u64 = (piece_position - (8 * i)) - i;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_BISHOPS
                            && Board::board_is_white(piece_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_BISHOPS
                            && Board::board_is_black(piece_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_BISHOPS
                            && Board::board_is_black(piece_at_position as usize)
                            && piece_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_BISHOPS
                            && Board::board_is_white(piece_at_position as usize)
                            && piece_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }
                //Upp vänster
                //Brädets bredd - Pjäs.x
                //Pjäs.y
                for i in 1..=u64::min(piece_position % 8, 7 - (piece_position / 8)) {
                    let looked_at_position: u64 = (piece_position + (8 * i)) - i;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_BISHOPS
                            && Board::board_is_white(piece_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_BISHOPS
                            && Board::board_is_black(piece_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_BISHOPS
                            && Board::board_is_black(piece_at_position as usize)
                            && piece_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_BISHOPS
                            && Board::board_is_white(piece_at_position as usize)
                            && piece_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }
            }
            Board::B_QUEENS | Board::W_QUEENS => {
                //Upp höger
                //brädets bredd - pjäs.x - 1
                //Brädets höjd - pjäs.y - 1
                for i in 1..=u64::min(7 - (piece_position % 8), 7 - (piece_position / 8)) {
                    let looked_at_position: u64 = piece_position + (8 * i) + i;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_white(piece_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_black(piece_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_black(piece_at_position as usize)
                            && piece_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_white(piece_at_position as usize)
                            && piece_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }
                //Ner höger
                //Brädets bredd - pjäs.x - 1
                //Pjäs.y
                for i in 1..=u64::min(7 - (piece_position % 8), piece_position / 8) {
                    let looked_at_position: u64 = (piece_position - (8 * i)) + i;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_white(piece_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_black(piece_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_black(piece_at_position as usize)
                            && piece_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_white(piece_at_position as usize)
                            && piece_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }
                //Ner vänster
                //Brädets bredd - pjäs.x
                //Pjäs.y
                for i in 1..=u64::min(piece_position % 8, piece_position / 8) {
                    let looked_at_position: u64 = (piece_position - (8 * i)) - i;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_white(piece_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_black(piece_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_black(piece_at_position as usize)
                            && piece_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_white(piece_at_position as usize)
                            && piece_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }
                //Upp vänster
                //Brädets bredd - Pjäs.x
                //Pjäs.y
                for i in 1..=u64::min(piece_position % 8, 7 - (piece_position / 8)) {
                    let looked_at_position: u64 = (piece_position + (8 * i)) - i;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_white(piece_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_black(piece_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_black(piece_at_position as usize)
                            && piece_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_white(piece_at_position as usize)
                            && piece_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }

                //från rook ut åt höger. Kan bli error vid 8..8?
                for i in 1..=(7 - (piece_position % 8)) {
                    let looked_at_position: u64 = piece_position + i;
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);

                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_white(piece_type_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_black(piece_type_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_black(piece_type_at_position as usize)
                            && piece_type_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_white(piece_type_at_position as usize)
                            && piece_type_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                }

                //Från rook neråt
                //1 kan vara större än piece_position/8
                for i in 1..=(piece_position / 8) {
                    let looked_at_position: u64 = piece_position - (8 * i);
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_white(piece_type_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_black(piece_type_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_black(piece_type_at_position as usize)
                            && piece_type_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_white(piece_type_at_position as usize)
                            && piece_type_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }

                //Från rook till vänster
                for i in 1..=(piece_position % 8) {
                    let looked_at_position: u64 = piece_position - i;
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_white(piece_type_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_black(piece_type_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_black(piece_type_at_position as usize)
                            && piece_type_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_white(piece_type_at_position as usize)
                            && piece_type_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }

                //Från rook uppåt
                //1 kan vara större än piece_position/8
                for i in 1..=(7 - (piece_position / 8)) {
                    let looked_at_position: u64 = piece_position + (8 * i);
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_white(piece_type_at_position as usize)
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_black(piece_type_at_position as usize)
                        {
                            break;
                        }
                    }
                    possible_moves += 1 << looked_at_position;
                    if piece_type_at_position >= 0 {
                        if piece_type == Board::W_QUEENS
                            && Board::board_is_black(piece_type_at_position as usize)
                            && piece_type_at_position != Board::B_KINGS as isize
                        {
                            break;
                        }
                        if piece_type == Board::B_QUEENS
                            && Board::board_is_white(piece_type_at_position as usize)
                            && piece_type_at_position != Board::W_KINGS as isize
                        {
                            break;
                        }
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }
            }
            Board::W_KINGS | Board::B_KINGS => {
                if (piece_position + 1) % 8 == (piece_position % 8) + 1 {
                    if Board::is_different_color(
                        Board::piece_type_on_position(boards, piece_position as usize + 1),
                        piece_type as isize,
                    ) || Board::piece_type_on_position(boards, piece_position as usize + 1) == -1
                    {
                        possible_moves += 1 << (piece_position + 1);
                    }
                    //possible_moves.push((piece_position, piece_position + 1));
                    if (piece_position + 9) <= 63 {
                        if Board::is_different_color(
                            Board::piece_type_on_position(boards, piece_position as usize + 9),
                            piece_type as isize,
                        ) || Board::piece_type_on_position(boards, piece_position as usize + 9)
                            == -1
                        {
                            possible_moves += 1 << (piece_position + 9);
                        }
                        //possible_moves.push((piece_position, piece_position + 9));
                    }
                    if (piece_position as i32 - 7) >= 0 {
                        if Board::is_different_color(
                            Board::piece_type_on_position(boards, piece_position as usize - 7),
                            piece_type as isize,
                        ) || Board::piece_type_on_position(boards, piece_position as usize - 7)
                            == -1
                        {
                            possible_moves += 1 << (piece_position - 7);
                        }
                        //possible_moves.push((piece_position, piece_position - 7));
                    }
                }
                if (piece_position as i32 - 1) % 8 == (piece_position as i32 % 8) - 1 {
                    if Board::is_different_color(
                        Board::piece_type_on_position(boards, piece_position as usize - 1),
                        piece_type as isize,
                    ) || Board::piece_type_on_position(boards, piece_position as usize - 1) == -1
                    {
                        possible_moves += 1 << (piece_position - 1);
                    }
                    //possible_moves.push((piece_position, piece_position + 1));
                    if (piece_position + 7) <= 63 {
                        if Board::is_different_color(
                            Board::piece_type_on_position(boards, piece_position as usize + 7),
                            piece_type as isize,
                        ) || Board::piece_type_on_position(boards, piece_position as usize + 7)
                            == -1
                        {
                            possible_moves += 1 << (piece_position + 7);
                        }

                        //possible_moves.push((piece_position, piece_position + 7));
                    }
                    if (piece_position as i32 - 9) >= 0 {
                        if Board::is_different_color(
                            Board::piece_type_on_position(boards, piece_position as usize - 9),
                            piece_type as isize,
                        ) || Board::piece_type_on_position(boards, piece_position as usize - 9)
                            == -1
                        {
                            possible_moves += 1 << (piece_position - 9);
                        }

                        //possible_moves.push((piece_position, piece_position - 9));
                    }
                }
                if (piece_position + 8) <= 63 {
                    if Board::is_different_color(
                        Board::piece_type_on_position(boards, piece_position as usize + 8),
                        piece_type as isize,
                    ) || Board::piece_type_on_position(boards, piece_position as usize + 8) == -1
                    {
                        possible_moves += 1 << (piece_position + 8);
                    }
                    //possible_moves.push((piece_position, piece_position + 8));
                }
                if (piece_position as i32 - 8) >= 0 {
                    if Board::is_different_color(
                        Board::piece_type_on_position(boards, piece_position as usize - 8),
                        piece_type as isize,
                    ) || Board::piece_type_on_position(boards, piece_position as usize - 8) == -1
                    {
                        possible_moves += 1 << (piece_position - 8);
                    }
                    //possible_moves.push((piece_position, piece_position - 8));
                }

                if !boards.w_k_moved && piece_type == Board::W_KINGS && piece_position == 4 {
                    {
                        if Board::piece_type_on_position(boards, piece_position as usize + 1) == -1
                            && Board::piece_type_on_position(boards, piece_position as usize + 2)
                                == -1
                            && Board::piece_type_on_position(boards, piece_position as usize + 3)
                                == Board::W_ROOKS as isize
                            && !boards.w_r_rook_moved
                        {
                            possible_moves += 1 << piece_position + 2;
                        }
                        if Board::piece_type_on_position(boards, piece_position as usize - 1) == -1
                            && Board::piece_type_on_position(boards, piece_position as usize - 2)
                                == -1
                            && Board::piece_type_on_position(boards, piece_position as usize - 3)
                                == -1
                            && Board::piece_type_on_position(boards, piece_position as usize - 4)
                                == Board::W_ROOKS as isize
                            && !boards.w_l_rook_moved
                        {
                            possible_moves += 1 << piece_position - 2;
                        }
                    }
                }
                if !boards.b_k_moved && piece_type == Board::B_KINGS && piece_position == 4 + 7 * 8
                {
                    {
                        if Board::piece_type_on_position(boards, piece_position as usize + 1) == -1
                            && Board::piece_type_on_position(boards, piece_position as usize + 2)
                                == -1
                            && Board::piece_type_on_position(boards, piece_position as usize + 3)
                                == Board::B_ROOKS as isize
                            && !boards.b_r_rook_moved
                        {
                            possible_moves += 1 << piece_position + 2;
                        }
                        if Board::piece_type_on_position(boards, piece_position as usize - 1) == -1
                            && Board::piece_type_on_position(boards, piece_position as usize - 2)
                                == -1
                            && Board::piece_type_on_position(boards, piece_position as usize - 3)
                                == -1
                            && Board::piece_type_on_position(boards, piece_position as usize - 4)
                                == Board::B_ROOKS as isize
                            && !boards.b_l_rook_moved
                        {
                            possible_moves += 1 << piece_position - 2;
                        }
                    }
                }
            }

            _ => print!(""),
        }
        return possible_moves;
    }

    pub fn get_possible_attacks(boards: &Board, piece_position: u64, piece_type: usize) -> u64 {
        //Lista med möjliga drag
        let mut possible_attacks: u64 = 0;

        match piece_type {
            Board::W_PAWNS | Board::B_PAWNS => {
                if piece_type == Board::W_PAWNS {
                    if piece_position + 8 <= 63 {
                        if (piece_position as i64 + 7) % 8 == (piece_position as i64 % 8) - 1 {
                            possible_attacks += 1 << (piece_position + 7);
                        }
                        if (piece_position as i64 + 9) % 8 == (piece_position as i64 % 8) + 1 {
                            possible_attacks += 1 << (piece_position + 9);
                        }
                    }
                }
                if piece_type == Board::B_PAWNS {
                    if piece_position as i32 - 8 >= 0 {
                        if (piece_position as i64 - 7) % 8 == (piece_position as i64 % 8) + 1 {
                            possible_attacks += 1 << (piece_position - 7);
                        }
                        if (piece_position as i64 - 9) % 8 == (piece_position as i64 % 8) - 1 {
                            possible_attacks += 1 << (piece_position - 9);
                        }
                    }
                }
            }
            Board::W_ROOKS | Board::B_ROOKS => {
                //från rook ut åt höger. Kan bli error vid 8..8?
                let mut hit_piece: bool = false;
                for i in 1..=(7 - (piece_position % 8)) {
                    let looked_at_position: u64 = piece_position + i;
                    possible_attacks += (1 << looked_at_position);
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0
                        && piece_type_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_type_at_position >= 0
                        && piece_type_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }

                    //possible_moves.push((piece_position, looked_at_position));
                }

                //Från rook neråt
                //1 kan vara större än piece_position/8
                for i in 1..=(piece_position / 8) {
                    let looked_at_position: u64 = piece_position - (8 * i);
                    possible_attacks += (1 << looked_at_position);
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0
                        && piece_type_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_type_at_position >= 0
                        && piece_type_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }

                //Från rook till vänster
                for i in 1..=(piece_position % 8) {
                    let looked_at_position: u64 = piece_position - i;
                    possible_attacks += (1 << looked_at_position);
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0
                        && piece_type_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_type_at_position >= 0
                        && piece_type_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }

                //Från rook uppåt
                //1 kan vara större än piece_position/8
                for i in 1..=(7 - (piece_position / 8)) {
                    let looked_at_position: u64 = piece_position + (8 * i);
                    possible_attacks += (1 << looked_at_position);
                    let piece_type_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_type_at_position >= 0
                        && piece_type_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_type_at_position >= 0
                        && piece_type_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }
            }
            Board::W_KNIGHTS | Board::B_KNIGHTS => {
                let mut looked_at_position: i32 = 0;
                for i in 0..8 {
                    match i {
                        0 => {
                            //plats upp till vänster
                            looked_at_position = piece_position as i32 + (2 * 8) - 1;
                            if looked_at_position % 8 != (piece_position as i32 % 8) - 1 {
                                continue;
                            }
                        }
                        1 => {
                            //plats upp till höger
                            looked_at_position = piece_position as i32 + (2 * 8) + 1;
                            if looked_at_position % 8 != (piece_position as i32 % 8) + 1 {
                                continue;
                            }
                        }
                        2 => {
                            //plats höger upp
                            looked_at_position = piece_position as i32 + (1 * 8) + 2;
                            if looked_at_position % 8 != (piece_position as i32 % 8) + 2 {
                                continue;
                            }
                        }
                        3 => {
                            //plats höger ner
                            looked_at_position = piece_position as i32 + (-1 * 8) + 2;
                            if looked_at_position % 8 != (piece_position as i32 % 8) + 2 {
                                continue;
                            }
                        }
                        4 => {
                            //plats ner höger
                            looked_at_position = piece_position as i32 + (-2 * 8) + 1;
                            if looked_at_position % 8 != (piece_position as i32 % 8) + 1 {
                                continue;
                            }
                        }
                        5 => {
                            //ner vänster
                            looked_at_position = piece_position as i32 + (-2 * 8) - 1;
                            if looked_at_position % 8 != (piece_position as i32 % 8) - 1 {
                                continue;
                            }
                        }
                        6 => {
                            //vänster ner
                            looked_at_position = piece_position as i32 + (-1 * 8) - 2;
                            if looked_at_position % 8 != (piece_position as i32 % 8) - 2 {
                                continue;
                            }
                        }
                        7 => {
                            //vänster upp
                            looked_at_position = piece_position as i32 + (1 * 8) - 2;
                            if looked_at_position % 8 != (piece_position as i32 % 8) - 2 {
                                continue;
                            }
                        }
                        _ => {}
                    }
                    if looked_at_position <= 63 && looked_at_position >= 0 {
                        let piece_type_at_position: isize =
                            Board::piece_type_on_position(boards, looked_at_position as usize);
                        if piece_type_at_position >= 0 {
                            if piece_type == Board::W_KNIGHTS
                                && piece_type_at_position >= Board::B_PAWNS as isize
                            {
                                possible_attacks += 1 << looked_at_position as u32;
                                //possible_moves.push((piece_position, looked_at_position as u32));
                            } else if piece_type == Board::B_KNIGHTS
                                && piece_type_at_position < Board::B_PAWNS as isize
                            {
                                possible_attacks += 1 << looked_at_position as u32;
                                //possible_moves.push((piece_position, looked_at_position as u32));
                            }
                        } else {
                            possible_attacks += 1 << looked_at_position as u32;
                            //possible_moves.push((piece_position, looked_at_position as u32));
                        }
                    }
                }
            }
            Board::B_BISHOPS | Board::W_BISHOPS => {
                //Upp höger
                //brädets bredd - pjäs.x - 1
                //Brädets höjd - pjäs.y - 1
                for i in 1..=u64::min(7 - (piece_position % 8), 7 - (piece_position / 8)) {
                    let looked_at_position: u64 = piece_position + (8 * i) + i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }
                //Ner höger
                //Brädets bredd - pjäs.x - 1
                //Pjäs.y
                for i in 1..=u64::min(7 - (piece_position % 8), piece_position / 8) {
                    let looked_at_position: u64 = (piece_position - (8 * i)) + i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }
                //Ner vänster
                //Brädets bredd - pjäs.x
                //Pjäs.y
                for i in 1..=u64::min(piece_position % 8, piece_position / 8) {
                    let looked_at_position: u64 = (piece_position - (8 * i)) - i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }
                //Upp vänster
                //Brädets bredd - Pjäs.x
                //Pjäs.y
                for i in 1..=u64::min(piece_position % 8, 7 - (piece_position / 8)) {
                    let looked_at_position: u64 = (piece_position + (8 * i)) - i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }
            }
            Board::B_QUEENS | Board::W_QUEENS => {
                //Upp höger
                //brädets bredd - pjäs.x - 1
                //Brädets höjd - pjäs.y - 1
                for i in 1..=u64::min(7 - (piece_position % 8), 7 - (piece_position / 8)) {
                    let looked_at_position: u64 = piece_position + (8 * i) + i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }
                //Ner höger
                //Brädets bredd - pjäs.x - 1
                //Pjäs.y
                for i in 1..=u64::min(7 - (piece_position % 8), piece_position / 8) {
                    let looked_at_position: u64 = (piece_position - (8 * i)) + i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }
                //Ner vänster
                //Brädets bredd - pjäs.x
                //Pjäs.y
                for i in 1..=u64::min(piece_position % 8, piece_position / 8) {
                    let looked_at_position: u64 = (piece_position - (8 * i)) - i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }
                //Upp vänster
                //Brädets bredd - Pjäs.x
                //Pjäs.y
                for i in 1..=u64::min(piece_position % 8, 7 - (piece_position / 8)) {
                    let looked_at_position: u64 = (piece_position + (8 * i)) - i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }

                //från rook ut åt höger. Kan bli error vid 8..8?
                let mut hit_piece: bool = false;
                for i in 1..=(7 - (piece_position % 8)) {
                    let looked_at_position: u64 = piece_position + i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                    //possible_moves.push((piece_position, looked_at_position));
                }

                //Från rook neråt
                //1 kan vara större än piece_position/8
                for i in 1..=(piece_position / 8) {
                    let looked_at_position: u64 = piece_position - (8 * i);
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }

                //Från rook till vänster
                for i in 1..=(piece_position % 8) {
                    let looked_at_position: u64 = piece_position - i;
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }

                //Från rook uppåt
                //1 kan vara större än piece_position/8
                for i in 1..=(7 - (piece_position / 8)) {
                    let looked_at_position: u64 = piece_position + (8 * i);
                    possible_attacks += 1 << looked_at_position;
                    let piece_at_position: isize =
                        Board::piece_type_on_position(boards, looked_at_position as usize);
                    if piece_at_position >= 0
                        && piece_at_position != Board::W_KINGS as isize
                        && Board::board_is_black(piece_type)
                    {
                        break;
                    }
                    if piece_at_position >= 0
                        && piece_at_position != Board::B_KINGS as isize
                        && Board::board_is_white(piece_type)
                    {
                        break;
                    }
                }
            }
            Board::W_KINGS | Board::B_KINGS => {
                if (piece_position + 1) % 8 == (piece_position % 8) + 1 {
                    possible_attacks += 1 << (piece_position + 1);
                    if (piece_position + 8) <= 63 {
                        possible_attacks += 1 << (piece_position + 9);
                    }
                    if (piece_position as i32 - 8) >= 0 {
                        possible_attacks += 1 << (piece_position - 7);
                    }
                }
                if (piece_position as i32 - 1) % 8 == (piece_position as i32 % 8) - 1 {
                    possible_attacks += 1 << (piece_position - 1);
                    if (piece_position + 8) <= 63 {
                        possible_attacks += 1 << (piece_position + 7);
                    }
                    if (piece_position as i32 - 8) >= 0 {
                        possible_attacks += 1 << (piece_position - 9);
                    }
                }
                if (piece_position + 8) <= 63 {
                    possible_attacks += 1 << (piece_position + 8);
                }
                if (piece_position as i32 - 8) >= 0 {
                    possible_attacks += 1 << (piece_position - 8);
                }
            }

            _ => print!(""),
        }
        return possible_attacks;
    }

    // Ta bort alla drag där kungen dödar sig själv.
    // Maska bort alla drag kungen har till position som en pjäs kan attackera
    // Pinnade pjäser kan attackera kungar
    pub fn limit_kings(
        board: &Board,
        mut all_moves: [u64; 64],
        all_attacks: [u64; 64],
    ) -> [u64; 64] {
        // För varje element i all_moves[i] som tillhör en kung
        // Loopa igenom alla drag som tillhör motståndar pjäser
        // 1or i alla drag för kungen där kungen har en 1a men inte motståndar pjäsen har en 1a

        // i är en kungs position
        for i in 0..64 {
            let king_type: isize = Board::piece_type_on_position(&board, i);
            if king_type == Board::W_KINGS as isize || king_type == Board::B_KINGS as isize {
                // j är attackerande pjäsens position
                for j in 0..64 {
                    let attacker_type = Board::piece_type_on_position(&board, j);
                    //Om det är en fiende ta bort platserna den kan attackera från vart kungen kan gå
                    if Board::is_different_color(king_type, attacker_type) {
                        all_moves[i] = all_moves[i] & !all_attacks[j];
                    }
                }
                if i == 4 || i == 4 + 8 * 7 {
                    if (all_moves[i] >> i - 1) & 1 == 0 {
                        all_moves[i] = all_moves[i] & !(1 << i - 2)
                    }
                    if (all_moves[i] >> i + 1) & 1 == 0 {
                        all_moves[i] = all_moves[i] & !(1 << i + 2)
                    }
                }
            }
        }

        return all_moves;
    }

    //let num: u32 = 2^n;
    //let index = num.trailing_zeros();

    // Titta om kungen är i chack
    // Maska kungens position mot motståndares möjliga drag och titta om bitboarden inte är 0
    // Ta reda på hur många försvarande pjäser som står mellan den attackerande pjäsen och kungen.
    // Om 1, pin_pieces.
    // Om 0, protect_the_king.
    pub fn check(board: &Board, all_attacks: [u64; 64], mut all_moves: [u64; 64]) -> [u64; 64] {
        // i = attacker position
        for i in 0..64 {
            let attacker_type = Board::piece_type_on_position(&board, i);
            let mut king_position: u64 = 0;
            for j in 0..64 as u64 {
                if Board::piece_type_on_position(board, j as usize) == Board::W_KINGS as isize
                    || Board::piece_type_on_position(board, j as usize) == Board::B_KINGS as isize
                {
                    king_position = j;
                }
            }
            //Om attackerarens typ är annorlunda från en vit kung
            if Board::is_different_color(attacker_type, Board::W_KINGS as isize) {
                if all_attacks[i] & board.boards[Board::W_KINGS] != 0
                    && (attacker_type == Board::B_PAWNS as isize
                        || attacker_type == Board::B_KNIGHTS as isize)
                {
                    //protect_the_king

                    all_moves =
                        Board::protect_the_king(board, all_moves, 1 << i, king_position as usize);
                } else {
                    let spaces_between_king_and_attacker: u64 =
                        Board::get_spaces_between_king_and_attacker(
                            board,
                            i as u64,
                            board.boards[Board::W_KINGS],
                        );
                    if Board::get_amount_of_pieces_on_spaces(
                        board,
                        spaces_between_king_and_attacker,
                    ) == 2
                    {
                        // pin
                        all_moves = Board::pin_piece(
                            board,
                            all_moves,
                            spaces_between_king_and_attacker,
                            king_position as usize,
                        );
                    } else if Board::get_amount_of_pieces_on_spaces(
                        board,
                        spaces_between_king_and_attacker,
                    ) == 1
                    {
                        // protect_the_king
                        //Board::print_bitboard(spaces_between_king_and_attacker);
                        all_moves = Board::protect_the_king(
                            board,
                            all_moves,
                            spaces_between_king_and_attacker,
                            king_position as usize,
                        );
                    }
                }
            }
            if Board::is_different_color(attacker_type, Board::B_KINGS as isize) {
                if all_attacks[i] & board.boards[Board::B_KINGS] != 0
                    && (attacker_type == Board::W_PAWNS as isize
                        || attacker_type == Board::W_KNIGHTS as isize)
                {
                    //protect_the_king

                    all_moves =
                        Board::protect_the_king(board, all_moves, 1 << i, king_position as usize);
                } else {
                    let spaces_between_king_and_attacker: u64 =
                        Board::get_spaces_between_king_and_attacker(
                            board,
                            i as u64,
                            board.boards[Board::B_KINGS],
                        );
                    if Board::get_amount_of_pieces_on_spaces(
                        board,
                        spaces_between_king_and_attacker,
                    ) == 2
                    {
                        // pin
                        all_moves = Board::pin_piece(
                            board,
                            all_moves,
                            spaces_between_king_and_attacker,
                            king_position as usize,
                        );
                    } else if Board::get_amount_of_pieces_on_spaces(
                        board,
                        spaces_between_king_and_attacker,
                    ) == 1
                    {
                        // protect_the_king
                        all_moves = Board::protect_the_king(
                            board,
                            all_moves,
                            spaces_between_king_and_attacker,
                            king_position as usize,
                        );
                    }
                    /*println!(
                        "Amount of pieces: {}",
                        Board::get_amount_of_pieces_on_spaces(
                            board,
                            spaces_between_king_and_attacker
                        )
                    );*/
                }
            }
        }
        return all_moves;
    }

    pub fn get_amount_of_pieces_on_spaces(board: &Board, spaces: u64) -> i8 {
        if spaces == 0 {
            return -1;
        }
        let mut amount_of_pieces: i8 = 0;
        for i in 0..64 {
            if (spaces >> i) & 1 == 1 && Board::piece_type_on_position(&board, i) >= 0 {
                amount_of_pieces += 1;
            }
        }
        return amount_of_pieces;
    }

    //Attacker position är attackerarens index
    pub fn get_spaces_between_king_and_attacker(
        boards: &Board,
        attacker_position: u64,
        king_board: u64,
    ) -> u64 {
        let mut pinned_spaces: u64 = 0;
        let attacker_type: isize =
            Board::piece_type_on_position(boards, attacker_position as usize);
        //Hämta kungens index från king_board
        for i in 0..64 {
            if (king_board >> i) & 1 == 1 {
                let king_position: u64 = i;
                //osv
                //8  9  10 11 12 13 14 15
                //0  1  2  3  4  5  6  7
                //(attackerare % 8) - (kung % 8) > 0 och (attackerare / 8) - (kung / 8) > 0 kungen är ner vänster
                if attacker_type == Board::B_BISHOPS as isize
                    || attacker_type == Board::B_QUEENS as isize
                    || attacker_type == Board::W_QUEENS as isize
                    || attacker_type == Board::W_BISHOPS as isize
                {
                    if (attacker_position as i64 % 8) - (king_position as i64 % 8) > 0
                        && (attacker_position as i64 / 8) - (king_position as i64 / 8) > 0
                    {
                        let mut j = 0;
                        while attacker_position as i64 - j - j * 8 != king_position as i64 {
                            if attacker_position as i64 - j - j * 8 < 0 {
                                //Det kommer alltid vara minst 2 kungar på brädet
                                pinned_spaces = u64::max_value();
                                break;
                            }
                            pinned_spaces += 1 << attacker_position as i64 - j - j * 8;
                            j += 1;
                        }
                    }
                    //(attackerare % 8) - (kung % 8) < 0 och (attackerare / 8) - (kung / 8) > 0 kungen är ner höger
                    if (attacker_position as i64 % 8) - (king_position as i64 % 8) < 0
                        && (attacker_position as i64 / 8) - (king_position as i64 / 8) > 0
                    {
                        let mut j = 0;
                        while attacker_position as i64 + j - j * 8 != king_position as i64 {
                            if attacker_position as i64 + j - j * 8 > 63
                                || attacker_position as i64 + j - j * 8 < 0
                            {
                                pinned_spaces = u64::max_value();
                                break;
                            }
                            pinned_spaces += 1 << attacker_position as i64 + j - j * 8;
                            j += 1;
                        }
                    }
                    //(attackerare % 8) - (kung % 8) < 0 och (attackerare / 8) - (kung / 8) < 0 kungen är upp höger
                    if (attacker_position as i64 % 8) - (king_position as i64 % 8) < 0
                        && (attacker_position as i64 / 8) - (king_position as i64 / 8) < 0
                    {
                        let mut j = 0;
                        while attacker_position + j + j * 8 != king_position {
                            if attacker_position + j + j * 8 > 63 {
                                pinned_spaces = u64::max_value();
                                break;
                            }
                            pinned_spaces += 1 << (attacker_position + j + j * 8);
                            j += 1;
                        }
                    }
                    //(attackerare % 8) - (kung % 8) > 0 och (attackerare / 8) - (kung / 8) < 0 kungen är upp vänster
                    if (attacker_position as i64 % 8) - (king_position as i64 % 8) > 0
                        && (attacker_position as i64 / 8) - (king_position as i64 / 8) < 0
                    {
                        let mut j = 0;
                        while attacker_position as i64 - j + j * 8 != king_position as i64 {
                            if attacker_position as i64 - j + j * 8 > 63
                                || attacker_position as i64 - j + j * 8 < 0
                            {
                                pinned_spaces = u64::max_value();
                                break;
                            }
                            pinned_spaces += 1 << attacker_position as i64 - j + j * 8;
                            j += 1;
                        }
                    }
                }
                if attacker_type == Board::B_ROOKS as isize
                    || attacker_type == Board::B_QUEENS as isize
                    || attacker_type == Board::W_QUEENS as isize
                    || attacker_type == Board::W_ROOKS as isize
                {
                    //(attackerare % 8) == (kung % 8) && kung > attackerare kungen är ovanför attackeraren
                    if (attacker_position % 8) == (king_position % 8)
                        && attacker_position < king_position
                    {
                        let mut j = 0;
                        while attacker_position + j * 8 != king_position {
                            if attacker_position + j * 8 > 63 {
                                pinned_spaces = u64::max_value();
                                break;
                            }
                            pinned_spaces += 1 << attacker_position + j * 8;
                            j += 1;
                        }
                    }
                    //(attackerare % 8) == (kung % 8) && kung < attackerare kungen är under attackeraren
                    if (attacker_position % 8) == (king_position % 8)
                        && attacker_position > king_position
                    {
                        let mut j = 0;
                        while attacker_position as i64 - j * 8 != king_position as i64 {
                            if attacker_position as i64 - j * 8 < 0 {
                                pinned_spaces = u64::max_value();
                                break;
                            }
                            pinned_spaces += 1 << attacker_position as i64 - j * 8;
                            j += 1;
                        }
                    }
                    //(attackerare / 8) == (kung / 8) && kung < attackerare kungen är till vänster om attackeraren
                    if (attacker_position / 8) == (king_position / 8)
                        && attacker_position > king_position
                    {
                        let mut j = 0;
                        while attacker_position as i64 - j != king_position as i64 {
                            if (attacker_position as i64 % 8) - j
                                != (attacker_position as i64 - j) % 8
                            {
                                pinned_spaces = u64::max_value();
                                break;
                            }
                            pinned_spaces += 1 << attacker_position as i64 - j;
                            j += 1;
                        }
                    }
                    //(attackerare / 8) == (kung / 8) && kung > attackerare kungen är till höger om attackeraren
                    if (attacker_position / 8) == (king_position / 8)
                        && attacker_position < king_position
                    {
                        let mut j = 0;
                        while attacker_position + j != king_position {
                            if (attacker_position % 8) + j != (attacker_position + j) % 8 {
                                pinned_spaces = u64::max_value();
                                break;
                            }
                            pinned_spaces += 1 << attacker_position + j;
                            j += 1;
                        }
                    }
                }
            }
        }
        return pinned_spaces;
    }

    // Maska den försvarande pjäsens möjliga drag med rutorna mellan den attackerande pjäsen och kungen.
    pub fn pin_piece(
        board: &Board,
        mut all_moves: [u64; 64],
        dangerous_spaces: u64,
        king_position: usize,
    ) -> [u64; 64] {
        for i in 0..64 {
            if i != king_position
                && !Board::is_different_color(
                    Board::piece_type_on_position(board, king_position as usize),
                    Board::piece_type_on_position(board, i as usize),
                )
                && Board::piece_type_on_position(board, i) != -1
            {
                if (1 << i) & dangerous_spaces == 1 << i {
                    //Board::print_bitboard(all_moves[i]);
                    all_moves[i] = all_moves[i] & dangerous_spaces;
                    //Board::print_bitboard(all_moves[i]);
                }
            }
        }

        return all_moves;
    }

    // Ta bort alla drag från försvarande pjäser som inte skyddar kungen.
    // Maska alla försvarande pjäsers drag med dragen attackerande pjäsen gör i riktning mot kungen.
    pub fn protect_the_king(
        board: &Board,
        mut all_moves: [u64; 64],
        dangerous_spaces: u64,
        king_position: usize,
    ) -> [u64; 64] {
        for i in 0..64 {
            if i == king_position {
                if i == 4 || i == 4 + 8 * 7 {
                    all_moves[i] = all_moves[i] & !(1 << i - 2);
                    all_moves[i] = all_moves[i] & !(1 << i + 2);
                }
            }
            if i != king_position
                && !Board::is_different_color(
                    Board::piece_type_on_position(board, king_position as usize),
                    Board::piece_type_on_position(board, i as usize),
                )
            {
                all_moves[i] = all_moves[i] & dangerous_spaces;
            }
        }
        return all_moves;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let initial_boards: [u64; 12] = [0; 12];
        let mut board = Board {
            boards: initial_boards,
            w_enpesant: 0,
            b_enpesant: 0,

            w_l_rook_moved: false,
            w_r_rook_moved: false,
            w_k_moved: false,

            b_l_rook_moved: false,
            b_r_rook_moved: false,
            b_k_moved: false,

            white_turn: true,
        };

        Board::add_piece(&mut board, Board::W_KINGS, 2);
        Board::add_piece(&mut board, Board::B_BISHOPS, 16);
        Board::add_piece(&mut board, Board::B_BISHOPS, 8);
        Board::add_piece(&mut board, Board::B_BISHOPS, 24);
        Board::add_piece(&mut board, Board::B_BISHOPS, 32);

        assert_eq!(true, Board::is_mate_white(&board));
    }
}
