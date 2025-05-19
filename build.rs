use std::{
    env::var,
    ffi::OsStr,
    fs,
    io::Result,
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn main() {
    if !should_skip_shader_compilation() {
        compile_shaders();
    }
}

fn should_skip_shader_compilation() -> bool {
    var("SKIP_SHADER_COMPILATION")
        .map(|var| var.parse::<bool>().unwrap_or(false))
        .unwrap_or(false)
}

fn compile_shaders() {
    println!("Compiling shaders");

    let shader_dir_path = get_shader_source_dir_path();

    fs::read_dir(shader_dir_path.clone())
        .unwrap()
        .map(Result::unwrap)
        .filter(|dir| dir.file_type().unwrap().is_file())
        .filter(|dir| dir.path().extension() != Some(OsStr::new("spv")) && dir.path().extension() != Some(OsStr::new("hlsl")) && dir.path().extension() != Some(OsStr::new("md")))
        .for_each(|dir| {
            let path = dir.path();
            let name = path.file_name().unwrap().to_str().unwrap();
            println!("Found file {:?}.\nCompiling...", path.as_os_str());

            // this is wrong
            // let stage = name.strip_prefix("shader_stage.").unwrap();
            let output_name = format!("../assets/shaders/{}.spv", &name);

            let result = Command::new("glslangValidator")
                .current_dir(&shader_dir_path)
                .arg("-V")
                .arg(&path)
                .arg("-o")
                .arg(output_name)
                .arg("-e")
                .arg("main")
                // .arg("-S")
                // .arg(stage)
                .output();

            handle_program_result(result);
        });
}

fn get_shader_source_dir_path() -> PathBuf {
    let path = get_root_path().join("shaders");
    println!("Shader source directory: {:?}", path.as_os_str());
    path
}

fn get_root_path() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn handle_program_result(result: Result<Output>) {
    match result {
        Ok(output) => {
            if output.status.success() {
                println!("Shader compilation succedeed.");
                print!(
                    "stdout: {}",
                    String::from_utf8(output.stdout).unwrap_or("Failed to print program stdout".to_string())
                );
            } else {
                eprintln!("Shader compilation failed. Status: {}", output.status);
                eprint!(
                    "stdout: {}",
                    String::from_utf8(output.stdout).unwrap_or("Failed to print program stdout".to_string())
                );
                eprint!(
                    "stderr: {}",
                    String::from_utf8(output.stderr).unwrap_or("Failed to print program stderr".to_string())
                );
                panic!("Shader compilation failed. Status: {}", output.status);
            }
        }
        Err(error) => {
            panic!("Failed to compile shader. Cause: {}", error);
        }
    }
}
