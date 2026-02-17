#!/usr/bin/env python3
"""Migrate Filter::parse("...") calls to typed Filter constructors.

Phase 4 of the typed filter API migration. Converts all remaining
Filter::parse("...") call sites to use Filter::new(), factory methods,
or typed Predicate builders.
"""
import re
import sys
import os

# Mapping from parse string to typed replacement expression.
# Factory methods where available, Filter::new() for the rest.
REPLACEMENTS = {
    # === Factory methods (already exist) ===
    '""': 'Filter::new("all", Predicate::All)',
    '"all"': 'Filter::new("all", Predicate::All)',
    '"self"': 'Filter::self_reference()',
    '"enchanted creature"': 'Filter::enchanted_creature()',
    '"equipped creature"': 'Filter::equipped_creature()',
    '"creature"': 'Filter::any_creature()',
    '"creatures"': 'Filter::any_creature()',
    '"land"': 'Filter::any_land()',
    '"basic land"': 'Filter::basic_land()',
    '"creature you control"': 'Filter::creature_you_control()',
    '"creatures you control"': 'Filter::creature_you_control()',
    '"creatures opponents control"': 'Filter::creatures_opponents_control()',
    '"land card"': 'Filter::land_card()',
    '"card"': 'Filter::any_card()',

    # === Filter::new() with predicates ===

    # -- Creature type filters --
    '"creature spells"': 'Filter::new("creature spell", Predicate::creature())',
    '"creature spell you cast"': 'Filter::new("creature spell", Predicate::creature())',
    '"noncreature spells"': 'Filter::new("noncreature spell", Predicate::NotCardType(CardType::Creature))',
    '"noncreature spell you cast"': 'Filter::new("noncreature spell", Predicate::NotCardType(CardType::Creature))',
    '"creatures and planeswalkers"': 'Filter::new("creature or planeswalker", Predicate::creature().or(Predicate::planeswalker()))',
    '"spell or creature"': 'Filter::new("spell or creature", Predicate::creature())',
    '"spell"': 'Filter::new("spell", Predicate::All)',
    '"creature or land"': 'Filter::new("creature or land", Predicate::creature().or(Predicate::land()))',
    '"creature or enchantment"': 'Filter::new("creature or enchantment", Predicate::creature().or(Predicate::enchantment()))',
    '"creature with flying"': 'Filter::new("creature with flying", Predicate::creature().and(Predicate::HasKeyword(KeywordAbilities::FLYING)))',
    '"creatures with flying"': 'Filter::new("creature with flying", Predicate::creature().and(Predicate::HasKeyword(KeywordAbilities::FLYING)))',
    '"tapped creature"': 'Filter::new("tapped creature", Predicate::creature().and(Predicate::IsTapped))',
    '"creature token you control"': 'Filter::new("creature token you control", Predicate::creature().and(Predicate::IsToken).and(Predicate::Controller(TargetController::You)))',
    '"opponent creature"': 'Filter::creature_opponent_controls()',
    '"creature target player controls"': 'Filter::new("creature target player controls", Predicate::creature())',
    '"creatures target player controls"': 'Filter::new("creature target player controls", Predicate::creature())',
    '"creature defending player controls"': 'Filter::new("creature defending player controls", Predicate::creature())',
    '"another creature"': 'Filter::new("another creature", Predicate::creature()).excludes_source()',
    '"other creatures you control"': 'Filter::new("other creatures you control", Predicate::creature().and(Predicate::Controller(TargetController::You))).excludes_source()',

    # -- Creature with power/toughness filters --
    '"creature you control with power 4 or greater"': 'Filter::new("creature you control with power 4 or greater", Predicate::creature().and(Predicate::Controller(TargetController::You)).and(Predicate::PowerCompare(ComparisonType::GreaterOrEqual, 4)))',
    '"creature you control with power 2 or less"': 'Filter::new("creature you control with power 2 or less", Predicate::creature().and(Predicate::Controller(TargetController::You)).and(Predicate::PowerCompare(ComparisonType::LessOrEqual, 2)))',
    '"creature with mana value 6 or greater"': 'Filter::new("creature with mana value 6 or greater", Predicate::creature().and(Predicate::ManaValueCompare(ComparisonType::GreaterOrEqual, 6)))',
    '"creature an opponent controls with power 2 or less"': 'Filter::new("creature an opponent controls with power 2 or less", Predicate::creature().and(Predicate::Controller(TargetController::Opponent)).and(Predicate::PowerCompare(ComparisonType::LessOrEqual, 2)))',

    # -- Nonland permanent filters --
    '"nonland permanent with mana value 3 or less"': 'Filter::new("nonland permanent with mana value 3 or less", Predicate::nonland_permanent().and(Predicate::ManaValueCompare(ComparisonType::LessOrEqual, 3)))',
    '"non-Aura permanent"': 'Filter::new("non-Aura permanent", Predicate::Not(Box::new(Predicate::HasSubType(SubType::Aura))))',
    '"other nonland permanent you control"': 'Filter::new("other nonland permanent you control", Predicate::nonland_permanent().and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other nontoken creatures you control"': 'Filter::new("other nontoken creatures you control", Predicate::creature().and(Predicate::IsNontoken).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"nonlegendary creature you control"': 'Filter::new("nonlegendary creature you control", Predicate::creature().and(Predicate::Not(Box::new(Predicate::HasSuperType(SuperType::Legendary)))).and(Predicate::Controller(TargetController::You)))',
    '"permanent with mana value 3 or greater"': 'Filter::new("permanent with mana value 3 or greater", Predicate::ManaValueCompare(ComparisonType::GreaterOrEqual, 3))',
    '"permanent an opponent controls with mana value 2 or less"': 'Filter::new("permanent an opponent controls with mana value 2 or less", Predicate::Controller(TargetController::Opponent).and(Predicate::ManaValueCompare(ComparisonType::LessOrEqual, 2)))',

    # -- Artifact/enchantment/other type combos --
    '"artifact or creature an opponent controls"': 'Filter::new("artifact or creature an opponent controls", Predicate::Or(vec![Predicate::artifact(), Predicate::creature()]).and(Predicate::Controller(TargetController::Opponent)))',
    '"artifact, enchantment, or creature with flying"': 'Filter::new("artifact, enchantment, or creature with flying", Predicate::Or(vec![Predicate::artifact(), Predicate::enchantment(), Predicate::creature().and(Predicate::HasKeyword(KeywordAbilities::FLYING))]))',
    '"artifact, creature, or planeswalker"': 'Filter::new("artifact, creature, or planeswalker", Predicate::Or(vec![Predicate::artifact(), Predicate::creature(), Predicate::planeswalker()]))',
    '"instant or sorcery card"': 'Filter::new("instant or sorcery card", Predicate::instant().or(Predicate::sorcery()))',

    # -- Land filters --
    '"basic land card"': 'Filter::new("basic land card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()))',
    '"nonbasic land an opponent controls"': 'Filter::new("nonbasic land an opponent controls", Predicate::land().and(Predicate::Not(Box::new(Predicate::HasSuperType(SuperType::Basic)))).and(Predicate::Controller(TargetController::Opponent)))',
    '"basic Plains card"': 'Filter::new("basic Plains card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()).and(Predicate::HasSubType(SubType::Plains)))',
    '"basic Forest card"': 'Filter::new("basic Forest card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()).and(Predicate::HasSubType(SubType::Forest)))',
    '"basic Island you control"': 'Filter::new("basic Island you control", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()).and(Predicate::HasSubType(SubType::Island)).and(Predicate::Controller(TargetController::You)))',
    '"basic Swamp, Forest, or Island card"': 'Filter::new("basic Swamp, Forest, or Island card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()).and(Predicate::Or(vec![Predicate::HasSubType(SubType::Swamp), Predicate::HasSubType(SubType::Forest), Predicate::HasSubType(SubType::Island)])))',
    '"basic Plains, Swamp, or Forest card"': 'Filter::new("basic Plains, Swamp, or Forest card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()).and(Predicate::Or(vec![Predicate::HasSubType(SubType::Plains), Predicate::HasSubType(SubType::Swamp), Predicate::HasSubType(SubType::Forest)])))',
    '"basic Mountain, Plains, or Swamp card"': 'Filter::new("basic Mountain, Plains, or Swamp card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()).and(Predicate::Or(vec![Predicate::HasSubType(SubType::Mountain), Predicate::HasSubType(SubType::Plains), Predicate::HasSubType(SubType::Swamp)])))',
    '"basic Island, Mountain, or Plains card"': 'Filter::new("basic Island, Mountain, or Plains card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()).and(Predicate::Or(vec![Predicate::HasSubType(SubType::Island), Predicate::HasSubType(SubType::Mountain), Predicate::HasSubType(SubType::Plains)])))',
    '"basic Forest, Island, or Mountain card"': 'Filter::new("basic Forest, Island, or Mountain card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()).and(Predicate::Or(vec![Predicate::HasSubType(SubType::Forest), Predicate::HasSubType(SubType::Island), Predicate::HasSubType(SubType::Mountain)])))',
    '"up to two basic land cards"': 'Filter::new("basic land card", Predicate::HasSuperType(SuperType::Basic).and(Predicate::land()))',
    '"Forest"': 'Filter::new("Forest", Predicate::HasSubType(SubType::Forest))',

    # -- Subtype creature filters --
    '"Elf"': 'Filter::new("Elf", Predicate::HasSubType(SubType::Elf))',
    '"elf"': 'Filter::new("Elf", Predicate::HasSubType(SubType::Elf))',
    '"Goblin"': 'Filter::new("Goblin", Predicate::HasSubType(SubType::Goblin))',
    '"Human"': 'Filter::new("Human", Predicate::HasSubType(SubType::Human))',
    '"Warrior"': 'Filter::new("Warrior", Predicate::HasSubType(SubType::Warrior))',

    # -- Subtype lord filters (other X you control) --
    '"other Elf you control"': 'Filter::new("other Elf you control", Predicate::creature().and(Predicate::HasSubType(SubType::Elf)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Merfolk you control"': 'Filter::new("other Merfolk you control", Predicate::creature().and(Predicate::HasSubType(SubType::Merfolk)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Zombie you control"': 'Filter::new("other Zombie you control", Predicate::creature().and(Predicate::HasSubType(SubType::Zombie)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Spirit you control"': 'Filter::new("other Spirit you control", Predicate::creature().and(Predicate::HasSubType(SubType::Spirit)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Elementals you control"': 'Filter::new("other Elemental you control", Predicate::creature().and(Predicate::HasSubType(SubType::Elemental)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Goblin you control"': 'Filter::new("other Goblin you control", Predicate::creature().and(Predicate::HasSubType(SubType::Goblin)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Kithkin you control"': 'Filter::new("other Kithkin you control", Predicate::creature().and(Predicate::HasSubType(SubType::Kithkin)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Pirates you control"': 'Filter::new("other Pirate you control", Predicate::creature().and(Predicate::HasSubType(SubType::Pirate)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Giants you control"': 'Filter::new("other Giant you control", Predicate::creature().and(Predicate::HasSubType(SubType::Giant)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Dragon you control"': 'Filter::new("other Dragon you control", Predicate::creature().and(Predicate::HasSubType(SubType::Dragon)).and(Predicate::Controller(TargetController::You))).excludes_source()',
    '"other Cats you control"': 'Filter::new("other Cat you control", Predicate::creature().and(Predicate::HasSubType(SubType::Cat)).and(Predicate::Controller(TargetController::You))).excludes_source()',

    # -- Subtype you control (non-lord) --
    '"Elf you control"': 'Filter::new("Elf you control", Predicate::creature().and(Predicate::HasSubType(SubType::Elf)).and(Predicate::Controller(TargetController::You)))',
    '"each Elf you control"': 'Filter::new("Elf you control", Predicate::creature().and(Predicate::HasSubType(SubType::Elf)).and(Predicate::Controller(TargetController::You)))',
    '"Goblin you control"': 'Filter::new("Goblin you control", Predicate::creature().and(Predicate::HasSubType(SubType::Goblin)).and(Predicate::Controller(TargetController::You)))',
    '"Dragon you control"': 'Filter::new("Dragon you control", Predicate::creature().and(Predicate::HasSubType(SubType::Dragon)).and(Predicate::Controller(TargetController::You)))',
    '"Dragons you control"': 'Filter::new("Dragon you control", Predicate::creature().and(Predicate::HasSubType(SubType::Dragon)).and(Predicate::Controller(TargetController::You)))',
    '"Merfolk you control"': 'Filter::new("Merfolk you control", Predicate::creature().and(Predicate::HasSubType(SubType::Merfolk)).and(Predicate::Controller(TargetController::You)))',
    '"Kithkin you control"': 'Filter::new("Kithkin you control", Predicate::creature().and(Predicate::HasSubType(SubType::Kithkin)).and(Predicate::Controller(TargetController::You)))',
    '"Treefolk you control"': 'Filter::new("Treefolk you control", Predicate::creature().and(Predicate::HasSubType(SubType::Treefolk)).and(Predicate::Controller(TargetController::You)))',
    '"Skeleton you control"': 'Filter::new("Skeleton you control", Predicate::creature().and(Predicate::HasSubType(SubType::Skeleton)).and(Predicate::Controller(TargetController::You)))',
    '"Ally you control"': 'Filter::new("Ally you control", Predicate::creature().and(Predicate::HasSubType(SubType::Ally)).and(Predicate::Controller(TargetController::You)))',

    # -- Subtype non-creature filters --
    '"non-Elemental creatures"': 'Filter::new("non-Elemental creature", Predicate::creature().and(Predicate::Not(Box::new(Predicate::HasSubType(SubType::Elemental)))))',

    # -- Attacking creature filters --
    '"attacking creature you control"': 'Filter::new("attacking creature you control", Predicate::creature().and(Predicate::Controller(TargetController::You))).requires_attacking()',
    '"another attacking creature you control"': 'Filter::new("another attacking creature you control", Predicate::creature().and(Predicate::Controller(TargetController::You))).excludes_source().requires_attacking()',
    '"attacking Vampires you control"': 'Filter::new("attacking Vampire you control", Predicate::creature().and(Predicate::HasSubType(SubType::Vampire)).and(Predicate::Controller(TargetController::You))).requires_attacking()',
    '"attacking token you control"': 'Filter::new("attacking token you control", Predicate::IsToken.and(Predicate::Controller(TargetController::You))).requires_attacking()',
    '"other tapped creatures you control"': 'Filter::new("other tapped creature you control", Predicate::creature().and(Predicate::IsTapped).and(Predicate::Controller(TargetController::You))).excludes_source()',

    # -- "creatures of chosen type" --
    '"creatures of chosen type"': 'Filter::new("creature of the chosen type", Predicate::creature())',
    '"other creatures of chosen type"': 'Filter::new("other creature of the chosen type", Predicate::creature()).excludes_source()',

    # -- Dragon spells --
    '"Dragon spells"': 'Filter::new("Dragon spell", Predicate::HasSubType(SubType::Dragon))',
    '"Dragon spell"': 'Filter::new("Dragon spell", Predicate::HasSubType(SubType::Dragon))',
    '"sorcery spell or Dragon spell"': 'Filter::new("sorcery or Dragon spell", Predicate::sorcery().or(Predicate::HasSubType(SubType::Dragon)))',
    '"spells of chosen type"': 'Filter::new("spell of the chosen type", Predicate::All)',
    '"spells you cast from hand"': 'Filter::new("spell you cast from hand", Predicate::All)',

    # -- Color filters --
    '"green or white creature"': 'Filter::new("green or white creature", Predicate::creature().and(Predicate::Or(vec![Predicate::HasColor(Color::Green), Predicate::HasColor(Color::White)])))',

    # -- Tribal land search filters (Merfolk or Plains or Island, etc.) --
    '"Merfolk or Plains or Island"': 'Filter::new("Merfolk, Plains, or Island card", Predicate::Or(vec![Predicate::HasSubType(SubType::Merfolk), Predicate::HasSubType(SubType::Plains), Predicate::HasSubType(SubType::Island)]))',
    '"Kithkin or Forest or Plains"': 'Filter::new("Kithkin, Forest, or Plains card", Predicate::Or(vec![Predicate::HasSubType(SubType::Kithkin), Predicate::HasSubType(SubType::Forest), Predicate::HasSubType(SubType::Plains)]))',
    '"Goblin or Swamp or Mountain"': 'Filter::new("Goblin, Swamp, or Mountain card", Predicate::Or(vec![Predicate::HasSubType(SubType::Goblin), Predicate::HasSubType(SubType::Swamp), Predicate::HasSubType(SubType::Mountain)]))',
    '"Elf or Swamp or Forest"': 'Filter::new("Elf, Swamp, or Forest card", Predicate::Or(vec![Predicate::HasSubType(SubType::Elf), Predicate::HasSubType(SubType::Swamp), Predicate::HasSubType(SubType::Forest)]))',
    '"Elemental or Island or Mountain"': 'Filter::new("Elemental, Island, or Mountain card", Predicate::Or(vec![Predicate::HasSubType(SubType::Elemental), Predicate::HasSubType(SubType::Island), Predicate::HasSubType(SubType::Mountain)]))',
    '"Elves and Faeries you control"': 'Filter::new("Elf or Faerie you control", Predicate::creature().and(Predicate::Or(vec![Predicate::HasSubType(SubType::Elf), Predicate::HasSubType(SubType::Faerie)])).and(Predicate::Controller(TargetController::You)))',

    # -- Named card filters --
    '"Tempest Hawk"': 'Filter::new("Tempest Hawk", Predicate::NameIs("Tempest Hawk".to_string()))',
    '"Dragonstorm Globe or Boulderborn Dragon"': 'Filter::new("Dragonstorm Globe or Boulderborn Dragon", Predicate::Or(vec![Predicate::NameIs("Dragonstorm Globe".to_string()), Predicate::NameIs("Boulderborn Dragon".to_string())]))',

    # -- Special / complex --
    '"self if creature attacking you"': 'Filter::self_reference()',
    '"creature an opponent controls that was dealt damage this turn"': 'Filter::new("creature an opponent controls", Predicate::creature().and(Predicate::Controller(TargetController::Opponent)))',
    '"creature card with mana value X or less"': 'Filter::new("creature card", Predicate::creature())',
    '"creature you control and creature card in your graveyard"': 'Filter::new("creature you control", Predicate::creature().and(Predicate::Controller(TargetController::You)))',
    '"two creatures you control"': 'Filter::creature_you_control()',

    # -- Graveyard / zone-related (these are used for card matching, not permanent matching) --
    '"Elf cards in your graveyard"': 'Filter::new("Elf card", Predicate::HasSubType(SubType::Elf))',
    '"land cards in your graveyard"': 'Filter::land_card()',
    '"card in opponents\' graveyards"': 'Filter::any_card()',
    '"instants/sorceries in graveyard"': 'Filter::new("instant or sorcery card", Predicate::instant().or(Predicate::sorcery()))',
}

