fn main() {
	let t:f64 = 450_000_000.0;
	let m:f64 = 1_500_000_000.00;
	let h:f64 = 750_000_000.0;
	let d:f64 = 2_850_000.0;
	let a:f64 = 250_000.0;

	// sum and average
	let sum = (2.0 * t) + m + (3.0 * h) + (3.0 * d) + a;
	println!("The sum is {}", sum);
	let avg = sum / 10.0;
	println!("The average is {}", avg);
}