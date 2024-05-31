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
import uuid
from dataclasses import dataclass

from esm.core.esports.rts.rts_player import RTSPlayer
from esm.core.serializable import Serializable


@dataclass
class RTSTeam(Serializable):
    team_id: uuid.UUID
    name: str
    nationality: str
    roster: list[RTSPlayer]

    def serialize(self):
        return {
            "id": self.team_id.hex,
            "name": self.name,
            "nationality": self.nationality,
            "roster": [player.player_id for player in self.roster],
        }

    @classmethod
    def get_from_dict(cls, dictionary: dict, players: list[RTSPlayer]):
        return cls(
            team_id=uuid.UUID(dictionary["id"]),
            name=dictionary["name"],
            nationality=dictionary["nationality"],
            roster=players,
        )

    def __str__(self):
        return f"{self.name}"
