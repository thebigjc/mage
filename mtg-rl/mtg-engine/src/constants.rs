use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::fmt;

// ── Zones ──────────────────────────────────────────────────────────────────

/// Game zones where objects can exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Zone {
    Hand,
    Battlefield,
    Graveyard,
    Library,
    Exile,
    Stack,
    Command,
    Outside,
}

impl Zone {
    pub fn is_public(self) -> bool {
        matches!(
            self,
            Zone::Graveyard | Zone::Battlefield | Zone::Stack | Zone::Exile | Zone::Command
        )
    }
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Zone::Hand => "hand",
            Zone::Battlefield => "battlefield",
            Zone::Graveyard => "graveyard",
            Zone::Library => "library",
            Zone::Exile => "exile",
            Zone::Stack => "stack",
            Zone::Command => "command zone",
            Zone::Outside => "outside the game",
        };
        write!(f, "{}", s)
    }
}

// ── Turn phases and steps ──────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TurnPhase {
    Beginning,
    PrecombatMain,
    Combat,
    PostcombatMain,
    Ending,
}

impl TurnPhase {
    pub fn is_main(self) -> bool {
        matches!(self, TurnPhase::PrecombatMain | TurnPhase::PostcombatMain)
    }
}

/// Turn phases and steps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum PhaseStep {
    Untap = 0,
    Upkeep = 1,
    Draw = 2,
    PrecombatMain = 3,
    BeginCombat = 4,
    DeclareAttackers = 5,
    DeclareBlockers = 6,
    FirstStrikeDamage = 7,
    CombatDamage = 8,
    EndCombat = 9,
    PostcombatMain = 10,
    EndStep = 11,
    Cleanup = 12,
}

impl PhaseStep {
    pub fn index(self) -> u8 {
        self as u8
    }

    pub fn is_before(self, other: PhaseStep) -> bool {
        (self as u8) < (other as u8)
    }

    pub fn is_after(self, other: PhaseStep) -> bool {
        (self as u8) > (other as u8)
    }

    pub fn phase(self) -> TurnPhase {
        match self {
            PhaseStep::Untap | PhaseStep::Upkeep | PhaseStep::Draw => TurnPhase::Beginning,
            PhaseStep::PrecombatMain => TurnPhase::PrecombatMain,
            PhaseStep::BeginCombat
            | PhaseStep::DeclareAttackers
            | PhaseStep::DeclareBlockers
            | PhaseStep::FirstStrikeDamage
            | PhaseStep::CombatDamage
            | PhaseStep::EndCombat => TurnPhase::Combat,
            PhaseStep::PostcombatMain => TurnPhase::PostcombatMain,
            PhaseStep::EndStep | PhaseStep::Cleanup => TurnPhase::Ending,
        }
    }

    /// Steps in normal turn order.
    pub const ALL: [PhaseStep; 13] = [
        PhaseStep::Untap,
        PhaseStep::Upkeep,
        PhaseStep::Draw,
        PhaseStep::PrecombatMain,
        PhaseStep::BeginCombat,
        PhaseStep::DeclareAttackers,
        PhaseStep::DeclareBlockers,
        PhaseStep::FirstStrikeDamage,
        PhaseStep::CombatDamage,
        PhaseStep::EndCombat,
        PhaseStep::PostcombatMain,
        PhaseStep::EndStep,
        PhaseStep::Cleanup,
    ];
}

// ── Card types ─────────────────────────────────────────────────────────────

/// Card types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardType {
    Artifact,
    Battle,
    Creature,
    Enchantment,
    Instant,
    Land,
    Planeswalker,
    Sorcery,
    Kindred,
    Tribal,
}

impl CardType {
    pub fn is_permanent(self) -> bool {
        matches!(
            self,
            CardType::Artifact
                | CardType::Battle
                | CardType::Creature
                | CardType::Enchantment
                | CardType::Land
                | CardType::Planeswalker
        )
    }
}

// ── Supertypes ─────────────────────────────────────────────────────────────

