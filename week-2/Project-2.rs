//Rust program to calculate the sum and average
fn main() {
	let a:f64 = 450000.00;
	let b:f64 = 1500000.00;
	let c:f64 = 750000.00;
	let d:f64 = 2850000.00;
	let e:f64 = 250000.00;

	// sum
	let s = a + b + c + d + e;
	println!("Sum is {}", s );
	let a = s / 5.0;
	println!("Average is {}",a );
}