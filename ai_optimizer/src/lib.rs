#![forbid(unsafe_code)]

/// Deterministic optimizer (stable v1.x).
///
/// In v1.x this crate provides a predictable optimization interface.
/// ML-driven optimization can be introduced later behind a feature flag
/// or as a major-version upgrade.

/// A deterministic optimization pass over a textual quantum circuit.
///
/// The pass receives the circuit as a slice of lines and returns a new,
/// optimized set of lines. This keeps the implementation simple and stable
/// for v1.x while allowing future expansion into richer IR-based passes.
pub trait OptimizationPass {
    /// Human-readable name for logging/debugging.
    fn name(&self) -> &'static str;

    /// Apply the optimization pass to the circuit lines.
    fn run(&self, lines: Vec<String>) -> Vec<String>;
}

/// Removes empty lines and trims surrounding whitespace.
/// This is deterministic and safe, and helps normalize output.
pub struct NormalizeWhitespacePass;

impl OptimizationPass for NormalizeWhitespacePass {
    fn name(&self) -> &'static str {
        "normalize_whitespace"
    }

    fn run(&self, lines: Vec<String>) -> Vec<String> {
        lines
            .into_iter()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect()
    }
}

/// Removes consecutive duplicate instructions.
///
/// Example:
/// X 0
/// X 0
///
/// becomes:
/// X 0
///
/// This is intentionally conservative for v1.x:
/// - only exact consecutive duplicates are removed
/// - behavior is fully deterministic
pub struct RemoveConsecutiveDuplicatePass;

impl OptimizationPass for RemoveConsecutiveDuplicatePass {
    fn name(&self) -> &'static str {
        "remove_consecutive_duplicate"
    }

    fn run(&self, lines: Vec<String>) -> Vec<String> {
        let mut optimized = Vec::with_capacity(lines.len());

        for line in lines {
            let should_skip = optimized
                .last()
                .map(|previous| previous == &line)
                .unwrap_or(false);

            if !should_skip {
                optimized.push(line);
            }
        }

        optimized
    }
}

/// A deterministic optimizer pipeline.
///
/// v1.x keeps the optimizer simple, explicit, and reproducible:
/// - pass ordering is fixed
/// - no randomness
/// - no ML/AI heuristics
/// - same input always yields the same output
pub struct Optimizer {
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self {
            passes: vec![
                Box::new(NormalizeWhitespacePass),
                Box::new(RemoveConsecutiveDuplicatePass),
            ],
        }
    }
}

impl Optimizer {
    /// Create an optimizer with the default deterministic passes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an optimizer from a custom deterministic pass pipeline.
    pub fn with_passes(passes: Vec<Box<dyn OptimizationPass>>) -> Self {
        Self { passes }
    }

    /// Run all configured deterministic optimization passes.
    pub fn optimize(&self, circuit_text: &str) -> String {
        println!("Running deterministic optimization pipeline...");

        let mut lines: Vec<String> = circuit_text.lines().map(str::to_string).collect();

        for pass in &self.passes {
            println!("Applying optimization pass: {}", pass.name());
            lines = pass.run(lines);
        }

        if lines.is_empty() {
            String::new()
        } else {
            format!("{}\n", lines.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_whitespace_removes_empty_lines_and_trims() {
        let input = "  H 0  \n\n   \nX 1   \n";
        let optimizer = Optimizer::with_passes(vec![Box::new(NormalizeWhitespacePass)]);

        let output = optimizer.optimize(input);

        assert_eq!(output, "H 0\nX 1\n");
    }

    #[test]
    fn remove_consecutive_duplicate_pass_removes_adjacent_duplicates() {
        let input = "H 0\nH 0\nX 1\nX 1\nMEASURE 0\n";
        let optimizer = Optimizer::with_passes(vec![Box::new(RemoveConsecutiveDuplicatePass)]);

        let output = optimizer.optimize(input);

        assert_eq!(output, "H 0\nX 1\nMEASURE 0\n");
    }

    #[test]
    fn default_optimizer_applies_both_passes() {
        let input = "  H 0  \n\nH 0\nX 1\nX 1\n";
        let optimizer = Optimizer::new();

        let output = optimizer.optimize(input);

        assert_eq!(output, "H 0\nX 1\n");
    }
}
