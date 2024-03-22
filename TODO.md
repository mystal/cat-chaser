# Bevy Port TODO
## Port
* [x] Loading infrastructure
* [x] Animated sprites
  * Mostly working, seems to be a bug with bevy_aseprite
* [ ] Set window icon: https://bevy-cheatbook.github.io/window/icon.html
* [ ] Set exe/app icon
  * On Linux
  * On Windows: https://bevy-cheatbook.github.io/platforms/windows.html#creating-an-icon-for-your-app
  * On macOS
* [ ] Dog
  * [x] Movement
  * [x] Bark
  * [ ] Get hit and play sound and blink
* [ ] Cats
  * Different Cat types
    * [x] Basic
    * [x] Chonk
    * [x] Kitten
  * [x] Cat colors
  * Behaviors
    * [ ] Wandering
    * [x] Flee
    * [x] In Pen
    * [x] Angry!
* [ ] Level win condition
* [x] Levels!
* [ ] Menus
  * [x] Start menu
  * [x] How to play
  * [ ] Credits
* [x] HUD
* [ ] Loading screen
* [ ] Full-screen support
  * Adjust viewport and use black borders to make sure we render to a multiple of GAME_SIZE
* Audio
  * Sound effects
    * [x] Dog bark
    * [x] Cat sounds
  * [x] Background music

## Tech
* [x] Update to bevy 0.13
* [ ] Try out bevy_xpbd
* [x] Either fix bevy_aseprite or find a different aseprite plugin
  * Switched to bevy_asepritesheet
* [ ] Make a schedule for the game, something like: Input, Logic, Movement, UI
