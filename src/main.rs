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
		(@arg APP_ID: --uuid +takes_value "App UUID (Windows only)")
		(@arg APP_NAME: -H --human +takes_value "Human name of the installer")
		(@arg SIGN: -R "Whether to sign the created installer")
		(@arg CERTIFICATE: -c +takes_value "Certificate to be used for signing")
		(@arg OUTPUT: -o --output +takes_value default_value(".") "Output directory")
		(@arg PACKAGE_VERSION: -V +takes_value default_value("1.0.0") "Package version")
		(@arg PACKAGE_BUILD: -B +takes_value default_value("1") "Package build number")
		(@arg USER: -U "Create an installer for an unprivileged user (Windows only)")

		(@subcommand speller =>
			(about: "Build speller installers")
			(@arg TAG: -l +takes_value +required "Language tag in BCP 47 format (eg: sma-Latn-NO)")
			(@arg APP_CODE_SIGN_ID: -a +takes_value "App Developer ID code sign identifier")
			(@arg INSTALLER_CODE_SIGN_ID: -i +takes_value "Installer Developer ID code sign identifier")
			(@arg ZHFST: -z --zhfst +takes_value +required "ZHFST (Speller) file (eg: se.zhfst)")
		)
		(@subcommand checker =>
			(about: "Build spell checker installers")
			(@arg PACKAGE: -P +takes_value +required "Path to the DLL or package")
		)
	)
	.get_matches();

	// let file = File::open("./se.zhfst").unwrap();
	// let mut archive = ZipArchive::new(file).unwrap();

	// if is_compressed(&mut archive) {
	//     archive = create_stored_zip(&mut archive);
	// }

	let output_path = Path::new(matches.value_of("OUTPUT").unwrap());

	let sign = matches.is_present("SIGN");
	let certificate_path = match sign {
		true => Some(Path::new(
			matches
				.value_of("CERTIFICATE")
				.expect("valid certificate path"),
		)),
		_ => None,
	};

	let target = matches.value_of("TARGET");
	let user = matches.is_present("USER");

	let app_name = matches.value_of("APP_NAME").expect("a valid app name");

	let package_version = matches.value_of("PACKAGE_VERSION").unwrap();
	let package_build = matches
		.value_of("PACKAGE_BUILD")
		.and_then(|v| v.parse::<u64>().ok())
		.expect("a valid build number");

	match matches.subcommand() {
		("speller", Some(sub_c)) => {
			let lang_tag = sub_c.value_of("TAG").expect("a valid BCP47 language tag");
			let zhfst_path = Path::new(sub_c.value_of("ZHFST").unwrap());
			match target {
				Some("osx") => {
					let app_code_sign_id = sub_c.value_of("APP_CODE_SIGN_ID").unwrap();
					let inst_code_sign_id = sub_c.value_of("INSTALLER_CODE_SIGN_ID").unwrap();
					println!("Building Mac installer...");
					targets::osx::create_installer(
						lang_tag,
						package_version,
						package_build,
						zhfst_path,
						output_path,
						inst_code_sign_id,
						app_code_sign_id
					);
				}
				Some("win") => {
					let app_id = match matches.value_of("APP_ID") {
						Some(v) => v,
						None => {
							eprintln!("No UUID provided!");
							return;
						}
					};

					println!("Building Windows installer for {} speller", lang_tag);
					targets::win::create_installer_speller(
						app_id,
						app_name,
						lang_tag,
						package_version,
						package_build,
						zhfst_path,
						output_path,
						certificate_path
					);
				}
				_ => (),
			}
		}
		("checker", Some(sub_c)) => match target {
			Some("win") => {
				let app_id = matches.value_of("APP_ID").expect("valid UUID");
				let dll_path = Path::new(sub_c.value_of("PACKAGE").expect("valid DLL path"));

				println!("Building Windows installer for spell checker...");
				targets::win::create_installer_spellchecker(
					app_id,
					app_name,
					dll_path,
					package_version,
					package_build,
					output_path,
					certificate_path
				);
			}
			Some("osx") => unimplemented!(),
			_ => (),
		},
		_ => eprintln!("Invalid subcommand"),
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
