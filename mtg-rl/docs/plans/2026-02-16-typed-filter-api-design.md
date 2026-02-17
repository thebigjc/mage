# Typed Filter API Design

**Date:** 2026-02-16
**Status:** Approved

## Problem

The filter system uses `Filter::parse("creature you control")` to convert human-readable strings into `Predicate` trees at card-creation time. While the hot path already evaluates typed `Predicate` enums (not strings), three issues remain:

1. **Correctness**: `Filter::parse("cretaure you control")` silently becomes `SubType::Custom("Cretaure")` — a typo that never matches anything, with no error at any point.
2. **Performance**: `find_matching_permanents` does `msg.contains("enchanted")`, `msg.contains("other")`, `msg.contains("attacking")` — string scans that duplicate information already in the predicate tree.
3. **Ergonomics**: 231 stringly-typed call sites with no IDE completion or refactoring support.

## Design

### Filter Struct

```rust
pub struct Filter {
    /// Human-readable display text for targeting prompts. Display-only, never used for logic.
    pub message: &'static str,
    /// The predicate tree evaluated at runtime.
    pub predicate: Predicate,
    /// This filter refers to the source object ("self", "enchanted creature", "equipped creature").
    pub is_self_referential: bool,
    /// Exclude the source object from matches ("other creature you control").
    pub excludes_source: bool,
    /// Only match attacking permanents ("attacking creature you control").
    pub requires_attacking: bool,
}
```

Key changes from current:
- **`message: &'static str`** replaces `Arc<str>` — zero allocation, 16 bytes, clone is a pointer copy. All filter messages are string literals known at compile time.
- **`predicate: Predicate`** replaces `Arc<Predicate>` — direct ownership, no Arc overhead. Predicate trees are small (1-5 nodes typically); deep clone cost is negligible (~50-200ns). Arc can be reintroduced later if profiling shows need.
- **Removed `message_lower: Arc<str>`** — no longer needed; bool flags replace all string-based logic checks.
- **Added three bool flags** — encode semantic information previously extracted from `msg.contains(...)` at runtime.
- **Dropped `Deserialize`**, kept `Serialize` — filters are always constructed in Rust code, never deserialized from external data.

### Card-Facing API

**Pre-built factories** for the ~20 most common patterns (~60% of call sites):

```rust
Filter::self_reference()                    // "self" (16 uses)
Filter::enchanted_creature()                // "enchanted creature" (13 uses)
Filter::creature_opponent_controls()        // "creature an opponent controls" (8 uses)
Filter::artifact_or_enchantment()           // "artifact or enchantment" (6 uses)
Filter::other_creature_you_control()        // "another creature you control" (6 uses)
Filter::nonland_permanent()                 // "nonland permanent" (5 uses)
Filter::equipped_creature()                 // "equipped creature" (4 uses)
Filter::creature_you_control()              // "creature you control" (4 uses)
```

**Direct construction** for the long tail (~40% of call sites):

```rust
Filter::new("other Elf you control",
    Predicate::creature()
        .and(Predicate::HasSubType(SubType::Elf))
        .and(Predicate::Controller(TargetController::You)))
    .excludes_source()

Filter::new("nonland permanent with mana value 3 or less",
    Predicate::nonland_permanent()
        .and(Predicate::ManaValueCompare(Cmp::Lte, 3)))
```

**Constructor and flag setters:**

```rust
impl Filter {
    pub fn new(message: &'static str, predicate: Predicate) -> Self {
        Filter {
            message,
            predicate,
            is_self_referential: false,
            excludes_source: false,
            requires_attacking: false,
        }
    }

    pub fn excludes_source(mut self) -> Self {
        self.excludes_source = true;
        self
    }

    pub fn requires_attacking(mut self) -> Self {
        self.requires_attacking = true;
        self
    }
}
```

### find_matching_permanents Changes

Before (string checks):
```rust
fn find_matching_permanents(&self, source_id: ObjectId, controller: PlayerId, filter: &Filter) -> Vec<ObjectId> {
    let msg = filter.message_lower();
    if msg == "self" { return vec![source_id]; }
    if msg.contains("enchanted") || msg.contains("equipped") { ... }
    let exclude_self = msg.contains("other");
    let is_attacking = msg.contains("attacking");
    // ...
}
```

After (bool flags):
```rust
fn find_matching_permanents(&self, source_id: ObjectId, controller: PlayerId, filter: &Filter) -> Vec<ObjectId> {
    if filter.is_self_referential {
        return self.resolve_self_referential(source_id, filter);
    }
    self.state.battlefield.iter()
        .filter(|perm| !(filter.excludes_source && perm.id() == source_id))
        .filter(|perm| !filter.requires_attacking || self.state.combat.is_attacking(perm.id()))
        .filter(|perm| filter.matches_permanent(perm, controller))
        .map(|perm| perm.id())
        .collect()
}
```

## Migration Strategy

Each phase is gated by `cargo check && cargo clippy && cargo test --lib`.

**Phase 1: Infrastructure.** Change the Filter struct. Add bool flags, pre-built factories, `Filter::new()`. Update `find_matching_permanents` to use bool flags. Keep `Filter::parse()` temporarily (it sets the new bool flags internally so existing code keeps working).

**Phase 2: Migrate common patterns.** Mechanical search-and-replace for the top ~20 patterns:
```
Filter::parse("self")                           -> Filter::self_reference()
Filter::parse("enchanted creature")             -> Filter::enchanted_creature()
Filter::parse("creature an opponent controls")  -> Filter::creature_opponent_controls()
```

**Phase 3: Migrate long-tail patterns.** Convert remaining ~40% of call sites to `Filter::new(msg, predicate_chain)`. Python script can handle most of this.

**Phase 4: Delete the parser.** Remove `Filter::parse()`, `parse_filter_string()` (~250 lines), `message_lower` field, and related helpers (`depluralize`, `capitalize`).

**Phase 5: Update Python code-gen scripts.** Emit typed constructors instead of `Filter::parse("...")` strings.

## Testing

1. **Equivalence tests (temporary):** During migration, validate that each converted call site produces identical matching behavior to the old `Filter::parse()` path. Deleted in Phase 4 with the parser.
2. **Factory unit tests (permanent):** Each pre-built factory method tested against matching/non-matching permanents.
3. **Bool flag tests (permanent):** `find_matching_permanents` correctly respects `excludes_source`, `requires_attacking`, and `is_self_referential`.

## Scope

- **231 call sites:** 89 in mtg-engine, 142 in mtg-cards
- **74 unique filter strings:** ~20 become pre-built factories, ~54 become `Filter::new()` calls
- **~250 lines deleted:** The parser, `message_lower`, string helpers
- **Net LOC:** Roughly neutral — parser deletion offsets new factory methods
