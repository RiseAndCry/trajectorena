# Trajectorena

A small, fast-paced 2D game where wizards try to take each other out 
by slinging various spells and outsmarting their opponents

### Setup
1. Install rust - https://www.rust-lang.org/tools/install
2. `cargo run` 

### Gameplay instructions

<i>note: Players can only move inside their castles.</i>
#### Player 1
- use `W A S D` keys to move
- left mouse button to shoot.
- Pressing and holding the right mouse button in the vicinity of spell will stop it,
releasing the button will resume spell movement. 

#### Player 2
<i>note: Until multiplayer / AI is implemented this player can only move.</i>

use `Up, Left, Down, Right` keys to move

### Roadmap
- [x] Arena created
- [x] Players are able to move inside their castles
- [x] Basic spell shooting (cooldown, bouncing against walls)
- [x] Player health tracking and game over state
- [x] UI (main menu and game over screen)
- [x] Spell holding
- [ ] AI for single player
- [ ] Different kinds of spells
- [ ] Mana system
- [ ] Powerups appearing on the map
- [ ] Graphics
- [ ] Multiplayer (over network)
...