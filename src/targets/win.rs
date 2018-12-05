use log::Level::Info;
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
	pfx_path: Option<&Path>,
) {
	fs::create_dir_all(output_dir).expect("output dir to be created");

	let zhfst_file = zhfst_file.to_str().expect("valid zhfst path");
	let installer_path = output_dir.join("installer.iss");

	let app_name = format!("Divvun Spellers - Speller Dictionary {}", bcp47code);
	let sign_pfx_password = pfx_path.as_ref().map(|_| get_pfx_password());

	let speller_path = output_dir.join("speller.zhfst");
	info!("Copying speller archive");
	fs::copy(zhfst_file, &speller_path).expect("zhfst file to copy successfully");

	{
		let mut iss_file = File::create(&installer_path).expect("iss file to be created");
		info!("Generating InnoSetup script");
		iss_file
			.write_all(
				make_iss(
					app_id,
					&app_name,
					bcp47code,
					version,
					build,
					pfx_path,
					sign_pfx_password,
				)
				.as_bytes(),
			)
			.expect("iss file to be written");
	}

	let iscc = get_inno_setup_path()
		.expect("Valid Inno Setup path")
		.join("ISCC.exe");

	let iss_path = wine_path(&installer_path).expect("valid path to installer");

	info!("Building installer binary..");

	let output = wine_cmd!(iscc)
		.arg(format!("/O{}", output_dir.to_str().unwrap()))
		.arg("/Ssigntool=$p")
		.arg(&iss_path)
		.output()
		.expect("process to spawn");

	info!("ISCC output");
	info!("{}", std::str::from_utf8(&output.stdout).unwrap());

	if !log_enabled!(Info) {
		fs::remove_file(installer_path).expect("to remove installer script");
	}

	fs::remove_file(speller_path).expect("to remove speller file");

	if !output.status.success() {
		eprintln!("ISCC failed!");
		eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
		return;
	}

	let installer_name = format!("divvun-spellers-dict-{}.exe", bcp47code);
	fs::rename(
		output_dir.join("install.exe"),
		output_dir.join(installer_name),
	)
	.expect("to rename installer executable");
}

fn iss_setup_signtool(app_name: &str, pfx_path: &Path, sign_pfx_password: &str) -> String {
	let signtool_path = get_signtool_path();
	let pfx_path_wine = wine_path(pfx_path).expect("valid PFX path");
	if cfg!(windows) {
		format!(
			"SignTool=signtool {signtool_path} sign \
			 /t http://timestamp.verisign.com/scripts/timstamp.dll \
			 /f $q{pfx_path}$q \
			 /p $q{sign_pfx_password}$q \
			 /d $q{app_name}$q $f",
			pfx_path = pfx_path_wine,
			app_name = app_name,
			sign_pfx_password = sign_pfx_password,
			signtool_path = signtool_path
		)
	} else {
		format!(
			"SignTool=signtool cmd /c {signtool_path} sign \
			 -pkcs12 $q{pfx_path}$q \
			 -pass $q{sign_pfx_password}$q \
			 -n $q{app_name}$q \
			 -t http://timestamp.verisign.com/scripts/timstamp.dll \
			 $f \
			 signed && del $f && move signed $f",
			pfx_path = pfx_path_wine,
			app_name = app_name,
			sign_pfx_password = sign_pfx_password,
			signtool_path = signtool_path
		)
	}
}

fn get_signtool_path() -> String {
	if cfg!(windows) {
		"SignTool".to_string()
	} else {
		wine_path(Path::new(
			&env::var("OSSLSIGNCODE_PATH")
				.expect("OSSLSIGNCODE_PATH to point to win32 osslsigncode"),
		))
		.unwrap()
	}
}

fn get_pfx_password() -> String {
	env::var("SIGN_PFX_PASSWORD").expect("SIGN_PFX_PASSWORD environment variable")
}

fn make_iss(
	app_id: &str,
	app_name: &str,
	bcp47code: &str,
	version: &str,
	build: u64,
	pfx_path: Option<&Path>,
	sign_pfx_password: Option<String>,
) -> String {
	format!(
		r#"[Setup]
AppId={app_id}
AppName={app_name}
AppVersion={version}.{build}
DefaultDirName={{pf}}\Divvun Spellers\dictionaries\{bcp47code}
DefaultGroupName=Divvun Spellers
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
OutputBaseFilename=install
{sign_tool}

[Files]
Source: "speller.zhfst"; DestDir: "{{app}}"; DestName: "{bcp47code}.zhfst"
"#,
		app_id = app_id,
		bcp47code = bcp47code,
		version = version,
		build = build,
		app_name = app_name,
		sign_tool = pfx_path.map_or("".to_string(), |path| iss_setup_signtool(
			app_name,
			&path,
			&sign_pfx_password.unwrap()
		))
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
		// InnoSetup can't handle extended length paths, Rust prefix absolute paths with \\?\
		abs_path[4..].to_string()
	} else {
		format!("Z:{}", abs_path.replace("/", "\\"))
	})
}
