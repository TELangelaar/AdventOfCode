pub mod loading {
    use anyhow::{anyhow, Context};
    use std::fs;

    #[derive(Debug)]
    struct Config {
        file_path: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Config {
            let file_path = args[0].clone();

            Config { file_path }
        }
    }

    pub fn read_file(args: &Vec<String>) -> anyhow::Result<String> {
        if args.len() < 1 {
            return Err(anyhow!("Problem parsing arguments: not enough arguments"));
        }

        let config = Config::new(&args);

        let result = fs::read_to_string(&config.file_path)
            .with_context(|| format!("Could not read file '{}'", config.file_path))?;

        Ok(result)
    }
}

pub mod parsing {
    use anyhow::Context;

    // each line of the input str should contain only one number
    pub fn lines_to_vec_u32(content: &str) -> anyhow::Result<Vec<u32>> {
        let mut results = Vec::<u32>::new();

        for line in content.lines() {
            results.push(
                line.trim()
                    .parse::<u32>()
                    .with_context(|| format!("Could not parse '{}' into u32", line))?, // We can just use this ? operator here because it will cascade the error back to the caller
            );
        }

        Ok(results)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn one_result() {
            let content = String::from("123\r\n456\r\n789\r\n");

            assert_eq!(vec![123, 456, 789], lines_to_vec_u32(&content).unwrap());
        }
    }
}
