use regex_syntax::ast::Ast;

/// 正则表达式抽象语法树的解析结果
#[derive(Clone, Debug)]
pub(crate) struct AstAnalysis {
    /// 是否包含大写字符
    any_uppercase: bool,
    /// 是否包含文字
    any_literal: bool,
}

impl AstAnalysis {

    fn new() -> AstAnalysis {
        AstAnalysis {
            any_uppercase: false,
            any_literal: false
        }
    }

    pub(crate) fn from_ast(ast: &Ast) -> AstAnalysis {
        let mut analysis = AstAnalysis::new();
        analysis.from_ast_impl(ast);
        analysis
    }

    fn from_ast_impl(&mut self, ast: &Ast) {
        if self.done() {
            return;
        }
        // ？？？
        match *ast {
            Ast::Empty(_) => {}
            Ast::Flags(_)
            | Ast::Dot(_)
            | Ast::Assertion(_)
            | Ast::ClassUnicode(_)
            | Ast::ClassPerl(_) => {}
            Ast::Literal(ref x) => {
                self.from_ast_literal(x);
            }
            Ast::ClassBracketed(ref x) => {
                self.from_ast_class_set(&x.kind);
            }
            Ast::Repetition(ref x) => {
                self.from_ast_impl(&x.ast);
            }
            Ast::Group(ref x) => {
                self.from_ast_impl(&x.ast);
            }
            Ast::Alternation(ref alt) => {
                for x in &alt.asts {
                    self.from_ast_impl(x);
                }
            }
            Ast::Concat(ref alt) => {
                for x in &alt.asts {
                    self.from_ast_impl(x);
                }
            }
        }
    }

    fn done(&self) -> bool {
        self.any_uppercase && self.any_literal
    }
}