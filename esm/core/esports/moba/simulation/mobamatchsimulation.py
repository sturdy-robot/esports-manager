#      eSports Manager - free and open source eSports Management game
#      Copyright (C) 2020-2024  Pedrenrique G. Guimarães
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
import time
from typing import Optional, Tuple, Union

from esm.core.esports.moba.mobamatch import MobaMatch
from esm.core.esports.moba.mobateam import MobaTeamSimulation
from esm.core.esports.moba.simulation.mobasimulationengine import MobaSimulationEngine


class MobaMatchSimulation:

    def __init__(
        self,
        game: MobaMatch,
        team1: MobaTeamSimulation,
        team2: MobaTeamSimulation,
        show_commentary: bool = True,
        match_speed: int = 1,
        simulation_delay: bool = True,
        difficulty_level: int = 1,
    ):
        self.game = game
        self.team1 = team1
        self.team2 = team2
        self.teams = [self.team1, self.team2]
        self.show_commentary = show_commentary
        self.match_speed = match_speed
        self.is_match_over = False
        self.is_running = False
        self.simulation_delay = simulation_delay
        self.engine = MobaSimulationEngine(self.show_commentary)
        self.difficulty_level = difficulty_level

    def reset_match(self) -> None:
        self.reset_teams()
        self.is_match_over = False
        self.engine = MobaSimulationEngine(self.show_commentary)

    def check_is_player_match(self) -> bool:
        return any(team.is_players_team for team in self.teams)

    def reset_teams(self) -> None:
        for team in self.teams:
            team.reset_values()

    def calculate_both_teams_win_prob(self) -> None:
        total_prob = sum(team.total_skill for team in self.teams)

        for team in self.teams:
            team.win_prob = team.total_skill / total_prob

    def check_inhibitor_cooldown(self):
        for team in self.teams:
            for inhib, cooldown in team.inhibitors_cooldown.items():
                if cooldown > 0.0:
                    team.inhibitors_cooldown[inhib] -= 0.5
                if cooldown <= 0.0 and team.inhibitors[inhib] == 0:
                    team.inhibitors_cooldown[inhib] = 0.0
                    team.inhibitors_cooldown[inhib] = 1

    def get_team_exposed_nexus(
        self,
    ) -> Optional[
        Union[Tuple[MobaTeamSimulation, MobaTeamSimulation], MobaTeamSimulation]
    ]:
        """
        Gets the exposed nexus from one or both of the teams.
        """
        if self.team1.is_nexus_exposed():
            if self.team2.is_nexus_exposed():
                return self.team1, self.team2
            return self.team1
        elif self.team2.is_nexus_exposed():
            return self.team2
        else:
            return None

    def get_towers_number(self) -> int:
        return sum(sum(team.towers.values()) for team in self.teams)

    def check_match_over(self) -> None:
        """
        Checks if one of the nexus is down and terminates the simulation
        """
        for team in self.teams:
            if team.nexus == 0:
                self.is_match_over = True

    def is_any_inhib_open(self) -> bool:
        """
        Checks for open inhibitors, to decide whether a base tower or nexus can be attacked
        """
        return any(team.get_exposed_inhibs() for team in self.teams)

    def get_winning_team(self) -> Optional[MobaTeamSimulation]:
        if self.is_match_over:
            if self.team1.nexus == 1:
                self.game.victorious_team = self.game.team1
                return self.team1
            else:
                self.game.victorious_team = self.game.team2
                return self.team2

        return None

    def run(self) -> None:
        while not self.is_running:
            self.step()

    def step(self):
        self.calculate_both_teams_win_prob()
        self.engine.run()
        self.check_inhibitor_cooldown()
        self.check_match_over()

        if not self.is_match_over:
            self.engine.advance_simulation()

        if self.simulation_delay:
            time.sleep(self.match_speed)
