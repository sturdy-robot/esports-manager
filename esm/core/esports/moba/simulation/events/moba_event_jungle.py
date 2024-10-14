#  eSports Manager - free and open source eSports Management game
#  Copyright (C) 2020-2024  Pedrenrique G. Guimarães
#
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU General Public License for more details.
#
#  You should have received a copy of the GNU General Public License
#  along with this program.  If not, see <https://www.gnu.org/licenses/>.
from datetime import timedelta
from enum import Enum, auto

from ...mobateam import MobaTeamSimulation
from ..moba_event_base import MobaEvent, MobaEventBase, MobaEventPriority
from ..moba_event_type import MobaEventType


class MobaEventJungleError(Exception):
    pass


class MobaEventJungle(MobaEvent, MobaEventBase):
    def __init__(
        self,
        event_type: MobaEventType,
        team1: MobaTeamSimulation,
        team2: MobaTeamSimulation,
        event_time: timedelta,
        points: float,
    ):
        super().__init__(
            event_type,
            team1,
            team2,
            MobaEventPriority.HIGH,
            event_time,
            points,
        )

    def calculate_baron(self):
        pass

    def calculate_dragon(self):
        pass

    def calculate_voidgrubs(self):
        pass

    def calculate_herald(self):
        pass

    def calculate_event(self):
        if self.event_type == MobaEventType.JUNGLE_VOIDGRUBS:
            self.calculate_voidgrubs()
        elif self.event_type == MobaEventType.JUNGLE_BARON:
            self.calculate_baron()
        elif self.event_type == MobaEventType.JUNGLE_DRAGON:
            self.calculate_dragon()
        elif self.event_type == MobaEventType.JUNGLE_HERALD:
            self.calculate_herald()

        else:
            raise MobaEventJungleError("Invalid event type was passed to Jungle event!")
