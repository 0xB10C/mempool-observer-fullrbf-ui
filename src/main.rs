use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::time::SystemTime;

use tinytemplate::TinyTemplate;

mod html;
mod types;

use types::RBFEvent;

const REPLACEMENT_EVENTS_PER_PAGE: u32 = 100;
const MAX_PAGES: u32 = 10;

fn build_replacement_context(
    event: &RBFEvent,
    replaced_tx: &bitcoin::Transaction,
    replacement_tx: &bitcoin::Transaction,
) -> html::ReplacementContext {
    html::ReplacementContext {
        timestamp: event.timestamp,
        replaced: html::TransactionContext {
            txid: replaced_tx.txid().to_string(),
            fee: event.replaced_fee,
            vsize: event.replaced_vsize,
            feerate: format!(
                "{:.2}",
                event.replaced_fee as f64 / event.replaced_vsize as f64
            ),
            op_return: replaced_tx
                .output
                .iter()
                .any(|o| o.script_pubkey.is_op_return()),
            raw: hex::encode(&event.replaced_raw),
        },
        deltas: html::ReplacementDeltaContext {
            fee: event.replacement_fee as i64 - event.replaced_fee as i64,
            vsize: event.replacement_vsize as i64 - event.replaced_vsize as i64,
            feerate: format!(
                "{:.2}",
                (event.replacement_fee as f64 / event.replacement_vsize as f64)
                    - (event.replaced_fee as f64 / event.replaced_vsize as f64)
            ),
        },
        replacement: html::TransactionContext {
            txid: replacement_tx.txid().to_string(),
            fee: event.replacement_fee,
            vsize: event.replacement_vsize,
            feerate: format!(
                "{:.2}",
                (event.replacement_fee as f64 / event.replacement_vsize as f64)
            ),
            op_return: replaced_tx
                .output
                .iter()
                .any(|o| o.script_pubkey.is_op_return()),
            raw: hex::encode(&event.replacement_raw),
        },
    }
}

fn get_reverse_fullrbf_replacements(csv_file_path: &str) -> Vec<html::ReplacementContext> {
    let mut rdr = csv::Reader::from_path(csv_file_path).unwrap();
    let mut replacements: Vec<html::ReplacementContext> = Vec::new();

    for csv_replacement in rdr.deserialize() {
        let event: RBFEvent = csv_replacement.unwrap();
        let replaced_tx: bitcoin::Transaction =
            bitcoin::consensus::encode::deserialize(&event.replaced_raw).unwrap();
        let replacement_tx: bitcoin::Transaction =
            bitcoin::consensus::encode::deserialize(&event.replacement_raw).unwrap();
        let optin_rbf = replaced_tx.input.iter().all(|i| i.sequence.is_rbf());
        if !optin_rbf {
            replacements.push(build_replacement_context(
                &event,
                &replaced_tx,
                &replacement_tx,
            ))
        }
    }

    replacements.reverse();
    replacements
}

fn generate_html_files(replacements: Vec<html::ReplacementContext>, html_output_dir: &str) {

    let mut tt = TinyTemplate::new();
    tt.add_template("tmpl_transaction", html::TEMPLATE_TX)
        .unwrap();
    tt.add_template("tmpl_deltas", html::TEMPLATE_DELTAS)
        .unwrap();
    tt.add_template("tmpl_replacement", html::TEMPLATE_REPLACEMENT)
        .unwrap();
    tt.add_template("tmpl_site", html::TEMPLATE_SITE).unwrap();

    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let pages = sequence(min(
        (replacements.len() as f32 / REPLACEMENT_EVENTS_PER_PAGE as f32) as u32 + 1,
        MAX_PAGES,
    ));

    for (page, chunk) in (0_u32..).zip(replacements.chunks(REPLACEMENT_EVENTS_PER_PAGE as usize)) {
        let rendered = tt
            .render(
                "tmpl_site",
                &html::SiteContext {
                    replacements: chunk.to_vec(),
                    num_replacements: chunk.len(),
                    timestamp,
                    page,
                    pages: pages.clone(),
                },
            )
            .unwrap();

        let mut file =
            File::create(format!("{}/{}.html", html_output_dir, get_filename(page))).unwrap();
        write!(file, "{}", rendered).unwrap();
    }
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

    generate_html_files(replacements, html_output_dir);
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
