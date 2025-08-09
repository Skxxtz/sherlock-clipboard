use regex::Regex;
use serde::Serialize;
use serde_json;
use std::process::Command;
fn main() {
    // Execute 'cliphist list' and capture its output
    let output = Command::new("cliphist")
        .arg("list")
        .output()
        .expect("Failed to execute 'cliphist list'");

    // Convert the stdout (in bytes) to a string and then lines
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.split('\n');

    // Construct regex
    let pattern = r"^\d*\s*\[\[ binary data.*\]\]$";
    let re = Regex::new(pattern).unwrap();

    // Iterate through the lines and process
    let data: Vec<PipeData> = lines
        .take(100)
        .map(|line| {
            let mut title = None;
            let mut binary: Option<Vec<u8>> = None;
            let mut result = None;

            // Simple check to see if it's an image
            if re.is_match(line) {
                if let Some((id, t)) = line.split_once("\t") {
                    let decoded_output = Command::new("cliphist")
                        .arg("decode")
                        .arg(id)
                        .output()
                        .expect("Failed to execute 'cliphist decode'");

                    // Data for struct
                    title = Some(t.to_string());
                    binary = Some(decoded_output.stdout);
                    result = Some(id.to_string());
                }
            } else {
                if let Some((id, t)) = line.split_once("\t") {
                    title = Some(t.to_string());
                    result = Some(id.to_string());
                }
            }
            // Return a PipeData instance
            PipeData {
                title,
                result,
                binary,
                icon_size: Some(100),
            }
        })
        .collect();

    // Serialize to json and print
    let res = PipeResult::new(data);

    let json = serde_json::to_string(&res).expect("failed to serialize");
    print!("{}", json);
}

#[derive(Debug, Serialize, Clone)]
pub struct PipeData {
    pub title: Option<String>,
    pub result: Option<String>,
    pub binary: Option<Vec<u8>>,
    pub icon_size: Option<i32>,
}
#[derive(Debug, Serialize, Clone)]
pub struct PipeResult {
    settings: Vec<String>,
    elements: Vec<PipeData>,
}
impl PipeResult {
    fn new(data: Vec<PipeData>) -> Self {
        Self {
            settings: Vec::new(),
            elements: data,
        }
    }
}
