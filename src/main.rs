// #[macro_use]
extern crate serde_derive;

extern crate url;
extern crate serde;
extern crate serde_json;
extern crate getopts;
extern crate hyper;
extern crate term;

use std::str::FromStr;
use std::env;
use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use getopts::Options;
use url::Url;



static BASE_URL :&'static str = "http://api.worldweatheronline.com/free/v2/weather.ashx";
static KEY :&'static str= "a444bbde1001764c4634bc7079a7c";




static iconUnknown: [&'static str; 5] = [
		"    .-.      ",
		"     __)     ",
		"    (        ",
		"     `-’     ",
		"      •      "];
static iconSunny: [&'static str; 5] = [
		"\u{1b}[38;5;226m    \\   /    \u{1b}[0m",
		"\u{1b}[38;5;226m     .-.     \u{1b}[0m",
		"\u{1b}[38;5;226m  ― (   ) ―  \u{1b}[0m",
		"\u{1b}[38;5;226m     `-’     \u{1b}[0m",
		"\u{1b}[38;5;226m    /   \\    \u{1b}[0m"];
static iconPartlyCloudy: [&'static str; 5] = [
		"\u{1b}[38;5;226m   \\  /\u{1b}[0m      ",
		"\u{1b}[38;5;226m _ /\"\"\u{1b}[38;5;250m.-.    \u{1b}[0m",
		"\u{1b}[38;5;226m   \\_\u{1b}[38;5;250m(   ).  \u{1b}[0m",
		"\u{1b}[38;5;226m   /\u{1b}[38;5;250m(___(__) \u{1b}[0m",
		"             "];
static iconCloudy: [&'static str; 5] = [
		"             ",
		"\u{1b}[38;5;250m     .--.    \u{1b}[0m",
		"\u{1b}[38;5;250m  .-(    ).  \u{1b}[0m",
		"\u{1b}[38;5;250m (___.__)__) \u{1b}[0m",
		"             "];
static iconVeryCloudy: [&'static str; 5] = [
		"             ",
		"\u{1b}[38;5;240;1m     .--.    \u{1b}[0m",
		"\u{1b}[38;5;240;1m  .-(    ).  \u{1b}[0m",
		"\u{1b}[38;5;240;1m (___.__)__) \u{1b}[0m",
		"             "];
static iconLightShowers: [&'static str; 5] = [
		"\u{1b}[38;5;226m _`/\"\"\u{1b}[38;5;250m.-.    \u{1b}[0m",
		"\u{1b}[38;5;226m  ,\\_\u{1b}[38;5;250m(   ).  \u{1b}[0m",
		"\u{1b}[38;5;226m   /\u{1b}[38;5;250m(___(__) \u{1b}[0m",
		"\u{1b}[38;5;111m     ‘ ‘ ‘ ‘ \u{1b}[0m",
		"\u{1b}[38;5;111m    ‘ ‘ ‘ ‘  \u{1b}[0m"];
static iconHeavyShowers: [&'static str; 5] = [
		"\u{1b}[38;5;226m _`/\"\"\u{1b}[38;5;240;1m.-.    \u{1b}[0m",
		"\u{1b}[38;5;226m  ,\\_\u{1b}[38;5;240;1m(   ).  \u{1b}[0m",
		"\u{1b}[38;5;226m   /\u{1b}[38;5;240;1m(___(__) \u{1b}[0m",
		"\u{1b}[38;5;21;1m   ‚‘‚‘‚‘‚‘  \u{1b}[0m",
		"\u{1b}[38;5;21;1m   ‚’‚’‚’‚’  \u{1b}[0m"];
static iconLightSnowShowers: [&'static str; 5] = [
		"\u{1b}[38;5;226m _`/\"\"\u{1b}[38;5;250m.-.    \u{1b}[0m",
		"\u{1b}[38;5;226m  ,\\_\u{1b}[38;5;250m(   ).  \u{1b}[0m",
		"\u{1b}[38;5;226m   /\u{1b}[38;5;250m(___(__) \u{1b}[0m",
		"\u{1b}[38;5;255m     *  *  * \u{1b}[0m",
		"\u{1b}[38;5;255m    *  *  *  \u{1b}[0m"];
static iconHeavySnowShowers: [&'static str; 5] = [
		"\u{1b}[38;5;226m _`/\"\"\u{1b}[38;5;240;1m.-.    \u{1b}[0m",
		"\u{1b}[38;5;226m  ,\\_\u{1b}[38;5;240;1m(   ).  \u{1b}[0m",
		"\u{1b}[38;5;226m   /\u{1b}[38;5;240;1m(___(__) \u{1b}[0m",
		"\u{1b}[38;5;255;1m    * * * *  \u{1b}[0m",
		"\u{1b}[38;5;255;1m   * * * *   \u{1b}[0m"];
