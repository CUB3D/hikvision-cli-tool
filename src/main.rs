mod hv_client;
mod types;

use crate::hv_client::HvClient;
use crate::types::{Password, Probe, ProbeMatchSuccessBody};
use clap::{App, AppSettings, Arg, SubCommand};
use cli_table::format::CellFormat;
use cli_table::{Cell, Row, Table};
use std::num::NonZeroU64;
use std::time::Duration;
use uuid::Uuid;

//Questions to answer:
// How does activation work
// How does the password reset work

fn find_cameras(client: &HvClient, uuid: &Uuid) -> Vec<ProbeMatchSuccessBody> {
    let probe = Probe::Inquiry {
        uuid: uuid.to_string(),
    };
    client
        .send_broadcast(&probe)
        .expect("Unable to send discovery broadcast");
    wait_for_cameras(client)
        .into_iter()
        .map(|c| c.expect("Inquire failed"))
        .collect()
}

fn wait_for_cameras(client: &HvClient) -> Vec<Result<ProbeMatchSuccessBody, String>> {
    let mut cameras = vec![];
    while let Some(probe_match) = client.read_packet() {
        cameras.push(probe_match.payload());
    }
    cameras
}

fn print_table(cameras: Vec<ProbeMatchSuccessBody>) -> Table {
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
    Table::new(table_cells, Default::default()).unwrap()
}

fn do_inquire(client: &HvClient, uuid: &Uuid) {
    println!("Starting discovery!");
    let cameras = find_cameras(&client, &uuid);

    let table = print_table(cameras);
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
        .subcommand(
            SubCommand::with_name("update")
                .about("Change settings")
                .arg(
                    Arg::with_name("serial")
                        .long("serial")
                        .value_name("serial")
                        .required(true),
                )
                .arg(
                    Arg::with_name("dhcp")
                        .long("dhcp")
                        .required(false)
                        .value_name("enabled")
                        .possible_values(&["true", "false"]),
                )
                .arg(
                    Arg::with_name("password")
                        .long("password")
                        .required(true)
                        .value_name("password"),
                ),
        )
        .get_matches();

    let our_uuid = Uuid::new_v4();

    let client = HvClient::new_with_timeout(Duration::from_secs(
        matches.value_of("timeout").unwrap().parse().unwrap(),
    ));

    if let Some(_matches) = matches.subcommand_matches("inquire") {
        do_inquire(&client, &our_uuid);
    }

    if let Some(matches) = matches.subcommand_matches("update") {
        let target_serial = matches.value_of("serial").unwrap();
        println!("Searching for cameras");
        let cams = find_cameras(&client, &our_uuid);

        if let Some(camera) = cams.iter().find(|c| c.device_sn == target_serial) {
            println!("Found target camera");

            let dhcp = matches
                .value_of("dhcp")
                .map(|v| v == "true")
                .unwrap_or(camera.dhcp);

            let password = matches.value_of("password").expect("No password given");

            let probe = Probe::Update {
                uuid: our_uuid.to_string(),
                pw_error_parse: "true".to_string(),
                mac: camera.mac.clone(),
                password: Password::hash(password),
                ipv4_address: camera.ipv4_address.clone(),
                command_port: camera.command_port.to_string(),
                http_port: camera.http_port.clone(),
                ipv4_subnet_mask: camera.ipv4_subnet_mask.clone(),
                ipv4_gateway: camera.ipv4_gateway.clone(),
                ipv6_address: camera.ipv6_address.clone(),
                ipv6_gateway: camera.ipv6_gateway.clone(),
                ipv6_mask_len: camera.ipv6_mask_len,
                dhcp,
                sdk_over_tls_port: 0,
            };

            client.send_broadcast(&probe).unwrap();

            let camera = wait_for_cameras(&client)
                .first()
                .cloned()
                .expect("Camera didn't respond in time");

            match camera {
                Ok(camera) => {
                    if camera.device_sn != target_serial {
                        println!("Incorrect device responded");
                        return;
                    }
                    println!("Camera updated successfully, new config:");
                    print_table(vec![camera]).print_stdout().unwrap()
                }
                Err(_msg) => println!("Failed to update camera settings, is the password correct?"),
            }
        } else {
            println!("Target camera not found");
        }
    }
}
