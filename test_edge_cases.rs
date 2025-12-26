use get_harness::types::{EnvValue, HarnessKind};

fn main() {
    // Test edge cases
    let cases = vec![
        "${}",
        "${VAR with spaces}",
        "${VAR${NESTED}}",
        "${VAR",
        "$VAR}",
        "${VAR}",
        "plain text",
    ];
    
    for case in cases {
        let result = EnvValue::from_native(case, HarnessKind::ClaudeCode);
        println!("'{}' -> {:?}", case, result);
    }
}
