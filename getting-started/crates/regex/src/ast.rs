use regex_syntax::ast;
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

    pub(crate) fn any_uppercase(&self) -> bool {
        self.any_uppercase
    }

    pub(crate) fn any_literal(&self) -> bool {
        self.any_literal
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

    fn from_ast_literal(&mut self, ast: &ast::Literal) {
        self.any_literal = true;
        self.any_uppercase = self.any_uppercase || ast.c.is_uppercase();
    }

    fn from_ast_class_set(&mut self, ast: &ast::ClassSet) {
        if self.done() {
            return;
        }
        match *ast {
            ast::ClassSet::Item(ref item) => {
                self.from_ast_class_set_item(item);
            }
            ast::ClassSet::BinaryOp(ref x) => {
                self.from_ast_class_set(&x.lhs);
                self.from_ast_class_set(&x.rhs);
            }
        }
    }

    fn from_ast_class_set_item(&mut self, ast: &ast::ClassSetItem) {
        if self.done() {
            return;
        }
        match *ast {
            ast::ClassSetItem::Empty(_)
            | ast::ClassSetItem::Ascii(_)
            | ast::ClassSetItem::Unicode(_)
            | ast::ClassSetItem::Perl(_) => {}
            ast::ClassSetItem::Literal(ref x) => {
                self.from_ast_literal(x);
            }
            ast::ClassSetItem::Range(ref x) => {
                self.from_ast_literal(&x.start);
                self.from_ast_literal(&x.end);
            }
            ast::ClassSetItem::Bracketed(ref x) => {
                self.from_ast_class_set(&x.kind);
            }
            ast::ClassSetItem::Union(ref union) => {
                for x in &union.items {
                    self.from_ast_class_set_item(x);
                }
            }
        }
    }

    fn done(&self) -> bool {
        self.any_uppercase && self.any_literal
    }
}