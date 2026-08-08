use std::time::SystemTime;

#[derive(Clone, Debug, Default)]
pub struct UsageSection {
    pub percentage: f64,
    pub resets_at: Option<SystemTime>,
}

#[derive(Clone, Debug)]
pub struct ScopedUsage {
    /// Server-provided model display name, e.g. "Fable". Never hardcoded — it
    /// changes when Anthropic changes which model the weekly sub-limit covers.
    pub label: String,
    pub section: UsageSection,
}

#[derive(Clone, Debug, Default)]
pub struct UsageData {
    pub session: UsageSection,
    pub weekly: UsageSection,
    pub scoped_weekly: Option<ScopedUsage>,
}

#[derive(Clone, Debug, Default)]
pub struct AppUsageData {
    pub claude_code: Option<UsageData>,
    pub codex: Option<UsageData>,
    pub antigravity: Option<UsageData>,
}
