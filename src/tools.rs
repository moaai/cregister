use std::iter::FromIterator;

const COLUMNS: usize = 10;

pub fn calc_crc(data: &[u8], out: &mut [u8; 4]) {
    let hex_crc = format!(
        "{:0>4X}",
        data.iter()
            .enumerate()
            .fold(0, |crc, (i, elem)| { crc ^ (*elem as u32) << (i % 9) })
    )
    .chars()
    .map(|c| c as u8)
    .collect::<Vec<u8>>();

    out.clone_from_slice(&hex_crc);
}

/// # Safety
/// Single allocation object, not null data with constant width
#[allow(dead_code)]
pub unsafe fn struct_to_u8<T>(p: &T) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>())
}

pub fn ll_dump<F>(buf: &[u8], op: F)
where
    F: Fn(),
{
    if buf.len() < 6 {
        return;
    }

    op();
    //TODO Detect extendect packets, starting with '#'
    let st = String::from_iter(
        &buf[3..6]
            .iter()
            .copied()
            .map(|x| x as char)
            .collect::<Vec<char>>(),
    );

    eprintln!(
        "Packet: {:?} tpe: {} stpe: {}  dir: {} size: {} crc: {:?}",
        buf[1] as char,
        buf[2] as char,
        st,
        buf[6] as char,
        buf.len(),
        &buf[buf.len() - 4..]
    );

    eprint!("\t   ");
    for i in 0..COLUMNS {
        eprint!("{: >4} ", i);
    }
    eprintln!();
    for (i, chunk) in buf[..buf.len()].chunks(COLUMNS).enumerate() {
        let v: Vec<u8> = chunk
            .iter()
            .map(|v| if *v < 32 { b'.' } else { *v })
            .collect();
        let str: String = String::from_utf8_lossy(&v).to_string();
        let chunk_len = chunk.len();
        eprintln!(
            "\t{:0>2} {:0>3?} {:>width$}|{}|",
            i,
            chunk,
            "",
            str,
            width = 5 * (COLUMNS - chunk_len)
        );
    }
}
