// For color for X and O
extern crate termcolor;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Displays a start message
fn game_start_message() {
    println!(
        "\nWelcome to tic-tac-toe!\n\
        Below is an example of how the board will be displayed, on your turn:
        Take a square and enter the number you wish to take (make sure it is empty.)
        "
    )
}

// Displays X and O
fn display_x_o(user: &char) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if user == &'X' {
        stdout
            // Picks a teal color for X
            .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(3, 252, 223))))
            .unwrap();
    } else if user == &'O' {
        stdout
            // Picks a magenta color for O
            .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
            .unwrap();
    }
    write!(&mut stdout, "{}", user).unwrap();
    stdout.reset().unwrap();
}

// The top and separating pieces of the board
fn board(status: &[char]) {
    println!("\n");
    for i in (0..3).rev() {
        let offset = i * 3;

        print!("-------------\n| ");
        display_x_o(&status[offset]);
        print!(" | ");
        display_x_o(&status[offset + 1]);
        print!(" | ");
        display_x_o(&status[offset + 2]);
        println!(" |");
    }
    println!("-------------");
}

// Allows for the user to take their turn
fn user_action(status: &mut [char], user: char) {
    loop {
        print!("Player '");
        display_x_o(&user);
        println!("', enter the number in the square: ");

        // Checks for reading errors
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Please try again.");
            continue;
        }
        // Checks for numbers in the boards range
        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("The number must be 1-9.");
                continue;
            }

            let number = number - 1;
            
            // Checks if the space is already taken.
            if status[number] == 'X' || status[number] == 'O' {
                print!("That square is already taken by '");
                display_x_o(&status[number]);
                println!("'.");
                continue;
            }

            status[number] = user;

            break;
        } else {
            // Checks for anything other then numbers
            println!("Only numbers are accepted.");
            continue;
        }
    }
}

// Checks for win conditions
fn has_won(status: &[char]) -> bool {
    for i in 0..3 {
        if status[i] == status[i + 3] && status[i] == status[i + 6] {
            return true;
        }

        let i = i * 3;

        if status[i] == status[i + 1] && status[i] == status[i + 2] {
            return true;
        }
    }

    if (status[0] == status[4] && status[0] == status[8])
        || (status[2] == status[4] && status[2] == status[6])
    {
        return true;
    }

    false
}

// Checks for the end of turn
fn turn_is_over(status: &[char]) -> bool {
    status.iter().all(|&v| v == 'X' || v == 'O')
}

// Main game loop logic
fn main() {
    let mut status = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut user = 'X';

    // Start of game message
    game_start_message();

    loop {
        // Draw the board
        board(&status);

        // Get input
        user_action(&mut status, user);

        // Check if a player won
        if has_won(&status) {
            board(&status);
            print!("Player '");
            display_x_o(&user);
            println!("' has won!");
            break;
        }

        // Check if all squares are used
        if turn_is_over(&status) {
            board(&status);
            println!("All squares are used. It is a tie!");
            break;
        }

        // Switch player
        user = if user == 'X' { 'O' } else { 'X' }
    }
}