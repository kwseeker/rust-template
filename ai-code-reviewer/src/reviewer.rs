use futures::future::join_all;
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
        // 1 通过 GithubClient 获取 PullRequestDiff 信息
        let pr_future = self.github.get_pr_diffs(pr_number);
        let pr = self.rt.block_on(pr_future)?;

        // 2、3
        let mut handles = Vec::new();
        for diff in pr.diffs() {
            // 对每个文件的处理（Code Review、评论追加）都异步进行
            let jh = self.rt.spawn(async {
                // 2 AI 评审，从 PR 中提取 diff 信息
                // self.openai.code_review(diff);

                // 3 在 PullRequest 下追加评论，填写 AI 评审结果
                //   https://docs.github.com/zh/rest/pulls/comments?apiVersion=2022-11-28#create-a-review-comment-for-a-pull-request
            });
            handles.push(jh);
        }
        // 等待所有 handles 完成
        let results = self.rt.block_on(join_all(handles));

        // 4 异步通知到企业微信

        Ok(())
    }
}