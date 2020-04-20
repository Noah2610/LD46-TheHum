# LD46 Level Creation Documentation
## Properties
- `z` (`float`)

No other tiled properties are used.  
Everything is configured within settings files.

## Tile types
Can be any string.  
Each type can have configuration specified in the  
[`resources/settings/tiles.ron`] config file.

## Object types
- `Player`
- `Bonfire`
- `Wood`
- `WoodSpawner`
- `Ladder`  
  The ladder object's size is used as the ladder's hitbox.
- `Beartrap(NUMBER)`  
  Beartrap spawns when bonfire has reached `NUMBER` amount of woods.
- `Radio`

[`resources/settings/tiles.ron`]: https://github.com/Noah2610/LD46/blob/master/resources/settings/tiles.ron
