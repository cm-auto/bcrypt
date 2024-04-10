use clap::{Args, Parser, Subcommand};

enum ExitCode {
    Success = 0,
    // failure means verification of hash failed
    Failure = 1,
    // error means there was an Error, as in there was a Result::Err
    Error = 10,
}

// TODO: find a way to make this look more like the other sections of help (underline and font-weight)
macro_rules! after_help_text {
    () => {
        format!(
            r#"Error Codes:
  Success:             {: >2}
  Verification Failed: {: >2}
  Error:               {: >2}"#,
            ExitCode::Success as i32,
            ExitCode::Failure as i32,
            ExitCode::Error as i32
        )
    };
}

#[derive(Parser)]
#[command(
    about,
    version,
    after_help = after_help_text!()
)]
#[command(args_conflicts_with_subcommands = true)]
struct ParsedArgs {
    // one of these options is Some, while the other is None
    // see method sub_command for more information
    #[clap(subcommand)]
    subcommand: Option<SubCommand>,
    #[command(flatten)]
    hash: Option<HashArgs>,
}

impl ParsedArgs {
    fn sub_command(self) -> SubCommand {
        match self.subcommand {
            // can be safely unwrapped
            // because if there is no subcommand supplied
            // clap will try to parse the args for Hash
            // and if the args were not properly supplied
            // for it, it would error during parsing
            // so basically this is guaranteed to be Some
            // if the other is None
            None => SubCommand::Hash(self.hash.unwrap()),
            Some(sub_command) => sub_command,
        }
    }
}

#[derive(Args)]
struct HashArgs {
    #[clap(short, long, default_value_t = bcrypt::DEFAULT_COST)]
    cost: u32,
    password: String,
}

#[derive(Args)]
struct VerifyArgs {
    password: String,
    hash: String,
}

#[derive(Subcommand)]
enum SubCommand {
    #[command(after_help = after_help_text!())]
    Hash(HashArgs),
    #[command(after_help = after_help_text!())]
    Verify(VerifyArgs),
}

fn exit(exit_code: ExitCode) -> ! {
    std::process::exit(exit_code as i32)
}

fn do_hash(hash_args: HashArgs) -> ! {
    let HashArgs { cost, password } = hash_args;
    match bcrypt::hash(password, cost) {
        Ok(hash) => {
            println!("{}", hash);
            exit(ExitCode::Success);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(ExitCode::Error);
        }
    }
}

fn do_verify(verify_args: VerifyArgs) -> ! {
    let VerifyArgs { password, hash } = verify_args;
    match bcrypt::verify(password, &hash) {
        Ok(verified) => {
            if verified {
                eprintln!("verification succeeded");
                exit(ExitCode::Success);
            } else {
                eprintln!("verification failed");
                exit(ExitCode::Failure);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(ExitCode::Error);
        }
    }
}

fn main() {
    let parsed_args = ParsedArgs::parse();

    match parsed_args.sub_command() {
        SubCommand::Hash(hash_args) => do_hash(hash_args),
        SubCommand::Verify(verify_args) => do_verify(verify_args),
    }
}
