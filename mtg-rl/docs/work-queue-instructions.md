# Work Queue Instructions

Paste this into a new Claude Code session to run the card remediation loop:

---

Read `docs/work-queue.md` and do the next unchecked batch. For each batch:

1. Read the engine files (`mtg-engine/src/abilities.rs`, `mtg-engine/src/game.rs`) to understand current Effect/StaticEffect variants
2. If the batch has a NOTE about uncertainty, investigate first and update the queue with findings before committing the fix
3. If engine work is needed: add the variant, constructor, and match arm, and look for the Java implementation to see the bigger picture and understand nuance
4. Add a test for any new engine effect (follow the pattern in `game.rs` `mod tests`)
5. Fix all cards listed in the batch (grep to find exact lines, fix across all 4 set files)
6. `cargo check -p mtg-cards && cargo test --lib`
7. Update docs: mark cards in `docs/{fdn,tla,tdm,ecl}-remediation.md`, update `ROADMAP.md`
8. Check off the batch in `docs/work-queue.md` with today's date
9. If you discover new cards that fit the current or a future batch while working, add them to the queue
10. Double check that the implementations work similar to how the Java engine works

If a batch turns out to be wrong or more complex than described, update the queue with what you learned and move on to the next one.

Then do the next batch. Repeat until context is getting long, then stop and make sure `work-queue.md` is fully current before ending.