def migrate_file(filepath):
    """Replace Filter::parse("...") calls in a file using REPLACEMENTS map."""
    with open(filepath) as f:
        content = f.read()

    original = content

    # Find all Filter::parse("...") patterns
    pattern = re.compile(r'Filter::parse\(("(?:[^"\\]|\\.)*")\)')

    unmapped = set()

    def replace_match(m):
        key = m.group(1)
        if key in REPLACEMENTS:
            return REPLACEMENTS[key]
        else:
            unmapped.add(key)
            return m.group(0)  # leave unchanged

    content = pattern.sub(replace_match, content)

    if unmapped:
        for u in sorted(unmapped):
            print(f"  UNMAPPED: {u}", file=sys.stderr)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    return False


def main():
    base = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

    # Files to process (skip filters.rs parser definition/tests for now)
    files = [
        'mtg-engine/src/abilities.rs',
        'mtg-engine/src/game.rs',
        'mtg-cards/src/sets/fdn.rs',
        'mtg-cards/src/sets/tla.rs',
        'mtg-cards/src/sets/tdm.rs',
        'mtg-cards/src/sets/ecl.rs',
        'mtg-engine/src/tests/effects.rs',
        'mtg-engine/src/tests/combat.rs',
        'mtg-engine/src/tests/continuous_effects.rs',
        'mtg-engine/src/tests/keywords.rs',
        'mtg-engine/src/tests/costs.rs',
        'mtg-engine/src/tests/special_mechanics.rs',
        'mtg-engine/src/tests/triggers.rs',
        'mtg-engine/src/tests/abilities.rs',
    ]

    total_changed = 0
    for rel in files:
        path = os.path.join(base, rel)
        if not os.path.exists(path):
            print(f"  SKIP (not found): {rel}", file=sys.stderr)
            continue
        changed = migrate_file(path)
        if changed:
            total_changed += 1
            print(f"  Updated: {rel}")
        else:
            print(f"  No changes: {rel}")

    print(f"\nUpdated {total_changed} files")


if __name__ == '__main__':
    main()
