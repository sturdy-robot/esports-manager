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
import random
from datetime import timedelta

from ..mobateam import MobaTeamSimulation
from .events import MobaEvent, MobaEventFactory, get_event_definitions
from .moba_event_type import MobaEventType


class MobaSimEngine:
    def __init__(self, team1: MobaTeamSimulation, team2: MobaTeamSimulation):
        self.team1 = team1
        self.team2 = team2
        self.event_definitions = get_event_definitions()
        self.enabled_events = []
        self.event_history = []
        self.event_factory = MobaEventFactory()
        self.match_time: timedelta = timedelta(0)

    def is_match_over(self) -> bool:
        if self.team1.nexus == 0 or self.team2.nexus == 0:
            return True

        return False

    def update_cooldowns(self):
        self.team1.inhibitors.update_cooldown(self.match_time)
        self.team2.inhibitors.update_cooldown(self.match_time)

    def get_enabled_events(self) -> None:
        self.enabled_events = []

        for event_type in self.event_definitions:
            match_time = int(self.match_time.seconds / 60)
            if event_type == MobaEventType.INHIB_ASSAULT:
                if self.team1.are_inhibs_exposed() or self.team2.are_inhibs_exposed():
                    self.enabled_events.append(event_type)
                    continue
            elif event_type == MobaEventType.NEXUS_ASSAULT:
                if self.team1.is_nexus_exposed() or self.team2.is_nexus_exposed():
                    self.enabled_events.append(event_type)
                    continue
            elif self.event_definitions[event_type]["end_time"] != 0:
                if (
                    self.event_definitions[event_type]["start_time"]
                    <= match_time
                    < self.event_definitions[event_type]["end_time"]
                ):
                    self.enabled_events.append(event_type)
            else:
                if match_time >= self.event_definitions[event_type]["start_time"]:
                    self.enabled_events.append(event_type)

    def get_event(self) -> MobaEvent:
        self.priorities = []
        for event in self.enabled_events:
            for ev in self.event_definitions:
                if event == ev:
                    self.priorities.append(self.event_definitions[event]["priority"])

        chosen_event_type = random.choices(
            self.enabled_events, weights=self.priorities
        )[0]
        event = self.event_factory.create_event(
            chosen_event_type, self.team1, self.team2, self.match_time
        )
        self.event_history.append(event)
        return event

    def run(self):
        while not self.is_match_over():
            self.update_cooldowns()
            self.get_enabled_events()
            event = self.get_event()
            event.calculate_event()
            self.match_time += event.duration
