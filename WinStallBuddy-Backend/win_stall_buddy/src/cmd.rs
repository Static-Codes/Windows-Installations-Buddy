use std::io::{Error, ErrorKind, Write};
use std::process::{Command, Output};

//let _ = run_cmd("download", "vivaldi");

pub fn run_cmd(action_arg: &str, app_name: &str) {
    // let result: Result<Output, Box<dyn std::error::Error>> = run_cmd("download", "vivaldi");
    let output_result: Result<Output, Error> = Command::new("DU_CLI.exe")
        .args([action_arg, app_name])
        .output();

    if output_result.is_err() {
        let unwrapped_error = output_result.unwrap_err();
        handle_errors(unwrapped_error);
    }
    //else if output_result.{}
    else {
        let value = output_result.unwrap();
        let _ = std::io::stdout().write_all(&value.stdout);
    }
}

pub fn handle_errors(unwrapped_error: Error) {
    let error_obj = unwrapped_error.raw_os_error();
    if !error_obj.is_none() {
        let error_code = error_obj.unwrap();

        // ErrorKind::NotFound
        if error_code == 101 {
            println!(
                "DU_CLI.exe was not found in the same directory was wsb.exe, please open DU_CLI.sln and compile using the following commands:"
            );
            println!(
                "dotnet clean\ndotnet publish -c Release -r win-x86 /p:PublishSingleFile=true --self-contained true"
            );
            println!(
                "Ensure the selenium-manager, and DU_CLI.exe are present in the same directory as wsb.exe"
            );
        }
        // ErrorKind::Uncategorized -> UNSTABLE see error_string.contains
        else if error_code == 216 {
            println!("This application was compiled for x86 based Windows systems.\n");
            println!("If you see this error, it means either:");
            println!(
                "-> You're attempting to run this application on non x86 hardware like x64 or ARM."
            );
            println!("-> The contents of DU_CLI.exe was modified or corrupted.\n");
            println!("Please open DU_CLI.sln and compile using the following commands:\n");
            println!(
                "dotnet clean\ndotnet publish -c Release -r win-x86 /p:PublishSingleFile=true --self-contained true\n"
            );
            println!(
                "Ensure the selenium-manager, and DU_CLI.exe are present in the same directory as wsb.exe"
            );
        } else {
            println!("Please write code to handle the following error:");
            println!("{:?}", unwrapped_error.kind());
            println!("{}", unwrapped_error);
        }
    } else {
        let kind_obj: ErrorKind = unwrapped_error.kind();
        let error_string: String = unwrapped_error.to_string();
        if kind_obj == ErrorKind::NotFound {
            println!(
                "DU_CLI.exe was not found in the same directory was wsb.exe, please open DU_CLI.sln and compile using the following commands:"
            );
            println!(
                "dotnet clean\ndotnet publish -c Release -r win-x86 /p:PublishSingleFile=true --self-contained true"
            );
            println!(
                "Ensure the selenium-manager, and DU_CLI.exe are present in the same directory as wsb.exe"
            );
        }
        // Since ErrorKind::Uncategorized is unstable, and raw_os_error() can panic, this is the best solution ive found.
        else if error_string.contains(
            "Check your computer's system information and then contact the software publisher.",
        ) {
            println!("This application was compiled for x86 based Windows systems.\n");
            println!("If you see this error, it means either:");
            println!(
                "-> You're attempting to run this application on non x86 hardware like x64 or ARM."
            );
            println!("-> The contents of DU_CLI.exe was modified or corrupted.\n");
            println!("Please open DU_CLI.sln and compile using the following commands:\n");
            println!(
                "dotnet clean\ndotnet publish -c Release -r win-x86 /p:PublishSingleFile=true --self-contained true\n"
            );
            println!(
                "Ensure the selenium-manager, and DU_CLI.exe are present in the same directory as wsb.exe"
            );
        } else {
            println!("Please write code to handle the following error:");
            println!("{:?}", unwrapped_error.kind());
            println!("{}", unwrapped_error);
        }
    }
}
