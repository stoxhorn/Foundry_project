# Foundry_project
Public example of my first Rust project.
I wanted to create a simulation system for anvil.
I have an idea for my own DeFi project, where i want to be able to experiment with ideas, as i'm new to Solidity.
Instead of going with a crate already build, or using a javascript framework, i wanted to learn Rust.
On top of this i can personalize it, and know all the ins and outs, such that i can more easily and fast configure it.



# mad_simmer crate
This is my simulation crate. The sim_structs.rs, was a bit overkill to start out with, and is mostly unused for now.
It will get used more, once i start implementing logging, so i can see only the results i need.
When i was making my diagrams and planning, there was one thing i didn't account for. That was the sol! macro from Alloy.
It is a huuuge pain to work with. As it is a macro it creates a new struct for every solidity contract, and it's types have a lot of generic magic, that i couldn't figure out.
I really wanted to be able to describe a simulation in a json format, where i could list the contracts i wanted it to include, and then a list of "actions", transaction or other events.
But because of the sol! macro, i'm uncertain of how to make this as dynamic as i want it to be.

So i've instead had to rely on pattern matching. This is seen in the solidity_files.rs file.


# fraquility crate
Mostly empty, contains a template contract, and my own weird token contract. These are simply used to test the mad_simmer crate against.
This crate has foundry and alloy installed.
