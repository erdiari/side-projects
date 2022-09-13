use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Cell, Paragraph, Row, Table},
    Frame, Terminal,
};

use array2d::Array2D;

#[derive(PartialEq, Eq)]
enum TurnStates {
    X,
    O,
}

#[derive(Clone, PartialEq, Eq)]
enum BoardStates {
    B,
    X,
    O,
}

#[derive(PartialEq, Eq)]
enum GameState {
    Ongoing,
    Finished,
}

// TODO: Learn how to write these. Use these instead of match cases in some areas.
//
// impl Into<char> for TurnStates {
//     fn into(s: Self) -> c {
//         match s {
//             TurnStates::O => 'O',
//             TurnStates::X => 'X',
//         }
//     }
// }

// impl From<&TurnStates> for char {
//     fn from(source: &TurnStates) -> char {
//         match source {
//             TurnStates::O => 'O',
//             TurnStates::X => 'X',
//         }
//     }
// }

struct Game {
    turn: TurnStates,
    state: GameState,
    board: array2d::Array2D<BoardStates>,
}

impl Default for Game {
    fn default() -> Game {
        let array = Array2D::filled_with(BoardStates::B, 3, 3);
        Game {
            turn: TurnStates::X,
            board: array,
            state: GameState::Ongoing,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let game = Game::default();
    let res = run_game(&mut terminal, game);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn check_if_game_ended(game_board: &Array2D<BoardStates>) -> bool {
    let win_checks = vec![
        // Horizontal conditions
        vec![(0, 0), (0, 1), (0, 2)],
        vec![(1, 0), (1, 1), (1, 2)],
        vec![(2, 0), (2, 1), (2, 2)],
        // Vertical conditions
        vec![(0, 0), (1, 0), (2, 0)],
        vec![(0, 1), (1, 1), (2, 1)],
        vec![(0, 2), (1, 2), (2, 2)],
        // Diagonal condition
        vec![(0, 0), (1, 1), (2, 2)],
        vec![(0, 2), (1, 1), (0, 2)],
    ];
    for win_check in win_checks {
        let check = &game_board[win_check[0]];
        if check == &BoardStates::B {
            continue;
        } else {
            if &game_board[win_check[1]] == check && &game_board[win_check[2]] == check {
                return true;
            }
        }
    }
    return false;
}

fn run_game<B: Backend>(terminal: &mut Terminal<B>, mut game: Game) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &game))?;

        if let Event::Key(key) = event::read()? {
            match game.state {
                GameState::Ongoing => match key.code {
                    KeyCode::Char('s') => {
                        game.state = match game.state {
                            GameState::Finished => (GameState::Ongoing),
                            GameState::Ongoing => (GameState::Finished),
                        }
                    }
                    KeyCode::Char('t') => {
                        game.turn = match game.turn {
                            TurnStates::X => (TurnStates::O),
                            TurnStates::O => (TurnStates::X),
                        }
                    }
                    KeyCode::Char('1')
                    | KeyCode::Char('2')
                    | KeyCode::Char('3')
                    | KeyCode::Char('4')
                    | KeyCode::Char('5')
                    | KeyCode::Char('6')
                    | KeyCode::Char('7')
                    | KeyCode::Char('8')
                    | KeyCode::Char('9') => {
                        // TODO: Is there a any better way to do this. At least vim macros helped.
                        let mut pressed: [usize; 2] = [0; 2];
                        match key.code {
                            KeyCode::Char('1') => {
                                pressed[0] = 0;
                                pressed[1] = 0
                            }
                            KeyCode::Char('2') => {
                                pressed[0] = 0;
                                pressed[1] = 1
                            }
                            KeyCode::Char('3') => {
                                pressed[0] = 0;
                                pressed[1] = 2
                            }
                            KeyCode::Char('4') => {
                                pressed[0] = 1;
                                pressed[1] = 0
                            }
                            KeyCode::Char('5') => {
                                pressed[0] = 1;
                                pressed[1] = 1
                            }
                            KeyCode::Char('6') => {
                                pressed[0] = 1;
                                pressed[1] = 2
                            }
                            KeyCode::Char('7') => {
                                pressed[0] = 2;
                                pressed[1] = 0
                            }
                            KeyCode::Char('8') => {
                                pressed[0] = 2;
                                pressed[1] = 1
                            }
                            KeyCode::Char('9') => {
                                pressed[0] = 2;
                                pressed[1] = 2
                            }
                            _ => {}
                        }
                        // Wow, i can't believe it got worse.
                        match game.board.get(pressed[1], pressed[0]) {
                            Some(BoardStates::B) => {
                                // TODO: hande error.
                                game.board.set(
                                    pressed[1],
                                    pressed[0],
                                    match game.turn {
                                        TurnStates::X => BoardStates::X,
                                        TurnStates::O => BoardStates::O,
                                    },
                                );
                                let is_game_ended = check_if_game_ended(&game.board);
                                if is_game_ended {
                                    game.state = GameState::Finished;
                                } else {
                                    game.turn = match game.turn {
                                        TurnStates::O => TurnStates::X,
                                        TurnStates::X => TurnStates::O,
                                    }
                                }
                            }
                            _ => (),
                        };
                    }
                    _ => {}
                },
                GameState::Finished => return Ok(()),
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, game: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Text at the top.
    let turn_text = match game.turn {
        TurnStates::O => ("O"),
        TurnStates::X => ("X"),
    };
    let (msg, style) = match game.state {
        GameState::Finished => (
            vec![
                Span::raw("Game is over. The player "),
                Span::styled(turn_text, Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" has won. Press any key to exit."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        GameState::Ongoing => (
            vec![
                Span::raw("Game is ongoing. It's "),
                Span::styled(turn_text, Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("'s turn to play."),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);

    f.render_widget(help_message, chunks[0]);

    // Board
    let mut counter = 0;
    let board = Table::new(
        game.board
            .as_columns()
            .iter()
            .map(|r| {
                Row::new({
                    r.iter()
                        .map(|c| {
                            counter += 1;
                            Cell::from(match c {
                                BoardStates::B => (format!("{}", counter)),
                                BoardStates::X => ("X").to_string(),
                                BoardStates::O => ("O").to_string(),
                            })
                        })
                        .collect::<Vec<Cell>>()
                })
            })
            .collect::<Vec<Row>>(),
    )
    .style(Style::default().bg(Color::Gray).fg(Color::Black))
    .widths(&[Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
    .column_spacing(20);

    f.render_widget(board, chunks[1])
}
