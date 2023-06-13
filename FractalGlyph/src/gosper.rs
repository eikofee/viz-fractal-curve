
///! Gosper curve. Blind conversion of implementation from wulip of David Auber
///! Untested code
///! Cannot be used in pixel based represention as the coordinates do not lie on a grid

use rayon::prelude::*;

type Complex = num_complex::Complex64;
type Point = nalgebra::Point2<f64> ;


fn expIThetaCPP(theta: f64) -> Complex {
    Complex::new(theta.cos(), theta.sin())
}


fn isIn(id: u64, a: i64, b: i64) -> bool {
	let id = id as i64;
	let mi = a.min(b);
	let ma = a.max(b);
    id >= mi && id < ma
}


fn createCurveCPPONEID(x: Complex, y: Complex, order: u32, mut minI: i64, mut maxI: i64, id: u64, sens: bool) -> Point {

    if order == 0  {
        minI = minI.min(maxI);
        if sens {
            return Point::new(x.re, x.im);
		} else {
			return Point::new(y.re, y.im);
		}
    }

	let u = expIThetaCPP(std::f64::consts::PI/3.);
    let theta = -1.0 * (3.0f64.sqrt()/5.0).atan();
    let v =  (y - x) * expIThetaCPP(theta) / 7.0f64.sqrt();

    let P = x + v;
    let Q = P + u * v;
    let R = Q - v;
    let S = R + u * u * v;
    let T = S + v;
	let U = T + v;
	
    let size: i64 = (maxI - minI).abs() ;
    if size%7 != 0 {
		println!( "error : {}", size);
	}
    let size = size / 7;

	let rev = if minI > maxI {
		-1i64
	}
	else {
		1i64
	};

    if isIn(id,  minI + rev * size * 0, minI + rev * size * 1) {
		return createCurveCPPONEID(x, P, order-1, minI + rev * size * 0, minI + rev * size * 1, id, sens);
	}
    if isIn(id,  minI + rev * size * 1, minI + rev * size * 2) {
        return createCurveCPPONEID(Q, P, order-1, minI + rev * size * 2, minI + rev * size * 1, id, !sens);
	}
    if isIn(id, minI + rev * size * 2, minI + rev * size * 3) {
        return createCurveCPPONEID(R, Q, order-1, minI + rev * size * 3, minI + rev * size * 2, id, !sens);
	}
    if isIn(id, minI + rev * size * 3, minI + rev * size * 4) {
        return createCurveCPPONEID(R, S, order-1, minI + rev * size * 3, minI + rev * size * 4, id, sens);
	}
    if isIn(id, minI + rev * size * 4, minI + rev * size * 5) {
        return createCurveCPPONEID(S, T, order-1, minI + rev * size * 4, minI + rev * size * 5, id, sens);
	}
    if isIn(id, minI + rev * size * 5, minI + rev * size * 6) {
        return createCurveCPPONEID(T, U, order-1, minI + rev * size * 5, minI + rev * size * 6, id, sens);
	}
    if isIn(id, minI + rev * size * 6, minI + rev * size * 7) {
        return createCurveCPPONEID(y, U, order-1, minI + rev * size * 7, minI + rev * size * 6, id, !sens);
	}

    panic!("Gosper curve invalid configuration");
}


pub fn getCurveElement(order: u32, id: u64) -> Point {
    //=============================
    //generate oracle
	let xx = Complex::new(
		-(7.0f64.powi(order as _).sqrt()),
		0f64
	);
	let yy = Complex::new(
		7.0f64.powi(order as _).sqrt(),
		0f64
	);
	
    createCurveCPPONEID(xx, yy, order, 0, 7i64.pow(order as _), id, true)

}


pub fn curve(nb: u64) -> Vec<Point> {

	// todo implement an efficient method
	let order = {
		let mut order: u32 = 1;
		while (order as u64).pow(7)  < nb {
			order += 1;
		}
		order
	};
	
	(0..nb).into_par_iter()
		.map(|idx| {getCurveElement(order, idx)})
		.collect::<Vec<Point>>()
}

#[cfg(test)]
mod test{

	#[test]
	fn test_point(){

		println!("{}", super::getCurveElement(0, 1));
	}


	#[test]
	fn test_curve() {
		println!{"{:?}", super::curve(10)};
	}

}