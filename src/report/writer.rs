use crate::analysis::engine::AnalysisResult;

pub fn print_text(result: &AnalysisResult) {
    println!("–––––––Retry Safety Report–––––––");

    println!("Safety: {:?}", result.classification.safety);

    if result.classification.reasons.is_empty() {
        println!("No persistent state changes detected..");
    } else {
        println!("Reasons: ");
        for r in &result.classification.reasons {
            println!("- {}", r);
        }
    }
}