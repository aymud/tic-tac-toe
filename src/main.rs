use std::rc::Rc;

use slint::{Color, Model, ModelRc, VecModel};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    let ui = AppWindow::new().unwrap();

    ui.on_check_if_game_over({
        let ui_handle = ui.as_weak();
        move || {
            let ui: AppWindow = ui_handle.unwrap();
            let mut board: Vec<GamePieceData> = get_game_board(&ui);
            if let Some((a, b, c)) = get_indices_of_winning_combination(&board) {
                // Update properties of the winning combination.
                let green_color = Color::from_rgb_u8(0, 128, 0);
                board[a].marker_color = green_color;
                board[b].marker_color = green_color;
                board[c].marker_color = green_color;

                let board_model = ModelRc::from(Rc::new(VecModel::from(board)));
                ui.set_board(board_model);
                ui.set_is_game_over(true);
            }
        }
    });

    ui.on_restart_game({
        let ui_handle = ui.as_weak();
        move || {
            let ui: AppWindow = ui_handle.unwrap();

            let mut board: Vec<GamePieceData> = get_game_board(&ui);
            clear_game_board(&mut board);
            let board_model = ModelRc::from(Rc::new(VecModel::from(board)));
            ui.set_board(board_model);

            ui.set_current_player("X".into());
            ui.set_is_game_over(false);
        }
    });

    ui.run().unwrap();
}

fn get_game_board(ui: &AppWindow) -> Vec<GamePieceData> {
    ui.get_board().iter().collect()
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

fn clear_game_board(board: &mut [GamePieceData]) {
    for cell in board.iter_mut() {
        cell.marker = "".into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_indices_of_winning_combination() {
        // Test a winning combination in a row.
        let board = vec![
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
        ];

        assert_eq!(get_indices_of_winning_combination(&board), Some((0, 1, 2)));

        // Test a winning combination in a column.
        let board = vec![
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
        ];

        assert_eq!(get_indices_of_winning_combination(&board), Some((0, 3, 6)));

        // Test a winning combination in a diagonal.
        let board = vec![
            GamePieceData {
                marker: "O".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "O".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "O".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
        ];

        assert_eq!(get_indices_of_winning_combination(&board), Some((0, 4, 8)));

        // Test no winning combination.
        let board = vec![
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "O".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "O".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "X".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "O".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
            GamePieceData {
                marker: "".into(),
                marker_color: Color::from_rgb_u8(0, 0, 0),
            },
        ];

        assert_eq!(get_indices_of_winning_combination(&board), None);
    }
}
