use predicates::prelude::PredicateBooleanExt;

#[test]
fn completions_command_outputs_shell_completions() {
    let shells = ["bash", "elvish", "fish", "nushell", "powershell", "zsh"];

    for shell in shells {
        assert_cmd::Command::cargo_bin("opad")
            .expect("it not to raise")
            .arg("--completions")
            .arg(shell)
            .assert()
            .success()
            .stdout(predicates::str::is_empty().not());
    }
}
