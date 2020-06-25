import json
import os
import random
from math import floor

from src.resources.generator.generate_players import NUM_PLAYERS, JSON_FILE, THIS_FOLDER, generate_file
from src.resources.generator.get_names import gen_nick_or_team_name


def get_players() -> list:
    with open(JSON_FILE, "r") as fp:
        players = json.load(fp)

    return players


# TODO: let teams have a bigger roster than 5 players
def choose_five_players(players: list) -> list:
    chosen_players_id = []

    for _ in range(5):
        player = random.choice(players)
        players.remove(player)
        chosen_players_id.append(player["id"])

    return chosen_players_id


def generate_each_team(players: list) -> dict:
    team_name = gen_nick_or_team_name("team_names.txt")
    roster_id = choose_five_players(players)

    return {"name": team_name, "roster_id": roster_id}


def generate_teams(players: list) -> list:
    num_teams = floor(int(NUM_PLAYERS / 5))

    teams = []
    for i in range(num_teams):
        team = generate_each_team(players)
        team["id"] = i
        teams.append(team)

    return teams


def run_generation() -> None:
    team_file = os.path.join(THIS_FOLDER, '../src/resources/db/teams.json')
    players = get_players()
    teams = generate_teams(players)
    generate_file(teams, team_file)
