extern crate quick_xml;

const WIDTH: usize = 72;

fn print_p(s: &str) {
    let words: Vec<_> = s.split_whitespace().collect();

    let w: Vec<_> = words.iter().map(|word| word.chars().count() + 1).collect();
    let n = w.len();

    fn p(v: usize) -> usize {
        v * v * v
    }

    fn f(ws: usize, j: usize) -> usize {
        let w = WIDTH.saturating_sub(ws);
        if j > 1 {
            p(w / (j - 1)) * (j - 1)
        } else {
            p(w)
        }
    }

    let mut d = vec![p(WIDTH * 2); n + 1];
    let mut q = vec![(0, 0); n + 1];
    d[0] = 0;
    for i in 1..=n {
        let mut ws = 0;
        for j in 1..=i {
            ws += w[i - j];
            if ws - 1 > WIDTH {
                break;
            }
            let c = if i < n { d[i - j] + f(ws, j) } else { d[i - j] };
            if c < d[i] {
                d[i] = c;
                q[i] = (j, ws);
            }
        }
    }
    let mut number_per_line = Vec::new();
    let mut i = n - q[n].0;
    while i > 0 {
        let (j, ws) = q[i];
        number_per_line.push((j, WIDTH - (ws - j)));
        i = i.saturating_sub(j);
    }

    let mut words = words.into_iter();
    for (k, mut ws) in number_per_line.into_iter().rev() {
        print!("{}", words.next().unwrap());
        for i in 1..k {
            let a = ws / (k - i);
            print!("{: <1$}", "", a);
            print!("{}", words.next().unwrap());
            ws -= a;
        }
        println!();
    }
    for w in words {
        print!("{} ", w);
    }
    println!();
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = std::env::args().nth(1).expect("file name");

    let mut reader = quick_xml::Reader::from_file(file_name)?;
    reader.trim_text(true);

    let mut ns = Vec::new();
    let mut event_buffer = Vec::new();

    use quick_xml::events::Event;
    const FB_NS: &[u8] = b"http://www.gribuser.ru/xml/fictionbook/2.0";
    loop {
        match reader.read_namespaced_event(&mut event_buffer, &mut ns)? {
            (Some(FB_NS), Event::Start(ref e)) if e.local_name() == b"FictionBook" => break,
            (_, Event::Start(_)) => {
                return Err("bad root".into());
            }
            _ => (),
        }
        event_buffer.clear();
    }
    loop {
        match reader.read_namespaced_event(&mut event_buffer, &mut ns)? {
            (Some(FB_NS), Event::Start(ref e)) if e.local_name() == b"p" => loop {
                match reader.read_namespaced_event(&mut event_buffer, &mut ns)? {
                    (_, Event::Text(ref e)) => {
                        let t = e.unescaped()?;
                        print_p(&String::from_utf8_lossy(&t));
                    }
                    (Some(FB_NS), Event::End(ref e)) if e.local_name() == b"p" => break,
                    _ => (),
                }
                event_buffer.clear();
            },
            (_, Event::Eof) => break,
            (_, _) => {}
        }
        event_buffer.clear();
    }

    Ok(())
}
