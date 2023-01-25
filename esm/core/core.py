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
import os
import logging
from textwrap import dedent

from .settings import Settings
from .db import DB
from ..definitions import DEBUG, LOG_FILE
from .game_manager import GameManager


class AmountPlayersError(Exception):
    pass


class Core:
    """
    Core module deals with core functions of the game
    """
    def __init__(self):
        self.settings = Settings()
        self.settings.load_config_file()
        self.db = DB(self.settings)
        self.game_manager = GameManager(self.settings)

    def initialize_logging(self):
        logging.basicConfig(
            filename=LOG_FILE,
            encoding="utf-8",
            format="%(levelname)s %(asctime)s: %(message)s",
            datefmt="%m/%d/%Y %I:%M:%S %p ",
        )
        logger = logging.getLogger(__name__)

        if DEBUG:
            logging.basicConfig(level=logging.DEBUG)
            logger.debug()
        else:
            logging.basicConfig(level=logging.ERROR)


    @property
    def amount_players(self):
        return self.settings.amount_players

    @amount_players.setter
    def amount_players(self, value):
        try:
            self.check_player_amount()
        except AmountPlayersError:
            self.settings.amount_players = 50
        else:
            self.settings.amount_players = value

    def check_player_amount(self):
        if self.amount_players > 300 or self.amount_players < 50:
            error_message = dedent("""
            Number of players is not supported! Ranges from 50 to 300 players! Defaulting to 50.
            """).strip()
            raise AmountPlayersError(error_message)

        if self.amount_players % 5 != 0:
            error_message = dedent("""
            Number of players is not supported! Number should be a multiple of 5!
            """)
            raise AmountPlayersError(error_message)

    def check_files(self) -> None:
        if not os.path.exists(self.settings.champions_file) or not os.path.exists(
                self.settings.players_file) or not os.path.exists(self.settings.teams_file):
            raise FileNotFoundError
