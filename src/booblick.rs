use crate::vga_buffer::{Buffer, Color, ColorCode, ScreenChar, VGA_WRITER};
use micromath::F32Ext;

const COLOR_CODE: ColorCode = ColorCode(((Color::Black as u8) << 4) | Color::Yellow as u8);

fn r(mul: i32, shift: i32, x: &mut i32, y: &mut i32) -> i32 {
    let temp = *x;
    *x -= mul * *y >> shift;
    *y += mul * temp >> shift;
    let temp = 3145728 - *x * *x - *y * *y >> 11;
    *x = *x * temp >> 10;
    *y = *y * temp >> 10;
    temp
}

pub fn booblick() {
    let (mut A, mut B): (f32, f32) = (0.0, 0.0);

    loop {
        A += 0.07;
        B += 0.03;
        let ((sinA, cosA), (sinB, cosB), mut b) = (
            A.sin_cos(),
            B.sin_cos(),
            [ScreenChar {
                ascii_character: b' ',
                color_code: COLOR_CODE,
            }; 2000],
        );

        let (mut z, mut j): ([f32; 1760], f32) = ([0.0; 1760], 0.0);
        while j <= 6.28 {
            let (u, v) = j.sin_cos();
            let mut i: f32 = 0.0;
            while i <= 6.28 {
                let (w, c) = i.sin_cos();
                let h = v + 2.0;
                let (d, t) = (
                    1.0 / (w * h * sinA + u * cosA + 5.0),
                    w * h * cosA - u * sinA,
                );
                let (x, y) = (
                    (40.0 + 30.0 * d * (c * h * cosB - t * sinB)) as usize,
                    (12.0 + 15.0 * d * (c * h * sinB + t * cosB)) as usize,
                );
                let (o, n) = (
                    x + 80 * y,
                    8.0 * ((u * sinA - w * v * cosA) * cosB
                        - w * v * sinA
                        - u * cosA
                        - c * v * sinB),
                );
                if y < 22 && x < 79 && d > z[o] {
                    z[o] = d;
                    b[o] = (".,-~:;=!*#$@")
                        .chars()
                        .nth(n as usize)
                        .or(Some('.'))
                        .map(|x| ScreenChar {
                            ascii_character: x as u8,
                            color_code: COLOR_CODE,
                        })
                        .unwrap();
                }
                i += 0.02
            }
            j += 0.07
        }

        use x86_64::instructions::interrupts::without_interrupts;

        without_interrupts(|| {
            unsafe { *VGA_WRITER.lock().buffer = core::mem::transmute(b) };
        });
        // print!("{}", String::<2100>::from_iter(b.iter()));
        // print!(
        //     "\x1B[H{}",
        //     b.chunks(80)
        //         .map(|l| l.iter().collect::<String<80>>())
        //         .collect::<Vec<String<80>, 25>>()
        //         .join("\n")
        // );
    }
}

pub fn booblick_fast() {
    let mut b = [ScreenChar {
        ascii_character: b' ',
        color_code: COLOR_CODE,
    }; 2000];
    let mut z = [127i8; 2000];
    let (mut sa, mut ca, mut sb, mut cb) = (1024i32, 0i32, 1024i32, 0i32);

    // print!("\x1B[2J");

    // let mut frame_count = 0;

    loop {
        for i in b.iter_mut() {
            i.ascii_character = 32; // Clearing the text buffer
        }
        for i in z.iter_mut() {
            *i = 127; // Clearing the z buffer
        }

        let (mut sj, mut cj) = (0i32, 1024i32);
        for _j in 0..90 {
            let (mut si, mut ci) = (0i32, 1024i32);
            for _i in 0..324 {
                let r1 = 1;
                let r2 = 2048;
                let k2 = 5120 * 1024;
                let x0 = r1 * cj + r2;
                let x1 = ci * x0 >> 10;
                let x2 = ca * sj >> 10;
                let x3 = si * x0 >> 10;
                let x4 = r1 * x2 - (sa * x3 >> 10);
                let x5 = sa * sj >> 10;
                let x6 = k2 + r1 * 1024 * x5 + ca * x3;
                let x7 = cj * si >> 10;
                let x = 40 + 30 * (cb * x1 - sb * x4) / x6;
                let y = 12 + 15 * (cb * x4 + sb * x1) / x6;
                let n = (-ca * x7 - cb * ((-sa * x7 >> 10) + x2) - ci * (cj * sb >> 10) >> 10) - x5
                    >> 7;
                let o = x + 80 * y;
                let zz = ((x6 - k2) >> 15) as i8;
                if 22 > y && y > 0 && x > 0 && 80 > x && zz < z[o as usize] {
                    z[o as usize] = zz;
                    b[o as usize].ascii_character = b".,-~:;=!*#$@"[n.max(0) as usize];
                }
                r(5, 8, &mut ci, &mut si);
            }
            r(9, 7, &mut cj, &mut sj);
        }

        r(5, 7, &mut ca, &mut sa);
        r(5, 8, &mut cb, &mut sb);

        use x86_64::instructions::interrupts::without_interrupts;

        without_interrupts(|| {
            unsafe { *VGA_WRITER.lock().buffer = core::mem::transmute(b) };
        });

        for _ in 0..3_000_000 {
            x86_64::instructions::nop();
        }
    }
}
