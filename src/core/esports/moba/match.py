#      eSports Manager - free and open source eSports Management game
#      Copyright (C) 2020  Pedrenrique G. Guimarães
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

import uuid

from src.core.esports.moba.team import Team


class Match:
    """
    The Match class is used to represent a match, whether they include
    the user's team or not.
    """

    def __init__(self,
                 championship_id: int,
                 team1: Team,
                 team2: Team):
        """
        Initializes elements of the match
        :param match_id: match ID
        :param championship_id: championship ID to which the match belongs
        :param team1: first team (blue side/radiant)
        :param team2: second team (red side/dire)
        """
        self.match_id = uuid.uuid4().int
        self.championship_id = championship_id
        self.team1 = team1
        self.team2 = team2
        self._teams = []

    @property
    def teams(self):
        self._teams.clear()

        self._teams = [self.team1, self.team2]
        return self._teams

    def __repr__(self) -> str:
        return '{0} {1}'.format(self.__class__.__name__, self.match_id)

    def __str__(self) -> str:
        return '{0} ID: {1}'.format(self.__class__.__name__, self.match_id)
