mod commands;

use tracel_xtask::prelude::*;

#[macros::base_commands(
    Bump,
    Compile,
    Coverage,
    Doc,
    Dependencies,
    Fix,
    Publish,
    Validate,
    Vulnerabilities
)]
pub enum Command {
    /// Check CubeK Audio workspace code quality.
    Check(CheckCmdArgs),
    /// Test CubeK Audio crates on selected backends.
    Test(commands::test::CubeKAudioTestCmdArgs),
}

fn main() -> anyhow::Result<()> {
    let (args, env) = init_xtask::<Command>(parse_args::<Command>()?)?;
    match args.command {
        Command::Test(cmd_args) => commands::test::handle_command(cmd_args, env, args.context),
        Command::Check(cmd_args) => {
            base_commands::check::handle_command(cmd_args, env, args.context)
        }
        _ => dispatch_base_commands(args, env),
    }?;
    Ok(())
}
