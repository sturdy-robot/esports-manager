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
import datetime
import random
import uuid
from typing import Optional

from esm.core.esports.default_player_nick_names import get_default_player_nick_names
from esm.core.esports.generator import GeneratorInterface
from esm.core.esports.rts.rts_player import RTSPlayer, RTSPlayerAttributes, RTSRace
from esm.core.utils import get_nations


class RTSPlayerGeneratorError(Exception):
    pass


class RTSPlayerAttributesGenerator(GeneratorInterface):

    def generate(self, *args, **kwargs) -> RTSPlayerAttributes:
        return RTSPlayerAttributes(
            attack=random.randint(0, 100),
            defense=random.randint(0, 100),
            speed=random.randint(0, 100),
            strategy=random.randint(0, 100),
            micro=random.randint(0, 100),
            macro=random.randint(0, 100),
        )


class RTSPlayerGenerator(GeneratorInterface):
    def __init__(
        self,
        names: list[dict[str, dict[str, str | int]]],
        today: datetime.date = datetime.date.today(),
        min_age: int = 16,
        max_age: int = 25,
    ):
        self.nationalities = get_nations(names)
        self.nick_names = get_default_player_nick_names()

        if min_age > max_age:
            raise RTSPlayerGeneratorError(
                "Minimum age cannot be higher than maximum age!"
            )

        self.attribute_gen = RTSPlayerAttributesGenerator()
        self.min_age = min_age
        self.max_age = max_age
        self.td = today  # used to calculate the date of birth. Varies according to the current season calendar
        self.names = names

    @staticmethod
    def generate_id() -> uuid.UUID:
        return uuid.uuid4()

    def get_nationality(self) -> str:
        """
        Defines players nationalities
        """
        return random.choice(self.nationalities)

    def generate_dob(self) -> datetime.date:
        """
        Generates the player's date of birth
        """
        year = datetime.timedelta(
            seconds=31556952
        )  # definition of a Gregorian calendar date

        max_age = (
            self.max_age * year
        )  # players should be a max of self.max_age years old
        min_age = (
            self.min_age * year
        )  # players can't be less than self.min_age years old
        min_year = self.td - max_age  # minimum date for birthday
        max_year = self.td - min_age  # max date for birthday

        days_interval = max_year - min_year
        rand_date = random.randrange(
            days_interval.days
        )  # chooses a random date from the max days interval
        return min_year + datetime.timedelta(days=rand_date)  # assigns date of birth

    def generate_first_name(self, nationality: str) -> str:
        """
        Generates the player's real name
        """
        for name_dict in self.names:
            if name_dict["region"] == nationality:
                return random.choice(name_dict["male"])

        raise RTSPlayerGeneratorError("Invalid region!")

    def generate_last_name(self, nationality: str) -> str:
        """
        Generates the player's real name
        """
        for name_dict in self.names:
            if name_dict["region"] == nationality:
                return random.choice(name_dict["surnames"])

        raise RTSPlayerGeneratorError("Invalid region!")

    def generate_nick(self) -> str:
        """
        Generates the player's nickname
        """
        return random.choice(self.nick_names)

    def generate_multipliers(self, main_race: RTSRace) -> dict[RTSRace, float]:
        return {
            race: random.randrange(55, 100) / 100 if race != main_race else 1
            for race in list(RTSRace)
        }

    def generate_attributes(self) -> RTSPlayerAttributes:
        return self.attribute_gen.generate()

    def generate(
        self, race: Optional[RTSRace] = None, nationality: Optional[str] = None
    ) -> RTSPlayer:
        if nationality is None:
            nationality = self.get_nationality()

        if race is None:
            race = random.choice(list(RTSRace))

        first_name = self.generate_first_name(nationality)
        last_name = self.generate_last_name(nationality)

        return RTSPlayer(
            self.generate_id(),
            nationality,
            first_name,
            last_name,
            self.generate_dob(),
            self.generate_nick(),
            self.generate_attributes(),
            self.generate_multipliers(race),
        )
