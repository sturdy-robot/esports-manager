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
from uuid import UUID
from typing import Union
from abc import ABC, abstractmethod
from .team import Team
from .player import MobaPlayer


class Progression(ABC):
    @abstractmethod
    def progress(self, *args, **kwargs):
        pass


class PlayerProgression(Progression):
    def progress(self, opp_team: Team, player: MobaPlayer):
        pass


class ChampionProgression(Progression):
    def progress(self, player: MobaPlayer, champion_id: Union[UUID, int]):
        pass