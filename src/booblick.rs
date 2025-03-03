use crate::vga_buffer::{Buffer, Color, ColorCode, ScreenChar, VGA_WRITER};
use micromath::F32Ext;

const COLOR_CODE: ColorCode = ColorCode(((Color::Black as u8) << 4) | Color::Yellow as u8);

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
