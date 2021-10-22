//! Module related to message template handling.
//!
//! This module gathers all message template commands.  

use anyhow::Result;

use crate::{
    config::Account,
    domain::{
        imap::ImapServiceInterface,
        msg::{Msg, TplOverride},
    },
    output::OutputServiceInterface,
};

/// Generate a new message template.
pub fn new<'a, OutputService: OutputServiceInterface>(
    opts: TplOverride<'a>,
    account: &'a Account,
    output: &'a OutputService,
) -> Result<()> {
    let tpl = Msg::default().to_tpl(opts, account);
    output.print(tpl)
}

/// Generate a reply message template.
pub fn reply<'a, OutputService: OutputServiceInterface, ImapService: ImapServiceInterface<'a>>(
    seq: &str,
    all: bool,
    opts: TplOverride<'a>,
    account: &'a Account,
    output: &'a OutputService,
    imap: &'a mut ImapService,
) -> Result<()> {
    let tpl = imap
        .find_msg(seq)?
        .into_reply(all, account)?
        .to_tpl(opts, account);
    output.print(tpl)
}

/// Generate a forward message template.
pub fn forward<'a, OutputService: OutputServiceInterface, ImapService: ImapServiceInterface<'a>>(
    seq: &str,
    opts: TplOverride<'a>,
    account: &'a Account,
    output: &'a OutputService,
    imap: &'a mut ImapService,
) -> Result<()> {
    let tpl = imap
        .find_msg(seq)?
        .into_forward(account)?
        .to_tpl(opts, account);
    output.print(tpl)
}
