# Left-Right Reaction Game in Rust

This is a learning project.

When it comes to game ideas, the simplest one for me is a reaction game. The player just have to perform a certain action in a certain time. The simplest one will be like "Press the button after a signal". Here is an example of a marginally more difficult game. The player has to press one of two buttons.

## Architecture

As my frontend experience is mostly limited to Elm, I decided to use a similar approach here. I have a model: a specification for the state of the application. I have an update function, that changes the state as time goes. I have another update function that changes the state when user presses some button, and I have a render function, that draws the current state of the application on rhe screen.

### Model

The application can have several different states that change in response to user's actions, or simply when time passes. They are represented as an `enum`. First state is an initial state and it doesn't carry any additional information. The main reason for adding it was using it in `mem::replace`. Next state is a countdown. It stores the remaining time before the actual game begins. After the countdown the side is chosen, and the program waits for user input while counting the time it takes for user to react. After the user input the program goes to the finished game state that contains the passed time, the original requested side, and the side chosen by a player. If a player chooses the side during the countdown period, the program goes to a special state - false start.

### Update I

There are two states, where the time matters: the countdown, and the main game state. In the first one, the counter goes down as the time passes. In the main game state, the counter of the passed time goes up.

### Update II

There are only three buttons the program understands: space, left arrow, and right arrow. Using the space button, you can go from any non-counting state (Initial screen, Finished game, and False start) to the countdown. The left and right arrows work only for the countdown and the main game loop. However, for countdown they always result in false start state.

### View

To implement view, I decided to use an internal view representation that contains only the text to write at the top of the screen and colors for the left and right rectangle. Thus, I render the state into this internal format, and render the latter phisically.

## Doubtful implementation details

### Piston Window

There is not enough documentation on conventions how to use `piston_window`. I don't understand, whther it is OK to call `render_2d` in one branch of the `match` on `Input`. In the examples I saw, it is called on each and every event, even when it is not a `Render` one.

### mem::replace

This idiom isn't very clear for me. If I transfer some information frome one state to another I am forced by a compiler to replace the current state with some dummy. It looks OK if there is a valid state with no additional info in it. What should I do if every state carries some complex structure? Should I add an empty dummy state to the enum? What is the cost of the creation of the replacement structure that will be surely thrown away when the function returns?
