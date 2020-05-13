# Rusty Snake CLI in 20 minutes
As I often do, for some reason, I challenged myself to write  
a Snake CLI clone in Rust, with a time constraint of 20 minutes.

It took me three attempts to finally get it done in under 20 minutes,  
with all basic Snake features (movement, body parts, food, game over).

Code for each attempt is in its respective branch.

The third and only successful attempt took me about 12.5 minutes to do,  
although at that point it was primarily memorization and avoiding previous mistakes.

## Crates used
- `ncurses`  
  for reading single characters as input,  
  and for drawing characters to the alternate `stdout` buffer/window/whatever
- `rand`  
  for food spawning randomization

## What I learned
- `ncurses` is easy to use, although it's definitely not Rust idiomatic ('cause it's not a Rust library).  
  For simple cases like this, where I only really needed the `getch()` function, it's nice to have.
- I learned how to practically use `std::mpsc`, in this case for reading user input  
  on a separate thread, so it doesn't block the main game loop.
- I learned that Rust _can_ be very efficient for quick development, _if you know the necessary building blocks!_  
  My third attempt was so quick (in comparison), because I had figured out all the problems before-hand.  
  I did spend probably about an hour researching how to read a single character from `stdin` (using `ncurses` in the end),  
  and how to use it with the `mpsc` thread to avoid blocking.  
  The business-logic part of it all was very quick to write though!
