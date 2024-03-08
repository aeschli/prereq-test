/*---------------------------------------------------------------------------------------------
 *  Copyright (c) Microsoft Corporation. All rights reserved.
 *  Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------*/

// use serde::Deserialize;
// use std::{collections::HashMap, io::IsTerminal};

use const_format::concatcp;

/// Name of the application without quality information.
pub const QUALITYLESS_PRODUCT_NAME: &str = match option_env!("VSCODE_CLI_QUALITYLESS_PRODUCT_NAME")
{
    Some(n) => n,
    None => "Code",
};

/// Name of the application without quality information.
pub const QUALITYLESS_SERVER_NAME: &str = concatcp!(QUALITYLESS_PRODUCT_NAME, " Server");

