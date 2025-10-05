use regex::Regex;
use std::collections::{HashMap, HashSet};
use crate::extractor::extract_definition_content;
use crate::command_parser::{CommandDefinition, parse_command_definition, Command, parse_result_definition, ResultDefinition, BidiResult};
use crate::event_parser::{EventDefinition, parse_event_definition, Event};



#[derive(Debug)]
pub struct BidiType {
    pub name: String,
    pub properties: Vec<crate::parser::Property>,
    pub is_enum: bool,
    pub is_alias: bool,
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
    pub pending_types: HashSet<String>,
}

#[derive(Debug)]
pub struct CommandType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct CommandDataType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct EmptyParamsType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct MessageType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct CommandResponseType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct ErrorResponseType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct ResultDataType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct EmptyResultType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct EventType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct EventDataType {
    pub properties: Vec<crate::parser::Property>,
}

#[derive(Debug)]
pub struct RootProtocol {
    pub command: CommandType,
    pub command_data: CommandDataType,
    pub empty_params: EmptyParamsType,
    pub message: MessageType,
    pub command_response: CommandResponseType,
    pub error_response: ErrorResponseType,
    pub result_data: ResultDataType,
    pub empty_result: EmptyResultType,
    pub event: EventType,
    pub event_data: EventDataType,
    pub additional_types: Vec<BidiType>,
}

fn unwrap_union_property(properties: Vec<crate::parser::Property>, temp_module: &mut Module) -> Vec<crate::parser::Property> {
    if properties.len() == 1 {
        let property_value = &properties[0].value;
        if let Some(idx) = temp_module.types.iter().position(|t| &t.name == property_value) {
            let bidi_type = temp_module.types.remove(idx);
            return bidi_type.properties;
        }
    }
    properties
}

pub fn detect_root_protocol(cddl_strings: Vec<&str>) -> Result<RootProtocol, Box<dyn std::error::Error>> {
    let mut temp_module = Module {
        name: "root".to_string(),
        command_definition: None,
        event_definition: None,
        result_definition: None,
        commands: Vec::new(),
        events: Vec::new(),
        types: Vec::new(),
        results: Vec::new(),
        pending_types: HashSet::new(),
    };

    let (content, _) = crate::parser::find_and_extract_type_content("Command", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("Command"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let command = CommandType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("CommandData", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("CommandData"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let command_data = CommandDataType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("EmptyParams", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("EmptyParams"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let empty_params = EmptyParamsType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("Message", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("Message"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let message = MessageType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("CommandResponse", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("CommandResponse"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let command_response = CommandResponseType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("ErrorResponse", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("ErrorResponse"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let error_response = ErrorResponseType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("ResultData", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("ResultData"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let result_data = ResultDataType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("EmptyResult", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("EmptyResult"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let empty_result = EmptyResultType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("Event", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("Event"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let event = EventType { properties };

    let (content, _) = crate::parser::find_and_extract_type_content("EventData", &cddl_strings, &temp_module).unwrap();
    let (properties, _) = crate::parser::process_cddl_to_struct(&content, cddl_strings.clone(), &mut temp_module, Some("EventData"))?;
    let properties = unwrap_union_property(properties, &mut temp_module);
    let event_data = EventDataType { properties };

    // Extract any additional types that were generated (like SuccessEnum, ErrorEnum, EventEnum)
    let additional_types = temp_module.types;

    Ok(RootProtocol {
        command,
        command_data,
        empty_params,
        message,
        command_response,
        error_response,
        result_data,
        empty_result,
        event,
        event_data,
        additional_types,
    })
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
                    pending_types: HashSet::new(),
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
                    pending_types: HashSet::new(),
                });

                let event_def = parse_event_definition(format!("{}Event", name), content, cddl_strings.clone(), module)?;
                module.event_definition = Some(event_def);
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
                    pending_types: HashSet::new(),
                });

                let result_def = parse_result_definition(format!("{}Result", name), content, cddl_strings.clone(), module)?;
                module.result_definition = Some(result_def);
            }
        }
    }

    let mut result: Vec<Module> = modules.into_values().collect();

    // Process deferred types for each module
    crate::parser::process_deferred_types(&mut result, cddl_strings)?;

    result.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(result)
}


