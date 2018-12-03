#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap; 
use clap::{clap_app, crate_version, crate_authors};
 
use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Seek, Write};

use zip::{ZipArchive, CompressionMethod};

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
	).get_matches();

	
	// TODO: use Clap for CLI
	
    // let file = File::open("./se.zhfst").unwrap();
    // let mut archive = ZipArchive::new(file).unwrap();

    // if is_compressed(&mut archive) {
    //     archive = create_stored_zip(&mut archive);
    // }

	match matches.value_of("TARGET") {
		Some("osx") => {
			let lang_tag = matches.value_of("TAG").expect("a valid BCP47 language tag");
			let bundle_version = matches.value_of("BUNDLE_VERSION").unwrap();
			let bundle_build = matches.value_of("BUNDLE_BUILD").and_then(|v| v.parse::<u64>().ok()).expect("a valid build number");
			let zhfst_path = Path::new(matches.value_of("ZHFST").unwrap());
			let output_path = Path::new(matches.value_of("OUTPUT").unwrap());

			println!("Building Mac bundle...");
			create_bundle(lang_tag, bundle_version, bundle_build, zhfst_path, output_path);
		},
		Some(v) => {
			error!("Invalid target: {}", v);
		},
		_ => ()
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

fn create_bundle(bcp47code: &str, version: &str, build: u64, zhfst_file: &Path, output_dir: &Path) {
    let package = "no.divvun.MacDivvun";
    let bundle_name = format!("{}.{}.bundle", package, bcp47code);

    fs::create_dir_all(output_dir).expect("output dir to be created");
    let bundle_dir = output_dir.join(&bundle_name);
    match fs::remove_dir_all(&bundle_dir) {
        Ok(_) => {},
        Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {},
        Err(err) => panic!(err)
    };

    let content_path = bundle_dir.join("Contents");
    let resources_path = content_path.join("Resources");
    fs::create_dir_all(&resources_path).expect("bundle dir to be created");
    fs::copy(zhfst_file, resources_path.join("speller.zhfst")).expect("zhfst file to copy successfully");

    let mut plist_file = File::create(content_path.join("Info.plist")).expect("plist file to be created");
    plist_file.write_all(make_plist(bcp47code, version, build, "MacDivvun", package).as_bytes()).expect("plist to be written");
}

fn make_plist(bcp47code: &str, version: &str, build: u64, app_name: &str, package: &str) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleDevelopmentRegion</key>
	<string>en</string>
	<key>CFBundleIdentifier</key>
	<string>{package}.{bcp47code}</string>
	<key>CFBundleName</key>
	<string>{bcp47code}</string>
	<key>CFBundlePackageType</key>
	<string>BNDL</string>
	<key>CFBundleShortVersionString</key>
	<string>{version}</string>
	<key>CFBundleSupportedPlatforms</key>
	<array>
		<string>MacOSX</string>
	</array>
	<key>CFBundleVersion</key>
	<string>{build}</string>
	<key>NSHumanReadableCopyright</key>
	<string>See license file.</string>
	<key>NSServices</key>
	<array>
		<dict>
			<key>NSExecutable</key>
			<string>{app_name}</string>
			<key>NSLanguages</key>
			<array>
				<string>{bcp47code}</string>
			</array>
			<key>NSMenuItem</key>
			<dict/>
			<key>NSPortName</key>
			<string>{app_name}</string>
			<key>NSSpellChecker</key>
			<string>{app_name}</string>
		</dict>
	</array>
</dict>
</plist>
"#, package = package, bcp47code = bcp47code, app_name = app_name, version = version, build = build)
}