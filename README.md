# Left-Right Reaction Game in Rust

This is a learning project. Here is the process of its creation step by step: [tutorial.md](tutorial.md).

When it comes to game ideas, the simplest one for me is a reaction game. The player just has to perform a certain action at a certain time as fast as possible. The easiest one will be like "Press the button after a signal."

Here is an example of a marginally more difficult game. The player has to press one of two buttons.

## Architecture

As my frontend experience is mostly limited to Elm, I decided to use a similar approach here. I have a model: a specification for the state of the application. I have an update function that changes the state as time goes. I have another update function that changes the state when the user presses some button. And I have a render function, that draws the current state of the application on the screen.

### Model

The application can have several different states that change in response to user's actions, or simply when time passes. They are represented as an `enum`. The first state is an initial state and it doesn't carry any additional information. The next state is a countdown. It stores the remaining time before the actual game begins. After the countdown, the side is chosen, and the program waits for user input while counting the time it takes for the user to react. After the user input, the program goes to the finished game state that contains the passed time and whether the player has chosen the right side. If a player chooses the side during the countdown period, the program goes to a special state - false start.

### Update I

There are two states, where the time matters: the countdown and the main game state. In the first one, the counter goes down as the time passes. In the main game state, the counter of the passed time goes up.

### Update II

There are only three buttons the program understands: space, left arrow, and right arrow. Using the space button, you can go from any non-counting state (Initial screen, Finished game, and False start) to the countdown. The left and right arrows work only for the countdown and the main game loop. However, for the countdown, they always result in the false start state.

### View

To implement view, I decided to use an internal view representation that contains only the text to write at the top of the screen and which rectangle should be highlighted if any. Thus, I convert the state into this internal format and render the latter physically.

## References

1. [Documentation](http://docs.piston.rs/piston_window/piston_window/) for `piston_window`
2. [Rendering Text in Rust with Piston-Window](https://medium.com/@arpith/rendering-text-in-rust-with-piston-window-5811b63b1324)
