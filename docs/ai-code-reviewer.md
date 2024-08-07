# AI 代码评审

即 AI Code Review，使用 AI 大模型对代码质量、安全性进行评审；一些常见的代码管理平台比如GitHub、GitLab、Azure DevOps都有提供代码评审的功能，但是只是提供给用户审查的窗口供用户人工评审（可能也有一些自动评审的工具不过没注意）。

> 关于 Github 开源项目评审案例参考下面文章：
>
> [GitHub 代码评审设置](https://docs.github.com/zh/organizations/organizing-members-into-teams/managing-code-review-settings-for-your-team)
>
> [在 GitHub 上玩转开源项目的 Code Review](https://blog.devstream.io/posts/how-to-code-review-zh)

开发工具一般也可以集成一些第三方 Code Review 插件可以对代码进行自动评审（通常称为静态代码分析工具）。

但是 AI 代码评审相较于传统的静态代码分析工具，有下面优势：

+ 上下文感知性： AI能够更好地理解代码上下文，识别代码逻辑关系，从而提高检测的准确性。
+ 自适应性： 比如AI系统可以学习和适应新的攻击模式和漏洞类型，而无需手动更新规则。
+ 深度学习技术：比如使用深度学习技术的AI可以通过训练模型来理解复杂的代码结构和漏洞模式。

不过 AI在代码审计领域的应用仍然在不断发展中，评审质量取决于许多因素，如训练数据的质量和数量，缺陷模型设计等。

另外现在的 AI 模型还不具备对整个项目源码的逻辑分析能力，代码评审仍旧离不开人工评审，AI 代码评审也只是作为人工评审的辅助。



## AI代码评审方案

现在看有两种方案：

+ **第三方的AI代码评审工具**

  比如：

  + [CodeRabbit](https://docs.coderabbit.ai/)

    作为一个应用部署。

  + [Wasps](https://www.wasps.dev/)

    一款 IDE 插件暂时只支持 VsCode。

  + Jigsaw

  + 一些 AI 编程助手也提供了对代码片段进行代码审查的能力

+ **自行对接 AI 大模型 API 实现**

  代码管理平台对接AI大模型参考：

  + [OpenAI 代码自动评审组件](https://bugstack.cn/md/zsxq/project/openai-code-review.html)
  + [CI+GPT双引擎驱动， 开启AI代码评审新纪元](https://developer.jdcloud.com/article/3826)
  + [ai-code-reviewer](https://github.com/buxuku/ai-code-reviewer)

核心原理一样，就是将合并请求信息推给 AI 大语言模型（API 调用）进行评审，评审完毕后返回并展示评审结果。

### GitHub对接AI大模型实现AI代码评审

> 关于 **[GitHub Actions](https://docs.github.com/en/actions)** （注意官方有中文文档）:
>
> GitHub Actions是一个持续集成和持续交付（CI/CD）平台，可让您自动化构建，测试和部署管道。您可以创建工作流，以构建和测试向存储库的每个拉取请求，或将合并后的拉取请求部署到生产中。
>
> 和 GitLab CI/CD 模块类似。
>
> 官方提供了一些案例 [Examples]() 可以帮助快速理解 Github Actions 工作原理和配置流程。

**工作原理**：

其实和下面这张图中流程类似（有空重新画一张图）。

1. 首先需要有一个Github仓库，然后需要为这个仓库创建一个工作流（在仓库根目录 .github/workflows 中创建工作流配置文件，比如：github-actions-demo.yml）;

   > 工作流文件名随意，每个文件对应一个工作流。

2. 通过工作流配置文件，设置监听合并请求（MR、Github称为PR）；当有合并请求事件触发，会执行对应的处理任务，所以需要在处理任务中执行AI代码评审脚本；

   > Code Review 一般是在合并请求时执行，如果想push时执行就设置监听push请求。
   >
   > 工作流详细配置：[Understanding the workflow file](https://docs.github.com/en/actions/using-workflows/about-workflows#understanding-the-workflow-file)。

   关键配置举例：

   ```yaml
   on: [push]	# 监听的事件类型，这里是监听 push 请求，更多事件参考：https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows
   jobs: 			# 事件处理任务，可以有多个
   	Code-Review-Actions:	# 任务名称, 自行命名
       	runs-on: ...		# 定义要运行作业的计算机类型, Github默认提供了一些免费的标准运行器（参考：https://docs.github.com/zh/actions/using-workflows/workflow-syntax-for-github-actions#%E7%94%A8%E4%BA%8E%E5%85%AC%E5%85%B1%E5%AD%98%E5%82%A8%E5%BA%93%E7%9A%84-github-%E6%89%98%E7%AE%A1%E7%9A%84%E6%A0%87%E5%87%86%E8%BF%90%E8%A1%8C%E5%99%A8），如果无法满足需要还可以购买Github提供的自托管运行器
       	steps:				# 任务包含的步骤，可以有多个步骤
       		-name：		    # 步骤名称    	
       		 run: 			# 执行步骤命令行程序
   ```

   关于工作流任务步骤执行命令 [jobs.<job_id>.steps[*].run](https://docs.github.com/zh/actions/using-workflows/workflow-syntax-for-github-actions#jobsjob_idstepsrun)：

   支持使用操作系统的 shell 运行不超过 21,000 个字符的命令行程序。所以可以使用任何方式（语言）实现我们的 AI 代码审查工具，只需要编译好后将二进制程序加入到某个开源的仓库，Github Actions 构建工作流时可以下载到即可。

3. AI代码评审脚本实现；

   1. 读取合并请求的变更数据；

      首先需要获取当前 PR 的 `pull_request_number`，因为下面的 API 接口需要；使用 AI 编程助手，发现可以通过 [Github 上下文](https://docs.github.com/zh/actions/learn-github-actions/contexts)获取。

      ```yaml
      jobs:
        print-pr-number:
          runs-on: ubuntu-latest
          steps:
          - name: Print PR Number
            run: echo "Pull Request Number: ${{ github.event.pull_request.number }}"
      ```

      然后调用 Github API 获取数据，详细参考 [GitHub REST API documentation](https://docs.github.com/en/rest?apiVersion=2022-11-28)。

      用于读取Github PR 信息的端点：`GET /repos/{owner}/{repo}/pulls/{pull_number}`。

      > Pull Request API 端点：[REST API endpoints for pull requests](https://docs.github.com/en/rest/pulls/pulls?apiVersion=2022-11-28)。

   2. 调用大模型 API 执行代码评审；

      这里选择的是智谱AI，接口调用需要一些鉴权等操作，可以使用官方提供的SDK进行调用，这些SDK封装了接口鉴权等操作；

      官方提供了 Python、Java 的SDK，其他语言应该也有一些开源工作者已经提供，比如 Rust 语言的 [RustGLM](https://github.com/blueokanna/RustGLM)。

   3. 将评审结果输出到期望的目的地; 

      比如 [ai-code-reviewer](https://github.com/buxuku/ai-code-reviewer) 这个脚本将结果输出到了 GitLab`/projects/${this.projectId}/merge_requests/${this.mrId}/discussions` 。

      Github 的话，使用 API `https://api.github.com/repos/kwseeker/rust-template/pulls/{}/reviews` 可以输出到 `Pull Request`-> `Conversation` 标签下。 

      > 关于Github [Projects](https://docs.github.com/en/issues/planning-and-tracking-with-projects/learning-about-projects):
      >
      > Project 是一个适应性强的电子表格、任务板和路线图，它集成了你的问题和GitHub上的pull请求，帮助你有效地计划和跟踪你的工作。

4. 提交工作流配置文件后会自动启动工作流，每次发生配置文件中定义的事件，就会执行一次对应的处理任务；

5. 查看工作流执行结果（在仓库名称下单击 “Actions”），工作流执行完毕后可以到 Pull Requests 下看到 AI 评审内容。	

最终实现效果如下：

![](imgs/ai-code-reviewer.png)



### GitHub 仓库集成 CodeRabbit

CodeRabbit使用GitHub或GitLab webhook集成到代码库中，并监控与Pull Request (PR)和Merge Request (MR)更改相关的事件。在创建PR或MR时，以及针对bot的增量提交和注释时，执行全面的审查。然后将反馈直接发送给PR或MR。
