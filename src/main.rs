mod ioctl;
mod linux;

pub(crate) fn map_io_err<T: ToString>(e: T) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string().as_str())
}

pub(crate) fn map_io_err_msg<T: ToString>(e: T, msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, format!("{}: {}", e.to_string(), msg))
}

fn main() -> std::io::Result<()> {
    let iface =
        linux::IFace::new("vpn0".to_string(), &["10.10.0.1".parse().map_err(map_io_err)?], 600)?;
    iface.into_tun_fd();
    Ok(())
}
