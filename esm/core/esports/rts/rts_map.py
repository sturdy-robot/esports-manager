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

from esm.core.esports.rts.rts_definitions import RTSRace
from esm.core.serializable import Serializable


@dataclass
class RTSMap(Serializable):
    map_id: uuid.UUID
    name: str
    race_multiplier: dict[RTSRace, float]

    @classmethod
    def get_from_dict(cls, dictionary: dict):
        race_multiplier = {
            RTSRace(race): multiplier
            for race, multiplier in dictionary["race_multiplier"].items()
        }
        return cls(
            map_id=uuid.UUID(dictionary["id"]),
            name=dictionary["name"],
            race_multiplier=race_multiplier,
        )

    def serialize(self):
        return {
            "id": self.map_id.hex,
            "name": self.name,
            "race_multiplier": {
                race.value: multiplier
                for race, multiplier in self.race_multiplier.items()
            },
        }
