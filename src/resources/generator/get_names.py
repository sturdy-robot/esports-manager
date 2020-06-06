import random


# TODO: change these functions to a single one
# TODO: create a file with names from more nations

def get_usa_first_names():
    first_names = [
        "Alexander",
        "Aaron",
        "Adam",
        "Zachary",
        "Arthur",
        "Austin",
        "Joe",
        "Noah",
        "Sean",
        "Kyle",
        "John",
        "Peter",
        "David",
        "Anthony",
        "Carl",
        "William",
        "Ted",
        "Carlson",
        "Michael",
        "James",
        "Robert",
        "Richard",
        "Thomas",
        "Charles",
        "Christopher",
        "Daniel",
        "Matthew",
        "Donald",
        "Paul",
        "George",
        "Joshua",
        "Edward",
        "Brian",
        "Timothy",
        "Kevin",
        "Jason",
        "Gary",
        "Jacob",
        "Eric",
        "Nicholas",
        "Stephen",
        "Steve",
        "Steven",
        "Jonathan",
        "Brandon",
        "Justin",
        "Frank",
        "Benjamin"
    ]

    return first_names


def get_usa_last_names():
    last_names = [
        "Johnson",
        "Williams",
        "Jones",
        "Smith",
        "Brown",
        "Davis",
        "Wilson",
        "Moore",
        "Taylor",
        "Anderson",
        "Jackson",
        "Harris",
        "White",
        "Martin",
        "Garcia",
        "Martinez",
        "Clark",
        "Rodriguez",
        "Lewis",
        "Walker",
        "Lee",
        "Lopez",
        "Hill",
        "Scott",
        "Green",
        "Adams",
        "Baker",
        "Rogers",
        "Bell",
        "Henderson",
        "Washington",
        "Butler",
        "Bryant",
        "Freeman",
        "Mason",
        "Wheeler",
        "Chapman",
        "Morrison",
        "Lynch",
        "Gilbert",
        "Fowler",
        "Wade",
        "Stevenson",
        "Rogers"
    ]

    return last_names


def get_br_first_names():
    first_names = [
        "Alex",
        "André",
        "Antônio",
        "Anderson",
        "Diego",
        "Carlos",
        "Adriano",
        "Aldo",
        "Augusto",
        "César",
        "Cléber",
        "Pedro",
        "Gabriel",
        "Rafael",
        "Francisco",
        "Felipe",
        "Fábio",
        "Fabiano",
        "Bernardo",
        "Alan",
        "Fernando",
        "Lucas",
        "Fabrício",
        "Henrique",
        "Júlio",
        "Marcos",
        "Daniel",
        "Bruno",
        "Eduardo",
        "Luís Felipe",
        "Luís Eduardo",
        "Luís Adriano",
        "Luiz",
        "Paulo",
        "Gustavo",
        "Mateus",
        "Mario",
        "Mariano",
        "Márcio",
        "Tiago",
        "Thiago",
        "Eric",
        "Nicolas",
        "Nicolau",
        "Marcelo",
        "José",
        "João",
        "Rodrigo",
        "Roberto"
    ]

    return first_names


def get_br_last_names():
    last_names = [
        "Abreu",
        "Silva",
        "Carvalho",
        "Alencar",
        "Alves",
        "Aparecido",
        "Barbosa",
        "Barroso",
        "Batista",
        "Campos",
        "Castello",
        "Chagas",
        "Kardec",
        "Couto",
        "Coutinho",
        "Espínola",
        "Castro",
        "Oliveira",
        "de Oliveira",
        "Azevedo",
        "Guimarães",
        "Gonçalves",
        "Feitosa",
        "Feliciano",
        "Bolsonaro",
        "Pires",
        "Pinto",
        "Rodrigues",
        "Sales",
        "Santana",
        "Santos",
        "dos Santos",
        "Vieira",
        "Xavier",
        "do Valle",
        "Tavares",
        "Toffoli",
        "Souza",
        "de Souza",
        "Soares",
        "Faria",
        "Silveira",
        "da Silva",
        "Novaes",
        "Torres",
        "Neto",
        "Nazário",
        "Menezes",
        "Mendonça",
        "Miranda",
        "Neves"
    ]

    return last_names


def get_kr_first_names():
    first_names = [
        "Do-yun",
        "Dong-ha",
        "Ha-joon",
        "Hyuk-kyu",
        "Won-seok",
        "Jong",
        "Ji-woo",
        "Ji-soo",
        "Jae",
        "Ji-ho",
        "Ji-hoon",
        "Jin-seong",
        "Joon-woo",
        "Ju-won",
        "Myeong",
        "Myung",
        "Min-ju",
        "Min-seung",
        "Seung",
        "Sang-hyeok",
        "Suk",
        "Sung",
        "Sung-ho",
        "Seo-jun",
        "Si-woo",
        "Ye-jun",
        "Yu-jun",
        "Joo-won",
        "Jun-seo",
        "U-jin",
        "Tae-min",
        "Young",
        "Young-ho",
        "Young-soo",
        "Yun-seo"
    ]

    return first_names


def get_kr_last_names():
    last_names = [
        "An",
        "Lee",
        "Park",
        "Kim",
        "Moon",
        "Heo",
        "Cho",
        "Kang",
        "Jung",
        "Song"
    ]

    return last_names


def gen_nick_or_team_name(filename):
    with open(filename, "r") as fp:
        names = fp.read().splitlines()

    return random.choice(names)
