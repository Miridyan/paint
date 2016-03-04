# Paint

This project is pretty useless and only serves as a means to teach myself how to use gtk+ and the
rust programming language. As of right now it can't do very much. When it finished that will
probably still be the case considering it's inspired MSPaint (even though it looks nothing like it).

## Building

Okay, so you want to build this project (I'm not sure why but I won't question your judgment).
I recommend you build this on Linux since it's the easiest platform to install gtk+ on, but it
does build on windows without a problem and I can't see any reason why it wouldn't build on OS X
as well.

### Linux Dependencies

_Installation procedure will differ depending on your choice of Linux distribution and package
manager. All install instructions are for Arch Linux and pacman._

Install gtk3 along with rustc and cargo if it is not a part of the rust package.
```
sudo pacman -Syu
sudo pacman -S gtk3 rust
```
If you're the adventurous type of person or you just enjoy being on the bleeding edge, install the
nightly version of rust from the AUR using whatever method you prefer (I use yaourt).
```
yaourt -S rust-nightly
```

### Windows Dependencies

This is the _really fun_ one to do. Unfortunately for you, Windows users, you have to just through a
lot more hoops than either the Linux users or the Mac OS X users (and there aren't even instructions
for them here). You're going to need to install the [MSYS2 toolchain](https://msys2.github.io/) in
order get gtk+ and its dependencies. Once you have installed that, open the MSYS2 shell and enter
the following commands (you may need to close and reopen the shell a couple of times throughout this
process).
```
update-core
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain mingw-w64-x86_64-gtk3
```
Once you've done this, you need to re-install rust so we can use the MSYS2 toolchain. Go over to the
[rust website](https://www.rust-lang.org/downloads.html) and download the installer (it doesn't
matter if the stable, beta, or nightly version) that utilizes the GNU ABI. Do a custom install and
uncheck the box for the "install linkers" option, but keep everything else. You may need to restart
your pc. Once this is done, open the MSYS2 shell and ensure that rustc and cargo can be used. Now
just cd into your user directory by doing `cd /c/Users/$USER` and we can begin the building process.

### The Actual Building Part

You're almost there! And luckily this is the easiest step (in theory, at least).

```
git clone https://github.com/Miridyan/paint.git
cd paint
cargo run --features 3.14
```

You can replace `3.14` for `3.16` if you want, but you cannot compile below gtk+ version 3.14.

## Congration you done it!

Yay! You installed my terrible version of paint. I wish you luck on your useless adventures drawing
some quality lines that slightly resemble a face if you squint hard enough and turn your head a bit.

Happy painting :D
