use std::{collections::HashMap, ffi::OsString, fs::read_to_string};

// TODO new crate

pub(crate) struct EditorConfigResolver {
    cache: HashMap<OsString, EditorConfig>,
}

impl EditorConfigResolver {
    pub(crate) fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    /**
     * Expects absolute paths
     */
    pub(crate) fn resolve(&mut self, path: &std::path::Path) -> EditorConfigSettings {
        // ancestors is a list of ancestors i.e. [./path/subpath, ./path, ./]
        let ancestors: Vec<&std::path::Path> = path.ancestors().skip(1).collect();

        let mut parsed: HashMap<&std::path::Path, EditorConfig> =
            HashMap::with_capacity(ancestors.len());

        let mut first_uncached_index = 0;

        for (index, ancestor) in ancestors.iter().enumerate() {
            // We keep going up the ancestors until we find a cached value
            if let Some(entry) = self.cache.get(ancestor.as_os_str()) {
                parsed.insert(ancestor, entry.clone());
                break;
            }

            first_uncached_index = index + 1;

            // If it is not cached, we must parse it
            if let Ok(content) = read_to_string(ancestor.join(".editorconfig")) {
                // TODO should we handle not existing different from permissions type errors?

                let parse_result = parse(&content);

                let root = parse_result.root;

                parsed.insert(ancestor, parse_result);

                // If it is a root, then we stop traversing upwards
                if root {
                    break;
                }
            }
        }

        let mut previous = ancestors
            .get(first_uncached_index)
            .and_then(|ancestor| self.cache.get(ancestor.as_os_str()))
            .map(|config| config.clone())
            .unwrap_or(EditorConfig::default());

        // Now we iterate the ancestors in reverse and resolve
        // TODO if we immediately get a cache hit...
        for current_path in ancestors.iter().rev().skip(ancestors.len() - first_uncached_index) {
            let current = &parsed[current_path];

            let combined = EditorConfig::combine(&previous, current);

            self.cache
                .insert(current_path.as_os_str().to_os_string(), combined.clone());

            previous = combined;
        }

        let config = &self.cache[ancestors.first().unwrap().as_os_str()];

        config.resolve(path)
    }
}

fn parse(content: &str) -> EditorConfig {
    let mut lines: Vec<&str> = content.lines().collect();

    let mut blocks = Vec::new();

    while let Some(block_index) = lines.iter().rposition(|line| line.starts_with("[")) {
        let block_lines = lines.split_off(block_index);

        let criteria = block_lines
            .first()
            .and_then(|str| str.strip_prefix("["))
            .and_then(|str| str.strip_suffix("]"))
            .unwrap()
            .to_owned();

        let parsed = parse_key_value(&block_lines);

        let as_settings = EditorConfigSettings::from(parsed);

        blocks.push((criteria, as_settings));
    }

    let top = parse_key_value(&lines);

    EditorConfig {
        root: top
            .get("root")
            .map(|value| *value == "true")
            .unwrap_or(false),
        blocks: blocks,
    }
}

fn parse_key_value<'lifetime>(
    content: &Vec<&'lifetime str>,
) -> HashMap<&'lifetime str, &'lifetime str> {
    content
        .iter()
        .filter(|line| !line.starts_with("#") && !line.starts_with(";"))
        .filter(|line| line.contains("="))
        .map(|line| line.split(" = "))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .collect()
}

// TODO do we need clone?
#[derive(Default, Clone)]
pub(crate) struct EditorConfig {
    root: bool,
    blocks: Vec<(String, EditorConfigSettings)>,
}

impl EditorConfig {
    fn combine(first: &EditorConfig, second: &EditorConfig) -> EditorConfig {
        EditorConfig {
            root: second.root,
            blocks: vec![second.blocks.iter(), first.blocks.iter()]
                .into_iter()
                .flatten()
                .map(|entry| entry.clone())
                .collect(),
        }
    }

    fn resolve(&self, path: &std::path::Path) -> EditorConfigSettings {
        self.blocks
            .iter()
            .filter(|(rule, _)| true)
            .map(|pair| &pair.1)
            .fold(EditorConfigSettings::default(), |first, second| {
                EditorConfigSettings::combine(&first, second)
            })
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct EditorConfigSettings {
    pub(crate) indent_size: Option<usize>,
}

impl EditorConfigSettings {
    fn combine(
        first: &EditorConfigSettings,
        second: &EditorConfigSettings,
    ) -> EditorConfigSettings {
        EditorConfigSettings { 
            indent_size: second.indent_size.or(first.indent_size)
        }
    }
}

impl From<HashMap<&str, &str>> for EditorConfigSettings {
    fn from(map: HashMap<&str, &str>) -> Self {
        EditorConfigSettings {
            indent_size: map.get("indent_size").and_then(|int| (**int).parse().ok()),
        }
    }
}
