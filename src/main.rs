mod hv_client;
mod types;

use crate::hv_client::HvClient;
use crate::types::{Password, Probe, ProbeMatch};
use clap::{App, AppSettings, Arg, SubCommand};
use cli_table::format::CellFormat;
use cli_table::{Cell, Row, Table};
use std::num::NonZeroU64;
use std::time::Duration;
use uuid::Uuid;
use serde::Serialize;

//Questions to answer:
// How does activation work
// How does the password reset work

fn find_cameras(client: &HvClient, uuid: &Uuid) -> Vec<ProbeMatch> {
    let probe = Probe::Inquiry {
        uuid: uuid.to_string(),
    };
    client
        .send_broadcast(&probe)
        .expect("Unable to send discovery broadcast");
    let mut cameras = vec![];
    while let Some(probe_match) = client.read_packet() {
        cameras.push(probe_match);
    }
    cameras
}

fn do_inquire(client: &HvClient, uuid: &Uuid) {
    println!("Starting discovery!");
    let cameras = find_cameras(&client, &uuid);

    let title_format = CellFormat::builder().bold(true).build();
    let mut table_cells = vec![Row::new(vec![
        Cell::new("Description", title_format),
        Cell::new("IPv4", title_format),
        Cell::new("Command Port", title_format),
        Cell::new("Software Version", title_format),
        Cell::new("IPv4 Gateway", title_format),
        Cell::new("HTTP Port", title_format),
        Cell::new("Device Serial", title_format),
        Cell::new("IPv4 Subnet Mask", title_format),
        Cell::new("MAC Address", title_format),
        Cell::new("DSP Version", title_format),

        Cell::new("Boot Time", title_format),
        Cell::new("IPv6 Address", title_format),
        Cell::new("IPv6 Gateway", title_format),
        Cell::new("IPv6 Prefix Length", title_format),
        Cell::new("DHCP Enabled", title_format),
        Cell::new("Supports Hik-Connect", title_format),
        Cell::new("Hik-Connect Enabled", title_format),
    ])];

    for probe_match in cameras {
        table_cells.push(Row::new(vec![
            Cell::new(&probe_match.device_description, Default::default()),
            Cell::new(&probe_match.ipv4_address, Default::default()),
            Cell::new(&probe_match.command_port, Default::default()),
            Cell::new(&probe_match.software_version, Default::default()),
            Cell::new(&probe_match.ipv4_gateway, Default::default()),
            Cell::new(&probe_match.http_port, Default::default()),
            Cell::new(&probe_match.device_sn, Default::default()),
            Cell::new(&probe_match.ipv4_subnet_mask, Default::default()),
            Cell::new(&probe_match.mac, Default::default()),
            Cell::new(&probe_match.dsp_version, Default::default()),
            Cell::new(&probe_match.boot_time, Default::default()),
            Cell::new(&probe_match.ipv6_address, Default::default()),
            Cell::new(&probe_match.ipv6_gateway, Default::default()),
            Cell::new(&probe_match.ipv6_mask_len, Default::default()),
            Cell::new(&probe_match.dhcp, Default::default()),
            Cell::new(&probe_match.support_hc_platform, Default::default()),
            Cell::new(&probe_match.hc_platform_enable, Default::default()),
        ]));
    }
    let table = Table::new(table_cells, Default::default()).unwrap();
    table.print_stdout().unwrap();
}

fn main() {
    let matches = App::new("hikvision-sadp-client")
        .version("0.1")
        .author("CUB3D")
        .about("Configure Hikvision IP cameras")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .value_name("seconds")
                .help("Sets the socket timeout")
                .takes_value(true)
                .default_value("5")
                .validator(|v| {
                    v.parse::<NonZeroU64>()
                        .map(|_| ())
                        .map_err(|_| "Timeout must be a positive number".to_string())
                }),
        )
        .subcommand(SubCommand::with_name("inquire").about("Find IP cameras on local network"))
        .get_matches();

    let our_uuid = Uuid::new_v4();

    if let Some(_matches) = matches.subcommand_matches("inquire") {
        let client = HvClient::new_with_timeout(Duration::from_secs(
            matches.value_of("timeout").unwrap().parse().unwrap(),
        ));
        do_inquire(&client, &our_uuid);
    }
}
