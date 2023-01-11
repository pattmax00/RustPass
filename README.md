# RustPass

An easy to use command-line password generator that creates secure passwords. Randomness is seeded cryptographically.

# Usage:

`./rustpass [number of characters]`<br>
or <br>
`./rustpass [number of characters] "[excluded characters]"`

<hr>

eg: `./rustpass 32` <br>
output: `E$bRUSTiPASS*,ISl{!M&<\[COOL0eVw` <br>
eg (with excluded characters): `./gopass 32 "$,!"` <br>
output: `EYbRUSTiPASS*2ISl{?M&<\[COOL0eVw` <- note the excluded characters are not present in the output

# How to install/use for Windows:

1. Download Windows binary, it may be helpful to simply rename it to "rustpass.exe"  to keep
   commands shorter.
2. Move the file to your `C:\Users\[Username]` directory
3. Opening the console *without* Administrator privileges will put you in the directory mentioned above by default
4. Now type `rustpass [number of characters]` (Ex. `rustpass 64`) to generate a new password!

# How to install/use for Linux:

1. Download Linux binary, it may be helpful to simply rename it to "rustpass" to keep commands shorter.
2. Move the file to your home directory.
3. `chmod +x rustpass` if necessary
4. Opening your terminal should put you in your home directory by default, if not, just type `cd` to get sent back to
   home.
5. Now type `./rustpass [number of characters]` (Ex. `./rustpass 64`) to generate a new password!