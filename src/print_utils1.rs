// Copyright (c) 2019 10X Genomics, Inc. All rights reserved.

use ansi_escape::*;
use defs::*;
use io_utils::*;
use itertools::*;
use std::cmp::max;
use std::collections::HashMap;
use std::io::Write;
use string_utils::*;
use tables::*;
use vdj_ann::refx::*;
use vector_utils::*;

// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

pub fn make_table(
    ctl: &EncloneControl,
    rows: &mut Vec<Vec<String>>,
    justify: &Vec<u8>,
    mlog: &Vec<u8>,
    logz: &mut String,
) {
    // In plain mode, strip escape characters.

    if !ctl.pretty {
        for i in 0..rows.len() {
            for j in 0..rows[i].len() {
                let mut x = Vec::<u8>::new();
                let mut escaped = false;
                let s = rows[i][j].as_bytes();
                for l in 0..s.len() {
                    if s[l] == b'' {
                        escaped = true;
                    }
                    if escaped {
                        if s[l] == b'm' {
                            escaped = false;
                        }
                        continue;
                    }
                    x.push(s[l]);
                }
                rows[i][j] = stringme(&x);
            }
        }
    }

    // Make table.

    let log0 = stringme(&mlog);
    let mut log = String::new();
    if ctl.debug_table_printing {
        for i in 0..rows.len() {
            println!("");
            for j in 0..rows[i].len() {
                println!(
                    "row = {}, col = {}, entry = {}, vis width = {}",
                    i,
                    j,
                    rows[i][j],
                    visible_width(&rows[i][j])
                );
            }
        }
        println!("");
    }
    print_tabular_vbox(&mut log, &rows, 2, &justify, ctl.debug_table_printing);
    if ctl.debug_table_printing {
        println!("{}", log);
    }
    let mut cs = vec![Vec::<char>::new(); rows.len() + 2];
    let mut row = 0;
    for c in log.chars() {
        if c == '\n' {
            row += 1;
        } else {
            cs[row].push(c);
        }
    }
    log = log0;

    // Process each row.

    for i in 0..cs.len() {
        for j in 0..cs[i].len() {
            log.push(cs[i][j]);
        }
        log.push('\n');
    }

    // Make some character substitutions.

    for c in log.chars() {
        if c == '$' {
            logz.push('•');
        } else if c == '+' {
            logz.push('◼');
        } else if c == '%' {
            logz.push('+');
        } else {
            logz.push(c);
        }
    }
}

// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

pub fn emit_codon_color_escape(c: &[u8], log: &mut Vec<u8>) {
    let mut s = 0;
    if c == b"CTG" {
        s = 3;
    } else if c == b"AGG" {
        s = 1;
    } else if c == b"AGT" {
        s = 2;
    } else {
        for i in 0..3 {
            if c[i] == b'A' {
            } else if c[i] == b'C' {
                s += 1;
            } else if c[i] == b'G' {
                s += 2;
            } else if c[i] == b'T' {
                s += 3;
            } else {
                panic!("Illegal codon: \"{}\".", strme(&c));
            }
        }
    }
    let s = s % 6;
    print_color(s, log);
}
// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

pub fn print_digit(p: usize, i: usize, digits: usize, ds: &mut String) {
    if digits == 1 {
        *ds += &format!("{}", p);
    } else if digits == 2 {
        if i == 0 {
            if p >= 10 {
                *ds += &format!("{}", p / 10);
            } else {
                ds.push(' ');
            }
        } else {
            *ds += &format!("{}", p % 10);
        }
    } else {
        if i == 0 {
            if p >= 100 {
                *ds += &format!("{}", p / 100);
            } else {
                ds.push(' ');
            }
        } else if i == 1 {
            if p >= 10 {
                *ds += &format!("{}", (p % 100) / 10);
            } else {
                ds.push(' ');
            }
        } else {
            *ds += &format!("{}", p % 10);
        }
    }
}

// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

// Determine the number of digits in a nonnegative integer.

pub fn ndigits(n: usize) -> usize {
    let mut d = 1;
    let mut x = n;
    while x >= 10 {
        d += 1;
        x /= 10;
    }
    d
}

// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

pub fn make_diff_row(
    ctl: &EncloneControl,
    rsi: &ColInfo,
    cols: usize,
    diff_pos: usize,
    drows: &Vec<Vec<String>>,
    row1: &mut Vec<String>,
    rows: &mut Vec<Vec<String>>,
) {
    let nc = row1.len();
    let cvars = &ctl.clono_print_opt.cvars;
    if !ctl.clono_print_opt.amino.is_empty() || cvars.contains(&"var".to_string()) {
        let mut ncall = 0;
        for j in 0..cols {
            for z in 0..rsi.cvars[j].len() {
                let mut c = Vec::<Vec<u8>>::new();
                let mut start = 5 + drows.len();
                if drows.len() >= 1 {
                    start += 2;
                }
                for k in start..rows.len() {
                    if rows[k][0].starts_with("$") {
                        continue;
                    }
                    if rows[k][ncall + z + nc].len() > 0 {
                        c.push(rows[k][ncall + z + nc].as_bytes().to_vec());
                    }
                }

                // Package characters with ANSI escape codes that come before them.
                // The is a dorky way of identifying codons that are different, by
                // virtue of them being shown as colored amino acids.

                let mut c2 = Vec::<Vec<Vec<u8>>>::new();
                for i in 0..c.len() {
                    c2.push(package_characters_with_escapes(&c[i]));
                }

                // Proceed.

                if (rsi.cvars[j][z] != "amino" && rsi.cvars[j][z] != "var") || c.len() == 0 {
                    row1.push("".to_string());
                    continue;
                }
                let mut dots = Vec::<u8>::new();
                for m in 0..c2[0].len() {
                    let mut digits_or_blanks = true;
                    for l in 0..c2.len() {
                        if c2[l].is_empty() {
                            // needed?
                            continue;
                        }
                        if !(c2[l][m] == b" ".to_vec()
                            || (c2[l][m] >= b"0".to_vec() && c2[l][m] <= b"9".to_vec()))
                        {
                            digits_or_blanks = false;
                        }
                        if c2[l][m].contains(&b' ') {
                            digits_or_blanks = true;
                        }
                    }
                    let mut same = true;
                    for l in 1..c2.len() {
                        if c2[l].is_empty() {
                            // needed?
                            continue;
                        }
                        if c2[l][m] != c2[0][m] {
                            same = false;
                        }
                    }
                    let mut sep = true;
                    for l in 0..c2.len() {
                        if c2[l].is_empty() {
                            // needed?
                            continue;
                        }
                        if c2[l][m] != b"|" {
                            sep = false;
                        }
                    }
                    if sep {
                    } else if digits_or_blanks {
                        dots.push(b' ');
                    } else if same {
                        dots.push(b'.');
                    } else {
                        dots.push(b'x');
                    }
                }
                row1.push(format!("{}", strme(&dots)));
            }
            ncall += rsi.cvars[j].len();
        }
        rows[diff_pos] = row1.to_vec();
    } else {
        for i in 0..row1.len() {
            rows[diff_pos - 1][i] = row1[i].clone();
        }
    }
}

// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

