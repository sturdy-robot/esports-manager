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

import PySimpleGUI as sg
from ..gui_components import *
from src.resources.utils import find_file
from .layoutinterface import LayoutInterface


class MainScreenLayout(LayoutInterface):
    def __init__(self, controller):
        super().__init__(controller)
        self.lay = self.layout()
        self.col = self.column()

    def column(self):
        return sg.Column(self.lay,
                         key='main_screen',
                         element_justification="center"
                         )

    def layout(self):
        """
        Defines the main screen. This screen shows the initial options to play a new game, load game,
        use the Database Editor, or exit the game.
        """
        logo_path = find_file('esportsmanager.png')

        button_pad = (0, 10)
        button_size = (20, 2)

        return [
            [sg.Image(logo_path, pad=(50, 0))],
            [esm_button('Debug Game Mode',
                        key='main_debug_btn',
                        pad=button_pad,
                        size=button_size
                        )],
            [esm_button('New Game',
                        key='main_newgame_btn',
                        pad=button_pad,
                        size=button_size
                        )],
            [esm_button('Load Game',
                        key='main_loadgame_btn',
                        pad=button_pad,
                        size=button_size
                        )],
            [esm_button('Settings',
                        key='main_settings_btn',
                        pad=button_pad,
                        size=button_size
                        )],
            [esm_button('Exit',
                        key='main_exit_btn',
                        pad=button_pad,
                        size=button_size
                        )],
        ]

    def update(self, event, values, make_screen, *args, **kwargs):
        if event == 'main_debug_btn':
            make_screen('main_screen', 'debug_game_mode_screen')

        elif event == 'main_newgame_btn':
            make_screen('main_screen', 'new_game_screen')

        elif event == 'main_loadgame_btn':
            make_screen('main_screen', 'load_game_screen')

        elif event == 'main_settings_btn':
            make_screen('main_screen', 'settings_screen')