/// Card supertypes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SuperType {
    Basic,
    Legendary,
    Snow,
    World,
}

// ── Subtype categories ─────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubTypeSet {
    CreatureType,
    SpellType,
    BasicLandType,
    NonBasicLandType,
    EnchantmentType,
    ArtifactType,
    PlaneswalkerType,
    BattleType,
}

impl SubTypeSet {
    pub fn is_land(self) -> bool {
        matches!(self, SubTypeSet::BasicLandType | SubTypeSet::NonBasicLandType)
    }
}

/// SubType uses a string-interned approach: the description is the canonical name.
/// We store commonly used subtypes as enum variants for pattern matching and filter
/// efficiency, with a Custom variant for any not in the list.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubType {
    // ── Basic land types (these matter for game rules: domain, landwalk, etc.)
    Forest,
    Island,
    Mountain,
    Plains,
    Swamp,

    // ── Non-basic land types
    Cave,
    Desert,
    Gate,
    Lair,
    Locus,

    // ── Artifact types
    Clue,
    Equipment,
    Food,
    Gold,
    Treasure,
    Vehicle,
    Blood,
    Map,
    Powerstone,
    Incubator,

    // ── Enchantment types
    Aura,
    Cartouche,
    Case,
    Class,
    Curse,
    Role,
    Room,
    Saga,
    Shrine,

    // ── Spell types
    Adventure,
    Arcane,
    Lesson,
    Trap,

    // ── Battle types
    Siege,

    // ── Common creature types (gameplay-relevant subset)
    Advisor, Ally, Angel, Ape, Archer, Assassin, Avatar,
    Barbarian, Bear, Beast, Berserker, Bird,
    Cat, Centaur, Cleric, Construct, Coward, Crocodile,
    Demon, Devil, Dinosaur, Djinn, Dog, Dragon, Drake, Druid, Dwarf,
    Elder, Eldrazi, Elemental, Elephant, Elf, Elk,
    Faerie, Fish, Fox, Frog, Fungus,
    Gargoyle, Giant, Gnome, Goat, Goblin, God, Golem, Gorgon, Griffin,
    Halfling, Hero, Horror, Horse, Human, Hydra, Hyena,
    Illusion, Imp, Insect,
    Jellyfish,
    Knight, Kobold, Kraken,
    Mercenary, Merfolk, Minotaur, Monk, Mouse, Mutant, Myr,
    Nightmare, Ninja, Noble,
    Ogre, Ooze, Orc, Otter, Ox,
    Peasant, Pegasus, Performer, Phoenix, Phyrexian, Pilot, Pirate, Plant, Praetor,
    Rabbit, Raccoon, Ranger, Rat, Rebel, Rhino, Robot, Rogue,
    Salamander, Samurai, Saproling, Satyr, Scarecrow, Scout, Serpent, Servo,
    Shade, Shaman, Shapeshifter, Skeleton, Sliver, Snake, Soldier, Sorcerer,
    Spawn, Specter, Sphinx, Spider, Spirit, Squirrel,
    Tiefling, Treefolk, Troll, Turtle,
    Unicorn,
    Vampire, Vedalken,
    Wall, Warlock, Warrior, Weird, Werewolf, Wizard, Wolf, Worm, Wurm,
    Zombie,

    // ── Planeswalker types (common)
    PwAjani, PwChandra, PwElspeth, PwGarruk, PwGideon, PwJace, PwKarn,
    PwLiliana, PwNahiri, PwNissa, PwNixilis, PwRal, PwSarkhan, PwSorin,
    PwTeferi, PwUgin, PwVivien, PwVraska,

    /// Catch-all for subtypes not in the above list.
    Custom(String),
}

