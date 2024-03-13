
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env; // deal with command line 
use regex::Regex; //regular expression library tool , research more
use std::process;

pub struct Token { // create the tokens having their name and const val
    lexeme: String, 
    val_type: i32
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token { lexeme:self.lexeme.clone(), val_type: self.val_type, }
    }
}

// const values for tokens

const TOKEN_DATA: i32 = 0;
const TOKEN_INPUT: i32 = 1;
const TOKEN_PROCESS: i32 = 2;
const TOKEN_OUTPUT: i32 = 3;
const TOKEN_END: i32 = 4;
const TOKEN_ID: i32 = 5;
const TOKEN_NUM: i32 = 6;
const TOKEN_TRUE: i32 = 7;
const TOKEN_FALSE: i32 = 8;
const TOKEN_READ: i32 = 9;
const TOKEN_COLON: i32 = 10;
const TOKEN_COMMA: i32 = 11;
const TOKEN_PERIOD: i32 = 12;
const TOKEN_LPAREN: i32 = 13;
const TOKEN_RPAREN: i32 = 14;
const TOKEN_ASSIGN: i32 = 15;
const TOKEN_VECTOR: i32 = 16;
const TOKEN_NUMBER: i32 = 17;
const TOKEN_REGRESSIONA: i32 = 18;
const TOKEN_REGRESSIONB: i32 = 19;
const TOKEN_MEAN: i32 = 20;
const TOKEN_STDDEV: i32 = 21;
const TOKEN_CORRELATION: i32 = 22;
const TOKEN_STRING: i32 = 23;

