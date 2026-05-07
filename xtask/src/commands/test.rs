use tracel_xtask::prelude::*;

const CRATES: [&str; 4] = [
    "cubek-phasevocoder",
    "cubek-resample",
    "cubek-sinc-filter",
    "cubek-stft",
];

#[macros::extend_command_args(TestCmdArgs, Target, TestSubCommand)]
pub struct CubeKAudioTestCmdArgs {
    /// Run in CI mode with the portable CPU backend only.
    #[arg(long)]
    pub ci: bool,
}

pub(crate) fn handle_command(
    args: CubeKAudioTestCmdArgs,
    _env: Environment,
    _context: Context,
) -> anyhow::Result<()> {
    let backend = if args.ci {
        "std,test-cpu"
    } else if cfg!(target_os = "macos") {
        "std,test-metal"
    } else {
        "std,test-vulkan"
    };

    for crate_name in CRATES {
        let mut cmd_args = vec![
            "test",
            "-p",
            crate_name,
            "--no-default-features",
            "--features",
            backend,
        ];
        if !args.ci {
            cmd_args.extend(["--", "--test-threads=1"]);
        }
        run_process(
            "cargo",
            &cmd_args,
            None,
            None,
            &format!("{crate_name} tests on backend {backend:?}"),
        )?;
    }

    Ok(())
}
