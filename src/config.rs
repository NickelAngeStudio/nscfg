/* 
Copyright (c) 2024  NickelAnge.Studio 
Email               mathieu.grenier@nickelange.studio
Git                 https://github.com/NickelAngeStudio/nswnd

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::{env, path::Path, fs};

use crate::errors::NSCFGError;

#[cfg(test)]
#[path = "../tests/unit/config.rs"]
mod unit_tests; // Unit tests located in tests folder

// Contants
pub(crate) const ENV_KEY_PREDICATE : &str = "nscfg_predicate-";   // Key used to fetch custom predicate
pub(crate) const ENV_KEY_ALIAS : &str = "nscfg-";                 // Key used to fetch custom aliases
pub(crate) const PREDICATE_PLACEHOLDER : &str = "{}";               // Predicate placeholder
const AUTO_DOC_KEY : &str = "nscfg_autodoc";                    // Key for nscfg autodocumentation parameter.
const MODIFIER_BEHAVIOUR_KEY : &str = "nscfg_release_modifier_behaviour";                    // Key for nscfg release modifier behaviour parameter.
const NSCFG_CARGO_CACHE : &str = "CFG_BOOST_ATTR_DOC_SET";      // Key value of cargo.toml caching.
const NSCFG_DOCRS_TAG : &str = "[package.metadata.docs.rs]";    // Tag to search in Cargo.toml
const CARGO_MANIFEST_DIR : &str = "CARGO_MANIFEST_DIR";             // Cargo manifest dir key
const CARGO_MANIFEST_NAME : &str = "Cargo.toml";                    // Cargo manifest file name
pub(crate) const DOC_ALIAS : &str = "doc";                          // Doc alias

// Aliases
pub(crate) const ALIASES : [(&str, &str); 12] = [
    ("linux", "linux:os"),                              // Linux alias and value
    ("unix", "unix:_"),                                 // Unix alias and value
    ("windows", "windows:_"),                           // Windows alias and value
    ("macos", "macos:os"),                              // Macos alias and value
    ("android", "android:os"),                          // Android alias and value
    ("ios", "ios:os"),                                  // Ios alias and value
    ("wasm", "wasm:_"),                                 // Wasm alias and value
    (DOC_ALIAS, "doc:_"),                               // Doc alias and value
    ("test", "test:_"),                                 // Test alias and value
    ("debug", "debug_assertions:_"),                    // Debug alias and value
    ("desktop", "linux:os | windows:_ | macos:os"),     // Desktop alias and value
    ("mobile", "android:os | ios:os")                   // Mobile alias and value
];

// Predicates
pub(crate) const PREDICATES : [(&str, &str); 12] = [
    ("ar", "target_arch = \"{}\""),             // Target architecture predicate
    ("tf", "target_feature = \"{}\""),          // Target feature predicate
    ("os", "target_os = \"{}\""),               // Target os predicate
    ("fm", "target_family = \"{}\""),           // Target family predicate
    ("ev", "target_env = \"{}\""),              // Target environment predicate
    ("ed", "target_endian = \"{}\""),           // Target endian predicate
    ("pw", "target_pointer_width = \"{}\""),    // Target pointer width predicate
    ("vn", "target_vendor = \"{}\""),           // Target vendor predicate
    ("at", "target_has_atomic = \"{}\""),       // Target has atomic predicate
    ("pn", "panic = \"{}\""),                   // Panic predicate
    ("ft", "feature = \"{}\""),                 // Feature predicate
    ("_", PREDICATE_PLACEHOLDER)                // Wildcard predicate
];

pub(crate) enum ReleaseModifierBehaviour {
    /// Panic! when trying to use modifiers on releae
    Panic,

    /// Ignore modifiers on release
    Ignore,
}

/// Get the modifier behaviour on release.
/// 
/// Modifiers are usually used for quick testing and create a different debug behaviour vs release.
/// By default, panic is used so user know he forgot some modifier. This behaviour can be changed
/// in config.toml to just ignore the modifiers and not panic. This must be done manually so the user
/// can acknowledge the risk.
#[allow(dead_code)]
pub(crate) fn get_release_modifier_behaviour() -> ReleaseModifierBehaviour{

    match std::env::var(MODIFIER_BEHAVIOUR_KEY) {
        Ok(value) => match value.as_str() {
            "panic" => ReleaseModifierBehaviour::Panic,
            "ignore" => ReleaseModifierBehaviour::Ignore,
            _ => ReleaseModifierBehaviour::Panic,  // Any other value is considered panic.
        },
        Err(_) => ReleaseModifierBehaviour::Panic,     // If not set, return panic as default
    }

}

/// Get if autodocumentation is true or false.
/// 
/// If not set, default is true.
#[inline(always)]
pub(crate) fn is_nscfg_autodoc() -> bool {
    match std::env::var(AUTO_DOC_KEY) {
        Ok(value) => match value.as_str() {
            "true" => true,
            "false" => false,
            _ => true,  // Any other value is considered true.
        },
        Err(_) => true,     // If not set, return true as default
    }
}


/// Returns True if cfg-attr is generated for documentation labels.
#[inline(always)]
pub(crate) fn if_docsrs_enabled() -> bool {
    // 1. Get previous result from cache. 
    match env::var(NSCFG_CARGO_CACHE) {
        Ok(value) => {
            value.eq("true")
        },
        Err(_) => {
            // 2. Read Cargo.toml if no result
            let str_path =  format!("{}/{}", env::var(CARGO_MANIFEST_DIR).unwrap(), CARGO_MANIFEST_NAME);
            let file_path = Path::new(&str_path);

            match fs::read_to_string(file_path){
                Ok(content) => {
                    match content.find(NSCFG_DOCRS_TAG){
                        Some(_) => { 
                            env::set_var(NSCFG_CARGO_CACHE, "true");    // Cache result
                            true
                        },
                        None => {
                            env::set_var(NSCFG_CARGO_CACHE, "false");    // Cache result
                            false
                        },
                    }
                },

                // Cargo.toml not found, return false.
                Err(_) => {
                    env::set_var(NSCFG_CARGO_CACHE, "false");
                    false
                },
            }
        }
    }
}


/// Parse tokens to generate configuration predicate.
/// 
/// Error(s)
/// Returns Err([SyntaxParseError::InvalidConfigurationPredicate]) if predicate not defined.
#[inline(always)]
pub fn get_nscfg_predicate(tokens : &str) -> Result<String, NSCFGError> {

    // 1. Extract label and predicate from tokens
    match tokens.find(":") {
        Some(position) => {
            let label = tokens[0..position].trim();
            let cfg_opt = tokens[position + 1..].trim();

            // 2. Try to match environment variable to see if predicate was defined in config.toml.
            match env::var(format!("{}{}", ENV_KEY_PREDICATE, cfg_opt)) {
                Ok(cfg_value) => Ok(String::from(cfg_value.replace(PREDICATE_PLACEHOLDER, label))),
                Err(_) =>  {
                    // 3. Find predefined predicates
                    match PREDICATES.iter().find(|p| p.0.eq(cfg_opt)){
                        // Predicate found, return value
                        Some(pred) =>  Ok(String::from(pred.1.replace(PREDICATE_PLACEHOLDER, label))),

                        // Not found, raise error.
                        None => Err(NSCFGError::InvalidConfigurationPredicate(String::from(cfg_opt))),
                    }
                },
            }
        },

        // Should never happen but good to have in hand
        None => Err(NSCFGError::InvalidConfigurationPredicate(String::from(tokens))),
    } 

}


/// Parse label to generate alias content.
/// 
/// Error(s)
/// Returns Err([TargetCfgError::AliasNotFound]) if alias not defined.
#[inline(always)]
pub fn get_nscfg_alias(label : &str) -> Result<String, NSCFGError> {

    // 1. Try to match environment variable to see if it was defined in config.toml.
    match env::var(format!("{}{}", ENV_KEY_ALIAS, label)) {
        Ok(alias) => Ok(alias.clone()),     
        Err(_e) => {
            // 2. Find predefined alias
            match ALIASES.iter().find(|a| a.0.eq(label)){
                // Alias found, return value
                Some(alias) => Ok(String::from(alias.1)),

                // Not found, raise error.
                None => Err(NSCFGError::AliasNotFound(String::from(label))),
            }
        },
    }

}