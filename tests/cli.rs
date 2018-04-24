extern crate assert_cli;

#[cfg(test)]
mod cli {
    use assert_cli;

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
            .stdout().contains("garden_data/trump_tweets/realDonaldTrump_poll_tweets.csv")
            .unwrap();
    }
}
