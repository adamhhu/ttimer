# TTIMER

Welcome to TTimer, a small terminal based timer written in Rust.

## Installation

<details>
1. You need to install Rust to be able to compile the code.
Install it however you like or by using the official install script below
  
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repo
```
git clone https://github.com/adamhhu/ttimer.git
cd ttimer
```

3. Install it locally
```
cargo install --path .
```

4. Start it with the command
```
ttimer
```

### If it doesn't work you probably don't have the path configured.
<details>
Instructions for fish
```
fish_add_path ~/.cargo/bin
```

Instrucions for zshrc
```
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

Instructions för bashrc
```
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```
</details>
</details>


### What it does

Once you have started the application you can start a timer of (basically) any length
by typing for example "50s" to start a timer for fifty seconds, "2m" for two minutes
or "1h" to start a timer for one hour.
It will automatically keep track of the time and count it down, so if you
for example write "120s" it will start counting down from "2m" and not
"120s".

## Visuals

TTimer comes with a STUNNING (/s) user interface loaded with ASCII art and colors!
It even has a loading bar!!

### Disclaimer

AI was used to create this, although every single line was hand written by me.
