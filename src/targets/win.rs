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

pub fn create_installer_speller(
	app_id: &str,
	app_name: &str,
	bcp47code: &str,
	version: &str,
	build: u64,
	zhfst_file: &Path,
	output_dir: &Path,
	pfx_path: Option<&Path>
) {
	fs::create_dir_all(output_dir).expect("output dir to be created");

	let zhfst_file = zhfst_file.to_str().expect("valid zhfst path");
	let installer_path = output_dir.join("installer.nsi");

	let sign_pfx_password = pfx_path.as_ref().map(|_| get_pfx_password());

	let speller_path = output_dir.join("speller.zhfst");
	info!("Copying speller archive");
	fs::copy(zhfst_file, &speller_path).expect("zhfst file to copy successfully");

	{
		let mut nsi_file = File::create(&installer_path).expect("nsi file to be created");
		info!("Generating NSIS script");
		nsi_file
			.write_all(
				make_nsi_speller(
					app_id,
					&app_name,
					bcp47code,
					version,
					build,
					pfx_path,
					sign_pfx_password,
					output_dir,
				)
				.as_bytes(),
			)
			.expect("nsi file to be written");
	}

	let nsis = match get_nsis_path() {
		Some(v) => v.join("makensis.exe"),
		None => {
			eprintln!("NSIS 3 not found, and NSIS_PATH not set.");
			return;
		}
	};

	let nsis_path = wine_path(&installer_path).expect("valid path to installer");

	info!("Building installer binary...");

	let output = wine_cmd!(nsis)
		.current_dir(output_dir)
		.arg(format!(
			"/XOutFile {}\\installer.exe",
			output_dir.to_str().unwrap()
		))
		.arg(&nsis_path)
		.output()
		.expect("process to spawn");

	info!("NSIS output");
	info!("{}", std::str::from_utf8(&output.stdout).unwrap());

	if !log_enabled!(Info) {
		fs::remove_file(installer_path).expect("to remove installer script");
	}

	fs::remove_file(speller_path).expect("to remove speller file");

	if !output.status.success() {
		eprintln!("NSIS failed!");
		eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
		return;
	}

	let installer_name = format!("{}-{}.exe", bcp47code, version);
	fs::rename(
		output_dir.join("installer.exe"),
		output_dir.join(installer_name),
	)
	.expect("to rename installer executable");
}

pub fn create_installer_spellchecker(
	app_id: &str,
	app_name: &str,
	dll_path: &Path,
	version: &str,
	build: u64,
	output_dir: &Path,
	pfx_path: Option<&Path>
) {
	fs::create_dir_all(output_dir).expect("output dir to be created");
	let installer_name = format!("windivvun-{}.exe", version);

	let nsis = match get_nsis_path() {
		Some(v) => v.join("makensis.exe"),
		None => {
			eprintln!("NSIS 3 not found, and NSIS_PATH not set.");
			return;
		}
	};

	let installer_path = output_dir.join("installer.nsi");

	let sign_pfx_password = pfx_path.as_ref().map(|_| get_pfx_password());

	let dll_path_out = output_dir.join("windivvun.dll");
	info!("Copying spell checker DLL");
	fs::copy(dll_path, &dll_path_out).expect("spell checker DLL to copy successfully");

	{
		let mut nsi_file = File::create(&installer_path).expect("nsi file to be created");
		info!("Generating NSIS script");
		nsi_file
			.write_all(
				make_nsi_spellchecker(
					app_id,
					&app_name,
					version,
					build,
					pfx_path,
					sign_pfx_password
				)
				.as_bytes(),
			)
			.expect("nsi file to be written");
	}

	let nsis_path = wine_path(&installer_path).expect("valid path to installer");

	info!("Building installer binary..");

	let output = wine_cmd!(nsis)
		.current_dir(output_dir)
		.arg(format!(
			"/XOutFile {}\\installer.exe",
			output_dir.to_str().unwrap()
		))
		.arg(&nsis_path)
		.output()
		.expect("process to spawn");

	info!("NSIS output");
	info!("{}", std::str::from_utf8(&output.stdout).unwrap());

	if !log_enabled!(Info) {
		fs::remove_file(installer_path).expect("to remove installer script");
	}

	fs::remove_file(dll_path_out).expect("to remove spell checker DLL");

	if !output.status.success() {
		eprintln!("NSIS failed!");
		eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
		return;
	}

	fs::rename(
		output_dir.join("installer.exe"),
		output_dir.join(installer_name),
	)
	.expect("to rename installer executable");
}

