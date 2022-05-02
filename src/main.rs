use clap::Parser;
use needletail::Sequence;
use std::io::Write;

use sdust::symmetric_dust;

const DUSTMASKER_LINE_LEN: usize = 60;

#[derive(Copy, Clone, Debug, Eq, PartialEq, clap::ArgEnum)]
enum OutputFormat {
    /// Masked bases are lowercased
    Soft,
    /// Masked bases are replaced with Ns
    Hard,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Soft
    }
}

/// SDustMasker takes its input as a FASTA formatted file containing one
/// or more nucleotide sequences. It then identifies and masks out the
/// low complexity parts of the input sequences.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file. Use `-` for stdin
    #[clap(short, long)]
    input: String,
    /// Output file. Use `-` for stdout
    #[clap(short, long)]
    output: String,
    #[clap(short, long, arg_enum)]
    out_format: OutputFormat,
}

fn main() {
    let args = Args::parse();
    let mut fastx_reader = if args.input == "-" {
        needletail::parse_fastx_stdin().expect("Impossible to read from stdin")
    } else {
        needletail::parse_fastx_file(&args.input).expect("Impossible to open file")
    };

    let mut out_file = std::io::BufWriter::new(
        std::fs::File::create(args.output).expect("Impossible to create file"),
    );
    let mut past_first_record = false;
    while let Some(record) = fastx_reader.next() {
        let seqrec = record.unwrap();
        if past_first_record {
            out_file.write_all(b"\n").expect("write to disk");
        } else {
            past_first_record = true;
        }
        out_file.write_all(b">").expect("write to disk");
        out_file.write_all(seqrec.id()).expect("write to disk");
        out_file.write_all(b"\n").expect("write to disk");
        let norm_seq = seqrec.normalize(false);

        let mut intervals = symmetric_dust(&norm_seq);
        intervals.reverse();

        if intervals.is_empty() {
            out_file
                .write_all(seqrec.sequence())
                .expect("write to disk");
        } else {
            let mut current = intervals.pop();
            for (i, b) in norm_seq.iter().enumerate() {
                if i != 0 && i % DUSTMASKER_LINE_LEN == 0 {
                    out_file.write_all(&[b'\n']).expect("write to disk");
                }
                let mut mask = false;
                if let Some(curr) = &current {
                    if i > curr.end {
                        current = intervals.pop();
                    } else if curr.contains(&i) {
                        mask = true;
                    }
                }

                match (args.out_format, mask, b) {
                    (OutputFormat::Soft, true, b'A') => out_file.write_all(&[b'a']),
                    (OutputFormat::Soft, true, b'T') => out_file.write_all(&[b't']),
                    (OutputFormat::Soft, true, b'C') => out_file.write_all(&[b'c']),
                    (OutputFormat::Soft, true, b'G') => out_file.write_all(&[b'g']),
                    (OutputFormat::Hard, true, _) => out_file.write_all(&[b'N']),
                    (_, _, _) => out_file.write_all(&[*b]),
                }
                .expect("write to disk");
            }
        }
    }
    out_file.write_all(&[b'\n']).expect("write to disk");
}
