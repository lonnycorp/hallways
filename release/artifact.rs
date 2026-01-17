pub enum Artifact {
    LinuxBinaryTarGz,
    LinuxAppImage,
    LinuxDeb,
    WindowsZip,
}

impl Artifact {
    pub fn file_name(&self, version: &str) -> String {
        match self {
            Artifact::LinuxBinaryTarGz => {
                return format!("hallways-{version}-linux-amd64.tar.gz");
            }
            Artifact::LinuxAppImage => {
                return format!("hallways-{version}-linux-amd64.AppImage");
            }
            Artifact::LinuxDeb => {
                return format!("hallways-{version}-linux-amd64.deb");
            }
            Artifact::WindowsZip => {
                return format!("hallways-{version}-windows-amd64.zip");
            }
        }
    }
}
