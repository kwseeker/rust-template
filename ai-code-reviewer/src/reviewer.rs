use std::sync::Arc;
use anyhow::Error;
use futures::future::join_all;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use crate::github::Github;
use crate::openai::OpenAI;
use crate::options::Args;
use crate::options::Event::{PullRequest, Push};

/// AI代码审查器
pub(crate) struct Reviewer {
    /// runtime 需要被多个所有者共享，另外值还需要在多个线程中安全共享，所以使用 Arc
    // runtime: &'a Runtime,    //由于Runtime实例是在new()中创建的，这种写法有生命周期问题
    runtime: Arc<Runtime>,
    /// 读取事件数据
    github: Github,
    /// AI 客户端
    openai: OpenAI,
}

impl Reviewer {
    pub(crate) fn new() -> Self {
        // 方法内创建的局部变量，如果要在方法外使用，要么转移所有权，要么返回引用，
        // 如果还要在多个地方（不同生命周期）使用，有下面实现方式：
        // 所有权的实现方式：克隆（新值有独立的所有权）、Rc智能指针（多个所有者共享所有权，某一时刻只有一个所有者），
        // 引用的方式：返回引用可以在多个地方访问(但是有生命周期的限制)，如果涉及修改一般还需要使用 RefCell（实现编译期可变、不可变引用共存, 如果涉及多个地方还需要搭配 Rc\Arc）
        let runtime_rc = Arc::new(Runtime::new().unwrap());  //TODO 这里直接unwrap() Err默认会被怎么处理？
        let ai = OpenAI::new();
        Reviewer {
            runtime: runtime_rc.clone(),
            github: Github::new(),
            openai: ai,
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
        let pr = self.runtime.block_on(pr_future)?;

        // 2、3
        let mut handles = Vec::new();
        let diffs = pr.diffs_filtered();
        for diff in diffs {
            // 对每个文件的处理（Code Review、评论追加）都异步进行
            let openai = self.openai.clone();
            let jh: JoinHandle<Result<(), Error>> = self.runtime.spawn(async move {
                // 2 AI 评审，先从 PR 中提取 diff 信息，然后调用 AI 模型进行代码评审
                let code_diffs = diff.code_diffs()?;
                for code_diff in code_diffs {
                    println!("code_diff: {code_diff}");
                    let response = openai.code_review(code_diff).await;
                }

                // 3 在 PullRequest 下追加评论，填写 AI 评审结果
                //   https://docs.github.com/zh/rest/pulls/comments?apiVersion=2022-11-28#create-a-review-comment-for-a-pull-request

                Ok(())
            });
            handles.push(jh);
        }
        // 等待所有 handles 完成
        let results = self.runtime.block_on(join_all(handles));

        // 4 异步通知到企业微信

        println!("exit review!");
        Ok(())
    }
}