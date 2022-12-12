use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::time::SystemTime;

use tinytemplate::TinyTemplate;

mod html;
mod types;

use types::RBFEvent;

use rawtx_rs::bitcoin;
use rawtx_rs::tx::TxInfo;
use rawtx_rs::{input::InputType, output::OutputType};

const REPLACEMENT_GROUPS_PER_PAGE: u32 = 100;
const MAX_PAGES: u32 = 10;

fn in_and_outputs_to_strings(txinfo: &TxInfo) -> (Vec<String>, Vec<String>) {
    let mut output_type_counts: HashMap<OutputType, u32> = HashMap::new();
    for ot in txinfo.output_infos.iter().map(|i| i.out_type) {
        if let Some(count) = output_type_counts.clone().get(&ot) {
            output_type_counts.insert(ot, *count + 1);
        } else {
            output_type_counts.insert(ot, 1);
        }
    }

    let mut input_type_counts: HashMap<InputType, u32> = HashMap::new();
    for it in txinfo.input_infos.iter().map(|i| i.in_type) {
        if let Some(count) = input_type_counts.clone().get(&it) {
            input_type_counts.insert(it, *count + 1);
        } else {
            input_type_counts.insert(it, 1);
        }
    }

    let mut inputs_strs: Vec<String> = input_type_counts
        .iter()
        .map(|(k, v)| format!("{}x {}", v, k))
        .collect();
    let mut outputs_strs: Vec<String> = output_type_counts
        .iter()
        .map(|(k, v)| format!("{}x {}", v, k))
        .collect();

    // Rust 'randomizes' the order of elements is HashMaps to make sure people
    // don't rely on the order. For us it's nicer to have them always sorted to
    // be able to better compare them.
    inputs_strs.sort();
    outputs_strs.sort();

    (inputs_strs, outputs_strs)
}

fn build_replacement_context(
    event: &RBFEvent,
    replaced_tx: &bitcoin::Transaction,
    replacement_tx: &bitcoin::Transaction,
) -> html::ReplacementContext {
    let replaced_txinfo = TxInfo::new(replaced_tx).unwrap();
    let replacement_txinfo = TxInfo::new(replacement_tx).unwrap();

    let (replaced_input_infos, repalced_output_infos) = in_and_outputs_to_strings(&replaced_txinfo);
    let (replacement_input_infos, repalcement_output_infos) =
        in_and_outputs_to_strings(&replacement_txinfo);

    html::ReplacementContext {
        timestamp: event.timestamp,
        replaced: html::TransactionContext {
            txid: replaced_tx.txid().to_string(),
            fee: event.replaced_fee,
            vsize: event.replaced_vsize,
            time_in_mempool: if event.replaced_entry_time > 0 {
                event.timestamp as i64 - event.replaced_entry_time as i64
            } else {
                i64::default()
            },
            feerate: format!(
                "{:.2}",
                event.replaced_fee as f64 / event.replaced_vsize as f64
            ),
            op_return: replaced_txinfo.has_opreturn_output(),
            raw: hex::encode(&event.replaced_raw),
            optin_rbf: replaced_txinfo.is_signaling_explicit_rbf_replicability(),
            inputs: replaced_input_infos,
            outputs: repalced_output_infos,
        },
        replacement: html::TransactionContext {
            txid: replacement_tx.txid().to_string(),
            fee: event.replacement_fee,
            vsize: event.replacement_vsize,
            time_in_mempool: i64::default(),
            feerate: format!(
                "{:.2}",
                (event.replacement_fee as f64 / event.replacement_vsize as f64)
            ),
            op_return: replacement_txinfo.has_opreturn_output(),
            optin_rbf: replacement_txinfo.is_signaling_explicit_rbf_replicability(),
            raw: hex::encode(&event.replacement_raw),
            inputs: replacement_input_infos,
            outputs: repalcement_output_infos,
        },
    }
}

fn conflict(tx1: &bitcoin::Transaction, tx2: &bitcoin::Transaction) -> bool {
    let tx1_outpoints: HashSet<bitcoin::OutPoint> =
        tx1.input.iter().map(|i| i.previous_output).collect();
    let tx2_outpoints: HashSet<bitcoin::OutPoint> =
        tx2.input.iter().map(|i| i.previous_output).collect();

    tx1_outpoints.intersection(&tx2_outpoints).count() > 0
}

fn get_reverse_fullrbf_replacements(csv_file_path: &str) -> Vec<html::ReplacementContext> {
    println!("Reading replacements from {}", csv_file_path);
    let mut rdr = csv::Reader::from_path(csv_file_path).unwrap();
    let mut replacements: Vec<html::ReplacementContext> = Vec::new();

    for csv_replacement in rdr.deserialize() {
        let event: RBFEvent = csv_replacement.unwrap();
        let replaced_tx: bitcoin::Transaction =
            bitcoin::consensus::encode::deserialize(&event.replaced_raw).unwrap();
        let replacement_tx: bitcoin::Transaction =
            bitcoin::consensus::encode::deserialize(&event.replacement_raw).unwrap();
        let optin_rbf = replaced_tx.input.iter().any(|i| i.sequence.is_rbf());

        // A transaction that did not opt-in to RBF can still be replaced, if it
        // does not directly conflict with the replacement transaction. These
        // are not full-RBF replacements though.
        if !optin_rbf && conflict(&replaced_tx, &replacement_tx) {
            replacements.push(build_replacement_context(
                &event,
                &replaced_tx,
                &replacement_tx,
            ))
        }
    }

    println!(
        "Read {} full-rbf replacements from {}",
        replacements.len(),
        csv_file_path
    );
    replacements.reverse();
    replacements
}

