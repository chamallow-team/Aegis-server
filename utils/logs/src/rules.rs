use std::collections::HashMap;
use regex::Regex;
use crate::log::{Log, LogType};
use crate::StreamID;

/// The type of rule.
///
/// It can be used to exclude logs from being written to a stream, or to write logs to a stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleType {
    ExcludeRoute = 0,
    ExcludeText = 1,
    WriteTo = 2,
    GreaterLevelThan = 3,
    LessLevelThan = 4,
    RequireRoute = 5
}

/// A rule that applies to a specific route.
///
/// It can be used to exclude logs from being written to a stream, or to write logs to a stream.
#[derive(Debug, Clone)]
pub struct Rule {
    rule_type: RuleType,
    pattern: Option<Pattern>,
    lvl: Option<LogType>
}

impl Rule {
    /// Creates a new rule.
    pub fn new(rule_type: RuleType) -> Self {
        Self {
            rule_type,
            pattern: None,
            lvl: None
        }
    }

    /// Sets the pattern of the rule.
    pub fn set_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = Some(pattern);
        self
    }

    /// Sets the level of the rule.
    pub fn set_pattern_mut(&mut self, pattern: Pattern) {
        self.pattern = Some(pattern);
    }

    /// Sets the level of the rule.
    pub fn set_lvl(mut self, lvl: LogType) -> Self {
        self.lvl = Some(lvl);
        self
    }

    /// Sets the level of the rule.
    pub fn set_lvl_mut(&mut self, lvl: LogType) {
        self.lvl = Some(lvl);
    }

    pub(crate) fn is_allowed(&self, log: &Log) -> bool {
        match self.rule_type {
            RuleType::ExcludeRoute => {
                if let Some(pattern) = &self.pattern {
                    if pattern.is_match(log.route().unwrap_or_default()) {
                        return false;
                    }
                }
                true
            },
            RuleType::ExcludeText => {
                if let Some(pattern) = &self.pattern {
                    if pattern.is_match(log.text()) {
                        return false;
                    }
                }
                true
            },
            RuleType::GreaterLevelThan => {
                if let Some(lvl) = self.lvl {
                    let log_lvl = log.log_type();
                    return log_lvl > lvl;
                }
                false
            },
            RuleType::LessLevelThan => {
                if let Some(lvl) = self.lvl {
                    let log_lvl = log.log_type();
                    return log_lvl < lvl;
                }
                false
            },
            RuleType::RequireRoute => {
                if let Some(pattern) = &self.pattern {
                    if pattern.is_match(log.route().unwrap_or_default()) {
                        return true;
                    }
                }
                false
            },
            _ => true
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RouteRules {
    rules: Vec<Rule>
}

impl RouteRules {

    pub(crate) fn is_allowed(&self, log: &Log) -> bool {
        self.rules.iter().all(|rule| rule.is_allowed(log))
    }
}

pub type Pattern = Regex;

/// A pattern that matches the root of a route.
///
/// It can a pattern like "api", "api::backend" or "api.backend" for example.
///
/// This type is used to know which rules to apply to a specific route.
pub type RuleRootPattern = String;

#[derive(Debug, Clone, Default)]
pub struct Rules {
    /// A list of global rules that apply to all streams.
    ///
    /// These rules can, for example, be used to write all logs to a file, or to a file.
    /// It also allows the user to exclude certain logs from being written to any stream.
    globals: Vec<Rule>,
    /// A list of rules that apply to specific routes.
    ///
    /// These rules can be used to write logs to a specific file, or to exclude logs from being written to a specific file.
    ///
    /// For example, a rule could be created to write all logs from the `/api` route to a file, or to exclude all logs from the `/api` route from being written to a file.
    locals: HashMap<RuleRootPattern, RouteRules>
}

impl Rules {
    /// Adds a global rule to the rules.
    pub fn add_global_rule(&mut self, rule: Rule) {
        self.globals.push(rule);
    }

    /// Adds a route rule to the rules.
    pub fn add_route_rule(&mut self, route: RuleRootPattern, rule: Rule) {
        let route_rules = self.locals.entry(route).or_insert_with(RouteRules::default);
        route_rules.rules.push(rule);
    }

    /// Returns the global rules.
    pub fn get_global_rules(&self) -> &[Rule] {
        &self.globals
    }

    /// Returns the route rules.
    pub fn get_route_rules(&self) -> &HashMap<RuleRootPattern, RouteRules> {
        &self.locals
    }

    /// Returns the global rule at the given index.
    pub fn get_global_rule(&self, index: usize) -> Option<&Rule> {
        self.globals.get(index)
    }

    /// Returns the route rule at the given index.
    pub fn get_route_rule(&self, route: &RuleRootPattern, index: usize) -> Option<&Rule> {
        self.locals.get(route).and_then(|route_rules| route_rules.rules.get(index))
    }

    /// Check if the log is allowed by the global rules.
    pub(crate) fn is_global_allowed(&self, log: &Log) -> bool {
        self.globals.iter().all(|rule| rule.is_allowed(log))
    }

    /// Get the route rules from the given stream ID.
    pub(crate) fn get_route_rules_from_id(&self, id: &StreamID) -> Option<&RouteRules> {
        self.locals.get(id)
    }

    /// Check if the log is allowed by the local rules.
    #[allow(unused)]
    fn is_locally_allowed(log: &Log, route: &RouteRules) -> bool {
        route.is_allowed(log)
    }

    /// Returns the first rule that matches the log.
    #[allow(unused)]
    /// Returns the first rule that matches the log.
    pub(crate) fn get_route_for_log(&self, log: &Log) -> Option<(&RuleRootPattern, &RouteRules)> {
        let route_path = log.route().unwrap_or_default();
        // Check if the log is allowed by the global rules
        for (pattern, route) in &self.locals {
            if route_path.starts_with(pattern) && Self::is_locally_allowed(log, route) {
                return Some((pattern, route));
            }
        }
        None
    }
}