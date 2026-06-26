use crate::config::CharacterEntry;
use crate::history::History;

fn compute_score(query: &str, entry: &CharacterEntry, history: &History) -> u32 {
    let usage = history.get_count(&entry.symbol);

    if query.is_empty() {
        return usage;
    }

    let match_priority = entry
        .tags
        .iter()
        .map(|tag| {
            let tag = tag.to_lowercase();
            if tag == query { 4 }
            else if tag.starts_with(query) { 3 }
            else if tag.contains(query) { 2 }
            else { 0 }
        })
        .max()
        .unwrap_or(0)
        // Name match is a fallback when no tag matches.
        .max(if entry.name.to_lowercase().contains(query) { 1 } else { 0 });

    if match_priority == 0 {
        return 0;
    }

    // Multiplier of 1000 ensures match quality always dominates over usage frequency.
    // Usage only breaks ties within the same match tier.
    match_priority * 1000 + usage
}

pub fn search(
    query: &str,
    characters: &[CharacterEntry],
    history: &History,
) -> Vec<CharacterEntry> {
    let query = query.trim().to_lowercase();

    let mut results: Vec<(u32, CharacterEntry)> = characters
        .iter()
        .filter_map(|entry| {
            let score = compute_score(&query, entry, history);
            if score > 0 || query.is_empty() {
                Some((score, entry.clone()))
            } else {
                None
            }
        })
        .collect();

    // Descending order: highest score (best match + most used) comes first.
    results.sort_by(|a, b| b.0.cmp(&a.0));

    results.into_iter().map(|(_, entry)| entry).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CharacterEntry;
    use crate::history::History;

    fn make_entry(symbol: &str, name: &str, tags: &[&str]) -> CharacterEntry {
        CharacterEntry {
            symbol: symbol.to_string(),
            name: name.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
        }
    }

    #[test]
    fn test_exact_tag_wins() {
        let entries = vec![
            make_entry("→", "right arrow", &["arrow", "right"]),
            make_entry("≈", "approximately equal", &["approx", "math"]),
        ];
        let history = History::default();
        let results = search("arrow", &entries, &history);
        assert_eq!(results[0].symbol, "→");
    }

    #[test]
    fn test_empty_query_returns_all() {
        let entries = vec![
            make_entry("→", "right arrow", &["arrow"]),
            make_entry("α", "alpha", &["greek"]),
        ];
        let history = History::default();
        let results = search("", &entries, &history);
        assert_eq!(results.len(), 2);
    }
}
