/// A list of versions, generated using `stencila binaries versions python` on Linux
/// for use on Windows (where `asdf` is not available). In the future, this should be
/// generated in a build script or release script.
/// 
/// Not all of these releases have a Window binary available so will fail to install!
#[rustfmt::skip]
pub const VERSIONS: &[&str] = &[
    "3.10.2",
    "3.10.1",
    "3.10.0",
    "3.9.10",
    "3.9.9",
    "3.9.8",
    "3.9.7",
    "3.9.6",
    "3.9.5",
    "3.9.4",
    "3.9.2",
    "3.9.1",
    "3.9.0",
    "3.8.12",
    "3.8.11",
    "3.8.10",
    "3.8.9",
    "3.8.8",
    "3.8.7",
    "3.8.6",
    "3.8.5",
    "3.8.4",
    "3.8.3",
    "3.8.2",
    "3.8.1",
    "3.8.0",
    "3.7.12",
    "3.7.11",
    "3.7.10",
    "3.7.9",
    "3.7.8",
    "3.7.7",
    "3.7.6",
    "3.7.5",
    "3.7.4",
    "3.7.3",
    "3.7.2",
    "3.7.1",
    "3.7.0",
    "3.6.15",
    "3.6.14",
    "3.6.13",
    "3.6.12",
    "3.6.11",
    "3.6.10",
    "3.6.9",
    "3.6.8",
    "3.6.7",
    "3.6.6",
    "3.6.5",
    "3.6.4",
    "3.6.3",
    "3.6.2",
    "3.6.1",
    "3.6.0",
    "3.5.10",
    "3.5.9",
    "3.5.8",
    "3.5.7",
    "3.5.6",
    "3.5.5",
    "3.5.4",
    "3.5.3",
    "3.5.2",
    "3.5.1",
    "3.5.0",
    "3.4.10",
    "3.4.9",
    "3.4.8",
    "3.4.7",
    "3.4.6",
    "3.4.5",
    "3.4.4",
    "3.4.3",
    "3.4.2",
    "3.4.1",
    "3.4.0",
    "3.3.7",
    "3.3.6",
    "3.3.5",
    "3.3.4",
    "3.3.3",
    "3.3.2",
    "3.3.1",
    "3.3.0",
    "3.2.6",
    "3.2.5",
    "3.2.4",
    "3.2.3",
    "3.2.2",
    "3.2.1",
    "3.2.0",
    "3.1.5",
    "3.1.4",
    "3.1.3",
    "3.1.2",
    "3.1.1",
    "3.1.0",
    "3.0.1",
    "2.7.18",
    "2.7.17",
    "2.7.16",
    "2.7.15",
    "2.7.14",
    "2.7.13",
    "2.7.12",
    "2.7.11",
    "2.7.10",
    "2.7.9",
    "2.7.8",
    "2.7.7",
    "2.7.6",
    "2.7.5",
    "2.7.4",
    "2.7.3",
    "2.7.2",
    "2.7.1",
    "2.7.0",
    "2.6.9",
    "2.6.8",
    "2.6.7",
    "2.6.6",
    "2.6.5",
    "2.6.4",
    "2.6.3",
    "2.6.2",
    "2.6.1",
    "2.6.0",
    "2.5.6",
    "2.5.5",
    "2.5.4",
    "2.5.3",
    "2.5.2",
    "2.5.1",
    "2.5.0",
    "2.4.6",
    "2.4.5",
    "2.4.4",
    "2.4.3",
    "2.4.2",
    "2.4.1",
    "2.4.0",
    "2.3.7",
    "2.2.3",
    "2.1.3",
];
