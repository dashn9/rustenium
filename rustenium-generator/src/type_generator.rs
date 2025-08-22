use regex::Regex;
use std::collections::HashMap;
use crate::definitions::{CommandDefinition, EventDefinition, ResultDefinition, extract_definition_content};

#[derive(Debug)]
pub struct Command {
    pub raw: String,
}

#[derive(Debug)]
pub struct Event {
    pub raw: String,
}

#[derive(Debug)]
pub struct BidiType {
    pub raw: String,
}

#[derive(Debug)]
pub struct BidiResult {
    pub raw: String,
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub command_definition: Option<CommandDefinition>,
    pub event_definition: Option<EventDefinition>,
    pub result_definition: Option<ResultDefinition>,
    pub commands: Vec<Command>,
    pub events: Vec<Event>,
    pub types: Vec<BidiType>,
    pub results: Vec<BidiResult>,
}

pub fn generate_types(cddl_strings: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let modules = detect_modules(cddl_strings)?;
    
    let mut output = String::new();
    output.push_str("// Detected modules:\n");
    
    for module in modules {
        output.push_str(&format!("// Module: {} (command_def: {}, event_def: {}, result_def: {})\n", 
            module.name, module.command_definition.is_some(), module.event_definition.is_some(), module.result_definition.is_some()));
    }
    
    Ok(output)
}

pub fn detect_modules(cddl_strings: Vec<&str>) -> Result<Vec<Module>, Box<dyn std::error::Error>> {
    let mut modules: HashMap<String, Module> = HashMap::new();
    
    let command_pattern = Regex::new(r"([A-Z]\w*)Command\s*=\s*\(")?;
    let event_pattern = Regex::new(r"([A-Z]\w*)Event\s*=\s*\(")?;
    let result_pattern = Regex::new(r"([A-Z]\w*)Result\s*=\s*\(")?;
    
    for cddl_content in cddl_strings {
        for line in cddl_content.lines() {
            let line = line.trim();
            
            if let Some(captures) = command_pattern.captures(line) {
                let name = captures[1].to_lowercase();
                let content = extract_definition_content(cddl_content, line)?;
                modules.entry(name.clone()).or_insert(Module {
                    name: name.clone(),
                    command_definition: None,
                    event_definition: None,
                    result_definition: None,
                    commands: Vec::new(),
                    events: Vec::new(),
                    types: Vec::new(),
                    results: Vec::new(),
                }).command_definition = Some(CommandDefinition {
                    name: format!("{}Command", name),
                    content,
                });
            }
            
            if let Some(captures) = event_pattern.captures(line) {
                let name = captures[1].to_lowercase();
                let content = extract_definition_content(cddl_content, line)?;
                modules.entry(name.clone()).or_insert(Module {
                    name: name.clone(),
                    command_definition: None,
                    event_definition: None,
                    result_definition: None,
                    commands: Vec::new(),
                    events: Vec::new(),
                    types: Vec::new(),
                    results: Vec::new(),
                }).event_definition = Some(EventDefinition {
                    name: format!("{}Event", name),
                    content,
                });
            }
            
            if let Some(captures) = result_pattern.captures(line) {
                let name = captures[1].to_lowercase();
                let content = extract_definition_content(cddl_content, line)?;
                modules.entry(name.clone()).or_insert(Module {
                    name: name.clone(),
                    command_definition: None,
                    event_definition: None,
                    result_definition: None,
                    commands: Vec::new(),
                    events: Vec::new(),
                    types: Vec::new(),
                    results: Vec::new(),
                }).result_definition = Some(ResultDefinition {
                    name: format!("{}Result", name),
                    content,
                });
            }
        }
    }
    
    let mut result: Vec<Module> = modules.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(result)
}


