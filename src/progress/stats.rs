use anyhow::Result;
use crossbeam::channel::Receiver;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use utils::utils::get_percentage;

pub fn stats_loop(stats_rx: Receiver<u64>, total_blocks: u64) -> Result<()> {
    //let progress = Arc::new(Mutex::new((0, false))); // (progress, completed)
    let progress_bar = ProgressBar::new(100);

    let progress_bar_style = ProgressStyle::default_bar()
        .template(
            "[{elapsed_precise}] {spinner:.green} [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap();
    progress_bar.set_style(progress_bar_style);
    let mut current_progress = 0;
    loop {
        let normalized_progress = match stats_rx.recv() {
            Ok(n) => {
                current_progress = current_progress + n;

                get_percentage(current_progress, total_blocks)
            }
            Err(e) => {
                eprintln!("Error receiving progress{} ", e);
                break;
            }
        };
        if normalized_progress >= 100 {
            progress_bar.set_position(100 as u64);

            progress_bar.finish_with_message("Processing complete");
            break;
        }
        //let (current_progress, completed) = *progress.lock().unwrap();
        progress_bar.set_position(normalized_progress as u64);

        thread::sleep(std::time::Duration::from_millis(100));
    }
    progress_bar.finish_with_message("Processing complete");

    Ok(())
}

/* pub fn stats_loop(stats_rx: Receiver<u64>, total_blocks: u64) -> Result<()> {
    let progress_bar = ProgressBar::new(total_blocks);
    let progress_bar_style = ProgressStyle::default_bar()
        .template(
            "[{elapsed_precise}] {spinner:.green} [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap();
    progress_bar.set_style(progress_bar_style);

    let mut current_progress = 0;
    while current_progress < total_blocks {
        println!("TOTAL BLOCKS {}", total_blocks);
        match stats_rx.recv() {
            Ok(n) => {
                current_progress += n;
                let normalized_progress = get_percentage(current_progress, total_blocks);
                progress_bar.set_position(normalized_progress.min(total_blocks) as u64);
            }
            Err(_) => {
                // Channel has been closed
                break;
            }
        }
    }

    progress_bar.finish_with_message("Processing complete");
    Ok(())
}
 */
