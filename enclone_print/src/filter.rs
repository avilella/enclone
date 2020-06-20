// Copyright (c) 2020 10X Genomics, Inc. All rights reserved.

// Test a clonotype to see if it passes the filters.
// See also enclone_core src for a list of these filters and
// the related struct.

use vdj_ann::*;

use self::refx::*;
use enclone_core::defs::*;
use std::cmp::*;
use string_utils::*;
use vector_utils::*;

pub fn survives_filter(
    exacts: &Vec<usize>,
    rsi: &ColInfo,
    ctl: &EncloneControl,
    exact_clonotypes: &Vec<ExactClonotype>,
    refdata: &RefData,
    gex_info: &GexInfo,
) -> bool {
    let mut mults = Vec::<usize>::new();
    for i in 0..exacts.len() {
        mults.push(exact_clonotypes[exacts[i]].clones.len());
    }
    let n: usize = mults.iter().sum();
    if n == 0 {
        return false;
    }
    // Clonotypes with at least n cells
    if n < ctl.clono_filt_opt.ncells_low {
        return false;
    }
    // Clonotypes marked by heuristics
    if ctl.clono_filt_opt.marked {
        let mut marked = false;
        for s in exacts.iter() {
            let ex = &exact_clonotypes[*s];
            for i in 0..ex.clones.len() {
                if ex.clones[i][0].marked {
                    marked = true;
                }
            }
        }
        if !marked {
            return false;
        }
    }
    // Marked clonotypes which are also B cells by annotation
    if ctl.clono_filt_opt.marked_b {
        let mut marked_b = false;
        for s in exacts.iter() {
            let ex = &exact_clonotypes[*s];
            for i in 0..ex.clones.len() {
                if ex.clones[i][0].marked {
                    let li = ex.clones[i][0].dataset_index;
                    let bc = &ex.clones[i][0].barcode;
                    if gex_info.cell_type[li].contains_key(&bc.clone()) {
                        if gex_info.cell_type[li][&bc.clone()].starts_with('B') {
                            marked_b = true;
                        }
                    }
                }
            }
        }
        if !marked_b {
            return false;
        }
    }
    let cols = rsi.vids.len();
    // Barcode required
    if ctl.clono_filt_opt.barcode.len() > 0 {
        let mut ok = false;
        for s in exacts.iter() {
            let ex = &exact_clonotypes[*s];
            for i in 0..ex.clones.len() {
                for j in 0..ctl.clono_filt_opt.barcode.len() {
                    if ex.clones[i][0].barcode == ctl.clono_filt_opt.barcode[j] {
                        ok = true;
                    }
                }
            }
        }
        if !ok {
            return false;
        }
    }
    // Clonotypes with deletions
    if ctl.clono_filt_opt.del {
        let mut ok = false;
        for s in exacts.iter() {
            let ex = &exact_clonotypes[*s];
            for m in 0..ex.share.len() {
                if ex.share[m].seq_del.contains(&b'-') {
                    ok = true;
                }
            }
        }
        if !ok {
            return false;
        }
    }
    // Clonotypes with same V gene in 2 chains
    if ctl.clono_filt_opt.vdup {
        let mut dup = false;
        let mut x = rsi.vids.clone();
        x.sort();
        let mut i = 0;
        while i < x.len() {
            let j = next_diff(&x, i);
            if j - i > 1 {
                dup = true;
            }
            i = j;
        }
        if !dup {
            return false;
        }
    }
    // Clonotypes with constant region differences
    if ctl.clono_filt_opt.cdiff {
        let mut cdiff = false;
        for s in exacts.iter() {
            let ex = &exact_clonotypes[*s];
            for m in 0..ex.share.len() {
                let cstart = ex.share[m].j_stop;
                let clen = ex.share[m].full_seq.len() - cstart;
                let cid = ex.share[m].c_ref_id;
                if cid.is_some() {
                    let r = &refdata.refs[cid.unwrap()];
                    for i in 0..min(clen, r.len()) {
                        let tb = ex.share[m].full_seq[cstart + i];
                        let rb = r.to_ascii_vec()[i];
                        if tb != rb {
                            cdiff = true;
                        }
                    }
                }
            }
        }
        if !cdiff {
            return false;
        }
    }
    // Clonotypes with onesie exact subclonotypes
    if ctl.clono_filt_opt.have_onesie {
        let mut have = false;
        for i in 0..exacts.len() {
            if exact_clonotypes[exacts[i]].share.len() == 1 {
                have = true;
            }
        }
        if !have {
            return false;
        }
    }
    // Clonotypes with full length V..J
    if !ctl.clono_filt_opt.vj.is_empty() {
        let mut have_vj = false;
        for s in exacts.iter() {
            let ex = &exact_clonotypes[*s];
            for j in 0..ex.share.len() {
                if ex.share[j].seq == ctl.clono_filt_opt.vj {
                    have_vj = true;
                }
            }
        }
        if !have_vj {
            return false;
        }
    }
    // Clonotypes with no more than n cells
    if n > ctl.clono_filt_opt.ncells_high {
        return false;
    }
    // Clonotypes with at least n chains
    if exacts.len() < ctl.clono_filt_opt.min_exacts {
        return false;
    }
    // Clonotypes with given V gene name
    for i in 0..ctl.clono_filt_opt.seg.len() {
        let mut hit = false;
        for j in 0..ctl.clono_filt_opt.seg[i].len() {
            for cx in 0..cols {
                if refdata.name[rsi.vids[cx]] == ctl.clono_filt_opt.seg[i][j] {
                    hit = true;
                }
                let did = rsi.dids[cx];
                if did.is_some() {
                    let did = did.unwrap();
                    if refdata.name[did] == ctl.clono_filt_opt.seg[i][j] {
                        hit = true;
                    }
                }
                if refdata.name[rsi.jids[cx]] == ctl.clono_filt_opt.seg[i][j] {
                    hit = true;
                }
                if rsi.cids[cx].is_some() {
                    if refdata.name[rsi.cids[cx].unwrap()] == ctl.clono_filt_opt.seg[i][j] {
                        hit = true;
                    }
                }
            }
        }
        if !hit {
            return false;
        }
    }
    // Clonotypes with given V gene number/allele
    for i in 0..ctl.clono_filt_opt.segn.len() {
        let mut hit = false;
        for j in 0..ctl.clono_filt_opt.segn[i].len() {
            for cx in 0..cols {
                if refdata.id[rsi.vids[cx]] == ctl.clono_filt_opt.segn[i][j].force_i32() {
                    hit = true;
                }
                let did = rsi.dids[cx];
                if did.is_some() {
                    let did = did.unwrap();
                    if refdata.id[did] == ctl.clono_filt_opt.segn[i][j].force_i32() {
                        hit = true;
                    }
                }
                if refdata.id[rsi.jids[cx]] == ctl.clono_filt_opt.segn[i][j].force_i32() {
                    hit = true;
                }
                if rsi.cids[cx].is_some() {
                    if refdata.id[rsi.cids[cx].unwrap()]
                        == ctl.clono_filt_opt.segn[i][j].force_i32()
                    {
                        hit = true;
                    }
                }
            }
        }
        if !hit {
            return false;
        }
    }
    // Clonotypes with at least n cells
    if mults.iter().sum::<usize>() < ctl.clono_filt_opt.ncells_low {
        return false;
    }
    let mut numi = 0;
    for i in 0..exacts.len() {
        let ex = &exact_clonotypes[exacts[i]];
        numi = max(numi, ex.max_umi_count());
    }
    // Clonotypes with at least n UMIs for contig
    if numi < ctl.clono_filt_opt.min_umi {
        return false;
    }
    let mut lis = Vec::<usize>::new();
    for s in exacts.iter() {
        let mut z = exact_clonotypes[*s].dataset_indices();
        lis.append(&mut z);
    }
    unique_sort(&mut lis);
    // Clonotypes found in at least n datasets
    if lis.len() < ctl.clono_filt_opt.min_datasets {
        return false;
    }
    // Clonotypes in no more than n datasets
    if lis.len() > ctl.clono_filt_opt.max_datasets {
        return false;
    }

    // Implement MIN_DATASET_RATIO.

    if ctl.clono_filt_opt.min_dataset_ratio > 0 {
        let mut datasets = Vec::<usize>::new();
        for i in 0..exacts.len() {
            let ex = &exact_clonotypes[exacts[i]];
            for j in 0..ex.ncells() {
                datasets.push(ex.clones[j][0].dataset_index);
            }
        }
        datasets.sort();
        let mut freq = Vec::<(u32, usize)>::new();
        make_freq(&datasets, &mut freq);
        if freq.len() == 1
            || freq[0].0 < ctl.clono_filt_opt.min_dataset_ratio as u32 * max(1, freq[1].0)
        {
            return false;
        }
    }

    // Clonotypes with no more and no less than min and max chains
    if cols < ctl.clono_filt_opt.min_chains || cols > ctl.clono_filt_opt.max_chains {
        return false;
    }
    // Clonotypes with given junction AA sequence
    if ctl.clono_filt_opt.cdr3.is_some() {
        let mut ok = false;
        for s in exacts.iter() {
            let ex = &exact_clonotypes[*s];
            for j in 0..ex.share.len() {
                if ctl
                    .clono_filt_opt
                    .cdr3
                    .as_ref()
                    .unwrap()
                    .is_match(&ex.share[j].cdr3_aa)
                {
                    ok = true;
                }
            }
        }
        if !ok {
            return false;
        }
    }
    let mut donors = Vec::<usize>::new();
    for u in 0..exacts.len() {
        let ex = &exact_clonotypes[exacts[u]];
        for m in 0..ex.clones.len() {
            if ex.clones[m][0].donor_index.is_some() {
                donors.push(ex.clones[m][0].donor_index.unwrap());
            }
        }
    }
    unique_sort(&mut donors);
    if ctl.clono_filt_opt.fail_only && donors.len() <= 1 {
        return false;
    }
    return true;
}