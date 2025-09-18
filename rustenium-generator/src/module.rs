use regex::Regex;
use std::collections::HashMap;
use crate::extractor::extract_definition_content;
use crate::command_parser::{CommandDefinition, parse_command_definition, Command};
use crate::event_parser::EventDefinition;
use crate::result_parser::ResultDefinition;


#[derive(Debug)]
pub struct Event {
    pub raw: String,
}

#[derive(Debug)]
pub struct BidiType {
    pub name: String,
    pub properties: Vec<crate::parser::Property>,
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


pub fn detect_modules(cddl_strings: Vec<&str>) -> Result<Vec<Module>, Box<dyn std::error::Error>> {
    let mut modules: HashMap<String, Module> = HashMap::new();
    
    let command_pattern = Regex::new(r"([A-Z]\w*)Command\s*=\s*\(")?;
    let event_pattern = Regex::new(r"([A-Z]\w*)Event\s*=\s*\(")?;
    let result_pattern = Regex::new(r"([A-Z]\w*)Result\s*=\s*\(")?;
    
    for cddl_content in cddl_strings.clone() {
        for line in cddl_content.lines() {
            let line = line.trim();
            
            if let Some(captures) = command_pattern.captures(line) {
                let name = captures[1].to_string();
                let content = extract_definition_content(cddl_content, line)?;

                let module = modules.entry(name.clone()).or_insert_with(|| Module {
                    name: name.clone(),
                    command_definition: None,
                    event_definition: None,
                    result_definition: None,
                    commands: Vec::new(),
                    events: Vec::new(),
                    types: Vec::new(),
                    results: Vec::new(),
                });

                let command_def = parse_command_definition(format!("{}Command", name), content, cddl_strings.clone(), module)?;
                module.command_definition = Some(command_def);
            }
            
            if let Some(captures) = event_pattern.captures(line) {
                let name = captures[1].to_string();
                let content = extract_definition_content(cddl_content, line)?;

                let module = modules.entry(name.clone()).or_insert_with(|| Module {
                    name: name.clone(),
                    command_definition: None,
                    event_definition: None,
                    result_definition: None,
                    commands: Vec::new(),
                    events: Vec::new(),
                    types: Vec::new(),
                    results: Vec::new(),
                });

                module.event_definition = Some(EventDefinition {
                    name: format!("{}Event", name),
                    content,
                });
            }
            
            if let Some(captures) = result_pattern.captures(line) {
                let name = captures[1].to_string();
                let content = extract_definition_content(cddl_content, line)?;

                let module = modules.entry(name.clone()).or_insert_with(|| Module {
                    name: name.clone(),
                    command_definition: None,
                    event_definition: None,
                    result_definition: None,
                    commands: Vec::new(),
                    events: Vec::new(),
                    types: Vec::new(),
                    results: Vec::new(),
                });

                module.result_definition = Some(ResultDefinition {
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