pub fn set_speakers(ctl: &EncloneControl, parseable_fields: &mut Vec<String>) {
    // Make some abbreviations.

    let cvars = &ctl.clono_print_opt.cvars;
    let lvars = &ctl.clono_print_opt.lvars;

    // Define parseable output columns.  The entire machinery for parseable output is controlled
    // by macros that begin with "speak".

    let pcols_sort = &ctl.parseable_opt.pcols_sort;
    macro_rules! speaker {
        ($var:expr) => {
            if ctl.parseable_opt.pcols.is_empty() || bin_member(&pcols_sort, &$var.to_string()) {
                parseable_fields.push($var.to_string());
            }
        };
    }
    macro_rules! speakerc {
        ($col:expr, $var:expr) => {
            let varc = format!("{}{}", $var, $col + 1);
            if ctl.parseable_opt.pcols.is_empty() || bin_member(&pcols_sort, &varc) {
                parseable_fields.push(format!("{}{}", $var, $col + 1));
            }
        };
    }
    for x in lvars.iter() {
        speaker!(x);
    }
    for col in 0..ctl.parseable_opt.pchains {
        for x in cvars.iter() {
            speakerc!(col, x);
        }
        for x in &["v_name", "d_name", "j_name", "v_id", "d_id", "j_id"] {
            speakerc!(col, x);
        }
        for x in &[
            "var_indices_dna",
            "var_indices_aa",
            "share_indices_dna",
            "share_indices_aa",
        ] {
            speakerc!(col, x);
        }
        for x in &[
            "v_start",
            "const_id",
            "utr_id",
            "utr_name",
            "cdr3_start",
            "cdr3_aa",
        ] {
            speakerc!(col, x);
        }
        for x in &["seq", "vj_seq", "var_aa"] {
            speakerc!(col, x);
        }
        for i in 0..pcols_sort.len() {
            if pcols_sort[i].starts_with('q') && pcols_sort[i].ends_with(&format!("_{}", col + 1)) {
                let x = pcols_sort[i].after("q").rev_before("_");
                if x.parse::<usize>().is_ok() {
                    parseable_fields.push(pcols_sort[i].clone());
                }
            }
        }
    }
    speaker!("group_id");
    speaker!("group_ncells");
    speaker!("clonotype_id");
    speaker!("clonotype_ncells");
    speaker!("nchains");
    speaker!("exact_subclonotype_id");
    speaker!("barcodes");
    let mut pfsort = parseable_fields.clone();
    unique_sort(&mut pfsort);
    for x in pcols_sort.iter() {
        if !bin_member(&pfsort, x) {
            eprintln!("\nUnknown parseable output field: {}.\n", x);
            std::process::exit(1);
        }
    }
}

// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

// Start to generate parseable output.  And other things.

pub fn start_gen(
    ctl: &EncloneControl,
    exacts: &Vec<usize>,
    exact_clonotypes: &Vec<ExactClonotype>,
    refdata: &RefData,
    rsi: &ColInfo,
    out_data: &mut Vec<HashMap<String, String>>,
    mut mlog: &mut Vec<u8>,
) {
    let pcols_sort = &ctl.parseable_opt.pcols_sort;
    macro_rules! speak {
        ($u:expr, $var:expr, $val:expr) => {
            if ctl.parseable_opt.pout.len() > 0 {
                if pcols_sort.is_empty() || bin_member(&pcols_sort, &$var.to_string()) {
                    out_data[$u].insert($var.to_string(), $val);
                }
            }
        };
    }
    macro_rules! speakc {
        ($u:expr, $col:expr, $var:expr, $val:expr) => {
            if ctl.parseable_opt.pout.len() > 0 && $col + 1 <= ctl.parseable_opt.pchains {
                let varc = format!("{}{}", $var, $col + 1);
                if pcols_sort.is_empty() || bin_member(&pcols_sort, &varc) {
                    out_data[$u].insert(varc, format!("{}", $val));
                }
            }
        };
    }
    let nexacts = exacts.len();
    let mut n = 0;
    for u in 0..nexacts {
        n += exact_clonotypes[exacts[u]].ncells();
    }
    if ctl.parseable_opt.pout.len() > 0 {
        *out_data = vec![HashMap::<String, String>::new(); nexacts];
    }
    let cols = rsi.vids.len();
    let mut ncells = 0;
    for u in 0..exacts.len() {
        ncells += exact_clonotypes[exacts[u]].ncells();
    }
    for u in 0..exacts.len() {
        speak!(u, "nchains", format!("{}", cols));
        speak!(u, "clonotype_ncells", format!("{}", ncells));
        let mut bc = Vec::<String>::new();
        for x in exact_clonotypes[exacts[u]].clones.iter() {
            bc.push(x[0].barcode.clone());
        }
        bc.sort();
        speak!(u, "barcodes", format!("{}", bc.iter().format(",")));
        for cx in 0..cols {
            let vid = rsi.vids[cx];
            speakc!(u, cx, "v_name", refdata.name[vid]);
            speakc!(u, cx, "v_id", refdata.id[vid]);
            let did = rsi.dids[cx];
            if did.is_some() {
                let did = did.unwrap();
                speakc!(u, cx, "d_name", refdata.name[did]);
                speakc!(u, cx, "d_id", refdata.id[did]);
            }
            let jid = rsi.jids[cx];
            speakc!(u, cx, "j_name", refdata.name[jid]);
            speakc!(u, cx, "j_id", refdata.id[jid]);
        }
    }

    // Start to print the clonotype.

    let mut donors = Vec::<usize>::new();
    for u in 0..exacts.len() {
        let ex = &exact_clonotypes[exacts[u]];
        for m in 0..ex.clones.len() {
            let lena = ex.clones[m][0].lena_index;
            donors.push(ctl.sample_info.donor_index[lena]);
        }
    }
    unique_sort(&mut donors);
    fwriteln!(&mut mlog, "CLONOTYPE = {} CELLS", n);
    if donors.len() > 1 {
        fwriteln!(
            &mut mlog,
            "🔴 WARNING: This clonotype contains cells from multiple donors."
        );
        for i in 0..donors.len() {
            let mut lenas = Vec::<String>::new();
            for u in 0..nexacts {
                let ex = &exact_clonotypes[exacts[u]];
                for l in 0..ex.clones.len() {
                    let li = ex.clones[l][0].lena_index;
                    if ctl.sample_info.donor_index[li] == donors[i] {
                        lenas.push(ctl.sample_info.dataset_id[li].clone());
                    }
                }
            }
            unique_sort(&mut lenas);
            fwriteln!(&mut mlog, "donor {}: {}", i + 1, lenas.iter().format(","));
        }
    }

    // Print barcodes.

    if ctl.clono_print_opt.barcodes {
        let mut bc = Vec::<String>::new();
        for u in 0..nexacts {
            let ex = &exact_clonotypes[exacts[u]];
            for l in 0..ex.clones.len() {
                bc.push(ex.clones[l][0].barcode.clone());
            }
        }
        unique_sort(&mut bc);
        fwriteln!(&mut mlog, "• {}", bc.iter().format(","));
    }
}

// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

pub fn insert_position_rows(
    rsi: &ColInfo,
    show_aa: &Vec<Vec<usize>>,
    vars: &Vec<Vec<usize>>,
    row1: &Vec<String>,
) -> Vec<Vec<String>> {
    let cols = rsi.cdr3_starts.len();
    let mut drows = Vec::<Vec<String>>::new();
    let mut digits = 0;
    for zpass in 1..=2 {
        if zpass == 2 {
            drows = vec![vec![String::new(); row1.len()]; digits];
        }
        for cx in 0..cols {
            let cs = rsi.cdr3_starts[cx] / 3;
            let n = rsi.cdr3_lens[cx];
            for m in 0..rsi.cvars[cx].len() {
                if zpass == 1 {
                    if rsi.cvars[cx][m] == "amino".to_string() {
                        for p in show_aa[cx].iter() {
                            digits = max(digits, ndigits(*p));
                        }
                    } else if rsi.cvars[cx][m] == "var".to_string() {
                        for p in vars[cx].iter() {
                            digits = max(digits, ndigits(*p));
                        }
                    }
                } else {
                    for i in 0..digits {
                        if rsi.cvars[cx][m] == "amino".to_string() {
                            let mut ds = String::new();
                            for (j, p) in show_aa[cx].iter().enumerate() {
                                if j > 0 && *p == cs {
                                    ds += " ";
                                }
                                print_digit(*p, i, digits, &mut ds);
                                if j < show_aa[cx].len() - 1 && *p == cs + n - 1 {
                                    ds += " ";
                                }
                            }
                            drows[i].push(ds);
                        } else if rsi.cvars[cx][m] == "var".to_string() {
                            let mut ds = String::new();
                            for p in vars[cx].iter() {
                                print_digit(*p, i, digits, &mut ds);
                            }
                            drows[i].push(ds);
                        } else {
                            drows[i].push(String::new());
                        }
                    }
                }
            }
        }
    }
    drows
}