impl SubType {
    pub fn set(&self) -> SubTypeSet {
        match self {
            SubType::Forest | SubType::Island | SubType::Mountain
            | SubType::Plains | SubType::Swamp => SubTypeSet::BasicLandType,

            SubType::Cave | SubType::Desert | SubType::Gate
            | SubType::Lair | SubType::Locus => SubTypeSet::NonBasicLandType,

            SubType::Clue | SubType::Equipment | SubType::Food | SubType::Gold
            | SubType::Treasure | SubType::Vehicle | SubType::Blood | SubType::Map
            | SubType::Powerstone | SubType::Incubator => SubTypeSet::ArtifactType,

            SubType::Aura | SubType::Cartouche | SubType::Case | SubType::Class
            | SubType::Curse | SubType::Role | SubType::Room | SubType::Saga
            | SubType::Shrine => SubTypeSet::EnchantmentType,

            SubType::Adventure | SubType::Arcane | SubType::Lesson
            | SubType::Trap => SubTypeSet::SpellType,

            SubType::Siege => SubTypeSet::BattleType,

            SubType::PwAjani | SubType::PwChandra | SubType::PwElspeth | SubType::PwGarruk
            | SubType::PwGideon | SubType::PwJace | SubType::PwKarn | SubType::PwLiliana
            | SubType::PwNahiri | SubType::PwNissa | SubType::PwNixilis | SubType::PwRal
            | SubType::PwSarkhan | SubType::PwSorin | SubType::PwTeferi | SubType::PwUgin
            | SubType::PwVivien | SubType::PwVraska => SubTypeSet::PlaneswalkerType,

            // All creature types and Custom
            _ => SubTypeSet::CreatureType,
        }
    }

    pub fn is_basic_land_type(&self) -> bool {
        self.set() == SubTypeSet::BasicLandType
    }

    /// Look up a subtype by its description string (card text name).
    pub fn by_description(desc: &str) -> SubType {
        SUBTYPE_DESCRIPTION_MAP
            .iter()
            .find(|(_, d)| *d == desc)
            .map(|(idx, _)| SUBTYPE_VARIANTS[*idx].clone())
            .unwrap_or_else(|| SubType::Custom(desc.to_string()))
    }

    pub fn description(&self) -> &str {
        match self {
            SubType::Custom(s) => s.as_str(),
            other => {
                SUBTYPE_DESCRIPTION_MAP
                    .iter()
                    .find(|(idx, _)| SUBTYPE_VARIANTS[*idx] == *other)
                    .map(|(_, d)| *d)
                    .unwrap_or("Unknown")
            }
        }
    }
}

impl fmt::Display for SubType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

