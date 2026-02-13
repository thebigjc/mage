// Set-specific keyword mechanics.
//
// These modules provide builder functions that create properly configured
// `Ability` instances for set-specific keyword mechanics:
//
// - **Mobilize N** (TDM): "Whenever this creature attacks, create N 1/1 red
//   Warrior creature tokens tapped and attacking. Sacrifice them at the
//   beginning of the next end step."
//
// - **Blight N** (ECL): An additional cost that puts N -1/-1 counters on a
//   creature you control.
//
// - **Behold <subtype>** (ECL): An optional additional cost — choose a
//   creature of the named subtype you control or reveal a card of that
//   subtype from your hand. If the behold cost was paid, the spell gets
//   an enhanced effect.

pub mod mobilize;
pub mod blight;
pub mod behold;
