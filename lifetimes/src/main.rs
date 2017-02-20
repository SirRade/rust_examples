fn main() {
    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
        v = skip_prefix(line, p.as_str());  //  |
    }                                       // -+ `p` goes out of scope.
    println!("{}", v);
}

fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    // ...
    line
}
