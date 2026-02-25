use cddl::{
    ast::{GroupEntry, GroupRule, Identifier, Rule},
    cddl_from_str,
};

use crate::frontend::types::{TokenBuilder, TypeDef};

pub fn group_rule_group_entry(grge: GroupEntry, group_rule_name: Identifier) {
    return match grge {
        GroupEntry::ValueMemberKey {
            ge,
            span,
            leading_comments,
            trailing_comments,
        } => {
            for type_choice in ge.entry_type.type_choices {
                if let Some(type_choice_operator) = type_choice.type1.operator {
                    println!("{}", type_choice_operator.operator);
                }
                println!("{}", type_choice.type1.type2);
            }
            if let Some(m_key) = ge.member_key {
            println!("{}", m_key);
            }
            let type_def = TypeDef::Struct { 
                name: TokenBuilder::default().name(group_rule_name.to_string()).build(),
                properties: Vec::new()
            };
        }
        GroupEntry::InlineGroup {
            occur,
            group,
            span,
            comments_before_group,
            comments_after_group,
        } => {
            for group_choice in group.group_choices {
                for group_entry in group_choice.group_entries {
                    group_rule_group_entry(group_entry.0, group_rule_name.clone());
                    // println!("{}", group_entry.0);
                }
            }
        }
        GroupEntry::TypeGroupname {
            ge,
            span,
            leading_comments,
            trailing_comments,
        } => {
            println!("{}", ge)
        }
    }
}

pub fn group_rule(group_rule: Box<GroupRule>) {
    group_rule_group_entry(group_rule.entry, group_rule.name);
}

pub fn parse_cddl_str(cddl_str: &str) {
    let parsed_cddl = cddl_from_str(cddl_str, true).unwrap();
    for rule in parsed_cddl.rules {
        match rule {
            Rule::Type {
                rule,
                span,
                comments_after_rule,
            } => {
                // println!("{}", rule)
            }
            Rule::Group {
                rule,
                span,
                comments_after_rule,
            } => {
                println!("{}", rule.name);
                group_rule(rule);
                println!()
            }
        }
    }
}
