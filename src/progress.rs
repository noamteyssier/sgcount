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

#[cfg(test)]
mod testing {
    
    #[test]
    fn test_initialize_multi_progress() {
        let sample_names = vec!["sample1".to_string(), "sample2".to_string()];
        let (mp, pbs) = super::initialize_multi_progress(&sample_names);
        assert!(mp.is_some());
        assert!(pbs.is_some());
        let pbs = pbs.unwrap();
        assert_eq!(pbs.len(), 2);
    }

    #[test]
    fn test_initialize_progress_bar() {
        let pb = super::initialize_progress_bar();
        super::finish_progress_bar(&Some(pb), "done".to_string());
    }

    #[test]
    fn test_start_progress_bar_ref() {
        let pb = super::initialize_progress_bar();
        super::start_progress_bar_ref(Some(&pb), "starting".to_string());
        super::finish_progress_bar(&Some(pb), "done".to_string());
    }

    #[test]
    fn test_finish_progress_bar_ref() {
        let pb = super::initialize_progress_bar();
        super::start_progress_bar_ref(Some(&pb), "starting".to_string());
        super::finish_progress_bar_ref(Some(&pb), "done".to_string());
    }

    #[test]
    fn test_start_progress_bar() {
        let pb = super::initialize_progress_bar();
        super::start_progress_bar(&Some(pb), "starting".to_string());
    }
}
