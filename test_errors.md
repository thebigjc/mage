There are a number of warnings in our test suite. We need to work through them one at a time and resolve them. If they require implementing new functionality, we can skip them (such as 'never used' warnings), but otherwise we should do our best to clean them up.

They occur when we run cargo test --lib -p mtg-engine --no-run 2>&1
