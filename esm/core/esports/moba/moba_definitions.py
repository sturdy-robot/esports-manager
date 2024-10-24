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
from enum import Enum, auto

from ...serializable import Serializable


class LaneError(Exception):
    pass


class Lanes(Enum):
    """
    Defines lanes that can be played during a MOBA match
    """

    TOP = "TOP"
    JNG = "JNG"
    MID = "MID"
    ADC = "ADC"
    SUP = "SUP"


class LaneMultiplierError(Exception):
    pass


@dataclass
class LaneMultipliers(Serializable):
    top: float
    jng: float
    mid: float
    adc: float
    sup: float

    @staticmethod
    def _check_multiplier(value: float) -> bool:
        return 0.0 <= value <= 1.0

    def __post_init__(self):
        if not (
            self._check_multiplier(self.top)
            and self._check_multiplier(self.jng)
            and self._check_multiplier(self.mid)
            and self._check_multiplier(self.adc)
            and self._check_multiplier(self.sup)
        ):
            raise LaneMultiplierError(
                "Lane value cannot be negative or greater than 1.0!"
            )

    def __getitem__(self, key: Lanes):
        if key == Lanes.TOP:
            return self.top
        elif key == Lanes.JNG:
            return self.jng
        elif key == Lanes.MID:
            return self.mid
        elif key == Lanes.ADC:
            return self.adc
        elif key == Lanes.SUP:
            return self.sup
        else:
            raise KeyError(key)

    def __setitem__(self, key: Lanes, value: float):
        if key == Lanes.TOP:
            self.top = value
        elif key == Lanes.JNG:
            self.jng = value
        elif key == Lanes.MID:
            self.mid = value
        elif key == Lanes.ADC:
            self.adc = value
        elif key == Lanes.SUP:
            self.sup = value
        else:
            raise KeyError(key)

    def get_best_attribute(self) -> Lanes:
        lanes = self.serialize()
        max_attr = max(list(lanes.values()))

        if self.top == max_attr:
            return Lanes.TOP
        if self.jng == max_attr:
            return Lanes.JNG
        if self.mid == max_attr:
            return Lanes.MID
        if self.adc == max_attr:
            return Lanes.ADC
        if self.sup == max_attr:
            return Lanes.SUP

    @classmethod
    def get_from_dict(cls, dictionary: dict):
        attributes = get_lanes_from_dict(dictionary)

        return cls(
            attributes[Lanes.TOP],
            attributes[Lanes.JNG],
            attributes[Lanes.MID],
            attributes[Lanes.ADC],
            attributes[Lanes.SUP],
        )

    def serialize(self) -> dict[str, float]:
        return {
            Lanes.TOP.name: self.top,
            Lanes.JNG.name: self.jng,
            Lanes.MID.name: self.mid,
            Lanes.ADC.name: self.adc,
            Lanes.SUP.name: self.sup,
        }


def get_lanes_from_dict(lanes: dict[str, float]) -> dict[Lanes, float]:
    return {Lanes(lane): mult for lane, mult in lanes.items()}
