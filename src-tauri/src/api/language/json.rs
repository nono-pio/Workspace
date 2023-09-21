use crate::api::grammar::grammar::Grammar;
use crate::api::grammar::json_to_grammar::json_to_grammar;
use serde_json::json;

// url: https://www.json.org/json-en.html
//TODO: WS, escape characters (\n, \t, \u, \x,...)

pub fn get_json_grammar() -> Grammar {
    let grammar_json = json!({
        "grammarName": "JSON",
        // json tokens : [] { } : , " " true false null
        "tokenDefinitions": {
            "NUMBER": r"-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?", // -? int (decimal)? (exponent)?
            "NULL": "null",
            "TRUE": "true",
            "FALSE": "false",
            "STRING": "\"((\\\")|[^\"])*\"", // " (\" or not quote)* "

            "LEFT_BRACKET": "\\[",
            "RIGHT_BRACKET": "\\]",
            "LEFT_BRACE": "\\{",
            "RIGHT_BRACE": "\\}",
            "COLON": ":",
            "COMMA": ",",
        },
        "fragments": {
            "value": {
                "main": true,
                "rule": {
                    "type": "or",
                    "values": [
                        "object",
                        "array",
                        "STRING",
                        "NUMBER",
                        "TRUE",
                        "FALSE",
                        "NULL"
                    ]
                }
            },
            "object": {
                "rule": [
                    "LEFT_BRACE",
                    {
                        "type": "loop",
                        "value": [
                                "STRING",
                                "COLON",
                                "value"
                            ],
                        "separator": "COMMA"
                    },
                    "RIGHT_BRACE"
                ]
            },
            "array": {
                "rule": [
                    "LEFT_BRACKET",
                    {
                        "type": "loop",
                        "value": "value",
                        "separator": "COMMA"
                    },
                    "RIGHT_BRACKET"
                ]
            },
        },
    });

    json_to_grammar(grammar_json).unwrap()
}