// Static lookup tables for SubType descriptions.
// Using indices into a variants array to avoid bloating the binary with
// hundreds of match arms in both directions.
static SUBTYPE_VARIANTS: &[SubType] = &[
    SubType::Forest, SubType::Island, SubType::Mountain, SubType::Plains, SubType::Swamp,
    SubType::Cave, SubType::Desert, SubType::Gate, SubType::Lair, SubType::Locus, SubType::Clue,
    SubType::Equipment, SubType::Food, SubType::Gold, SubType::Treasure, SubType::Vehicle,
    SubType::Blood, SubType::Map, SubType::Powerstone, SubType::Incubator, SubType::Aura,
    SubType::Cartouche, SubType::Case, SubType::Class, SubType::Curse, SubType::Role,
    SubType::Room, SubType::Saga, SubType::Shrine, SubType::Adventure, SubType::Arcane,
    SubType::Lesson, SubType::Trap, SubType::Siege, SubType::Advisor, SubType::Ally,
    SubType::Angel, SubType::Ape, SubType::Archer, SubType::Assassin, SubType::Avatar,
    SubType::Barbarian, SubType::Bear, SubType::Beast, SubType::Berserker, SubType::Bird,
    SubType::Cat, SubType::Centaur, SubType::Cleric, SubType::Construct, SubType::Coward,
    SubType::Crocodile, SubType::Demon, SubType::Devil, SubType::Dinosaur, SubType::Djinn,
    SubType::Dog, SubType::Dragon, SubType::Drake, SubType::Druid, SubType::Dwarf, SubType::Elder,
    SubType::Eldrazi, SubType::Elemental, SubType::Elephant, SubType::Elf, SubType::Elk,
    SubType::Faerie, SubType::Fish, SubType::Fox, SubType::Frog, SubType::Fungus,
    SubType::Gargoyle, SubType::Giant, SubType::Gnome, SubType::Goat, SubType::Goblin,
    SubType::God, SubType::Golem, SubType::Gorgon, SubType::Griffin, SubType::Halfling,
    SubType::Hero, SubType::Horror, SubType::Horse, SubType::Human, SubType::Hydra, SubType::Hyena,
    SubType::Illusion, SubType::Imp, SubType::Insect, SubType::Jellyfish, SubType::Knight,
    SubType::Kobold, SubType::Kraken, SubType::Mercenary, SubType::Merfolk, SubType::Minotaur,
    SubType::Monk, SubType::Mouse, SubType::Mutant, SubType::Myr, SubType::Nightmare,
    SubType::Ninja, SubType::Noble, SubType::Ogre, SubType::Ooze, SubType::Orc, SubType::Otter,
    SubType::Ox, SubType::Peasant, SubType::Pegasus, SubType::Performer, SubType::Phoenix,
    SubType::Phyrexian, SubType::Pilot, SubType::Pirate, SubType::Plant, SubType::Praetor,
    SubType::Rabbit, SubType::Raccoon, SubType::Ranger, SubType::Rat, SubType::Rebel,
    SubType::Rhino, SubType::Robot, SubType::Rogue, SubType::Salamander, SubType::Samurai,
    SubType::Saproling, SubType::Satyr, SubType::Scarecrow, SubType::Scout, SubType::Serpent,
    SubType::Servo, SubType::Shade, SubType::Shaman, SubType::Shapeshifter, SubType::Skeleton,
    SubType::Sliver, SubType::Snake, SubType::Soldier, SubType::Sorcerer, SubType::Spawn,
    SubType::Specter, SubType::Sphinx, SubType::Spider, SubType::Spirit, SubType::Squirrel,
    SubType::Tiefling, SubType::Treefolk, SubType::Troll, SubType::Turtle, SubType::Unicorn,
    SubType::Vampire, SubType::Vedalken, SubType::Wall, SubType::Warlock, SubType::Warrior,
    SubType::Weird, SubType::Werewolf, SubType::Wizard, SubType::Wolf, SubType::Worm,
    SubType::Wurm, SubType::Zombie, SubType::PwAjani, SubType::PwChandra, SubType::PwElspeth,
    SubType::PwGarruk, SubType::PwGideon, SubType::PwJace, SubType::PwKarn, SubType::PwLiliana,
    SubType::PwNahiri, SubType::PwNissa, SubType::PwNixilis, SubType::PwRal, SubType::PwSarkhan,
    SubType::PwSorin, SubType::PwTeferi, SubType::PwUgin, SubType::PwVivien, SubType::PwVraska,
];

