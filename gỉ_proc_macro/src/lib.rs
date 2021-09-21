use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "InLỗi" => "Err",
        "Được" => "Ok",
        "Xâu" => "String",
        "ÁnhXạBăm" => "HashMap",
        "MặcĐịnh" => "Default",
        "Lỗi" => "Error",
        "Tùy" | "Tuỳ" => "Option",
        "Vài" | "MộtVài" => "Some",
        "KhôngCóGì" | "Trống" | "Rỗng" => "None",
        "KếtQuả" => "Result",
        "BảnThân" => "Self",
        "indòng" => "println",
        "phá" => "break",
        "bất_đồng_bộ" => "async",
        "chờ" => "await",
        "lặp" => "loop",
        "di_chuyển" => "move",
        "thùng" => "crate",
        "mã_không_được_chạy" => "unreachable_code",
        "như" => "as",
        "hằng" | "hằng_số" => "const",
        "tính_chất" | "tính" => "trait",
        "nguy_hiểm" | "không_an_toàn" => "unsafe",
        "trong" => "in",
        "từ" => "from",
        "động" => "dyn",
        "tháo_bọc" => "unwrap",
        "mặc_định" => "default",
        "bằng_tham_chiếu" => "as_ref",
        "nx" => "io",
        "ngoại" => "extern",
        "sai" => "false",
        "hàm" => "fn",
        "siêu" => "super",
        "chèn" => "insert",
        "đọc" => "get",
        "cho_phép" => "allow",
        "đù" | "trời_đất" | "ối" => "panic",
        "mô_đun" => "mod",
        "khả_biến" => "mut",
        "taọ_mới" | "mới" => "new",
        "tại" => "where",
        "với" => "for",
        "đọc_hoặc_chèn_với" => "get_or_insert_with",
        "chính" => "main",
        "công_khai" => "pub",
        "không_có" => None?,
        "trả_về" => "return",
        "triển_khai" => "impl",
        "tham_chiếu" => "ref",
        "dựa_vào" => "match",
        "nếu" => "if",
        "không_thì" => "else",
        "bản_thân" => "self",
        "đặt" => "let",
        "tĩnh" => "static",
        "cấu_trúc" => "struct",
        "mong" => "expect",
        "trong_khi" | "khi" => "while",
        "dùng" | "sử_dụng" => "use",
        "vào" => "into",
        "đúng" => "true",
        "liệt_kê" => "enum",

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
pub fn gỉ(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
