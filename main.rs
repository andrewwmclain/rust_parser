use std::env;
use std::fs;
use std::process;
use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::str; 

fn main() {
    //collect command line args in vector
    let args: Vec<String> = env::args().collect();

    //make struct to store command line args
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if(config.flag == "-s"){
        println!("; processing input file {}", config.filename);
    }
    if(config.flag == "-p"){
        println!("/* processing input file {}", config.filename);
    }

    //store input file content into "contents" string
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
    //store "contents" string characters into a vector
    let mut chars: Vec<char> = contents.chars().collect();
    //build lexical analysis regular expression
    let lex_check_regex = r"([a-z]|[0-9]|=|\(|\)|\.|;|[[:space:]]|,)";
    let lex_check = Regex::new(lex_check_regex).unwrap();
    //check each character from the input file to see if there are any invalid tokens
    for c in &chars{
        if !(lex_check.is_match(&c.to_string())){
            println!("Lexical anlaysis failed");
            process::exit(1);
        }
    }
    //build syntax analysis regular expression
    let syntax_check_regex2 = r"^[[:space:]]*[a-z]+[[:space:]]*=[[:space:]]*point[[:space:]]*\([[:space:]]*\d+[[:space:]]*,[[:space:]]*\d+[[:space:]]*\)[[:space:]]*;[[:space:]]*[a-z]+[[:space:]]*=[[:space:]]*point[[:space:]]*\([[:space:]]*\d+[[:space:]]*,[[:space:]]*\d+[[:space:]]*\)[[:space:]]*;[[:space:]]*[a-z]+[[:space:]]*=[[:space:]]*point[[:space:]]*\([[:space:]]*\d+[[:space:]]*,[[:space:]]*\d+[[:space:]]*\)[[:space:]]*\.[[:space:]]*$";
    let syntax_check2 = Regex::new(syntax_check_regex2).unwrap();
    //check to see if the file contents match the syntax analysis regex, i.e. is syntactically valid
    if !(syntax_check2.is_match(&contents)){
        println!("Syntax anlaysis failed");
        process::exit(1);
    }else{
        if(config.flag == "-s"){
            println!("; Lexical and Syntax analysis passed");
        }
        if(config.flag == "-p"){
            println!("   Lexical and Syntax analysis passed */");
        }
    }

    //declare a vector of strings which will be used to store each point as a string
    let mut points: Vec<String> = Vec::new();
    let mut point = String::new();
    //build the vector of point strings
    let mut i = 0;
    while(&i < &chars.len()){
        //skip past non-numbers
        while(i < chars.len() && !char::is_digit(chars[i], 10)){
            i += 1;
        }
        //add each digit to the point
        while(i < chars.len() && char::is_digit(chars[i], 10)){
            point.push(chars[i]);
            i += 1;
        }
        //add the point to the points vector
        points.push(point.to_string());
        point.clear();
    }
    //for scheme output
    if(config.flag == "-s"){
        println!("(calculate-triangle (make-point {0} {1}) (make-point {2} {3}) (make-point {4} {5}))", 
        points[0], points[1], points[2], points[3], points[4], points[5]);
    }
    //for prolog output
    if(config.flag == "-p"){
        //make vector of each shape
        let mut shapes = vec!["line","triangle","vertical","horizontal","equilateral","isosceles",
                              "right","scalene","acute","obtuse"];
        //print the prolog functions
        let mut j = 0;
        while(&j < &shapes.len()){
            println!("query({0}(point2d({1},{2}), point2d({3},{4}), point2d({5}, {6})))", shapes[j],
            points[0], points[1], points[2], points[3], points[4], points[5]);
            j += 1;
        }
        println!("writeln(T) :- write(T), nl.");
        println!("main:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),");
        println!("      halt.");
    }
}

struct Config {
    flag: String,
    filename: String,
}

//configuration implementation
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("incorrect number of arguments");
        }

        let filename = args[1].clone();
        let flag = args[2].clone();

        if (flag != "-p" && flag != "-s"){
            return Err("invalid flag")
        }

        Ok(Config { flag, filename })
    }
}
