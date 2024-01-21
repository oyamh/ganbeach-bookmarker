pub struct DatabaseConfig {
    version: u32,
    tables: Vec<TableConfig>,
}

impl DatabaseConfig {
    pub fn new(version: u32) -> Self {
        Self {
            version,
            tables: Vec::new(),
        }
    }

    pub fn with(mut self, table_config: TableConfig) -> Self {
        self.tables.push(table_config);
        self
    }

    pub fn new_table_config(&self, name: &'static str) -> TableConfig {
        TableConfig::new(self.version, name)
    }
}

impl IntoIterator for DatabaseConfig {
    type Item = TableConfig;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.tables.into_iter()
    }
}

#[derive(Debug, Default, Clone)]
pub struct TableConfig {
    version: u32,
    name: &'static str,
    primary_key: Option<&'static str>,
    index_keys: Vec<&'static str>,
}

impl TableConfig {
    pub fn new(version: u32, name: &'static str) -> Self {
        Self {
            version,
            name,
            ..Default::default()
        }
    }

    pub fn with_primary_key(mut self, primary_key: &'static str) -> Self {
        self.primary_key = Some(primary_key);
        self
    }

    pub fn with_index_keys(mut self, index_keys: Vec<&'static str>) -> Self {
        self.index_keys = index_keys;
        self
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn primary_key(&self) -> Option<&'static str> {
        self.primary_key
    }

    pub fn index_keys(&self) -> Vec<&'static str> {
        self.index_keys.to_owned()
    }
}
