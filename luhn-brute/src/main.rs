use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
#[command(about = "Generate list of values with valid Luhn checksum", arg_required_else_help = true, long_about = None)]
pub struct Args {
    /// Provide a list of values that will be expanded into a list of values, Example: --values 411111,400000 N 0,1,3,8,9 11111111
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub values: Vec<String>,

    /// Provided values do not contain checksum, generate Luhn checksums for the provided values. Defaults to false
    #[arg(short, long, default_value_t = false)]
    generate_luhn: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let explodable_values = args
        .values
        .clone()
        .into_iter()
        .map(|value| {
            if value == "N" {
                vec![
                    "0".to_string(),
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                    "5".to_string(),
                    "6".to_string(),
                    "7".to_string(),
                    "8".to_string(),
                    "9".to_string(),
                ]
            } else {
                value.split(",").map(|value| value.to_string()).collect()
            }
        })
        .collect();
    let mut values = explode_arrays(explodable_values);

    if args.generate_luhn {
        add_luhn_checksums(&mut values);
    }

    let valid_luhns = get_valid_luhns(values.clone());
    for i in valid_luhns.iter() {
        println!("{}", i);
    }

    if valid_luhns.len() == 0 {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

pub fn explode_arrays(list: Vec<Vec<String>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in list.iter() {
        let expand_list = result.clone();
        if expand_list.len() > 0 {
            result.clear();
            for j in expand_list.iter() {
                for k in i.iter() {
                    let mut pan: String = j.to_string();
                    pan.push_str(k);
                    result.push(pan);
                }
            }
        } else {
            for j in i.iter() {
                result.push(j.to_string());
            }
        }
    }

    result
}

pub fn get_valid_luhns(list: Vec<String>) -> Vec<String> {
    list.into_iter().partition(|value| luhn::valid(value)).0
}

pub fn add_luhn_checksums(list: &mut Vec<String>) {
    for i in list.iter_mut() {
        let mut value: String = i.to_string();
        value.push(luhn::checksum(value.as_bytes()) as char);

        *i = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_array_exploding() {
        assert_eq!(luhn::valid("4111111111111111"), true);

        let mut list: Vec<Vec<String>> = Vec::new();
        list.push(vec!["411111".to_string()]);
        list.push(vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
        ]);
        list.push(vec![
            "0".to_string(),
            "1".to_string(),
            "3".to_string(),
            "8".to_string(),
            "9".to_string(),
        ]);
        list.push(vec!["11111111".to_string()]);

        let result = explode_arrays(list);
        assert_eq!(result.len(), 50);

        for i in result.iter() {
            assert_eq!(i.len(), 16);
        }

        let valid_luhns = get_valid_luhns(result);
        assert_eq!(valid_luhns.len(), 5);
    }

    #[test]
    fn test_luhn_checksum_generation() {
        let mut list: Vec<String> = Vec::new();
        list.push("411111111111111".to_string());
        list.push("411111001111111".to_string());

        assert_eq!(get_valid_luhns(list.clone()).len(), 0);

        add_luhn_checksums(&mut list);
        assert_eq!(get_valid_luhns(list.clone()).len(), 2);

        assert_eq!(list[0], "4111111111111111");
        assert_eq!(list[1], "4111110011111114");
    }
}
