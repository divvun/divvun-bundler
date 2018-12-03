use std::fs::{self, File};
use std::path::Path;
use std::io::Write;

pub fn create_bundle(bcp47code: &str, version: &str, build: u64, zhfst_file: &Path, output_dir: &Path) {
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
