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

from ...mobateam import MobaTeamSimulation
from ..moba_event_base import MobaEvent
from ..moba_event_type import MobaEventType
from .moba_event_fight import MobaEventFight
from .moba_event_inhib import MobaEventInhibAssault
from .moba_event_jungle import MobaEventJungle
from .moba_event_nexus import MobaEventNexusAssault
from .moba_event_nothing import MobaEventNothing
from .moba_event_tower import MobaEventTowerAssault


def get_event_definitions() -> dict[MobaEventType, dict[str, str | int | float]]:
    return {
        MobaEventType.NOTHING: {
            "points": 0,
            "priority": 20,
            "cooldown": 0,
            "start_time": 0,
            "end_time": 0,
        },
        MobaEventType.FIGHT: {
            "points": 0,
            "priority": 5,
            "cooldown": 0,
            "start_time": 0,
            "end_time": 0,
        },
        MobaEventType.JUNGLE_VOIDGRUBS: {
            "points": 15,
            "priority": 15,
            "cooldown": 4,
            "start_time": 6,
            "end_time": 14,
        },
        MobaEventType.JUNGLE_HERALD: {
            "points": 15,
            "priority": 15,
            "cooldown": 0,
            "start_time": 14,
            "end_time": 20,
        },
        MobaEventType.JUNGLE_BARON: {
            "points": 15,
            "priority": 20,
            "cooldown": 5,
            "start_time": 20,
            "end_time": 0,
        },
        MobaEventType.JUNGLE_DRAGON: {
            "points": 15,
            "priority": 20,
            "cooldown": 5,
            "start_time": 5,
            "end_time": 0,
        },
        MobaEventType.TOWER_ASSAULT: {
            "points": 15,
            "priority": 15,
            "cooldown": 5,
            "start_time": 10,
            "end_time": 0,
        },
        MobaEventType.INHIB_ASSAULT: {
            "points": 10,
            "priority": 20,
            "cooldown": 5,
            "start_time": 0,
            "end_time": 0,
        },
        MobaEventType.NEXUS_ASSAULT: {
            "points": 40,
            "priority": 50,
            "cooldown": 0,
            "start_time": 0,
            "end_time": 0,
        },
    }


class MobaEventFactory:
    def get_points(self, event_type: MobaEventType) -> float:
        return get_event_definitions()[event_type]["points"]

    def create_event(
        self,
        event_type: MobaEventType,
        team1: MobaTeamSimulation,
        team2: MobaTeamSimulation,
        event_time: timedelta,
    ) -> MobaEvent:
        if event_type == MobaEventType.NOTHING:
            return MobaEventNothing(team1, team2, event_time)
        elif event_type == MobaEventType.FIGHT:
            return MobaEventFight(team1, team2, event_time, self.get_points(event_type))
        elif event_type in [
            MobaEventType.JUNGLE_VOIDGRUBS,
            MobaEventType.JUNGLE_HERALD,
            MobaEventType.JUNGLE_DRAGON,
            MobaEventType.JUNGLE_BARON,
        ]:
            return MobaEventJungle(
                event_type, team1, team2, event_time, self.get_points(event_type)
            )
        elif event_type == MobaEventType.INHIB_ASSAULT:
            return MobaEventInhibAssault(
                team1, team2, event_time, self.get_points(event_type)
            )
        elif event_type == MobaEventType.TOWER_ASSAULT:
            return MobaEventTowerAssault(
                team1, team2, event_time, self.get_points(event_type)
            )
        elif event_type == MobaEventType.NEXUS_ASSAULT:
            return MobaEventNexusAssault(
                team1, team2, event_time, self.get_points(event_type)
            )
