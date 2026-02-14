# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

XMage (Magic, Another Game Engine) is a Java implementation of Magic: The Gathering with full rules enforcement for 28,000+ unique cards. It includes a game server, Swing GUI client, AI opponents, and tournament support. The repo also contains `mtg-rl/`, a Rust workspace reimplementing the engine for reinforcement learning research.

## Build Commands (Java)

```bash
# Build without tests (most common during development)
mvn install package -DskipTests
# or
make build

# Run all tests
mvn test

# Run a single test class (must be run from Mage.Tests directory)
mvn test -pl Mage.Tests -Dtest=PowerWordKillTest

# Run a single test method
mvn test -pl Mage.Tests -Dtest=PowerWordKillTest#killGrizzlyBears

# Clean build
mvn clean

# Build with code coverage
mvn install -Djacoco.skip=false -Dmaven.test.failure.ignore=true

# Full clean + build + package
make install
```

Java target is 1.8 but builds require JDK 17+. Maven surefire uses JUnit 5 with JUnit 4 vintage engine.

## Module Architecture

- **Mage/** — Core engine: abilities, effects, game rules, constants, filters, targets, watchers. Package: `mage.*`
- **Mage.Sets/** — All card implementations (~31K files). Cards in `mage.cards.<first-letter>/`, sets in `mage.sets/`
- **Mage.Tests/** — Test suite using custom card testing framework. Tests in `org.mage.test.*`
- **Mage.Server/** — Game server with session, table, game, and tournament management
- **Mage.Client/** — Swing-based GUI client
- **Mage.Common/** — Shared utilities between client and server
- **Mage.Server.Plugins/** — Pluggable game types (Commander, TwoPlayerDuel, FreeForAll, etc.), AI players, deck validators, tournament formats
- **Mage.Verify/** — Card data verification against external sources
- **Utils/** — Perl scripts for card code generation (`gen-card.pl`), test generation (`gen-card-test.pl`), and build packaging

## Card Implementation Pattern

Every card extends `CardImpl` and lives in `Mage.Sets/src/mage/cards/<first-letter>/<CardName>.java`. The class name is the card name with spaces/punctuation removed (& becomes And):

```java
public final class AbruptDecay extends CardImpl {
    private static final FilterPermanent filter = new FilterNonlandPermanent(
            "nonland permanent with mana value 3 or less");
    static {
        filter.add(new ManaValuePredicate(ComparisonType.FEWER_THAN, 4));
    }

    public AbruptDecay(UUID ownerId, CardSetInfo setInfo) {
        super(ownerId, setInfo, new CardType[]{CardType.INSTANT}, "{B}{G}");
        this.addAbility(new CantBeCounteredSourceAbility());
        this.getSpellAbility().addEffect(new DestroyTargetEffect());
        this.getSpellAbility().addTarget(new TargetPermanent(filter));
    }
    private AbruptDecay(final AbruptDecay card) { super(card); }
    @Override
    public AbruptDecay copy() { return new AbruptDecay(this); }
}
```

Key conventions:
- Class must be `final`
- Must have a private copy constructor and `copy()` method
- Creature cards set `this.power`, `this.toughness`, and `this.subtype`
- Keyword abilities use singletons: `FlyingAbility.getInstance()`, `VigilanceAbility.getInstance()`
- For spells, add effects/targets to `this.getSpellAbility()`
- For permanents with activated/triggered/static abilities, use `this.addAbility(...)`
- Static filters should be `private static final` fields, initialized in `static {}` blocks if predicates are needed

## Filter System

Filters are the primary mechanism for targeting and selection. They form a predicate-based hierarchy:

- **FilterPermanent** — Matches battlefield objects. Common subclasses: `FilterCreaturePermanent`, `FilterNonlandPermanent`, `FilterControlledCreaturePermanent`
- **FilterCard** — Matches cards in any zone. Subclasses: `FilterCreatureCard`, `FilterInstantOrSorceryCard`
- **FilterSpell** — Matches spells on the stack
- **FilterPlayer** — Matches players

Compose filters with predicates (`filter.add(predicate)`):
- `ColorPredicate`, `SubtypePredicate`, `ManaValuePredicate`, `CardTypePredicate`
- `AnotherPredicate` — Excludes the source permanent
- Combinators: `Predicates.and()`, `Predicates.or()`, `Predicates.not()`

**StaticFilters** (`mage.filter.StaticFilters`) contains pre-built common filters — always prefer these over creating new instances for frequently used filters like "creature you control" or "nonland permanent".

## Core Abstractions

**Ability hierarchy:** `Ability` → `AbilityImpl` → `ActivatedAbilityImpl` / `TriggeredAbilityImpl` / `StaticAbility` / `SpellAbility`

**Effect hierarchy:** `Effect` → `OneShotEffect` (one-time) / `ContinuousEffect` (ongoing, applied in 7 layers: Copy → Control → Text → Type → Color → Ability → P/T)

**Common ability classes** (in `mage.abilities.common`):
- Triggered: `EntersBattlefieldTriggeredAbility`, `DiesSourceTriggeredAbility`, `AttacksTriggeredAbility`, `BeginningOfUpkeepTriggeredAbility`
- Activated: `SimpleActivatedAbility` (with Zone, Effect, Cost)
- Static: `SimpleStaticAbility` (with Zone, ContinuousEffect)

**Key types in `mage.constants`:**
- `CardType`, `SubType`, `SuperType` — Card classification
- `Zone` — Hand, Battlefield, Graveyard, Library, Exile, Stack, Command
- `PhaseStep` — Turn steps (UNTAP, UPKEEP, DRAW, PRECOMBAT_MAIN, etc.)

**SubType usage:** In card code use `SubType.byDescription("Human")`. The `SubType.fromString()` method is for tests only.

## Test Framework

Tests extend `CardTestPlayerBase` (2-player duel) and use a declarative API:

```java
public class MyCardTest extends CardTestPlayerBase {
    @Test
    public void testBasicFunction() {
        addCard(Zone.HAND, playerA, "My Card");
        addCard(Zone.BATTLEFIELD, playerA, "Swamp", 2);
        addCard(Zone.BATTLEFIELD, playerB, "Grizzly Bears");

        castSpell(1, PhaseStep.PRECOMBAT_MAIN, playerA, "My Card", "Grizzly Bears");

        setStopAt(1, PhaseStep.POSTCOMBAT_MAIN);
        setStrictChooseMode(true);
        execute();

        assertGraveyardCount(playerB, "Grizzly Bears", 1);
    }
}
```

Key test API methods:
- `addCard(Zone, player, cardName)` / `addCard(Zone, player, cardName, count)` — Set up game state
- `castSpell(turn, step, player, spellName)` / `castSpell(turn, step, player, spellName, targetName)` — Cast spells
- `activateAbility(turn, step, player, abilityText)` — Activate abilities
- `attack(turn, player, attackerName)` / `block(turn, player, blockerName, attackerName)` — Combat
- `setChoice(player, "Yes"/"No"/choiceString)` — Make choices
- `setModeChoice(player, "1"/"2"/etc)` — Choose modes for modal spells
- `setStopAt(turn, step)` + `execute()` — Run game to specified point
- `assertLife(player, expected)`, `assertPermanentCount(player, cardName, count)`, `assertGraveyardCount(player, cardName, count)`, `assertHandCount(player, count)`, `assertPowerToughness(player, cardName, power, toughness)`
- `checkPlayableAbility(checkName, turn, step, player, abilityStartText, shouldBePlayable)` — Verify ability availability

Tests live in `Mage.Tests/src/test/java/org/mage/test/cards/single/<set-code>/` organized by set abbreviation.

## Code Generation (Utils/)

Perl scripts generate card and test boilerplate from a card database (`Utils/mtg-cards-data.txt`, 12.5 MB):

```bash
# Generate a card implementation file
cd Utils && perl gen-card.pl "Power Word Kill"

# Generate a test skeleton (first arg is card under test, rest are supporting cards)
cd Utils && perl gen-card-test.pl "Power Word Kill" "Grizzly Bears"
```

`gen-card.pl` creates the Java file with correct class name, mana cost, types, and P/T from the database. `gen-card-test.pl` creates a test class under the appropriate `single/<set-code>/` directory. Both support partial name matching.

## Set Registration

Each set is a singleton extending `ExpansionSet` in `Mage.Sets/src/mage/sets/`. Cards are registered via `SetCardInfo` entries referencing the card class. The card database scan discovers cards by class reference, not manual listing.

## Event System

The game uses an event-driven architecture: `GameEvent` types (600+) drive `TriggeredAbility` and `Watcher` interactions. Effects can be `ReplacementEffect` (modifying events before they happen) or respond after events fire.

## Plugin System

Game variants, AI players, deck validators, and tournament types are plugins in `Mage.Server.Plugins/`. Each game variant extends `GameImpl` and registers a `MatchType`. Plugins are loaded via `PluginClassLoader`.

---

## mtg-rl — Rust Workspace

A Rust reimplementation of the MTG engine for reinforcement learning, located at `mtg-rl/`.

### Build Commands (Rust)

```bash
cd mtg-rl

# Check compilation (fast feedback)
cargo check
cargo check -p mtg-cards
cargo check -p mtg-engine

# Build
cargo build
cargo build --release

# Run tests
cargo test --lib
cargo test --release

# Benchmarks (Criterion)
cargo bench --bench game_bench
```

### Crate Architecture

```
mtg-engine   — Core game engine: game loop, state, combat, abilities, effects, zones
mtg-cards    — Card factory registry + set implementations (1,333 cards across 4 sets)
mtg-ai       — AI players: random, heuristic, minimax + Gymnasium environment
mtg-tests    — Integration test framework + Criterion benchmarks
mtg-python   — PyO3 bindings for Python/Gymnasium RL training
```

Dependency flow: `mtg-engine` ← `mtg-cards` ← `mtg-ai` ← `mtg-tests` / `mtg-python`

### Card Sets

| Set | Code | Cards |
|-----|------|-------|
| Foundations | FDN | 512 |
| Avatar: The Last Airbender | TLA | 280 |
| Tolkien | TDM | 273 |
| Eclogue | ECL | 268 |

Card factories are registered in `mtg-cards/src/sets/{fdn,tla,tdm,ecl}.rs` and collected by `CardRegistry::with_all_sets()` in `mtg-cards/src/registry.rs`.

### Key Engine Types (mtg-engine)

- **Game** (`game.rs`) — Main game loop with `execute()`, effect resolution, state-based actions
- **Effect** (`abilities.rs`) — Enum with ~30 variants: `DealDamage`, `Destroy`, `Exile`, `Bounce`, `DrawCards`, `GainLife`, `CreateToken`, `AddCounters`, `CounterSpell`, etc. Uses `Effect::Custom(String)` as fallback for unimplemented effects
- **StaticEffect** (`abilities.rs`) — Continuous effects: `Boost`, `GrantKeyword`, `CantBlock`
- **KeywordAbilities** (`constants.rs`) — Bitflags struct (FLYING, DEATHTOUCH, FIRST_STRIKE, etc.)
- **ManaCost** (`mana.rs`) — Parsed from strings like `"{1}{U}{B}"`
- **SubType** (`constants.rs`) — Enum with common types + `SubType::Custom("Name".into())` fallback

### Rust Conventions

- All types are `Send + Sync` (verified at compile time via `static_assertions`)
- Immutable data structures via the `im` crate
- Card factory signature: `fn(ObjectId, PlayerId) -> CardData`
- ~88 games/sec single-threaded, ~585/sec with rayon parallelism
