export const pokemonSeriesSets: Record<string, string[]> = {
    'Mega Evolution': [
        'Mega Evolution Base', 'Phantasmal Flames', 'Ascended Heroes', 'Perfect Order', 
        'Chaos Rising', 'Pitch Black', '30th Celebration', 'Delta Reign'
    ],
    'Scarlet & Violet': [
        'Scarlet & Violet Base', 'Paldea Evolved', 'Obsidian Flames', '151', 
        'Paradox Rift', 'Paldean Fates', 'Temporal Forces', 'Twilight Masquerade', 
        'Shrouded Fable', 'Stellar Crown', 'Surging Sparks', 'Prismatic Evolutions',
        'Destined Rivals', 'Journey Together', 'Black Bolt', 'White Flare'
    ],
    'Sword & Shield': [
        'Sword & Shield Base', 'Rebel Clash', 'Darkness Ablaze', 'Champion\'s Path', 
        'Vivid Voltage', 'Shining Fates', 'Battle Styles', 'Chilling Reign', 
        'Evolving Skies', 'Celebrations', 'Fusion Strike', 'Brilliant Stars', 
        'Astral Radiance', 'Pokémon GO', 'Lost Origin', 'Silver Tempest', 'Crown Zenith'
    ],
    'Sun & Moon': [
        'Sun & Moon Base', 'Guardians Rising', 'Burning Shadows', 'Shining Legends', 
        'Crimson Invasion', 'Ultra Prism', 'Forbidden Light', 'Celestial Storm', 
        'Dragon Majesty', 'Lost Thunder', 'Team Up', 'Detective Pikachu', 
        'Unbroken Bonds', 'Unified Minds', 'Hidden Fates', 'Cosmic Eclipse'
    ],
    'XY': [
        'XY Base', 'Flashfire', 'Furious Fists', 'Phantom Forces', 'Primal Clash', 
        'Roaring Skies', 'Ancient Origins', 'BREAKthrough', 'BREAKpoint', 
        'Generations', 'Fates Collide', 'Steam Siege', 'Evolutions'
    ],
    'Black & White': [
        'Black & White Base', 'Emerging Powers', 'Noble Victories', 'Next Destinies', 
        'Dark Explorers', 'Dragons Exalted', 'Boundaries Crossed', 'Plasma Storm', 
        'Plasma Freeze', 'Plasma Blast', 'Legendary Treasures'
    ],
    'HeartGold & SoulSilver': [
        'HeartGold & SoulSilver Base', 'Unleashed', 'Undaunted', 'Triumphant', 'Call of Legends'
    ],
    'Platinum': [
        'Platinum Base', 'Rising Rivals', 'Supreme Victors', 'Arceus'
    ],
    'Diamond & Pearl': [
        'Diamond & Pearl Base', 'Mysterious Treasures', 'Secret Wonders', 'Great Encounters', 
        'Majestic Dawn', 'Legends Awakened', 'Stormfront'
    ],
    'EX': [
        'Ruby & Sapphire', 'Sandstorm', 'Dragon', 'Team Magma vs Team Aqua', 'Hidden Legends', 
        'FireRed & LeafGreen', 'Team Rocket Returns', 'Deoxys', 'Emerald', 'Unseen Forces', 
        'Delta Species', 'Legend Maker', 'Holon Phantoms', 'Crystal Guardians', 'Dragon Frontiers', 'Power Keepers'
    ],
    'E-Card': [
        'Expedition', 'Aquapolis', 'Skyridge'
    ],
    'Neo': [
        'Genesis', 'Discovery', 'Revelation', 'Destiny'
    ],
    'Vintage': [
        'Base Set', 'Jungle', 'Fossil', 'Base Set 2', 'Team Rocket', 'Gym Heroes', 'Gym Challenge'
    ]
};

export const formatFilters = () => {
    return Object.keys(pokemonSeriesSets).map(series => {
        return {
            name: series.toUpperCase(),
            expanded: false, // Default to collapsed so the sidebar isn't massively long
            options: pokemonSeriesSets[series].map(setName => ({
                id: setName.toLowerCase().replace(/\s+/g, '-'),
                label: setName,
                checked: false
            }))
        }
    });
};
