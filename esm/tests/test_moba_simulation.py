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
from datetime import timedelta

import pytest

from esm.core.esports.moba.simulation.moba_sim_engine import (
    MobaEventFactory,
    MobaEventType,
    MobaSimEngine,
    get_event_definitions,
)
from esm.core.esports.moba.simulation.moba_sim_match import (
    MobaSimMatch,
    NoChampionError,
)


def test_try_start_simulation_without_picking_champions(
    moba_match_simulation: MobaSimMatch,
) -> None:
    with pytest.raises(NoChampionError):
        moba_match_simulation.run()


def test_get_enabled_events_at_2_min(moba_match_simulation: MobaSimMatch) -> None:
    sim_engine = moba_match_simulation.simulation_engine
    sim_engine.match_time = timedelta(minutes=2)
    expected_enabled_events = [
        MobaEventType.NOTHING,
        MobaEventType.FIGHT,
    ]
    sim_engine.get_enabled_events()
    assert sim_engine.enabled_events == expected_enabled_events


def test_get_enabled_events_at_5_min(moba_match_simulation: MobaSimMatch) -> None:
    sim_engine = moba_match_simulation.simulation_engine
    sim_engine.match_time = timedelta(minutes=5)
    expected_enabled_events = [
        MobaEventType.NOTHING,
        MobaEventType.FIGHT,
        MobaEventType.JUNGLE_DRAGON,
    ]
    sim_engine.get_enabled_events()
    assert sim_engine.enabled_events == expected_enabled_events


def test_get_enabled_events_at_6_min(moba_match_simulation: MobaSimMatch) -> None:
    sim_engine = moba_match_simulation.simulation_engine
    sim_engine.match_time = timedelta(minutes=6)
    expected_enabled_events = [
        MobaEventType.NOTHING,
        MobaEventType.FIGHT,
        MobaEventType.JUNGLE_VOIDGRUBS,
        MobaEventType.JUNGLE_DRAGON,
    ]
    sim_engine.get_enabled_events()
    assert sim_engine.enabled_events == expected_enabled_events


def test_get_enabled_events_at_14_min(moba_match_simulation: MobaSimMatch) -> None:
    sim_engine = moba_match_simulation.simulation_engine
    sim_engine.match_time = timedelta(minutes=14)
    expected_enabled_events = [
        MobaEventType.NOTHING,
        MobaEventType.FIGHT,
        MobaEventType.JUNGLE_HERALD,
        MobaEventType.JUNGLE_DRAGON,
        MobaEventType.TOWER_ASSAULT,
    ]
    sim_engine.get_enabled_events()
    assert sim_engine.enabled_events == expected_enabled_events


def test_get_enabled_events_at_15_min(moba_match_simulation: MobaSimMatch) -> None:
    sim_engine = moba_match_simulation.simulation_engine
    sim_engine.match_time = timedelta(minutes=15)
    expected_enabled_events = [
        MobaEventType.NOTHING,
        MobaEventType.FIGHT,
        MobaEventType.JUNGLE_HERALD,
        MobaEventType.JUNGLE_DRAGON,
        MobaEventType.TOWER_ASSAULT,
    ]
    sim_engine.get_enabled_events()
    assert sim_engine.enabled_events == expected_enabled_events


def test_get_enabled_events_at_20_min(moba_match_simulation: MobaSimMatch) -> None:
    sim_engine = moba_match_simulation.simulation_engine
    sim_engine.match_time = timedelta(minutes=20)
    expected_enabled_events = [
        MobaEventType.NOTHING,
        MobaEventType.FIGHT,
        MobaEventType.JUNGLE_BARON,
        MobaEventType.JUNGLE_DRAGON,
        MobaEventType.TOWER_ASSAULT,
    ]
    sim_engine.get_enabled_events()
    assert sim_engine.enabled_events == expected_enabled_events


def test_get_enabled_events_with_inhibs_exposed(
    moba_match_simulation: MobaSimMatch,
) -> None:
    sim_engine = moba_match_simulation.simulation_engine
    sim_engine.match_time = timedelta(minutes=22)
    sim_engine.team1.towers.top = 0
    sim_engine.team1.inhibitors.take_down_inhib(
        "top", sim_engine.match_time, timedelta(5)
    )
    expected_enabled_events = [
        MobaEventType.NOTHING,
        MobaEventType.FIGHT,
        MobaEventType.JUNGLE_BARON,
        MobaEventType.JUNGLE_DRAGON,
        MobaEventType.TOWER_ASSAULT,
        MobaEventType.INHIB_ASSAULT,
    ]
    sim_engine.get_enabled_events()
    assert sim_engine.enabled_events == expected_enabled_events


def test_get_enabled_events_with_nexus_exposed(
    moba_match_simulation: MobaSimMatch,
) -> None:
    sim_engine = moba_match_simulation.simulation_engine
    sim_engine.match_time = timedelta(minutes=22)
    sim_engine.team1.towers.top = 0
    sim_engine.team1.inhibitors.take_down_inhib(
        "top", sim_engine.match_time, timedelta(5)
    )
    sim_engine.team1.towers.base = 0
    expected_enabled_events = [
        MobaEventType.NOTHING,
        MobaEventType.FIGHT,
        MobaEventType.JUNGLE_BARON,
        MobaEventType.JUNGLE_DRAGON,
        MobaEventType.TOWER_ASSAULT,
        MobaEventType.INHIB_ASSAULT,
        MobaEventType.NEXUS_ASSAULT,
    ]
    sim_engine.get_enabled_events()
    assert sim_engine.enabled_events == expected_enabled_events
