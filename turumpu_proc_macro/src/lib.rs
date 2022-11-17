use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "തുരുമ്പ്" => "turumpu",
        "中" => "std",
        "仓" => "collections",
        "തെറ്റ്" => "Err",
        "ശരി" => "Ok",
        "ചരട്" => "String",
        "典" => "HashMap",
        "തനിനില" => "Default",
        "പിഴവ്" => "Error",
        "或" => "Option",
        "ചില" => "Some",
        "യാതുമില്ല" => "None",
        "കായ്" => "Result",
        "താനെ" => "Self",
        "അച്ചടി" => "println",
        "ഉട" => "break",
        "പലനേര" => "async",
        "കാത്തിരിക്കു" => "await",
        "ചുരുളി" => "loop",
        "അനങ്ങു" => "move",
        "കൂട്" => "crate",
        "禁" => "unreachable_code",
        "作" => "as",
        "常" => "const",
        "性" => "trait",
        "危" => "unsafe",
        "在" => "in",
        "从" => "from",
        "动" => "dyn",
        "解" => "unwrap",
        "准" => "default",
        "作引" => "as_ref",
        "言" => "io",
        "外" => "extern",
        "假" => "false",
        "函" => "fn",
        "超" => "super",
        "入" => "insert",
        "取" => "get",
        "匀" => "allow",
        "警" => "panic",
        "模" => "mod",
        "变" => "mut",
        "新" => "new",
        "于" => "where",
        "令" | "为" => "for",
        "取入" => "get_or_insert_with",
        "主" => "main",
        "公" => "pub",
        "无或" => None?,
        "返" => "return",
        "阐" => "impl",
        "引" => "ref",
        "കിട" => "match",
        "ആണെങ്കിൽ" => "if",
        "അല്ലെങ്കിൽ" => "else",
        "താൻ" => "self",
        "വിടുക" => "let",
        "അനക്കമറ്റ്" => "static",
        "പണിയു" => "struct",
        "കരുതുക" => "expect",
        "അപ്പോൾ" => "while",
        "വഴക്കു" => "use",
        "ഇലേക്ക്" => "into",
        "നേര്" => "true",
        "എണ്ണു" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn 锈(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
