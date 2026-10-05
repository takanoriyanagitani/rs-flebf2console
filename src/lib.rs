use std::io;

use io::BufWriter;
use io::Write;

use io::Read;

use flexbuffers::Buffer;
use flexbuffers::Reader;

pub fn flebuf2wtr<B, W>(flebuf: B, mut wtr: W) -> Result<(), io::Error>
where
    B: Buffer,
    W: Write,
{
    let rdr = Reader::get_root(flebuf).map_err(io::Error::other)?;
    writeln!(wtr, "{rdr}")?;
    wtr.flush()
}

pub const STDIN_LIMIT_DEFAULT: u64 = 65536;

pub fn stdin2flebuf2stdout(limit: u64) -> Result<(), io::Error> {
    let rdr = io::stdin().lock();
    let mut taken = rdr.take(limit);
    let mut buf: Vec<u8> = vec![];
    taken.read_to_end(&mut buf)?;

    let o = io::stdout();
    let mut ol = o.lock();
    let s: &[u8] = &buf;
    flebuf2wtr(s, BufWriter::new(&mut ol))?;
    ol.flush()
}

pub fn stdin2flebuf2stdout_default() -> Result<(), io::Error> {
    stdin2flebuf2stdout(STDIN_LIMIT_DEFAULT)
}
