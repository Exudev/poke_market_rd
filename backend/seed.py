import psycopg2
import random
import uuid

# Connect to the database
conn = psycopg2.connect("dbname=poke_market user=exudev")
cur = conn.cursor()

# Get a valid user_id
cur.execute("SELECT id FROM users LIMIT 1")
user = cur.fetchone()

if not user:
    print("No users found. Please create a user first.")
    exit(1)

user_id = user[0]

pokemon_names = ["Charizard", "Pikachu", "Blastoise", "Venusaur", "Mewtwo", "Mew", "Lugia", "Ho-Oh", "Rayquaza", "Gengar", "Umbreon", "Espeon", "Tyranitar", "Dragonite", "Snorlax", "Gyarados", "Arcanine", "Alakazam", "Machamp", "Golem", "Lapras", "Eevee", "Jolteon", "Flareon", "Vaporeon", "Articuno", "Zapdos", "Moltres"]
conditions = ["Pristine (PSA 10)", "Near Mint (NM)", "Lightly Played (LP)", "Moderately Played (MP)", "Heavily Played (HP)", "Damaged"]

series_sets = {
    'Sword & Shield': [
        'Crown Zenith',
        'Sword & Shield—Silver Tempest',
        'Sword & Shield—Lost Origin',
        'Pokémon TCG: Pokémon GO'
    ],
    'Scarlet & Violet': [
        'Scarlet & Violet—Black Bolt',
        'Scarlet & Violet—White Flare',
        'Scarlet & Violet—Destined Rivals',
        'Scarlet & Violet—Journey Together'
    ],
    'Mega Evolution': [
        '30th Celebration',
        'Mega Evolution—Pitch Black',
        'Mega Evolution—Chaos Rising',
        'Mega Evolution—Perfect Order'
    ],
    'Vintage': [
        'Base Set',
        'Jungle',
        'Fossil',
        'Team Rocket'
    ]
}

currencies = ['USD', 'DOP']

for i in range(100):
    name = random.choice(pokemon_names)
    condition = random.choice(conditions)
    series = random.choice(list(series_sets.keys()))
    set_name = random.choice(series_sets[series])
    currency = random.choice(currencies)
    
    price_cents = random.randint(100, 100000) # $1.00 to $1000.00
    
    cur.execute("""
        INSERT INTO posts (id, user_id, pokemon_name, series, set_name, condition, price_cents, currency, description)
        VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s)
    """, (
        str(uuid.uuid4()),
        user_id,
        f"{name} VMAX",
        series,
        set_name,
        condition,
        price_cents,
        currency,
        f"Awesome {name} card in {condition} condition."
    ))

conn.commit()
cur.close()
conn.close()

print("Inserted 100 fake cards!")
