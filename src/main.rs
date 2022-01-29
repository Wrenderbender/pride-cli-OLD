use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::env;
use getopt::Opt;
use terminal_size::terminal_size;

fn main(){
    let mut count:usize = 17; //default char width
    let args: Vec<String> = env::args().collect();
    let mut opts = getopt::Parser::new(&args,"C:Flgbtaq");
    let twidth = terminal_size().unwrap().0.0;
    loop {
        match opts.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt{
                Opt('C', Some(arg)) => count = arg.trim().parse().unwrap(),
                Opt('F', None) => count = twidth as usize, 
                Opt('l', None) => lesbianflag(count),
                Opt('g', None) => gayflag(count),
                Opt('b', None) => biflag(count),
                Opt('t', None) => transflag(count),
                Opt('a', None) => aceflag(count),
                Opt('p', None) => prideflag(count),
                Opt('q', None) => queerflag(count),
                _ => println!("option {} was not found, if you would like this flag to be added consider raising a github issue, please note I can only so many flags until there are no characters left.", opt)
            }
        }
    }
}

fn lesbianflag(count:usize){//18
    let outchar: &str = "█";//yes I know its technically a str but dont care + didn't ask + ratio + i have your ip(127.0.0.1) 
    let outstr = outchar.repeat(count);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(213,45,0)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(239,118,39)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(251,154,86)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();    
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,255,255)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap(); 
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(209,98,164)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap(); 
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(181,86,144)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap(); 
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(163,2,98)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
}

fn gayflag(count:usize){
    let outchar: &str = "█";//yes I know its technically a str but dont care + didn't ask + ratio + i have your ip(127.0.0.1) 
    let outstr = outchar.repeat(count); 
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(7,141,112)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(38,206,170)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(152,232,193)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,255,255)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(123,173,226)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(80,73,204)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(61,26,120)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
}

fn biflag(count:usize){
    let outchar: &str = "█";//yes I know its technically a str but dont care + didn't ask + ratio + i have your ip(127.0.0.1) 
    let outstr = outchar.repeat(count);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(214,2,112)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(155,79,150)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0,56,168)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();

}

fn transflag(count:usize){
    let outchar: &str = "█";//yes I know its technically a str but dont care + didn't ask + ratio + i have your ip(127.0.0.1) 
    let outstr = outchar.repeat(count);   
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(91,206,250)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(245,169,184)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,255,255)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(245,169,184)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(91,206,250)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
}

fn prideflag(count:usize){
    let outchar: &str = "█";//yes I know its technically a str but dont care + didn't ask + ratio + i have your ip(127.0.0.1) 
    let outstr = outchar.repeat(count); 
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,0,0)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,142,0)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,255,0)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0,142,0)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(64,0,152)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(142,0,142)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
}

fn aceflag(count:usize){
    let outchar: &str = "█";//yes I know its technically a str but dont care + didn't ask + ratio + i have your ip(127.0.0.1) 
    let outstr = outchar.repeat(count); 
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0,0,0)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(163,163,163)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,255,255)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(128,0,128)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
}

fn queerflag(count:usize){
    let outchar: &str = "█";//yes I know its technically a str but dont care + didn't ask + ratio + i have your ip(127.0.0.1) 
    let outstr = outchar.repeat(count); 
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(181,126,220)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255,255,255)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(74,129,35)))).unwrap();
    writeln!(&mut stdout, "{}",outstr).unwrap();
   
}
