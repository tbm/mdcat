// Copyright 2018-2020 Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Terminal size.

/// The size of a text terminal, in characters and lines.
#[derive(Debug, Copy, Clone)]
pub struct Size {
    /// The width of the terminal, in characters aka columns.
    pub width: usize,
    /// The height of the terminal, in lines.
    pub height: usize,
}

impl Default for Size {
    /// A good default size assumption for a terminal: 80x24.
    fn default() -> Size {
        Size {
            width: 80,
            height: 24,
        }
    }
}

impl Size {
    fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }

    /// Get terminal size from `$COLUMNS` and `$LINES`.
    pub fn from_env() -> Option<Size> {
        let columns = std::env::var("COLUMNS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok());
        let rows = std::env::var("LINES")
            .ok()
            .and_then(|value| value.parse::<usize>().ok());

        match (columns, rows) {
            (Some(columns), Some(rows)) => Some(Size::new(columns, rows)),
            _ => None,
        }
    }

    /// Detect the terminal size.
    ///
    /// Get the terminal size from the underlying TTY, and fallback to
    /// `$COLUMNS` and `$LINES`.
    pub fn detect() -> Option<Size> {
        term_size::dimensions()
            .map(|(w, h)| Size::new(w, h))
            .or_else(Size::from_env)
    }
}
