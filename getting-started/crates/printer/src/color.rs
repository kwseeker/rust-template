use termcolor::{Color, ColorSpec};

/// 颜色定制类型，可以为各种不同数据设置不同的输出颜色
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorSpecs {
    /// 路径信息的颜色，比如ripgrep默认设置为了紫色
    path: ColorSpec,
    /// 匹配行的颜色，默认是白色
    line: ColorSpec,
    ///
    column: ColorSpec,
    /// 匹配行中匹配切片的颜色，默认是红色
    matched: ColorSpec,
}

impl Default for ColorSpecs {
    fn default() -> Self {
        ColorSpecs {
            path: config_color(Color::Magenta, false),
            line: config_color(Color::Green, false),
            column: ColorSpec::default(),
            matched: config_color(Color::Red, true),
        }
    }
}

fn config_color(fg: Color, bold: bool) -> ColorSpec {
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(fg))
        .set_bold(bold);
    color_spec
}

impl ColorSpecs {
    // pub fn new(specs: &[UserColorSpec]) -> ColorSpecs {
    //     let mut merged = ColorSpecs::default();
    //     for spec in specs {
    //         match spec.ty {
    //             OutType::Path => spec.merge_into(&mut merged.path),
    //             OutType::Line => spec.merge_into(&mut merged.line),
    //             OutType::Column => spec.merge_into(&mut merged.column),
    //             OutType::Match => spec.merge_into(&mut merged.matched),
    //         }
    //     }
    //     merged
    // }

    pub fn path(&self) -> &ColorSpec {
        &self.path
    }

    pub fn line(&self) -> &ColorSpec {
        &self.line
    }

    pub fn column(&self) -> &ColorSpec {
        &self.column
    }

    pub fn matched(&self) -> &ColorSpec {
        &self.matched
    }
}

