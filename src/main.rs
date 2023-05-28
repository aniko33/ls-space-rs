use fs2;
use clap::Parser;

const C: char = '#';

/// Simple program to scan memory disk
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to be scanned 
    #[arg(short, long)]
    path: String,

    /// Number of chars
    #[arg(short, long, default_value_t = 30)]
    chars: usize,
}

fn unit_converter(byte: u64) -> String {
    if byte < 1024 {
        format!("{} B", byte)
    } else if byte < 1024 * 1024 {
        format!("{:.2} KB", byte as f64 / 1024.0)
    } else if byte < 1024 * 1024 * 1024 {
        format!("{:.2} MB", byte as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", byte as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn make_bar(bar_vector: &mut Vec<char>, v: u64) -> String{
    for i in 0..v{
        let index: usize = i.try_into().unwrap();
        bar_vector[index] = C;
    }

    let bar: String = bar_vector.iter().collect();
    return bar;
}

fn main() {
    let args = Args::parse();

    let path: String = args.path;
    let size: usize = args.chars;


    let total_space: u64 = fs2::total_space(&path).unwrap();
    let available_space: u64 = fs2::available_space(&path).unwrap();
    let used_space: u64 = total_space - available_space;
    
    let size_int: i64 = size.try_into().unwrap();

    let used_p: f64 = used_space as f64 / total_space as f64 * 100.0;
    let available_p: f64 = available_space as f64 / total_space as f64 * 100.0;

    let calc_used: u64 = (used_p / 100.0 * size_int as f64) as u64;
    let calc_available: u64 = (available_p / 100.0 * size_int as f64) as u64;

    let mut bar_available: Vec<char> = vec!['.'; size];
    let mut bar_used: Vec<char> = vec!['.'; size];

    println!("\nTotal space: {} - Path: {path}\n", unit_converter(total_space));

    if available_p > 70.0{
        println!("[\x1b[0;32m{}\x1b[0m] {}", make_bar(&mut bar_available, calc_available), unit_converter(available_space));
    }
    else if available_p > 50.0{
        println!("[\x1b[0;33m{}\x1b[0m] {}", make_bar(&mut bar_available, calc_available), unit_converter(available_space));
    }
    else{
        println!("[\x1b[0;31m{}\x1b[0m] {}", make_bar(&mut bar_available, calc_available), unit_converter(available_space));
    }

    if used_p > 70.0 {
        println!("[\x1b[0;31m{}\x1b[0m] {}", make_bar(&mut bar_used, calc_used), unit_converter(used_space));
    }
    else if used_p > 40.0 {
        println!("[\x1b[0;33m{}\x1b[0m] {}", make_bar(&mut bar_used, calc_used), unit_converter(used_space));
    }
    else{
        println!("[\x1b[0;32m{}\x1b[0m] {}", make_bar(&mut bar_used, calc_used), unit_converter(used_space));
    }

}
