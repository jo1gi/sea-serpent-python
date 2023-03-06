mod errors;

use pyo3::prelude::*;
use std::{
    path::PathBuf,
    collections::{HashSet, HashMap},
    str::FromStr,
};

#[pymodule]
fn sea_serpent(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Database>()?;
    m.add_class::<SearchResult>()?;
    Ok(())
}

#[pyclass]
struct Database{
    inner: seaserpent::database::Database,
}

#[pymethods]
impl Database {

    #[new]
    fn new(database_path: Option<PathBuf>) -> Result<Self, errors::SeaSerpentError> {
        Ok(Database {
            inner: match database_path {
                Some(path) => seaserpent::database::Database::load(path)?,
                None => seaserpent::database::Database::load_from_current_dir()?,
            }
        })
    }

    fn add_tag(&mut self, path: PathBuf, tag: String) -> Result<(), errors::SeaSerpentError> {
        Ok(self.inner.add_tag(&path, &tag)?)
    }

    fn remove_tag(&mut self, path: PathBuf, tag: String) -> Result<(), errors::SeaSerpentError> {
        Ok(self.inner.remove_tag(&path, &tag)?)
    }

    fn save(&self) -> Result<(), errors::SeaSerpentError> {
        Ok(self.inner.save()?)
    }

    fn search(&self, search_string: String) -> Result<Vec<SearchResult>, errors::SeaSerpentError> {
        let search_expression = seaserpent::search::parse(&search_string)?;
        let results = self.inner.search(search_expression).iter()
            .map(|result| result.into())
            .collect();
        Ok(results)
    }

    fn file_info(&self, path: PathBuf) -> Result<SearchResult, errors::SeaSerpentError> {
        let file_info = &self.inner.get_file_info(&path)?;
        Ok(file_info.into())
    }

    fn move_file(&mut self, old_path: PathBuf, template: String) -> Result<(), errors::SeaSerpentError> {
        let fileinfo = self.inner.get_file_info(&old_path)?;
        let new_path_str = seaserpent::format::format_result(&fileinfo, &template)
            .map_err(|_| errors::SeaSerpentError::Formatting)?;
        let new_path = std::path::PathBuf::from_str(&new_path_str).unwrap();
        self.inner.move_file(&old_path, &new_path)?;
        Ok(())
    }
}


#[pyclass]
struct SearchResult {
    #[pyo3(get)]
    path: PathBuf,
    #[pyo3(get)]
    tags: HashSet<String>,
    #[pyo3(get)]
    attributes: HashMap<String, HashSet<String>>,
}

#[pymethods]
impl SearchResult {
    fn __repr__(&self) -> String {
        format!("SearchResult({})", self.path.display())
    }
}

impl From<&seaserpent::database::SearchResult<'_>> for SearchResult {
    fn from(result: &seaserpent::database::SearchResult) -> Self {
        Self {
            path: result.path.clone(),
            tags: result.tags.clone(),
            attributes: result.attributes.clone()
        }
    }
}