/// (index into SUBTYPE_VARIANTS, description string)
static SUBTYPE_DESCRIPTION_MAP: &[(usize, &str)] = &[
    (0, "Forest"), (1, "Island"), (2, "Mountain"), (3, "Plains"), (4, "Swamp"), (5, "Cave"),
    (6, "Desert"), (7, "Gate"), (8, "Lair"), (9, "Locus"), (10, "Clue"), (11, "Equipment"),
    (12, "Food"), (13, "Gold"), (14, "Treasure"), (15, "Vehicle"), (16, "Blood"), (17, "Map"),
    (18, "Powerstone"), (19, "Incubator"), (20, "Aura"), (21, "Cartouche"), (22, "Case"),
    (23, "Class"), (24, "Curse"), (25, "Role"), (26, "Room"), (27, "Saga"), (28, "Shrine"),
    (29, "Adventure"), (30, "Arcane"), (31, "Lesson"), (32, "Trap"), (33, "Siege"),
    (34, "Advisor"), (35, "Ally"), (36, "Angel"), (37, "Ape"), (38, "Archer"), (39, "Assassin"),
    (40, "Avatar"), (41, "Barbarian"), (42, "Bear"), (43, "Beast"), (44, "Berserker"),
    (45, "Bird"), (46, "Cat"), (47, "Centaur"), (48, "Cleric"), (49, "Construct"), (50, "Coward"),
    (51, "Crocodile"), (52, "Demon"), (53, "Devil"), (54, "Dinosaur"), (55, "Djinn"), (56, "Dog"),
    (57, "Dragon"), (58, "Drake"), (59, "Druid"), (60, "Dwarf"), (61, "Elder"), (62, "Eldrazi"),
    (63, "Elemental"), (64, "Elephant"), (65, "Elf"), (66, "Elk"), (67, "Faerie"), (68, "Fish"),
    (69, "Fox"), (70, "Frog"), (71, "Fungus"), (72, "Gargoyle"), (73, "Giant"), (74, "Gnome"),
    (75, "Goat"), (76, "Goblin"), (77, "God"), (78, "Golem"), (79, "Gorgon"), (80, "Griffin"),
    (81, "Halfling"), (82, "Hero"), (83, "Horror"), (84, "Horse"), (85, "Human"), (86, "Hydra"),
    (87, "Hyena"), (88, "Illusion"), (89, "Imp"), (90, "Insect"), (91, "Jellyfish"),
    (92, "Knight"), (93, "Kobold"), (94, "Kraken"), (95, "Mercenary"), (96, "Merfolk"),
    (97, "Minotaur"), (98, "Monk"), (99, "Mouse"), (100, "Mutant"), (101, "Myr"),
    (102, "Nightmare"), (103, "Ninja"), (104, "Noble"), (105, "Ogre"), (106, "Ooze"), (107, "Orc"),
    (108, "Otter"), (109, "Ox"), (110, "Peasant"), (111, "Pegasus"), (112, "Performer"),
    (113, "Phoenix"), (114, "Phyrexian"), (115, "Pilot"), (116, "Pirate"), (117, "Plant"),
    (118, "Praetor"), (119, "Rabbit"), (120, "Raccoon"), (121, "Ranger"), (122, "Rat"),
    (123, "Rebel"), (124, "Rhino"), (125, "Robot"), (126, "Rogue"), (127, "Salamander"),
    (128, "Samurai"), (129, "Saproling"), (130, "Satyr"), (131, "Scarecrow"), (132, "Scout"),
    (133, "Serpent"), (134, "Servo"), (135, "Shade"), (136, "Shaman"), (137, "Shapeshifter"),
    (138, "Skeleton"), (139, "Sliver"), (140, "Snake"), (141, "Soldier"), (142, "Sorcerer"),
    (143, "Spawn"), (144, "Specter"), (145, "Sphinx"), (146, "Spider"), (147, "Spirit"),
    (148, "Squirrel"), (149, "Tiefling"), (150, "Treefolk"), (151, "Troll"), (152, "Turtle"),
    (153, "Unicorn"), (154, "Vampire"), (155, "Vedalken"), (156, "Wall"), (157, "Warlock"),
    (158, "Warrior"), (159, "Weird"), (160, "Werewolf"), (161, "Wizard"), (162, "Wolf"),
    (163, "Worm"), (164, "Wurm"), (165, "Zombie"), (166, "Ajani"), (167, "Chandra"),
    (168, "Elspeth"), (169, "Garruk"), (170, "Gideon"), (171, "Jace"), (172, "Karn"),
    (173, "Liliana"), (174, "Nahiri"), (175, "Nissa"), (176, "Nixilis"), (177, "Ral"),
    (178, "Sarkhan"), (179, "Sorin"), (180, "Teferi"), (181, "Ugin"), (182, "Vivien"),
    (183, "Vraska"),
];

// ── Color ──────────────────────────────────────────────────────────────────

/// The five colors of mana plus colorless.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

