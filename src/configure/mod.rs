use std::error::Error;
use std::fs;
use std::path::Path;

use serde_json::Value;

pub mod http_jobs;

/// Retrieves the configuration from a file.
///
/// This function reads the content from the file and determines the file format based on the file extension.
/// It supports JSON, YAML, and YML file formats.
///
/// # Errors
///
/// This function may return an error if:
/// - The file doesn't exist or cannot be read.
/// - The file format is not supported.
/// - There are multiple files with conflicting extensions.
/// - An error occurs while parsing the file content.
///
/// # Returns
///
/// The configuration value extracted from the file.
///
/// # Examples
///
/// ```
/// match get_configure() {
///     Ok(config) => {
///         // Use the configuration
///         println!("Configuration: {:?}", config);
///     },
///     Err(err) => {
///         eprintln!("Failed to retrieve configuration: {}", err);
///     },
/// }
/// ```
pub fn get_configure() -> Result<Value, Box<dyn Error>> {

    let file_content = get_jobs_file_content()?;

    let configure = match file_content {
        FileContent::Json(content) => serde_json::from_str::<Value>(&content)
            .map_err(|e| {format!("An error occurred while parsing the 'jobs.json' file: {}", e)})?,
        FileContent::Yaml(content) => serde_yaml::from_str::<Value>(&content)
            .map_err(|e| {format!("An error occurred while parsing the 'jobs.yaml' file: {}", e)})?,
        FileContent::Yml(content) => serde_yaml::from_str::<Value>(&content)
            .map_err(|e| {format!("An error occurred while parsing the 'jobs.yml' file: {}", e)})?,
        FileContent::None => return Err("No 'jobs' file found.".into()),
    };

    Ok(configure)
}

/// Reads the content of the specified file.
///
/// # Arguments
///
/// * `file_path` - The path of the file to read.
///
/// # Errors
///
/// Returns an error if:
/// * The file fails to be read.
///
/// # Examples
///
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let content = read_file("./jobs.json")?;
/// println!("File content: {}", content);
/// #
/// #     Ok(())
/// # }
/// ```
fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(file_path)
        .map_err(|e| format!("An error occurred while reading the file '{}': {}", file_path, e).into())
}

/// Retrieves the content of the 'jobs' file.
///
/// This function searches for the 'jobs' file in different formats (JSON, YAML, YML) in the current directory.
/// It returns the content of the first file found, and determines the file format based on the file extension.
///
/// # Errors
///
/// This function may return an error if:
/// - No 'jobs' file is found.
/// - Multiple 'jobs' files with conflicting extensions are found.
/// - An error occurs while reading or processing the file.
///
/// # Returns
///
/// The content of the 'jobs' file, wrapped in a `FileContent` enum that represents the file format.
///
/// # Examples
///
/// ```
/// match get_jobs_file_content() {
///     Ok(content) => {
///         // Process the content
///         println!("File content: {:?}", content);
///     },
///     Err(err) => {
///         eprintln!("Failed to retrieve 'jobs' file content: {}", err);
///     },
/// }
/// ```
fn get_jobs_file_content() -> Result<FileContent, Box<dyn Error>> {
    let files = ["./jobs.json", "./jobs.yaml", "./jobs.yml"];

    let mut content: FileContent = FileContent::new_none();
    let mut count = 0;

    for file in &files {
        if fs::metadata(file).is_ok() {
            if count > 0 {
                return Err("Multiple 'jobs' files exist. Please ensure only one file is present.".into());
            }
            content = FileContent::from(read_file(file)?, file);
            count += 1;
        }
    }

    if count == 0 {
        return Err("No 'jobs' file found.".into());
    }

    Ok(content)
}

/// Represents the content of a file in different formats (JSON, YAML, YML).
///
/// The `FileContent` enum has three variants, each corresponding to a specific file format.
///
/// - `Json`: Represents the file content as a JSON string.
/// - `Yaml`: Represents the file content as a YAML string.
/// - `Yml`: Represents the file content as a YML string.
///
/// # Examples
///
/// ```
/// let json_content = FileContent::Json("{ \"name\": \"John\", \"age\": 30 }".into());
/// let yaml_content = FileContent::Yaml("name: John\nage: 30".into());
/// let yml_content = FileContent::Yml("name: John\nage: 30".into());
///
/// match json_content {
///     FileContent::Json(content) => {
///         // Process JSON content
///         println!("JSON content: {}", content);
///     },
///     _ => unreachable!(),
/// }
/// ```
enum FileContent {
    Json(String),
    Yaml(String),
    Yml(String),
    None
}

impl FileContent {

    /// Creates a new `FileContent` variant with the value set to `None`.
    ///
    /// This can be used to represent an empty file content.
    ///
    /// # Examples
    ///
    /// ```
    /// let none_content = FileContent::new_none();
    /// ```
    fn new_none() -> Self {
        FileContent::None
    }

    /// Creates a new `FileContent` variant based on the provided content and file extension.
    ///
    /// The file extension is used to determine the appropriate variant of `FileContent`.
    /// If the file extension is not recognized, the default variant is `Json`.
    ///
    /// # Arguments
    ///
    /// * `content`: A string representing the content of the file.
    /// * `file`: The file path or name from which the content originated.
    ///
    /// # Examples
    ///
    /// ```
    /// let content = "{ \"name\": \"John\", \"age\": 30 }".into();
    /// let json_content = FileContent::from(content, "data.json");
    /// ```
    fn from(content: String, file: &str) -> Self {
        let file_extension = get_file_extension(file);
        match file_extension {
            Some("json") => FileContent::Json(content),
            Some("yaml") => FileContent::Yaml(content),
            Some("yml") => FileContent::Yml(content),
            _ => FileContent::Json(content)
        }
    }
}

/// Get the file extension from the given file path.
/// Returns `Some(extension)` if the file has an extension, or `None` if it doesn't.
///
/// # Arguments
///
/// * `file` - A string representing the file path.
///
/// # Examples
///
/// ```
/// let file_path = "example.json";
/// let extension = get_file_extension(file_path);
/// println!("File extension: {:?}", extension);
/// ```
fn get_file_extension(file: &str) -> Option<&str> {
    Path::new(file)
        .extension()
        .and_then(|ext| ext.to_str())
}