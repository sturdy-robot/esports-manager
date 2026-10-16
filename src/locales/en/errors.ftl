## Errors from New Game, saves and queries.

error-manager-name-empty = Enter a manager name.
error-manager-name-long = The manager name can have at most { $max } characters.
error-save-name-empty = Enter a save name.
error-save-name-long = The save name can have at most { $max } characters.
# $field is "manager" or "save".
error-control-characters = { $field ->
    [manager] The manager name
   *[save] The save name
} can't contain control characters.
error-unknown-team = There is no team "{ $team }".
error-unknown-player = There is no player "{ $player }".
error-generation = The world couldn't be generated from this package.
error-save-name-taken = A save named "{ $name }" already exists.
error-save-not-found = There is no save "{ $file }".
error-save-not-a-save = This file isn't a save.
error-save-newer = This save was made by a newer version of the game.
error-save-in-use = This save is open somewhere else.
error-storage = The save couldn't be read or written: { $detail }