impl ManaColor {
    pub fn symbol(self) -> char {
        match self {
            ManaColor::White => 'W',
            ManaColor::Blue => 'U',
            ManaColor::Black => 'B',
            ManaColor::Red => 'R',
            ManaColor::Green => 'G',
            ManaColor::Colorless => 'C',
        }
    }

    pub fn from_symbol(c: char) -> Option<ManaColor> {
        match c {
            'W' => Some(ManaColor::White),
            'U' => Some(ManaColor::Blue),
            'B' => Some(ManaColor::Black),
            'R' => Some(ManaColor::Red),
            'G' => Some(ManaColor::Green),
            'C' => Some(ManaColor::Colorless),
            _ => None,
        }
    }

    pub fn is_colored(self) -> bool {
        !matches!(self, ManaColor::Colorless)
    }

    pub const COLORS: [ManaColor; 5] = [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ];

    pub const ALL: [ManaColor; 6] = [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
        ManaColor::Colorless,
    ];
}

/// A separate Color enum for the five MTG colors (no colorless).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    pub fn symbol(self) -> char {
        match self {
            Color::White => 'W',
            Color::Blue => 'U',
            Color::Black => 'B',
            Color::Red => 'R',
            Color::Green => 'G',
        }
    }

    pub fn from_symbol(c: char) -> Option<Color> {
        match c {
            'W' => Some(Color::White),
            'U' => Some(Color::Blue),
            'B' => Some(Color::Black),
            'R' => Some(Color::Red),
            'G' => Some(Color::Green),
            _ => None,
        }
    }

    pub fn to_mana_color(self) -> ManaColor {
        match self {
            Color::White => ManaColor::White,
            Color::Blue => ManaColor::Blue,
            Color::Black => ManaColor::Black,
            Color::Red => ManaColor::Red,
            Color::Green => ManaColor::Green,
        }
    }

    pub const ALL: [Color; 5] = [
        Color::White,
        Color::Blue,
        Color::Black,
        Color::Red,
        Color::Green,
    ];
}

// ── Keyword abilities (bitflags for O(1) checks) ──────────────────────────

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct KeywordAbilities: u64 {
        const FLYING           = 1 << 0;
        const FIRST_STRIKE     = 1 << 1;
        const DOUBLE_STRIKE    = 1 << 2;
        const TRAMPLE          = 1 << 3;
        const HASTE            = 1 << 4;
        const VIGILANCE        = 1 << 5;
        const LIFELINK         = 1 << 6;
        const DEATHTOUCH       = 1 << 7;
        const REACH            = 1 << 8;
        const DEFENDER         = 1 << 9;
        const MENACE           = 1 << 10;
        const FLASH            = 1 << 11;
        const INDESTRUCTIBLE   = 1 << 12;
        const HEXPROOF         = 1 << 13;
        const SHROUD           = 1 << 14;
        const FEAR             = 1 << 15;
        const INTIMIDATE       = 1 << 16;
        const PROTECTION       = 1 << 17;
        const WARD             = 1 << 18;
        const PROWESS          = 1 << 19;
        const UNDYING          = 1 << 20;
        const PERSIST          = 1 << 21;
        const WITHER           = 1 << 22;
        const INFECT           = 1 << 23;
        const SHADOW           = 1 << 24;
        const UNBLOCKABLE      = 1 << 25;
        const CHANGELING       = 1 << 26;
        const CASCADE          = 1 << 27;
        const CONVOKE          = 1 << 28;
        const DELVE            = 1 << 29;
        const EVOLVE           = 1 << 30;
        const EXALTED          = 1 << 31;
        const EXPLOIT          = 1 << 32;
        const FLANKING         = 1 << 33;
        const FORESTWALK       = 1 << 34;
        const ISLANDWALK       = 1 << 35;
        const MOUNTAINWALK     = 1 << 36;
        const PLAINSWALK       = 1 << 37;
        const SWAMPWALK        = 1 << 38;
        const TOTEM_ARMOR      = 1 << 39;
        const TOXIC            = 1 << 40;
        const AFFLICT          = 1 << 41;
        const BATTLE_CRY       = 1 << 42;
        const SKULK            = 1 << 43;
        const FABRICATE        = 1 << 44;
        const STORM            = 1 << 45;
        const PARTNER          = 1 << 46;
    }
}

