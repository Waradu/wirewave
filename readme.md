# WireWave

Use [Wave API](https://wireway.ch) Wave API to fetch music by search query

Example:

```rs
let music_items = WaveMusic::new("example search term".to_string());
for item in music_items.unwrap() {
    println!("{}", item);
}
```
