use std::env;
use std::ffi::OsString;
use std::fmt::Debug;
use anyhow::{bail, Context};
use lexopt::{Arg, Parser, ValueExt};

/// 命令行参数
#[derive(Debug)]
pub(crate) struct Args {
    /// 事件类型
    /// OPTIONS:
    ///     --pr-number=[PR_NUMBER]     用于 pull_request 事件
    ///     --ref=[REF]                 用于 push 事件
    event: Option<Event>,
}

impl Args {
    fn new() -> Self {
        Args {
            event: None,
        }
    }

    pub(crate) fn is_event_none(&self) -> bool {
        self.event.is_none()
    }

    pub(crate) fn event(&self) -> anyhow::Result<&Event> {
        return match &self.event {
            None => bail!("Invalid event in Args"),
            Some(event) => {
                Ok(event)
            }
        }
    }
}

pub(crate) fn parse_env_args() -> anyhow::Result<Args> {
    let argv: Vec<OsString> = env::args_os().skip(1).collect();
    parse(argv)
}

fn parse(argv: Vec<OsString>) -> anyhow::Result<Args> {
    let mut args = Args::new();
    // 将命令行参数解析为 Args
    let mut parser = Parser::from_args(argv);
    while let Some(arg) = parser.next().context("Invalid option")? {
        match arg {
            Arg::Long("pr-number") => {
                let value = parser.value().context("Invalid value for --pr-number")?;
                let pr_number = value.parse::<usize>().context("Invalid value (unparseable) for --pr-number")?;
                args.event = Some(Event::PullRequest(pr_number));
            },
            Arg::Long("ref") => {
                let value = parser.value().context("Invalid value for --ref")?;
                let ref_value = value.string().context("Invalid value (unparseable) for --pr-number")?;
                args.event = Some(Event::Push(ref_value));
            },
            _ => bail!("unknown option"),
        }
    }
    Ok(args)
}

#[derive(Debug, PartialEq)]
pub(crate) enum Event {
    PullRequest(usize),
    Push(String)
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use crate::options;

    #[test]
    fn parse() {
        let argv = vec![OsString::from("--pr-number=1")];
        let args = options::parse(argv);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().event, Some(options::Event::PullRequest(1)));

        let argv = vec![OsString::from("--ref=refs/heads/master")];
        let args = options::parse(argv);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().event, Some(options::Event::Push(String::from("refs/heads/master"))));
    }
}