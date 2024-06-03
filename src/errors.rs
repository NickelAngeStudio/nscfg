use crate::arm::{ARM_SEPARATOR, CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1, WILDCARD_ARM, MODIFIER_ACTIVATE, MODIFIER_DEACTIVATE, MODIFIER_PANIC};

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

/// Possible nscfg errors.
pub enum NSCFGError {
    /// Missing operator (happens when a leaf contains a space)
    MissingOperator,

    /// Empty node due to missing variable.
    EmptyNode,

    /// Invalid character used
    InvalidCharacter(String),

    /// Alias written is not found
    AliasNotFound(String),

    /// Invalid configuration predicate
    InvalidConfigurationPredicate(String),

    /// Happens when having an empty arm.
    EmptyArm,

    /// Happens when wildcard arm _ is not the last.
    WildcardArmNotLast,

    /// Happens when a separator `,` is missing between arms.
    ArmSeparatorMissing,

    /// Happens when a content separator `=>` is malformed.
    ContentSeparatorError,

    /// Happens when wildcard arm is not set for match_cfg!.
    WildcardArmMissing,

    /// Happens when wildcard arm is set when using single_cfg!
    WildcardArmOnTarget,

    /// Happens when trying to use target_cfg! inside a function.
    TargetInFunction,

    /// Happens when legacy syntax is incorrect
    LegacySyntaxError,

    /// Happens when mixing legacy and simplifier syntax on same arm.
    MixedSyntaxError,

    /// Happens when a separator `=>` is missing between arms.
    ContentSeparatorMissing,

    /// Happens when a modifier `+` or `-` isn't the first character of arm.
    ModifierNotFirst,

    /// Happens when a modifier `+` or `-` is used during release compilation and not set to ignore.
    #[allow(dead_code)]
    ModifierPanicRelease,

    /// Happens when more than 1 modifier `+` in match_cfg!.
    MatchModifierMoreThanOneActivate,

    /// Happens when using modifier `-` on wildcard arm of match_cfg!.
    MatchDeactivatedWildArm,
}

/// Error message implementation.
impl NSCFGError {
    pub fn message(&self, tokens : &str) -> String {
        match self {
            NSCFGError::MissingOperator => format!("Operator `&` or '|' missing for `{:?}`. Target must not contain space.", tokens),
            NSCFGError::EmptyNode =>  format!("Empty node generated from attributes. Are you missing a statement between separator?"),
            NSCFGError::InvalidCharacter(c) => format!("Invalid character `{}` for `{:?}`.", c, tokens),
            NSCFGError::AliasNotFound(alias) => format!("Alias `{}` has no match! Is it added in config.toml as `target_cfg-{}`?", alias, alias),
            NSCFGError::InvalidConfigurationPredicate(cfg_prd) => format!("Configuration predicate `{}` has no match! Is it added in config.toml as `target_cfg_predicate-{}`?", cfg_prd, cfg_prd),
            NSCFGError::EmptyArm => format!("Empty arm with no attributes detected!"),
            NSCFGError::WildcardArmNotLast => format!("Wildcard branch `_` must ALWAYS be the last branch."),
            NSCFGError::ArmSeparatorMissing => format!("Arm syntax incorrect. Are you missing a separator `{}` between arms?", ARM_SEPARATOR),
            NSCFGError::ContentSeparatorError => format!("Arm syntax incorrect. Is your arm separator `{}{}` syntax Ok?", CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1),
            NSCFGError::WildcardArmMissing => format!("Ensure that all possible cases are being handled by adding a match arm with a `{}` wildcard pattern.", WILDCARD_ARM),
            NSCFGError::WildcardArmOnTarget => format!("target_cfg! macro cannot have a `{}` wildcard pattern.", WILDCARD_ARM),
            NSCFGError::TargetInFunction => format!("target_cfg! macro cannot be used inside a function. Use match_cfg! instead."),
            NSCFGError::LegacySyntaxError => format!("Legacy syntax error in `{}`.", tokens),
            NSCFGError::MixedSyntaxError => format!("Legacy syntax and simplified syntax can't be mixed on same arm!"),
            NSCFGError::ContentSeparatorMissing => format!("Arm content separator `{}{}` missing!", CONTENT_SEPARATOR_0, CONTENT_SEPARATOR_1),
            NSCFGError::ModifierNotFirst => format!("Arm modifiers `{}`, `{}` and `{}` must be the first character of arm!", MODIFIER_ACTIVATE, MODIFIER_DEACTIVATE, MODIFIER_PANIC),
            NSCFGError::ModifierPanicRelease => format!("Arm modifiers `{}` and `{}` will panic during release compilation by default! This behaviour can be changed. See https://github.com/NickelAngeStudio/nscfg/wiki/Syntax#six-modifiers", MODIFIER_ACTIVATE, MODIFIER_DEACTIVATE),
            NSCFGError::MatchModifierMoreThanOneActivate => format!("match_cfg! cannot have more than one `{}` modifier!", MODIFIER_ACTIVATE),
            NSCFGError::MatchDeactivatedWildArm => format!("match_cfg! cannot deactivate wildcard arm with `{}` modifier!", MODIFIER_DEACTIVATE),
        }
    }
}