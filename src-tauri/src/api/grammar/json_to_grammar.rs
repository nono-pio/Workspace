use crate::api::grammar::fragment::Fragment;
use crate::api::grammar::grammar::Grammar;
use crate::api::grammar::rules::{
    FragmentRule, OptionalRule, OrRule, Rule, SequenceRule, TokenRule,
};
use crate::api::grammar::token::TokenDefinition;
use serde_json::Value;

#[derive(Debug)]
pub enum Error {
    Default,
    NullRule,
    UndefinedRule,
    UnknownRuleType,
    UndefinedToken,
}

/// # Grammar
///
/// {
///     grammarName: String,
///     tokenDefinitions: {
///         tokenName: TokenDefinition,
///         tokenName: TokenDefinition,
///         ...},
///     fragments: {
///         fragmentName: Fragment,
///         fragmentName: Fragment,
///         ...},
/// }
#[tauri::command]
pub fn json_to_grammar(json: Value) -> Result<Grammar, Error> {
    let grammar_name = json.get("grammarName").unwrap().as_str().unwrap();
    let token_definitions = json_to_tokens_definition(json.get("tokenDefinitions").unwrap())?;
    let fragments = json_to_fragments(json.get("fragments").unwrap())?;

    Ok(Grammar::new(grammar_name, token_definitions, fragments, 2))
}

// region ---TokenDefinition
fn json_to_tokens_definition(json: &Value) -> Result<Vec<TokenDefinition>, Error> {
    let token_map = json.as_object().unwrap();
    let mut tokens = Vec::with_capacity(token_map.len());
    for (name, token) in token_map {
        tokens.push(json_to_token_definition(token, name)?);
    }

    Ok(tokens)
}

/// # TokenDefinition
/// 1. string : if []+*? -> regex else -> keyword
/// 1. { regex: string } -> regex,
/// 1. { keyword: string } -> keyword,
fn json_to_token_definition(json: &Value, name: &str) -> Result<TokenDefinition, Error> {
    match json {
        Value::String(_) => todo!(),
        Value::Object(object) => {
            // Regex
            if let Some(regex) = object.get("regex") {
                let regex = regex.as_str().unwrap();
                Ok(TokenDefinition::new_regex(
                    name,
                    format!("{}{}", "^", regex).as_str(),
                ))
            // Keyword
            } else if let Some(keyword) = object.get("keyword") {
                let keyword = keyword.as_str().unwrap();
                Ok(TokenDefinition::new_keyword(name, keyword))
            // Undefined
            } else {
                Err(Error::UndefinedToken)
            }
        }
        _ => Err(Error::UndefinedToken),
    }
}

// endregion

// region ---Fragment

fn json_to_fragments(json: &Value) -> Result<Vec<Fragment>, Error> {
    let fragment_map = json.as_object().unwrap();
    let mut fragments = Vec::with_capacity(fragment_map.len());
    for (name, fragment) in fragment_map {
        fragments.push(json_to_fragment(fragment, name)?);
    }

    Ok(fragments)
}

///
/// # Fragment
/// {
///     rule: Rule
///}
fn json_to_fragment(json: &Value, name: &str) -> Result<Fragment, Error> {
    let rule = json_to_rule(json.get("rule").unwrap())?;
    Ok(Fragment::new(name, rule))
}

// region ---Rule

/// # Rule
/// string -> TokenRule | FragmentRule!,
/// [Rule, Rule, Rule, ...] -> SequenceRule,
/// {
///     type: "sequence" | "token" | "fragment" | ...,
///     value?: Rule | usize (index tokenDefinition) | usize (index fragment),
///     values?: [Rule, Rule, Rule, ...],
///     otherData?: any
/// }
///
/// ## Example 1
/// {
///     type: "sequence",
///     values: [
///         {
///             type: "token",
///             value: 0
///         },
///         {
///             type: "token",
///             value: 3
///         },
///         {
///             type: "token",
///             value: 0
///         }
///     ]
/// }
/// or equivalent
/// [0, 3, 0]
fn json_to_rule(json: &Value) -> Result<Box<dyn Rule>, Error> {
    match json {
        Value::Null => Err(Error::NullRule),
        Value::String(_) => todo!("TokenRule or FragmentRule"),
        Value::Array(_) => todo!("SequenceRule"),
        Value::Object(object) => {
            let rule_type = object.get("type").unwrap().as_str().unwrap();

            match rule_type {
                "token" => json_to_token_rule(&json),
                "fragment" => json_to_fragment_rule(&json),
                "sequence" => json_to_sequence_rule(&json),
                "or" => json_to_or_rule(&json),
                "optional" => json_to_optional_rule(&json),
                _ => Err(Error::UnknownRuleType),
            }
        }
        _ => Err(Error::UndefinedRule),
    }
}

fn json_to_token_rule(object: &Value) -> Result<Box<dyn Rule>, Error> {
    let token_definition_index = object.get("value").unwrap().as_u64().unwrap() as usize;
    Ok(Box::new(TokenRule(token_definition_index)))
}

fn json_to_fragment_rule(object: &Value) -> Result<Box<dyn Rule>, Error> {
    let token_definition_index = object.get("value").unwrap().as_u64().unwrap() as usize;
    Ok(Box::new(FragmentRule(token_definition_index)))
}

fn json_to_sequence_rule(object: &Value) -> Result<Box<dyn Rule>, Error> {
    Ok(Box::new(SequenceRule(get_rules(object)?)))
}

fn json_to_or_rule(object: &Value) -> Result<Box<dyn Rule>, Error> {
    Ok(Box::new(OrRule(get_rules(object)?)))
}

fn json_to_optional_rule(object: &Value) -> Result<Box<dyn Rule>, Error> {
    let rule = object.get("value").unwrap();
    return Ok(Box::new(OptionalRule(json_to_rule(rule)?)));
}

fn get_rules(object: &Value) -> Result<Vec<Box<dyn Rule>>, Error> {
    let rules_objects = object.get("values").unwrap().as_array().unwrap();
    let mut rules = Vec::new();
    for rule in rules_objects {
        rules.push(json_to_rule(rule)?);
    }
    Ok(rules)
}
// endregion

// endregion
