use std::net::SocketAddr;

const LOGO: &str = r#"     _                              ____           _
    | | __ _  ___  __ _  ___ _ __|  _ \ ___  ___| |_ __ _ _ __ ___  ___
 _  | |/ _` |/ _ \/ _` |/ _ \ '__| |_) / _ \/ __| __/ _` | '__/ _ \/ __|
| |_| | (_| |  __/ (_| |  __/ |  |  __/ (_) \__ \ || (_| | | |  __/\__ \
 \___/ \__,_|\___|\__, |\___|_|  |_|   \___/|___/\__\__, |_|  \___||___/
                  |___/                              |___/"#;

pub fn startup_banner(address: SocketAddr) -> String {
    format!(
        r#"{LOGO}

  PostgreSQL trace storage is ready
  ------------------------------------------------------------
  gRPC bind       {address}
  OTLP writes     opentelemetry.proto.collector.trace.v1.TraceService
  Trace reads     jaeger.storage.v2.TraceReader
  Dependencies    jaeger.storage.v2.DependencyReader
  ------------------------------------------------------------
  All gRPC services share port {}.
"#,
        address.port()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_displays_bind_address_and_services() {
        let banner = startup_banner("127.0.0.1:17271".parse().unwrap());
        assert!(banner.is_ascii());
        assert!(banner.contains("gRPC bind       127.0.0.1:17271"));
        assert!(banner.contains("OTLP writes"));
        assert!(banner.contains("TraceReader"));
        assert!(banner.contains("DependencyReader"));
        assert!(banner.contains("share port 17271"));
    }

    #[test]
    fn banner_formats_ipv6_addresses() {
        let banner = startup_banner("[::1]:4317".parse().unwrap());
        assert!(banner.contains("[::1]:4317"));
        assert!(banner.contains("share port 4317"));
    }
}
