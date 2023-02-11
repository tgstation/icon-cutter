#![allow(dead_code)]

//Internal modules.
mod config;
mod glob;
mod helpers;

use anyhow::Result;
use dmi::icon;
use image::imageops;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;

fn main() {
	let mut args: Vec<String> = env::args().collect();

	let self_path = args.remove(0);

	let prefs = match config::load_configs(self_path) {
		Ok(thing) => thing,
		Err(_e) => {
			println!("Failed to load configs.\nSolution: add a properly-filled config.yaml file to the folder executing the program. Check the namesake folder for examples.");
			dont_disappear::any_key_to_continue::default();
			return;
		}
	};

	match &prefs.file_to_open {
		Some(thing) => args.push(thing.clone()),
		None => (),
	};

	if args.is_empty() {
		println!("Unable to produce any icons. \nSolution: Either add a file to be opened in the config.yaml file or click and drag one or more files into the executable file.");
		dont_disappear::any_key_to_continue::default();
		return;
	}

	for (icons_built, image_path_string) in args.iter().enumerate() {
		let path = Path::new(&image_path_string);
		let mut file = match File::open(path) {
			Ok(f) => f,
			Err(e) => {
				println!("Wrong file path: {:#?}", e);
				dont_disappear::any_key_to_continue::default();
				return;
			}
		};
		let mut contents = Vec::new();
		if let Err(e) = file.read_to_end(&mut contents) {
			println!("Unable to read file: {:#?}", e);
			dont_disappear::any_key_to_continue::default();
			return;
		};
		let cursor = Cursor::new(contents);

		let mut formatted_file_name = image_path_string.clone();

		let dot_offset = image_path_string
			.find('.')
			.unwrap_or(image_path_string.len());
		formatted_file_name = formatted_file_name.drain(..dot_offset).collect(); //Here we remove everything after the dot. Whether .dmi or .png is the same for us.

		let building_return = build_icons(cursor, formatted_file_name, &prefs, icons_built);
		match building_return {
			Ok(_x) => println!("Icons built successfully."),
			Err(x) => println!("Error building icon: {:#?}", x),
		};
		dont_disappear::any_key_to_continue::default();
	}

	println!("Program finished.");
	dont_disappear::any_key_to_continue::default();
}

