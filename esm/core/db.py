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
import os
import uuid
from pathlib import Path

from .esports.moba.champion import Champion
from .esports.moba.generator import ChampionGenerator, TeamGenerator
from .esports.moba.mobaplayer import MobaPlayer
from .esports.moba.mobaregion import MobaRegion
from .esports.moba.mobateam import MobaTeam
from .gamestate import GameState


class DB:
    def generate_moba_champions(
        self, champion_defs: list[dict[str, str | int]]
    ) -> list[Champion]:
        champion_gen = ChampionGenerator()
        return [champion_gen.generate(champion_def) for champion_def in champion_defs]

    def generate_moba_teams(
        self,
        player_names: list[dict[str, dict[str, str | int]]],
        champions_list: list[Champion],
        teams_def: list[dict[str, str | int]],
    ) -> list[MobaTeam]:
        team_gen = TeamGenerator(champions_list, player_names)
        return [team_gen.generate(team_def) for team_def in teams_def]

    def get_moba_region_definitions(
        self, regions: list[dict[str, str]], directory: Path
    ) -> list[dict[str, str]]:
        for region in regions:
            filename = region["filename"]
            path = os.path.join(directory, filename)
            if not os.path.exists(path):
                raise FileNotFoundError(f"Could not find the filename: {path}")
            region["filename"] = path

        return regions

    def generate_moba_region(
        self, region: dict[str, str], teams: list[MobaTeam]
    ) -> MobaRegion:
        return MobaRegion(
            uuid.uuid4(),
            region["id"],
            region["name"],
            region["short_name"],
            teams,
        )

    def extract_teams_from_region(
        self,
        region: dict[str, str],
        champions: list[Champion],
        player_names: list[dict[str, dict[str, str | int]]],
    ) -> list[MobaTeam]:
        team_file = Path(region["filename"])
        with team_file.open("r", encoding="utf-8") as fp:
            teams_list = json.load(fp)
        return self.generate_moba_teams(player_names, champions, teams_list)

    def extract_regions_from_region_file(
        self,
        regions: list[dict[str, str]],
        champions: list[Champion],
        player_names: list[dict[str, dict[str, str | int]]],
    ) -> list[MobaRegion]:
        return [
            self.generate_moba_region(
                region, self.extract_teams_from_region(region, champions, player_names)
            )
            for region in regions
        ]

    @staticmethod
    def get_moba_players_from_teams(teams_list: list[MobaTeam]) -> list[MobaPlayer]:
        players_list = []
        for team in teams_list:
            for player in team.roster:
                players_list.append(player)

        return players_list

    @staticmethod
    def serialize_champions(
        champions_list: list[Champion],
    ) -> dict[str, dict[str, str | list[str]]]:
        return {
            champion.champion_id.hex: champion.serialize()
            for champion in champions_list
        }

    @staticmethod
    def serialize_teams(
        teams_list: list[MobaTeam],
    ) -> dict[str, dict[str, str | list[str]]]:
        return {team.team_id.hex: team.serialize() for team in teams_list}

    @staticmethod
    def serialize_players(
        players_list: list[MobaPlayer],
    ) -> dict[str, dict[str, str | list[str]]]:
        return {player.player_id.hex: player.serialize() for player in players_list}

    @staticmethod
    def serialize_regions(regions_list: list[MobaRegion]) -> dict[str, dict]:
        return {region.region_id.hex: region.serialize() for region in regions_list}

    def generate_moba_file(
        self, filepath: Path, serialized_data: dict[str, dict[str, str | list[str]]]
    ) -> None:
        filepath.parent.mkdir(parents=True, exist_ok=True)
        with open(filepath, "w", encoding="utf-8") as fp:
            json.dump(serialized_data, fp, indent=4, ensure_ascii=False, sort_keys=True)

    def generate_moba_files(
        self,
        champions_file: Path,
        teams_file: Path,
        players_file: Path,
        regions_file: Path,
        champions_list: list[Champion],
        teams_list: list[MobaTeam],
        regions: list[MobaRegion],
        players_list: list[MobaPlayer],
    ) -> None:
        serialized_champions = self.serialize_champions(champions_list=champions_list)
        serialized_teams = self.serialize_teams(teams_list=teams_list)
        serialized_players = self.serialize_players(players_list=players_list)
        serialized_regions = self.serialize_regions(regions_list=regions)

        self.generate_moba_file(champions_file, serialized_champions)
        self.generate_moba_file(teams_file, serialized_teams)
        self.generate_moba_file(players_file, serialized_players)
        self.generate_moba_file(regions_file, serialized_regions)

    def load_moba_teams(self) -> list[MobaTeam]:
        pass

    def get_gamestate(self) -> GameState:
        pass

    def load_from_gamestate(self, gamestate: GameState):
        pass
