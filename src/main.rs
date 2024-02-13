use std::rc::Rc;

use slint::{Color, Model, ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_check_if_game_over({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut board: Vec<GamePieceData> = ui.get_board().iter().collect();
            if let Some((a, b, c)) = get_indices_of_winning_combination(&board) {
                // Update properties of the winning combination.
                let green_color = Color::from_rgb_u8(0, 128, 0);
                board[a].marker_color = green_color;
                board[b].marker_color = green_color;
                board[c].marker_color = green_color;

                println!("the winner is: {}", get_winner_marker(&board, (a, b, c)));

                let board_model = ModelRc::from(Rc::new(VecModel::from(board)));
                ui.set_board(board_model);
                ui.set_is_game_over(true);
            }
        }
    });
    ui.run()
}

fn get_indices_of_winning_combination(grid: &[GamePieceData]) -> Option<(usize, usize, usize)> {
    const WINNING_COMBINATIONS: &[(usize, usize, usize)] = &[
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8), // Rows
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8), // Columns
        (0, 4, 8),
        (2, 4, 6), // Diagonals
    ];

    // Check each winning combination.
    for &(a, b, c) in WINNING_COMBINATIONS {
        if !grid[a].marker.is_empty()
            && grid[a].marker == grid[b].marker
            && grid[b].marker == grid[c].marker
        {
            return Some((a, b, c)); // winner winner chicken dinner.
        }
    }
    None // No winner found.
}

fn get_winner_marker(board: &[GamePieceData], (a, _, _): (usize, usize, usize)) -> String {
    board[a].marker.clone().to_string()
}
