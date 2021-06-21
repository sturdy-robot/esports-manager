#      eSports Manager - free and open source eSports Management game
#      Copyright (C) 2020-2021  Pedrenrique G. Guimarães
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

from esm.definitions import ROOT_DIR, RES_DIR
from esm.resources.utils import write_to_json, get_list_from_file, load_list_from_json
from esm.core.esports.moba.champion import Champion


class ChampionGenerator:
    def __init__(
        self,
        name: str = None,
        skill: int = None,
        lane: str = None,
        champion_dict: dict = None,
        file_name: str = "champions.json",
        champion_obj: Champion = None,
        champion_names: list = None,
    ):
        self.champion_id = 0
        self.name = name
        self.skill = skill
        self.lane = lane
        self.file_name = file_name
        self.champion_dict = champion_dict
        self.champion_obj = champion_obj
        self.champion_names = champion_names
        self.champions_list = []
        self.champions_obj = []

    def generate_champion_id(self) -> None:
        """
        Generates champion UUID
        """
        self.champion_id = uuid.uuid4().int

    def get_champion_names(self) -> None:
        """
        List used to generate champions, all these names will be used to generate champions

        TODO: limit the number of champion names to generate
        TODO: This should also be used to generate champions in different patches
        """
        self.champion_names = get_list_from_file("champions.txt")

    def generate_champion_lanes(self) -> None:
        pass

    def generate_champion_skill(self) -> None:
        """
        Generates Champion Skills. Perhaps it should also follow a Normal Distribution?

        Also, easter egg for Teemo
        """
        if self.name == "TEEMO":
            self.skill = random.randint(0, 50)
        else:
            self.skill = random.randint(50, 99)

    def generate_champion_dict(self) -> None:
        """
        Generates the champion dictionary
        """
        self.champion_dict = {
            "name": self.name,
            "id": self.champion_id,
            "skill": self.skill,
        }

    def generate_champion_obj(self) -> None:
        """
        Generates the champion object based on the Champion class
        """
        self.champion_obj = Champion(self.champion_id, self.name, self.skill)

    def create_champions_list(self) -> None:
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
            self.generate_champion_dict()
            self.generate_champion_obj()
            self.champions_list.append(self.champion_dict)
            self.champions_obj.append(self.champion_obj)

    def get_champions(self, folder: Union[str, Path] = ROOT_DIR) -> None:
        """
        Retrieves champions from the list of champions. Perhaps at the point when we implement
        a database this can replace the load_list_from_json function.
        """
        self.champions_list = load_list_from_json(self.file_name, folder)
        self.champion_obj = []
        for champion in self.champions_list:
            self.name = champion["name"]
            self.champion_id = champion["id"]
            self.skill = champion["skill"]
            self.generate_champion_obj()
            self.champions_obj.append(self.champion_obj)

    def generate_file(
        self,
        folder: Union[str, Path] = ROOT_DIR,
        res_folder: Union[str, Path] = RES_DIR,
    ) -> None:
        """
        Generates the champion file
        """
        write_to_json(self.champions_list, self.file_name, folder, res_folder)