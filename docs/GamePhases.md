# Game UI Layout
We have a playing field where the cat selects actions with a moveable cursor and where the game takes place. A right bar for dynamic game information such as menu options or display data. And a bottom bar showing HUD information about the cat's characters like health and status.

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

Displays information about the cat's characters
- health
- status
- hot key / color / glyph

# Game Phases

## cat Turn
Nothing selected
Move Cursor Around
Select Units with number keys
Press A on Entity to select it
- cat -> catSelected
- Enemy -> Open Stats on overlay UI
Press A on field to open game menu

## cat Selected
Display move tiles
Cursor snaps to cat on phase start
Cursor can select move tile with A
Input B to cancel selection

## Menu Opened
Select End turn
Close Menu

## Enemy Turn
Enemies move themselves

cat Turn <-> cat Selected + Command Menu Displayed on side bar
|
Game Menu Opened -> Enemy Turn
|
Quit