Chess Position Trainer
======================

Defines (load) the chess position on which you want to train yourself,
then play it against a very strong computer level.

Be careful ! When you want to castle, you must move the king on the cell
of the rook in the side of the given castle : and not two cells towards the rook.

Development
-----------

Using Rust language and Gtk-Rs GUI.

Windows user
------------

You'd better use this command rather to build the application

    cargo rustc --release -- -Clink-args="-Wl,--subsystem,windows"

That way the program won't be launched in console mode.