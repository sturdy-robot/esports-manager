import json
import random
from math import floor

from .get_names import gen_nick_or_team_name
from ..utils import write_to_json, load_list_from_json


# TODO: let teams have a bigger roster than 5 players
def choose_five_players(players: list) -> list:
    chosen_players_id = []

    for _ in range(5):
        player = random.choice(players)
        players.remove(player)
        chosen_players_id.append(player["id"])

    return chosen_players_id


def generate_each_team(players: list) -> dict:
    team_name = gen_nick_or_team_name('team_names.txt')
    roster_id = choose_five_players(players)

    return {"name": team_name, "roster_id": roster_id}


def generate_teams(players: list) -> list:
    num_teams = floor(int(len(players) / 5))

    teams = []
    for i in range(num_teams):
        team = generate_each_team(players)
        team["id"] = i
        teams.append(team)

    return teams


def run_generation() -> None:
    players = load_list_from_json('players.json')
    teams = generate_teams(players)
    write_to_json(teams, 'teams.json')