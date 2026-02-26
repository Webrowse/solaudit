use crate::analysis::engine::AnalysisResult;

pub fn print_text(result: &AnalysisResult) {
    println!("–––––––Retry Safety Report–––––––");

    println!("Account: {}", result.before.pubkey);

    println!("Safety: {:?}", result.classification.safety);

    if result.classification.reasons.is_empty() {
        println!("No state changes detected");
        return;
    }
    
    println!("\nState Changes:");

    // Lamport changed
    if result.before.lamports != result.after.lamports {
        println!(
            "- Lamports: {} -> {}",
            result.before.lamports, result.after.lamports
        );

    }

    // Owner 
    if result.before.owner != result.after.owner {
        println!(
            "Owner: {} -> {}",
            result.before.owner, result.after.owner
        );

    }

    // Executable
        if result.before.executable != result.after.executable {
        println!(
            "- Executable: {} -> {}",
            result.before.executable,
            result.after.executable
        );
    }

    // Data Size
    if result.before.data_len != result.after.data_len {
        println!(
            "Data Size: {} -> {}",
            result.before.data_len, result.after.data_len
        );

    }


    println!("\nReasons: ");
    for r in &result.classification.reasons {
        println!("- {}", r);
    }

    if !result.simulation_logs.is_empty() {
        println!("\nSimulation Logs:");
        for log in &result.simulation_logs {
            println!("  {}", log);
        }
    }

}

pub fn print_json(result: &AnalysisResult) {
    println!(
        "{}",
        serde_json::to_string_pretty(result).unwrap()
    );
}