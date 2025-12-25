#[derive(Debug)]
#[allow(dead_code)]
pub struct EventDefinition {
    pub name: String,
    pub content: String,
}

pub fn extract_definition_content(content: &str, target_line: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lines: Vec<&str> = content.lines().collect();
    let mut definition = String::new();
    let mut found_start = false;
    let mut paren_count = 0;
    
    for line in lines {
        if line.trim() == target_line.trim() {
            found_start = true;
        }
        
        if found_start {
            definition.push_str(line);
            definition.push('\n');
            
            // Count parentheses to find the end
            for ch in line.chars() {
                match ch {
                    '(' => paren_count += 1,
                    ')' => {
                        paren_count -= 1;
                        if paren_count == 0 {
                            return Ok(definition.trim().to_string());
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    Ok(definition.trim().to_string())
}