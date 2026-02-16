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
        write!(f, "{s}")
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
    Advisor, Ally, Angel, Ape, Archer, Armadillo, Artificer, Assassin, Avatar,
    Badger, Barbarian, Bard, Bat, Bear, Beast, Berserker, Bird, Bison, Boar,
    Cat, Centaur, Citizen, Cleric, Construct, Coward, Crocodile, Cyclops,
    Demon, Devil, Dinosaur, Djinn, Dog, Dragon, Drake, Druid, Dryad, Dwarf,
    Elder, Eldrazi, Elemental, Elephant, Elf, Elk,
    Faerie, Fish, Fox, Frog, Fungus,
    Gargoyle, Giant, Gnome, Goat, Goblin, God, Golem, Gorgon, Gremlin, Griffin,
    Halfling, Hero, Homunculus, Horror, Horse, Human, Hydra, Hyena,
    Illusion, Imp, Incarnation, Insect,
    Jellyfish, Juggernaut,
    Kangaroo, Kirin, Kithkin, Knight, Kobold, Kor, Kraken,
    Lemur, Lizard, Mercenary, Merfolk, Minotaur, Mole, Mongoose, Monk, Monkey, Mouse, Mutant, Myr,
    Naga, Nightmare, Ninja, Noble, Noggle,
    Octopus, Ogre, Ooze, Orc, Otter, Ouphe, Ox,
    Peasant, Pegasus, Performer, Phoenix, Phyrexian, Pilot, Pirate, Plant, Platypus, Porcupine, Praetor,
    Rabbit, Raccoon, Ranger, Rat, Rebel, Rhino, Robot, Rogue,
    Salamander, Samurai, Saproling, Satyr, Scarecrow, Scout, Seal, Serpent, Servo, Shark,
    Shade, Shaman, Shapeshifter, Skeleton, Sliver, Snake, Soldier, Sorcerer,
    Spawn, Specter, Sphinx, Spider, Spirit, Squirrel,
    Tiefling, Treefolk, Troll, Turtle,
    Unicorn,
    Vampire, Vedalken,
    Wall, Warlock, Warrior, Weird, Werewolf, Wizard, Wolf, Worm, Wurm,
    Yeti,
    Zombie,

    // ── Planeswalker types (common)
    PwAjani, PwChandra, PwElspeth, PwGarruk, PwGideon, PwJace, PwKaito, PwKarn,
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
            | SubType::PwGideon | SubType::PwJace | SubType::PwKaito | SubType::PwKarn | SubType::PwLiliana
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
    SubType::Forest, SubType::Island, SubType::Mountain, SubType::Plains, SubType::Swamp, SubType::Cave, SubType::Desert,
    SubType::Gate, SubType::Lair, SubType::Locus, SubType::Clue, SubType::Equipment, SubType::Food, SubType::Gold,
    SubType::Treasure, SubType::Vehicle, SubType::Blood, SubType::Map, SubType::Powerstone, SubType::Incubator, SubType::Aura,
    SubType::Cartouche, SubType::Case, SubType::Class, SubType::Curse, SubType::Role, SubType::Room, SubType::Saga,
    SubType::Shrine, SubType::Adventure, SubType::Arcane, SubType::Lesson, SubType::Trap, SubType::Siege, SubType::Advisor,
    SubType::Ally, SubType::Angel, SubType::Ape, SubType::Archer, SubType::Armadillo, SubType::Artificer, SubType::Assassin,
    SubType::Avatar, SubType::Badger, SubType::Barbarian, SubType::Bard, SubType::Bat, SubType::Bear, SubType::Beast,
    SubType::Berserker, SubType::Bird, SubType::Bison, SubType::Boar, SubType::Cat, SubType::Centaur, SubType::Citizen,
    SubType::Cleric, SubType::Construct, SubType::Coward, SubType::Crocodile, SubType::Cyclops, SubType::Demon, SubType::Devil,
    SubType::Dinosaur, SubType::Djinn, SubType::Dog, SubType::Dragon, SubType::Drake, SubType::Druid, SubType::Dryad,
    SubType::Dwarf, SubType::Elder, SubType::Eldrazi, SubType::Elemental, SubType::Elephant, SubType::Elf, SubType::Elk,
    SubType::Faerie, SubType::Fish, SubType::Fox, SubType::Frog, SubType::Fungus, SubType::Gargoyle, SubType::Giant,
    SubType::Gnome, SubType::Goat, SubType::Goblin, SubType::God, SubType::Golem, SubType::Gorgon, SubType::Gremlin,
    SubType::Griffin, SubType::Halfling, SubType::Hero, SubType::Homunculus, SubType::Horror, SubType::Horse, SubType::Human,
    SubType::Hydra, SubType::Hyena, SubType::Illusion, SubType::Imp, SubType::Incarnation, SubType::Insect, SubType::Jellyfish,
    SubType::Juggernaut, SubType::Kangaroo, SubType::Kirin, SubType::Kithkin, SubType::Knight, SubType::Kobold, SubType::Kor,
    SubType::Kraken, SubType::Lemur, SubType::Lizard, SubType::Mercenary, SubType::Merfolk, SubType::Minotaur, SubType::Mole,
    SubType::Mongoose, SubType::Monk, SubType::Monkey, SubType::Mouse, SubType::Mutant, SubType::Myr, SubType::Naga,
    SubType::Nightmare, SubType::Ninja, SubType::Noble, SubType::Noggle, SubType::Octopus, SubType::Ogre, SubType::Ooze,
    SubType::Orc, SubType::Otter, SubType::Ouphe, SubType::Ox, SubType::Peasant, SubType::Pegasus, SubType::Performer,
    SubType::Phoenix, SubType::Phyrexian, SubType::Pilot, SubType::Pirate, SubType::Plant, SubType::Platypus, SubType::Porcupine,
    SubType::Praetor, SubType::Rabbit, SubType::Raccoon, SubType::Ranger, SubType::Rat, SubType::Rebel, SubType::Rhino,
    SubType::Robot, SubType::Rogue, SubType::Salamander, SubType::Samurai, SubType::Saproling, SubType::Satyr, SubType::Scarecrow,
    SubType::Scout, SubType::Seal, SubType::Serpent, SubType::Servo, SubType::Shark, SubType::Shade, SubType::Shaman,
    SubType::Shapeshifter, SubType::Skeleton, SubType::Sliver, SubType::Snake, SubType::Soldier, SubType::Sorcerer, SubType::Spawn,
    SubType::Specter, SubType::Sphinx, SubType::Spider, SubType::Spirit, SubType::Squirrel, SubType::Tiefling, SubType::Treefolk,
    SubType::Troll, SubType::Turtle, SubType::Unicorn, SubType::Vampire, SubType::Vedalken, SubType::Wall, SubType::Warlock,
    SubType::Warrior, SubType::Weird, SubType::Werewolf, SubType::Wizard, SubType::Wolf, SubType::Worm, SubType::Wurm,
    SubType::Yeti, SubType::Zombie, SubType::PwAjani, SubType::PwChandra, SubType::PwElspeth, SubType::PwGarruk, SubType::PwGideon,
    SubType::PwJace, SubType::PwKaito, SubType::PwKarn, SubType::PwLiliana, SubType::PwNahiri, SubType::PwNissa, SubType::PwNixilis,
    SubType::PwRal, SubType::PwSarkhan, SubType::PwSorin, SubType::PwTeferi, SubType::PwUgin, SubType::PwVivien, SubType::PwVraska,
];

