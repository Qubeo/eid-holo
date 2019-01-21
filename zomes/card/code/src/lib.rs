#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,
};

// pub mod card;

// see https://developer.holochain.org/api/0.0.3/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "CardNumber" that can be committed to the
// agent's chain via the exposed function create_card_number_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct CardNumber {
    content: String,
}

pub fn handle_create_card_number_entry(entry: CardNumber) -> ZomeApiResult<Address> {
    let entry = Entry::App("card_number".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_card_number_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

pub fn handle_add_card_number(card_number: CardNumber) -> ZomeApiResult<String> {
    let entry = Entry::App("card_number".into(), card_number.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address.to_string())
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: "card_number",
        description: "Card number.",
        sharing: Sharing::Public,
        native_type: CardNumber,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_card_number_entry: CardNumber, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}
define_zome! {
    entries: [
       definition()
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_card_number_entry: {
                inputs: |entry: CardNumber|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: handle_create_card_number_entry
            }
            get_card_number_entry: {
                inputs: |address: Address|,
                outputs: |result: ZomeApiResult<Option<Entry>>|,
                handler: handle_get_card_number_entry
            }
            add_card_number: {
                inputs: |card_number: CardNumber|,
                outputs: |result: ZomeApiResult<String>|,
                handler: handle_add_card_number
            }
        }
    }
}
