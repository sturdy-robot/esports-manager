import os
import json
import random

from ..utils import write_to_json


# THIS FILE HAS THE SOLE PURPOSE OF GENERATING A JSON FILE
# WITH CHAMPION NAMES AND RANDOM SKILL LEVELS

champion_names = [
    "AATROX",
    "AHRI",
    "AKALI",
    "ALISTAR",
    "AMUMU",
    "ANIVIA",
    "ANNIE",
    "APHELIOS",
    "ASHE",
    "AURELION SOL",
    "AZIR",
    "BARD",
    "BLITZCRANK",
    "BRAND",
    "BRAUM",
    "CAITLYN",
    "CAMILLE",
    "CASSIOPEIA",
    "CHO'GATH",
    "CORKI",
    "DARIUS",
    "DIANA",
    "DR. MUNDO",
    "DRAVEN",
    "EKKO",
    "ELISE",
    "EVELYNN",
    "EZREAL",
    "FIDDLESTICKS",
    "FIORA",
    "FIZZ",
    "GALIO",
    "GANGPLANK",
    "GAREN",
    "GNAR",
    "GRAGAS",
    "GRAVES",
    "HECARIM",
    "HEIMERDINGER",
    "ILLAOI",
    "IRELIA",
    "IVERN",
    "JANNA",
    "JARVAN IV",
    "JAX",
    "JAYCE",
    "JHIN",
    "JINX",
    "KAI'SA",
    "KALISTA",
    "KARMA",
    "KARTHUS",
    "KASSADIN",
    "KATARINA",
    "KAYLE",
    "KAYN",
    "KENNEN",
    "KHA'ZIX",
    "KINDRED",
    "KLED",
    "KOG'MAW",
    "LEBLANC",
    "LEE SIN",
    "LEONA",
    "LISSANDRA",
    "LUCIAN",
    "LULU",
    "LUX",
    "MALPHITE",
    "MALZAHAR",
    "MAOKAI",
    "MASTER YI",
    "MISS FORTUNE",
    "MORDEKAISER",
    "MORGANA",
    "NAMI",
    "NASUS",
    "NAUTILUS",
    "NEEKO",
    "NIDALEE",
    "NOCTURNE",
    "NUNU & WILLUMP",
    "OLAF",
    "ORIANNA",
    "ORNN",
    "PANTHEON",
    "POPPY",
    "PYKE",
    "QIYANA",
    "QUINN",
    "RAKAN",
    "RAMMUS",
    "REK'SAI",
    "RENEKTON",
    "RENGAR",
    "RIVEN",
    "RUMBLE",
    "RYZE",
    "SEJUANI",
    "SENNA",
    "SETT",
    "SHACO",
    "SHEN",
    "SHYVANA",
    "SINGED",
    "SION",
    "SIVIR",
    "SKARNER",
    "SONA",
    "SORAKA",
    "SWAIN",
    "SYLAS",
    "SYNDRA",
    "TAHM KENCH",
    "TALIYAH",
    "TALON",
    "TARIC",
    "TEEMO",
    "THRESH",
    "TRISTANA",
    "TRUNDLE",
    "TRYNDAMERE",
    "TWISTED FATE",
    "TWITCH",
    "UDYR",
    "URGOT",
    "VARUS",
    "VAYNE",
    "VEIGAR",
    "VEL'KOZ",
    "VI",
    "VIKTOR",
    "VLADIMIR",
    "VOLIBEAR",
    "WARWICK",
    "WUKONG",
    "XAYAH",
    "XERATH",
    "XIN ZHAO",
    "YASUO",
    "YORICK",
    "YUUMI",
    "ZAC",
    "ZED",
    "ZIGGS",
    "ZILEAN",
    "ZOE",
    "ZYRA"
]


def generate_champion_dict(champion_name: str, counter: int) -> dict:
    if champion_name == "TEEMO":
        skill = random.randint(0, 50)
    else:
        skill = random.randint(50, 99)

    return {"name": champion_name,
            "id": counter,
            "skill": skill
            }


def create_champions_list() -> list:
    list_champions = []
    counter = 0
    
    for champion_name in champion_names:
        champion = generate_champion_dict(champion_name, counter)
        list_champions.append(champion)
        counter += 1 

    return list_champions


if __name__ == '__main__':
    list_of_champions = create_champions_list()
    write_to_json(list_of_champions, 'champions.json')