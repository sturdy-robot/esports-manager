#      eSports Manager - free and open source eSports Management game
#      Copyright (C) 2020-2022  Pedrenrique G. Guimarães
#
#      This program is free software: you can redistribute it and/or modify
#      it under the terms of the GNU General Public License as published by
#      the Free Software Foundation, either version 3 of the License, or
#      (at your option) any later version.
#
#      This program is distributed in the hope that it will be useful,
#      but WITHOUT ANY WARRANTY; without even the implied warranty of
#      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#      GNU General Public License for more details.
#
#      You should have received a copy of the GNU General Public License
#      along with this program.  If not, see <https://www.gnu.org/licenses/>.
import re
import os

from pathlib import Path
from typing import Union
from unicodedata import normalize

from esm.core.esports.manager import Manager
from esm.core.gamestate import GameState
from esm.core.load_game import LoadGame
from esm.core.save_game import SaveGame
from esm.core.settings import Settings
from esm.core.generator.generate_champions import ChampionGenerator
from esm.core.generator.generate_players import MobaPlayerGenerator
from esm.core.generator.generate_teams import TeamGenerator


class GameManager:
    """
    Manages the state of the current game session.
    """

    def __init__(
            self,
            manager: Manager,
            filename: str,
            esport: str,
            season: str,
            game_name: str,
            settings: Settings,
            auto_save_enabled: bool = True
    ):
        self.manager = manager
        self.esport = esport
        self.season = season
        self.filename = self.normalize_filename(filename)
        self.game_name = game_name
        self.teams = TeamGenerator()
        self.players = MobaPlayerGenerator()
        self.champions = ChampionGenerator()
        self.gamestate = self.get_gamestate()
        self.settings = settings
        self.auto_save_enabled = auto_save_enabled
        self.load = None
        self.save = None

    def get_gamestate(self):
        self.teams.get_teams_objects()
        self.players.get_players_objects()
        self.champions.get_champions()
        gamestate = GameState(
            self.game_name,
            self.filename,
            self.manager.get_dict(),
            self.season,
            self.esport,
            self.teams.teams_dict.copy(),
            self.players.players_dict.copy(),
            self.champions.champions_list.copy(),
        )
        self.reset_generators()
        return gamestate
    
    def normalize_filename(self, filename, delim=u'_'):
        """
        Normalizes the save game filename. This will prevent unsupported filenames from being saved.

        Solution from: https://stackoverflow.com/questions/9042515/normalizing-unicode-text-to-filenames-etc-in-python
        """
        _punct_re = re.compile(r'[\t !"#$%&\'()*\-/<=>?@\[\\\]^_`{|},.:]+')
        result = []
        for word in _punct_re.split(filename.lower()):
            word = normalize('NFKD', word).encode('ascii', 'ignore')
            word = word.decode('utf-8')
            if word:
                result.append(word)
        
        filename = delim.join(result)
        filename = ''.join(filename)
        filename = filename + '.cbor'
        return filename
    
    def reset_generators(self):
        """
        Resets generators to avoid keeping them in memory.
        """
        self.teams = TeamGenerator()
        self.players = MobaPlayerGenerator()
        self.champions = ChampionGenerator()

    def create_save_game(self):
        self.save = SaveGame(
            self.get_gamestate(),
            self.filename,
            save_directory=self.settings.save_file_dir,
            autosave_enabled=self.auto_save_enabled,
        )

    def save_game(self):
        """
        Calls the SaveGame module to save the game file.
        """
        self.reset_generators()
        self.create_save_game()
        self.save.save_game()

    def auto_save(self):
        """
        Calls the SaveGame module to save an autosave file.
        """
        self.reset_generators()
        self.create_save_game()
        self.save.save_autosave()

    def get_load_game_files(self):
        self.load = LoadGame()
        self.load.get_load_game_files('.cbor')

    def check_if_save_file_exists(self, filename: Union[str, Path]):
        return os.path.exists(filename)

    def get_autosave_files(self):
        self.load = LoadGame()
        self.load.get_load_game_files('.autosav')

    def load_game(self, filename: Union[Path, str]):
        self.filename = filename
        self.gamestate = self.load.load_game_state(filename)
        self.save = SaveGame(self.gamestate, filename)

    def get_temporary_file(self):
        self.reset_generators()
        self.create_save_game()
        self.save.save_temporary_file()
        return self.save.temporary_file
