mod hv_client;
mod types;

use crate::hv_client::HvClient;
use crate::types::{Password, Probe, ProbeMatchSuccessBody};
use clap::{App, AppSettings, Arg, SubCommand};
use cli_table::{print_stdout, Cell, Style, Table, TableStruct};
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

fn get_table(cameras: Vec<ProbeMatchSuccessBody>) -> TableStruct {
    let title = vec![
        "Description".cell().bold(true),
        "IPv4".cell().bold(true),
        "Command Port".cell().bold(true),
        "Software Version".cell().bold(true),
        "IPv4 Gateway".cell().bold(true),
        "HTTP Port".cell().bold(true),
        "Device Serial".cell().bold(true),
        "IPv4 Subnet Mask".cell().bold(true),
        "MAC Address".cell().bold(true),
        "DSP Version".cell().bold(true),
        "Boot Time".cell().bold(true),
        "IPv6 Address".cell().bold(true),
        "IPv6 Gateway".cell().bold(true),
        "IPv6 Prefix Length".cell().bold(true),
        "DHCP Enabled".cell().bold(true),
        "Supports Hik-Connect".cell().bold(true),
        "Hik-Connect Enabled".cell().bold(true),
    ];

    cameras
        .into_iter()
        .map(|probe_match| {
            vec![
                probe_match.device_description.cell(),
                probe_match.ipv4_address.cell(),
                probe_match.command_port.cell(),
                probe_match.software_version.cell(),
                probe_match.ipv4_gateway.cell(),
                probe_match.http_port.cell(),
                probe_match.device_sn.cell(),
                probe_match.ipv4_subnet_mask.cell(),
                probe_match.mac.cell(),
                probe_match.dsp_version.cell(),
                probe_match.boot_time.cell(),
                probe_match.ipv6_address.cell(),
                probe_match.ipv6_gateway.cell(),
                probe_match.ipv6_mask_len.cell(),
                probe_match.dhcp.cell(),
                probe_match.support_hc_platform.cell(),
                probe_match.hc_platform_enable.cell(),
            ]
        })
        .table()
        .title(title)
}

fn do_inquire(client: &HvClient, uuid: &Uuid) {
    println!("Starting discovery!");
    let cameras = find_cameras(&client, &uuid);

    let table = get_table(cameras);
    print_stdout(table).unwrap();
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
                    print_stdout(get_table(vec![camera])).unwrap();
                }
                Err(_msg) => println!("Failed to update camera settings, is the password correct?"),
            }
        } else {
            println!("Target camera not found");
        }
    }
}
