use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Temp converter")]
#[command(about = "Small script to convert temperatures")]
struct Cli {
    #[arg(short, long)]
    celsius: Option<f64>,

    #[arg(short, long)]
    fahrenheit: Option<f64>,

    #[arg(short, long)]
    kelvin: Option<f64>,
}

fn print(c: f64, f: f64, k: f64) {
    println!("Celcius: {:.2} - Fahrenheit: {:.2} - Kelvin: {:.2}", c, f, k)
}

fn main() {
    let cli = Cli::parse();

    if let Some(temp) = cli.kelvin {
        let to_celcius: f64 = temp - 273.15;
        let to_farenheit: f64 = (temp - 273.15) * 9.0 / 5.0 + 32.0;
        print(to_celcius, to_farenheit, temp);
    }

    if let Some(temp) = cli.fahrenheit {
        let to_celcius: f64 = (temp - 32.0) * 5.0 / 9.0;
        let to_kelvin: f64 = (temp - 32.0) * 5.0 / 9.0 + 273.15;
        print(to_celcius, temp, to_kelvin);
    }

    if let Some(temp) = cli.celsius {
        let to_kelvin: f64 = temp + 273.15;
        let to_farenheit: f64 = temp * 9.0 / 5.0 + 32.0;
        print(temp, to_farenheit, to_kelvin);
    }
}
