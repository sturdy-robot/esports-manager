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
import datetime
import uuid
from dataclasses import dataclass
from typing import Optional

from esm.core.esports.rts.rts_map import RTSMap
from esm.core.esports.rts.rts_player import RTSPlayer
from esm.core.serializable import Serializable


@dataclass
class RTSMatch(Serializable):
    id: uuid.UUID
    player1: RTSPlayer
    player2: RTSPlayer
    rts_map: RTSMap
    date: datetime.datetime
    winner: Optional[RTSPlayer] = None

    def serialize(self) -> dict:
        return {
            "id": self.id.hex,
            "player1": self.player1.player_id.int,
            "player2": self.player2.player_id.int,
            "map": self.rts_map.map_id.int,
            "date": self.date.strftime("%Y-%m-%d, %H:%M"),
            "winner": self.winner.player_id.int if self.winner else None,
        }

    @classmethod
    def get_from_dict(
        cls, dictionary: dict, player1: RTSPlayer, player2: RTSPlayer, rts_map: RTSMap
    ):
        winner = None
        if dictionary["winner"] is not None:
            winner = (
                player1 if dictionary["winner"] == player1.player_id.int else player2
            )
        return cls(
            id=uuid.UUID(dictionary["id"]),
            player1=player1,
            player2=player2,
            rts_map=rts_map,
            date=datetime.datetime.strptime(dictionary["date"], "%Y-%m-%d, %H:%M"),
            winner=winner,
        )
