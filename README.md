# Number-Guessing Game

## Introduction
The Number-Guessing Game is a simple yet engaging Rust program where the player tries to guess a randomly generated number within a specified range. The game provides feedback on whether the guess is too high, too low, or correct, making it a fun exercise in Rust programming.

## Project Overview
This command-line game challenges the player to guess a number between 1 and 15. The game generates a random number and prompts the player to input their guess. The game continues until the correct number is guessed, offering hints along the way. The project demonstrates fundamental Rust concepts like loops, user input handling, and random number generation.

## Detailed Features
- **Random Number Generation**: Utilizes the `rand` crate to generate a random number between 1 and 15.
- **User Input Handling**: Reads the user's guess from the command line and validates it.
- **Feedback Mechanism**: Provides immediate feedback to the user on whether their guess is too low, too high, or correct.
- **Looping Mechanism**: The game continues in a loop until the player guesses the correct number, ensuring continuous play.
- **Error Handling**: Handles invalid inputs gracefully, prompting the user to enter a valid number.

## Technologies Used
- **Language**: Rust
- **Libraries**: `rand` for random number generation, `std::io` for handling user input.

## Tech Stack
- **Language**: Rust
- **Dependencies**: `rand`, `std::io`