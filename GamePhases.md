# Game UI Layout
We have a playing field where the player selects actions with a moveable cursor and where the game takes place. A right bar for dynamic game information such as menu options or display data. And a bottom bar showing HUD information about the player's characters like health and status.

## Playing Field (Map)
- Grid with entities
- Rectangular

## Right SideBar
  - Displays Game information depending on its own internal state
    - Menu'ing
      - Move
      - Attack
      - Skill
      - Cancel
    - Display Enemy Information
    - Display Game Information

## Bottom Footer

Displays information about the player's characters
- health
- status
- hot key / color / glyph

# Game Phases

## Player Turn
Nothing selected
Move Cursor Around
Select Units with number keys
Press A on Entity to select it
- Player -> PlayerSelected
- Enemy -> Open Stats on overlay UI
Press A on field to open game menu

## Player Selected
Display move tiles
Cursor snaps to player on phase start
Cursor can select move tile with A
Input B to cancel selection

## Menu Opened
Select End turn
Close Menu

## Enemy Turn
Enemies move themselves

Player Turn <-> Player Selected + Command Menu Displayed on side bar
|
Game Menu Opened -> Enemy Turn
|
Quit