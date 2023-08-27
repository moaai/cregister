/// Various i18n stuff.

/// Encoding crate does not have cp852 which is used by Novitus Next.
/// Polish characters are encoded manually to win1250 first and
/// sent to encoding crate.

/// Converts CP852 Polish diactric characters to win1250
pub(crate) fn cp852_to_win1250(name: &[u8]) -> Vec<u8> {
    let v: Vec<u8> = name
        .iter()
        .map(|&x| {
            match x {
                0xA5 => 0xB9, //ą
                0x86 => 0xE6, //ć
                0xA9 => 0xEA, //ę
                0x88 => 0xB3, //ł
                0xE4 => 0xF1, //ń
                0xA2 => 0xF3, //ó
                0x98 => 0x9C, //ś
                0xAB => 0x9F, //ź
                0xBE => 0xBF, //ż
                0xA4 => 0xA5, //Ą
                0x8F => 0xC6, //Ć
                0xA8 => 0xCA, //Ę
                0x9D => 0xA3, //Ł
                0xE3 => 0xD1, //Ń
                0xE0 => 0xD3, //Ó
                0x97 => 0x8C, //Ś
                0x8D => 0x8F, //Ź
                0xBD => 0xAF, //Ż

                0x8E => 0xC4, //Ä
                0x81 => 0xFC, //ü
                _ => x,
            }
        })
        .collect(); //.collect::<Vec<u8>>();

    v
}

/// Converts win1250 encoded Polish characters into CP852
// pub(crate) fn _win1250_to_cp852(name: &[u8; 40]) -> Vec<u8> {
pub(crate) fn _win1250_to_cp852(name: &[u8]) -> Vec<u8> {
    let v: Vec<u8> = name
        .iter()
        .map(|&x| {
            match x {
                0xB9 => 0xA5, //ą
                0xE6 => 0x86, //ć
                0xEA => 0xA9, //ę
                0xB3 => 0x88, //ł
                0xF1 => 0xE4, //ń
                0xF3 => 0xA2, //ó
                0x9C => 0x98, //ś
                0x9F => 0xAB, //ź
                0xBF => 0xBE, //ż
                0xA5 => 0xA4, //Ą
                0xC6 => 0x8F, //Ć
                0xCA => 0xA8, //Ę
                0xA3 => 0x9D, //Ł
                0xD1 => 0xE3, //Ń
                0xD3 => 0xE0, //Ó
                0x8C => 0x97, //Ś
                0x8F => 0x8D, //Ź
                0xAF => 0xBD, //Ż

                0xC4 => 0x8E, //Ä
                0xFC => 0x81, //ü
                _ => x,
            }
        })
        .collect(); //.collect::<Vec<u8>>();

    v
}
