use std::sync::Arc;
use tokio::runtime::Runtime;
use crate::github::{Comment, Github, PullRequestDiffs};
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

        // 2 AI 评审, 由于API有限速，并发请求容易被限，改成顺序执行
        let review_future = self.do_review(pr);
        let comments = self.runtime.block_on(review_future)?;

        // 3 汇总AI评审结果在 PullRequest 的Review 列表下追加评论
        //   https://docs.github.com/zh/rest/pulls/comments?apiVersion=2022-11-28#create-a-review-comment-for-a-pull-request
        let cr_future = self.github.create_review(pr_number, comments);
        let cr_result = self.runtime.block_on(cr_future)?;

        // 4 异步通知到企业微信
        // TODO

        println!("exit review!");
        Ok(())
    }

    async fn do_review(&self, pr: PullRequestDiffs) -> anyhow::Result<Vec<Comment>> {
        let mut comments = Vec::new();
        let diffs = pr.diffs_filtered();
        for diff in diffs {
            let openai = self.openai.clone();
            // AI 评审，先从 PR 中提取 diff 信息，然后调用 AI 模型进行代码评审
            let diff_hunks = diff.code_diffs()?;
            for diff_hunk in diff_hunks {
                println!("diff_hunk: {}", diff_hunk.to_string()?);
                let review_comment = openai.code_review(&diff_hunk).await?;
                // 代码 diff 信息 + AI 评审结果，组合成 Review Comments 信息
                comments.push(Comment::new_with_line(diff_hunk.filename(), review_comment, diff_hunk.last_line()));
            }
        }
        Ok(comments)
    }
}