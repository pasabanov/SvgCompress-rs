//! svgc is a tool for compressing SVG files
//! Copyright (C) © 2024  Petr Alexandrovich Sabanov
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Affero General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Affero General Public License for more details.
//!
//! You should have received a copy of the GNU Affero General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use flate2::{Compression, write::GzEncoder};

pub fn compress_to_svgz(filepath: &Path) -> io::Result<PathBuf> {

	let file = fs::File::open(filepath)?;
	let reader = io::BufReader::new(file);

	let svgz_filepath = filepath.with_extension("svgz");

	let file = fs::File::create(&svgz_filepath)?;
	let mut encoder = GzEncoder::new(file, Compression::best());

	// Copy contents from reader to encoder
	io::copy(&mut reader.take(u64::MAX), &mut encoder)?;

	encoder.finish()?;
	fs::remove_file(filepath)?;
	Ok(svgz_filepath)
}