use crate::parse::{self, Ptr};
use std::fmt::Write;

pub(crate) fn transform(input: &str) -> String {
    let sig = parse::parse(input);
    let args = transform_args(sig.args);
    let ret = transform_ret(sig.ret_type);
    format!("pub fn {}({}){}", ident(sig.name), args, ret)
}

fn transform_ret(type_: parse::Type) -> String {
    if type_.pointer == Ptr::No && type_.ident == "void" {
        String::new()
    } else {
        format!(" -> {}", transform_type(type_))
    }
}

fn transform_args(args: Vec<parse::Arg>) -> String {
    let mut out = String::new();
    for arg in args {
        write!(&mut out, "{}, ", transform_arg(arg)).unwrap();
    }
    // Remove last `, `
    out.pop();
    out.pop();
    out
}

fn transform_arg(arg: parse::Arg) -> String {
    format!("{}: {}", ident(arg.name), transform_type(arg.type_))
}

fn transform_type(type_: parse::Type) -> String {
    let p = match type_.pointer {
        Ptr::No => "",
        Ptr::Const => "*const ",
        Ptr::Mut => "*mut ",
    };
    format!("{}{}", p, conv_type_ident(type_.ident))
}

fn conv_type_ident(ident_: &str) -> String {
    ident(match ident_ {
        "size_t" => "usize",
        "std::size_t" => "usize",
        "int" => "c_int",
        "unsigned int" => "c_uint",
        "float" => "f32",
        "double" => "f64",
        "void" => "c_void",
        "char" => "c_char",
        "char32_t" => "char",
        "int8_t" => "i8",
        "uint8_t" => "u8",
        "int16_t" => "i16",
        "uint16_t" => "u16",
        "int32_t" => "i32",
        "uint32_t" => "u32",
        "int64_t" => "i64",
        "uint64_t" => "u64",
        "sf::Int8" => "i8",
        "sf::Uint8" => "u8",
        "sf::Int16" => "i16",
        "sf::Uint16" => "u16",
        "sf::Int32" => "i32",
        "sf::Uint32" => "u32",
        "sf::Int64" => "i64",
        "sf::Uint64" => "u64",
        "std::string" => "sfStdString",
        "std::vector<std::string>" => "sfStdStringVector",
        "std::vector<sf::VideoMode>" => "sfVideoModeVector",
        "std::vector<sf::SoundChannel>" => "sfSoundChannelVector",
        _ => ident_,
    })
}

fn ident(input: &str) -> String {
    let mut out = String::from(input);
    match input {
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move"
        | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super"
        | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" | "abstract"
        | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
        | "unsized" | "virtual" | "yield" | "try" => out.push('_'),
        _ => {}
    }
    out.remove_matches("::");
    out
}

#[test]
fn test_transform() {
    assert_eq!(
        transform(
            r#"extern "C" void sfRenderWindow_drawPrimitives(sfRenderWindow *renderWindow,
        const sfVertex *vertices, size_t vertexCount,
        sfPrimitiveType type, const sf::RenderStates *states) "#
        ),
        "pub fn sfRenderWindow_drawPrimitives(\
            renderWindow: *mut sfRenderWindow, \
            vertices: *const sfVertex, \
            vertexCount: usize, \
            type_: sfPrimitiveType, \
            states: *const sfRenderStates)"
    );
    assert_eq!(
        transform(
            r#"extern "C" sf::WindowHandle sfRenderWindow_getSystemHandle(const sfRenderWindow *renderWindow) {"#
        ),
        "pub fn sfRenderWindow_getSystemHandle(renderWindow: *const sfRenderWindow) -> sfWindowHandle"
    )
}
