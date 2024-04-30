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

import pytest

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


def test_get_team_definition_from_single_region_def(
    db: DB,
    tmp_path: Path,
) -> None:
    region_mock_files = [
        {
            "id": "testregion",
            "name": "TestRegion",
            "short_name": "TR",
            "filename": "testregion/teams.json",
        },
    ]
    mock_team_file = tmp_path / "testregion" / "teams.json"
    mock_team_file.mkdir(parents=True)
    expected_mock_files = [
        {
            "id": "testregion",
            "name": "TestRegion",
            "short_name": "TR",
            "filename": mock_team_file.absolute().as_posix(),
        },
    ]
    assert (
        db.get_moba_region_definitions(region_mock_files, tmp_path)
        == expected_mock_files
    )


def test_get_team_definition_files_from_region_defs(
    db: DB,
    tmp_path: Path,
) -> None:
    region_mock_files = [
        {
            "id": "testregion",
            "name": "TestRegion",
            "short_name": "TR",
            "filename": "testregion/teams.json",
        },
        {
            "id": "testregion2",
            "name": "TestRegion2",
            "short_name": "TR2",
            "filename": "testregion2/teams2.json",
        },
    ]
    mock_team_file1 = tmp_path / "testregion" / "teams.json"
    mock_team_file2 = tmp_path / "testregion2" / "teams2.json"
    mock_team_file1.mkdir(parents=True)
    mock_team_file2.mkdir(parents=True)
    expected_mock_files = [
        {
            "id": "testregion",
            "name": "TestRegion",
            "short_name": "TR",
            "filename": mock_team_file1.absolute().as_posix(),
        },
        {
            "id": "testregion2",
            "name": "TestRegion2",
            "short_name": "TR2",
            "filename": mock_team_file2.absolute().as_posix(),
        },
    ]
    assert (
        db.get_moba_region_definitions(region_mock_files, tmp_path)
        == expected_mock_files
    )


def test_get_region_defs_non_existant_file(
    db: DB,
    tmp_path: Path,
) -> None:
    region_mock_files = [
        {
            "id": "testregion",
            "name": "TestRegion",
            "short_name": "TR",
            "filename": "testregion/teams.json",
        },
        {
            "id": "testregion2",
            "name": "TestRegion2",
            "short_name": "TR2",
            "filename": "testregion2/teams2.json",
        },
    ]
    with pytest.raises(FileNotFoundError):
        db.get_moba_region_definitions(region_mock_files, tmp_path)
