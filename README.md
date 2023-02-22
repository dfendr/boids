# Boids 🕊

**boids** is a flocking simulator written in Rust, using the Nannou framework.

![flock](https://i.imgur.com/pX1fNWi.png)

## Features 🐤

- Modeled after Craig Reynold's Boids algorithm, follows three simple steering
  behaviours
  - `Alignment` - Steer towards the average direction of local flockmates.
  - `Cohesion` - Steer towards direction of average position of nearby flockmates.
  - `Separation`- Steer away from flockmates that are too close to avoid crowding.
- 🐣 Adjustable number of boids, from 100-1000.
- 🦅 Predator boid that "hunts" the prey boids.
- 🖼️ Themes!

## Building / Getting Started ⚙️

### Prerequisites

- **Rust** installed on your system to build the project, which you can [install
  here](https://www.rust-lang.org/learn/get-started).
- **Git** to download the repo from the terminal, which you can find [install
  instructions here](https://github.com/git-guides/install-git)

### MacOS/Linux

- To download and run the program, execute the following commands in your terminal.

```sh
git clone https://github.com/postfen/boids.git
cd boids
cargo build --release
cd target/release
open .
./boids
```

### Windows

- To download and run the program, execute the following commands in Powershell.
- *Note - Please let me know if this works. I don't have a Windows
  machine to test it, currently.*

```sh
git clone https://github.com/postfen/boids.git
cd boids
cargo build --release
cd target/release
start .
.\boids
```

To run again, just find the compiled `boids` executable (default location is
`boids/target/release`), and run the program by double clicking on it.

## Controls ⌨️

| Modifier         |   Keys    |
| ---------------- | :-------: |
| Alignment        |   `u/i`   |
| Cohesion         |   `j/k`   |
| Separation       |   `m/,`   |
| Add/Remove Boids | `Up/Down` |
| Change Theme     |    `t`    |
| Hide Menu        |    `h`    |
