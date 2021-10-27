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

import os
import json
import cbor2
from typing import Union
from pathlib import Path

from esm.definitions import ROOT_DIR, DB_DIR, TEAMS_FILE


def write_to_file(
        contents: list,
        filename: Union[str, Path],
        folder: Union[str, Path] = ROOT_DIR,
        res_folder: Union[str, Path] = DB_DIR,
) -> None:
    file = filename
    try:
        file = find_file(filename, folder)
    except FileNotFoundError:
        # Maybe file creation should be in a separate function?
        file = os.path.join(res_folder, filename)  # prevents hard-coding "/" or "\"
    finally:
        if not os.path.exists(res_folder):
            os.makedirs(res_folder)
        filename = str(filename)
        if filename.endswith(".json"):
            with open(file, "w") as fp:
                json.dump(contents, fp, sort_keys=True, indent=4)
        elif filename.endswith(".cbor"):
            with open(file, "wb") as fp:
                cbor2.dump(contents, fp)


def find_file(filename: str, folder: Union[str, Path] = ROOT_DIR) -> str:
    """
    This function is used to find files used by the project. It receives the
    folder and searches from there. It removes the need to hard-code
    certain file paths.

    It was added because tests were failing outside of PyCharm, so running tests
    from terminal or VS Code was producing tons of errors. So I had to refactor
    everything, come up with ways that tests would work in all platforms, and that's
    what I ended up with.

    I hope this won't affect performance that much, and maybe in the future more
    elegant solutions will come out. I added the folder parameter to possibly
    prevent some performance issues that might arise with the expansion of this project.
    It's still unclear if it hits performance that much, but it's better safe than sorry.

    :param folder: folder to start searching for the file
    :param filename:
    :return:
    """
    for root, _, files in os.walk(folder):
        if filename in files:
            return os.path.join(root, filename)
    else:
        raise FileNotFoundError("File not found!")


def get_list_from_file(filename: Union[str, Path]) -> list:
    file = find_file(filename)

    with open(file, "r", encoding="utf-8") as fp:
        lst = fp.read().splitlines()

    return lst


def get_from_file(file_name: Union[str, Path]) -> list:
    """
    General function used to read a JSON/CBOR file, extracting its data to a dictionary/list
    :param file_name:
    :return:
    """
    if file_name.endswith(".json"):
        with open(file_name, "r", encoding="utf-8") as fp:
            dictionary = json.load(fp)
    else:
        with open(file_name, "rb") as fp:
            dictionary = cbor2.load(fp)

    return dictionary


def load_list_from_file(filepath: Union[str, Path], folder: Union[str, Path] = ROOT_DIR) -> list:
    """
    Reads a specified file (champions, player or team json) and
    returns the list from that file
    :param filepath:
    :param folder:
    :return:
    """
    try:
        file = find_file(filepath, folder)
    except FileNotFoundError as e:
        print("File was not found!")
        print("Error occurred: {}".format(e.errno))
    else:
        return get_from_file(file)


def get_key_from_json(key: str = "name", file: Union[str, Path] = TEAMS_FILE) -> list:
    """
    Gets a key from a json file. By default it is used by the GUI to get
    names from the file teams.json, but we can repurpose that for other
    files too, such as get player names, champion names, etc...
    :param key:
    :param file:
    :return:
    """
    return [obj[key] for obj in load_list_from_file(file)]