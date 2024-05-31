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
from dataclasses import asdict, dataclass
from typing import Optional

from esm.core.esports.player import Player
from esm.core.esports.rts.rts_definitions import RTSRace
from esm.core.serializable import Serializable


@dataclass
class RTSPlayerAttributes(Serializable):
    attack: int
    defense: int
    speed: int
    strategy: int
    micro: int
    macro: int

    def serialize(self) -> dict:
        return asdict(self)

    @classmethod
    def get_from_dict(cls, data: dict):
        return cls(**data)

    def get_overall(self) -> int:
        return int(
            (
                self.attack
                + self.defense
                + self.speed
                + self.strategy
                + self.micro
                + self.macro
            )
            / 6
        )


@dataclass
class RTSPlayer(Player, Serializable):
    attributes: RTSPlayerAttributes
    races_multiplier: dict[RTSRace, float]

    def serialize(self) -> dict:
        races_multiplier = {
            race.value: multiplier for race, multiplier in self.races_multiplier.items()
        }
        return {
            "player_id": self.player_id.hex,
            "first_name": self.first_name,
            "last_name": self.last_name,
            "attributes": self.attributes.serialize(),
            "races_multiplier": races_multiplier,
        }

    @classmethod
    def get_from_dict(cls, dictionary: dict):
        races_multiplier = {
            RTSRace(race): multiplier
            for race, multiplier in dictionary["races_multiplier"].items()
        }

        return cls(
            player_id=uuid.UUID(dictionary["player_id"]),
            nationality=dictionary["nationality"],
            first_name=dictionary["first_name"],
            last_name=dictionary["last_name"],
            birthday=datetime.datetime.strptime(
                dictionary["birthday"], "%Y-%m-%d"
            ).date(),
            nick_name=dictionary["nick_name"],
            attributes=RTSPlayerAttributes.get_from_dict(dictionary["attributes"]),
            races_multiplier=races_multiplier,
        )

    def get_best_race(self) -> RTSRace:
        return max(self.races_multiplier, key=self.races_multiplier.get)

    def get_overall(self, race: Optional[RTSRace] = None) -> int:
        if race is None:
            race = self.get_best_race()

        return int(self.attributes.get_overall() * self.races_multiplier[race])

    def __str__(self):
        return f"{self.nick_name}"

    def __repr__(self):
        return self.__str__()