impl Default for KeywordAbilities {
    fn default() -> Self {
        KeywordAbilities::empty()
    }
}

impl KeywordAbilities {
    /// Parse a keyword name string into the corresponding flag.
    /// Returns `None` if the string doesn't match any known keyword.
    pub fn keyword_from_name(name: &str) -> Option<KeywordAbilities> {
        match name.to_lowercase().as_str() {
            "flying" => Some(KeywordAbilities::FLYING),
            "first strike" | "first_strike" => Some(KeywordAbilities::FIRST_STRIKE),
            "double strike" | "double_strike" => Some(KeywordAbilities::DOUBLE_STRIKE),
            "trample" => Some(KeywordAbilities::TRAMPLE),
            "haste" => Some(KeywordAbilities::HASTE),
            "vigilance" => Some(KeywordAbilities::VIGILANCE),
            "lifelink" => Some(KeywordAbilities::LIFELINK),
            "deathtouch" => Some(KeywordAbilities::DEATHTOUCH),
            "reach" => Some(KeywordAbilities::REACH),
            "defender" => Some(KeywordAbilities::DEFENDER),
            "menace" => Some(KeywordAbilities::MENACE),
            "flash" => Some(KeywordAbilities::FLASH),
            "indestructible" => Some(KeywordAbilities::INDESTRUCTIBLE),
            "hexproof" => Some(KeywordAbilities::HEXPROOF),
            "shroud" => Some(KeywordAbilities::SHROUD),
            "fear" => Some(KeywordAbilities::FEAR),
            "intimidate" => Some(KeywordAbilities::INTIMIDATE),
            "protection" => Some(KeywordAbilities::PROTECTION),
            "ward" => Some(KeywordAbilities::WARD),
            "prowess" => Some(KeywordAbilities::PROWESS),
            "undying" => Some(KeywordAbilities::UNDYING),
            "persist" => Some(KeywordAbilities::PERSIST),
            _ => None,
        }
    }
}

impl Serialize for KeywordAbilities {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.bits().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for KeywordAbilities {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bits = u64::deserialize(deserializer)?;
        KeywordAbilities::from_bits(bits)
            .ok_or_else(|| serde::de::Error::custom("invalid KeywordAbilities bits"))
    }
}

// ── Continuous effect layers ───────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Layer {
    CopyEffects = 1,
    ControlChanging = 2,
    TextChanging = 3,
    TypeChanging = 4,
    ColorChanging = 5,
    AbilityAddingRemoving = 6,
    PTChanging = 7,
    PlayerEffects = 8,
    RulesEffects = 9,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SubLayer {
    CopyEffects1a,
    FaceDown1b,
    CharacteristicDefining7a,
    SetPT7b,
    ModifyPT7c,
    Counters7d,
    SwitchPT7e,
    NA,
}

// ── Duration ───────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Duration {
    OneUse,
    EndOfGame,
    WhileOnBattlefield,
    WhileControlled,
    WhileOnStack,
    WhileInGraveyard,
    EndOfTurn,
    UntilYourNextTurn,
    UntilEndOfYourNextTurn,
    EndOfCombat,
    EndOfStep,
    UntilSourceLeavesBattlefield,
    Custom,
}

// ── Outcome (for AI evaluation) ────────────────────────────────────────────

/// Whether an effect is good or bad for its target. Used by AI to evaluate choices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Outcome {
    Damage,
    DestroyPermanent,
    BoostCreature,
    UnboostCreature,
    AddAbility,
    LoseAbility,
    GainLife,
    LoseLife,
    ExtraTurn,
    BecomeCreature,
    PutCreatureInPlay,
    PutCardInPlay,
    PutLandInPlay,
    GainControl,
    DrawCard,
    Discard,
    Sacrifice,
    PlayForFree,
    ReturnToHand,
    Exile,
    Protect,
    PutManaInPool,
    Regenerate,
    PreventDamage,
    PreventCast,
    RedirectDamage,
    Tap,
    Transform,
    Untap,
    Win,
    Copy,
    Benefit,
    Detriment,
    Neutral,
    Removal,
    AIDontUseIt,
    Vote,
}

