use super::glob;
use dmi::error;

pub fn smooth_dir_to_combination_key(smooth_dirs: u8, is_diagonal: bool) -> u8 {
	let mut combination_key = glob::NONE;
	for dir in glob::ADJ_CARDINALS.iter() {
		if smooth_dirs & dir != glob::NONE {
			combination_key |= dir;
		}
	}
	if !is_diagonal {
		return combination_key;
	}
	if smooth_dirs & glob::ADJ_N != glob::NONE {
		if smooth_dirs & glob::ADJ_E != glob::NONE && smooth_dirs & glob::ADJ_NE != glob::NONE {
			combination_key |= glob::ADJ_NE;
		}
		if smooth_dirs & glob::ADJ_W != glob::NONE && smooth_dirs & glob::ADJ_NW != glob::NONE {
			combination_key |= glob::ADJ_NW;
		}
	}
	if smooth_dirs & glob::ADJ_S != glob::NONE {
		if smooth_dirs & glob::ADJ_E != glob::NONE && smooth_dirs & glob::ADJ_SE != glob::NONE {
			combination_key |= glob::ADJ_SE;
		}
		if smooth_dirs & glob::ADJ_W != glob::NONE && smooth_dirs & glob::ADJ_SW != glob::NONE {
			combination_key |= glob::ADJ_SW;
		}
	}
	combination_key
}

pub fn smooth_dir_to_corner_type(corner: u8, smooth_dirs: u8) -> u8 {
	match corner {
		glob::NE_INDEX => {
			if smooth_dirs & glob::ADJ_N == glob::NONE {
				if smooth_dirs & glob::ADJ_E == glob::NONE {
					return glob::CONVEX;
				}
				return glob::HORIZONTAL;
			}
			if smooth_dirs & glob::ADJ_E == glob::NONE {
				return glob::VERTICAL;
			}
			if smooth_dirs & glob::ADJ_NE == glob::NONE {
				return glob::CONCAVE;
			}
			glob::FLAT
		}
		glob::SE_INDEX => {
			if smooth_dirs & glob::ADJ_S == glob::NONE {
				if smooth_dirs & glob::ADJ_E == glob::NONE {
					return glob::CONVEX;
				}
				return glob::HORIZONTAL;
			}
			if smooth_dirs & glob::ADJ_E == glob::NONE {
				return glob::VERTICAL;
			}
			if smooth_dirs & glob::ADJ_SE == glob::NONE {
				return glob::CONCAVE;
			}
			glob::FLAT
		}
		glob::SW_INDEX => {
			if smooth_dirs & glob::ADJ_S == glob::NONE {
				if smooth_dirs & glob::ADJ_W == glob::NONE {
					return glob::CONVEX;
				}
				return glob::HORIZONTAL;
			}
			if smooth_dirs & glob::ADJ_W == glob::NONE {
				return glob::VERTICAL;
			}
			if smooth_dirs & glob::ADJ_SW == glob::NONE {
				return glob::CONCAVE;
			}
			glob::FLAT
		}
		glob::NW_INDEX => {
			if smooth_dirs & glob::ADJ_N == glob::NONE {
				if smooth_dirs & glob::ADJ_W == glob::NONE {
					return glob::CONVEX;
				}
				return glob::HORIZONTAL;
			}
			if smooth_dirs & glob::ADJ_W == glob::NONE {
				return glob::VERTICAL;
			}
			if smooth_dirs & glob::ADJ_NW == glob::NONE {
				return glob::CONCAVE;
			}
			glob::FLAT
		}
		_ => panic!("smooth_dir_to_corner_type called with {}", corner),
	}
}

pub fn dir_offset_signature(icon_signature: u8, byond_dir: u8) -> Result<u8, dmi::error::DmiError> {
	if byond_dir == glob::BYOND_SOUTH {
		return Ok(icon_signature);
	}
	let mut all_junctions = [
		icon_signature & glob::ADJ_N,
		icon_signature & glob::ADJ_S,
		icon_signature & glob::ADJ_E,
		icon_signature & glob::ADJ_W,
		icon_signature & glob::ADJ_NE,
		icon_signature & glob::ADJ_SE,
		icon_signature & glob::ADJ_SW,
		icon_signature & glob::ADJ_NW,
	];
	match byond_dir {
		glob::BYOND_NORTH => {
			//Reverse directions.
			all_junctions[0] <<= 1;
			all_junctions[1] >>= 1;
			all_junctions[2] <<= 1;
			all_junctions[3] >>= 1;
			all_junctions[4] <<= 2;
			all_junctions[5] <<= 2;
			all_junctions[6] >>= 2;
			all_junctions[7] >>= 2;
		}
		glob::BYOND_EAST => {
			//Counter-clockwise 90 degrees rotation.
			all_junctions[0] <<= 3;
			all_junctions[1] <<= 1;
			all_junctions[2] >>= 2;
			all_junctions[3] >>= 2;
			all_junctions[4] <<= 3;
			all_junctions[5] >>= 1;
			all_junctions[6] >>= 1;
			all_junctions[7] >>= 1;
		}
		glob::BYOND_WEST => {
			//Clockwise 90 degrees rotation.
			all_junctions[0] <<= 2;
			all_junctions[1] <<= 2;
			all_junctions[2] >>= 1;
			all_junctions[3] >>= 3;
			all_junctions[4] <<= 1;
			all_junctions[5] <<= 1;
			all_junctions[6] <<= 1;
			all_junctions[7] >>= 3;
		}
		_ => {
			return Err(error::DmiError::Generic(format!(
				"dir_offset_signature called with invalid byond_dir: {}",
				byond_dir
			)))
		}
	};
	let offset_signature = all_junctions[0]
		| all_junctions[1]
		| all_junctions[2]
		| all_junctions[3]
		| all_junctions[4]
		| all_junctions[5]
		| all_junctions[6]
		| all_junctions[7];
	Ok(offset_signature)

	//Reverse the cardinals first.
	//let mut offset_signature = ((icon_signature & 0b0101) << 1) | ((icon_signature & 0b1010) >> 1);
}

///Takes everything that comes before the first dot in the string, discarding the rest.
pub fn trim_path_after_first_dot(mut text: String) -> String {
	let dot_offset = text.find('.').unwrap_or(text.len());
	text.drain(dot_offset..); //.collect();
	text
}

///Takes everything that comes after the last slash (or backslash) in the string, discarding the rest.
pub fn trim_path_before_last_slash(mut text: String) -> String {
	if text.is_empty() {
		return text;
	};
	let is_slash = |c| c == '/' || c == '\\';
	let slash_offset = match text.rfind(is_slash) {
		Some(num) => num + 1,
		None => 0,
	};
	text.drain(..slash_offset);
	text
}
