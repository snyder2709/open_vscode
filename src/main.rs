use std::io::{self, Write};
use std::process::Command;
use which::which;

fn main() {
    // Параметры подключения
    let user = "ilya_v";
    let host = "192.168.0.2";
    let remote_folder = "/home/ilya_v/projects/grenmag-frotend/";
    // Формирование строки для подключения
    let folder_uri = format!(
        "vscode-remote://ssh-remote+{}@{}{}",
        user, host, remote_folder
    );

    println!("Folder URI: {}", folder_uri); // Отладка: вывод значения folder_uri

    // Проверка пути к code.cmd
    match which("code") {
        Ok(code_path) => {
            println!("Path to code: {:?}", code_path); // Отладка: вывод пути к code.cmd
                                                       // Запуск VS Code
            match Command::new(code_path)
                .arg("--folder-uri")
                .arg(folder_uri)
                .spawn()
            {
                Ok(_) => println!("VS Code started successfully."),
                Err(e) => eprintln!("Failed to start VS Code: {}", e),
            }
        }
        Err(_) => eprintln!("Failed to find 'code' in PATH."),
    }

    // Ожидание ввода пользователя перед закрытием
    println!("Press Enter to exit...");
    let _ = io::stdout().flush(); // Убедитесь, что сообщение выведено
    let _ = io::stdin().read_line(&mut String::new());
}