fn generate_html_files(replacements: Vec<html::ReplacementGroupContext>, html_output_dir: &str) {
    println!("Generating HTML files to {} ...", html_output_dir);
    let mut tt = TinyTemplate::new();
    tt.add_template("tmpl_transaction", html::TEMPLATE_TX)
        .unwrap();
    tt.add_template("tmpl_deltas", html::TEMPLATE_DELTAS)
        .unwrap();
    tt.add_template("tmpl_replacement", html::TEMPLATE_REPLACEMENT)
        .unwrap();
    tt.add_template("tmpl_navigation", html::TEMPLATE_PAGE_NAVIGATION)
        .unwrap();
    tt.add_template("tmpl_site", html::TEMPLATE_SITE).unwrap();

    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let pages = sequence(min(
        (replacements.len() as f32 / REPLACEMENT_GROUPS_PER_PAGE as f32) as u32 + 1,
        MAX_PAGES,
    ));

    for (page, chunk) in (0_u32..).zip(replacements.chunks(REPLACEMENT_GROUPS_PER_PAGE as usize)) {
        println!("... rendering page {}", page);
        let rendered = tt
            .render(
                "tmpl_site",
                &html::SiteContext {
                    replacements: chunk.to_vec(),
                    timestamp,
                    page,
                    navigation: html::NavigationContext {
                        pages: pages.clone(),
                    },
                },
            )
            .unwrap();

        let filename = format!("{}/{}.html", html_output_dir, get_filename(page));
        println!("... writing page {} to {}", page, filename);
        let mut file = File::create(filename).unwrap();
        write!(file, "{}", rendered).unwrap();
    }
}

fn build_replacement_groups(
    replacements: Vec<html::ReplacementContext>,
) -> Vec<html::ReplacementGroupContext> {
    let mut replacement_groups: HashMap<
        (html::TransactionContext, u64),
        Vec<html::TransactionContext>,
    > = HashMap::new();

    for replacement_event in replacements.iter() {
        match replacement_groups.get_mut(&(
            replacement_event.replacement.clone(),
            replacement_event.timestamp,
        )) {
            None => {
                replacement_groups.insert(
                    (
                        replacement_event.replacement.clone(),
                        replacement_event.timestamp,
                    ),
                    vec![replacement_event.replaced.clone()],
                );
            }
            Some(x) => x.push(replacement_event.replaced.clone()),
        }
    }

    let mut replacement_group_contexts: Vec<html::ReplacementGroupContext> = replacement_groups
        .iter()
        .map(|(k, v)| html::ReplacementGroupContext {
            replaced: v.to_vec(),
            replacement: k.0.clone(),
            timestamp: k.1,
            delta: html::ReplacementGroupDeltaContext {
                fee: k.0.fee as i64 - v.iter().map(|tx| tx.fee).sum::<u64>() as i64,
                vsize: k.0.vsize as i64 - v.iter().map(|tx| tx.vsize).sum::<u64>() as i64,
                feerate: if v.len() > 1 {
                    String::new()
                } else {
                    format!(
                        "+{:.2} sat/vByte",
                        (k.0.fee as f64) / (k.0.vsize as f64)
                            - (v.iter().map(|tx| tx.fee).sum::<u64>() as f64
                                / v.iter().map(|tx| tx.vsize).sum::<u64>() as f64)
                    )
                },
            },
        })
        .collect();
    replacement_group_contexts.sort_by_key(|k| k.timestamp);
    replacement_group_contexts.reverse();
    replacement_group_contexts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <path/to/*.csv> <html output dir>", args[0]);
        exit(1);
    }

    let csv_file_path = &args[1];
    let html_output_dir = &args[2];

    let replacements = get_reverse_fullrbf_replacements(csv_file_path);
    let replacement_group_contexts = build_replacement_groups(replacements);

    let replacement_group_contexts_without_opreturn = replacement_group_contexts
        .iter()
        .filter(|r| !r.replacement.op_return)
        .cloned()
        .collect();

    generate_html_files(replacement_group_contexts, html_output_dir);
    generate_html_files(
        replacement_group_contexts_without_opreturn,
        &format!("{}/no_opreturn", html_output_dir),
    );
    println!("Done generating pages");
}

fn sequence(n: u32) -> Vec<u32> {
    (0..n).collect()
}

fn get_filename(page: u32) -> String {
    if page == 0 {
        String::from("index")
    } else {
        format!("page_{}", page)
    }
}
