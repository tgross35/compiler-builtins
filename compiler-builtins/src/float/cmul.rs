use crate::float::Complex;
use crate::support::Float;

fn complex_mul<F: Float>(mut a: F, mut b: F, mut c: F, mut d: F) -> Complex<F> {
    let ac = a * c;
    let bd = b * d;
    let ad = a * d;
    let bc = b * c;

    let mut re = ac - bd;
    let mut im = ad + bc;

    let one_if_inf = |x: F| if x.is_infinite() { F::ONE } else { F::ZERO };

    if re.is_nan() && im.is_nan() {
        let mut recalc = false;

        if a.is_infinite() || b.is_infinite() {
            a = one_if_inf(a).copysign(a);
            b = one_if_inf(b).copysign(b);
            if c.is_nan() {
                c = F::ZERO.copysign(c);
            }
            if d.is_nan() {
                d = F::ZERO.copysign(d);
            }
            recalc = true;
        }

        if c.is_infinite() || d.is_infinite() {
            c = one_if_inf(c).copysign(c);
            d = one_if_inf(d).copysign(d);
            if a.is_nan() {
                a = F::ZERO.copysign(a);
            }
            if b.is_nan() {
                b = F::ZERO.copysign(b);
            }
            recalc = true;
        }

        if !recalc && (ac.is_infinite() || bd.is_infinite() || ad.is_infinite() || bc.is_infinite())
        {
            if a.is_nan() {
                a = a.copysign(F::ZERO);
            }
            if b.is_nan() {
                b = b.copysign(F::ZERO);
            }
            if c.is_nan() {
                c = c.copysign(F::ZERO);
            }
            if d.is_nan() {
                d = d.copysign(F::ZERO);
            }
            recalc = true;
        }

        if recalc {
            re = F::INFINITY * (a * c - b * d);
            im = F::INFINITY * (a * d + b * c);
        }
    }

    Complex { re, im }
}

intrinsics! {
    #[cfg(f16_enabled)]
    pub extern "C" fn __mulhc3(a: f16, b: f16, c: f16, d: f16) -> crate::float::Complex<f16> {
        complex_mul(a, b, c, d)
    }

    pub extern "C" fn __mulsc3(a: f32, b: f32, c: f32, d: f32) -> crate::float::Complex<f32> {
        complex_mul(a, b, c, d)
    }

    pub extern "C" fn __muldc3(a: f64, b: f64, c: f64, d: f64) -> crate::float::Complex<f64> {
        complex_mul(a, b, c, d)
    }

    #[ppc_alias = __mulkf3]
    #[cfg(f128_enabled)]
    pub extern "C" fn __multc3(a: f128, b: f128, c: f128, d: f128) -> crate::float::Complex<f128> {
        complex_mul(a, b, c, d)
    }
}
