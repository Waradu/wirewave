# WireWave

Use [Wave API](https://wireway.ch) Wave API to fetch music by search query

### Example:

Get Music vec by search query:
```rs
let music_items = WaveMusic::new("example search term".to_string());
for item in music_items.unwrap() {
    println!("{}", item);
}
```

Get and save thumpnail from music:
```rs
let music_items = WaveMusic::new("example search term".to_string()).unwrap();
if let Some(item) = music_items.first() {
    let mut thumbnail_response = item.thumbnail().unwrap();
    let name = format!("{}.png", item.id.as_ref().unwrap());
    let path = Path::new(&name);
    let mut file = File::create(&path).unwrap();
    copy(&mut thumbnail_response, &mut file).unwrap();
    println!("Image fetched and saved to {:?}", path);
}
```
