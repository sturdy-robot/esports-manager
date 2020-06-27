import json
import os
import random

from src.resources.generator.get_names import get_br_first_names, get_kr_first_names, get_usa_first_names, \
    get_br_last_names, get_kr_last_names, get_usa_last_names, gen_nick_or_team_name

from ..utils import write_to_json


def get_players_nationalities() -> list:
    """
    Defines nationalities
    :return nationality: string
    """
    return [
        "br",
        "kr",
        "usa"
    ]


def generate_player(nationality: str) -> dict:
    """
    Generates player dictionary
    :param nationality: string
    :return player: dictionary
    """
    if nationality == "br":
        first_names = get_br_first_names()
        last_names = get_br_last_names()
    elif nationality == "usa":
        first_names = get_usa_first_names()
        last_names = get_usa_last_names()
    else:
        first_names = get_kr_first_names()
        last_names = get_kr_last_names()

    first_name = random.choice(first_names)
    last_name = random.choice(last_names)
    nick_name = gen_nick_or_team_name("nicknames.txt")

    skill = get_players_skills(nationality)
    skill = int(skill)

    return {
        "first_name": first_name,
        "last_name": last_name,
        "nick_name": nick_name,
        "nationality": nationality,
        "skill": skill
    }


def get_players_skills(nationality: str) -> int:
    """
    Randomly generates players skills according to their nationality
    :param nationality: string
    :return skill: int
    """
    if nationality == "br":
        mu = 50
        sigma = 20
    elif nationality == "kr":
        mu = 80
        sigma = 10
    elif nationality == "usa":
        mu = 65
        sigma = 20
    else:
        mu = 50
        sigma = 10
        
    skill = random.gauss(mu, sigma)

    # Players' skill will follow the 30 < skill < 90 interval
    if skill > 93:
        skill = 90
    elif skill < 30:
        skill = 30
    
    return skill


def generate_player_list() -> list:
    """
    Generates each player and adds to a list
    :return player_list:
    """
    players_list = []
    nationalities = get_players_nationalities()
    
    for i in range(get_num_players()):
        nationality = random.choice(nationalities)
        player = generate_player(nationality)
        player["id"] = i
        players_list.append(player)
    
    return players_list


def get_num_players():
    return 200


def generate() -> None:
    """
    Runs the entire thing
    """
    players = generate_player_list()
    write_to_json(players, 'players.json')