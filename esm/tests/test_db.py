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
import json

from esm.core.db import DB


def test_generate_champion_files(db: DB) -> None:
    champions = db.generate_moba_champions()
    expected_champions = db.serialize_champions(champions)
    db.generate_moba_champions_file(expected_champions)
    assert db.settings.moba_champions.exists()
    with open(db.settings.moba_champions) as fp:
        actual_champions = json.load(fp)
    assert actual_champions == expected_champions


def test_generate_team_files(db: DB) -> None:
    champions = db.generate_moba_champions()
    teams, _ = db.generate_moba_teams(champions)
    serialized_teams = db.serialize_teams(teams)
    db.generate_moba_teams_file(serialized_teams)
    assert db.settings.moba_teams.exists()
    with open(db.settings.moba_teams) as fp:
        actual_teams = json.load(fp)
    assert actual_teams == serialized_teams
