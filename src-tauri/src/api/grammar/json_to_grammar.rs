use crate::api::grammar::fragment::Fragment;
use crate::api::grammar::grammar::Grammar;
use crate::api::grammar::rules::{
    FragmentRule, OptionalRule, OrRule, Rule, SequenceRule, TokenRule,
};
use crate::api::grammar::token::{Pattern, TokenDefinition};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    Default,
    NullRule,
    UndefinedRule,
    UnknownRuleType,
    UndefinedToken,
    UndefinedRegexExpression,
    UnknownTokenOrFragment,
}

struct GrammarData {
    tokens_index_map: HashMap<String, usize>,
    fragments_index_map: HashMap<String, usize>,
}

impl GrammarData {
    fn new(
        tokens_index_map: HashMap<String, usize>,
        fragments_index_map: HashMap<String, usize>,
    ) -> Self {
        GrammarData {
            tokens_index_map,
            fragments_index_map,
        }
    }

    fn get_token_definition_index(&self, name: &str) -> Result<usize, Error> {
        match self.tokens_index_map.get(name) {
            Some(index) => Ok(*index),
            None => Err(Error::UnknownTokenOrFragment),
        }
    }

    fn get_fragment_index(&self, name: &str) -> Result<usize, Error> {
        match self.fragments_index_map.get(name) {
            Some(index) => Ok(*index),
            None => Err(Error::UnknownTokenOrFragment),
        }
    }

    fn get_rule(&self, name: &str) -> Result<Box<dyn Rule>, Error> {
        if let Ok(index) = self.get_fragment_index(name) {
            Ok(Box::new(FragmentRule(index)))
        } else if let Ok(index) = self.get_token_definition_index(name) {
            Ok(Box::new(TokenRule(index)))
        } else {
            Err(Error::UnknownTokenOrFragment)
        }
    }
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
    let (token_definitions, tokens_index_map) =
        json_to_tokens_definition(json.get("tokenDefinitions").unwrap())?;

    let fragments = json_to_fragments(json.get("fragments").unwrap(), tokens_index_map)?;

    // TODO: main fragment
    Ok(Grammar::new(grammar_name, token_definitions, fragments, 2))
}

// region ---TokenDefinition
fn json_to_tokens_definition(
    json: &Value,
) -> Result<(Vec<TokenDefinition>, HashMap<String, usize>), Error> {
    let token_map = json.as_object().unwrap();
    let mut tokens = Vec::with_capacity(token_map.len());

    let token_map: HashMap<String, usize> = token_map
        .iter()
        .map(|(name, token)| {
            tokens.push(json_to_token_definition(token, name).unwrap());
            (name.clone(), tokens.len() - 1)
        })
        .collect();

    Ok((tokens, token_map))
}

/// # TokenDefinition
/// 1. string : if []+*? -> regex else -> keyword
/// 1. { regex: string } -> regex,
/// 1. { keyword: string } -> keyword,
fn json_to_token_definition(json: &Value, name: &str) -> Result<TokenDefinition, Error> {
    match json {
        Value::String(str) => Ok(TokenDefinition::new(name, string_to_pattern(str)?)),
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

fn string_to_pattern(string: &String) -> Result<Pattern, Error> {
    // TODO
    let regex = format!("{}{}", "^", string);
    if let Ok(regex) = Regex::new(regex.as_str()) {
        return Ok(Pattern::Regex(regex));
    }
    Err(Error::UndefinedRegexExpression)
}

// endregion

// region ---Fragment

fn json_to_fragments(
    json: &Value,
    tokens_index_map: HashMap<String, usize>,
) -> Result<Vec<Fragment>, Error> {
    let fragment_map = json.as_object().unwrap();

    let mut index = 0;
    let fragment_index_map: HashMap<String, usize> = fragment_map
        .iter()
        .map(|(name, fragment)| {
            index += 1;
            (name.clone(), index - 1)
        })
        .collect();

    let grammar_data = GrammarData::new(tokens_index_map, fragment_index_map);

    let mut fragments = Vec::with_capacity(fragment_map.len());
    for (name, fragment) in fragment_map {
        fragments.push(json_to_fragment(fragment, name, &grammar_data)?);
    }

    Ok(fragments)
}

///
/// # Fragment
/// {
///     rule: Rule
///}
fn json_to_fragment(
    json: &Value,
    name: &str,
    grammar_data: &GrammarData,
) -> Result<Fragment, Error> {
    let rule = json_to_rule(json.get("rule").unwrap(), grammar_data)?;
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
fn json_to_rule(json: &Value, grammar_data: &GrammarData) -> Result<Box<dyn Rule>, Error> {
    match json {
        Value::Null => Err(Error::NullRule),
        Value::String(str) => json_to_token_or_fragment_rule(str, grammar_data),
        Value::Array(array) => json_to_sequence_rule_from_array(array, grammar_data),
        Value::Object(object) => {
            let rule_type = object.get("type").unwrap().as_str().unwrap();

            match rule_type {
                "token" => json_to_token_rule(&json),
                "fragment" => json_to_fragment_rule(&json),
                "sequence" => json_to_sequence_rule(&json, grammar_data),
                "or" => json_to_or_rule(&json, grammar_data),
                "optional" => json_to_optional_rule(&json, grammar_data),
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

fn json_to_token_or_fragment_rule(
    value: &String,
    grammar_data: &GrammarData,
) -> Result<Box<dyn Rule>, Error> {
    grammar_data.get_rule(value)
}

fn json_to_fragment_rule(object: &Value) -> Result<Box<dyn Rule>, Error> {
    let token_definition_index = object.get("value").unwrap().as_u64().unwrap() as usize;
    Ok(Box::new(FragmentRule(token_definition_index)))
}

fn json_to_sequence_rule_from_array(
    array: &Vec<Value>,
    grammar_data: &GrammarData,
) -> Result<Box<dyn Rule>, Error> {
    let mut rules = Vec::with_capacity(array.len());
    for rule in array {
        rules.push(json_to_rule(&rule, grammar_data)?);
    }
    Ok(Box::new(SequenceRule(rules)))
}
fn json_to_sequence_rule(
    object: &Value,
    grammar_data: &GrammarData,
) -> Result<Box<dyn Rule>, Error> {
    Ok(Box::new(SequenceRule(get_rules(object, grammar_data)?)))
}

fn json_to_or_rule(object: &Value, grammar_data: &GrammarData) -> Result<Box<dyn Rule>, Error> {
    Ok(Box::new(OrRule(get_rules(object, grammar_data)?)))
}

fn json_to_optional_rule(
    object: &Value,
    grammar_data: &GrammarData,
) -> Result<Box<dyn Rule>, Error> {
    let rule = object.get("value").unwrap();
    return Ok(Box::new(OptionalRule(json_to_rule(rule, grammar_data)?)));
}

fn get_rules(object: &Value, grammar_data: &GrammarData) -> Result<Vec<Box<dyn Rule>>, Error> {
    let rules_objects = object.get("values").unwrap().as_array().unwrap();
    let mut rules = Vec::new();
    for rule in rules_objects {
        rules.push(json_to_rule(rule, grammar_data)?);
    }
    Ok(rules)
}
// endregion

// endregion
