use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::definitions::CommandDefinition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommandDefinitionCommand {
    pub name: String,
    pub module: String,
    pub params_type: Option<String>,
    pub attributes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub struct ParsedCommandDefinition {
    pub name: String,
    pub commands: Vec<ParsedCommandDefinitionCommand>,
    pub attributes: Vec<String>,
}

pub fn parse_command_definition(cmd_def: &CommandDefinition) -> Result<ParsedCommandDefinition, Box<dyn std::error::Error>> {
    let command_enum_pattern = Regex::new(r"(\w+)\.(\w+)\s*//")?;
    let mut commands = Vec::new();
    
    // Extract command names from the enum definition
    for line in cmd_def.content.lines() {
        let line = line.trim();
        
        if let Some(captures) = command_enum_pattern.captures(line) {
            let module = captures[1].to_string(); // Extract module name (before the dot)
            let name = captures[2].to_string(); // Extract command name (after the dot)
            let params_type = extract_params_type(line)?;
            let attributes = extract_attributes(line)?;
            
            commands.push(ParsedCommandDefinitionCommand {
                name,
                module,
                params_type,
                attributes,
            });
        }
    }
    
    // Extract attributes for the overall definition
    let definition_attributes = vec![
        "#[derive(Debug, Serialize, Deserialize)]".to_string(),
        "#[serde(untagged)]".to_string(),
    ];
    
    Ok(ParsedCommandDefinition {
        name: cmd_def.name.clone(),
        commands,
        attributes: definition_attributes,
    })
}

fn extract_params_type(line: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let params_pattern = Regex::new(r#"params:\s*(\w+(?:\.\w+)*)"#)?;
    
    if let Some(captures) = params_pattern.captures(line) {
        Ok(Some(captures[1].to_string()))
    } else {
        Ok(None)
    }
}

fn extract_attributes(line: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut attributes = Vec::new();
    
    // Extract attributes from comments or other patterns as needed
    // For now, return empty vector - can be expanded based on your specific format
    
    Ok(attributes)
}