#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unsafe_op_in_unsafe_fn
)]

use crate::support::Float;

pub type int64_t = i64;
pub type uint64_t = u64;
pub type ushort = ::core::ffi::c_ushort;
pub type u64_0 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union b64u64_u {
    pub f: f64,
    pub u: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub c0: ushort,
    pub c1: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub c0: ushort,
    pub c1: i16,
}
#[inline]
unsafe extern "C" fn fasttwosum(mut x: f64, mut y: f64, mut e: *mut f64) -> f64 {
    let mut s: f64 = x + y;
    let mut z: f64 = s - x;
    *e = y - z;
    return s;
}
#[inline]
unsafe extern "C" fn twosum(mut x: f64, mut y: f64, mut e: *mut f64) -> f64 {
    if (x.abs() > y.abs()) as i32 as ::core::ffi::c_long != 0 {
        return fasttwosum(x, y, e);
    } else {
        return fasttwosum(y, x, e);
    };
}
#[inline]
unsafe extern "C" fn fastsum(
    mut xh: f64,
    mut xl: f64,
    mut yh: f64,
    mut yl: f64,
    mut e: *mut f64,
) -> f64 {
    let mut sl: f64 = 0.;
    let mut sh: f64 = fasttwosum(xh, yh, &raw mut sl);
    *e = xl + yl + sl;
    return sh;
}
#[inline]
unsafe extern "C" fn sumdd(
    mut xh: f64,
    mut xl: f64,
    mut yh: f64,
    mut yl: f64,
    mut e: *mut f64,
) -> f64 {
    let mut sl: f64 = 0.;
    let mut sh: f64 = 0.;
    let mut o: ::core::ffi::c_char = (xh.abs() > yh.abs()) as i32 as ::core::ffi::c_char;
    if o as ::core::ffi::c_long != 0 {
        sh = fasttwosum(xh, yh, &raw mut sl);
    } else {
        sh = fasttwosum(yh, xh, &raw mut sl);
    }
    sl += xl + yl;
    *e = sl;
    return sh;
}
#[inline]
unsafe extern "C" fn muldd(
    mut xh: f64,
    mut xl: f64,
    mut ch: f64,
    mut cl: f64,
    mut l: *mut f64,
) -> f64 {
    let mut ahhh: f64 = ch * xh;
    *l = ch * xl + cl * xh + ch.fma(xh, -ahhh);
    return ahhh;
}
#[inline]
unsafe extern "C" fn mulddd(mut x: f64, mut ch: f64, mut cl: f64, mut l: *mut f64) -> f64 {
    let mut ahhh: f64 = ch * x;
    *l = cl * x + ch.fma(x, -ahhh);
    return ahhh;
}
#[inline]
unsafe extern "C" fn polydd(
    mut xh: f64,
    mut xl: f64,
    mut n: i32,
    mut c: *const [f64; 2],
    mut l: *mut f64,
) -> f64 {
    let mut i: i32 = n - 1 as i32;
    let mut cl: f64 = 0.;
    let mut ch: f64 = fasttwosum((*c.offset(i as isize))[0 as i32 as usize], *l, &raw mut cl);
    cl += (*c.offset(i as isize))[1 as i32 as usize];
    loop {
        i -= 1;
        if !(i >= 0 as i32) {
            break;
        }
        ch = muldd(xh, xl, ch, cl, &raw mut cl);
        ch = fastsum(
            (*c.offset(i as isize))[0 as i32 as usize],
            (*c.offset(i as isize))[1 as i32 as usize],
            ch,
            cl,
            &raw mut cl,
        );
    }
    *l = cl;
    return ch;
}
#[inline]
unsafe extern "C" fn polydddfst(
    mut x: f64,
    mut n: i32,
    mut c: *const [f64; 2],
    mut l: *mut f64,
) -> f64 {
    let mut i: i32 = n - 1 as i32;
    let mut cl: f64 = 0.;
    let mut ch: f64 = fasttwosum((*c.offset(i as isize))[0 as i32 as usize], *l, &raw mut cl);
    cl += (*c.offset(i as isize))[1 as i32 as usize];
    loop {
        i -= 1;
        if !(i >= 0 as i32) {
            break;
        }
        ch = mulddd(x, ch, cl, &raw mut cl);
        ch = fastsum(
            (*c.offset(i as isize))[0 as i32 as usize],
            (*c.offset(i as isize))[1 as i32 as usize],
            ch,
            cl,
            &raw mut cl,
        );
    }
    *l = cl;
    return ch;
}
#[inline]
unsafe extern "C" fn polyd(mut x: f64, mut n: i32, mut c: *const [f64; 2]) -> f64 {
    let mut i: i32 = n - 1 as i32;
    let mut ch: f64 = (*c.offset(i as isize))[0 as i32 as usize];
    loop {
        i -= 1;
        if !(i >= 0 as i32) {
            break;
        }
        ch = (*c.offset(i as isize))[0 as i32 as usize] + x * ch;
    }
    return ch;
}
#[inline(never)]
unsafe extern "C" fn as_lgamma_database(mut x: f64, mut f: f64) -> f64 {
    static mut db: [[f64; 3]; 19] = [
        [
            -8.85603270715682f64,
            -10.506245619347793f64,
            -4.440892098500626e-16f64,
        ],
        [
            -6.008620783070753f64,
            -1.841700176574477f64,
            5.551115123125783e-17f64,
        ],
        [
            -3.626316074670735f64,
            -1.4055111322744689f64,
            -5.551115123125783e-17f64,
        ],
        [
            -3.258905889183534f64,
            -0.662171512729061f64,
            2.7755575615628916e-17f64,
        ],
        [
            -3.1269625754058225f64,
            0.1370203577948594f64,
            -6.938893903907229e-18f64,
        ],
        [
            -2.917271760153873f64,
            0.8146556408417135f64,
            2.7755575615628916e-17f64,
        ],
        [
            -2.7874876471589888f64,
            0.0928190368890024f64,
            -3.469446951953614e-18f64,
        ],
        [
            -2.6971098422429526f64,
            -0.07456920522531016f64,
            3.469446951953614e-18f64,
        ],
        [
            -2.6610372434948985f64,
            -0.10426569273435452f64,
            -3.469446951953614e-18f64,
        ],
        [
            -2.651891020274131f64,
            -0.10909964663423262f64,
            3.469446951953614e-18f64,
        ],
        [
            -2.6343763295031938f64,
            -0.11553421597167577f64,
            3.469446951953614e-18f64,
        ],
        [
            -2.627412242635043f64,
            -0.11709799146343644f64,
            -3.469446951953614e-18f64,
        ],
        [
            -2.5947641728565555f64,
            -0.11726539617095948f64,
            3.469446951953614e-18f64,
        ],
        [
            -1.4590196197500394f64,
            0.8967586155061278f64,
            2.7755575615628916e-17f64,
        ],
        [
            -1.400340585925395f64,
            0.9774829289091477f64,
            2.7755575615628916e-17f64,
        ],
        [
            -1.317970956429359f64,
            1.1529580441928147f64,
            5.551115123125783e-17f64,
        ],
        [
            -1.3152477778717045f64,
            1.1601744875861138f64,
            -5.551115123125783e-17f64,
        ],
        [
            -0.2973583944297599f64,
            1.4704714463332447f64,
            -5.551115123125783e-17f64,
        ],
        [
            -0.28766719635854179f64,
            1.4919855103221743f64,
            5.551115123125783e-17f64,
        ],
    ];
    let mut a: i32 = 0 as i32;
    let mut b: i32 = (::core::mem::size_of::<[[f64; 3]; 19]>() as usize)
        .wrapping_div(::core::mem::size_of::<[f64; 3]>() as usize)
        .wrapping_sub(1 as usize) as i32;
    let mut m: i32 = (a + b) / 2 as i32;
    while a <= b {
        if db[m as usize][0 as i32 as usize] < x {
            a = m + 1 as i32;
        } else if db[m as usize][0 as i32 as usize] == x {
            f = db[m as usize][1 as i32 as usize] + db[m as usize][2 as i32 as usize];
            break;
        } else {
            b = m - 1 as i32;
        }
        m = (a + b) / 2 as i32;
    }
    return f;
}
#[inline(never)]
unsafe extern "C" fn as_lgamma_accurate(mut x: f64) -> f64 {
    static mut c0: [[f64; 2]; 34] = [
        [-0.5772156649015329f64, 4.9429151524306259e-18f64],
        [0.8224670334241132f64, 1.520336175199303e-17f64],
        [-0.40068563438653145f64, 2.2507470426642095e-18f64],
        [0.27058080842778456f64, 1.1871280105127509e-17f64],
        [-0.20738555102867399f64, -4.099767599495712e-18f64],
        [0.1695571769974082f64, 2.239386985427282e-18f64],
        [-0.1440498967688461f64, -9.622975771423665e-18f64],
        [0.12550966952474305f64, -2.5222724219598054e-18f64],
        [-0.11133426586956469f64, -4.696432977311065e-18f64],
        [0.1000994575127818f64, 2.8101016776923048e-18f64],
        [-0.09095401714582903f64, -4.5584601001204179e-18f64],
        [0.08335384054610898f64, -3.645490075700542e-18f64],
        [-0.07693251641135349f64, -6.947367368298923e-19f64],
        [0.07143294629536476f64, 4.571889960152985e-18f64],
        [-0.06666870588230577f64, -3.4366186193619733e-18f64],
        [0.06250095514094958f64, 1.5268985147216393e-18f64],
        [-0.058823978665988877f64, 3.028524825969812e-18f64],
        [0.05555576764222826f64, 3.230849795023974e-19f64],
        [-0.05263167903796568f64, 1.6234984715058276e-18f64],
        [0.05000004707683866f64, -2.925404479670744e-18f64],
        [-0.04761908220859991f64, -3.0870127688049247e-18f64],
        [0.04545457586103725f64, 8.29051239211358e-19f64],
        [-0.04347795804714225f64, 6.848859612533598e-19f64],
        [0.04166620534199169f64, 1.852558660954507e-19f64],
        [-0.04000592413210242f64, 1.0699454040195507e-18f64],
        [0.038469752676883458f64, -6.572684971201799e-19f64],
        [-0.03695397262772693f64, -1.0313900403933702e-18f64],
        [0.03560755138359309f64, 1.4325996414225666e-18f64],
        [-0.03530441627917072f64, -3.05388943812827e-18f64],
        [0.03431656175381843f64, -6.580439925243937e-19f64],
        [-0.02687987963167471f64, -9.873868071775923e-19f64],
        [0.02523361290587071f64, 1.5388848305953908e-18f64],
        [-0.05061761990065339f64, -3.0209318725927525e-18f64],
        [0.05069261759708643f64, 1.1871113981682286e-18f64],
    ];
    static mut b: [[f64; 2]; 30] = [
        [-0.12078223763524522f64, -4.179704749294626e-18f64],
        [0.03648997397857652f64, 1.9534229894801887e-19f64],
        [0.46740110027233969f64, -9.901065975283489e-18f64],
        [-0.13813277403905334f64, -2.748487779662932e-18f64],
        [0.05871212641676822f64, -2.3420398912200819e-18f64],
        [-0.028952081888893544f64, -1.1102875358773664e-19f64],
        [0.0154354841700493f64, -9.580861739264141e-21f64],
        [-0.008622603929171287f64, -6.456433651198196e-19f64],
        [0.004965728809475818f64, -2.590571167697595e-19f64],
        [-0.002920970458667952f64, -7.438489878919766e-20f64],
        [0.00174503557579013f64, -8.184090787275508e-20f64],
        [-0.001054915693867632f64, 8.941706151564488e-20f64],
        [0.0006437029830381538f64, -6.917738061417578e-21f64],
        [-0.00039577153964651357f64, -1.3437282670728619e-20f64],
        [0.0002448711904826001f64, -1.706015998477272e-22f64],
        [-0.0001523159381423555f64, 5.00164600858749e-21f64],
        [0.00009517939664099184f64, -1.369483803287959e-21f64],
        [-0.000059713623377073707f64, -3.3803460560282447e-21f64],
        [0.00003759490873808808f64, -1.4583136103548058e-21f64],
        [-0.000023743185012554187f64, -1.138206954765272e-21f64],
        [0.00001503699614442912f64, 4.735987397032349e-23f64],
        [-0.000009547161475467618f64, 6.778707283792083e-22f64],
        [0.000006075188910706963f64, 2.5960902181646655e-22f64],
        [-0.0000038739854785405079f64, 2.236533064846564e-22f64],
        [0.0000024777247790489758f64, -1.5650279539703273e-22f64],
        [-0.0000015859692955569145f64, 7.10274565435088e-23f64],
        [9.953388722273115e-7f64, -4.7886246178634037e-23f64],
        [-6.378638915030802e-7f64, 4.6761586189123e-23f64],
        [5.105500045608663e-7f64, -4.4779096079291117e-23f64],
        [-3.310863120221838e-7f64, 2.285602965163975e-23f64],
    ];
    let mut sx: f64 = x;
    let mut fh: f64 = 0.;
    let mut fl: f64 = 0 as i32 as f64;
    let mut fll: f64 = 0 as i32 as f64;
    x = x.abs();
    if x < 7.888609052210118e-31f64 {
        let mut lll: f64 = 0.;
        let mut ll: f64 = 0.;
        let mut lh: f64 = as_logd_accurate(x, &raw mut ll, &raw mut lll);
        fh = -lh;
        fl = -ll;
        fll = -lll;
        fh = fasttwosum(fh, fl, &raw mut fl);
        fl = fasttwosum(fl, fll, &raw mut fll);
        let mut e: f64 = 0.;
        fasttwosum(fh, 2 as i32 as f64 * fl, &raw mut e);
        if e == 0 as i32 as f64
            && 1 as i32 as f64 + 5.551115123125783e-17f64
                == 1 as i32 as f64 - 5.551115123125783e-17f64
        {
            fl *= 1 as i32 as f64
                + 2.220446049250313e-16f64.copysign(fl) * (1 as i32 as f64).copysign(fll);
        }
    } else if x < 0.25f64 {
        fh = polydddfst(
            sx,
            (::core::mem::size_of::<[[f64; 2]; 34]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize) as i32,
            &raw const c0 as *const [f64; 2],
            &raw mut fl,
        );
        fh = mulddd(sx, fh, fl, &raw mut fl);
        let mut lll_0: f64 = 0.;
        let mut ll_0: f64 = 0.;
        let mut lh_0: f64 = as_logd_accurate(x, &raw mut ll_0, &raw mut lll_0);
        fh = sumdd(fh, fl, -ll_0, -lll_0, &raw mut fl);
        fh = twosum(-lh_0, fh, &raw mut fll);
        fl = twosum(fll, fl, &raw mut fll);
        let mut e_0: f64 = 0.;
        fasttwosum(fh, 2 as i32 as f64 * fl, &raw mut e_0);
        if e_0 == 0 as i32 as f64
            && 1 as i32 as f64 + 5.551115123125783e-17f64
                == 1 as i32 as f64 - 5.551115123125783e-17f64
        {
            fl *= 1 as i32 as f64
                + 2.220446049250313e-16f64.copysign(fl) * (1 as i32 as f64).copysign(fll);
        }
    } else {
        if (x - 0.5f64).abs() < 0.25f64 {
            fh = polydddfst(
                x - 0.5f64,
                (::core::mem::size_of::<[[f64; 2]; 30]>() as usize)
                    .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                    as i32,
                &raw const b as *const [f64; 2],
                &raw mut fl,
            );
            if sx > 0 as i32 as f64 {
                let mut lll_1: f64 = 0.;
                let mut ll_1: f64 = 0.;
                let mut lh_1: f64 = as_logd_accurate(x, &raw mut ll_1, &raw mut lll_1);
                fl = sumdd(fl, 0 as i32 as f64, -ll_1, -lll_1, &raw mut fll);
                fh = twosum(fh, -lh_1, &raw mut lh_1);
                fl = sumdd(fl, fll, lh_1, 0 as i32 as f64, &raw mut fll);
            }
        } else if (x - 2.5f64).abs() < 0.25f64 {
            fh = polydddfst(
                x - 2.5f64,
                (::core::mem::size_of::<[[f64; 2]; 30]>() as usize)
                    .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                    as i32,
                &raw const b as *const [f64; 2],
                &raw mut fl,
            );
            let mut lll_2: f64 = 0.;
            let mut ll_2: f64 = 0.;
            let mut lh_2: f64 =
                as_logd_accurate(x - 1 as i32 as f64, &raw mut ll_2, &raw mut lll_2);
            fl = sumdd(fl, 0 as i32 as f64, ll_2, lll_2, &raw mut fll);
            fh = twosum(fh, lh_2, &raw mut lh_2);
            fl = sumdd(fl, fll, lh_2, 0 as i32 as f64, &raw mut fll);
            if sx < 0 as i32 as f64 {
                lh_2 = as_logd_accurate(x, &raw mut ll_2, &raw mut lll_2);
                fl = sumdd(fl, fll, ll_2, lll_2, &raw mut fll);
                fh = twosum(fh, lh_2, &raw mut lh_2);
                fl = sumdd(fl, fll, lh_2, 0 as i32 as f64, &raw mut fll);
            }
        } else if (x - 3.5f64).abs() < 0.25f64 {
            let mut l2ll: f64 = 0.;
            let mut l2l: f64 = 0.;
            let mut l2h: f64 = as_logd_accurate(x - 2 as i32 as f64, &raw mut l2l, &raw mut l2ll);
            let mut l1ll: f64 = 0.;
            let mut l1l: f64 = 0.;
            let mut l1h: f64 = as_logd_accurate(x - 1 as i32 as f64, &raw mut l1l, &raw mut l1ll);
            l1l = sumdd(l1l, l1ll, l2l, l2ll, &raw mut l1ll);
            l1h = fasttwosum(l1h, l2h, &raw mut l2h);
            l1l = sumdd(l1l, l1ll, l2h, 0 as i32 as f64, &raw mut l1ll);
            fh = polydddfst(
                x - 3.5f64,
                (::core::mem::size_of::<[[f64; 2]; 30]>() as usize)
                    .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                    as i32,
                &raw const b as *const [f64; 2],
                &raw mut fl,
            );
            fl = sumdd(fl, 0 as i32 as f64, l1l, l1ll, &raw mut fll);
            fh = twosum(fh, l1h, &raw mut l1h);
            fl = sumdd(fl, fll, l1h, 0 as i32 as f64, &raw mut fll);
            if sx < 0 as i32 as f64 {
                let mut lll_3: f64 = 0.;
                let mut ll_3: f64 = 0.;
                let mut lh_3: f64 = as_logd_accurate(x, &raw mut ll_3, &raw mut lll_3);
                fl = sumdd(fl, fll, ll_3, lll_3, &raw mut fll);
                fh = twosum(fh, lh_3, &raw mut lh_3);
                fl = sumdd(fl, fll, lh_3, 0 as i32 as f64, &raw mut fll);
            }
        } else if (x - 1 as i32 as f64).abs() < 0.25f64 {
            fh = polydddfst(
                x - 1 as i32 as f64,
                (::core::mem::size_of::<[[f64; 2]; 34]>() as usize)
                    .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                    as i32,
                &raw const c0 as *const [f64; 2],
                &raw mut fl,
            );
            fh = mulddd(x - 1 as i32 as f64, fh, fl, &raw mut fl);
            if sx < 0 as i32 as f64 {
                let mut lll_4: f64 = 0.;
                let mut ll_4: f64 = 0.;
                let mut lh_4: f64 = as_logd_accurate(x, &raw mut ll_4, &raw mut lll_4);
                fl = sumdd(fl, 0 as i32 as f64, ll_4, lll_4, &raw mut fll);
                fh = twosum(fh, lh_4, &raw mut lh_4);
                fl = sumdd(fl, fll, lh_4, 0 as i32 as f64, &raw mut fll);
            }
        } else if (x - 1.5f64).abs() < 0.25f64 {
            fh = polydddfst(
                x - 1.5f64,
                (::core::mem::size_of::<[[f64; 2]; 30]>() as usize)
                    .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                    as i32,
                &raw const b as *const [f64; 2],
                &raw mut fl,
            );
            if sx < 0 as i32 as f64 {
                let mut lll_5: f64 = 0.;
                let mut ll_5: f64 = 0.;
                let mut lh_5: f64 = as_logd_accurate(x, &raw mut ll_5, &raw mut lll_5);
                fl = sumdd(fl, 0 as i32 as f64, ll_5, lll_5, &raw mut fll);
                fh = twosum(fh, lh_5, &raw mut lh_5);
                fl = sumdd(fl, fll, lh_5, 0 as i32 as f64, &raw mut fll);
            }
        } else if (x - 2 as i32 as f64).abs() < 0.25f64 {
            let mut lll_6: f64 = 0.;
            let mut ll_6: f64 = 0.;
            let mut lh_6: f64 =
                as_logd_accurate(x - 1 as i32 as f64, &raw mut ll_6, &raw mut lll_6);
            fh = polydddfst(
                x - 2 as i32 as f64,
                (::core::mem::size_of::<[[f64; 2]; 34]>() as usize)
                    .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                    as i32,
                &raw const c0 as *const [f64; 2],
                &raw mut fl,
            );
            fh = mulddd(x - 2 as i32 as f64, fh, fl, &raw mut fl);
            fl = sumdd(fl, 0 as i32 as f64, ll_6, lll_6, &raw mut fll);
            fh = twosum(fh, lh_6, &raw mut lh_6);
            fl = sumdd(fl, fll, lh_6, 0 as i32 as f64, &raw mut fll);
            if sx < 0 as i32 as f64 {
                lh_6 = as_logd_accurate(x, &raw mut ll_6, &raw mut lll_6);
                fl = sumdd(fl, fll, ll_6, lll_6, &raw mut fll);
                fh = twosum(fh, lh_6, &raw mut lh_6);
                fl = sumdd(fl, fll, lh_6, 0 as i32 as f64, &raw mut fll);
            }
        } else if (x - 3 as i32 as f64).abs() < 0.25f64 {
            let mut l2ll_0: f64 = 0.;
            let mut l2l_0: f64 = 0.;
            let mut l2h_0: f64 =
                as_logd_accurate(x - 2 as i32 as f64, &raw mut l2l_0, &raw mut l2ll_0);
            let mut l1ll_0: f64 = 0.;
            let mut l1l_0: f64 = 0.;
            let mut l1h_0: f64 =
                as_logd_accurate(x - 1 as i32 as f64, &raw mut l1l_0, &raw mut l1ll_0);
            l1l_0 = sumdd(l1l_0, l1ll_0, l2l_0, l2ll_0, &raw mut l1ll_0);
            l1h_0 = fasttwosum(l1h_0, l2h_0, &raw mut l2h_0);
            l1l_0 = sumdd(l1l_0, l1ll_0, l2h_0, 0 as i32 as f64, &raw mut l1ll_0);
            fh = polydddfst(
                x - 3 as i32 as f64,
                (::core::mem::size_of::<[[f64; 2]; 34]>() as usize)
                    .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                    as i32,
                &raw const c0 as *const [f64; 2],
                &raw mut fl,
            );
            fh = mulddd(x - 3 as i32 as f64, fh, fl, &raw mut fl);
            fl = sumdd(fl, 0 as i32 as f64, l1l_0, l1ll_0, &raw mut fll);
            fh = twosum(fh, l1h_0, &raw mut l1h_0);
            fl = sumdd(fl, fll, l1h_0, 0 as i32 as f64, &raw mut fll);
            if sx < 0 as i32 as f64 {
                let mut lll_7: f64 = 0.;
                let mut ll_7: f64 = 0.;
                let mut lh_7: f64 = as_logd_accurate(x, &raw mut ll_7, &raw mut lll_7);
                fl = sumdd(fl, fll, ll_7, lll_7, &raw mut fll);
                fh = twosum(fh, lh_7, &raw mut lh_7);
                fl = sumdd(fl, fll, lh_7, 0 as i32 as f64, &raw mut fll);
            }
        } else {
            if sx < 0 as i32 as f64 {
                x = fasttwosum(x, 1 as i32 as f64, &raw mut fl);
            }
            fh = as_lgamma_asym_accurate(x, &raw mut fl, &raw mut fll);
        }
        if sx < 0 as i32 as f64 {
            let mut phi: f64 = if sx < -0.5f64 { sx - sx.floor() } else { -sx };
            let mut sl: f64 = 0.;
            let mut sh: f64 = as_sinpipid_accurate(phi, &raw mut sl);
            let mut lll_8: f64 = 0.;
            let mut ll_8: f64 = 0.;
            let mut lh_8: f64 = as_logd_accurate(sh, &raw mut ll_8, &raw mut lll_8);
            ll_8 += sl / sh + lll_8;
            fl = sumdd(fl, fll, ll_8, 0 as i32 as f64, &raw mut fll);
            fh = twosum(fh, lh_8, &raw mut lh_8);
            fl = sumdd(fl, fll, lh_8, 0 as i32 as f64, &raw mut fll);
            fh = -fh;
            fl = -fl;
            fll = -fll;
        }
        fh = fasttwosum(fh, fl, &raw mut fl);
        fl = fasttwosum(fl, fll, &raw mut fll);
        fh = fasttwosum(fh, fl, &raw mut fl);
        fl = fasttwosum(fl, fll, &raw mut fll);
        let mut e_1: f64 = 0.;
        fasttwosum(fh, 2 as i32 as f64 * fl, &raw mut e_1);
        if e_1 == 0 as i32 as f64 {
            let mut dfl: f64 = 1 as i32 as f64
                + 1.4901161193847657e-8f64.copysign(fl) * 1.4901161193847657e-8f64.copysign(fll);
            fl *= dfl;
        }
    }
    if fh.abs() < 0.375f64 {
        if fh.abs() < 0.0908203125f64 && sx > SX_BND && sx < -(2 as i32) as f64 {
            static mut x0: [f64; 3] = [
                2.4570247382208008f64,
                3.7075610815513268e-17f64,
                1.3622663121726005e-33f64,
            ];
            static mut c: [[f64; 2]; 26] = [
                [1.5156034480216574f64, -4.0695290379659218e-17f64],
                [0.6072901189542496f64, -4.3074644813891967e-17f64],
                [0.022051424110593438f64, -4.837388309683664e-19f64],
                [0.017034731609014348f64, 1.3115425807977466e-18f64],
                [0.001416116837548797f64, -6.953988273178545e-20f64],
                [0.0007575971106535056f64, -2.120668598357596e-20f64],
                [0.00009170083260457213f64, -4.671949182528769e-21f64],
                [0.000039209917462846058f64, 1.0677924033220743e-21f64],
                [0.000005999095651239848f64, -1.1090897487220746e-22f64],
                [0.000002208623390701512f64, -1.2716484589977599e-23f64],
                [3.9597958855738816e-7f64, -1.8640179122176506e-23f64],
                [1.3160261537150776e-7f64, 3.41708283819243e-24f64],
                [2.635650771559206e-8f64, 6.755545851990776e-25f64],
                [8.162202817970769e-9f64, 2.876807548318887e-25f64],
                [1.7680936484318848e-9f64, -8.819873899696363e-26f64],
                [5.214579523393903e-10f64, -1.168425867115639e-26f64],
                [1.1947938390510046e-10f64, -6.858863996402397e-27f64],
                [3.4070997143167078e-11f64, 9.51293946805372e-28f64],
                [8.128624659713758e-12f64, 2.228685440173242e-28f64],
                [2.265047058981486e-12f64, -9.695350184634743e-29f64],
                [5.563791399595165e-13f64, -3.8230580387152878e-29f64],
                [1.5269196554599644e-13f64, 7.116828772804166e-30f64],
                [3.856075733774979e-14f64, 7.620884555112168e-32f64],
                [1.0552565632279717e-14f64, -1.181886992341033e-31f64],
                [2.4507631609930725e-15f64, 6.242623321291812e-32f64],
                [3.790054564298429e-16f64, 1.8172654048332516e-32f64],
            ];
            let sc: f64 = 8.0f64;
            let mut zl: f64 = 0.;
            let mut zh: f64 = fasttwosum(
                x0[0 as i32 as usize] + sx,
                x0[1 as i32 as usize],
                &raw mut zl,
            );
            zl += x0[2 as i32 as usize];
            let mut sh_0: f64 = zh * sc;
            let mut sl_0: f64 = zl * sc;
            let mut n: i32 = (::core::mem::size_of::<[[f64; 2]; 26]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k: i32 = 6 as i32;
            fl = sh_0
                * polyd(
                    sh_0,
                    k,
                    (&raw const c as *const [f64; 2])
                        .offset(n as isize)
                        .offset(-(k as isize)),
                );
            fh = polydd(
                sh_0,
                sl_0,
                n - k,
                &raw const c as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh, zl, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.0679931640625f64 && sx > -(3 as i32) as f64 && sx < SX_BND {
            static mut x0_0: [f64; 3] = [
                2.7476826467274129f64,
                -9.055340329338315e-17f64,
                -3.322761057167369e-33f64,
            ];
            static mut c_0: [[f64; 2]; 27] = [
                [-1.9143501856115988f64, -6.2884735081868e-17f64],
                [0.5984493422318542f64, -3.083329620630319e-17f64],
                [-0.07849662076891642f64, 3.3703745180561114e-18f64],
                [0.015289863943728935f64, -4.1572115134162519e-19f64],
                [-0.0029718956802894018f64, 1.403186188053118e-19f64],
                [0.0006169376370553287f64, -1.5482522132150064e-20f64],
                [-0.00013071970799449824f64, 2.1630621218500295e-21f64],
                [0.000028350941787132356f64, 5.14091971308821e-22f64],
                [-0.0000062409272465431388f64, 4.370763980073041e-23f64],
                [0.0000013914170506664917f64, -7.523793619893019e-24f64],
                [-3.1331873600250536e-7f64, -7.23313057110596e-24f64],
                [7.114337259537829e-8f64, -2.8845781152861487e-25f64],
                [-1.6266869076914259e-8f64, -2.099459113511008e-25f64],
                [3.7415591226771839e-9f64, -2.4113998560538235e-25f64],
                [-8.650120180816276e-10f64, -5.080918251256956e-26f64],
                [2.0087521914502575e-10f64, -4.969384035430066e-27f64],
                [-4.683066253589359e-11f64, -8.92910664644629e-28f64],
                [1.0955686557097896e-11f64, -5.8921795986053919e-28f64],
                [-2.570938046278725e-12f64, 1.609830680180377e-28f64],
                [6.04992707960597e-13f64, -2.670163596630992e-29f64],
                [-1.4272077444932836e-13f64, 3.68905012998895e-30f64],
                [3.373718380574273e-14f64, -2.089410423436686e-32f64],
                [-7.992781435590999e-15f64, 6.492434148865956e-32f64],
                [1.9105713183272994e-15f64, -2.1456230412380554e-32f64],
                [-4.633091381266672e-16f64, -4.209783814308295e-32f64],
                [1.0323682816979496e-16f64, 1.796007014910912e-33f64],
                [-1.4205446216531043e-17f64, 1.4007882936844792e-33f64],
            ];
            let sc_0: f64 = 16.0f64;
            let mut zl_0: f64 = 0.;
            let mut zh_0: f64 = fasttwosum(
                x0_0[0 as i32 as usize] + sx,
                x0_0[1 as i32 as usize],
                &raw mut zl_0,
            );
            zl_0 += x0_0[2 as i32 as usize];
            let mut sh_1: f64 = zh_0 * sc_0;
            let mut sl_1: f64 = zl_0 * sc_0;
            let mut n_0: i32 = (::core::mem::size_of::<[[f64; 2]; 27]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_0: i32 = 6 as i32;
            fl = sh_1
                * polyd(
                    sh_1,
                    k_0,
                    (&raw const c_0 as *const [f64; 2])
                        .offset(n_0 as isize)
                        .offset(-(k_0 as isize)),
                );
            fh = polydd(
                sh_1,
                sl_1,
                n_0 - k_0,
                &raw const c_0 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_0, zl_0, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.07354736328125f64 && sx > -3.5f64 && sx < -3.0f64 {
            static mut x0_1: [f64; 3] = [
                3.14358088834998f64,
                2.1818179852331715e-16f64,
                1.1246581285745781e-32f64,
            ];
            static mut c_1: [[f64; 2]; 20] = [
                [7.781884658131351f64, -1.2366266971852828e-16f64],
                [0.40361466206856186f64, 2.256507132146391e-17f64],
                [0.027409420482708985f64, 7.975419769600283e-19f64],
                [0.0022464399041671808f64, -1.8057767552738618e-19f64],
                [0.00019533597144456754f64, 4.3633184751417107e-21f64],
                [0.000017716607064340598f64, 2.0445884095176994e-22f64],
                [0.0000016525106073405938f64, -4.460939574310149e-24f64],
                [1.5735386166165304e-7f64, 3.2026319941217019e-24f64],
                [1.5221166349185358e-8f64, -1.0042189592870042e-24f64],
                [1.4907811127549827e-9f64, 7.698936425273238e-26f64],
                [1.4748388779528427e-10f64, 2.5001036292875317e-27f64],
                [1.471226056467086e-11f64, -8.498821454768307e-28f64],
                [1.4778851683615746e-12f64, -4.63471378042177e-29f64],
                [1.4934112040528185e-13f64, -8.78344592987408e-30f64],
                [1.5168402827332707e-14f64, -1.5521111054456111e-30f64],
                [1.5475065091369044e-15f64, 1.3074833155625988e-32f64],
                [1.5847563188836993e-16f64, -1.2469189350031983e-33f64],
                [1.6298406549181094e-17f64, 5.372580253301777e-34f64],
                [1.7115133724165106e-18f64, -6.332784744513021e-37f64],
                [1.704172988805029e-19f64, -6.432045606612771e-36f64],
            ];
            let sc_1: f64 = 64.0f64;
            let mut zl_1: f64 = 0.;
            let mut zh_1: f64 = fasttwosum(
                x0_1[0 as i32 as usize] + sx,
                x0_1[1 as i32 as usize],
                &raw mut zl_1,
            );
            zl_1 += x0_1[2 as i32 as usize];
            let mut sh_2: f64 = zh_1 * sc_1;
            let mut sl_2: f64 = zl_1 * sc_1;
            let mut n_1: i32 = (::core::mem::size_of::<[[f64; 2]; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_1: i32 = 7 as i32;
            fl = sh_2
                * polyd(
                    sh_2,
                    k_1,
                    (&raw const c_1 as *const [f64; 2])
                        .offset(n_1 as isize)
                        .offset(-(k_1 as isize)),
                );
            fh = polydd(
                sh_2,
                sl_2,
                n_1 - k_1,
                &raw const c_1 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_1, zl_1, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.0604248046875f64 && sx > -4.0f64 && sx < -3.5f64 {
            static mut x0_2: [f64; 3] = [
                3.955294284858598f64,
                1.999428391746348e-17f64,
                -6.2357435447617e-34f64,
            ];
            static mut c_2: [[f64; 2]; 20] = [
                [-20.725060845803708f64, 1.4319348367658337e-15f64],
                [0.9832604788550368f64, -3.7619105715711256e-17f64],
                [-0.05692451043213825f64, 9.241152045463742e-19f64],
                [0.0037305403899411f64, 3.5072435760016824e-20f64],
                [-0.0002607683422196475f64, -5.544578466134934e-21f64],
                [0.000018987625232060353f64, 1.6002400683983847e-21f64],
                [-0.0000014220695667698055f64, -6.152651250528969e-23f64],
                [1.0872411559584736e-7f64, -4.705236462345527e-24f64],
                [-8.444430182049292e-9f64, 3.389020772660754e-25f64],
                [6.64063862190515e-10f64, -2.413953408092126e-26f64],
                [-5.274899017604428e-11f64, 5.625076199286637e-28f64],
                [4.224959762532295e-12f64, 3.7607528038906903e-28f64],
                [-3.407669431488513e-13f64, -1.5047590696341238e-30f64],
                [2.7648385202505816e-14f64, 3.6236228570802107e-31f64],
                [-2.254778028043172e-15f64, 1.5392860475884965e-32f64],
                [1.8470126579643077e-16f64, 7.302836062436853e-33f64],
                [-1.5187187178381525e-17f64, -1.4054819581013738e-33f64],
                [1.2541831270537938e-18f64, -5.095201529103335e-35f64],
                [-1.0568309504135398e-19f64, 3.7731467336318074e-36f64],
                [8.410102067924965e-21f64, 3.630106645159742e-37f64],
            ];
            let sc_2: f64 = 256.0f64;
            let mut zl_2: f64 = 0.;
            let mut zh_2: f64 = fasttwosum(
                x0_2[0 as i32 as usize] + sx,
                x0_2[1 as i32 as usize],
                &raw mut zl_2,
            );
            zl_2 += x0_2[2 as i32 as usize];
            let mut sh_3: f64 = zh_2 * sc_2;
            let mut sl_3: f64 = zl_2 * sc_2;
            let mut n_2: i32 = (::core::mem::size_of::<[[f64; 2]; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_2: i32 = 7 as i32;
            fl = sh_3
                * polyd(
                    sh_3,
                    k_2,
                    (&raw const c_2 as *const [f64; 2])
                        .offset(n_2 as isize)
                        .offset(-(k_2 as isize)),
                );
            fh = polydd(
                sh_3,
                sl_3,
                n_2 - k_2,
                &raw const c_2 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_2, zl_2, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.125f64 && sx > -4.5f64 && sx < -(4 as i32) as f64 {
            static mut x0_3: [f64; 3] = [
                4.039361839740537f64,
                -2.1143995503980603e-16f64,
                3.5961421111626579e-33f64,
            ];
            static mut c_3: [[f64; 2]; 24] = [
                [26.790480886140594f64, 7.293033628616712e-16f64],
                [2.533244710451931f64, 2.3411175432088176e-17f64],
                [0.3335993542954794f64, 7.58186373585564e-18f64],
                [0.04966047058174868f64, -3.8196836012969178e-19f64],
                [0.00788520804635262f64, 5.303546406617818e-19f64],
                [0.0013042071354038698f64, -1.0223976934791295e-19f64],
                [0.00022187809089205406f64, -1.2361449534246988e-20f64],
                [0.000038533380860729437f64, -1.1984268713601214e-21f64],
                [0.000006798282915751465f64, 1.5024922339243902e-22f64],
                [0.0000012143849745457687f64, 4.2463676422797219e-23f64],
                [2.1911814442565046e-7f64, -1.1469731733279574e-24f64],
                [3.986616153420021e-8f64, -2.6764404380875495e-24f64],
                [7.3039359722554468e-9f64, 2.8742658743722906e-25f64],
                [1.3461297292405428e-9f64, -4.889098543390422e-26f64],
                [2.4936662923238775e-10f64, 1.5438536531798623e-27f64],
                [4.6400667942026468e-11f64, 2.9982364172186464e-27f64],
                [8.667818322589215e-12f64, -4.3620814583640739e-29f64],
                [1.6248110030957372e-12f64, 3.714214853035293e-29f64],
                [3.0552686429248745e-13f64, -1.7436753511390364e-29f64],
                [5.759264468843753e-14f64, 4.009709929718271e-30f64],
                [1.0866895585875598e-14f64, -3.878390753409508e-32f64],
                [2.0784013178783435e-15f64, 8.766189976726427e-32f64],
                [4.1897212814403035e-16f64, 8.054788977405033e-33f64],
                [6.924875797675587e-17f64, 5.735491054752757e-33f64],
            ];
            let sc_3: f64 = 128.0f64;
            let mut zl_3: f64 = 0.;
            let mut zh_3: f64 = fasttwosum(
                x0_3[0 as i32 as usize] + sx,
                x0_3[1 as i32 as usize],
                &raw mut zl_3,
            );
            zl_3 += x0_3[2 as i32 as usize];
            let mut sh_4: f64 = zh_3 * sc_3;
            let mut sl_4: f64 = zl_3 * sc_3;
            let mut n_3: i32 = (::core::mem::size_of::<[[f64; 2]; 24]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_3: i32 = 7 as i32;
            fl = sh_4
                * polyd(
                    sh_4,
                    k_3,
                    (&raw const c_3 as *const [f64; 2])
                        .offset(n_3 as isize)
                        .offset(-(k_3 as isize)),
                );
            fh = polydd(
                sh_4,
                sl_4,
                n_3 - k_3,
                &raw const c_3 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_3, zl_3, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.064453125f64 && sx > -(5 as i32) as f64 && sx < -4.5f64 {
            static mut x0_4: [f64; 3] = [
                4.991544640560048f64,
                -1.5174411760571723e-16f64,
                9.643515906617392e-34f64,
            ];
            static mut c_4: [[f64; 2]; 19] = [
                [-116.53578161624363f64, -5.7160465473691099e-15f64],
                [6.831285112201234f64, -1.3524475919286833e-16f64],
                [-0.5258750895035633f64, 5.275150321219115e-17f64],
                [0.04555246113711879f64, -2.3627913076894659e-19f64],
                [-0.004208911533841501f64, -3.263127119848692e-19f64],
                [0.00040509466204792017f64, 1.3692533918386723e-20f64],
                [-0.00004010307734895336f64, -3.306868250031476e-22f64],
                [0.000004052786464342459f64, 4.419461745795722e-23f64],
                [-4.1607264964158546e-7f64, 1.9915579818382904e-23f64],
                [4.3249356200841037e-8f64, 2.3528581219743455e-24f64],
                [-4.541035854099499e-9f64, -2.0023747099086515e-25f64],
                [4.80766655338772e-10f64, -2.905937848980549e-26f64],
                [-5.1255468476023427e-11f64, 2.787883755079559e-27f64],
                [5.496974377736452e-12f64, -1.5995316420200649e-28f64],
                [-5.925523661828163e-13f64, -3.1663747398819279e-29f64],
                [6.415208096183801e-14f64, -8.75460216824021e-31f64],
                [-6.977680968187545e-15f64, 2.4023665115881827e-32f64],
                [7.742718367088825e-16f64, 5.2637429930545069e-33f64],
                [-8.160064683422863e-17f64, -2.1739570900993e-33f64],
            ];
            let sc_4: f64 = 1024.0f64;
            let mut zl_4: f64 = 0.;
            let mut zh_4: f64 = fasttwosum(
                x0_4[0 as i32 as usize] + sx,
                x0_4[1 as i32 as usize],
                &raw mut zl_4,
            );
            zl_4 += x0_4[2 as i32 as usize];
            let mut sh_5: f64 = zh_4 * sc_4;
            let mut sl_5: f64 = zl_4 * sc_4;
            let mut n_4: i32 = (::core::mem::size_of::<[[f64; 2]; 19]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_4: i32 = 7 as i32;
            fl = sh_5
                * polyd(
                    sh_5,
                    k_4,
                    (&raw const c_4 as *const [f64; 2])
                        .offset(n_4 as isize)
                        .offset(-(k_4 as isize)),
                );
            fh = polydd(
                sh_5,
                sl_5,
                n_4 - k_4,
                &raw const c_4 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_4, zl_4, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.06640625f64 && sx > -5.5f64 && sx < -(5 as i32) as f64 {
            static mut x0_5: [f64; 3] = [
                5.0082181683225939f64,
                4.3926353491015818e-17f64,
                2.68183947324466e-33f64,
            ];
            static mut c_5: [[f64; 2]; 19] = [
                [123.3621845633534f64, -1.7685712092821767e-15f64],
                [7.231213312723322f64, 1.903533401884996e-16f64],
                [0.5727349934127912f64, -3.781936594104917e-17f64],
                [0.051043449301883449f64, -3.4600513285941028e-18f64],
                [0.004852382283738175f64, -4.2087311844161887e-19f64],
                [0.0004805059542032575f64, -2.477145760816965e-20f64],
                [0.00004894146837471284f64, -9.016108198665899e-22f64],
                [0.0000050887376273369699f64, 3.912541968422674e-22f64],
                [5.375050491519447e-7f64, -4.229429558870215e-23f64],
                [5.748440875909129e-8f64, 2.2286681298759424e-24f64],
                [6.209868375409607e-9f64, -3.8322639603106506e-25f64],
                [6.764237453376503e-10f64, 3.3839859740250857e-26f64],
                [7.41962157879379e-11f64, 3.2239022406771886e-28f64],
                [8.186953174470213e-12f64, -1.3650348618254179e-28f64],
                [9.079928039262221e-13f64, 1.1373783503716913e-30f64],
                [1.011399835295972e-13f64, 2.7740321796840816e-30f64],
                [1.1318120763963064e-14f64, -2.13669057382987e-31f64],
                [1.292281577619718e-15f64, -7.712826814421688e-32f64],
                [1.4026394309569665e-16f64, 7.437555072977163e-33f64],
            ];
            let sc_5: f64 = 1024.0f64;
            let mut zl_5: f64 = 0.;
            let mut zh_5: f64 = fasttwosum(
                x0_5[0 as i32 as usize] + sx,
                x0_5[1 as i32 as usize],
                &raw mut zl_5,
            );
            zl_5 += x0_5[2 as i32 as usize];
            let mut sh_6: f64 = zh_5 * sc_5;
            let mut sl_6: f64 = zl_5 * sc_5;
            let mut n_5: i32 = (::core::mem::size_of::<[[f64; 2]; 19]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_5: i32 = 6 as i32;
            fl = sh_6
                * polyd(
                    sh_6,
                    k_5,
                    (&raw const c_5 as *const [f64; 2])
                        .offset(n_5 as isize)
                        .offset(-(k_5 as isize)),
                );
            fh = polydd(
                sh_6,
                sl_6,
                n_5 - k_5,
                &raw const c_5 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_5, zl_5, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.13427734375f64 && sx > -(6 as i32) as f64 && sx < -5.5f64 {
            static mut x0_6: [f64; 3] = [
                5.998607480080875f64,
                3.311862478893795e-16f64,
                -3.4720224807210339e-33f64,
            ];
            static mut c_6: [[f64; 2]; 24] = [
                [-716.2454304275473f64, -2.9783242920028197e-14f64],
                [62.95204825184634f64, -3.7266741215182267e-16f64],
                [-7.357912192215734f64, -1.301185578480272e-16f64],
                [0.9675078557046558f64, 5.309909763481054e-18f64],
                [-0.13570102335383622f64, 1.7511206991014248e-20f64],
                [0.019826247007162926f64, 4.216454541733531e-19f64],
                [-0.002979424322311712f64, -5.96101133349189e-20f64],
                [0.000457066138103023f64, -7.168645134063293e-21f64],
                [-0.00007123036356387229f64, -6.7539744212739709e-21f64],
                [0.000011239482262743443f64, -3.221812993473791e-23f64],
                [-0.0000017913987092539257f64, 8.685506856657265e-23f64],
                [2.8790044217739529e-7f64, 3.831654601133958e-24f64],
                [-4.659280549825673e-8f64, -1.704854011624271e-25f64],
                [7.585300919040004e-9f64, 1.689813661567136e-25f64],
                [-1.2412184680242752e-9f64, -7.92964926099965e-26f64],
                [2.0401310762397542e-10f64, -2.849048324563446e-27f64],
                [-3.366413980063065e-11f64, -2.684827812168952e-27f64],
                [5.574268997291247e-12f64, -2.134254406293707e-28f64],
                [-9.25904841263582e-13f64, -7.036474671307364e-29f64],
                [1.541251322092213e-13f64, 2.8725659432027428e-31f64],
                [-2.567061136654444e-14f64, 8.72687756437199e-31f64],
                [4.364365205645974e-15f64, -3.0249388364386538e-31f64],
                [-7.842884852020418e-16f64, 4.669086863352322e-32f64],
                [1.0932623670278008e-16f64, -3.364496070678645e-35f64],
            ];
            let sc_6: f64 = 4096.0f64;
            let mut zl_6: f64 = 0.;
            let mut zh_6: f64 = fasttwosum(
                x0_6[0 as i32 as usize] + sx,
                x0_6[1 as i32 as usize],
                &raw mut zl_6,
            );
            zl_6 += x0_6[2 as i32 as usize];
            let mut sh_7: f64 = zh_6 * sc_6;
            let mut sl_7: f64 = zl_6 * sc_6;
            let mut n_6: i32 = (::core::mem::size_of::<[[f64; 2]; 24]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_6: i32 = 6 as i32;
            fl = sh_7
                * polyd(
                    sh_7,
                    k_6,
                    (&raw const c_6 as *const [f64; 2])
                        .offset(n_6 as isize)
                        .offset(-(k_6 as isize)),
                );
            fh = polydd(
                sh_7,
                sl_7,
                n_6 - k_6,
                &raw const c_6 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_6, zl_6, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.13525390625f64 && sx > -6.5f64 && sx < -(6 as i32) as f64 {
            static mut x0_7: [f64; 3] = [
                6.001385294453155f64,
                -6.415847287933042e-17f64,
                1.116080967205309e-33f64,
            ];
            static mut c_7: [[f64; 2]; 24] = [
                [723.7366299252801f64, 1.1249184125179747e-14f64],
                [63.6104517341538f64, 2.681202256944616e-15f64],
                [7.4736466968598169f64, 1.8026093030895137e-16f64],
                [0.9878517730797606f64, 4.7546247784509438e-17f64],
                [0.1392771038595229f64, 4.635072743819901e-18f64],
                [0.020454856551140545f64, 1.4030701871665997e-18f64],
                [0.00308992263164616f64, -1.9570343460122387e-19f64],
                [0.0004764898074029671f64, -1.3471228351626052e-21f64],
                [0.00007464471245439858f64, 2.643818290246005e-21f64],
                [0.000011839667760923355f64, 5.325049758116113e-22f64],
                [0.0000018969015316306986f64, -5.707583767191434e-23f64],
                [3.064461595639485e-7f64, -5.1145273124492529e-25f64],
                [4.985285486161596e-8f64, -2.6376109172686045e-25f64],
                [8.158368307748725e-9f64, -2.916920307355771e-25f64],
                [1.341955278225707e-9f64, -8.842804161006764e-26f64],
                [2.217211921595561e-10f64, 1.3694814497269498e-27f64],
                [3.67769720406255e-11f64, 2.7608262308088155e-27f64],
                [6.121470476838045e-12f64, 2.960537291353879e-28f64],
                [1.0221012514613567e-12f64, 3.631968267463486e-29f64],
                [1.7102446929733749e-13f64, 2.4399098048730309e-30f64],
                [2.863298486498948e-14f64, 2.8151415743266857e-30f64],
                [4.8936909104695728e-15f64, 2.374620544305254e-32f64],
                [8.843727075487546e-16f64, 1.4182421346264729e-32f64],
                [1.2395023698286619e-16f64, 5.365418727542277e-33f64],
            ];
            let sc_7: f64 = 4096.0f64;
            let mut zl_7: f64 = 0.;
            let mut zh_7: f64 = fasttwosum(
                x0_7[0 as i32 as usize] + sx,
                x0_7[1 as i32 as usize],
                &raw mut zl_7,
            );
            zl_7 += x0_7[2 as i32 as usize];
            let mut sh_8: f64 = zh_7 * sc_7;
            let mut sl_8: f64 = zl_7 * sc_7;
            let mut n_7: i32 = (::core::mem::size_of::<[[f64; 2]; 24]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_7: i32 = 6 as i32;
            fl = sh_8
                * polyd(
                    sh_8,
                    k_7,
                    (&raw const c_7 as *const [f64; 2])
                        .offset(n_7 as isize)
                        .offset(-(k_7 as isize)),
                );
            fh = polydd(
                sh_8,
                sl_8,
                n_7 - k_7,
                &raw const c_7 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_7, zl_7, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.150390625f64 && sx > -7.0f64 && sx < -6.5f64 {
            static mut x0_8: [f64; 3] = [
                6.999801507890638f64,
                -1.0550130037400023e-17f64,
                4.08696427365735e-34f64,
            ];
            static mut c_8: [[f64; 2]; 25] = [
                [-5035.967373768125f64, -3.431412146497793e-13f64],
                [774.5752664823718f64, -5.0509481400309396e-14f64],
                [-158.7848729072441f64, 4.529383312973554e-15f64],
                [36.61906092576687f64, 2.3967009284084728e-15f64],
                [-9.008116695471731f64, -4.2457391918122758e-16f64],
                [2.3082857541656507f64, 1.7032356423138812e-16f64],
                [-0.608386537571166f64, -3.1782704103747489e-17f64],
                [0.16369087195862024f64, 1.3648172113319614e-18f64],
                [-0.04474131593124671f64, 1.811722684116638e-18f64],
                [0.012381922363164773f64, -1.4358724420111935e-20f64],
                [-0.00346124391424406f64, 1.2070908818892667e-19f64],
                [0.000975619468301449f64, 7.738408708878204e-21f64],
                [-0.00027692053720603936f64, -2.5509749049876857e-20f64],
                [0.00007906919108830608f64, 1.391195509938253e-21f64],
                [-0.000022692423968453749f64, -1.6444453704883558e-21f64],
                [0.000006541675261743044f64, 3.90551143062539e-22f64],
                [-0.0000018932004697454583f64, 2.744366396246963e-23f64],
                [5.498059553915429e-7f64, 2.410128717886692e-23f64],
                [-1.60169496740767e-7f64, -1.677561657052989e-24f64],
                [4.679214266476125e-8f64, -1.4268235597711503e-24f64],
                [-1.368627655785664e-8f64, -2.6475280527490526e-25f64],
                [4.003237855030896e-9f64, -5.948068212719232e-26f64],
                [-1.2078287971765483e-9f64, -1.7597358933435724e-26f64],
                [3.855497962061076e-10f64, -1.5571221835996333e-26f64],
                [-8.981108063245766e-11f64, 4.0264874069687739e-27f64],
            ];
            let sc_8: f64 = 16384.0f64;
            let mut zl_8: f64 = 0.;
            let mut zh_8: f64 = fasttwosum(
                x0_8[0 as i32 as usize] + sx,
                x0_8[1 as i32 as usize],
                &raw mut zl_8,
            );
            zl_8 += x0_8[2 as i32 as usize];
            let mut sh_9: f64 = zh_8 * sc_8;
            let mut sl_9: f64 = zl_8 * sc_8;
            let mut n_8: i32 = (::core::mem::size_of::<[[f64; 2]; 25]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_8: i32 = 6 as i32;
            fl = sh_9
                * polyd(
                    sh_9,
                    k_8,
                    (&raw const c_8 as *const [f64; 2])
                        .offset(n_8 as isize)
                        .offset(-(k_8 as isize)),
                );
            fh = polydd(
                sh_9,
                sl_9,
                n_8 - k_8,
                &raw const c_8 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_8, zl_8, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.134918212890625f64 && sx > -7.5f64 && sx < -(7 as i32) as f64 {
            static mut x0_9: [f64; 3] = [
                7.000198333407325f64,
                -2.504354173632409e-16f64,
                -2.413795840298293e-32f64,
            ];
            static mut c_9: [[f64; 2]; 24] = [
                [5044.029941110829f64, 2.1055091809068127e-13f64],
                [387.9076792587706f64, -2.7725482949085594e-14f64],
                [39.791586624345928f64, -1.829735696471361e-15f64],
                [4.59205109431431f64, 7.668544076099298e-17f64],
                [0.5652634315721323f64, 1.3579873966825307e-17f64],
                [0.07248094327162517f64, 2.0749312105075068e-18f64],
                [0.00955941330755672f64, -2.760784939427283e-19f64],
                [0.0012870442659175482f64, -4.067247824799071e-20f64],
                [0.00017603343095748313f64, -2.8782824025862603e-21f64],
                [0.000024377650991265168f64, -1.509744812747179e-21f64],
                [0.0000034099920141579968f64, 2.1136869855613546e-22f64],
                [4.809711124425511e-7f64, -4.2388406023222007e-23f64],
                [6.831421446039519e-8f64, 1.2477867339103794e-24f64],
                [9.760691533717503e-9f64, -3.846302187941853e-25f64],
                [1.4017532075440515e-9f64, 6.872432900634605e-26f64],
                [2.0220735861252509e-10f64, -8.981842754182204e-27f64],
                [2.9283430901229168e-11f64, -1.2978328951892703e-27f64],
                [4.2555723342883965e-12f64, -9.318464281922794e-29f64],
                [6.203720843965529e-13f64, -2.584611038221688e-29f64],
                [9.063019242359053e-14f64, 1.2817688662933525e-31f64],
                [1.324773459531929e-14f64, 1.9799651694594507e-31f64],
                [1.9768372863842298e-15f64, 1.3640478648992975e-31f64],
                [3.118666036714592e-16f64, -9.366087282510196e-33f64],
                [3.8151449342516028e-17f64, 2.5561419702150695e-33f64],
            ];
            let sc_9: f64 = 32768.0f64;
            let mut zl_9: f64 = 0.;
            let mut zh_9: f64 = fasttwosum(
                x0_9[0 as i32 as usize] + sx,
                x0_9[1 as i32 as usize],
                &raw mut zl_9,
            );
            zl_9 += x0_9[2 as i32 as usize];
            let mut sh_10: f64 = zh_9 * sc_9;
            let mut sl_10: f64 = zl_9 * sc_9;
            let mut n_9: i32 = (::core::mem::size_of::<[[f64; 2]; 24]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_9: i32 = 6 as i32;
            fl = sh_10
                * polyd(
                    sh_10,
                    k_9,
                    (&raw const c_9 as *const [f64; 2])
                        .offset(n_9 as isize)
                        .offset(-(k_9 as isize)),
                );
            fh = polydd(
                sh_10,
                sl_10,
                n_9 - k_9,
                &raw const c_9 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_9, zl_9, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.182952880859375f64 && sx > -(8 as i32) as f64 && sx < -7.5f64 {
            static mut x0_10: [f64; 3] = [
                7.999975197095821f64,
                5.261737128572354e-17f64,
                2.0441803623138535e-33f64,
            ];
            static mut c_10: [[f64; 2]; 27] = [
                [-40315.71854218779f64, -1.3528231836250762e-12f64],
                [6200.904001888152f64, 2.639488827845725e-13f64],
                [-1271.6022458880412f64, 7.557031635553495e-14f64],
                [293.3594535106741f64, -4.9821398731548809e-15f64],
                [-72.19009497023368f64, 9.90069575057737e-16f64],
                [18.50477911088001f64, 4.47943629704666e-16f64],
                [-4.878930750427179f64, 2.0058239243793183e-16f64],
                [1.3131678613904015f64, 6.780715253006352e-18f64],
                [-0.3590502727822438f64, -1.9751210850498927e-17f64],
                [0.09939975379584011f64, -4.001703513901494e-18f64],
                [-0.0277958629556318f64, 2.96000361980726e-19f64],
                [0.00783752860891683f64, 7.273900171449109e-19f64],
                [-0.0022253820008886605f64, -6.035792908967389e-20f64],
                [0.0006356344484780356f64, -1.2703595350802793e-20f64],
                [-0.0001824869195934846f64, -2.6806489235833667e-21f64],
                [0.0000526248119996751f64, 1.764222894456204e-21f64],
                [-0.0000152352364251781f64, -6.1471240966605029e-22f64],
                [0.000004426016624911302f64, -3.2617542819357125e-22f64],
                [-0.000001289784802510836f64, -1.6101169173933263e-23f64],
                [3.7690428149098628e-7f64, 2.3287640302213267e-24f64],
                [-1.1043582909930043e-7f64, -5.222344914664483e-24f64],
                [3.242602086598796e-8f64, -7.166863435724572e-25f64],
                [-9.499419205568867e-9f64, 6.493605261214992e-25f64],
                [2.7887915548114007e-9f64, 4.175189529146613e-26f64],
                [-8.718138671255657e-10f64, -4.3260269885347187e-26f64],
                [2.823036452517662e-10f64, -1.5189442604325848e-27f64],
                [-5.85197626650742e-11f64, -1.5283633977615298e-27f64],
            ];
            let sc_10: f64 = 131072.0f64;
            let mut zl_10: f64 = 0.;
            let mut zh_10: f64 = fasttwosum(
                x0_10[0 as i32 as usize] + sx,
                x0_10[1 as i32 as usize],
                &raw mut zl_10,
            );
            zl_10 += x0_10[2 as i32 as usize];
            let mut sh_11: f64 = zh_10 * sc_10;
            let mut sl_11: f64 = zl_10 * sc_10;
            let mut n_10: i32 = (::core::mem::size_of::<[[f64; 2]; 27]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_10: i32 = 6 as i32;
            fl = sh_11
                * polyd(
                    sh_11,
                    k_10,
                    (&raw const c_10 as *const [f64; 2])
                        .offset(n_10 as isize)
                        .offset(-(k_10 as isize)),
                );
            fh = polydd(
                sh_11,
                sl_11,
                n_10 - k_10,
                &raw const c_10 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_10, zl_10, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.1829833984375f64 && sx > -8.5f64 && sx < -(8 as i32) as f64 {
            static mut x0_11: [f64; 3] = [
                8.000024800270682f64,
                4.354586297860107e-16f64,
                -2.3599860861934563e-32f64,
            ];
            static mut c_11: [[f64; 2]; 27] = [
                [40324.281108124356f64, -2.389812710483788e-12f64],
                [6202.220998111847f64, 4.451377843373119e-13f64],
                [1272.0073765617354f64, 7.469498358597566e-14f64],
                [293.48407866902297f64, 2.749933991570724e-15f64],
                [72.22843181095333f64, -5.655976506333277e-15f64],
                [18.516572182016018f64, -3.647029626717219e-17f64],
                [4.882558501810415f64, -2.4904122907471048e-16f64],
                [1.3142838200714913f64, 4.6720437078555648e-17f64],
                [0.3593935608545259f64, 2.164998564502465e-18f64],
                [0.09950535510767972f64, -3.891477028880026e-18f64],
                [0.027828347734373424f64, -2.423337891518839e-19f64],
                [0.007847521485259956f64, -7.156572903409503e-19f64],
                [0.00222845598142886f64, -6.347662026567574e-20f64],
                [0.0006365800577345467f64, 1.8812700877104714e-20f64],
                [0.00018277780525577288f64, -1.282084620370295e-20f64],
                [0.00005271429343060698f64, 2.6947367423299789e-21f64],
                [0.000015262762455254628f64, 1.1938057817849946e-21f64],
                [0.00000443448410240734f64, -5.60541261753958e-23f64],
                [0.0000012923895270217234f64, 7.938079214446736e-23f64],
                [3.777055445398687e-7f64, 2.3293151256063094e-23f64],
                [1.1068236611677656e-7f64, -1.1943764773898882e-24f64],
                [3.2501862145323729e-8f64, 2.0732400586997958e-24f64],
                [9.52263619370976e-9f64, -1.3706752187473134e-25f64],
                [2.7958982163615997e-9f64, -7.165919843989772e-26f64],
                [8.741363975793739e-10f64, -3.698617114457722e-26f64],
                [2.83090799305424e-10f64, 4.194983161617278e-27f64],
                [5.868901840510094e-11f64, 1.3721499461616776e-27f64],
            ];
            let sc_11: f64 = 131072.0f64;
            let mut zl_11: f64 = 0.;
            let mut zh_11: f64 = fasttwosum(
                x0_11[0 as i32 as usize] + sx,
                x0_11[1 as i32 as usize],
                &raw mut zl_11,
            );
            zl_11 += x0_11[2 as i32 as usize];
            let mut sh_12: f64 = zh_11 * sc_11;
            let mut sl_12: f64 = zl_11 * sc_11;
            let mut n_11: i32 = (::core::mem::size_of::<[[f64; 2]; 27]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_11: i32 = 6 as i32;
            fl = sh_12
                * polyd(
                    sh_12,
                    k_11,
                    (&raw const c_11 as *const [f64; 2])
                        .offset(n_11 as isize)
                        .offset(-(k_11 as isize)),
                );
            fh = polydd(
                sh_12,
                sl_12,
                n_11 - k_11,
                &raw const c_11 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_11, zl_11, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.19970703125f64 && sx > -(9 as i32) as f64 && sx < -8.5f64 {
            static mut x0_12: [f64; 3] = [
                8.999997244250978f64,
                2.2185620509727133e-16f64,
                -7.336677520259467e-33f64,
            ];
            static mut c_12: [[f64; 2]; 28] = [
                [-362875.4964746711f64, 1.3486075072381975e-11f64],
                [62790.04104997679f64, 2.190644882047882e-12f64],
                [-14486.381980459091f64, -7.367721847453442e-13f64],
                [3759.9461125059767f64, -1.7072519000098248e-13f64],
                [-1040.955184017187f64, -4.8056864515883558e-14f64],
                [300.2003615623997f64, 1.8820675266197668e-14f64],
                [-89.04813866485125f64, -2.4250963514779959e-15f64],
                [26.96455911435853f64, -6.144072976007356e-16f64],
                [-8.294710365800576f64, 8.748005141145044e-16f64],
                [2.5834743860537f64, 1.924111783189809e-16f64],
                [-0.8127779248294899f64, -1.5230663055025275e-17f64],
                [0.257836123701557f64, 3.5682345861612259e-18f64],
                [-0.0823648822343464f64, -3.0140144234031494e-18f64],
                [0.02646779871017542f64, -2.1380918520665438e-19f64],
                [-0.008548994235039208f64, 2.9490941911054229e-19f64],
                [0.0027736184870452206f64, -4.804471903588051e-20f64],
                [-0.0009033961242275431f64, 3.9109164850883897e-20f64],
                [0.0002952671880244623f64, 3.2047919478538049e-21f64],
                [-0.00009680414937226026f64, -4.158881667259764e-21f64],
                [0.000031825300015837007f64, -2.099603552892237e-21f64],
                [-0.00001048954483424287f64, 1.10947939781969e-22f64],
                [0.000003466395429015234f64, 1.6804344816357396e-22f64],
                [-0.0000011470787231792585f64, 1.0068233506201023e-22f64],
                [3.7764748783223968e-7f64, 6.781285530116278e-24f64],
                [-1.2518610379679313e-7f64, -9.491454606014991e-24f64],
                [4.510238464081823e-8f64, -2.4337598127819865e-24f64],
                [-1.6396127383601279e-8f64, -7.308356538005303e-25f64],
                [3.573207831532966e-9f64, 9.885703851678309e-26f64],
            ];
            let sc_12: f64 = 1048576.0f64;
            let mut zl_12: f64 = 0.;
            let mut zh_12: f64 = fasttwosum(
                x0_12[0 as i32 as usize] + sx,
                x0_12[1 as i32 as usize],
                &raw mut zl_12,
            );
            zl_12 += x0_12[2 as i32 as usize];
            let mut sh_13: f64 = zh_12 * sc_12;
            let mut sl_13: f64 = zl_12 * sc_12;
            let mut n_12: i32 = (::core::mem::size_of::<[[f64; 2]; 28]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_12: i32 = 6 as i32;
            fl = sh_13
                * polyd(
                    sh_13,
                    k_12,
                    (&raw const c_12 as *const [f64; 2])
                        .offset(n_12 as isize)
                        .offset(-(k_12 as isize)),
                );
            fh = polydd(
                sh_13,
                sl_13,
                n_12 - k_12,
                &raw const c_12 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_12, zl_12, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.19970703125f64 && sx > -9.5f64 && sx < -9.0f64 {
            static mut x0_13: [f64; 3] = [
                9.000002755714823f64,
                9.491348611623208e-17f64,
                5.762352109706189e-33f64,
            ];
            static mut c_13: [[f64; 2]; 28] = [
                [362884.5034850277f64, 5.41958198120839e-12f64],
                [62791.59957502321f64, -2.1910005014723966e-12f64],
                [14486.921338186938f64, -3.421064946094347e-13f64],
                [3760.1327676766838f64, -1.5379136739528892e-13f64],
                [1041.0197796481635f64, 1.046409231042842e-13f64],
                [300.2227161295166f64, 2.0641941900729106e-14f64],
                [89.05587489504883f64, -4.554288423742391e-15f64],
                [26.967236386405785f64, 9.245298751586884e-16f64],
                [8.295636887560147f64, 7.724364494307896e-16f64],
                [2.5837950268237899f64, 6.17068774616263e-17f64],
                [0.8128888887678786f64, 4.360170160899399e-17f64],
                [0.2578745249180319f64, 1.4573159837728157e-17f64],
                [0.0823781717178325f64, -3.6691482778964237e-18f64],
                [0.02647239779290105f64, 1.3385236264750828e-18f64],
                [0.00855058583654439f64, -8.107546403770367e-19f64],
                [0.002774169291520058f64, 1.614308849673973e-19f64],
                [0.0009035867407663032f64, 4.572540999663984e-20f64],
                [0.00029533315457470975f64, -1.7814252278406094e-20f64],
                [0.00009682697836908494f64, 6.6104061892001708e-21f64],
                [0.00003183320036357145f64, 2.7448733516952534e-21f64],
                [0.000010492278915845768f64, 5.028874407601408e-22f64],
                [0.000003467341880843386f64, -3.7948753491829935e-23f64],
                [0.0000011474062724535365f64, -2.248829794781347e-23f64],
                [3.7776016503626328e-7f64, 1.4541323562629138e-23f64],
                [1.2522489622644035e-7f64, -1.245287144361461e-23f64],
                [4.51167845096728e-8f64, -1.3071469582685434e-24f64],
                [1.6401627260518798e-8f64, 6.022812089107567e-25f64],
                [3.574498129774545e-9f64, -1.4470430689764528e-25f64],
            ];
            let sc_13: f64 = 1048576.0f64;
            let mut zl_13: f64 = 0.;
            let mut zh_13: f64 = fasttwosum(
                x0_13[0 as i32 as usize] + sx,
                x0_13[1 as i32 as usize],
                &raw mut zl_13,
            );
            zl_13 += x0_13[2 as i32 as usize];
            let mut sh_14: f64 = zh_13 * sc_13;
            let mut sl_14: f64 = zl_13 * sc_13;
            let mut n_13: i32 = (::core::mem::size_of::<[[f64; 2]; 28]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_13: i32 = 6 as i32;
            fl = sh_14
                * polyd(
                    sh_14,
                    k_13,
                    (&raw const c_13 as *const [f64; 2])
                        .offset(n_13 as isize)
                        .offset(-(k_13 as isize)),
                );
            fh = polydd(
                sh_14,
                sl_14,
                n_13 - k_13,
                &raw const c_13 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_13, zl_13, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.182952880859375f64 && sx > -10.0f64 && sx < -9.5f64 {
            static mut x0_14: [f64; 3] = [
                9.99999972442663f64,
                -4.883037618642443e-16f64,
                -3.548028340923709e-32f64,
            ];
            static mut c_14: [[f64; 2]; 27] = [
                [-3628795.296492739f64, -1.267174497055282e-10f64],
                [392442.1182847145f64, -1.699185333380073e-11f64],
                [-56588.37302144316f64, -3.3079179747484954e-12f64],
                [9179.759991390652f64, 4.898066853028483e-13f64],
                [-1588.4156927215118f64, -2.222156798788695e-14f64],
                [286.30292068845008f64, -4.376216261552064e-15f64],
                [-53.07889147896094f64, -4.75197458501171e-16f64],
                [10.0455276369496f64, 7.931605069720773e-16f64],
                [-1.931359347468218f64, 3.4162936954954967e-17f64],
                [0.37596589518844378f64, 9.25649979029307e-18f64],
                [-0.07392624054608163f64, -4.010325951178242e-19f64],
                [0.014657264184018701f64, -8.416460386218347e-19f64],
                [-0.0029263998435773497f64, 2.0082254359773014e-19f64],
                [0.0005877489162449598f64, -1.986660178376471e-20f64],
                [-0.00011865101796239036f64, 8.78326654676142e-23f64],
                [0.000024059444889599057f64, -1.6487223020189808e-21f64],
                [-0.000004897783241577327f64, 7.802069124498651e-23f64],
                [0.000001000503750570806f64, 7.394630454589685e-23f64],
                [-2.0501151434512164e-7f64, 8.401435835103403e-24f64],
                [4.21257244519922e-8f64, -2.8968033965576595e-24f64],
                [-8.679238009890648e-9f64, -7.3866335644776849e-25f64],
                [1.7919283761501609e-9f64, 2.5790419258993808e-27f64],
                [-3.691304330521188e-10f64, -1.1316473267408382e-26f64],
                [7.619973221424454e-11f64, -6.059905867173142e-27f64],
                [-1.6749889586610876e-11f64, 3.8392999040979087e-28f64],
                [3.8138516541753926e-12f64, 1.8541359251837476e-28f64],
                [-5.559377744453847e-13f64, -2.645108176561418e-29f64],
            ];
            let sc_14: f64 = 16777216.0f64;
            let mut zl_14: f64 = 0.;
            let mut zh_14: f64 = fasttwosum(
                x0_14[0 as i32 as usize] + sx,
                x0_14[1 as i32 as usize],
                &raw mut zl_14,
            );
            zl_14 += x0_14[2 as i32 as usize];
            let mut sh_15: f64 = zh_14 * sc_14;
            let mut sl_15: f64 = zl_14 * sc_14;
            let mut n_14: i32 = (::core::mem::size_of::<[[f64; 2]; 27]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_14: i32 = 6 as i32;
            fl = sh_15
                * polyd(
                    sh_15,
                    k_14,
                    (&raw const c_14 as *const [f64; 2])
                        .offset(n_14 as isize)
                        .offset(-(k_14 as isize)),
                );
            fh = polydd(
                sh_15,
                sl_15,
                n_14 - k_14,
                &raw const c_14 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_14, zl_14, fh, fl, &raw mut fl);
        } else if fh.abs() < 0.182952880859375f64 && sx > -10.5f64 && sx < -10.0f64 {
            static mut x0_15: [f64; 3] = [
                10.000000275573014f64,
                3.4909708332642059e-16f64,
                1.2687206116063324e-32f64,
            ];
            static mut c_15: [[f64; 2]; 27] = [
                [3628804.7035030957f64, -3.276412312089428e-11f64],
                [392443.1356215355f64, 1.6991853081897849e-11f64],
                [56588.59306461696f64, -2.0847224676270087e-12f64],
                [9179.807585262548f64, -7.50634251228206e-13f64],
                [1588.4259869587875f64, 9.631744629859817e-15f64],
                [286.3051472633613f64, 2.0835890620795287e-14f64],
                [53.07937307227404f64, -2.956221102622246e-15f64],
                [10.045631802373386f64, -4.665519780403634e-16f64],
                [1.931381877755116f64, -8.265809366326719e-17f64],
                [0.3759707683393345f64, 1.8684364483350375e-17f64],
                [0.07392729457613956f64, 5.220038326295607e-18f64],
                [0.01465749216369508f64, 3.6845193002859339e-19f64],
                [0.002926449154061857f64, 1.2198179136028978e-19f64],
                [0.0005877595817741028f64, 4.301274774796546e-20f64],
                [0.00011865332484525698f64, -1.3505198806751253e-21f64],
                [0.00002405994385298689f64, 9.304869515554322e-22f64],
                [0.00000489789116403735f64, -3.0730101114116945e-22f64],
                [0.0000010005270934806325f64, 4.478268038550523e-23f64],
                [2.0501656324546702e-7f64, 1.7750849838543735e-25f64],
                [4.212681649310639e-8f64, 9.510425915846602e-25f64],
                [8.679474241777709e-9f64, -4.130491100587912e-25f64],
                [1.7919794787199875e-9f64, -6.923328170846374e-26f64],
                [3.6914144880020119e-10f64, 6.4499299207415957e-27f64],
                [7.620210182422826e-11f64, 1.8357557014032627e-27f64],
                [1.6750427855773864e-11f64, -4.125813626085642e-28f64],
                [3.813979986617072e-12f64, 7.998354359253682e-30f64],
                [5.559578661352867e-13f64, -3.555566279974889e-29f64],
            ];
            let sc_15: f64 = 16777216.0f64;
            let mut zl_15: f64 = 0.;
            let mut zh_15: f64 = fasttwosum(
                x0_15[0 as i32 as usize] + sx,
                x0_15[1 as i32 as usize],
                &raw mut zl_15,
            );
            zl_15 += x0_15[2 as i32 as usize];
            let mut sh_16: f64 = zh_15 * sc_15;
            let mut sl_16: f64 = zl_15 * sc_15;
            let mut n_15: i32 = (::core::mem::size_of::<[[f64; 2]; 27]>() as usize)
                .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                as i32;
            let mut k_15: i32 = 6 as i32;
            fl = sh_16
                * polyd(
                    sh_16,
                    k_15,
                    (&raw const c_15 as *const [f64; 2])
                        .offset(n_15 as isize)
                        .offset(-(k_15 as isize)),
                );
            fh = polydd(
                sh_16,
                sl_16,
                n_15 - k_15,
                &raw const c_15 as *const [f64; 2],
                &raw mut fl,
            );
            fh = muldd(zh_15, zl_15, fh, fl, &raw mut fl);
        }
    }
    let mut tl: b64u64_u = b64u64_u { f: fl };
    let mut ft: ::core::ffi::c_uint = (tl.u.wrapping_add(2 as uint64_t)
        & (!(0 as ::core::ffi::c_ulong) >> 12 as i32) as uint64_t)
        as ::core::ffi::c_uint;
    if ft <= 2 as ::core::ffi::c_uint {
        return as_lgamma_database(sx, fh + fl);
    }
    return fh + fl;
}
pub const SX_BND: f64 = -2.61f64;

fn cr_lgamma(mut x: f64) -> (f64, i32) {
    static mut ubrd: [::core::ffi::c_uint; 20] = [
        0x1ff0000 as i32 as ::core::ffi::c_uint,
        0x1ff146c as i32 as ::core::ffi::c_uint,
        0x1ff2b7b as i32 as ::core::ffi::c_uint,
        0x1ff4532 as i32 as ::core::ffi::c_uint,
        0x1ff614c as i32 as ::core::ffi::c_uint,
        0x1ff8310 as i32 as ::core::ffi::c_uint,
        0x1ff93f7 as i32 as ::core::ffi::c_uint,
        0x1ffa880 as i32 as ::core::ffi::c_uint,
        0x1ffc05e as i32 as ::core::ffi::c_uint,
        0x1ffdb73 as i32 as ::core::ffi::c_uint,
        0x1fff8a5 as i32 as ::core::ffi::c_uint,
        0x2001147 as i32 as ::core::ffi::c_uint,
        0x2002703 as i32 as ::core::ffi::c_uint,
        0x20041ac as i32 as ::core::ffi::c_uint,
        0x200622a as i32 as ::core::ffi::c_uint,
        0x20084d9 as i32 as ::core::ffi::c_uint,
        0x2009ce7 as i32 as ::core::ffi::c_uint,
        0x200ba2c as i32 as ::core::ffi::c_uint,
        0x200ddd7 as i32 as ::core::ffi::c_uint,
        0x20104ba as i32 as ::core::ffi::c_uint,
    ];
    static mut offs: [f64; 19] = [
        0.5398929119110107f64,
        0.6248214244842529f64,
        0.7200825214385986f64,
        0.8251934051513672f64,
        0.9520089626312256f64,
        1.0899691581726075f64,
        1.2362160682678223f64,
        1.4096546173095704f64,
        1.6086616516113282f64,
        1.8284974098205567f64,
        2.106274127960205f64,
        2.4398040771484377f64,
        2.817882537841797f64,
        3.2799930572509767f64,
        3.8426990509033205f64,
        4.52740478515625f64,
        5.360559463500977f64,
        6.375238418579102f64,
        7.61406135559082f64,
    ];
    static mut cl: [[f64; 8]; 19] = [
        [
            -4.3855828736322819f64,
            6.74304716948889f64,
            -10.691556665244044f64,
            17.320089677641929f64,
            -28.509340697135305f64,
            47.5240184507774f64,
            -81.23812807030652f64,
            137.6099593614462f64,
        ],
        [
            -2.1199006347690338f64,
            2.810667838468926f64,
            -3.847437839999388f64,
            5.383629925426579f64,
            -7.656000217663416f64,
            11.027046791282995f64,
            -16.27486752173504f64,
            23.78986889216702f64,
        ],
        [
            -1.0481007894635864f64,
            1.2024480209472198f64,
            -1.426424285382713f64,
            1.730903147451216f64,
            -2.1353169257407984f64,
            2.668401762767786f64,
            -3.413867997599439f64,
            4.320915797535968f64,
        ],
        [
            -0.5340646162332441f64,
            0.532764286301865f64,
            -0.5505236082086437f64,
            0.5824341008439105f64,
            -0.626731669577075f64,
            0.6833200945227193f64,
            -0.7617985063131887f64,
            0.8377009489858692f64,
        ],
        [
            -0.2178685632385823f64,
            0.19443442802239706f64,
            -0.1779230820598908f64,
            0.16574405320835579f64,
            -0.1564728156812634f64,
            0.14930490504584313f64,
            -0.14600786237654704f64,
            0.1413959411492839f64,
        ],
        [
            -0.1360135469762406f64,
            0.10163721199612019f64,
            -0.07904637132182123f64,
            0.06310794225009704f64,
            -0.05132054542963853f64,
            0.042306311179100147f64,
            -0.03562330311428445f64,
            0.03066501153077977f64,
        ],
        [
            -0.07365447713890019f64,
            0.048216284156556338f64,
            -0.032934436375239687f64,
            0.023128653084148005f64,
            -0.016559915855514367f64,
            0.012027686340756912f64,
            -0.008938257408848188f64,
            0.006679662166058867f64,
        ],
        [
            -0.039029737224777299f64,
            0.022232103327598508f64,
            -0.013251052541686522f64,
            0.008134858619030914f64,
            -0.005097541669379363f64,
            0.003242917839922688f64,
            -0.0021125896372274396f64,
            0.0013767276801667998f64,
        ],
        [
            -0.020715212531142975f64,
            0.010248334855743911f64,
            -0.00532065915625662f64,
            0.002850783271958538f64,
            -0.0015611873990541228f64,
            0.0008688124175084181f64,
            -0.0004952786967792873f64,
            0.0002811304790398691f64,
        ],
        [
            -0.011269295466587818f64,
            0.004858451343346767f64,
            -0.00220425367470534f64,
            0.0010341351361842749f64,
            -0.0004965983648342055f64,
            0.00024260815562429826f64,
            -0.00012130715722327826f64,
            0.00005961942099582415f64,
        ],
        [
            0.0022372975692023829f64,
            -0.0008752298527159892f64,
            0.00035468436493193327f64,
            -0.00014732657824665505f64,
            0.00006230187507561146f64,
            -0.00002670796942029863f64,
            0.000011771195592074289f64,
            -0.00000517950586029462f64,
        ],
        [
            -0.0029258951139308618f64,
            0.0009233264688449566f64,
            -0.0003084005815634236f64,
            0.00010698933858065604f64,
            -0.00003811752344754421f64,
            0.000013846209328938745f64,
            -0.000005174442274013638f64,
            0.000001965125717390162f64,
        ],
        [
            -0.001510036376980578f64,
            0.00040755049857286606f64,
            -0.00011670592443704754f64,
            0.00003478011663441861f64,
            -0.000010661550991683322f64,
            0.0000033366905409672357f64,
            -0.0000010771858170969696f64,
            3.5174206814669986e-7f64,
        ],
        [
            -0.000758537123687775f64,
            0.00017363630865536238f64,
            -0.00004226349696977956f64,
            0.000010725508492678408f64,
            -0.0000028040815984259497f64,
            7.494545798144687e-7f64,
            -2.0712294790577418e-7f64,
            5.778438765301511e-8f64,
        ],
        [
            -0.0003734360488634534f64,
            0.00007202586926952018f64,
            -0.000014798721187565284f64,
            0.0000031753527882438198f64,
            -7.028928630435549e-7f64,
            1.5926040416008886e-7f64,
            -3.7394634811351609e-8f64,
            8.854558479800469e-9f64,
        ],
        [
            -0.00018094955695517755f64,
            0.00002924939520318509f64,
            -0.000005044192634194948f64,
            9.096794392027198e-7f64,
            -1.6944925297342574e-7f64,
            3.2343831278677678e-8f64,
            -6.410233985558833e-9f64,
            1.2804478534095248e-9f64,
        ],
        [
            -0.00008657501874694734f64,
            0.000011677913942230905f64,
            -0.0000016825302399449429f64,
            2.53778545086073e-7f64,
            -3.9576080862792748e-8f64,
            6.330163873965299e-9f64,
            -1.0530494209410835e-9f64,
            1.7649624795440942e-10f64,
        ],
        [
            -0.000040984203208360548f64,
            0.00000459679981694744f64,
            -5.511946581857602e-7f64,
            6.924897565158208e-8f64,
            -9.00215880719322e-9f64,
            1.2011830480561952e-9f64,
            -1.6693192237121977e-10f64,
            2.336716222736096e-11f64,
        ],
        [
            -0.00001920696421322942f64,
            0.0000017855363241300229f64,
            -1.775714068423284e-7f64,
            1.8514381377920574e-8f64,
            -1.9986064746145556e-9f64,
            2.215777273554748e-10f64,
            -2.5617508435779353e-11f64,
            2.9823830812267396e-12f64,
        ],
    ];
    static mut ch: [[[f64; 2]; 13]; 19] = [
        [
            [0.49779314307310626f64, -2.252692609694315e-17f64],
            [-1.7790821514808637f64, 8.620376993969491e-17f64],
            [2.166770277547164f64, 1.4561170058009992e-16f64],
            [-2.247351760431607f64, -2.1623720963978092e-16f64],
            [2.995740170809078f64, -1.897843073498453e-16f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [0.3610889679447541f64, 1.0124944699666282e-18f64],
            [-1.453316274060218f64, -3.919088224744755e-17f64],
            [1.7013837642368853f64, -4.636434864117629e-17f64],
            [-1.4792991143440604f64, 3.5416541508808e-17f64],
            [1.6840116798727778f64, 3.269920794415352e-17f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [0.2369290791467043f64, 1.1053267687785714e-17f64],
            [-1.1643723609264032f64, -9.6271267204735e-17f64],
            [1.3549241399215429f64, 1.0150436036276175e-16f64],
            [-0.9905213760317287f64, 4.527706801986922e-17f64],
            [0.9653560345086539f64, 4.250601356514479e-17f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [0.12846601879484066f64, -1.0185795168623203e-17f64],
            [-0.9084431487553527f64, 1.109809943286857e-17f64],
            [1.0962754190435285f64, -5.614949844121929e-18f64],
            [-0.6775669624797169f64, 3.551859882705589e-17f64],
            [0.5677816878651447f64, -1.5727219021656854e-17f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [0.6176405983965069f64, -1.415457646877005e-17f64],
            [-0.8628916458042565f64, -5.0263317635133937e-17f64],
            [0.44270740730620458f64, 1.3007266889501264e-17f64],
            [-0.31463997311236477f64, -1.0806633867136487e-17f64],
            [0.25357852234481378f64, 1.1299557777275394e-17f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [-0.04554939974484693f64, 2.8436020679551654e-18f64],
            [-0.43822655111540095f64, -1.2338707950769611e-17f64],
            [0.7261004140306115f64, -1.7388924639678674e-17f64],
            [-0.3179203884837028f64, -2.4716693385648899e-17f64],
            [0.19470753536204744f64, 1.0988381144123998e-17f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [-0.09502230435049918f64, -6.454479609743613e-18f64],
            [-0.2440847969498882f64, 9.903157731487784e-18f64],
            [0.6079493178142871f64, 1.161580760288068e-18f64],
            [-0.2278164825879971f64, -5.475022906586699e-18f64],
            [0.1208644557961213f64, -4.973948206207298e-18f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [-0.12015791857767591f64, -3.6403341005790105e-19f64],
            [-0.05153094697167279f64, 1.8279160250494296e-18f64],
            [0.5079455233582063f64, 9.147536924311291e-18f64],
            [-0.16196297932017979f64, 6.234804257517564e-18f64],
            [0.07392853379644592f64, 4.2872455568759717e-18f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [-0.11146786092895751f64, 2.896436280075421e-19f64],
            [0.13345661799181778f64, 9.746011738519173e-18f64],
            [0.426189593528316f64, -2.2323972071299669e-17f64],
            [-0.11567437662850684f64, -2.4514408944497057e-18f64],
            [0.04537223619525485f64, -2.257993167488531e-18f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [-0.06266510701757572f64, -6.233728952468509e-18f64],
            [0.30578331819413326f64, 2.1669071225055579e-17f64],
            [0.3611665800253705f64, 3.156787845375162e-18f64],
            [-0.08397577692906834f64, 5.661629354889024e-19f64],
            [0.028435527328492074f64, 1.5333882540709717e-18f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [0.45631734635435519f64, 1.861085343570739e-17f64],
            [0.3088150424931568f64, -1.6706505136195307e-17f64],
            [-0.06125861292039729f64, -8.919825240928365e-19f64],
            [0.01774118070851763f64, 1.2734861960687644e-19f64],
            [-0.0060321107496966249f64, 3.7623393939858649e-19f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [0.24325283680969055f64, 1.0672640110200405e-17f64],
            [0.6732028391131202f64, -1.871899176401435e-17f64],
            [0.2524968377851165f64, 6.059603664001719e-18f64],
            [-0.04171208408096856f64, -2.039585158081472e-18f64],
            [0.010162119118338547f64, 2.789739470193923e-19f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [0.531802220654563f64, 1.6637269229389968e-17f64],
            [0.8481778685827528f64, 3.971267064866604e-17f64],
            [0.21256031096733003f64, 1.4388483951935878e-18f64],
            [-0.029710221425710387f64, -1.035616020982115e-18f64],
            [0.006150311587069513f64, -1.805154674337546e-19f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [0.966465858752032f64, 4.8206888354776256e-17f64],
            [1.0277251213468393f64, 4.13165749702217e-17f64],
            [0.17799736279913493f64, 1.8626224126705089e-18f64],
            [-0.020914443778007755f64, -4.904407988831854e-19f64],
            [0.003651918702074802f64, 2.0533872012043118e-19f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [1.5977354641942279f64, -2.5120371932778839e-17f64],
            [1.2104517176981525f64, -1.639368407536868e-18f64],
            [0.14849689051084434f64, -1.1869917884734408e-17f64],
            [-0.01459825683027192f64, 5.269832406401742e-19f64],
            [0.0021382531661783236f64, -7.950944519643799e-20f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [2.4918914687856934f64, 9.795944975505956e-17f64],
            [1.3956641725930666f64, 6.661232056250518e-17f64],
            [0.12352473101289842f64, -6.698186765160315e-18f64],
            [-0.010122356601499206f64, -6.043762084367614e-19f64],
            [0.0012383102631996683f64, -9.839024675780454e-20f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [3.7351183972653927f64, 5.079145988356087e-17f64],
            [1.5829044278259022f64, 8.831148319281709e-17f64],
            [0.10251116984685079f64, -1.897109201157974e-18f64],
            [-0.006981772833199092f64, 1.127894153836708e-19f64],
            [0.0007108823069228628f64, 5.4108736023862304e-20f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [5.440178776437958f64, 4.4335764348164828e-16f64],
            [1.7719476969292474f64, 3.226809154574123e-17f64],
            [0.08489951377137237f64, -1.573123630400604e-18f64],
            [-0.004793942704791622f64, 3.5998103883206833e-20f64],
            [0.0004050986367333946f64, -2.3082264571602177e-20f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
        [
            [7.757336768680731f64, -3.222605492098108e-16f64],
            [1.9628937717025579f64, 2.389793105350135e-17f64],
            [0.07016840181040954f64, -5.971810671837805e-19f64],
            [-0.0032770814785318205f64, 5.8264872515432e-20f64],
            [0.00022920643792615048f64, -1.0839165227301463e-20f64],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
            [0.; 2],
        ],
    ];
    let mut signgam = 0;
    unsafe {
        let mut t: b64u64_u = b64u64_u { f: x };
        let mut nx: uint64_t = unsafe { t.u } << 1 as i32;
        if (nx >= 0xfeaea9b24f16a34c as uint64_t) as i32 as ::core::ffi::c_long != 0 {
            signgam = 1 as i32;
            if t.u == 0x7f5754d9278b51a6 as uint64_t {
                return (1.7976931348623155e308f64 - 4.9896007738368e291f64, signgam);
            }
            if t.u == 0x7f5754d9278b51a7 as uint64_t {
                return (1.7976931348623157e308f64 - 4.9896007738368e291f64, signgam);
            }
            if (nx >= (0x7ff as uint64_t) << 53 as i32) as i32 as ::core::ffi::c_long != 0 {
                if nx == (0x7ff as uint64_t) << 53 as i32 {
                    return (x.abs(), signgam);
                }
                return (x + x, signgam);
            }
            if t.u >> 63 as i32 != 0 {
                return (1.0f64 / 0.0f64, signgam);
            } else {
                return (
                    1.7415152243978685e308f64 * 1.7415152243978685e308f64,
                    signgam,
                );
            }
        }
        let mut fx: f64 = x.floor();
        if (fx == x) as i32 as ::core::ffi::c_long != 0 {
            if x <= 0.0f64 {
                signgam = (1 as uint64_t)
                    .wrapping_sub((2 as uint64_t).wrapping_mul(t.u >> 63 as i32))
                    as i32;
                return (1.0f64 / 0.0f64, signgam);
            }
            if x == 1.0f64 || x == 2.0f64 {
                signgam = 1 as i32;
                return (0.0f64, signgam);
            }
        }
        let mut au: ::core::ffi::c_uint = (nx >> 38 as i32) as ::core::ffi::c_uint;
        let mut fh: f64 = 0.;
        let mut fl: f64 = 0.;
        let mut eps: f64 = 0.;
        if au < ubrd[0 as i32 as usize] {
            signgam = 1 as i32 - 2 as i32 * (t.u >> 63 as i32) as i32;
            let mut ll: f64 = 0.;
            let mut lh: f64 = as_logd(x.abs(), &raw mut ll);
            if au < 0x1da0000 as i32 as ::core::ffi::c_uint {
                fh = -lh;
                fl = -ll;
                eps = 1.5e-22f64;
            } else if au < 0x1fd0000 as i32 as ::core::ffi::c_uint {
                static mut c0: [[f64; 2]; 4] = [
                    [-0.5772156649015329f64, 4.942947824046454e-18f64],
                    [0.8224670334241132f64, 1.5202997596725896e-17f64],
                    [-0.40068563438653145f64, -1.5791674234993953e-19f64],
                    [0.27058080842778456f64, 2.09444231679447e-17f64],
                ];
                static mut q: [f64; 8] = [
                    -0.20738555102864524f64,
                    0.16955717699734228f64,
                    -0.1440498968945344f64,
                    0.12550966973320325f64,
                    -0.11133401771978549f64,
                    0.1000991331423616f64,
                    -0.09117976762950081f64,
                    0.08359844158349017f64,
                ];
                let mut z: f64 = x;
                let mut z2: f64 = z * z;
                let mut z4: f64 = z2 * z2;
                let mut q0: f64 = q[0 as i32 as usize] + z * q[1 as i32 as usize];
                let mut q2: f64 = q[2 as i32 as usize] + z * q[3 as i32 as usize];
                let mut q4: f64 = q[4 as i32 as usize] + z * q[5 as i32 as usize];
                let mut q6: f64 = q[6 as i32 as usize] + z * q[7 as i32 as usize];
                fl = z * (q0 + z2 * q2 + z4 * (q4 + z2 * q6));
                fh = polydddfst(z, 4 as i32, &raw const c0 as *const [f64; 2], &raw mut fl);
                fh = mulddd(x, fh, fl, &raw mut fl);
                fh = sumdd(-lh, -ll, fh, fl, &raw mut fl);
                eps = 1.5e-22f64;
            } else {
                let mut xl: f64 = 0.;
                t.f = fasttwosum(1 as i32 as f64, t.f, &raw mut xl);
                au = (t.u >> 37 as i32) as ::core::ffi::c_uint;
                let mut ou: ::core::ffi::c_uint = au.wrapping_sub(ubrd[0 as i32 as usize]);
                let mut j: i32 = ((0x157ced865 as ::core::ffi::c_ulong)
                    .wrapping_sub(
                        ou.wrapping_mul(0x150d as ::core::ffi::c_uint) as ::core::ffi::c_ulong
                    )
                    .wrapping_mul(ou as ::core::ffi::c_ulong)
                    .wrapping_add(0x128000000000 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                    >> 45 as i32) as i32;
                j -= (au < ubrd[j as usize]) as i32;
                let mut z_0: f64 = t.f - offs[j as usize] + xl;
                let mut z2_0: f64 = z_0 * z_0;
                let mut z4_0: f64 = z2_0 * z2_0;
                let mut q_0: *const f64 =
                    &raw const *(&raw const cl as *const [f64; 8]).offset(j as isize) as *const f64;
                let mut q0_0: f64 =
                    *q_0.offset(0 as i32 as isize) + z_0 * *q_0.offset(1 as i32 as isize);
                let mut q2_0: f64 =
                    *q_0.offset(2 as i32 as isize) + z_0 * *q_0.offset(3 as i32 as isize);
                let mut q4_0: f64 =
                    *q_0.offset(4 as i32 as isize) + z_0 * *q_0.offset(5 as i32 as isize);
                let mut q6_0: f64 =
                    *q_0.offset(6 as i32 as isize) + z_0 * *q_0.offset(7 as i32 as isize);
                fl = z_0 * (q0_0 + z2_0 * q2_0 + z4_0 * (q4_0 + z2_0 * q6_0));
                fh = polydddfst(
                    z_0,
                    5 as i32,
                    &raw const *(&raw const ch as *const [[f64; 2]; 13]).offset(j as isize)
                        as *const [f64; 2],
                    &raw mut fl,
                );
                if (j == 4 as i32) as i32 as ::core::ffi::c_long != 0 {
                    z_0 = -x;
                    fh = mulddd(z_0, fh, fl, &raw mut fl);
                }
                eps = fh.abs() * 8.3e-20f64;
                fh = sumdd(-lh, -ll, fh, fl, &raw mut fl);
                eps += lh.abs() * 5e-22f64;
            }
        } else {
            let mut ax: f64 = x.abs();
            if au >= ubrd[19 as i32 as usize] {
                let mut ll_0: f64 = 0.;
                let mut lh_0: f64 = as_logd(ax, &raw mut ll_0);
                lh_0 -= 1 as i32 as f64;
                if (au >= 0x2198000 as i32 as ::core::ffi::c_uint) as i32 as ::core::ffi::c_long
                    != 0
                {
                    if (au >= 0x3fabaa6 as i32 as ::core::ffi::c_uint) as i32 as ::core::ffi::c_long
                        != 0
                    {
                        lh_0 = fasttwosum(lh_0, ll_0, &raw mut ll_0);
                    }
                    let mut hlh: f64 = lh_0 * 0.5f64;
                    lh_0 = mulddd(ax, lh_0, ll_0, &raw mut ll_0);
                    ll_0 -= hlh;
                } else {
                    lh_0 = mulddd(ax - 0.5f64, lh_0, ll_0, &raw mut ll_0);
                }
                static mut c: [[f64; 2]; 2] = [
                    [0.4189385332046728f64, -3.870494472086548e-18f64],
                    [0.08333333333333128f64, -1.743291330366859e-18f64],
                ];
                static mut q_1: [f64; 5] = [
                    -0.002777777776596716f64,
                    0.0007936504110986061f64,
                    -0.0005951722707912049f64,
                    0.0008355961988070037f64,
                    -0.001617392910785053f64,
                ];
                lh_0 = fastsum(
                    lh_0,
                    ll_0,
                    c[0 as i32 as usize][0 as i32 as usize],
                    c[0 as i32 as usize][1 as i32 as usize],
                    &raw mut ll_0,
                );
                if ax < 1.2676506002282295e30f64 {
                    let mut zh: f64 = 1.0f64 / ax;
                    let mut zl: f64 = zh.fma(-ax, 1.0f64) * zh;
                    let mut z2h: f64 = zh * zh;
                    let mut z4h: f64 = z2h * z2h;
                    let mut q0_1: f64 = q_1[0 as i32 as usize] + z2h * q_1[1 as i32 as usize];
                    let mut q2_1: f64 = q_1[2 as i32 as usize] + z2h * q_1[3 as i32 as usize];
                    let mut q4_1: f64 = q_1[4 as i32 as usize];
                    fl = z2h * (q0_1 + z4h * (q2_1 + z4h * q4_1));
                    fh = fasttwosum(c[1 as i32 as usize][0 as i32 as usize], fl, &raw mut fl);
                    fl += c[1 as i32 as usize][1 as i32 as usize];
                    fh = muldd(fh, fl, zh, zl, &raw mut fl);
                } else {
                    fh = 0 as i32 as f64;
                    fl = 0 as i32 as f64;
                }
                fh = fastsum(lh_0, ll_0, fh, fl, &raw mut fl);
                eps = fh.abs() * 4.5e-20f64;
            } else {
                let mut ou_0: ::core::ffi::c_uint = au.wrapping_sub(ubrd[0 as i32 as usize]);
                let mut j_0: i32 = ((0x157ced865 as ::core::ffi::c_ulong)
                    .wrapping_sub(
                        ou_0.wrapping_mul(0x150d as ::core::ffi::c_uint) as ::core::ffi::c_ulong
                    )
                    .wrapping_mul(ou_0 as ::core::ffi::c_ulong)
                    .wrapping_add(0x128000000000 as ::core::ffi::c_long as ::core::ffi::c_ulong)
                    >> 45 as i32) as i32;
                j_0 -= (au < ubrd[j_0 as usize]) as i32;
                let mut z_1: f64 = ax - offs[j_0 as usize];
                let mut z2_1: f64 = z_1 * z_1;
                let mut z4_1: f64 = z2_1 * z2_1;
                let mut q_2: *const f64 = &raw const *(&raw const cl as *const [f64; 8])
                    .offset(j_0 as isize) as *const f64;
                let mut q0_2: f64 =
                    *q_2.offset(0 as i32 as isize) + z_1 * *q_2.offset(1 as i32 as isize);
                let mut q2_2: f64 =
                    *q_2.offset(2 as i32 as isize) + z_1 * *q_2.offset(3 as i32 as isize);
                let mut q4_2: f64 =
                    *q_2.offset(4 as i32 as isize) + z_1 * *q_2.offset(5 as i32 as isize);
                let mut q6_1: f64 =
                    *q_2.offset(6 as i32 as isize) + z_1 * *q_2.offset(7 as i32 as isize);
                fl = z_1 * (q0_2 + z2_1 * q2_2 + z4_1 * (q4_2 + z2_1 * q6_1));
                fh = polydddfst(
                    z_1,
                    5 as i32,
                    &raw const *(&raw const ch as *const [[f64; 2]; 13]).offset(j_0 as isize)
                        as *const [f64; 2],
                    &raw mut fl,
                );
                if (j_0 == 4 as i32) as i32 as ::core::ffi::c_long != 0 {
                    z_1 = 1 as i32 as f64 - ax;
                    fh = mulddd(z_1, fh, fl, &raw mut fl);
                }
                if (j_0 == 10 as i32) as i32 as ::core::ffi::c_long != 0 {
                    z_1 = ax - 2 as i32 as f64;
                    fh = mulddd(z_1, fh, fl, &raw mut fl);
                }
                eps = fh.abs() * 8.3e-20f64 + 1e-24f64;
            }
            if t.u >> 63 as i32 != 0 {
                let mut sl: f64 = 0.;
                let mut sh: f64 = as_sinpipid(x - x.floor(), &raw mut sl);
                sh = mulddd(-x, sh, sl, &raw mut sl);
                let mut ll_1: f64 = 0.;
                let mut lh_1: f64 = as_logd(sh, &raw mut ll_1);
                ll_1 += sl / sh;
                fh = -sumdd(fh, fl, lh_1, ll_1, &raw mut fl);
                fl = -fl;
                eps += lh_1.abs() * 4e-22f64;
                let mut k: int64_t = fx as int64_t;
                signgam = (1 as int64_t - 2 as int64_t * (k & 1 as int64_t)) as i32;
            } else {
                signgam = 1 as i32;
            }
        }
        let mut ub: f64 = fh + (fl + eps);
        let mut lb: f64 = fh + (fl - eps);
        if ub != lb {
            return (as_lgamma_accurate(x), signgam);
        }
        return (ub, signgam);
    }
}
#[inline(never)]
unsafe extern "C" fn as_logd(mut x: f64, mut l: *mut f64) -> f64 {
    static mut B: [C2RustUnnamed; 32] = [
        C2RustUnnamed {
            c0: 301 as ushort,
            c1: 27565 as i16,
        },
        C2RustUnnamed {
            c0: 7189 as ushort,
            c1: 24786 as i16,
        },
        C2RustUnnamed {
            c0: 13383 as ushort,
            c1: 22167 as i16,
        },
        C2RustUnnamed {
            c0: 18923 as ushort,
            c1: 19696 as i16,
        },
        C2RustUnnamed {
            c0: 23845 as ushort,
            c1: 17361 as i16,
        },
        C2RustUnnamed {
            c0: 28184 as ushort,
            c1: 15150 as i16,
        },
        C2RustUnnamed {
            c0: 31969 as ushort,
            c1: 13054 as i16,
        },
        C2RustUnnamed {
            c0: 35231 as ushort,
            c1: 11064 as i16,
        },
        C2RustUnnamed {
            c0: 37996 as ushort,
            c1: 9173 as i16,
        },
        C2RustUnnamed {
            c0: 40288 as ushort,
            c1: 7372 as i16,
        },
        C2RustUnnamed {
            c0: 42129 as ushort,
            c1: 5657 as i16,
        },
        C2RustUnnamed {
            c0: 43542 as ushort,
            c1: 4020 as i16,
        },
        C2RustUnnamed {
            c0: 44546 as ushort,
            c1: 2457 as i16,
        },
        C2RustUnnamed {
            c0: 45160 as ushort,
            c1: 962 as i16,
        },
        C2RustUnnamed {
            c0: 45399 as ushort,
            c1: -(468 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 45281 as ushort,
            c1: -(1838 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 44821 as ushort,
            c1: -(3151 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 44032 as ushort,
            c1: -(4412 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 42929 as ushort,
            c1: -(5622 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 41522 as ushort,
            c1: -(6786 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 39825 as ushort,
            c1: -(7905 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 37848 as ushort,
            c1: -(8982 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 35602 as ushort,
            c1: -(10020 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 33097 as ushort,
            c1: -(11020 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 30341 as ushort,
            c1: -(11985 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 27345 as ushort,
            c1: -(12916 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 24115 as ushort,
            c1: -(13816 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 20661 as ushort,
            c1: -(14685 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 16989 as ushort,
            c1: -(15526 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 13107 as ushort,
            c1: -(16339 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 9022 as ushort,
            c1: -(17126 as i32) as i16,
        },
        C2RustUnnamed {
            c0: 4740 as ushort,
            c1: -(17889 as i32) as i16,
        },
    ];
    static mut r1: [f64; 33] = [
        1.0f64,
        0.97857666015625f64,
        0.9575958251953125f64,
        0.9370880126953125f64,
        0.9170074462890625f64,
        0.8973541259765625f64,
        0.8781280517578125f64,
        0.85931396484375f64,
        0.8408966064453125f64,
        0.8228759765625f64,
        0.8052520751953125f64,
        0.787994384765625f64,
        0.7711029052734375f64,
        0.75457763671875f64,
        0.7384185791015625f64,
        0.72259521484375f64,
        0.7071075439453125f64,
        0.69195556640625f64,
        0.6771240234375f64,
        0.6626129150390625f64,
        0.6484222412109375f64,
        0.634521484375f64,
        0.6209259033203125f64,
        0.6076202392578125f64,
        0.5946044921875f64,
        0.5818634033203125f64,
        0.56939697265625f64,
        0.55718994140625f64,
        0.545257568359375f64,
        0.5335693359375f64,
        0.5221405029296875f64,
        0.510955810546875f64,
        0.5f64,
    ];
    static mut r2: [f64; 33] = [
        1.0f64,
        0.9993209838867188f64,
        0.9986495971679688f64,
        0.9979705810546875f64,
        0.9972991943359375f64,
        0.9966201782226563f64,
        0.9959487915039063f64,
        0.995269775390625f64,
        0.994598388671875f64,
        0.993927001953125f64,
        0.993255615234375f64,
        0.992584228515625f64,
        0.991912841796875f64,
        0.991241455078125f64,
        0.990570068359375f64,
        0.989898681640625f64,
        0.989227294921875f64,
        0.988555908203125f64,
        0.9878921508789063f64,
        0.9872207641601563f64,
        0.9865570068359375f64,
        0.9858856201171875f64,
        0.9852218627929688f64,
        0.9845504760742188f64,
        0.98388671875f64,
        0.9832229614257813f64,
        0.9825515747070313f64,
        0.9818878173828125f64,
        0.9812240600585938f64,
        0.980560302734375f64,
        0.9798965454101563f64,
        0.9792327880859375f64,
        0.9785690307617188f64,
    ];
    static mut l1: [[f64; 2]; 33] = [
        [0.0f64, 0.0f64],
        [1.1805475025881996e-11f64, 0.021656150638591499f64],
        [-9.620459248759583e-12f64, 0.043329484411515298f64],
        [2.2381913049988747e-12f64, 0.06497807084815577f64],
        [-5.79458410707672e-13f64, 0.08663968648761511f64],
        [-2.6624789748145338e-11f64, 0.10830470558721572f64],
        [-2.2292260740422745e-11f64, 0.1299628511769697f64],
        [9.843703839717896e-12f64, 0.15162092336686329f64],
        [-1.7761531107138347e-11f64, 0.1732865677913651f64],
        [1.4251836448079744e-11f64, 0.19494978641159833f64],
        [2.887893963022136e-11f64, 0.21659991366323085f64],
        [1.8461633870186114e-11f64, 0.23826431506313384f64],
        [1.1859820251841252e-11f64, 0.2599334444385022f64],
        [8.834820147665132e-12f64, 0.28159710782347249f64],
        [-7.726273340607149e-12f64, 0.30324443482095378f64],
        [1.117916252476488e-11f64, 0.32490608241641896f64],
        [-1.98409739147048e-11f64, 0.3465725115966052f64],
        [-2.4593969522959917e-11f64, 0.36823353584622967f64],
        [2.576514930906998e-11f64, 0.38990082719828936f64],
        [-7.87506100249456e-12f64, 0.41156429785769435f64],
        [4.488887551075488e-12f64, 0.43321318802190947f64],
        [-1.4609868709141755e-12f64, 0.45488413207931446f64],
        [-5.6702471219541234e-12f64, 0.4765435224981047f64],
        [-2.580584990045347e-11f64, 0.4982051986735314f64],
        [-2.2628853941503408e-11f64, 0.5198588134953752f64],
        [2.8774315834786873e-11f64, 0.541519560967572f64],
        [4.147037585794395e-12f64, 0.5631774208741263f64],
        [6.48990635341273e-12f64, 0.5848490892676637f64],
        [1.3701873260374077e-11f64, 0.6064969934523106f64],
        [1.8042386061707886e-11f64, 0.6281662523979321f64],
        [2.619607956389178e-11f64, 0.6498185645905323f64],
        [7.639207107015402e-12f64, 0.6714721689349972f64],
        [1.619851018665656e-11f64, 0.6931471805437468f64],
    ];
    static mut l2: [[f64; 2]; 32] = [
        [0.0f64, 0.0f64],
        [1.709805765329567e-11f64, 0.0006792467320337892f64],
        [-2.231943549566878e-11f64, 0.0013513154699467123f64],
        [-3.638348190962335e-12f64, 0.0020314810099080207f64],
        [2.740056000223127e-12f64, 0.0027044594171456994f64],
        [9.168597520525822e-12f64, 0.003385546267963946f64],
        [2.4540440936088918e-11f64, 0.0040594368474558f64],
        [1.5009171463582346e-11f64, 0.004741447512060404f64],
        [1.2613662256246141e-11f64, 0.005416252766735852f64],
        [-2.792506862060493e-11f64, 0.006091513729188591f64],
        [-1.4668297352289027e-11f64, 0.0067672309232875709f64],
        [-2.905110310849348e-11f64, 0.00744340504752472f64],
        [2.336697837618598e-11f64, 0.00812003662576899f64],
        [5.45172079262684e-12f64, 0.008797126414719969f64],
        [1.4158898442100886e-11f64, 0.009474674938246608f64],
        [-2.6915924741535217e-11f64, 0.010152682894840837f64],
        [-1.8288002708318294e-11f64, 0.010831150808371604f64],
        [2.4381726776653703e-11f64, 0.011510079319123179f64],
        [-2.7556004673636036e-11f64, 0.012181746249552817f64],
        [2.7786481367291053e-11f64, 0.012861592636909336f64],
        [-1.7960528873510353e-11f64, 0.013534168247133494f64],
        [-2.580948445313029e-11f64, 0.014214935072232038f64],
        [1.0140750530812659e-11f64, 0.014888421748764813f64],
        [-1.0241168790061937e-11f64, 0.01557011145632714f64],
        [1.8309433726799585e-11f64, 0.016244511760305615f64],
        [-1.42875891813086e-11f64, 0.01691936724819243f64],
        [-1.1656619771583104e-11f64, 0.017602443287614734f64],
        [1.1096525510425294e-12f64, 0.018278216070029886f64],
        [3.2313014206992063e-12f64, 0.018954445840790869f64],
        [-2.711381937257544e-11f64, 0.019631133240181954f64],
        [5.923832621821805e-12f64, 0.020308278792072089f64],
        [2.4830589227130414e-11f64, 0.020985883194953204f64],
    ];
    let mut t: b64u64_u = b64u64_u { f: x };
    let mut ex: i32 = (t.u >> 52 as i32) as i32;
    if (ex == 0 as i32) as i32 as ::core::ffi::c_long != 0 {
        let mut k: i32 = t.u.leading_zeros() as i32;
        t.u <<= k - 11 as i32;
        ex -= k - 12 as i32;
    }
    let mut e: i32 = ex - 0x3ff as i32;
    t.u &= (!(0 as i32 as u64_0) >> 12 as i32) as uint64_t;
    let mut ed: f64 = e as f64;
    let mut i: u64_0 = t.u as u64_0 >> 52 as i32 - 5 as i32;
    let mut d: int64_t = (t.u & !(0 as i32 as uint64_t) >> 17 as i32) as int64_t;
    let mut j: u64_0 = (t.u as u64_0)
        .wrapping_add((B[i as usize].c0 as u64_0) << 33 as i32)
        .wrapping_add((B[i as usize].c1 as int64_t * (d >> 16 as i32)) as u64_0)
        >> 52 as i32 - 10 as i32;
    t.u |= ((0x3ff as i32 as int64_t) << 52 as i32) as uint64_t;
    let mut i1: i32 = (j >> 5 as i32) as i32;
    let mut i2: i32 = (j & 0x1f as u64_0) as i32;
    let mut r: f64 = r1[i1 as usize] * r2[i2 as usize];
    let mut o: f64 = r * t.f;
    let mut dxl: f64 = r.fma(t.f, -o);
    let mut dxh: f64 = o - 1 as i32 as f64;
    static mut c: [f64; 4] = [
        -0.4999999999999975f64,
        0.3333333333330846f64,
        -0.2500000406974906f64,
        0.20000154654086306f64,
    ];
    let mut dx: f64 = r.fma(t.f, -(1 as i32) as f64);
    let mut dx2: f64 = dx * dx;
    let mut f: f64 = dx2
        * (c[0 as i32 as usize]
            + dx * c[1 as i32 as usize]
            + dx2 * (c[2 as i32 as usize] + dx * c[3 as i32 as usize]));
    let mut lt: f64 = l1[i1 as usize][1 as i32 as usize]
        + l2[i2 as usize][1 as i32 as usize]
        + ed * 0.6931471805437468f64;
    let mut lh: f64 = lt + dxh;
    let mut ll: f64 = lt - lh + dxh;
    ll += l1[i1 as usize][0 as i32 as usize]
        + l2[i2 as usize][0 as i32 as usize]
        + 1.619851018665656e-11f64 * ed
        + dxl;
    ll += f;
    *l = ll;
    return lh;
}
#[inline(never)]
unsafe extern "C" fn as_logd_accurate(mut x: f64, mut l: *mut f64, mut l_: *mut f64) -> f64 {
    static mut B: [C2RustUnnamed_0; 32] = [
        C2RustUnnamed_0 {
            c0: 301 as ushort,
            c1: 27565 as i16,
        },
        C2RustUnnamed_0 {
            c0: 7189 as ushort,
            c1: 24786 as i16,
        },
        C2RustUnnamed_0 {
            c0: 13383 as ushort,
            c1: 22167 as i16,
        },
        C2RustUnnamed_0 {
            c0: 18923 as ushort,
            c1: 19696 as i16,
        },
        C2RustUnnamed_0 {
            c0: 23845 as ushort,
            c1: 17361 as i16,
        },
        C2RustUnnamed_0 {
            c0: 28184 as ushort,
            c1: 15150 as i16,
        },
        C2RustUnnamed_0 {
            c0: 31969 as ushort,
            c1: 13054 as i16,
        },
        C2RustUnnamed_0 {
            c0: 35231 as ushort,
            c1: 11064 as i16,
        },
        C2RustUnnamed_0 {
            c0: 37996 as ushort,
            c1: 9173 as i16,
        },
        C2RustUnnamed_0 {
            c0: 40288 as ushort,
            c1: 7372 as i16,
        },
        C2RustUnnamed_0 {
            c0: 42129 as ushort,
            c1: 5657 as i16,
        },
        C2RustUnnamed_0 {
            c0: 43542 as ushort,
            c1: 4020 as i16,
        },
        C2RustUnnamed_0 {
            c0: 44546 as ushort,
            c1: 2457 as i16,
        },
        C2RustUnnamed_0 {
            c0: 45160 as ushort,
            c1: 962 as i16,
        },
        C2RustUnnamed_0 {
            c0: 45399 as ushort,
            c1: -(468 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 45281 as ushort,
            c1: -(1838 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 44821 as ushort,
            c1: -(3151 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 44032 as ushort,
            c1: -(4412 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 42929 as ushort,
            c1: -(5622 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 41522 as ushort,
            c1: -(6786 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 39825 as ushort,
            c1: -(7905 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 37848 as ushort,
            c1: -(8982 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 35602 as ushort,
            c1: -(10020 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 33097 as ushort,
            c1: -(11020 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 30341 as ushort,
            c1: -(11985 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 27345 as ushort,
            c1: -(12916 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 24115 as ushort,
            c1: -(13816 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 20661 as ushort,
            c1: -(14685 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 16989 as ushort,
            c1: -(15526 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 13107 as ushort,
            c1: -(16339 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 9022 as ushort,
            c1: -(17126 as i32) as i16,
        },
        C2RustUnnamed_0 {
            c0: 4740 as ushort,
            c1: -(17889 as i32) as i16,
        },
    ];
    static mut r1: [f64; 33] = [
        1.0f64,
        0.97857666015625f64,
        0.9575958251953125f64,
        0.9370880126953125f64,
        0.9170074462890625f64,
        0.8973541259765625f64,
        0.8781280517578125f64,
        0.85931396484375f64,
        0.8408966064453125f64,
        0.8228759765625f64,
        0.8052520751953125f64,
        0.787994384765625f64,
        0.7711029052734375f64,
        0.75457763671875f64,
        0.7384185791015625f64,
        0.72259521484375f64,
        0.7071075439453125f64,
        0.69195556640625f64,
        0.6771240234375f64,
        0.6626129150390625f64,
        0.6484222412109375f64,
        0.634521484375f64,
        0.6209259033203125f64,
        0.6076202392578125f64,
        0.5946044921875f64,
        0.5818634033203125f64,
        0.56939697265625f64,
        0.55718994140625f64,
        0.545257568359375f64,
        0.5335693359375f64,
        0.5221405029296875f64,
        0.510955810546875f64,
        0.5f64,
    ];
    static mut r2: [f64; 33] = [
        1.0f64,
        0.9993209838867188f64,
        0.9986495971679688f64,
        0.9979705810546875f64,
        0.9972991943359375f64,
        0.9966201782226563f64,
        0.9959487915039063f64,
        0.995269775390625f64,
        0.994598388671875f64,
        0.993927001953125f64,
        0.993255615234375f64,
        0.992584228515625f64,
        0.991912841796875f64,
        0.991241455078125f64,
        0.990570068359375f64,
        0.989898681640625f64,
        0.989227294921875f64,
        0.988555908203125f64,
        0.9878921508789063f64,
        0.9872207641601563f64,
        0.9865570068359375f64,
        0.9858856201171875f64,
        0.9852218627929688f64,
        0.9845504760742188f64,
        0.98388671875f64,
        0.9832229614257813f64,
        0.9825515747070313f64,
        0.9818878173828125f64,
        0.9812240600585938f64,
        0.980560302734375f64,
        0.9798965454101563f64,
        0.9792327880859375f64,
        0.9785690307617188f64,
    ];
    static mut h1: [[f64; 3]; 33] = [
        [0.0f64, 0.0f64, 0.0f64],
        [
            3.631929162536462e-29f64,
            2.0941757827716039e-13f64,
            0.021656150650187557f64,
        ],
        [
            2.2598598029406586e-30f64,
            1.5660879529939536e-13f64,
            0.04332948440173823f64,
        ],
        [
            4.1769576015476018e-29f64,
            1.9182822600978608e-13f64,
            0.06497807085020213f64,
        ],
        [
            3.125925969906054e-29f64,
            1.0266261562202417e-13f64,
            0.08663968648693299f64,
        ],
        [
            7.31390738409895e-30f64,
            2.0530395415604763e-13f64,
            0.10830470556038563f64,
        ],
        [
            1.994040004111065e-29f64,
            2.177331284572296e-13f64,
            0.1299628511544597f64,
        ],
        [
            2.594441775177296e-29f64,
            6.663579565891687e-14f64,
            0.15162092337664036f64,
        ],
        [
            3.479485950410588e-29f64,
            2.009892528769876e-13f64,
            0.17328656777340258f64,
        ],
        [
            2.77942884873107e-29f64,
            1.5466857059935616e-13f64,
            0.1949497864256955f64,
        ],
        [
            3.0860856080722059e-29f64,
            2.4828489308877234e-15f64,
            0.2165999136921073f64,
        ],
        [
            2.2113750409948637e-30f64,
            4.436615928431593e-14f64,
            0.2382643150815511f64,
        ],
        [
            6.731128983411545e-30f64,
            3.638912879318423e-14f64,
            0.2599334444503256f64,
        ],
        [
            4.446039276577361e-29f64,
            1.9462048082231265e-13f64,
            0.28159710783211269f64,
        ],
        [
            4.7411841807855637e-30f64,
            4.431624462740781e-15f64,
            0.30324443481322307f64,
        ],
        [
            2.965320667894069e-29f64,
            3.785242804650824e-14f64,
            0.32490608242756027f64,
        ],
        [
            4.754925796688543e-29f64,
            1.6790952429962387e-13f64,
            0.3465725115765963f64,
        ],
        [
            1.5634267231987985e-29f64,
            1.8976110035237756e-13f64,
            0.3682335358214459f64,
        ],
        [
            3.4994051078322088e-29f64,
            7.19239839847586e-14f64,
            0.38990082722398258f64,
        ],
        [
            4.05640541367031e-29f64,
            8.301763801856115e-14f64,
            0.41156429784973627f64,
        ],
        [
            1.890356188539675e-30f64,
            1.6878771765407882e-13f64,
            0.43321318802622957f64,
        ],
        [
            3.0267797085988699e-29f64,
            1.3062885718844885e-13f64,
            0.4548841320777228f64,
        ],
        [
            1.2992782280660458e-29f64,
            1.4094764126678257e-14f64,
            0.4765435224924204f64,
        ],
        [
            1.3129329630988604e-29f64,
            1.147491000749832e-13f64,
            0.4982051986476108f64,
        ],
        [
            3.936541185274357e-29f64,
            1.0851360281979966e-13f64,
            0.5198588134726379f64,
        ],
        [
            2.3866240117401117e-29f64,
            1.2523272893963329e-13f64,
            0.5415195609962211f64,
        ],
        [
            2.1496551095060267e-29f64,
            5.431142781621759e-14f64,
            0.563177420878219f64,
        ],
        [
            1.5002017591055102e-29f64,
            1.2344344100223238e-13f64,
            0.5848490892740301f64,
        ],
        [
            3.438517882799412e-29f64,
            5.945273378015337e-14f64,
            0.606496993465953f64,
        ],
        [
            6.436459592372268e-30f64,
            7.986570169255212e-14f64,
            0.6281662524158946f64,
        ],
        [
            4.592126310488655e-30f64,
            4.810688792009239e-14f64,
            0.6498185646166803f64,
        ],
        [
            8.407218677377686e-30f64,
            1.3587581738874287e-13f64,
            0.6714721689425005f64,
        ],
        [
            1.94704509238075e-31f64,
            5.497923018708371e-14f64,
            0.6931471805598903f64,
        ],
    ];
    static mut h2: [[f64; 3]; 33] = [
        [0.0f64, 0.0f64, 0.0f64],
        [
            3.235803058596668e-29f64,
            4.5031995053266679e-14f64,
            0.0006792467490868148f64,
        ],
        [
            4.0670182827322257e-29f64,
            1.9055837321119353e-13f64,
            0.0013513154474367184f64,
        ],
        [
            1.2679584433279983e-29f64,
            2.270042915726098e-13f64,
            0.002031481006042668f64,
        ],
        [
            3.189791038972655e-29f64,
            1.1571894904342463e-14f64,
            0.0027044594198741835f64,
        ],
        [
            1.401862743337263e-29f64,
            7.365050279653966e-14f64,
            0.003385546277058893f64,
        ],
        [
            4.3520586956352688e-29f64,
            2.1145766366308583e-13f64,
            0.004059436871784783f64,
        ],
        [
            4.076976046100068e-29f64,
            2.5088843290295978e-15f64,
            0.004741447527067066f64,
        ],
        [
            6.091084835257411e-30f64,
            1.0811010686837793e-13f64,
            0.005416252779241404f64,
        ],
        [
            3.492819732455552e-29f64,
            4.189345891261241e-14f64,
            0.006091513701221629f64,
        ],
        [
            4.127408454073453e-29f64,
            1.1099155152105853e-13f64,
            0.006767230908508282f64,
        ],
        [
            4.134475766912839e-29f64,
            5.2727348240223589e-14f64,
            0.00744340501842089f64,
        ],
        [
            3.7399652272710016e-29f64,
            1.7486348097631164e-13f64,
            0.008120036648961105f64,
        ],
        [
            2.9585027653035446e-29f64,
            2.2212625743250315e-13f64,
            0.008797126419949564f64,
        ],
        [
            3.1285454289688139e-29f64,
            6.173056462049871e-14f64,
            0.009474674952343776f64,
        ],
        [
            4.46616902486834e-29f64,
            1.415426362094e-13f64,
            0.01015268286778337f64,
        ],
        [
            3.4358733316370246e-29f64,
            1.2926500258350504e-13f64,
            0.010831150789954336f64,
        ],
        [
            2.2428693945691509e-29f64,
            5.2743504227872217e-14f64,
            0.011510079343452162f64,
        ],
        [
            4.5706085709959196e-29f64,
            1.8358373043827692e-13f64,
            0.012181746221813228f64,
        ],
        [
            1.945219607319051e-29f64,
            4.6892963216740148e-14f64,
            0.012861592664648925f64,
        ],
        [
            5.760286522039024e-31f64,
            1.9914865049789937e-15f64,
            0.013534168229170973f64,
        ],
        [
            3.5386399061077458e-29f64,
            1.1111454739816318e-13f64,
            0.014214935046311439f64,
        ],
        [
            2.64134422197228e-29f64,
            1.3630881131044853e-13f64,
            0.014888421758769255f64,
        ],
        [
            3.1140126750143018e-29f64,
            2.1802028032673823e-13f64,
            0.015570111445867952f64,
        ],
        [
            5.033545809471978e-29f64,
            1.1953969134102054e-13f64,
            0.016244511778495509f64,
        ],
        [
            7.86540137262246e-30f64,
            3.6952371615020746e-14f64,
            0.016919367233867889f64,
        ],
        [
            2.380625606809352e-30f64,
            1.66811351464963e-13f64,
            0.017602443275791303f64,
        ],
        [
            3.317413768628623e-29f64,
            2.0015784926960109e-13f64,
            0.01827821607093938f64,
        ],
        [
            2.6110825194808475e-30f64,
            4.806996449395728e-14f64,
            0.0189544458439741f64,
        ],
        [
            2.160046182979622e-29f64,
            1.7102168061240756e-13f64,
            0.01963113321289711f64,
        ],
        [
            7.308706692955563e-30f64,
            1.2117060297771795e-14f64,
            0.020308278797983804f64,
        ],
        [
            1.228890698366805e-29f64,
            4.685860381811971e-14f64,
            0.020985883219736935f64,
        ],
        [
            2.223639156862798e-29f64,
            1.9494834532892497e-13f64,
            0.021663947100478255f64,
        ],
    ];
    let mut t: b64u64_u = b64u64_u { f: x };
    let mut ex: i32 = (t.u >> 52 as i32) as i32;
    if (ex == 0 as i32) as i32 as ::core::ffi::c_long != 0 {
        let mut k: i32 = t.u.leading_zeros() as i32;
        t.u <<= k - 11 as i32;
        ex -= k - 12 as i32;
    }
    let mut e: i32 = ex - 0x3ff as i32;
    t.u &= (!(0 as i32 as u64_0) >> 12 as i32) as uint64_t;
    let mut ed: f64 = e as f64;
    let mut i: u64_0 = t.u as u64_0 >> 52 as i32 - 5 as i32;
    let mut d: int64_t = (t.u & !(0 as i32 as uint64_t) >> 17 as i32) as int64_t;
    let mut j: u64_0 = (t.u as u64_0)
        .wrapping_add((B[i as usize].c0 as u64_0) << 33 as i32)
        .wrapping_add((B[i as usize].c1 as int64_t * (d >> 16 as i32)) as u64_0)
        >> 52 as i32 - 10 as i32;
    t.u |= ((0x3ff as i32 as int64_t) << 52 as i32) as uint64_t;
    let mut i1: i32 = (j >> 5 as i32) as i32;
    let mut i2: i32 = (j & 0x1f as u64_0) as i32;
    let mut r: f64 = r1[i1 as usize] * r2[i2 as usize];
    let mut o: f64 = r * t.f;
    let mut dxl: f64 = r.fma(t.f, -o);
    let mut dxh: f64 = o - 1 as i32 as f64;
    static mut c: [[f64; 2]; 9] = [
        [1.0f64, 4.9086000877204215e-36f64],
        [-0.5f64, 2.4435592135551846e-30f64],
        [0.3333333333333333f64, 1.8503717075875657e-17f64],
        [-0.25f64, -2.0062798904959609e-22f64],
        [0.2f64, -1.105743672876824e-17f64],
        [-0.16666666666666223f64, -2.115592810301991e-18f64],
        [0.1428571428566262f64, -5.34817649119905e-18f64],
        [-0.12500003652775225f64, 8.19918290175534e-18f64],
        [0.11111294767790256f64, 3.9233912358975889e-18f64],
    ];
    dxh = fasttwosum(dxh, dxl, &raw mut dxl);
    let mut fl: f64 = dxh
        * (c[6 as i32 as usize][0 as i32 as usize]
            + dxh
                * (c[7 as i32 as usize][0 as i32 as usize]
                    + dxh * c[8 as i32 as usize][0 as i32 as usize]));
    let mut fh: f64 = polydd(
        dxh,
        dxl,
        6 as i32,
        &raw const c as *const [f64; 2],
        &raw mut fl,
    );
    fh = muldd(dxh, dxl, fh, fl, &raw mut fl);
    let mut s2: f64 = h1[i1 as usize][2 as i32 as usize] + h2[i2 as usize][2 as i32 as usize];
    let mut s1: f64 = h1[i1 as usize][1 as i32 as usize] + h2[i2 as usize][1 as i32 as usize];
    let mut s0: f64 = h1[i1 as usize][0 as i32 as usize] + h2[i2 as usize][0 as i32 as usize];
    let mut L0: f64 = 0.6931471805598903f64 * ed;
    let mut L1: f64 = 5.49792301870721e-14f64 * ed;
    let mut L2: f64 = 1.1612227229362533e-26f64 * ed;
    L0 += s2;
    L1 = sumdd(L1, L2, s1, s0, &raw mut L2);
    L1 = sumdd(L1, L2, fh, fl, &raw mut L2);
    L0 = fasttwosum(L0, L1, &raw mut L1);
    L1 = fasttwosum(L1, L2, &raw mut L2);
    *l = L1;
    *l_ = L2;
    return L0;
}
static mut stpi: [[f64; 2]; 65] = [
    [0.0f64, 0.0f64],
    [9.514479430572978e-20f64, 0.0078117156579386079f64],
    [-3.356808024190129e-19f64, 0.015618725830463736f64],
    [-4.488158969254586e-19f64, 0.023416327866570369f64],
    [-9.380178760197209e-19f64, 0.031199824782363076f64],
    [1.6001221211963379e-18f64, 0.038964528090343477f64],
    [-1.0966216671001662e-18f64, 0.046705760623579808f64],
    [-3.0124784860919437e-18f64, 0.05441885935305737f64],
    [9.20673598389283e-19f64, 0.06209917819651286f64],
    [5.024142532993566e-18f64, 0.0697420908170606f64],
    [-1.9570961965201839e-18f64, 0.07734299340992491f64],
    [-2.1331132190283156e-19f64, 0.08489730747559987f64],
    [-1.893795836572451e-18f64, 0.09240048257776633f64],
    [3.639164294526783e-18f64, 0.09984799908430452f64],
    [-1.2191308145302099e-18f64, 0.10723537088975149f64],
    [-1.1718515279700323e-19f64, 0.11455814811756327f64],
    [-5.3292037395419329e-18f64, 0.12181191980055409f64],
    [-8.219579220473635e-18f64, 0.1289923165378981f64],
    [7.163316435563438e-18f64, 0.13609501312709308f64],
    [-1.0659199035262745e-17f64, 0.14311573116930063f64],
    [-1.1006507663357737e-18f64, 0.15005024164649365f64],
    [-4.0827691344789578e-18f64, 0.15689436746885875f64],
    [-8.187107512657328e-18f64, 0.16364398599091888f64],
    [1.3227727342393938e-17f64, 0.1702950314948608f64],
    [1.2097146246389688e-17f64, 0.17684349763957164f64],
    [3.675115397602226e-19f64, 0.1832854398739087f64],
    [-1.1544202501506568e-19f64, 0.18961697781274973f64],
    [2.01325261418969e-18f64, 0.19583429757439184f64],
    [-1.2112299665944573e-17f64, 0.20193365407789119f64],
    [-1.2524282108005139e-17f64, 0.20791137329895968f64],
    [5.631007787257419e-19f64, 0.21376385448305985f64],
    [-1.1341576838504279e-17f64, 0.2194875723143648f64],
    [3.4483068790877359e-18f64, 0.22507907903927652f64],
    [-4.685035179700337e-18f64, 0.23053500654322385f64],
    [9.742627205103034e-18f64, 0.23585206837948864f64],
    [1.1120845051005662e-17f64, 0.2410270617488385f64],
    [6.209717333381594e-18f64, 0.24605686942877323f64],
    [-1.2333052152714913e-17f64, 0.250938461651223f64],
    [1.5181570777356638e-17f64, 0.25566889792756755f64],
    [-1.7992131172162417e-17f64, 0.2602453288198764f64],
    [5.933965598278383e-18f64, 0.2646649976573037f64],
    [4.502132403456939e-18f64, 0.268925242196604f64],
    [-2.369380497996136e-17f64, 0.2730234962257676f64],
    [-3.768686171367908e-18f64, 0.2769572911098108f64],
    [-1.992186809289239e-18f64, 0.28072425727778968f64],
    [1.6876440435257715e-17f64, 0.2843221256501404f64],
    [-1.590926946626287e-17f64, 0.28774872900548867f64],
    [-1.1612288815171047e-17f64, 0.29100200328610195f64],
    [2.005008239729115e-17f64, 0.2940799888412014f64],
    [6.993213127276386e-18f64, 0.29698083160738206f64],
    [-2.2402346364675639e-17f64, 0.29970278422543097f64],
    [2.717783719541411e-17f64, 0.3022442070928713f64],
    [1.2374826528916318e-17f64, 0.3046035693515978f64],
    [2.6664262748243846e-17f64, 0.306779449810008f64],
    [-2.1419249910767317e-17f64, 0.30877053779907517f64],
    [2.59277790026451e-17f64, 0.3105756339618461f64],
    [-2.1454934564801215e-17f64, 0.3121936509758895f64],
    [-2.251345290189167e-17f64, 0.3136236142082575f64],
    [1.0260028774796226e-18f64, 0.3148646623025687f64],
    [-1.1201046639601087e-18f64, 0.3159160476978569f64],
    [-1.8866822508848975e-17f64, 0.31677713707887386f64],
    [-1.1710694613383182e-17f64, 0.3174474117575745f64],
    [2.1566713079995479e-17f64, 0.3179264679855557f64],
    [2.6127708245158929e-17f64, 0.3182140171972587f64],
    [-1.9678676675182487e-17f64, 0.3183098861837907f64],
];
#[inline(never)]
unsafe extern "C" fn as_sinpipid(mut x: f64, mut l: *mut f64) -> f64 {
    x -= 0.5f64;
    let mut ax: f64 = x.abs();
    let mut sx: f64 = ax * 128 as i32 as f64;
    let mut ix: f64 = sx.roundeven();
    let mut ky: i32 = ix as i32;
    let mut kx: i32 = 64 as i32 - ky;
    if (kx < 2 as i32) as i32 as ::core::ffi::c_long != 0 {
        static mut c: [f64; 2] = [-1.6449340668482265f64, -3.02274545379001e-17f64];
        static mut cl: [f64; 3] = [
            0.8117424252833463f64,
            -0.19075182400041194f64,
            0.026146961822224224f64,
        ];
        let mut z: f64 = 0.5f64 - ax;
        let mut z2: f64 = z * z;
        let mut z2l: f64 = z.fma(z, -z2);
        let mut fl: f64 = z2
            * (cl[0 as i32 as usize] + z2 * (cl[1 as i32 as usize] + z2 * cl[2 as i32 as usize]));
        let mut fh: f64 = fasttwosum(c[0 as i32 as usize], fl, &raw mut fl);
        let mut e: f64 = 0.;
        fl += c[1 as i32 as usize];
        fh = muldd(z2, z2l, fh, fl, &raw mut fl);
        fh = mulddd(z, fh, fl, &raw mut fl);
        fh = fasttwosum(z, fh, &raw mut e);
        fl += e;
        *l = fl;
        return fh;
    }
    let mut d: f64 = ix - sx;
    let mut d2: f64 = d * d;
    let mut sh: f64 = stpi[kx as usize][1 as i32 as usize];
    let mut sl: f64 = stpi[kx as usize][0 as i32 as usize];
    let mut ch: f64 = stpi[ky as usize][1 as i32 as usize];
    let mut cl_0: f64 = stpi[ky as usize][0 as i32 as usize];
    static mut c_0: [f64; 4] = [
        -0.0003011964233730883f64,
        1.5119880908790114e-8f64,
        -3.0360360343398626e-13f64,
        3.2658548907715e-18f64,
    ];
    let mut c0: f64 = -1.912015617779642e-20f64;
    static mut s: [f64; 4] = [
        0.02454369260617026f64,
        -0.0000024641574764489987f64,
        7.421954185251001e-11f64,
        -1.0645026354944996e-15f64,
    ];
    let mut s0: f64 = 9.567540887362087e-19f64;
    let mut P: f64 =
        d2 * (c_0[1 as i32 as usize] + d2 * (c_0[2 as i32 as usize] + d2 * c_0[3 as i32 as usize]));
    let mut Q: f64 =
        d2 * (s[1 as i32 as usize] + d2 * (s[2 as i32 as usize] + d2 * s[3 as i32 as usize]));
    let mut ql: f64 = 0.;
    let mut qh: f64 = fasttwosum(s[0 as i32 as usize], Q, &raw mut ql);
    ql += s0;
    ch = muldd(qh, ql, ch, cl_0, &raw mut cl_0);
    let mut tl: f64 = 0.;
    let mut th: f64 = fasttwosum(c_0[0 as i32 as usize], P, &raw mut tl);
    tl += c0;
    th = mulddd(d, th, tl, &raw mut tl);
    let mut pl: f64 = 0.;
    let mut ph: f64 = muldd(th, tl, sh, sl, &raw mut pl);
    ch = fastsum(ch, cl_0, ph, pl, &raw mut cl_0);
    ch = mulddd(d, ch, cl_0, &raw mut cl_0);
    sh = fastsum(sh, sl, ch, cl_0, l);
    return sh;
}
#[inline(never)]
unsafe extern "C" fn as_sinpipid_accurate(mut x: f64, mut l: *mut f64) -> f64 {
    x -= 0.5f64;
    x = x.abs();
    x *= 128 as i32 as f64;
    let mut ix: f64 = x.roundeven();
    let mut d: f64 = ix - x;
    let mut ky: i32 = ix as i32;
    let mut kx: i32 = 64 as i32 - ky;
    let mut sh: f64 = stpi[kx as usize][1 as i32 as usize];
    let mut sl: f64 = stpi[kx as usize][0 as i32 as usize];
    let mut ch: f64 = stpi[ky as usize][1 as i32 as usize];
    let mut cl: f64 = stpi[ky as usize][0 as i32 as usize];
    static mut c: [[f64; 2]; 5] = [
        [-0.0003011964233730883f64, -1.9120164516414152e-20f64],
        [1.5119880908790114e-8f64, -9.910019157311934e-25f64],
        [-3.036036034369748e-13f64, 3.451108199845882e-30f64],
        [3.2658685527788966e-18f64, -6.447101376310924e-35f64],
        [-2.1859212456183624e-23f64, 6.338816118440823e-41f64],
    ];
    static mut s: [[f64; 2]; 6] = [
        [0.02454369260617026f64, 9.567553118338697e-19f64],
        [-0.0000024641574764489987f64, 1.080781117715496e-22f64],
        [7.421954185344935e-11f64, -2.30822716328948e-27f64],
        [-1.064507645268961e-15f64, 5.77108356981848e-32f64],
        [8.906274872405494e-21f64, -3.228907213626295e-37f64],
        [-4.8773267651091936e-26f64, 1.9570448456525309e-42f64],
    ];
    let mut d2h: f64 = d * d;
    let mut d2l: f64 = d.fma(d, -d2h);
    let mut Pl: f64 = 0 as i32 as f64;
    let mut Ph: f64 = polydd(
        d2h,
        d2l,
        5 as i32,
        &raw const c as *const [f64; 2],
        &raw mut Pl,
    );
    let mut Ql: f64 = 0 as i32 as f64;
    let mut Qh: f64 = polydd(
        d2h,
        d2l,
        6 as i32,
        &raw const s as *const [f64; 2],
        &raw mut Ql,
    );
    Ph = mulddd(d, Ph, Pl, &raw mut Pl);
    Ph = muldd(sh, sl, Ph, Pl, &raw mut Pl);
    Qh = muldd(ch, cl, Qh, Ql, &raw mut Ql);
    ch = fastsum(Qh, Ql, Ph, Pl, &raw mut cl);
    ch = mulddd(d, ch, cl, &raw mut cl);
    sh = fastsum(sh, sl, ch, cl, l);
    return sh;
}
#[inline(never)]
unsafe extern "C" fn as_lgamma_asym_accurate(
    mut xh: f64,
    mut xl: *mut f64,
    mut e: *mut f64,
) -> f64 {
    let mut l2: f64 = 0.;
    let mut l1: f64 = 0.;
    let mut l0: f64 = as_logd_accurate(xh, &raw mut l1, &raw mut l2);
    let mut l0x: f64 = 0.;
    let mut l1x: f64 = 0.;
    let mut l2x: f64 = 0.;
    if xh < 1.329227995784916e36f64 {
        let mut zh: f64 = 1.0f64 / xh;
        let mut dz: f64 = *xl * zh;
        let mut zl: f64 = (zh.fma(-xh, 1.0f64) - dz) * zh;
        if *xl != 0 as i32 as f64 {
            let mut dl2: f64 = 0.;
            let mut dl1: f64 = mulddd(*xl, zh, zh.fma(-xh, 1.0f64) * zh, &raw mut dl2);
            dl2 -= dl1 * dl1 / 2 as i32 as f64;
            l1 = sumdd(l1, l2, dl1, dl2, &raw mut l2);
        }
        let mut t: b64u64_u = b64u64_u { f: xh };
        let mut wl: f64 = 0.;
        let mut wh: f64 = 0.;
        if t.u >> 52 as i32 > (0x3ff as i32 + 51 as i32) as uint64_t {
            wh = xh;
            wl = *xl - 0.5f64;
        } else {
            wh = xh - 0.5f64;
            wl = *xl;
        }
        l0 -= 1 as i32 as f64;
        l0x = l0 * wh;
        let mut l0xl: f64 = l0.fma(wh, -l0x);
        l1x = l1 * wh;
        let mut l1xl: f64 = l1.fma(wh, -l1x);
        l2x = l2 * wh;
        l1x = sumdd(l1x, l2x, l0xl, l1xl, &raw mut l2x);
        l1x = sumdd(l1x, l2x, l0 * wl, l1 * wl, &raw mut l2x);
        let mut z2l: f64 = 0.;
        let mut z2h: f64 = muldd(zh, zl, zh, zl, &raw mut z2l);
        let mut fh: f64 = 0.;
        let mut fl: f64 = 0.;
        if xh >= 48 as i32 as f64 {
            static mut c: [[f64; 2]; 8] = [
                [0.4189385332046727f64, 1.672820965058789e-17f64],
                [0.08333333333333333f64, 4.625929268255126e-18f64],
                [-0.002777777777777778f64, 1.0603809735373365e-19f64],
                [0.0007936507936507932f64, 2.685056578894273e-20f64],
                [-0.0005952380952348093f64, -4.794183978280348e-20f64],
                [0.0008417508267765657f64, 5.175926481109301e-20f64],
                [-0.0019174882752632506f64, 6.263245105325144e-20f64],
                [0.0063575847155219748f64, -1.2163750844633815e-19f64],
            ];
            l1x = sumdd(
                l1x,
                l2x,
                c[0 as i32 as usize][0 as i32 as usize],
                c[0 as i32 as usize][1 as i32 as usize],
                &raw mut l2x,
            );
            fl = 0 as i32 as f64;
            fh = polydd(
                z2h,
                z2l,
                7 as i32,
                (&raw const c as *const [f64; 2]).offset(1 as i32 as isize),
                &raw mut fl,
            );
        } else if xh >= 14.5f64 {
            static mut c_0: [[f64; 2]; 12] = [
                [0.4189385332046727f64, 1.672820965066974e-17f64],
                [0.08333333333333333f64, 4.625929260796161e-18f64],
                [-0.002777777777777778f64, 1.0603069487776985e-19f64],
                [0.0007936507936507937f64, -3.454893292939205e-20f64],
                [-0.0005952380952380534f64, 1.8814933751331697e-20f64],
                [0.0008417508417169863f64, 4.911653741608501e-20f64],
                [-0.0019175268984354319f64, 7.942088679532008e-20f64],
                [0.00641024883889698f64, -2.0330649361327507e-19f64],
                [-0.02954854786102894f64, 8.876886560845568e-19f64],
                [0.17924105501963864f64, -9.280706593725197e-18f64],
                [-1.3413511488178156f64, 9.366147931333444e-17f64],
                [9.457154291794766f64, 6.116914469775277e-16f64],
            ];
            l1x = sumdd(
                l1x,
                l2x,
                c_0[0 as i32 as usize][0 as i32 as usize],
                c_0[0 as i32 as usize][1 as i32 as usize],
                &raw mut l2x,
            );
            fl = 0 as i32 as f64;
            fh = polydd(
                z2h,
                z2l,
                11 as i32,
                (&raw const c_0 as *const [f64; 2]).offset(1 as i32 as isize),
                &raw mut fl,
            );
        } else {
            static mut c_1: [[f64; 2]; 28] = [
                [0.4189385332046727f64, 1.6728209940438197e-17f64],
                [0.08333333333333333f64, 4.625917010004774e-18f64],
                [-0.002777777777777778f64, 1.1142307215788528e-19f64],
                [0.0007936507936507917f64, -3.490670047641909e-20f64],
                [-0.0005952380952375601f64, -2.3344497708203455e-20f64],
                [0.000841750841642076f64, -1.6757499862024916e-20f64],
                [-0.0019175269003183426f64, -5.375554271274795e-20f64],
                [0.0064102542380049059f64, 5.867092683217645e-20f64],
                [-0.02955043046010787f64, 5.214403484068196e-19f64],
                [0.17962541477817255f64, -1.142848881310379e-17f64],
                [-1.3910812534622386f64, 1.603595861173478e-17f64],
                [13.321074167875223f64, 3.7359778274202467e-16f64],
                [-152.5883380597707f64, -4.151860399378012e-15f64],
                [1999.6350586983845f64, -5.4683268712995566e-14f64],
                [-28324.846626880604f64, 1.5664786357776814e-12f64],
                [408403.4843907638f64, 2.5814195460152124e-11f64],
                [-5684770.399618489f64, -2.6525822835950689e-10f64],
                [73279167.85728674f64, 3.7143219739261024e-9f64],
                [-847685565.8820181f64, -4.152006789152812e-8f64],
                [8585291064.381741f64, -1.520919199447847e-7f64],
                [-74520273996.62067f64, -0.0000037497699499844039f64],
                [542850852317.694f64, 0.000003450914370211638f64],
                [-3241602145407.3057f64, -0.00020192392222927104f64],
                [15405010471045.64f64, 0.0006165357865898429f64],
                [-55905227385018.88f64, 0.0008937551527940107f64],
                [145263794834068.85f64, -0.0042458108787747608f64],
                [-240351091852282.35f64, 0.005571580213149154f64],
                [190066983824398.67f64, 0.008247277998023994f64],
            ];
            l1x = sumdd(
                l1x,
                l2x,
                c_1[0 as i32 as usize][0 as i32 as usize],
                c_1[0 as i32 as usize][1 as i32 as usize],
                &raw mut l2x,
            );
            fl = 0 as i32 as f64;
            fh = polydd(
                z2h,
                z2l,
                (::core::mem::size_of::<[[f64; 2]; 28]>() as usize)
                    .wrapping_div(::core::mem::size_of::<[f64; 2]>() as usize)
                    .wrapping_sub(1 as usize) as i32,
                (&raw const c_1 as *const [f64; 2]).offset(1 as i32 as isize),
                &raw mut fl,
            );
        }
        fh = muldd(zh, zl, fh, fl, &raw mut fl);
        l1x = sumdd(l1x, l2x, fh, fl, &raw mut l2x);
        l0x = fasttwosum(l0x, l1x, &raw mut l1x);
        l1x = fasttwosum(l1x, l2x, &raw mut l2x);
    } else {
        let mut wl_0: f64 = *xl - 0.5f64;
        l0 -= 1 as i32 as f64;
        l0x = l0 * xh;
        let mut l0xl_0: f64 = l0.fma(xh, -l0x);
        l1x = l1 * xh;
        let mut l1xl_0: f64 = l1.fma(xh, -l1x);
        l2x = l2 * xh;
        l1x = sumdd(l1x, l2x, l0xl_0, l1xl_0, &raw mut l2x);
        l1x = sumdd(l1x, l2x, l0 * wl_0, l1 * wl_0, &raw mut l2x);
    }
    *xl = l1x;
    *e = l2x;
    return l0x;
}

pub fn lgamma(x: f64) -> f64 {
    cr_lgamma(x).0
}

pub fn lgamma_r(x: f64) -> (f64, i32) {
    cr_lgamma(x)
}
