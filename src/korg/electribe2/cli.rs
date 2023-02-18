use crate::electribe2::*;

#[derive(clap::Subcommand)]
pub enum Electribe2CLI {

    /// Manage pattern files
    Patterns {
        /// Import an existing e2sSample.all pattern bundle.
        #[clap(long)]
        import: Option<std::path::PathBuf>,
        /// Add a pattern
        #[clap(long)]
        add:    Vec<std::path::PathBuf>,
        /// Pick a pattern by number and append it to a new pattern bundle.
        #[clap(long)]
        pick:   Option<Vec<usize>>,
    },

    /// Manage sample files
    Samples {
        /// Import an existing e2sSample.all sample bundle.
        #[clap(long)]
        import: Option<String>,
        /// Add a sample
        #[clap(long)]
        add: Vec<std::path::PathBuf>,
    }

}

pub(crate) fn cli (command: &Electribe2CLI) {

    match command {

        Electribe2CLI::Patterns { import, add, pick } => {

            if let Some(import) = import {
                let data = crate::read(import);
                let mut bundle = Electribe2PatternBank::read(&data);
                for (index, pattern) in bundle.patterns.iter().enumerate() {
                    println!("{:>3} {}", index+1, pattern.name);
                }

                if let Some(pick) = pick {
                    let mut new_bundle = Electribe2PatternBank::empty();
                    for index in pick {
                        new_bundle.patterns.push(
                            bundle.patterns.get(*index-1).unwrap().clone()
                        );
                    }
                    println!("");
                    for (index, pattern) in new_bundle.patterns.iter().enumerate() {
                        println!("{:>3} {}", index+1, pattern.name);
                    }
                }
            }

        },

        Electribe2CLI::Samples { import, add } => {
        }

    }

}

impl std::fmt::Display for Electribe2Pattern {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:<30}", self.name)?;
        for part in self.parts.iter() {
            write!(f, "\n  {}", part)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Electribe2Part {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.sample)?;
        for step in self.steps.iter() {
            write!(f, "\n    {}", step)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Electribe2Step {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{} {} {} {} {} {} {} {}]",
            self.empty,
            self.gate,
            self.velocity,
            self.chord,
            self.note_1,
            self.note_2,
            self.note_3,
            self.note_4
        )
    }
}
