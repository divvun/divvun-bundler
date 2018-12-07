use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

static APP_NAME: &str = "MacDivvun";

pub fn create_installer(
    bcp47code: &str,
    version: &str,
    build: u64,
    zhfst_file: &Path,
    output_dir: &Path,
) {
    let package = "no.divvun.MacDivvun";
    let bundle_name = format!("{}.{}.bundle", package, bcp47code);

    create_bundle(
        &bundle_name,
        package,
        bcp47code,
        version,
        build,
        zhfst_file,
        output_dir,
    );
    create_installer_from_bundle(
        APP_NAME,
        &output_dir.join(bundle_name),
        version,
        package,
        output_dir,
    );
}

pub fn create_bundle(
    bundle_name: &str,
    package: &str,
    bcp47code: &str,
    version: &str,
    build: u64,
    zhfst_file: &Path,
    output_dir: &Path,
) {
    fs::create_dir_all(output_dir).expect("output dir to be created");
    let bundle_dir = output_dir.join(&bundle_name);
    match fs::remove_dir_all(&bundle_dir) {
        Ok(_) => {}
        Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => panic!(err),
    };

    let content_path = bundle_dir.join("Contents");
    let resources_path = content_path.join("Resources");
    fs::create_dir_all(&resources_path).expect("bundle dir to be created");
    fs::copy(zhfst_file, resources_path.join("speller.zhfst"))
        .expect("zhfst file to copy successfully");

    let mut plist_file =
        File::create(content_path.join("Info.plist")).expect("plist file to be created");
    plist_file
        .write_all(make_plist(bcp47code, version, build, APP_NAME, package).as_bytes())
        .expect("plist to be written");
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

fn create_installer_from_bundle(
    app_name: &str,
    bundle_dir: &Path,
    version: &str,
    package: &str,
    output_dir: &Path,
) {
    let component_pkg_name = create_component_package(output_dir, bundle_dir, package, version);

    let distribution_path = output_dir.join("distribution.xml");
    {
        let mut distribution_file =
            File::create(&distribution_path).expect("distribution file to be created");
        distribution_file
            .write_all(make_distribution(&app_name, package, &component_pkg_name).as_bytes())
            .expect("distribution file to be written");
    }

    let pkg_name = format!("{}.unsigned.pkg", package);

    let result = Command::new("productbuild")
        .current_dir(output_dir)
        .arg("--distribution")
        .arg(distribution_path)
        .arg("--version")
        .arg(version)
        // .arg("--package-path")
        // .arg(output_dir)
        .arg(pkg_name)
        .status()
        .expect("productbuild to execute successfully");

    if !result.success() {
        panic!("productbuild failed");
    }
}

fn create_component_package(
    output_dir: &Path,
    bundle_dir: &Path,
    package: &str,
    version: &str,
) -> String {
    let pkg_name = format!("{}.pkg", package);

    let result = Command::new("pkgbuild")
        .current_dir(output_dir)
        .arg("--component")
        .arg(bundle_dir)
        .arg("--ownership")
        .arg("recommended")
        .arg("--install-location")
        .arg("/Library/Services")
        .arg("--version")
        .arg(version)
        .arg(&pkg_name)
        .status()
        .expect("pkgbuild to complete successfully");

    if !result.success() {
        panic!("pkgbuild failed");
    }

    pkg_name
}

fn make_distribution(app_name: &str, package: &str, component_package: &str) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<installer-gui-script minSpecVersion="2">
	<title>{app_name}</title>
	<options customize="never" rootVolumeOnly="true"/>
	<choices-outline>
    	<line choice="default">
      		<line choice="{package}"/>
    	</line>
  	</choices-outline>

	<choice id="default" />
    <choice id="{package}" visible="false">
		<pkg-ref id="{package}"/>
	</choice>

	<pkg-ref id="{package}" onConclusion="RequireRestart" version="0" auth="root">{component_package}</pkg-ref>
</installer>"#, app_name = app_name, package = package, component_package = component_package)
}
