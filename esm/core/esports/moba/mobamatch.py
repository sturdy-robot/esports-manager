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
from dataclasses import dataclass
from datetime import datetime
from typing import Optional
from uuid import UUID

from esm.core.esports.moba.mobateam import MobaTeam
from esm.core.serializable import Serializable


class InvalidTeamId(Exception):
    pass


@dataclass
class MobaMatch(Serializable):
    game_id: UUID
    championship_id: UUID
    team1: MobaTeam
    team2: MobaTeam
    date: datetime
    victorious_team: Optional[MobaTeam] = None

    def serialize(self) -> dict:
        if self.victorious_team is not None and self.victorious_team not in [
            self.team1,
            self.team2,
        ]:
            raise InvalidTeamId("Team cannot be the victorious team in this match!")

        victorious_team = (
            self.victorious_team.team_id.hex if self.victorious_team else None
        )

        return {
            "game_id": self.game_id.hex,
            "championship_id": self.championship_id.hex,
            "team1": self.team1.team_id.hex,
            "team2": self.team2.team_id.hex,
            "date": self.date.strftime("%Y-%m-%d, %H:%M"),
            "victorious_team": victorious_team,
        }

    @classmethod
    def get_from_dict(cls, dictionary: dict, team1: MobaTeam, team2: MobaTeam):
        victorious_team = dictionary["victorious_team"]

        if victorious_team:
            if victorious_team == team1.team_id.hex:
                victorious_team = team1
            elif victorious_team == team2.team_id.hex:
                victorious_team = team2
            else:
                raise InvalidTeamId("Team ID in 'victorious_team' field is invalid!")

        return cls(
            UUID(hex=dictionary["game_id"]),
            UUID(hex=dictionary["championship_id"]),
            team1,
            team2,
            datetime.strptime(dictionary["date"], "%Y-%m-%d, %H:%M"),
            victorious_team,
        )

    def __repr__(self) -> str:
        return "{0} {1}".format(self.__class__.__name__, self.game_id)

    def __str__(self) -> str:
        return "{0} ID: {1}".format(self.__class__.__name__, self.game_id)
