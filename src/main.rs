use std::io::{stdin, Write, stdout};
use rand::Rng;
use std::process::Command;

const XAXIS:[isize;8] =[-1,-1,-1,0,0,1,1,1];
const YAXIS:[isize;8]=[-1,0,1,-1,1,-1,0,1];
fn clear_screen() {
    Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .expect("Failed to clear console");
}

fn welcome_strings(){
    println!();
    println!("\t\tSimple Minesweeper game.");
    println!();
}

fn mines_generator(real_board: &mut Vec<Vec<char>>){
    for _i in 1..=10{
        let rand_x:usize=rand::thread_rng().gen_range(0..=8);
        let rand_y:usize=rand::thread_rng().gen_range(0..=8);
        real_board[rand_x][rand_y]='*';
    }
    cal_count_mine_surround(real_board);
}

fn cell_parser(row: usize,col: usize,nth: usize) -> (isize,isize){
    let goes_x = row as isize +XAXIS[nth];
    let goes_y = col as isize +YAXIS[nth];

    (goes_x,goes_y)
}


fn cal_count_mine_surround(real_board: &mut Vec<Vec<char>>){
    let mut counter: u32;
    for i in 0..=8{
        for j in 0..=8{
            counter=0;
            if real_board[i][j]=='0'{
                for n in 0..=7 {
                    let (first_val,sec_val)=cell_parser(i, j,n);
                    if first_val >= 0 && first_val < 9 && sec_val >= 0 && sec_val < 9{
                        let row_index=first_val as usize;
                        let col_index=sec_val as usize;
                        if real_board[row_index][col_index] == '*'{
                            counter+=1;
                        }
                    }
                    if let Some(counter_char)=std::char::from_u32(counter+'0' as u32){
                        real_board[i][j]=counter_char;
                    }
                }
            }
        }
    }
}
fn print_any_board(disp_board:&Vec<Vec<char>>){
    print!("\t\t\t   ");
    for _i in 1..=21{
        print!("__");
    }
    println!("_");
    print!("\t\t\t ");
    for i in 0..=8{
        print!("  |{i}|");
    }
    println!();
    print!("\t\t\t   ");
    for _i in 1..=21{
        print!("__");
    }
    println!("_");
    for i in 0..=8{
        print!("\t\t\t|{i}| ");
        for j in 0..=8{
            if j==8{
                let val=disp_board[i][j];
                print!("{val}|")
            }
            else{
            print!("{:3}  ",disp_board[i][j]);}

        }
        println!();
    }
    print!("\t\t\t   ");
    for _i in 1..=21{
        print!("__");
    }
    println!("_");
}

fn input_dims(xaxis: &mut String,msg: &String){
    print!("\t\t\t");
    print!("{msg}");
    stdout().flush().expect("Failed to flush.");
    stdin().read_line(xaxis).expect("Failed to read line.");
}

fn input_parser(input_dim: &String, validator: &mut u32) -> u32 {
    let after_trim=input_dim.trim();
    let last_char = match after_trim.chars().last() {
        Some(c) => c,
        None => {
            return 9;
        }
    };

    let output = match last_char.to_digit(10) {
        Some(num) if (0..=8).contains(&num) => num,
        _ => {
            return 9;
        }
    };

    *validator += 1;
    output
}

fn open_connected_cells(
    real_board: &mut Vec<Vec<char>>,
    display_board: &mut Vec<Vec<char>>,
    xaxis: isize,
    yaxis: isize
) {
    let rows = real_board.len() as isize;
    let cols = real_board[0].len() as isize;

    if xaxis < 0 || xaxis >= rows || yaxis < 0 || yaxis >= cols {
        return;
    }


    if display_board[xaxis as usize][yaxis as usize] != '\u{2588}' {
        return;
    }

    if real_board[xaxis as usize][yaxis as usize] != '0' && real_board[xaxis as usize][yaxis as usize] != '*'
        && display_board[xaxis as usize][yaxis as usize] == '\u{2588}' {
        display_board[xaxis as usize][yaxis as usize] = real_board[xaxis as usize][yaxis as usize];
    }

    else if real_board[xaxis as usize][yaxis as usize] == '0' && display_board[xaxis as usize][yaxis as usize] == '\u{2588}' {
        display_board[xaxis as usize][yaxis as usize] = '-';
        open_connected_cells(real_board, display_board, xaxis - 1, yaxis - 1);
        open_connected_cells(real_board, display_board, xaxis - 1, yaxis);
        open_connected_cells(real_board, display_board, xaxis - 1, yaxis + 1);
        open_connected_cells(real_board, display_board, xaxis, yaxis - 1);
        open_connected_cells(real_board, display_board, xaxis, yaxis + 1);
        open_connected_cells(real_board, display_board, xaxis + 1, yaxis - 1);
        open_connected_cells(real_board, display_board, xaxis + 1, yaxis);
        open_connected_cells(real_board, display_board, xaxis + 1, yaxis + 1);
    }
}
fn open_cells(real_board: &mut Vec<Vec<char>>,display_board: &mut Vec<Vec<char>>,xaxis:u32,yaxis:u32) -> bool{
    let x1=xaxis as usize;
    let y1=yaxis as usize;
    if real_board[x1][y1] != '0' && real_board[x1][y1] != '*' && display_board[x1][y1] == '\u{2588}'{
        display_board[x1][y1]=real_board[x1][y1];
        return true;
    }
    else if real_board[x1][y1] == '*' && display_board[x1][y1] == '\u{2588}'{
        display_board[x1][y1] == '*';
        return false;
    }
    else if real_board[x1][y1] == '0' && display_board[x1][y1] == '\u{2588}' {
        open_connected_cells(real_board,display_board,xaxis as isize,yaxis as isize);
    }
    return false;
}

fn to_opt(real_board: &mut Vec<Vec<char>>,display_board: &mut Vec<Vec<char>>,path: &char,xaxis:u32,yaxis:u32){
    if *path == 'f'{
        display_board[xaxis as usize][yaxis as usize] = 'F';
    }
    else if *path == 'o'{
        open_cells(real_board,display_board,xaxis,yaxis);
    }

}
fn options() -> char {
    let mut user_opt = String::new();
    print!("\t\t\tPress 'O' or 'o' to open cell OR 'F' or 'f' to flag a cell: ");
    stdout().flush().expect("Failed to flush.");
    loop {
        stdin().read_line(&mut user_opt).expect("Failed to print line.");
        let user_opt = user_opt.trim().to_lowercase();
        match user_opt.as_str(){
          "o" => return 'o',
          "f" => return 'f',
          _ => continue
        };
    }
}

fn main() {
    let mut real_board: Vec<Vec<char>>=vec![vec!['0';9];9]; let mut disp_board: Vec<Vec<char>>=vec![vec!['\u{2588}';9];9];
    let mut xaxis=String::new(); let mut yaxis=String::new();
    let msg_x=String::from("Enter x-axis: "); let msg_y=String::from("Enter y-axis: ");
    let mut validator:u32=0;
    loop{
        clear_screen();
        welcome_strings();
        print_any_board(&disp_board);
        input_dims(&mut xaxis,&msg_x);
        input_dims(&mut yaxis,&msg_y);
        let mut xaxis:u32=input_parser(&xaxis,&mut validator);
        let mut yaxis:u32=input_parser(&yaxis,&mut validator);
        if xaxis > 8 || yaxis > 8{
            continue;
        }
        if validator == 2{
            mines_generator(&mut real_board);
        }
        let opts:char = options();
        to_opt(&mut real_board,&mut disp_board,&opts,xaxis,yaxis);


    }


}