# elevator_rs

## Step 1: Getting started
Install amethyst
and run `amethyst new elevator_rs`

- need to include features for macos
- need to change objc to be specific version
```
objc = "=0.2.6"
```
- need to rename "Resources" to "Resource"
- use nightly if you want extra debugging

## Step 2: Create the game state
- Create an elevator_game.rs game state file
- Add the struct and SimpleState impl for it
- pull it into main.rs