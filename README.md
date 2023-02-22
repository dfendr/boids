# Boids 🕊

**boids** is a Flocking simulator written in Rust, using the Nannou framework.

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

You'll need Rust installed on your system to build, which you can [install here](https://www.rust-lang.org/learn/get-started).

### MacOS/Linux/Windows

- To download and run the program, execute the following commands.

```sh
git clone https://github.com/postfen/boids.git
cd ./boids
cargo build --release
cd target/release
open .
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
