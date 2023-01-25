#      eSports Manager - free and open source eSports Management game
#      Copyright (C) 2020-2023  Pedrenrique G. Guimarães
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

import random
import uuid
from pathlib import Path
from typing import Union

from esm.core.esports.moba.champion import Champion
from esm.core.esports.moba.generator.default_champion_names import get_default_champion_names
from esm.core.utils import write_to_file, load_list_from_file
from esm.definitions import CHAMPIONS_FILE


class ChampionGenerator:
    def __init__(
            self,
            name: str = None,
            skill: int = None,
            lane: str = None,
            file_name: Union[str, Path] = CHAMPIONS_FILE,
            champion_obj: Champion = None,
            champion_names: list = None,
    ):
        self.champion_id = None
        self.name = name
        self.skill = skill
        self.lane = lane
        self.file_name = file_name
        self.champion_obj = champion_obj
        self.champion_names = champion_names
        self.champions = []

    def generate_champion_id(self) -> None:
        """
        Generates champion UUID
        """
        self.champion_id = uuid.uuid4()

    def get_champion_names(self) -> None:
        """
        List used to generate champions, all these names will be used to generate champions

        TODO: limit the number of champion names to generate
        TODO: This should also be used to generate champions in different patches
        """
        self.champion_names = get_default_champion_names()

    def generate_champion_lanes(self) -> None:
        pass

    def generate_champion_skill(self) -> None:
        """
        Generates Champion Skills.

        Also, Teemo is never a good champion.
        """
        self.skill = 30 if self.name == "TEEMO" else random.gauss(55, 20)
        self.skill = min(self.skill, 90)
        self.skill = max(self.skill, 30)
        # converting skill to int
        self.skill = int(self.skill)

    def generate_champion_obj(self) -> None:
        """
        Generates the champion object based on the Champion class
        """
        self.champion_obj = Champion(self.champion_id, self.name, self.skill)

    def generate_champions(self) -> None:
        """
        Creates the list of champions according to the names.
        Essentially this is the champion generation method.
        """
        if not self.champion_names:
            self.get_champion_names()
        for name in self.champion_names:
            self.generate_champion_id()
            self.name = name
            self.generate_champion_skill()
            self.generate_champion_obj()
            self.champions.append(self.champion_obj)

    def get_champions(self) -> None:
        """
        Retrieves champions from the list of champions. Perhaps at the point when we implement
        a database this can replace the load_list_from_json function.
        """
        champions_list = load_list_from_file(self.file_name)
        self.champions = []
        for champion in champions_list:
            self.name = champion["name"]
            self.champion_id = uuid.UUID(champion["id"])
            self.skill = champion["skill"]
            self.generate_champion_obj()
            self.champions.append(self.champion_obj)

    def get_champion_by_id(self, champ_id, ch_list) -> Union[Champion, None]:
        self.champions = ch_list

        return next((champion for champion in self.champions if champ_id == champion.champion_id), None)

    def get_from_data_file(self, data: list, only_dict: bool = False):
        champions_list = data.copy()
        if only_dict:
            self.champions = [Champion.get_from_dict(champion) for champion in champions_list]
