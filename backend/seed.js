const { Client } = require('pg');
const crypto = require('crypto');

async function seed() {
    const client = new Client({
        connectionString: 'postgresql://localhost/poke_market'
    });
    
    await client.connect();
    
    const res = await client.query('SELECT id FROM users LIMIT 1');
    if (res.rows.length === 0) {
        console.error("No users found.");
        process.exit(1);
    }
    const userId = res.rows[0].id;
    
    const pokemonNames = ["Charizard", "Pikachu", "Blastoise", "Venusaur", "Mewtwo", "Mew", "Lugia", "Ho-Oh", "Rayquaza", "Gengar", "Umbreon", "Espeon", "Tyranitar", "Dragonite", "Snorlax", "Gyarados", "Arcanine", "Alakazam"];
    const conditions = ["Pristine (PSA 10)", "Near Mint (NM)", "Lightly Played (LP)", "Moderately Played (MP)", "Heavily Played (HP)", "Damaged"];
    const seriesSets = {
        'Sword & Shield': ['Crown Zenith', 'Sword & Shield—Silver Tempest', 'Sword & Shield—Lost Origin', 'Pokémon TCG: Pokémon GO'],
        'Scarlet & Violet': ['Scarlet & Violet—Black Bolt', 'Scarlet & Violet—White Flare', 'Scarlet & Violet—Destined Rivals', 'Scarlet & Violet—Journey Together'],
        'Mega Evolution': ['30th Celebration', 'Mega Evolution—Pitch Black', 'Mega Evolution—Chaos Rising', 'Mega Evolution—Perfect Order'],
        'Vintage': ['Base Set', 'Jungle', 'Fossil', 'Team Rocket']
    };
    const currencies = ['USD', 'DOP'];
    const seriesKeys = Object.keys(seriesSets);

    for (let i = 0; i < 100; i++) {
        const name = pokemonNames[Math.floor(Math.random() * pokemonNames.length)];
        const condition = conditions[Math.floor(Math.random() * conditions.length)];
        const series = seriesKeys[Math.floor(Math.random() * seriesKeys.length)];
        const sets = seriesSets[series];
        const setName = sets[Math.floor(Math.random() * sets.length)];
        const currency = currencies[Math.floor(Math.random() * currencies.length)];
        const priceCents = Math.floor(Math.random() * 100000) + 100; // 1.00 to 1000.00
        
        await client.query(`
            INSERT INTO posts (id, user_id, pokemon_name, series, set_name, condition, price_cents, currency, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        `, [
            crypto.randomUUID(),
            userId,
            `${name} VMAX`,
            series,
            setName,
            condition,
            priceCents,
            currency,
            `Awesome ${name} card in ${condition} condition.`
        ]);
    }
    
    console.log("Inserted 100 fake cards!");
    await client.end();
}

seed().catch(console.error);
