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
import pytest

from esm.core.esports.moba.simulation.mobamatchsimulation import (
    MobaMatchSimulation,
    NoChampionError,
)
from esm.core.esports.moba.simulation.mobasimulationengine import MobaSimulationEngine
from esm.core.esports.moba.simulation.picksbans import PicksBans


@pytest.fixture
def moba_simulation_engine() -> MobaSimulationEngine:
    return MobaSimulationEngine(show_commentary=False)


def test_try_start_simulation_without_picking_champions(
    moba_match_simulation: MobaMatchSimulation,
) -> None:
    with pytest.raises(NoChampionError):
        moba_match_simulation.run()


def test_picksbans_bans_phase(
    moba_match_simulation: MobaMatchSimulation, moba_picks_bans: PicksBans
) -> None:
    pass


# TODO: write the proper picks and bans and make it work here
# def test_simulate_moba_game(moba_match_simulation: MobaMatchSimulation, moba_picks_bans: PicksBans) -> None:
#     moba_picks_bans.picks_bans()
#     moba_match_simulation.run()
#     assert moba_match_simulation.is_running is False
#     assert moba_match_simulation.is_match_over is True
#     assert moba_match_simulation.get_winning_team() is not None
#     assert moba_match_simulation.get_winning_team().nexus == 1