fn main() { 
    
    let args: Vec<String> = env::args().collect(); // arguments

    if args.len() != 3 { // should be ran with 3 args
        println!("Incorrect number of parameters");
        process::exit(1);      
    }

    let file = &args[1];
    let filename = File::open(file).expect("cant open");
    let reader = BufReader::new(filename);

    let token_regex = Regex::new(r#"[a-z0-9_][a-z_]*|[(),:=.]|read|number|vector|"(.*?)""#).unwrap(); // regex created to seperate things in this specific grammar

    let mut token_vector: Vec<Token> = Vec::new();
    
    for line in reader.lines() {
        match line {
            Ok(line) => {    

                for c in line.chars() { // make sure no character is uppercase (lexical error)
                    if c.is_uppercase() {
                        println!("Lexical error!!");
                        process::exit(1);
                    }
                }
                
                for term in token_regex.find_iter(&line) { // uses regex to correctly seperate all the terms 

                    let matched_token = term.as_str().to_string(); // converts the regex "tokens" from a 'h to a string, easier use with strings

                    // special case to find STRINGS then push to vector 

                    if matched_token.chars().nth(0) == Some('"') {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_STRING,
                        };
                        token_vector.push(tmp);
                    } 
                            
                    // intantiating all the tokens and pushing them all into a vector of type tokens.. 

                    else if matched_token == "data" { 
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_DATA,
                        };
                        token_vector.push(tmp); 
                    }
                    else if matched_token == ":" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_COLON,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "=" {
                        let tmp = Token {
                            lexeme: matched_token, 
                            val_type: TOKEN_ASSIGN,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "vector" {
                        let tmp = Token {
                            lexeme: matched_token, 
                            val_type: TOKEN_VECTOR,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "," {
                        let tmp = Token {
                            lexeme: matched_token, 
                            val_type: TOKEN_COMMA,
                        };
                        token_vector.push(tmp);
                    } 
                    else if matched_token == "input" {
                        let tmp = Token {
                            lexeme: matched_token, 
                            val_type: TOKEN_INPUT,
                        };
                        token_vector.push(tmp);
                    } 
                    else if matched_token == "read" {
                        let tmp = Token {
                            lexeme: matched_token, 
                            val_type: TOKEN_READ,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "false" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_FALSE,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "true" {
                        let tmp = Token {
                            lexeme: matched_token, 
                            val_type: TOKEN_TRUE,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "0" || matched_token == "1" || matched_token == "2" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_NUM,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "(" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_LPAREN,
                        };
                        token_vector.push(tmp); 
                    }
                    else if matched_token == ")" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_RPAREN,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == ")" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_RPAREN,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "regressiona" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_REGRESSIONA,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "regressionb" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_REGRESSIONB,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "mean" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_MEAN,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "stddev" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_STDDEV,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "correlation" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_CORRELATION,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "number" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_NUMBER,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "process" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_PROCESS,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "output" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_OUTPUT,
                        };
                        token_vector.push(tmp);
                    }
                    else if matched_token == "." {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_PERIOD,
                        };
                        token_vector.push(tmp);
                    } 
                    else if matched_token == "end" {
                        let tmp = Token {
                            lexeme: matched_token,
                            val_type: TOKEN_END,
                        };
                        token_vector.push(tmp);
                    }
                    else{
                        let tmp = Token {
                            lexeme: matched_token, 
                            val_type: TOKEN_ID,
                        };
                        token_vector.push(tmp);
                    }       
                }
            }
            Err(_) => {
                println!("Error reading a symbol");
            }
        }
    }
    
    let mut clone_vector = token_vector.clone(); // make a clone of vector so i can do the display

    parse_tokens(&mut token_vector);

   // need to do display

   let last_arg = &args[2];

   if last_arg == "-s" { // last arg wants a scheme out put
    println!("; processing input file {}", file);
    println!("; Lexical and Syntax analysis passed");

    // part 1 of display
    scheme_input(&mut clone_vector); 

    // part 2 
    scheme_process(&mut clone_vector);

    // part 3
    scheme_output(&mut clone_vector);
   }
   else if last_arg == "-p" { // last arg wants a prolog output 
    println!("/* processing input file {}", file);
    println!("   Lexical and Syntax analysis passed */"); 
    println!();
    println!("main :-");

    // part 1 of display
    prolog_input(&mut clone_vector);

    // part 2
    prolog_process(&mut clone_vector);

    // part 3
    prolog_output(&mut clone_vector);
   }
   else { // those are the only two last args, so else fail
    println!("Invalid last argument!");
    println!("Enter -s for a scheme output");
    println!("Enter -p for a prolog output");
    process::exit(1);
   }

}

// function to handle the whole parsing process

pub fn parse_tokens(vec_tokens: &mut Vec<Token>) {
 
    let mut popped_token = vec_tokens.remove(0); // should be token w lexeme: data

    if popped_token.val_type == TOKEN_DATA {  
        popped_token = vec_tokens.remove(0);
        if popped_token.val_type == TOKEN_COLON {
            process_datadefs(vec_tokens); 
        }
    }
    else {
        println!("Syntax error!");
        process::exit(1);
    }

    popped_token = vec_tokens.remove(0); // should be token w lexeme: input
    
    if popped_token.val_type == TOKEN_INPUT {
        popped_token = vec_tokens.remove(0);
        if popped_token.val_type == TOKEN_COLON {
            process_inputops(vec_tokens);
        }
    }
    else {
        println!("Syntax error!");
        process::exit(1);
    }

    popped_token = vec_tokens.remove(0); // should be token w lexeme: process

    if popped_token.val_type == TOKEN_PROCESS {
        popped_token = vec_tokens.remove(0);
        if popped_token.val_type == TOKEN_COLON {
            process_proops(vec_tokens);
        }
    }
    else {
        println!("Syntax error!");
        process::exit(1);
    }

    popped_token = vec_tokens.remove(0); // should be token w lexeme: output

    if popped_token.val_type == TOKEN_OUTPUT {
        popped_token = vec_tokens.remove(0);
        if popped_token.val_type == TOKEN_COLON {
            process_outputops(vec_tokens);
        }
    }
    else {
        println!("Syntax error!");
        process::exit(1);
    }

    popped_token = vec_tokens.remove(0); // should be the last token, which should be '.'

    if popped_token.val_type != TOKEN_PERIOD {
        panic!("ERROR");
    }
    
} 

// function to process the "DATADEFS" of the grammar

pub fn process_datadefs (vec_tokens: &mut Vec<Token>) { 

    let mut curr_token = vec_tokens.remove(0);
    let mut tmp_token:&Token;
    let mut _warning_token:Token; // where I remove back to back i get a warning since I dont "read" first one.

    while curr_token.val_type == TOKEN_ID && curr_token.val_type != TOKEN_INPUT { // want to end if 

        curr_token = vec_tokens.remove(0);
        if curr_token.val_type == TOKEN_COLON {
            curr_token = vec_tokens.remove(0);
            if curr_token.val_type == TOKEN_VECTOR || curr_token.val_type == TOKEN_NUMBER { // cant remove after this if next token is lexeme: input 

                tmp_token = &vec_tokens[0]; // need to see if next spot is a comma without removing it
                if tmp_token.val_type == TOKEN_COMMA {
                    if curr_token.val_type == TOKEN_VECTOR || curr_token.val_type == TOKEN_NUMBER {
                    _warning_token = vec_tokens.remove(0);
                    curr_token = vec_tokens.remove(0);
                    }
                }  
            }
        }
    }
}

// function to process the "INPUTOPS" of the grammar

pub fn process_inputops (vec_tokens: &mut Vec<Token>) { // current token of the function should be lexeme: input || val_type: TOKEN_INPUT

    let mut tmp_token:&Token; // use this to look at end of inputops without removing

    let mut curr_token = vec_tokens.remove(0); // refers to token that is removed

    let mut _warning_token:Token; // where I remove back to back i get a warning since I dont "read" first one.

    while curr_token.val_type == TOKEN_ID && curr_token.val_type != TOKEN_PROCESS { 
        curr_token = vec_tokens.remove(0);
        if curr_token.val_type == TOKEN_ASSIGN {
            curr_token = vec_tokens.remove(0);

            if curr_token.val_type == TOKEN_READ {
                curr_token = vec_tokens.remove(0);

                if curr_token.val_type == TOKEN_LPAREN {
                    curr_token = vec_tokens.remove(0);
    
                    if curr_token.val_type == TOKEN_STRING {
                        curr_token = vec_tokens.remove(0);
    
                        if curr_token.val_type == TOKEN_COMMA {
                            curr_token = vec_tokens.remove(0);
    
                            if curr_token.val_type == TOKEN_TRUE || curr_token.val_type == TOKEN_FALSE {
                                curr_token = vec_tokens.remove(0);
    
                                if curr_token.val_type == TOKEN_COMMA {
                                    curr_token = vec_tokens.remove(0);
    
                                    if curr_token.val_type == TOKEN_NUM {
                                        curr_token = vec_tokens.remove(0);
    
                                        if curr_token.val_type == TOKEN_RPAREN {
                                            tmp_token = &vec_tokens[0];
                                            if tmp_token.val_type == TOKEN_COMMA {
                                                _warning_token = vec_tokens.remove(0);
                                                curr_token = vec_tokens.remove(0);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }   
            }       
        }
    }
}

// function to process the "PROCESSOPS" of the grammar

pub fn process_proops (vec_tokens: &mut Vec<Token>) { // current token of the function should be lexeme: input || val_type: TOKEN_INPUT

    let mut tmp_token:&Token; // use this to look at end of inputops without removing

    let mut curr_token = vec_tokens.remove(0); // refers to token that is removed

    let mut _warning_token:Token; // where I remove back to back i get a warning since I dont "read" first one.


    while curr_token.val_type == TOKEN_ID && curr_token.val_type != TOKEN_OUTPUT { 
        curr_token = vec_tokens.remove(0);
        if curr_token.val_type == TOKEN_ASSIGN {
            curr_token = vec_tokens.remove(0);

            if curr_token.val_type == TOKEN_REGRESSIONA {
                curr_token = vec_tokens.remove(0); 
                if curr_token.val_type == TOKEN_LPAREN {
                    curr_token = vec_tokens.remove(0); 
                    if curr_token.val_type == TOKEN_ID {
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_COMMA {
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_ID {
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_RPAREN {
                                    tmp_token = &vec_tokens[0];
                                    if tmp_token.val_type == TOKEN_COMMA {
                                    _warning_token = vec_tokens.remove(0);
                                    curr_token = vec_tokens.remove(0);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            else if curr_token.val_type == TOKEN_REGRESSIONB {
                curr_token = vec_tokens.remove(0); 
                if curr_token.val_type == TOKEN_LPAREN {
                    curr_token = vec_tokens.remove(0); 
                    if curr_token.val_type == TOKEN_ID {
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_COMMA {
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_ID {
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_RPAREN {
                                    tmp_token = &vec_tokens[0];
                                    if tmp_token.val_type == TOKEN_COMMA {
                                    _warning_token = vec_tokens.remove(0);
                                    curr_token = vec_tokens.remove(0);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            else if curr_token.val_type == TOKEN_MEAN {
                curr_token = vec_tokens.remove(0); 
                if curr_token.val_type == TOKEN_LPAREN {
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_ID {
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_RPAREN {
                            tmp_token = &vec_tokens[0];
                            if tmp_token.val_type == TOKEN_COMMA {
                                _warning_token = vec_tokens.remove(0);
                                curr_token = vec_tokens.remove(0);
                            }
                        }
                    }
                }
            } 

            else if curr_token.val_type == TOKEN_STDDEV {
                curr_token = vec_tokens.remove(0);
                if curr_token.val_type == TOKEN_LPAREN {
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_ID {
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_RPAREN {
                            tmp_token = &vec_tokens[0];
                            if tmp_token.val_type == TOKEN_COMMA {
                                _warning_token = vec_tokens.remove(0);
                                curr_token = vec_tokens.remove(0);
                            }
                        }
                    }
                } 
            } 

            else if curr_token.val_type == TOKEN_CORRELATION {
                curr_token = vec_tokens.remove(0); 
                if curr_token.val_type == TOKEN_LPAREN {
                    curr_token = vec_tokens.remove(0); 
                    if curr_token.val_type == TOKEN_ID {
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_COMMA {
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_ID {
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_RPAREN {
                                    tmp_token = &vec_tokens[0];
                                    if tmp_token.val_type == TOKEN_COMMA {
                                    _warning_token = vec_tokens.remove(0);
                                    curr_token = vec_tokens.remove(0);
                                    }
                                }
                            }
                        }
                    }
                }
            }        
        }
    }
}

// function to process the "OUTPUTOPS" of the grammar

pub fn process_outputops (vec_tokens: &mut Vec<Token>) { 

    let mut curr_token = vec_tokens.remove(0);
    let mut tmp_token:&Token;

    while curr_token.val_type != TOKEN_END { // want to end if 
        curr_token = vec_tokens.remove(0); // need to make it so it doesn't pop if current is END.. but dont want to remove end. 
        if curr_token.val_type == TOKEN_COMMA {
            curr_token = vec_tokens.remove(0);
            tmp_token = &vec_tokens[0]; // need to see if next spot is a comma without removing it
                if tmp_token.val_type == TOKEN_COMMA {
                    curr_token = vec_tokens.remove(0);
                }  
        }
    }  
}

// display for scheme

pub fn scheme_input (vec_tokens: &mut Vec<Token>) {

    let mut curr_token = vec_tokens.remove(0); // set current token to first thing (should be lexeme:data)
    let mut _id_string = String::new();
    let f_string = String::from("#f"); // false
    let t_string = String::from("#t"); // true
    let mut file_change:String;
    let pre_file = String::from("\"./");
    
    while curr_token.val_type != TOKEN_INPUT { // want to get all the way to the unput stage of the vector
        curr_token = vec_tokens.remove(0);
    }

    curr_token = vec_tokens.remove(0);

    if curr_token.val_type == TOKEN_COLON {
        curr_token = vec_tokens.remove(0);
    }

    // inputs display

    while curr_token.val_type == TOKEN_ID {

        _id_string = curr_token.lexeme.clone();
        print!("(define {}", _id_string);
        curr_token = vec_tokens.remove(0);
        if curr_token.val_type == TOKEN_ASSIGN {
            curr_token = vec_tokens.remove(0);
            if curr_token.val_type == TOKEN_READ {
                print!(" ({}", curr_token.lexeme);
                print!("-csv");
                curr_token = vec_tokens.remove(0);
                if curr_token.val_type == TOKEN_LPAREN {
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_STRING {
                        file_change = curr_token.lexeme.chars().skip(1).collect();
                        print!(" {}", pre_file);
                        print!("{}", file_change);
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_COMMA {
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_FALSE {
                                print!(" {}", f_string);
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_COMMA {
                                    curr_token = vec_tokens.remove(0);
                                    if curr_token.val_type == TOKEN_NUM {
                                        print!(" {}", curr_token.lexeme);
                                        print!("))");
                                        curr_token = vec_tokens.remove(0);
                                        if curr_token.val_type == TOKEN_RPAREN {
                                            curr_token = vec_tokens.remove(0);
                                            if curr_token.val_type == TOKEN_COMMA {
                                                curr_token = vec_tokens.remove(0);
                                            }
                                        }
                                    }
                                }
                            }
                            else if curr_token.val_type == TOKEN_TRUE {
                                print!(" {}", t_string);
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_COMMA {
                                    curr_token = vec_tokens.remove(0);
                                    if curr_token.val_type == TOKEN_NUM {
                                        print!(" {}", curr_token.lexeme);
                                        print!("))");
                                        curr_token = vec_tokens.remove(0);
                                        if curr_token.val_type == TOKEN_RPAREN {
                                            curr_token = vec_tokens.remove(0);
                                            if curr_token.val_type == TOKEN_COMMA {
                                                curr_token = vec_tokens.remove(0);
                                            }
                                        }      
                                    }
                                }
                            }
                        }
                    }    
                }
            }
        }
        println!(); // endl
    }  
}

pub fn scheme_process (vec_tokens: &mut Vec<Token>) {

    let mut curr_token = vec_tokens.remove(0); // set current token to first thing (should be lexeme: :)

    if curr_token.val_type == TOKEN_COLON {
        curr_token = vec_tokens.remove(0);
        while curr_token.val_type == TOKEN_ID {
            print!("(define {}", curr_token.lexeme);
            curr_token = vec_tokens.remove(0);
            if curr_token.val_type == TOKEN_ASSIGN {
                curr_token = vec_tokens.remove(0);
                if curr_token.val_type == TOKEN_REGRESSIONA || curr_token.val_type == TOKEN_REGRESSIONB || curr_token.val_type == TOKEN_CORRELATION {
                    print!(" ({}", curr_token.lexeme);
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_LPAREN {
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_ID {
                            print!(" {}", curr_token.lexeme);
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_COMMA {
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_ID {
                                    print!(" {}", curr_token.lexeme);
                                    print!("))");
                                    curr_token = vec_tokens.remove(0);
                                    if curr_token.val_type == TOKEN_RPAREN {
                                        curr_token = vec_tokens.remove(0);
                                        if curr_token.val_type == TOKEN_COMMA {
                                            curr_token = vec_tokens.remove(0);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                else if curr_token.val_type == TOKEN_MEAN || curr_token.val_type == TOKEN_STDDEV {
                    print!(" ({}", curr_token.lexeme);
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_LPAREN {
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_ID {
                            print!(" {}", curr_token.lexeme);
                            print!("))");
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_RPAREN {
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_COMMA {
                                    curr_token = vec_tokens.remove(0);
                                }
                            }
                        }
                    }
                }
            }
            println!(); // endl
        }
    }
}

pub fn scheme_output (vec_tokens: &mut Vec<Token>) {
    let mut curr_token = vec_tokens.remove(0); // set current token to first thing (should be lexeme: :)

    if curr_token.val_type == TOKEN_COLON {
        curr_token = vec_tokens.remove(0);
        while curr_token.val_type == TOKEN_STRING {
            print!("(display {}", curr_token.lexeme);
            print!(")");
            println!();
            println!("{}", "(newline)");
            curr_token = vec_tokens.remove(0);
            if curr_token.val_type == TOKEN_COMMA {
                curr_token = vec_tokens.remove(0);
                if curr_token.val_type == TOKEN_ID {
                    print!("(display {}", curr_token.lexeme);
                    print!(")");
                    println!();
                    println!("{}", "(newline)");
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_COMMA {
                        curr_token = vec_tokens.remove(0);
                    }
                }
            }
        }
    }
}

// functions for prolog display

pub fn prolog_input (vec_tokens: &mut Vec<Token>) {

    let mut curr_token = vec_tokens.remove(0);

    let first_id:String = String::from("Data0"); // replace xvals
    let second_id:String = String::from("Data1"); // replace yvals
    let load_data = String::from("load_data_column"); // replace read with this

    let mut file_change:String; 
    let marks= String::from('\''); // these need to replace quotes on files for prolog

    let mut tmp_token:&Token; // need a tmp token so i can see net position without removing

    while curr_token.val_type != TOKEN_INPUT {
        curr_token = vec_tokens.remove(0);
    }

    curr_token = vec_tokens.remove(0);

    if curr_token.val_type == TOKEN_COLON {
        curr_token = vec_tokens.remove(0);
    }

    // start performing the lines

    while curr_token.val_type == TOKEN_ID {
        curr_token = vec_tokens.remove(0);
        if curr_token.val_type == TOKEN_ASSIGN {
            curr_token = vec_tokens.remove(0);
            if curr_token.val_type == TOKEN_READ {
                print!("   {}", load_data);
                curr_token = vec_tokens.remove(0);
                if curr_token.val_type == TOKEN_LPAREN {
                    print!("(");
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_STRING {
                        file_change = curr_token.lexeme.chars().skip(1).collect();
                        file_change = file_change.to_string();
                        if file_change.ends_with('"') {
                            file_change.pop();
                        }
                        print!{"{}", marks};
                        print!("{}", file_change);
                        print!{"{}", marks};
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_COMMA {
                            print!(", ");
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_FALSE {
                                print!("{}", curr_token.lexeme);
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_COMMA {
                                    print!(", ");
                                    curr_token = vec_tokens.remove(0);
                                    if curr_token.val_type == TOKEN_NUM {
                                        print!("{}", curr_token.lexeme);
                                        print!(", ");
                                        curr_token = vec_tokens.remove(0);
                                        if curr_token.val_type == TOKEN_RPAREN {
                                           tmp_token = &vec_tokens[0];
                                           if tmp_token.val_type == TOKEN_COMMA {
                                               print!("{}", first_id);
                                               print!("),");   
                                           }
                                           else {
                                               print!("{}", second_id);
                                               print!("),");
                                           }
                                           curr_token = vec_tokens.remove(0);
                                           if curr_token.val_type == TOKEN_COMMA {
                                            curr_token = vec_tokens.remove(0);
                                           }
                                        }
                                    }
                                }
                            }
                            else if curr_token.val_type == TOKEN_TRUE {
                                print!("{}", curr_token.lexeme);
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_COMMA {
                                    print!(", ");
                                    curr_token = vec_tokens.remove(0);
                                    if curr_token.val_type == TOKEN_NUM {
                                        print!("{}", curr_token.lexeme);
                                        print!(", ");
                                        curr_token = vec_tokens.remove(0);
                                        if curr_token.val_type == TOKEN_RPAREN {
                                           tmp_token = &vec_tokens[0];
                                           if tmp_token.val_type == TOKEN_COMMA {
                                               print!("{}", first_id);
                                               print!("),");   
                                           }
                                           else {
                                               print!("{}", second_id);
                                               print!("),");
                                           }
                                           curr_token = vec_tokens.remove(0);
                                           if curr_token.val_type == TOKEN_COMMA {
                                            curr_token = vec_tokens.remove(0);
                                           }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!();
    }
}

pub fn prolog_process (vec_tokens: &mut Vec<Token>) {
    
    let mut curr_token = vec_tokens.remove(0); // set current token to first thing (should be lexeme: :)

    let mut id_string = String::new(); 

    let first_id:String = String::from("Data0"); // replace xvals
    let second_id:String = String::from("Data1"); // replace yvals 


    if curr_token.val_type == TOKEN_COLON {
        curr_token = vec_tokens.remove(0);
        while curr_token.val_type == TOKEN_ID {
            if curr_token.lexeme == "a" {
                id_string = String::from("A");
            }
            else if curr_token.lexeme == "b" {
                id_string = String::from("B");
            }
            else if curr_token.lexeme == "r" {
                id_string = String::from("R");
            }
            else if curr_token.lexeme == "themean"{
                id_string = String::from("THEMEAN");
            }
            else if curr_token.lexeme == "thevar"{
                id_string = String::from("THEVAR");
            }
            curr_token = vec_tokens.remove(0);
            if curr_token.val_type == TOKEN_ASSIGN {
                curr_token = vec_tokens.remove(0);
                if curr_token.val_type == TOKEN_REGRESSIONA || curr_token.val_type == TOKEN_REGRESSIONB || curr_token.val_type == TOKEN_CORRELATION {
                    print!("   {}", curr_token.lexeme);
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_LPAREN {
                        print!("(");
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_ID {
                            print!("{}", first_id);
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_COMMA {
                                print!(", ");
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_ID {
                                    print!("{}", second_id);
                                    print!(", ");
                                    print!("{}", id_string);
                                    curr_token = vec_tokens.remove(0);
                                    if curr_token.val_type == TOKEN_RPAREN {
                                        print!(")");
                                        print!(",");
                                        curr_token = vec_tokens.remove(0);
                                        if curr_token.val_type == TOKEN_COMMA {
                                            curr_token = vec_tokens.remove(0);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                else if curr_token.val_type == TOKEN_MEAN {
                    print!("   {}", curr_token.lexeme);
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_LPAREN {
                        print!("(");
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_ID {
                            print!("{}", first_id);
                            print!(", ");
                            print!("THEMEAN");
                            print!(")");
                            print!(",");
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_RPAREN {
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_COMMA {
                                    curr_token = vec_tokens.remove(0);
                                }
                            }
                        }
                    }
                }
                else if curr_token.val_type == TOKEN_STDDEV {
                    print!("   {}", curr_token.lexeme);
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_LPAREN {
                        print!("(");
                        curr_token = vec_tokens.remove(0);
                        if curr_token.val_type == TOKEN_ID {
                            print!("{}", second_id);
                            print!(", ");
                            print!("THEVAR");
                            print!(")");
                            print!(",");
                            curr_token = vec_tokens.remove(0);
                            if curr_token.val_type == TOKEN_RPAREN {
                                curr_token = vec_tokens.remove(0);
                                if curr_token.val_type == TOKEN_COMMA {
                                    curr_token = vec_tokens.remove(0);
                                }
                            }
                        }
                    }
                }
            }
            println!(); // endl
        }
    }
}

pub fn prolog_output (vec_tokens: &mut Vec<Token>) {

    let mut curr_token = vec_tokens.remove(0); // set current token to first thing (should be lexeme: :)

    let mut id_string = String::new();

    if curr_token.val_type == TOKEN_COLON {
        curr_token = vec_tokens.remove(0);
        while curr_token.val_type == TOKEN_STRING {
            print!("   writeln({}", curr_token.lexeme);
            print!("),");
            println!();
            curr_token = vec_tokens.remove(0);
            if curr_token.val_type == TOKEN_COMMA {
                curr_token = vec_tokens.remove(0);
                if curr_token.val_type == TOKEN_ID {
                    if curr_token.lexeme == "a" {
                        id_string = String::from("A");
                    }
                    else if curr_token.lexeme == "b" {
                        id_string = String::from("B");
                    }
                    else if curr_token.lexeme == "r" {
                        id_string = String::from("R");
                    }
                    else if curr_token.lexeme == "themean"{
                        id_string = String::from("THEMEAN");
                    }
                    else if curr_token.lexeme == "thevar"{
                        id_string = String::from("THEVAR");
                    }
                    print!("   writeln({}", id_string);
                    print!(")"); 
                    curr_token = vec_tokens.remove(0);
                    if curr_token.val_type == TOKEN_COMMA {
                        print!(",");
                        println!();
                        curr_token = vec_tokens.remove(0);
                    }
                    else {
                        print!(".");
                        println!();
                    }
                }
            }
        }
    }
}