extern crate quick_xml;

const WIDTH: usize = 72;

fn print_p(s: &str) {
    let words: Vec<_> = s.split_whitespace().collect();

    let w: Vec<_> = words.iter().map(|word| word.chars().count() + 1).collect();
    let n = w.len();

    fn p(v: usize) -> usize {
        v * v * v
    }

    fn f(spaces: usize, words: usize) -> usize {
        if words > 1 {
            p(spaces / (words - 1)) * (words - 1)
        } else {
            p(spaces)
        }
    }

    let mut d = Vec::with_capacity(n + 1);
    struct Line {
        words: usize,
        spaces: usize,
    }
    let mut recovery = Vec::with_capacity(n + 1);
    d.push(0);
    recovery.push(Line {
        words: 0,
        spaces: 0,
    });
    for i in 1..=n {
        let mut spaces = (WIDTH + 1).saturating_sub(w[i - 1]);
        d.push(d[i - 1] + if i < n { f(spaces, 1) } else { 0 });
        recovery.push(Line { words: 1, spaces });
        for j in 2..=i {
            let (left, overflow) = spaces.overflowing_sub(w[i - j]);
            if overflow {
                break;
            }
            spaces = left;
            let nd = d[i - j] + if i < n { f(spaces, j) } else { 0 };
            if nd < d[i] {
                d[i] = nd;
                recovery[i] = Line {
                    words: j,
                    spaces: spaces + j,
                };
            }
        }
    }
    let mut number_per_line = Vec::new();
    let mut i = n - recovery[n].words;
    while i > 0 {
        number_per_line.push(&recovery[i]);
        i = i.saturating_sub(recovery[i].words);
    }

    let mut words = words.into_iter();
    for line in number_per_line.into_iter().rev() {
        print!("{}", words.next().unwrap());
        let mut spaces = line.spaces;
        for i in 1..line.words {
            let a = spaces / (line.words - i);
            print!("{: <1$}", "", a);
            print!("{}", words.next().unwrap());
            spaces -= a;
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
