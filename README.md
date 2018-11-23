Chess Position Trainer
======================

Defines (load) the chess position on which you want to train yourself,
then play it against a very strong computer level.

Development
-----------

Using Rust language and Gtk-Rs GUI.

Windows user
------------

You'd better use this command rather to build the application

    cargo rustc --release -- -Clink-args="-Wl,--subsystem,windows"

That way the program won't be launched in console mode.