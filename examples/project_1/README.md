# Project 1 - Hello, World!
This first project is about setting up the environment for using OpenGL. We will create an OpenGL window and color it using basic OpenGL functions. 
**CS6600 Bonus Requirement** Animate the background color.

[Full Project Requirements](https://graphics.cs.utah.edu/courses/cs6610/spring2024/?prj=1)

## Running

Test it out yourself by running: `cargo run --example project_1`

You can also run it with Nix Flakes to manage dependency hell: `nix run /path/to/flake -- run --example project_1`

## Notes
- I tried many different windowing libraries and ended up using GLFW.
- I spent considerable time creating a library for OpenGL. Wrapping types, and unsafe functions in a more ergnomic fashion.
- I also set up a Nix-Flake to provide a development environment

## Issues
- Getting a windowing library to work with Wayland was harder than expected. Confounded by using Rust, Nvidia, and Nix. I did learn a lot about linking, and patching libraries on Nix as a result.
- I am not sure if it works on X11 desktops, or other operating systems. This is because I restrict GLFW to using Wayland, otherwise it assumes X11 and crashes when it is not found on my system. I am still unsure how to inform GLFW to compile assuming Wayland.

-----

![Screen Shot of Project 1](https://github.com/xvrqt/cs6600/blob/master/examples/project_1/screenshot.png?raw=true "Screenshot of Project 1")
