## Martian Robots
This is trying to solve the problem of martian rovers disappearing on planet mars.

# Assumption
Assume that the original format is what is passed to the program. E.G (Spaces + Line breaks)
Assume that if a bad command is given we should not drop the rover.

# Running
You should just need Rust and an internet connection in order to run this project.
<br>
As i've assumed that the data is coming in the same format. I've used a make file to setup a quick cat from the sample txt file into the program.
You can run it with
<br>
`make run`

# Testing
You can run the tests that i've provided by using.
<br>
`cargo test`


# Things i would have liked to add
Would have added a github ci with actions to give a basic implementation where it could check against some tests
Some nicer handling with wrong input commands.
Some Enum based errors for rust
Check on other rover positions. E.G is the rover still on the grid after its moved?
Individual processing. Currently these robots are tied together if you give a bad command for one then it will affect all of them.