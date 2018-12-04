extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use clap::{clap_app, crate_authors, crate_version};

use std::io::{Read, Seek, Write};
use std::path::Path;

use zip::{CompressionMethod, ZipArchive};

use divvun_bundler::*;

fn main() {
	env_logger::init();

	let matches = clap_app!(app =>
		(version: crate_version!())
		(author: crate_authors!())
		(about: "Bundle stuff")
		(@arg TARGET: -t --target +required +takes_value
			possible_value("win") possible_value("osx")
			"Target platform")
		(@arg TAG: -l +takes_value +required "Language tag in BCP 47 format (eg: sma-Latn-NO)")
		(@arg ZHFST: -z --zhfst +takes_value +required "ZHFST (Speller) file (eg: se.zhfst)")
		(@arg OUTPUT: -o --output +takes_value default_value(".") "Output directory")
		(@arg BUNDLE_VERSION: -V +takes_value default_value("1.0.0") "Bundle version")
		(@arg BUNDLE_BUILD: -B +takes_value default_value("1") "Bundle build")
		(@arg APP_ID: --uuid +takes_value "App UUID (Windows only)")
		(@arg SIGN: -R "Whether to sign the created installer")
		(@arg CERTIFICATE: -c +takes_value "Certificate to be used for signing")
	)
	.get_matches();

	// TODO: use Clap for CLI

	// let file = File::open("./se.zhfst").unwrap();
	// let mut archive = ZipArchive::new(file).unwrap();

	// if is_compressed(&mut archive) {
	//     archive = create_stored_zip(&mut archive);
	// }
	let lang_tag = matches.value_of("TAG").expect("a valid BCP47 language tag");
	let bundle_version = matches.value_of("BUNDLE_VERSION").unwrap();
	let bundle_build = matches
		.value_of("BUNDLE_BUILD")
		.and_then(|v| v.parse::<u64>().ok())
		.expect("a valid build number");

	let zhfst_path = Path::new(matches.value_of("ZHFST").unwrap());
	let output_path = Path::new(matches.value_of("OUTPUT").unwrap());

	let sign = matches.is_present("SIGN");
	let certificate_path = match sign {
		true => Some(Path::new(matches.value_of("CERTIFICATE").expect("valid certificate path"))),
		_ => None
	};

	match matches.value_of("TARGET") {
		Some("osx") => {
			println!("Building Mac bundle...");
			targets::osx::create_bundle(
				lang_tag,
				bundle_version,
				bundle_build,
				zhfst_path,
				output_path,
			);
		}
		Some("win") => {
			let app_id = matches.value_of("APP_ID").expect("valid UUID");

			println!("Building Windows installer...");
			targets::win::create_installer(
				app_id,
				lang_tag,
				bundle_version,
				bundle_build,
				zhfst_path,
				output_path,
				certificate_path
			);
		}
		Some(v) => {
			error!("Invalid target: {}", v);
		}
		_ => (),
	}
}

fn is_compressed<R: Read + Seek>(archive: &mut ZipArchive<R>) -> bool {
	for i in 0..archive.len() {
		let record = archive.by_index(i).unwrap();

		if record.compression() != CompressionMethod::Stored {
			return true;
		}
	}

	false
}

fn create_stored_zip<R: Read + Seek>(archive: &mut ZipArchive<R>) -> ZipArchive<R> {
	unimplemented!()
}