/// (index into SUBTYPE_VARIANTS, description string)
static SUBTYPE_DESCRIPTION_MAP: &[(usize, &str)] = &[
    (0, "Forest"), (1, "Island"), (2, "Mountain"), (3, "Plains"), (4, "Swamp"), (5, "Cave"),
    (6, "Desert"), (7, "Gate"), (8, "Lair"), (9, "Locus"), (10, "Clue"), (11, "Equipment"),
    (12, "Food"), (13, "Gold"), (14, "Treasure"), (15, "Vehicle"), (16, "Blood"), (17, "Map"),
    (18, "Powerstone"), (19, "Incubator"), (20, "Aura"), (21, "Cartouche"), (22, "Case"), (23, "Class"),
    (24, "Curse"), (25, "Role"), (26, "Room"), (27, "Saga"), (28, "Shrine"), (29, "Adventure"),
    (30, "Arcane"), (31, "Lesson"), (32, "Trap"), (33, "Siege"), (34, "Advisor"), (35, "Ally"),
    (36, "Angel"), (37, "Ape"), (38, "Archer"), (39, "Armadillo"), (40, "Artificer"), (41, "Assassin"),
    (42, "Avatar"), (43, "Badger"), (44, "Barbarian"), (45, "Bard"), (46, "Bat"), (47, "Bear"),
    (48, "Beast"), (49, "Berserker"), (50, "Bird"), (51, "Bison"), (52, "Boar"), (53, "Cat"),
    (54, "Centaur"), (55, "Citizen"), (56, "Cleric"), (57, "Construct"), (58, "Coward"), (59, "Crocodile"),
    (60, "Cyclops"), (61, "Demon"), (62, "Devil"), (63, "Dinosaur"), (64, "Djinn"), (65, "Dog"),
    (66, "Dragon"), (67, "Drake"), (68, "Druid"), (69, "Dryad"), (70, "Dwarf"), (71, "Elder"),
    (72, "Eldrazi"), (73, "Elemental"), (74, "Elephant"), (75, "Elf"), (76, "Elk"), (77, "Faerie"),
    (78, "Fish"), (79, "Fox"), (80, "Frog"), (81, "Fungus"), (82, "Gargoyle"), (83, "Giant"),
    (84, "Gnome"), (85, "Goat"), (86, "Goblin"), (87, "God"), (88, "Golem"), (89, "Gorgon"),
    (90, "Gremlin"), (91, "Griffin"), (92, "Halfling"), (93, "Hero"), (94, "Homunculus"), (95, "Horror"),
    (96, "Horse"), (97, "Human"), (98, "Hydra"), (99, "Hyena"), (100, "Illusion"), (101, "Imp"),
    (102, "Incarnation"), (103, "Insect"), (104, "Jellyfish"), (105, "Juggernaut"), (106, "Kangaroo"), (107, "Kirin"),
    (108, "Kithkin"), (109, "Knight"), (110, "Kobold"), (111, "Kor"), (112, "Kraken"), (113, "Lemur"),
    (114, "Lizard"), (115, "Mercenary"), (116, "Merfolk"), (117, "Minotaur"), (118, "Mole"), (119, "Mongoose"),
    (120, "Monk"), (121, "Monkey"), (122, "Mouse"), (123, "Mutant"), (124, "Myr"), (125, "Naga"),
    (126, "Nightmare"), (127, "Ninja"), (128, "Noble"), (129, "Noggle"), (130, "Octopus"), (131, "Ogre"),
    (132, "Ooze"), (133, "Orc"), (134, "Otter"), (135, "Ouphe"), (136, "Ox"), (137, "Peasant"),
    (138, "Pegasus"), (139, "Performer"), (140, "Phoenix"), (141, "Phyrexian"), (142, "Pilot"), (143, "Pirate"),
    (144, "Plant"), (145, "Platypus"), (146, "Porcupine"), (147, "Praetor"), (148, "Rabbit"), (149, "Raccoon"),
    (150, "Ranger"), (151, "Rat"), (152, "Rebel"), (153, "Rhino"), (154, "Robot"), (155, "Rogue"),
    (156, "Salamander"), (157, "Samurai"), (158, "Saproling"), (159, "Satyr"), (160, "Scarecrow"), (161, "Scout"),
    (162, "Seal"), (163, "Serpent"), (164, "Servo"), (165, "Shark"), (166, "Shade"), (167, "Shaman"),
    (168, "Shapeshifter"), (169, "Skeleton"), (170, "Sliver"), (171, "Snake"), (172, "Soldier"), (173, "Sorcerer"),
    (174, "Spawn"), (175, "Specter"), (176, "Sphinx"), (177, "Spider"), (178, "Spirit"), (179, "Squirrel"),
    (180, "Tiefling"), (181, "Treefolk"), (182, "Troll"), (183, "Turtle"), (184, "Unicorn"), (185, "Vampire"),
    (186, "Vedalken"), (187, "Wall"), (188, "Warlock"), (189, "Warrior"), (190, "Weird"), (191, "Werewolf"),
    (192, "Wizard"), (193, "Wolf"), (194, "Worm"), (195, "Wurm"), (196, "Yeti"), (197, "Zombie"),
    (198, "Ajani"), (199, "Chandra"), (200, "Elspeth"), (201, "Garruk"), (202, "Gideon"), (203, "Jace"),
    (204, "Kaito"), (205, "Karn"), (206, "Liliana"), (207, "Nahiri"), (208, "Nissa"), (209, "Nixilis"),
    (210, "Ral"), (211, "Sarkhan"), (212, "Sorin"), (213, "Teferi"), (214, "Ugin"), (215, "Vivien"),
    (216, "Vraska"),
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
        const CONSPIRE         = 1 << 47;
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
            "changeling" => Some(KeywordAbilities::CHANGELING),
            "wither" => Some(KeywordAbilities::WITHER),
            "infect" => Some(KeywordAbilities::INFECT),
            "shadow" => Some(KeywordAbilities::SHADOW),
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
