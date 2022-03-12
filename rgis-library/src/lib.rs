pub struct LibraryEntry {
    pub name: &'static str,
    pub bytes: &'static [u8],
}

pub const ENTRIES: [LibraryEntry; 1] = [LibraryEntry {
    name: "USA Boundary",
    bytes: include_bytes!("../../sample-data/us-states.json"),
}];
