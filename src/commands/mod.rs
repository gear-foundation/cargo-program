mod build;
mod login;
mod new;
mod publish;
mod run;
mod test;

pub(crate) use build::BuildCommand;
pub(crate) use login::LoginCommand;
pub(crate) use new::NewCommand;
pub(crate) use publish::PublishCommand;
pub(crate) use run::RunCommand;
pub(crate) use test::TestCommand;

impl From<RunCommand> for BuildCommand {
    fn from(cmd: RunCommand) -> Self {
        Self {
            release: cmd.release,
            manifest_path: cmd.manifest_path,
        }
    }
}
