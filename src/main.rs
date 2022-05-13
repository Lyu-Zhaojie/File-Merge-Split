use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

enum CommandType {
    Merge,
    Split,
    Help,
}
struct Program {
    command_type: CommandType,
    single_file_name: String,
    file_names: Vec<String>,
    parts: usize,
}

impl Program {
    fn new(args: &[String]) -> Result<Program, String> {
        if args.len() < 2 {
            return Err(String::from("参数数目不足."));
        }
        let command_type = match &args[1] as &str {
            "--merge" => CommandType::Merge,
            "--split" => CommandType::Split,
            "--help" => CommandType::Help,
            other => return Err(format!("未知参数: {}", other)),
        };
        let mut single_file_name: String = String::new();
        let mut file_names: Vec<String> = Vec::new();
        let mut parts: usize = 0;
        match command_type {
            CommandType::Split => {
                if args.len() != 4 {
                    return Err(String::from("参数数目错误. 应输入 3 个参数."));
                }
                parts = match usize::from_str(&args[2]) {
                    Ok(value) => {
                        if value > 0 {
                            value
                        } else {
                            return Err(format!("参数不是正整数: {}", args[2]));
                        }
                    }
                    Err(_) => return Err(format!("参数不是正整数: {}", args[2])),
                };
                single_file_name = args[args.len() - 1].to_string();
                for i in 1..=parts {
                    file_names.push(format!("{}{}", single_file_name, i));
                }
            }
            CommandType::Merge => {
                if args.len() < 4 {
                    return Err(String::from("参数数目过少."));
                }
                parts = args.len() - 3_usize;
                file_names = args[2..args.len() - 1].to_vec();
                single_file_name = args.last().unwrap().to_string();
            }
            CommandType::Help => (),
        }
        return Ok(Program {
            command_type,
            single_file_name,
            file_names,
            parts,
        });
    }

    fn help() {
        println!();
        println!("文件分块/合并器");
        println!("--help");
        println!(" * 显示本帮助.");
        println!();
        println!("--merge <input file 1> <input file 2> ... <output file>");
        println!(" * 将 input file 1, input file 2, ... 合并输出至 output file.");
        println!();
        println!("--split <parts> <input file>");
        println!(" * 将 input file 分为 parts 份存储.");
        println!();
    }

    fn merge(&self) {
        let mut save: Vec<Vec<u8>> = Vec::new();
        for p in &self.file_names {
            save.push(match fs::read(p) {
                Ok(input) => input,
                Err(_) => {
                    eprintln!("打开文件时发生了问题: {}", p);
                    process::exit(1);
                }
            })
        }
        let min_len = save.last().unwrap().len();
        let mut write: Vec<u8> = Vec::new();
        for i in 0..min_len {
            for fs in &save {
                write.push(fs[i]);
            }
        }
        for fs in &save {
            if fs.len() == min_len + 1 {
                write.push(*fs.last().unwrap());
            }
        }
        match fs::write(&self.single_file_name, write) {
            Ok(_) => (),
            Err(_) => {
                println!("写入文件时发生了问题: {}", self.single_file_name);
                process::exit(1);
            }
        }
    }

    fn split(&self) {
        let open = match fs::read(&self.single_file_name) {
            Ok(input) => input,
            Err(_) => {
                eprintln!("打开文件时发生了问题: {}", self.single_file_name);
                process::exit(1);
            }
        };
        let mut save: Vec<Vec<u8>> = Vec::new();
        for _i in 0..self.parts {
            save.push(Vec::new());
        }
        for i in 0..open.len() {
            save[i % self.parts].push(open[i]);
        }
        for i in 0..self.parts {
            match fs::write(&self.file_names[i], &save[i]) {
                Ok(_) => (),
                Err(_) => {
                    println!("写入文件时发生了问题: {}", self.file_names[i]);
                    process::exit(1);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = match Program::new(&args) {
        Ok(res) => res,
        Err(result) => {
            eprintln!("参数解析错误: {}", result);
            Program::help();
            process::exit(1);
        }
    };
    match program.command_type {
        CommandType::Merge => program.merge(),
        CommandType::Split => program.split(),
        CommandType::Help => Program::help(),
    };
}
