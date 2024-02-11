use slint::Model;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_check_if_game_over({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let board: Vec<GamePieceData> = ui.get_board().iter().collect();
            let winner: Option<String> = check_winner(&board);
            println!("the winner is: {winner:?}")
        }
    });

    ui.run()
}

fn check_winner(grid: &Vec<GamePieceData>) -> Option<String> {
    const WINNING_COMBINATIONS: &[(usize, usize, usize)] = &[
        (0, 1, 2), (3, 4, 5), (6, 7, 8), // Rows
        (0, 3, 6), (1, 4, 7), (2, 5, 8), // Columns
        (0, 4, 8), (2, 4, 6),            // Diagonals
    ];

    // Check each winning combination.
    for &(a, b, c) in WINNING_COMBINATIONS {
        if let (ch1, ch2, ch3) = (
            grid[a].marker.clone(),
            grid[b].marker.clone(),
            grid[c].marker.clone()) {
            if ch1 == ch2 && ch2 == ch3 {
                return Some(ch1.to_string()); // winner winner chicken dinner.
            }
        }
    }

    None // No winner found.
}