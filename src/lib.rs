pub mod analysis {

    use regex::Regex;
    use serde::Serialize;
    use std::collections::HashMap;

    pub struct Analysis {
        words: HashMap<String, Stats>,
        special_chars_rgx: Regex,
    }

    #[derive(Default, Serialize, Clone)]
    struct Stats {
        count: u16,
    }

    #[derive(Serialize)]
    pub struct AnalysisSummary {
        words: HashMap<String, Stats>,
        total_words: u16,
        total_uniq_words: usize,
    }

    impl Analysis {
        pub fn new() -> Analysis {
            let pattern = r"[!@#$%^&*()_+=\[\]{};:\\|,.<>?0-9/']";
            Analysis {
                words: HashMap::new(),
                special_chars_rgx: Regex::new(pattern).unwrap(),
            }
        }

        pub fn process_sample(&mut self, sample_text: &str) {
            for line in sample_text.lines() {
                self.special_chars_rgx
                    .replace_all(&line, "")
                    .replace("\"", "")
                    .split_whitespace()
                    .for_each(|word| {
                        let mut word_stat = self.words.entry(word.to_lowercase()).or_default();
                        word_stat.count += 1;
                    });
            }
        }

        pub fn get_summary(&self) -> AnalysisSummary {
            AnalysisSummary {
                words: self.words.clone(),
                total_uniq_words: self.words.len(),
                total_words: self.get_total_words_even_repeated(),
            }
        }

        fn get_total_words_even_repeated(&self) -> u16 {
            let sum = self
                .words
                .values()
                .map(|each| each.count)
                .reduce(|accum, each_count| accum + each_count);
            match sum {
                Some(value) => value,
                None => 0,
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use crate::analysis::Analysis;

        static SAMPLE_TEXT_1: &str =
            "The infinite monkey theorem states that a monkey hitting keys at";

        static SAMPLE_TEXT_2: &str = concat!(
            "The infinite monkey theorem states that a monkey hitting keys at ",
            "random on a typewriter (keyboard) for an infinite amount of time will ",
            "almost surely type any given text, such as the complete works of ",
            "William Shakespeare. In fact, the monkey would almost surely type ",
            "every possible finite text an infinite number of times.[...]"
        );

        #[test]
        fn should_create_report_with_zero_words_len() {
            let mut analysis = Analysis::new();
            analysis.process_sample("");
            assert_eq!(analysis.words.len(), 0);
        }

        #[test]
        fn should_create_report_with_ten_words_len() {
            let mut analysis = Analysis::new();
            analysis.process_sample(&SAMPLE_TEXT_1);
            assert_eq!(analysis.words.len(), 10);
        }

        #[test]
        fn should_create_report_with_forty_words_len() {
            let mut analysis = Analysis::new();
            analysis.process_sample(&SAMPLE_TEXT_2);
            assert_eq!(analysis.words.len(), 40);
        }

        #[test]
        fn should_create_summary_with_forty_words_len() {
            let mut analysis = Analysis::new();
            analysis.process_sample(&SAMPLE_TEXT_2);
            assert_eq!(analysis.words.len(), 40);

            let summary = analysis.get_summary();
            assert_eq!(summary.total_words, 54);
            assert_eq!(summary.total_uniq_words, 40);
        }
    }
}
