use pulldown_cmark::{CodeBlockKind, Event as MdEvent, Parser, Tag, TagEnd};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};
use std::sync::OnceLock;
use syntect::{
    easy::HighlightLines,
    highlighting::{Theme, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};

static SYNTAXES: OnceLock<SyntaxSet> = OnceLock::new();
static THEMES: OnceLock<ThemeSet> = OnceLock::new();

fn syntax_set() -> &'static SyntaxSet {
    SYNTAXES.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn theme() -> &'static Theme {
    THEMES
        .get_or_init(ThemeSet::load_defaults)
        .themes
        .get("base16-ocean.dark")
        .expect("Default syntect theme not found")
}

fn find_language<'a>(language: &str, syntaxes: &'a SyntaxSet) -> &'a SyntaxReference {
    let normalized = language.trim().to_lowercase();
    let alias = match normalized.as_str() {
        "js" | "jsx" | "mjs" => "javascript",
        "ts" | "tsx" => "typescript",
        "rs" => "rust",
        "py" => "python",
        "sh" | "bash" | "zsh" => "bash",
        "yml" => "yaml",
        "kt" => "kotlin",
        "golang" => "go",
        other => other,
    };

    syntaxes
        .find_syntax_by_token(alias)
        .unwrap_or_else(|| syntaxes.find_syntax_plain_text())
}

fn syntax_spans(
    source: &str,
    language: &str,
    syntaxes: &SyntaxSet,
    theme: &Theme,
) -> Vec<Line<'static>> {
    let syntax = find_language(language, syntaxes);
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut output = Vec::new();

    for source_line in LinesWithEndings::from(source) {
        let spans: Vec<Span<'static>> = match highlighter.highlight_line(source_line, syntaxes) {
            Ok(regions) => regions
                .into_iter()
                .map(|(style, content)| {
                    let foreground =
                        Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                    let background = Color::Rgb(30, 35, 45);
                    Span::styled(
                        content.trim_end_matches(['\n', '\r']).to_string(),
                        Style::default().fg(foreground).bg(background),
                    )
                })
                .collect(),
            Err(_) => vec![Span::raw(
                source_line.trim_end_matches(['\n', '\r']).to_string(),
            )],
        };
        output.push(Line::from(spans));
    }

    if output.is_empty() {
        output.push(Line::default());
    }

    output
}

pub fn render_markdown(input: &str) -> Text<'static> {
    let syntaxes = syntax_set();
    let theme = theme();
    let mut lines: Vec<Line<'static>> = Vec::new();

    let mut in_code = false;
    let mut code = String::new();
    let mut language = String::new();
    let mut heading = false;
    let mut strong = false;
    let mut emphasis = false;
    let mut list_depth = 0usize;
    let mut item_start = false;

    for event in Parser::new(input) {
        match event {
            MdEvent::Start(Tag::CodeBlock(kind)) => {
                in_code = true;
                code.clear();
                language = match kind {
                    CodeBlockKind::Fenced(info) => {
                        info.split_whitespace().next().unwrap_or("text").to_string()
                    }
                    CodeBlockKind::Indented => "text".to_string(),
                };

                if !lines.is_empty() {
                    lines.push(Line::default());
                }

                lines.push(Line::from(vec![
                    Span::styled("┌─ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!(" {} ", language),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "────────────────────────",
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));
            }
            MdEvent::Text(text) if in_code => {
                code.push_str(&text);
            }
            MdEvent::End(TagEnd::CodeBlock) if in_code => {
                lines.extend(syntax_spans(&code, &language, syntaxes, theme));
                lines.push(Line::from(Span::styled(
                    "└────────────────────────────────",
                    Style::default().fg(Color::DarkGray),
                )));
                in_code = false;
            }
            MdEvent::Start(Tag::Heading { .. }) => {
                heading = true;
                lines.push(Line::default());
            }
            MdEvent::End(TagEnd::Heading(_)) => {
                heading = false;
                lines.push(Line::default());
            }
            MdEvent::Start(Tag::Strong) => strong = true,
            MdEvent::End(TagEnd::Strong) => strong = false,
            MdEvent::Start(Tag::Emphasis) => emphasis = true,
            MdEvent::End(TagEnd::Emphasis) => emphasis = false,
            MdEvent::Start(Tag::List(_)) => {
                list_depth = list_depth.saturating_add(1);
            }
            MdEvent::End(TagEnd::List(_)) => {
                list_depth = list_depth.saturating_sub(1);
            }
            MdEvent::Start(Tag::Item) => {
                item_start = true;
            }
            MdEvent::End(TagEnd::Item) => {
                item_start = false;
                lines.push(Line::default());
            }
            MdEvent::Text(text) => {
                let style = if heading {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else if strong {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else if emphasis {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::ITALIC)
                } else {
                    Style::default().fg(Color::Gray)
                };

                let prefix = if item_start {
                    item_start = false;
                    if list_depth > 0 {
                        format!("{}• ", "  ".repeat(list_depth - 1))
                    } else {
                        "• ".to_string()
                    }
                } else {
                    String::new()
                };

                if !prefix.is_empty() {
                    lines.push(Line::from(vec![
                        Span::styled(prefix, Style::default().fg(Color::Cyan)),
                        Span::styled(text.to_string(), style),
                    ]));
                } else if let Some(last) = lines.last_mut() {
                    last.spans.push(Span::styled(text.to_string(), style));
                } else {
                    lines.push(Line::from(Span::styled(text.to_string(), style)));
                }
            }
            MdEvent::Code(text) => {
                let inline_style = Style::default()
                    .fg(Color::LightCyan)
                    .bg(Color::Rgb(30, 35, 45));
                let span = Span::styled(format!(" {} ", text), inline_style);
                if let Some(last) = lines.last_mut() {
                    last.spans.push(span);
                } else {
                    lines.push(Line::from(span));
                }
            }
            MdEvent::SoftBreak | MdEvent::HardBreak => {
                lines.push(Line::default());
            }
            MdEvent::End(TagEnd::Paragraph) => {
                lines.push(Line::default());
            }
            _ => {}
        }
    }

    Text::from(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_language_mapping() {
        let syntaxes = syntax_set();
        let lang_js = find_language("js", syntaxes);
        assert_eq!(lang_js.name.to_lowercase(), "javascript");

        let lang_rs = find_language("rs", syntaxes);
        assert_eq!(lang_rs.name.to_lowercase(), "rust");

        let lang_unknown = find_language("unknown_lang_xyz", syntaxes);
        assert!(!lang_unknown.name.is_empty());
    }

    #[test]
    fn test_render_markdown_basic() {
        let md = "# Heading\n\nSome **bold** text and `inline code`.\n\n```rust\nfn main() {}\n```";
        let rendered = render_markdown(md);
        assert!(!rendered.lines.is_empty());
    }
}
