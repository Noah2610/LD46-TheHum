# deathframe-template
Rust game-dev project template for [`deathframe`] / [`amethyst`].

## Features
Using [`amethyst`] and [`deathframe`], this template sets-up  
the following boilerplate code and features:

- `ECS` ([`specs`]) structure  
  `components` and `systems` modules.
- Initialize dispatchers  
  Build dispatchers with common `deathframe` and `amethyst` bundles and systems.
- Setup common states  
  States for initially loading resources, the main menu, and the actual game.
- Input bindings  
  Setup and load (empty) input bindings from configs.

When running the template without changes, an empty window should appear.

## Reasoning
The [`amethyst`] game engine is __awesome__.  

But one issue I have with it, is that setting up a project takes  
_a lot_ of boilerplate code.  
I usually tend to just copy/paste boilerplate code from my previous  
`amethyst` game project, to speed-up the setup part.  
Still, this often takes a whole day for me.  

Because of similar reasons, I use my [`deathframe`] crate to share  
features/code across projects, which I usually just rewrite for each game.  
`deathframe` is like an `amethyst` _"goodie-bag"_ for me.

## License
Distributed under the [Unlicense].  
Do whatever you want with this template.  
Consider changing the license after cloning/forking to whatever you wish to use.

[`deathframe`]: https://github.com/Noah2610/deathframe
[`amethyst`]:   https://github.com/amethyst/amethyst
[`specs`]:      https://github.com/amethyst/specs
[Unlicense]:   ./UNLICENSE