fn nsis_setup_signtool(
	app_name: &str,
	pfx_path: &Path,
	sign_pfx_password: &str,
	file_name: &str,
) -> String {
	let signtool_path = get_signtool_path();
	let pfx_path_wine = wine_path(pfx_path).expect("valid PFX path");
	if cfg!(windows) {
		format!(
			"{signtool_path} sign \
			 /t http://timestamp.verisign.com/scripts/timstamp.dll \
			 /f \"{pfx_path}\" \
			 /p \"{sign_pfx_password}\" \
			 /d \"{app_name}\" {file_name}",
			pfx_path = pfx_path_wine,
			app_name = app_name,
			sign_pfx_password = sign_pfx_password,
			signtool_path = signtool_path,
			file_name = file_name
		)
	} else {
		format!(
			"{signtool_path} sign \
			 -pkcs12 \"{pfx_path}\" \
			 -pass \"{sign_pfx_password}\" \
			 -n \"{app_name}\" \
			 -t http://timestamp.verisign.com/scripts/timstamp.dll \
			 -in {file_name} \
			 -out signed && del {file_name} && move signed {file_name}",
			pfx_path = pfx_path_wine,
			app_name = app_name,
			sign_pfx_password = sign_pfx_password,
			signtool_path = signtool_path,
			file_name = file_name
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

fn make_nsi_speller(
	app_id: &str,
	app_name: &str,
	bcp47code: &str,
	version: &str,
	build: u64,
	pfx_path: Option<&Path>,
	sign_pfx_password: Option<String>,
	output_dir: &Path,
) -> String {
	let sign_tool = match pfx_path {
		Some(_) => nsis_setup_signtool(
			app_name,
			pfx_path.unwrap(),
			&sign_pfx_password.as_ref().unwrap(),
			"installer.exe",
		),
		None => "rem".to_string(),
	};

	let sign_tool_uninstaller = match pfx_path {
		Some(_) => nsis_setup_signtool(
			app_name,
			pfx_path.unwrap(),
			&sign_pfx_password.unwrap(),
			"uninstall.exe",
		),
		None => "rem".to_string(),
	};

	format!(
		include_str!("./win-speller.nsi"),
		app_id = app_id,
		app_name = app_name,
		bcp47code = bcp47code,
		sign_tool = sign_tool,
		sign_tool_uninstaller = sign_tool_uninstaller,
		version = version,
		build = build,
	)
}

fn make_nsi_spellchecker(
	app_id: &str,
	app_name: &str,
	version: &str,
	build: u64,
	pfx_path: Option<&Path>,
	sign_pfx_password: Option<String>
) -> String {
	let sign_tool = match pfx_path {
		Some(_) => nsis_setup_signtool(
			app_name,
			pfx_path.unwrap(),
			&sign_pfx_password.as_ref().unwrap(),
			"installer.exe",
		),
		None => "rem".to_string(),
	};

	let sign_tool_uninstaller = match pfx_path {
		Some(_) => nsis_setup_signtool(
			app_name,
			pfx_path.unwrap(),
			&sign_pfx_password.unwrap(),
			"uninstall.exe",
		),
		None => "rem".to_string(),
	};

	format!(
		include_str!("./win-spellchecker.nsi"),
		app_id = app_id,
		app_name = app_name,
		version = version,
		build = build,
		sign_tool = sign_tool,
		sign_tool_uninstaller = sign_tool_uninstaller
	)
}

fn get_nsis_path() -> Option<PathBuf> {
	if let Ok(dir) = env::var("NSIS_PATH") {
		return Some(PathBuf::from(dir));
	}

	if cfg!(windows) {
		let alternatives = vec![
			PathBuf::from(r"C:\Program Files\NSIS"),
			PathBuf::from(r"C:\Program Files (x86)\NSIS"),
		];

		alternatives.iter().filter(|p| p.is_dir()).next().cloned()
	} else {
		None
	}
}

fn wine_path(path: &Path) -> Option<String> {
	let abs_path = path.canonicalize().ok()?.to_str()?.to_string();
	Some(if cfg!(windows) {
		// InnoSetup can't handle extended length paths, Rust prefixes absolute paths with \\?\
		abs_path[4..].to_string()
	} else {
		format!("Z:{}", abs_path.replace("/", "\\"))
	})
}
