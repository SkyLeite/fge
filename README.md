# Meta Declare

Fully declarative fighting game engine powered by Bevy.

## Project

### Goals

- Simple: users should be able to create new features with the existing systems, avoiding the need for bespoke ones.
- Declarative: creating a new character must be about declaring the end-result, rather than programming the destination.
- Accessible: minimal traditional programming experience should be required to make a (basic) game.
- Extensible: programmers should be able to extend the engine with new functionality without recompiling it.
- Performant: for rollback netcode to work reliably the engine needs to work efficiently, as it requires simulating multiple game frames per 16.6ms interval.
- Pluggable: integrating ME with existing tools should be easy.

### Non-goals

- Extreme flexibility: not every fighting game mechanic in existance will be able to be reasonably replicated in ME.
