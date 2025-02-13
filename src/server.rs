// SPDX-FileCopyrightText: 2023-2025 Simon Repp
// SPDX-FileCopyrightText: 2025 Sandro Santilli
// SPDX-License-Identifier: AGPL-3.0-or-later

use actix_files::Files;
use actix_web::{App, HttpServer};
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};

const DEFAULT_PREVIEW_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_PREVIEW_PORT: u16 = 8080;

/// When DEFAULT_PREVIEW_PORT is unavailable, we try DEFAULT_PREVIEW_PORT + 1,
/// then DEFAULT_PREVIEW_PORT + 2, etc., but after MAX_PORT_ATTEMPTS
/// iterations we stop, as probably something is else is wrong then, rather
/// than all ports taken.
const MAX_PORT_ATTEMPTS: u16 = 10;

#[actix_web::main]
pub async fn serve_preview(build_dir: &Path, ip_requested: Option<IpAddr>, port_requested: Option<u16>) {
    let bind_server = |build_dir_moving: PathBuf, ip: IpAddr, port: u16| {
        HttpServer::new(move || {
            App::new()
                .service(
                    Files::new("/", &build_dir_moving)
                        .redirect_to_slash_directory()
                        .index_file("index.html")
                )
        })
            .bind((ip, port))
    };

    let ip = ip_requested.unwrap_or(DEFAULT_PREVIEW_IP);

    let (server, port_bound) = if let Some(port) = port_requested {
        match bind_server(build_dir.to_owned(), ip, port) {
            Ok(server) => (server, port),
            Err(err) => {
                error!("Could not bind preview server to {}:{} ({})", ip, port, err);
                return
            }
        }
    } else {
        let mut port = DEFAULT_PREVIEW_PORT;

        loop {
            match bind_server(build_dir.to_owned(), ip, port) {
                Ok(server) => break (server, port),
                Err(err) => {
                    if port > DEFAULT_PREVIEW_PORT + MAX_PORT_ATTEMPTS {
                        error!(
                            "Could not bind preview server to {} on any port in the range {}-{} (last error was {}).\nUse --preview-port to manually set a port, --preview-ip to manually set an ip and/or check that no other issue is at play.",
                            ip,
                            DEFAULT_PREVIEW_PORT,
                            port,
                            err
                        );
                        return
                    }

                    port += 1;
                }
            }
        }
    };

    let url = format!("http://{ip}:{port_bound}");

    println!("Serving the site preview at {url} (open this address in your browser)");
    println!("Press Ctrl+C to shut down the preview server (e.g. to perform another build)");

    let open_browser = || async {
        if webbrowser::open(&url).is_err() {
            error!("Could not open browser for previewing the site")
        }
    };

    tokio::join!(server.run(), open_browser()).0.unwrap();
}
