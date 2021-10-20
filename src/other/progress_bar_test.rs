// Copyright (c) 2021 unstable-errors
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::cmp::min;
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn progress_bar_test() {
    let mut downloaded = 0;
    let total_size = 231231231;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}
