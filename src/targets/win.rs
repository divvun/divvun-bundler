use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{self, Path, PathBuf};
use std::process::Command;

macro_rules! wine_cmd {
	($x:expr) => {{
		if cfg!(windows) {
			Command::new($x)
		} else {
			let mut cmd = Command::new("wine");
			cmd.arg($x);
			cmd
		}
	}};
}

pub fn create_installer(
	app_id: &str,
	bcp47code: &str,
	version: &str,
	build: u64,
	zhfst_file: &Path,
	output_dir: &Path,
) {
	fs::create_dir_all(output_dir).expect("output dir to be created");

	let zhfst_file = zhfst_file.to_str().expect("valid zhfst path");
	let installer_path = output_dir.join("installer.iss");

	info!("Copying speller archive");
	fs::copy(zhfst_file, output_dir.join("speller.zhfst"))
		.expect("zhfst file to copy successfully");

	{
		let mut iss_file = File::create(&installer_path).expect("iss file to be created");
		info!("Generating InnoSetup script");
		iss_file
			.write_all(make_iss(app_id, bcp47code, version, build, zhfst_file).as_bytes())
			.expect("iss file to be written");
	}

	let iscc = get_inno_setup_path()
		.expect("Valid Inno Setup path")
		.join("ISCC.exe");

	let iss_path = wine_path(&installer_path).expect("valid path to installer");

	info!("Building installer binary..");

	let mut process = wine_cmd!(iscc)
		.arg(format!("/O{}", output_dir.to_str().unwrap()))
		.arg(iss_path)
		.spawn()
		.expect("process to spawn");

	process.wait().unwrap();

	let installer_name = format!("divvun-spellers-dict-{}.exe", bcp47code);
	fs::rename(
		output_dir.join("install.exe"),
		output_dir.join(installer_name),
	)
	.expect("to rename installer executable");
}

fn make_iss(app_id: &str, bcp47code: &str, version: &str, build: u64, zhfst_file: &str) -> String {
	format!(
		r#"[Setup]
AppId={app_id}
AppName=Divvun Spellers - Speller Dictionary {bcp47code}
AppVersion={version}.{build}
DefaultDirName={{pf}}\Divvun Spellers\dictionaries\{bcp47code}
DefaultGroupName=Divvun Spellers
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename=install

[Files]
Source: "speller.zhfst"; DestDir: "{{app}}"; DestName: "{bcp47code}.zhfst"
"#,
		app_id = app_id,
		bcp47code = bcp47code,
		version = version,
		build = build
	)
}

fn get_inno_setup_path() -> Option<PathBuf> {
	if let Ok(dir) = env::var("INNO_PATH") {
		return Some(PathBuf::from(dir));
	}

	if cfg!(windows) {
		let alternatives = vec![
			PathBuf::from(r"C:\Program Files\Inno Setup 5"),
			PathBuf::from(r"C:\Program Files (x86)\Inno Setup 5"),
		];

		alternatives.iter().filter(|p| p.is_dir()).next().cloned()
	} else {
		None
	}
}

fn wine_path(path: &Path) -> Option<String> {
	let abs_path = path.canonicalize().ok()?.to_str()?.to_string();
	Some(if cfg!(windows) {
		abs_path
	} else {
		format!("Z:{}", abs_path)
	})
}