static iconLightSleetShowers: [&'static str; 5] = [
		"\u{1b}[38;5;226m _`/\"\"\u{1b}[38;5;250m.-.    \u{1b}[0m",
		"\u{1b}[38;5;226m  ,\\_\u{1b}[38;5;250m(   ).  \u{1b}[0m",
		"\u{1b}[38;5;226m   /\u{1b}[38;5;250m(___(__) \u{1b}[0m",
		"\u{1b}[38;5;111m     ‘ \u{1b}[38;5;255m*\u{1b}[38;5;111m ‘ \u{1b}[38;5;255m* \u{1b}[0m",
		"\u{1b}[38;5;255m    *\u{1b}[38;5;111m ‘ \u{1b}[38;5;255m*\u{1b}[38;5;111m ‘  \u{1b}[0m"];
static iconThunderyShowers: [&'static str; 5] = [
		"\u{1b}[38;5;226m _`/\"\"\u{1b}[38;5;250m.-.    \u{1b}[0m",
		"\u{1b}[38;5;226m  ,\\_\u{1b}[38;5;250m(   ).  \u{1b}[0m",
		"\u{1b}[38;5;226m   /\u{1b}[38;5;250m(___(__) \u{1b}[0m",
		"\u{1b}[38;5;228;5m    ⚡\u{1b}[38;5;111;25m‘ ‘\u{1b}[38;5;228;5m⚡\u{1b}[38;5;111;25m‘ ‘ \u{1b}[0m",
		"\u{1b}[38;5;111m    ‘ ‘ ‘ ‘  \u{1b}[0m"];
static iconThunderyHeavyRain: [&'static str; 5] = [
		"\u{1b}[38;5;240;1m     .-.     \u{1b}[0m",
		"\u{1b}[38;5;240;1m    (   ).   \u{1b}[0m",
		"\u{1b}[38;5;240;1m   (___(__)  \u{1b}[0m",
		"\u{1b}[38;5;21;1m  ‚‘\u{1b}[38;5;228;5m⚡\u{1b}[38;5;21;25m‘‚\u{1b}[38;5;228;5m⚡\u{1b}[38;5;21;25m‚‘   \u{1b}[0m",
		"\u{1b}[38;5;21;1m  ‚’‚’\u{1b}[38;5;228;5m⚡\u{1b}[38;5;21;25m’‚’   \u{1b}[0m"];
static iconThunderySnowShowers: [&'static str; 5] = [
		"\u{1b}[38;5;226m _`/\"\"\u{1b}[38;5;250m.-.    \u{1b}[0m",
		"\u{1b}[38;5;226m  ,\\_\u{1b}[38;5;250m(   ).  \u{1b}[0m",
		"\u{1b}[38;5;226m   /\u{1b}[38;5;250m(___(__) \u{1b}[0m",
		"\u{1b}[38;5;255m     *\u{1b}[38;5;228;5m⚡\u{1b}[38;5;255;25m *\u{1b}[38;5;228;5m⚡\u{1b}[38;5;255;25m * \u{1b}[0m",
		"\u{1b}[38;5;255m    *  *  *  \u{1b}[0m"];
static iconLightRain: [&'static str; 5] = [
		"\u{1b}[38;5;250m     .-.     \u{1b}[0m",
		"\u{1b}[38;5;250m    (   ).   \u{1b}[0m",
		"\u{1b}[38;5;250m   (___(__)  \u{1b}[0m",
		"\u{1b}[38;5;111m    ‘ ‘ ‘ ‘  \u{1b}[0m",
		"\u{1b}[38;5;111m   ‘ ‘ ‘ ‘   \u{1b}[0m"];
static iconHeavyRain: [&'static str; 5] = [
		"\u{1b}[38;5;240;1m     .-.     \u{1b}[0m",
		"\u{1b}[38;5;240;1m    (   ).   \u{1b}[0m",
		"\u{1b}[38;5;240;1m   (___(__)  \u{1b}[0m",
		"\u{1b}[38;5;21;1m  ‚‘‚‘‚‘‚‘   \u{1b}[0m",
		"\u{1b}[38;5;21;1m  ‚’‚’‚’‚’   \u{1b}[0m"];
static iconLightSnow: [&'static str; 5] = [
		"\u{1b}[38;5;250m     .-.     \u{1b}[0m",
		"\u{1b}[38;5;250m    (   ).   \u{1b}[0m",
		"\u{1b}[38;5;250m   (___(__)  \u{1b}[0m",
		"\u{1b}[38;5;255m    *  *  *  \u{1b}[0m",
		"\u{1b}[38;5;255m   *  *  *   \u{1b}[0m"];
