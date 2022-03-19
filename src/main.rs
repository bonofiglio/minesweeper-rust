use rand::Rng;
use yew::{function_component, html, use_state, MouseEvent, Properties, UseStateHandle};

const SIZE: i32 = 9;
const COLORS: [&str; 8] = [
    "#0000ff", "#00ff00", "#ff0000", "#2500ac", "#961e00", "#009480", "#000000", "#ffc21b",
];

#[derive(Debug)]
struct Square {
    mine: bool,
    flagged: bool,
    revealed: bool,
    mines_around: i32,
}

impl Clone for Square {
    fn clone(&self) -> Self {
        Self {
            mine: self.mine,
            flagged: self.flagged,
            revealed: self.revealed,
            mines_around: self.mines_around,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum GameState {
    Playing,
    Won,
    Lost,
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                GameState::Playing => "",
                GameState::Won => "win",
                GameState::Lost => "lose",
            }
        )
    }
}

#[derive(Properties, PartialEq)]
struct MinesweeperProps {
    game_state: UseStateHandle<GameState>,
}

#[function_component(Minesweeper)]
fn minesweeper(props: &MinesweeperProps) -> Html {
    let squares = use_state(|| create_squares());
    let game_state = &props.game_state;
    let flagged_squares = squares.iter().filter(|s| s.flagged).count();
    let mine_count = squares.iter().filter(|s| s.mine).count();

    let squares_view = |(i, square): (usize, &Square)| {
        let flagged = square.flagged;
        let mine = square.mine;
        let revealed = square.revealed;
        let onclick_squares = squares.clone();
        let oncontextmenu_squares = squares.clone();
        let color = if square.mines_around > 0 {
            COLORS[(square.mines_around - 1) as usize]
        } else {
            ""
        };
        let onclick_game_state = game_state.clone();
        let oncontextmenu_game_state = game_state.clone();

        html! {
            <button
                disabled={square.flagged}
                onclick={move |_| {
                    if !flagged {
                        if mine {
                            onclick_game_state.set(GameState::Lost);
                            onclick_squares.set(create_squares());
                            return();
                        }

                        let x = (i as i32) % (SIZE as i32);
                        let y = ((i as f32) / (SIZE as f32)).floor() as i32;

                        let mut new_squares = (*onclick_squares).clone();

                        recursively_reveal(&mut new_squares, x, y);

                        onclick_squares.set(new_squares)
                    }
                }}
                oncontextmenu={move |e: MouseEvent| {
                    e.prevent_default();
                    if revealed {return()};

                    let mut new_squares = (*oncontextmenu_squares).clone();

                    new_squares[i].flagged = !new_squares[i].flagged;

                    let correct_flags = new_squares.iter().filter(|square| {square.flagged && square.mine}).count();
                    if correct_flags == mine_count {
                        oncontextmenu_game_state.set(GameState::Won);
                        oncontextmenu_squares.set(create_squares());
                        return();
                    }

                    oncontextmenu_squares.set(new_squares)
                }}
                style={format!("
                    background: {};
                    border: 2px solid #eaeaea;
                    height: {}vh;
                    cursor: {};
                    color: {};
                    font-size: {};
                    font-weight: bold;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    aspect-ratio: 1/1
                ", 
                    // background
                    if square.revealed { "#777" } else { "#212121" },
                    // height
                    100 / SIZE - 1,
                    // cursor
                    if square.revealed { "default" } else { "pointer" },
                    // color
                    if square.revealed { color } else { "#fafafa" },
                    // font size
                    if square.flagged { "1.5rem" } else { "2rem" })}
            >
                {
                    if square.flagged { "🚩".to_string() }
                    else {
                        if square.revealed {
                            if square.mine { "💣".to_string() }
                            else {
                                if square.mines_around > 0 {
                                    format!("{}", square.mines_around)
                                }
                                else { "".to_string() }
                            }
                        }
                        else { "".to_string() }
                    }
                }
            </button>
        }
    };

    html! {
        <div>
            <div style="color: #fafafa; fontSize: 3rem; font-weight: 600; margin: 1rem;">
                {format!("Flagged: {} / {}", flagged_squares,
                    squares.iter().filter(|s| {s.mine}).count())
                }
            </div>

            <div style={format!("display: grid; grid-template-columns: repeat({}, 1fr); grid-template-rows: repeat({}, 1fr); margin: 0 auto; border: 6px solid #5e21ad; overflow: auto; max-width: 100vw;", SIZE, SIZE)}>
                {for squares.iter().enumerate().map(squares_view)}
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let game_state = use_state(|| GameState::Playing);

    html! {
        <div id="app">

            <Minesweeper game_state={game_state.clone()} />
            <dialog
                open={(*game_state) != GameState::Playing}
            >
                <div>
                    <h1>{format!("You {} !", *game_state)}</h1>
                    <button id="restart" onclick={move |_| {
                        game_state.set(GameState::Playing)
                    }}>{ "Restart" }</button>
                </div>
            </dialog>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}

fn create_squares() -> Vec<Square> {
    let mut rng = rand::thread_rng();

    let mut new_square = || Square {
        flagged: false,
        mine: rng.gen_range::<f32, std::ops::Range<f32>>(0.0..10.0) < 0.15,
        mines_around: 0,
        revealed: false,
    };

    let mut squares = vec![new_square(); (SIZE * SIZE).try_into().unwrap()];

    for x in 0..squares.len() {
        squares[x].mine = rng.gen_range::<f32, std::ops::Range<f32>>(0.0..1.0) < 0.15;
    }

    for i in 0..(squares.len() - 1) {
        if squares[i].mine {
            continue;
        }

        let x = (i as i32) % (SIZE as i32);
        let y = ((i as f32) / (SIZE as f32)).floor() as i32;

        let the_hood = [
            [x - 1, y - 1],
            [x, y - 1],
            [x + 1, y - 1],
            [x - 1, y],
            [x + 1, y],
            [x - 1, y + 1],
            [x, y + 1],
            [x + 1, y + 1],
        ];

        for [x, y] in the_hood {
            let parsed_size = SIZE as i32;

            if x < 0 || x >= parsed_size || y < 0 || y >= parsed_size {
                continue;
            }

            let index: i32 = x + y * parsed_size;
            let index: usize = index.try_into().unwrap();
            if squares[index].mine {
                squares[i].mines_around += 1;
            }
        }
    }

    squares
}

fn recursively_reveal(squares: &mut Vec<Square>, x: i32, y: i32) {
    let square = &mut squares[(x + y * SIZE) as usize];
    square.revealed = true;

    if square.mines_around == 0 {
        let the_hood = [
            [x - 1, y - 1],
            [x, y - 1],
            [x + 1, y - 1],
            [x - 1, y],
            [x + 1, y],
            [x - 1, y + 1],
            [x, y + 1],
            [x + 1, y + 1],
        ];

        for [x, y] in the_hood {
            let parsed_size = SIZE as i32;

            if x < 0 || x >= parsed_size || y < 0 || y >= parsed_size {
                continue;
            }

            let new_square = &mut squares[(x + y * SIZE) as usize];
            if !new_square.revealed {
                new_square.revealed = true;
                recursively_reveal(squares, x, y);
            }
        }
    }
}
