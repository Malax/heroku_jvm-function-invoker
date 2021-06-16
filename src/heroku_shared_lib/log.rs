use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn log_error<H: AsRef<str>, B: AsRef<str>>(header: H) -> impl Fn(B) {
    move |body| {
        let mut stream = StandardStream::stderr(ColorChoice::Always);
        stream
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))
            .unwrap();
        writeln!(&mut stream, "\n[Error: {}]", header.as_ref()).unwrap();
        stream.reset().unwrap();

        stream
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .unwrap();
        writeln!(&mut stream, "{}", body.as_ref()).unwrap();
        stream.flush().unwrap();
    }
}

pub fn log_header(title: impl AsRef<str>) {
    let mut stream = StandardStream::stdout(ColorChoice::Always);
    stream
        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)).set_bold(true))
        .unwrap();
    writeln!(&mut stream, "\n[{}]", title.as_ref()).unwrap();
    stream.reset().unwrap();
    stream.flush().unwrap();
}

pub fn log_info(message: impl AsRef<str>) {
    println!("{}", message.as_ref());
    std::io::stdout().flush().unwrap();
}
