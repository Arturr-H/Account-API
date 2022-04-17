
use termcolor::{ Color };
use termcolor::{ ColorChoice, ColorSpec, StandardStream, WriteColor };
use std::io::Write;

/*- Because when we change the terminal color, 
    it will keep the same color for future lines -*/
fn reset_terminal_color(stdout: &mut StandardStream) {
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Rgb(171, 178, 191))))
            .unwrap();
}

/*- Print a response with colors -*/
pub fn throw_res(clr:Color, msg:&str) {
    /*- Set new standard output -*/
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    /*- Set the color to the inputted one -*/
    stdout.set_color(
        ColorSpec::new()
            .set_fg(Some(clr)))
            .unwrap();

    /*- Print it -*/
    writeln!(&mut stdout, "{}", msg).unwrap();

    /*- Reset the color -*/
    reset_terminal_color(&mut stdout);
}