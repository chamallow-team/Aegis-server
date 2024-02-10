use std::collections::HashMap;
use regex::Regex;
use crate::log::Log;
use crate::StreamID;

/// The type of rule.
///
/// It can be used to exclude logs from being written to a stream, or to write logs to a stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleType {
    ExcludeRoute = 0,
    ExcludeText = 1,
    WriteTo = 2,
}

/// A rule that applies to a specific route.
///
/// It can be used to exclude logs from being written to a stream, or to write logs to a stream.
#[derive(Debug, Clone)]
pub struct Rule {
    rule_type: RuleType,
    pattern: Option<Pattern>
}

impl Rule {
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
    pub(crate) fn is_global_allowed(&self, log: &Log) -> bool {
        self.globals.iter().all(|rule| rule.is_allowed(log))
    }

    pub(crate) fn get_route_rules_from_id(&self, id: &StreamID) -> Option<&RouteRules> {
        self.locals.get(id)
    }

    #[allow(unused)]
    fn is_locally_allowed(log: &Log, route: &RouteRules) -> bool {
        route.is_allowed(log)
    }

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