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
from pathlib import Path

from esm.core.db import DB


def test_generate_champion_files(
    db: DB, mock_champion_defs: list[dict[str, str | int | float]], tmp_path: Path
) -> None:
    moba_champions_path = tmp_path / "moba_champions.json"
    champions = db.generate_moba_champions(mock_champion_defs)
    expected_champions = db.serialize_champions(champions)
    db.generate_moba_file(moba_champions_path, expected_champions)
    assert moba_champions_path.exists()
    with moba_champions_path.open("r") as fp:
        actual_champions = json.load(fp)
    assert actual_champions == expected_champions


def test_generate_team_files(
    db: DB,
    mock_champion_defs: list[dict[str, str | int | float]],
    mock_team_definitions: list[dict[str, str | int]],
    names_file: list[dict[str, dict[str, str | int]]],
    tmp_path: Path,
) -> None:
    teams_filepath = tmp_path / "moba_teams.json"
    champions = db.generate_moba_champions(mock_champion_defs)
    teams = db.generate_moba_teams(names_file, champions, mock_team_definitions)
    serialized_teams = db.serialize_teams(teams)
    db.generate_moba_file(teams_filepath, serialized_teams)
    assert teams_filepath.exists()
    with teams_filepath.open("r") as fp:
        actual_teams = json.load(fp)
    assert actual_teams == serialized_teams
