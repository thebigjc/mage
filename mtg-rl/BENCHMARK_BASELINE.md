# Benchmark Baseline

Captured: 2026-02-16
Rust version: see `rustc --version`
Platform: Linux 6.6.87.2-microsoft-standard-WSL2
Profile: release (optimized)

## Results

| Benchmark | Time (mean) | Range |
|-----------|-------------|-------|
| game_state_clone | 8.21 µs | [8.18 µs, 8.24 µs] |
| game_state_clone_empty | 7.65 µs | [7.62 µs, 7.69 µs] |
| sba_check | 598 ns | [598 ns, 599 ns] |
| full_game_simulation | 1.67 ms | [1.64 ms, 1.69 ms] |
| gym_env_step | 564 ns | [564 ns, 566 ns] |
| gym_env_reset | 1.26 µs | [1.26 µs, 1.27 µs] |
| parallel_games/sequential_10 | 16.78 ms | [16.42 ms, 17.14 ms] |
| parallel_games/parallel_10 | 4.72 ms | [4.64 ms, 4.80 ms] |

## Derived Metrics

- Single-threaded throughput: ~599 games/sec (1/1.67ms)
- Parallel throughput (10 games): ~2,119 games/sec (10/4.72ms)
- Parallelism speedup: ~3.56x (sequential_10 / parallel_10)

## Test Counts

- mtg-engine: 576 passed
- mtg-tests: 19 passed
- Total: 595 passed
