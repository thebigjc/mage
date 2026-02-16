# Ralph Guardrails — mtg-rl

- Use `sed -i` for bulk mechanical fixes to avoid Edit tool racing with the linter/hook
- Always run `cargo test --lib -p mtg-engine -p mtg-cards` after changes (576 tests expected)
- Card keyword lookups: check Java source in `Mage.Sets/src/mage/cards/<first-letter>/` for ground truth
- `git add -A` picks up untracked files from parent dirs — be careful with what's staged