fn build_icons(
	input: std::io::Cursor<Vec<u8>>,
	file_string_path: String,
	prefs: &config::PrefHolder,
	icons_built: usize,
) -> Result<bool> {
	let (corners, mounted_prefabs) = prefs.build_corners_and_prefabs(input, &file_string_path)?;

	let possible_icon_states = prepare_icon_states(prefs.is_diagonal);

	let number_of_icon_states = possible_icon_states.len() as u32;
	assert!(
		number_of_icon_states > 0,
		"prepare_icon_states() produced {} results",
		number_of_icon_states
	);

	let icon_directions = if prefs.produce_dirs {
		glob::BYOND_CARDINALS.to_vec()
	} else {
		vec![glob::BYOND_SOUTH]
	};

	let output_name = match &prefs.output_name {
		Some(thing) => {
			if icons_built == 0 {
				thing.to_string()
			} else {
				format!("{}({})", &thing, icons_built + 1)
			}
		}
		None => {
			if file_string_path.is_empty() {
				if icons_built == 0 {
					"output".to_string()
				} else {
					format!("output({})", icons_built + 1)
				}
			} else {
				format!(
					"{}-output",
					helpers::trim_path_before_last_slash(file_string_path.clone())
				)
			}
		}
	};
	let icon_state_name = match &prefs.base_icon_state {
		Some(thing) => thing.clone(),
		None => {
			if file_string_path.is_empty() {
				"icon".to_string()
			} else {
				helpers::trim_path_after_first_dot(helpers::trim_path_before_last_slash(
					file_string_path,
				))
			}
		}
	};

	let mut assembled_icons: config::ImageVecMap = HashMap::new();

	for icon_signature in possible_icon_states.iter() {
		let mut icon_state_images = vec![];
		if mounted_prefabs.contains_key(icon_signature) {
			for frame in 0..prefs.frames_per_state {
				let mut image_frame = image::DynamicImage::new_rgba8(
					prefs.output_icon_size_x,
					prefs.output_icon_size_y,
				);
				imageops::replace(
					&mut image_frame,
					&mounted_prefabs[icon_signature][frame as usize],
					prefs.output_west_start.into(),
					prefs.output_north_start.into(),
				);
				icon_state_images.push(image_frame);
			}
		} else {
			for frame in 0..prefs.frames_per_state {
				let mut image_frame = image::DynamicImage::new_rgba8(
					prefs.output_icon_size_x,
					prefs.output_icon_size_y,
				);
				let corner_img = &corners
					.get(&glob::NW_INDEX)
					.unwrap()
					.get(&helpers::smooth_dir_to_corner_type(
						glob::NW_INDEX,
						*icon_signature,
					))
					.unwrap()[frame as usize];
				imageops::overlay(
					&mut image_frame,
					corner_img,
					prefs.output_west_start.into(),
					prefs.output_north_start.into(),
				);
				let corner_img = &corners
					.get(&glob::NE_INDEX)
					.unwrap()
					.get(&helpers::smooth_dir_to_corner_type(
						glob::NE_INDEX,
						*icon_signature,
					))
					.unwrap()[frame as usize];
				imageops::overlay(
					&mut image_frame,
					corner_img,
					prefs.output_east_start.into(),
					prefs.output_north_start.into(),
				);
				let corner_img = &corners
					.get(&glob::SE_INDEX)
					.unwrap()
					.get(&helpers::smooth_dir_to_corner_type(
						glob::SE_INDEX,
						*icon_signature,
					))
					.unwrap()[frame as usize];
				imageops::overlay(
					&mut image_frame,
					corner_img,
					prefs.output_east_start.into(),
					prefs.output_south_start.into(),
				);
				let corner_img = &corners
					.get(&glob::SW_INDEX)
					.unwrap()
					.get(&helpers::smooth_dir_to_corner_type(
						glob::SW_INDEX,
						*icon_signature,
					))
					.unwrap()[frame as usize];
				imageops::overlay(
					&mut image_frame,
					corner_img,
					prefs.output_west_start.into(),
					prefs.output_south_start.into(),
				);
				icon_state_images.push(image_frame);
			}
		};
		assembled_icons.insert(*icon_signature, icon_state_images);
	}

	let mut icon_states = vec![];

	for icon_signature in possible_icon_states.iter() {
		let mut icon_state_frames = vec![];

		for icon_state_dir in icon_directions.iter() {
			icon_state_frames.extend(
				assembled_icons[&helpers::dir_offset_signature(*icon_signature, *icon_state_dir)?]
					.clone(),
			);
		}

		let delay = prefs.delay.clone();

		icon_states.push(icon::IconState {
			name: format!("{}-{}", &icon_state_name, icon_signature),
			dirs: icon_directions.len() as u8,
			frames: prefs.frames_per_state,
			images: icon_state_frames,
			delay,
			..Default::default()
		})
	}

	let new_icon = icon::Icon {
		version: Default::default(),
		width: prefs.output_icon_size_x,
		height: prefs.output_icon_size_y,
		states: icon_states,
	};
	let output_name = format!("{}.dmi", output_name);
	let dmi_path = Path::new(&output_name);
	let mut file = File::create(dmi_path)?;
	new_icon.save(&mut file)?;

	println!(
		"{} icon states produced, with {} frames each, for a total of {} frames.",
		number_of_icon_states,
		prefs.frames_per_state,
		number_of_icon_states * prefs.frames_per_state
	);
	Ok(true)
}

fn prepare_icon_states(is_diagonal: bool) -> Vec<u8> {
	let mut icon_variations: Vec<u8> = vec![];
	for smooth_dirs in glob::NONE..=glob::ADJ_ALL {
		let combination_key = helpers::smooth_dir_to_combination_key(smooth_dirs, is_diagonal);
		if icon_variations.contains(&combination_key) {
			continue;
		};
		icon_variations.push(combination_key);
	}
	icon_variations.sort_unstable();
	icon_variations
}
