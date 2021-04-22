#      eSports Manager - free and open source eSports Management game
#      Copyright (C) 2020-2021  Pedrenrique G. Guimarães
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

import threading

from src.core.esports.moba.mobaevent import MobaEventHandler
from src.core.core import Core
from src.core.esports.moba.match_live import MatchLive

from src.resources.generator.generate_teams import TeamGenerator
from src.resources.generator.generate_players import MobaPlayerGenerator
from src.resources.generator.generate_champions import ChampionGenerator
from src.ui.view import View


class ESM:
    def __init__(self, amount_players=400):
        self.core = Core()
        self._amount_players = amount_players
        self.view = View(self)
        self.current_match = None
        self.match_thread = None

    @property
    def amount_players(self) -> int:
        return self._amount_players

    @amount_players.setter
    def amount_players(self, amount):
        self.core.amount_players = amount
        self._amount_players = amount

    def start_match_sim(self) -> None:
        self.current_match.simulation()
        self.view.update_match_sim_elements()

    def generate_all_data(self) -> None:
        """
        Starts a thread to generate data and show a window progress bar.
        """
        self.reset_generators()
        try:
            generate_data_thread = threading.Thread(target=self.core.generate_all, daemon=True)
            generate_data_thread.start()
            self.view.print_generate_data_window(self.core.players.players_dict, self.core.teams.teams_dict, self.core.champions.champions_list)
            generate_data_thread.join()
        except Exception as e:
            self.view.print_error(e)

    def check_files(self) -> None:
        try:
            self.core.check_files()
        except FileNotFoundError as e:
            self.generate_all_data()

    def reset_generators(self) -> None:
        """
        Resets all generators. This prevents memory allocation of unnecessary elements.
        """
        self.core.teams = TeamGenerator()
        self.core.players = MobaPlayerGenerator()
        self.core.champions = ChampionGenerator()

    def reset_match(self, match) -> None:
        self.core.reset_team_values(match)
        match.is_match_over = False
        match.game_time = 0.0
        match.event_handler = MobaEventHandler()
        match.victorious_team = None
    
    def initialize_random_debug_match(self) -> MatchLive:
        self.core.initialize_random_debug_match()

        # Resetting
        self.reset_generators()

        self.core.match_simulation.picks_and_bans()
        
        self.current_match = self.core.match_simulation

        return self.core.match_simulation

    def start_match_sim_thread(self) -> None:
        try:
            self.match_thread = threading.Thread(target=self.start_match_sim, daemon=True)
            self.match_thread.start()
            self.view.disable_debug_buttons()
        except Exception as e:
            self.view.print_error(e)

    def app(self) -> None:
        self.view.start()


esm = ESM()
esm.app()
