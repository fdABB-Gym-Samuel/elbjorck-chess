use chess_library::Board;

fn main() {
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

    Board::print_board(board);
    println!("");
    println!("");
    println!("");
    Board::set_standard_board(&mut board);
    Board::print_board(board);
}
