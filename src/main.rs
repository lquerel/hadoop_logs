use walkdir::WalkDir;

fn main() {
    let root = "/Users/L.Querel/Downloads/Hadoop";
    let log_regex = regex::Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2},\d{3}) (INFO|DEBUG|ERROR|WARN) (.*)").unwrap();

    println!("ts,level,body");
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext.to_str() == Some("log") {
                    // read text file line by line
                    let contents = std::fs::read_to_string(entry.path()).unwrap();
                    for line in contents.lines() {
                        if let Some(captures) = log_regex.captures(line) {
                            // convert datetime to timestamp
                            let datetime = captures.get(1).unwrap().as_str();
                            let timestamp = chrono::NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S,%3f").unwrap().timestamp_nanos();
                            // get log level
                            let level = captures.get(2).unwrap().as_str();
                            // get log message
                            let message = captures.get(3).unwrap().as_str();
                            // print log
                            println!("\"{}\",\"{}\",\"{}\"", timestamp, level, message.replace("\"", "\"\""));
                        }
                    }
                }
            }
        }
    }
}
