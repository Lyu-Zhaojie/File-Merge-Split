use std::env;
use std::fs;
use std::process;
use std::str::FromStr;

enum CommandType {
    Merge {
        source_paths: Vec<String>,
        target_path: String,
    },
    Split {
        source_path: String,
        n_parts: usize,
    },
    Help,
}

const HELP_INFO: &'static str = "
文件分块/合并器

--help
 * 显示本帮助.

--merge <input file 1> <input file 2> ... <output file>
 * 将 input file 1, input file 2, ... 合并输出至 output file.

--split <parts> <input file>
 * 将 input file 分为 parts 份存储.
";

fn help() {
    println!("{}", (HELP_INFO));
}

fn parse_args() -> Result<CommandType, String> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        return Err("参数数目过少.".to_string());
    }
    match &args[1] as &str {
        "--merge" => {
            if args.len() < 4 {
                return Err("参数数目过少.".to_string());
            }
            let source_paths = args[2..args.len() - 1].to_vec();
            let target_path = args.last().unwrap().to_string();
            Ok(CommandType::Merge {
                source_paths,
                target_path,
            })
        }
        "--split" => {
            if args.len() != 4 {
                return Err("参数数目错误. 应输入 3 个参数.".to_string());
            }
            let n_parts = match usize::from_str(&args[2]) {
                Ok(val) if val > 0 => val,
                _ => return Err(format!("参数不是正整数: {}.", args[2])),
            };
            let source_path = args.last().unwrap().to_string();
            Ok(CommandType::Split {
                source_path,
                n_parts,
            })
        }
        "--help" => Ok(CommandType::Help),
        other => Err(format!("未知参数: {}.", other)),
    }
}

fn split(source_path: &String, n_parts: usize) {
    let open = fs::read(source_path).expect(&format!("打开文件时发生了问题: {}", source_path));
    let mut save: Vec<Vec<u8>> = Vec::with_capacity(n_parts);
    for _i in 0..n_parts {
        save.push(Vec::with_capacity(open.len() / n_parts + 1));
    }
    for i in 0..open.len() {
        save[i % n_parts].push(open[i]);
    }
    for i in 0..n_parts {
        fs::write(format!("{}{}", source_path, i + 1), &save[i]).expect(&format!(
            "写入文件时发生了问题: {}",
            format!("{}{}", source_path, i + 1)
        ))
    }
}

fn merge(source_paths: &[String], target_path: &String) {
    let mut save: Vec<Vec<u8>> = Vec::new();
    for p in source_paths {
        save.push(fs::read(p).expect(&format!("打开文件时发生了问题: {}", p)));
    }
    let min_len = save.last().unwrap().len();
    let mut write: Vec<u8> = Vec::with_capacity(min_len + 1);
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
    fs::write(target_path, write).expect(&format!("写入文件时发生了问题: {}", target_path))
}

fn main() {
    match parse_args() {
        Ok(command_detail) => match command_detail {
            CommandType::Help => help(),
            CommandType::Merge {
                source_paths,
                target_path,
            } => merge(&source_paths, &target_path),
            CommandType::Split {
                source_path,
                n_parts,
            } => split(&source_path, n_parts),
        },
        Err(why) => {
            eprintln!("参数解析错误: {}", why);
            help();
            process::exit(1)
        }
    }
}
