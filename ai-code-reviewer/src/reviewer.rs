use tokio::runtime::Runtime;
use crate::github::Github;
use crate::openai::OpenAI;
use crate::options::Args;
use crate::options::Event::{PullRequest, Push};

/// AI代码审查器
pub(crate) struct Reviewer {
    rt: Runtime,
    /// 读取事件数据
    github: Github,
    /// AI 客户端
    openai: OpenAI,
}

impl Reviewer {
    pub(crate) fn new() -> Self {
        Reviewer {
            rt: Runtime::new().unwrap(),
            github: Github::new(),
            openai: OpenAI::new(),
        }
    }

    pub(crate) fn review(&self, args: Args) -> anyhow::Result<()> {
        match args.event()? {
            PullRequest(pr_number) => self.review_pull_request(pr_number),
            Push(ref_) => {
                //TODO
                Ok(())
            }
        }
    }

    /// 评审 PullRequest 代码修改
    fn review_pull_request(&self, pr_number: &usize) -> anyhow::Result<()> {
        // 1 通过 GithubClient 获取 PullRequest 信息
        let pr_future = self.github.get_pull_request(pr_number);
        let pr = self.rt.block_on(pr_future)?;
        // if pr.is_mergeable() {
            // //2 通过 GithubClient 获取 PullRequest 的 diff 信息
            // let diff = self.github_client.get_pull_request_diff(pr_number)?;
            // //3 通过 GithubClient 获取 PullRequest 的 review 信息
            // let reviews = self.github_client.get_pull_request_reviews(pr_number)?;
            // //4 通过 GithubClient 获取 PullRequest 的 comments 信息
            // let comments = self.github_client.get_pull_request_comments(pr_number)?;

        // }

        // 2 AI 评审

        // 3 在 PullRequest 下追加评论，填写 AI 评审结果

        // 4 异步通知到企业微信

        Ok(())
    }
}