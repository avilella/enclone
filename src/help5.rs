// Copyright (c) 2019 10X Genomics, Inc. All rights reserved.
//
// Test for help request, under development.

use ansi_escape::*;
use help_utils::*;
use string_utils::*;
use tables::*;
use vector_utils::*;

// ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

pub fn help5(args: &Vec<String>) {
    // Set up.

    let mut args = args.clone();
    let mut rows = Vec::<Vec<String>>::new();
    macro_rules! doc {
        ($n1:expr, $n2:expr) => {
            rows.push(vec![$n1.to_string(), $n2.to_string()]);
        };
    }
    macro_rules! ldoc {
        ($n1:expr, $n2:expr) => {
            rows.push(vec!["\\hline".to_string(); 2]);
            rows.push(vec![$n1.to_string(), $n2.to_string()]);
        };
    }
    let mut plain = false;
    for i in 0..args.len() {
        if args[i] == "PLAIN" {
            args.remove(i);
            plain = true;
            unsafe {
                PLAIN = true;
            }
            break;
        }
    }
    if args.len() == 1 || (args.len() >= 2 && args[1] == "help") {
        let mut to_delete = vec![false; args.len()];
        for i in 1..args.len() {
            if args[i] == "NOPAGER" {
                to_delete[i] = true;
            }
        }
        erase_if(&mut args, &to_delete);
    }
    /*
    macro_rules! doc_red {
        ($n1:expr, $n2:expr) => {
            if !plain {
                let r1 = format!( "[01;31m{}[0m", $n1 );
                let r2 = format!( "[01;31m{}[0m", $n2 );
                rows.push( vec![ r1, r2 ] );
            } else {
        };
    }
    macro_rules! ldoc_red {
        ($n1:expr, $n2:expr) => {
            rows.push( vec![ "\\hline".to_string(); 2 ] );
            if !plain {
                let r1 = format!( "[01;31m{}[0m", $n1 );
                let r2 = format!( "[01;31m{}[0m", $n2 );
                rows.push( vec![ r1, r2 ] );
            } else {
                rows.push( vec![ $n1.to_string(), $n2.to_string() ] );
            }
        };
    }
    */
    macro_rules! bold {
        () => {
            if !plain {
                let mut log = Vec::<u8>::new();
                emit_bold_escape(&mut log);
                print!("{}", strme(&log));
            }
        };
    }
    macro_rules! end_escape {
        () => {
            if !plain {
                let mut log = Vec::<u8>::new();
                emit_end_escape(&mut log);
                print!("{}", strme(&log));
            }
        };
    }
    let mut help_all = false;
    unsafe {
        if HELP_ALL {
            help_all = true;
        }
    }
    macro_rules! begin_doc {
        ($x:expr) => {
            rows.clear();
            if help_all {
                banner($x, plain);
            }
        };
    }

    // ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

    // Provide indels help.

    if (args.len() == 3 && args[1] == "help" && args[2] == "indels") || help_all {
        begin_doc!("indels");
        print("\n\\bold{handling of insertions and deletions}\n\n");
        print(
            "enclone can recognize and display a single insertion or deletion in a contig \
             relative to the reference, so long as its length is divisible by three, is relatively \
             short, and occurs within the V segment, not too close to its right end.\n\n\
             These indels could be germline, however most such events are already captured in a \
             reference sequence.  Currently the donor reference code in enclone does not recognize \
             indels.\n\n\
             SHM deletions are rare, and SHM insertions are even more rare.\n\n\
             Deletions are displayed using hyphens (-).  If you use the \\bold{var} option for \
             \\bold{cvars}, the hyphens will be displayed in base space, where they are initially \
             observed.  For the \\bold{AMINO} option, the deletion is first shifted by up to two \
             bases, so that the deletion starts at a base position that is divisible by three.  \
             Then the deleted amino acids are shown as hyphens.\n\n\
             Insertions are shown only in amino acid space, in a special per-chain column that \
             appears if there is an insertion.  Colored amino acids are shown for the insertion, \
             and the position of the insertion is shown.  The position is the position of the \
             amino acid after which the insertion appears, where the first amino acid (start \
             codon) is numbered 0.\n\n",
        );
        if !help_all {
            std::process::exit(0);
        }
    }

    // ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

    // Provide ideas help.

    if (args.len() == 3 && args[1] == "help" && args[2] == "ideas") || help_all {
        begin_doc!("ideas");
        println!("");
        bold!();
        println!("features that might be implemented in enclone\n");
        end_escape!();
        doc!("speed", "make enclone faster");
        ldoc!(
            "CDRn",
            "make CDR1 and CDR2 viewable in the same way that CDR3 is now"
        );
        ldoc!(
            "distance grouping",
            "provide an option to group clonotypes by distance"
        );
        ldoc!("cloning", "package V..J region into a cloning vector");
        ldoc!(
            "phylogeny",
            "generate a phylogeny for the exact clonotypes within a clonotype"
        );
        let mut log = String::new();
        print_tabular_vbox(&mut log, &rows, 2, &b"l|l".to_vec(), false);
        print!("{}", log);
        println!(
            "\nPlease let us know if you are interested in these features, or if there are\n\
             other features that you would like us to implement!\n"
        );
        if !help_all {
            std::process::exit(0);
        }
    }

    // ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

    // Provide color help.

    if (args.len() == 3 && args[1] == "help" && args[2] == "color") || help_all {
        begin_doc!("color");
        println!("\nHere is the color palette that enclone uses for amino acids:\n");
        let mut pal = String::new();
        for s in 0..6 {
            let mut log = Vec::<u8>::new();
            if !plain {
                print_color(s, &mut log);
                pal += &stringme(&log);
            }
            pal.push('▓');
            let mut log = Vec::<u8>::new();
            if !plain {
                emit_end_escape(&mut log);
                pal += &stringme(&log);
            }
            if s < 6 {
                pal.push(' ');
            }
        }
        println!("{}", pal);
        println!(
            "\nWhen enclone shows amino acids, it colors each codon differently, via \
             the following scheme:\n"
        );
        println!("{}\n", colored_codon_table(plain));
        println!(
            "Colored amino acids enable the compact display of all the information in a \
             clonotype.\n"
        );
        println!(
            "The coloring is done using special characters, called ANSI escape characters.\n\
             Color is used occasionally elsewhere by enclone, and there is also some\n\
             bolding, accomplished using the same mechanism.\n\n\
             Correct display of colors and bolding depends on having a terminal window\n\
             that is properly set up.  As far as we know, this may always be the case,\n\
             but it is possible that there are exceptions.  In addition, in general, text\n\
             editors do not correctly interpret escape characters.\n\n\
             For both of these reasons, you may wish to turn off the \"special effects\",\n\
             either some or all of the time.  You can do this by adding the argument"
        );
        bold!();
        println!("PLAIN");
        end_escape!();
        println!("to any enclone command.\n");
        print(
            "We know of two methods to get enclone output into another document, along \
             with colors:\n\
             1. Take a screenshot.\n\
             2. Open a new terminal window, type the enclone command, and then convert the \
             terminal window into a pdf.  See \\bold{enclone help faq} for related \
             instructions.\n\n",
        );
        if !help_all {
            std::process::exit(0);
        }
    }

    // ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

    // Provide faq help.

    if (args.len() == 3 && args[1] == "help" && args[2] == "faq") || help_all {
        begin_doc!("faq");
        println!("");
        if !plain {
            let mut log = Vec::<u8>::new();
            emit_bold_escape(&mut log);
            emit_red_escape(&mut log);
            print!("{}", strme(&log));
        }
        print("\\bold{Frequently Asked Questions}\n\n");
        end_escape!();
        println!(
            "We're sorry you're having difficulty!  Please see the answers below, check out\n\
             the other help guides, and if you're still stuck, write to us at \
             enclone@10xgenomics.com.\n"
        );

        print("\\boldblue{1. Why is my enclone output garbled?}\n\n");
        println!( "We can think of two possibilities:\n\n\
            A. The escape characters that enclone emits for color and bolding are not getting\n\
            translated.  You have some options:\n\
            (a) Turn off escape character generation by adding PLAIN to your enclone commands.\n\
            This will work but you'll lose some information.\n\
            (b) If your terminal window is not translating escape characters, ask someone\n\
            with appropriate expertise to help you.  We have not observed this phenomenon,\n\
            but it should be fixable.\n\
            (c) If you're trying to view enclone output, with escape characters, using an editor,\n\
            that's probably not going to work well.\n\n\
            B. Perhaps enclone is emitting very wide lines.  Here are things you can do about \
            this:\n\
            (a) Make your terminal window wider or reduce the font size.\n\
            (b) Identify the field that is very wide and use the column controls to remove that\n\
            field.  See the help for lvars and cvars.  For example," );
        print(
            "\\bold{AMINO=cdr3}\n\
             may help, or even\n\
             \\bold{AMINO=}\n\n",
        );

        print("\\boldblue{2. How can I print the entire enclone documentation?}\n\n");
        println!(
            "We don't know how in general, but the following works for us from a mac:\n\n\
             A. open a new terminal window\n\
             B. make it 110 characters wide; font should be fixed width and roughly 12pt\n\
             C. type \"enclone help all NOPAGER\"\n\
             D. type command-A to select all\n\
             E. type option-command-P to print selection\n\
             F. click the PDF button in the lower left (drop down menu)\n\
             G. click \"Open in Preview\"\n\
             H. then print (or save the pdf, if you prefer).\n"
        );

        print("\\boldblue{3. Why is enclone slow for me?}\n\n");
        print(
            "It should not be.  When we use it, it typically takes a few seconds for a single \
             dataset, and longer for multiple datasets.  If you have \
             encountered an example where it is slow, please write to us, so we can improve its \
             performance.\n\n",
        );

        print("\\boldblue{4. How does enclone fit into the 10x Genomics software ecosystem?}\n\n");
        print(
            "There are several parts to the answer:\n\
             • enclone is a standalone executable that by default produces human-readable output.\n\
             • You can also run enclone to produce parseable output \
             (see \\bold{enclone help parseable}), \
             and that output can be digested using code that you write (for example, in R).\n\
             • When you run Cell Ranger to process 10x single cell immune profiling data, it in \
             effect calls enclone with a special option that yields only an output file for \
             the 10x visualization tool Loupe.\n\
             • Clonotypes may then be viewed using Loupe.  The view of a clonotype provided by \
             Loupe is different than the view provided by enclone.  Loupe shows a continuous \
             expanse of bases across each chain, which you can scroll across, rather than the \
             compressed view of \"important\" bases or amino acids that enclone shows.\n\n",
        );

        print("\\boldblue{5. What platforms does enclone run on?}\n\n");
        print(
            "1. linux/x86-64 (that's most servers)\n\
             2. mac\n\
             3. windows (maybe, we haven't yet tested).\n\n\
             However, we have not and cannot test every possible configuration of these \
             platforms.  Please let us know if you encounter problems!\n\n",
        );

        print("\\boldblue{6. How can I print out all the donor reference sequences?}\n\n");
        print(
            "Add the argument \\bold{DONOR_REF_FILE=filename} to your enclone command, \
             and fasta for the donor reference sequences will be dumped there.\n\n",
        );

        print("\\boldblue{7. How does enclone know what VDJ reference sequences I'm using?}\n\n");
        print(
            "It does not!  It assumes that you have the \\bold{human} reference sequences that \
             shipped with the latest version of Cell Ranger.  If instead your sequences are mouse, \
             then you can specify that by adding the argument \\bold{MOUSE} to your command \
             line.  If you are simply using another reference sequence, please specify that \
             by adding \\bold{REF=f} to your command line, where \\bold{f} is the name of your \
             VDJ reference fasta file.\n\n",
        );

        print("\\boldblue{8. Can I provide data from more than one donor?}\n\n");
        print( "Yes.  Type \\bold{enclone help input} for details.  If we encounter a clonotype \
            containing data from more than one donor, it will be flagged as an error.  The primary \
            reason for allowing entry of data from multiple donors is to allow estimation of \
            enclone's error rate.\n\n" );

        print("\\boldblue{9. What are some command line argument values quoted?}\n\n");
        print(
            "Command line argument values that contain any of these characters ;|* need to \
             be quoted like so\n\
             \\bold{TCR=\"a;b\"}\n\
             to prevent the shell from interpreting them for a purpose completely unrelated to \
             enclone.  This is a trap, because forgetting to add the quotes can result in \
             nonsensical and confusing behavior!\n\n",
        );

        std::process::exit(0);
    }

    // ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓

    // Catch unrecognized help requests.

    if args.len() >= 2 {
        args[1].make_ascii_lowercase();
        if args[1].contains("help") {
            println!("\nYour help request doesn't match one known to enclone.\n");
            println!("Please type \"enclone\" to see the help options.\n");
            std::process::exit(1);
        }
    }
}