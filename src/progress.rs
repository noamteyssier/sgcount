use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// Initializes a Multiprogress bar for parallel logging
#[must_use] pub fn initialize_multi_progress(sample_names: &[String]) -> (Option<MultiProgress>, Option<Vec<ProgressBar>>) {
    let mp = MultiProgress::new();
    let progress_bars: Vec<ProgressBar> = sample_names
        .iter()
        .map(|_| 
            ProgressBar::new_spinner()
                .with_style(ProgressStyle::default_spinner()
                    .template("{prefix} {spinner} [{elapsed_precise}] {msg}"))
            )
        .enumerate()
        .map(|(idx, pb)| mp.insert(idx, pb))
        .collect();
    (Some(mp), Some(progress_bars))
}

/// Initializes a progress spinner
#[must_use] pub fn initialize_progress_bar() -> ProgressBar {
    ProgressBar::new_spinner()
        .with_style(ProgressStyle::default_spinner()
            .template("{prefix} {spinner} [{elapsed_precise}] {msg}"))
}

/// Starts a progress bar that is an optional reference
pub fn start_progress_bar_ref(pb: Option<&ProgressBar>, msg: String) {
    if let Some(p) = pb {
        p.enable_steady_tick(75);
        p.set_message(msg);
    }
}

/// Finishes a progress bar that is an optional reference
pub fn finish_progress_bar_ref(pb: Option<&ProgressBar>, msg: String) {
    if let Some(p) = pb {
        p.set_prefix("🗸");
        p.finish_with_message(msg);
    }
}

/// Starts an optional progress bar
pub fn start_progress_bar(pb: &Option<ProgressBar>, msg: String) {
    if let Some(p) = pb {
        p.enable_steady_tick(75);
        p.set_message(msg);
    }
}

/// Finishes an optional progress bar
pub fn finish_progress_bar(pb: &Option<ProgressBar>, msg: String) {
    if let Some(p) = pb {
        p.set_prefix("🗸");
        p.finish_with_message(msg);
    }
}
