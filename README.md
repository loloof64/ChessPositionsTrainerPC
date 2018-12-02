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

You can use CygWin terminal and download Gtk devel package from it. By the way, you should keep the setup.exe, as it is also
an interface for installing packages, once CygWin has been installed.

You'd better use this command rather to build the application

    cargo rustc --release -- -Clink-args="-Wl,--subsystem,windows"

That way the program won't be launched in console mode.

Acknowledgements
------------------

* Mozilla for its wonderful language that is Rust and their efforts to make it efficient, understandable and usable.
* The members of [Gitter channel for Rust](https://gitter.im/rust-lang/rust) : great place for getting help.