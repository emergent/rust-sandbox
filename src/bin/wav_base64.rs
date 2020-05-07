use byteorder::{ByteOrder, LittleEndian};
use hound::WavWriter;
use std::fs::File;
use std::io::{BufReader, Read};

fn wav_header2() -> std::io::Result<()> {
    let in_file_name = "audio.raw";
    let f = File::open(in_file_name)?;
    let mut reader = BufReader::new(f);
    let mut out_cur = std::io::Cursor::new(Vec::new());
    let mut in_buf = [0u8; 2];

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut wav_writer = WavWriter::new(&mut out_cur, spec).unwrap();
    while let Ok(n) = reader.read(&mut in_buf) {
        match n {
            0 => break,
            _ => {
                wav_writer
                    .write_sample(LittleEndian::read_i16(&in_buf))
                    .unwrap_or(());
                print!("{:b} {:b}", in_buf[0], in_buf[1]);
            }
        }
    }
    wav_writer.finalize().unwrap_or(());

    let out_b64 = base64::encode(out_cur.get_ref());
    println!("out_b64: {}", out_b64);
    Ok(())
}

fn main() -> std::io::Result<()> {
    wav_header2()
}