static iconHeavySnow: [&'static str; 5] = [
		"\u{1b}[38;5;240;1m     .-.     \u{1b}[0m",
		"\u{1b}[38;5;240;1m    (   ).   \u{1b}[0m",
		"\u{1b}[38;5;240;1m   (___(__)  \u{1b}[0m",
		"\u{1b}[38;5;255;1m   * * * *   \u{1b}[0m",
		"\u{1b}[38;5;255;1m  * * * *    \u{1b}[0m"];
static iconLightSleet: [&'static str; 5] = [
		"\u{1b}[38;5;250m     .-.     \u{1b}[0m",
		"\u{1b}[38;5;250m    (   ).   \u{1b}[0m",
		"\u{1b}[38;5;250m   (___(__)  \u{1b}[0m",
		"\u{1b}[38;5;111m    ‘ \u{1b}[38;5;255m*\u{1b}[38;5;111m ‘ \u{1b}[38;5;255m*  \u{1b}[0m",
		"\u{1b}[38;5;255m   *\u{1b}[38;5;111m ‘ \u{1b}[38;5;255m*\u{1b}[38;5;111m ‘   \u{1b}[0m"];
static iconFog: [&'static str; 5] = [
		"             ",
		"\u{1b}[38;5;251m _ - _ - _ - \u{1b}[0m",
		"\u{1b}[38;5;251m  _ - _ - _  \u{1b}[0m",
		"\u{1b}[38;5;251m _ - _ - _ - \u{1b}[0m",
		"             "];

fn code_to_icon(code:i32) -> [&'static str; 5] {
    match code {
        113 => iconSunny,
         113 => iconSunny,
        116 => iconPartlyCloudy,
        119 => iconCloudy,
        122 => iconVeryCloudy,
        143 => iconFog,
        176 => iconLightShowers,
        179 => iconLightSleetShowers,
        182 => iconLightSleet,
        185 => iconLightSleet,
        200 => iconThunderyShowers,
        227 => iconLightSnow,
        230 => iconHeavySnow,
        248 => iconFog,
        260 => iconFog,
        263 => iconLightShowers,
        266 => iconLightRain,
        281 => iconLightSleet,
        284 => iconLightSleet,
        293 => iconLightRain,
        296 => iconLightRain,
        299 => iconHeavyShowers,
        302 => iconHeavyRain,
        305 => iconHeavyShowers,
        308 => iconHeavyRain,
        311 => iconLightSleet,
        314 => iconLightSleet,
        317 => iconLightSleet,
        320 => iconLightSnow,
        323 => iconLightSnowShowers,
        326 => iconLightSnowShowers,
        329 => iconHeavySnow,
        332 => iconHeavySnow,
        335 => iconHeavySnowShowers,
        338 => iconHeavySnow,
        350 => iconLightSleet,
        353 => iconLightShowers,
        356 => iconHeavyShowers,
        359 => iconHeavyRain,
        362 => iconLightSleetShowers,
        365 => iconLightSleetShowers,
        368 => iconLightSnowShowers,
        371 => iconHeavySnowShowers,
        374 => iconLightSleetShowers,
        377 => iconLightSleet,
        386 => iconThunderyShowers,
        389 => iconThunderyHeavyRain,
        392 => iconThunderySnowShowers,
        395 => iconHeavySnowShowers, 
        _ => iconUnknown
    }
}



// pub struct Weather {

// }

fn print_usage(progarm: &str, opts:&Options) {
    let brief = format!("Usage: {} [options] [CITY]", progarm);
    print!("{}",opts.usage(&brief));
}

fn main() {
    //首先得搞定args
    let args :Vec<String>= env::args().collect();

    let mut opts = Options::new();

    opts.optflag("h", "help", "print help message")
        .optflag("","zh","use chinese")
        .optopt("d",  "days", "number of days in output", "DAYS");


    //Result<T,E>
    let matches = match opts.parse(&args[1..])
    {
        Ok(m) => m,
        Err(f) => panic!("There was a program: {:?}", f)
    };

    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
        return;
    }

    if matches.opt_present("zh"){

    }

    let city = if !matches.free.is_empty() {
        matches.free.join(" ")
    } else {
        "Beijing".to_string()
    };

    let number_of_days:usize= matches.opt_str("days")
                                     .map(|ref s| usize::from_str(s).ok().expect("days must be a number"))
                                     .unwrap_or(3);

    let mut url = Url::parse(BASE_URL).unwrap();
    url.query_pairs_mut()
        .clear()
        .append_pair("q",&city)
        .append_pair("key", KEY)
        .append_pair("number_of_days", &number_of_days.to_string())
        .append_pair("lang","zh")
        .append_pair("format","json");

   
    
}
