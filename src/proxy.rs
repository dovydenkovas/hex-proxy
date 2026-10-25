use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    proxy: Vec<Proxy>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Proxy {
    listen_address: String,
    redirection_address: String,
    on_fail: Option<String>,
    on_open: Option<String>,
    on_close: Option<String>,
}

pub async fn mainloop(config: Config) -> Result<()> {
    let mut handles = vec![];
    info!("Started proxy: ");

    for proxy in config.proxy.into_iter() {
        let handle = tokio::spawn(async move {
            let _ = proxy_mainloop(proxy).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.await {
            Ok(_) => (),
            Err(e) => error!("Proxy crashed with error: {e:?}"),
        }
    }

    Ok(())
}

async fn proxy_mainloop(proxy: Proxy) -> Result<()> {
    let listener = TcpListener::bind(&proxy.listen_address).await?;
    info!(
        "  - {} -> {}",
        &proxy.listen_address, &proxy.redirection_address
    );

    let proxy = Arc::new(proxy);

    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(pair) => pair,
            Err(e) => {
                error!("Accept error: {}", e);
                continue;
            }
        };
        debug!(
            "New connection to {} from {}",
            &proxy.redirection_address, addr
        );

        let th_proxy = Arc::clone(&proxy);
        tokio::spawn(async move {
            handle_proxy(stream, th_proxy).await;
        });
    }
}

async fn handle_proxy(mut client: TcpStream, proxy: Arc<Proxy>) {
    let mut server = match TcpStream::connect(&proxy.redirection_address).await {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Fail to connect to redirection address '{}': {}",
                proxy.redirection_address, e
            );

            if let Some(cmd) = &proxy.on_fail {
                run_cmd(&cmd).await;
            }

            return;
        }
    };

    if let Some(cmd) = &proxy.on_open {
        run_cmd(&cmd).await;
    }

    // TODO: use splice?
    let _ = tokio::io::copy_bidirectional(&mut client, &mut server).await;

    if let Some(cmd) = &proxy.on_close {
        run_cmd(&cmd).await;
    }
}

async fn run_cmd(cmd: &str) {
    match tokio::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .await
    {
        Ok(_) => (),
        Err(e) => error!("Fail to execute '{}': {}", cmd, e),
    }
}
