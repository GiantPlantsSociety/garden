extern crate assert_cli;

#[cfg(test)]
mod cli {
    use assert_cli;
    use std::path::Path;

    #[test]
    fn calling_without_args() {
        assert_cli::Assert::main_binary()
            .fails_with(1)
            .stderr().contains("USAGE:")
            .stderr().contains("--help")
            .unwrap();
    }

    #[test]
    fn calling_help() {
        assert_cli::Assert::main_binary()
            .with_args(&["help"])
            .stdout().contains("USAGE:")
            .stdout().contains("--help")
            .unwrap();
    }

    #[test]
    fn calling_init() {
        assert_cli::Assert::main_binary()
            .with_args(&["init"])
            .stdout().contains("Nothing to do: 'garden.toml' file is already present.")
            .unwrap();
    }

    #[test]
    fn calling_info_mnist() {
        assert_cli::Assert::main_binary()
            .with_args(&["info", "mnist"])
            .stdout().contains("The MNIST database of handwritten digits.")
            .unwrap();
    }

    #[test]
    fn calling_add_trump_tweets() {
        assert_cli::Assert::main_binary()
            .with_args(&["add", "trump_tweets"])
            .stdout().contains("Downloading")
            .stdout().contains(
                Path::new("garden_data")
                    .join("trump_tweets")
                    .join("realDonaldTrump_poll_tweets.csv")
                .to_str()
                .unwrap()
            )
            .unwrap();
    }

    #[test]
    fn calling_add_trump_tweets_with_version() {
        assert_cli::Assert::main_binary()
            .with_args(&["add", "trump_tweets@0.1"])
            .stdout().contains("Downloading")
            .unwrap();
    }

    #[test]
    fn calling_add_trump_tweets_as_local_file() {
        assert_cli::Assert::main_binary()
            .with_args(&["add", "src/svalbard/greenhouse/trump_tweets.toml"])
            .stdout().contains("Downloading")
            .unwrap();
    }
}