impl Outcome {
    /// Whether this outcome is good for the target of the effect.
    pub fn is_good(self) -> bool {
        matches!(
            self,
            Outcome::BoostCreature
                | Outcome::AddAbility
                | Outcome::GainLife
                | Outcome::ExtraTurn
                | Outcome::BecomeCreature
                | Outcome::PutCreatureInPlay
                | Outcome::PutCardInPlay
                | Outcome::PutLandInPlay
                | Outcome::DrawCard
                | Outcome::PlayForFree
                | Outcome::Protect
                | Outcome::PutManaInPool
                | Outcome::Regenerate
                | Outcome::PreventDamage
                | Outcome::RedirectDamage
                | Outcome::Transform
                | Outcome::Untap
                | Outcome::Win
                | Outcome::Copy
                | Outcome::Benefit
                | Outcome::Neutral
                | Outcome::Vote
        )
    }

    /// Invert good/bad classification.
    pub fn inverse(self) -> Outcome {
        if self.is_good() {
            Outcome::Detriment
        } else {
            Outcome::Benefit
        }
    }
}

// ── Target controller ──────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetController {
    Active,
    Any,
    You,
    NotYou,
    Opponent,
    Team,
    Owner,
    SourceController,
    EachPlayer,
}

// ── Ability type ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbilityType {
    PlayLand,
    Spell,
    Static,
    Evasion,
    ActivatedNonMana,
    ActivatedMana,
    TriggeredNonMana,
    TriggeredMana,
    SpecialAction,
}

impl AbilityType {
    pub fn is_activated(self) -> bool {
        matches!(self, AbilityType::ActivatedNonMana | AbilityType::ActivatedMana)
    }

    pub fn is_triggered(self) -> bool {
        matches!(self, AbilityType::TriggeredNonMana | AbilityType::TriggeredMana)
    }

    pub fn is_mana(self) -> bool {
        matches!(self, AbilityType::ActivatedMana | AbilityType::TriggeredMana)
    }
}

// ── Effect type ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectType {
    OneShot,
    Continuous,
    ContinuousRuleModification,
    Replacement,
    Prevention,
    AsThough,
    Restriction,
    Requirement,
    CostModification,
}

// ── AsThoughEffectType ─────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AsThoughEffectType {
    Attack,
    AttackAsHaste,
    ActivateHaste,
    BlockTapped,
    BlockShadow,
    BlockLandwalk,
    DamageNotBlocked,
    PlayFromNotOwnHandZone,
    CastFromNotOwnHandZone,
    CastAsInstant,
    ActivateAsInstant,
    Shroud,
    Hexproof,
    LookAtFaceDown,
    SpendOtherMana,
    SpendOnlyMana,
}

// ── Timing rule ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimingRule {
    Instant,
    Sorcery,
}

// ── Rarity ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
    Special,
    Bonus,
    Land,
}

// ── Watcher scope ──────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WatcherScope {
    Game,
    Player,
    Card,
}

// ── Comparison ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComparisonType {
    LessThan,
    LessOrEqual,
    Equal,
    GreaterThan,
    GreaterOrEqual,
}

impl ComparisonType {
    pub fn compare(self, a: i32, b: i32) -> bool {
        match self {
            ComparisonType::LessThan => a < b,
            ComparisonType::LessOrEqual => a <= b,
            ComparisonType::Equal => a == b,
            ComparisonType::GreaterThan => a > b,
            ComparisonType::GreaterOrEqual => a >= b,
        }
    }
}

// ── Cost modification type ─────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CostModificationType {
    Increase,
    Reduce,
    SetAll,
}
