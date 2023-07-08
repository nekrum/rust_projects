use std::io::{self, Write};

fn fibonacci_fractal(iterations: u32) -> Vec<u32> {
    let mut fractal: Vec<u32> = Vec::new();

    for i in 0..iterations {
        if i <= 1 {
            fractal.push(i);
        } else {
            let next_number = fractal[i as usize - 1] + fractal[i as usize - 2];
            fractal.push(next_number);
        }
    }
    fractal
}



fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    
    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

fn draw_tree(depth: u32, x: f64, y: f64, angle: f64, length: f64) {
    if depth == 0 {
        return;
    }

    let x2 = x + angle.cos() * length;
    let y2 = y + angle.sin() * length;

    draw_line(x, y, x2, y2);

    let new_length = length / (fibonacci(depth + 2) as f64); // Utilizamos el n-ésimo número de Fibonacci para determinar la longitud
    let new_angle1 = angle + 0.3; // Ángulo para la rama derecha
    let new_angle2 = angle - 0.3; // Ángulo para la rama izquierda

    draw_tree(depth - 1, x2, y2, new_angle1, new_length);
    draw_tree(depth - 1, x2, y2, new_angle2, new_length);
}

fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64) {
    // Código para dibujar una línea en la pantalla
    // Aquí puedes usar alguna biblioteca de gráficos o dibujo en Rust, como Piston, minifb, etc.
    // Como este es solo un ejemplo básico, utilizaremos la salida de la consola para simular el dibujo.

    let mut stdout = io::stdout();
    let _ = stdout.write_fmt(format_args!("Line: ({}, {}) -> ({}, {})\n", x1, y1, x2, y2));
    let _ = stdout.flush();
}

fn main() {
    // let iterations = 10;
    // let fractal = fibonacci_fractal(iterations);
    // for number in fractal {
    //     println!("{}", number);
    // }
    let depth = 10;
    let initial_length = 100.0;

    draw_tree(depth, 0.0, 0.0, 0.0, initial_length);
}
