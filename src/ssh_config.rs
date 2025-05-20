use crate::model::Server;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// parse SSH config file into a vector of Server structs
pub(crate) fn parse_ssh_config<P: AsRef<str>>(path: P) -> anyhow::Result<Vec<Server>> {
    let file = File::open(shellexpand::tilde(path.as_ref()).into_owned())?;
    let reader = BufReader::new(file);

    let mut configs = Vec::new();
    let mut current_config: Option<Server> = None;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.is_empty() {
            continue;
        }

        let keyword = parts[0].to_lowercase();

        if keyword == "host" {
            if let Some(config) = current_config.take() {
                configs.push(config);
            }

            if parts.len() == 2 {
                let value = parts[1].trim().to_string();
                current_config = Some(Server::new(value));
            }
        } else if let Some(config) = &mut current_config {
            if parts.len() == 2 {
                let value = parts[1].trim().to_string();

                match keyword.as_str() {
                    "hostname" => {
                        if !value.is_empty() {
                            config.host = value;
                        }
                    }
                    "port" => {
                        if let Ok(port_num) = value.parse::<u16>() {
                            config.port = port_num;
                        }
                    }
                    "user" => {
                        if !value.is_empty() {
                            config.user = value
                        }
                    }
                    "identityfile" => {
                        if !value.is_empty() {
                            config.identity_file = Some(value);
                        }
                    }
                    _ => {} // Ignore unknown keywords
                }
            }
        }
    }

    if let Some(config) = current_config {
        configs.push(config);
    }

    Ok(configs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_ssh_config() -> anyhow::Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "# SSH Config Example")?;
        writeln!(file, "Host github.com")?;
        writeln!(file, "    HostName github.com")?;
        writeln!(file, "    User git")?;
        writeln!(file, "    IdentityFile ~/.ssh/github_rsa")?;
        writeln!(file, "    Port 22")?;
        writeln!(file)?;
        writeln!(file, "Host example.com")?;
        writeln!(file, "    HostName example.org")?;
        writeln!(file, "    User admin")?;
        writeln!(file, "    Port 2222")?;
        file.flush()?;

        let configs = parse_ssh_config(file.path().to_string_lossy())?;

        assert_eq!(configs.len(), 2);

        assert_eq!(configs[0].host, "github.com");
        assert_eq!(configs[0].name, "github.com");
        assert_eq!(configs[0].user, "git");
        assert_eq!(
            configs[0].identity_file,
            "~/.ssh/github_rsa".to_string().into()
        );
        assert_eq!(configs[0].port, 22);

        assert_eq!(configs[1].host, "example.org");
        assert_eq!(configs[1].name, "example.com");
        assert_eq!(configs[1].user, "admin");
        assert_eq!(configs[1].port, 2222);
        assert_eq!(configs[1].identity_file, "~/.ssh/id_rsa".to_string().into());

        Ok(())
    }

    #[test]
    fn test_default_values() -> anyhow::Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Host myserver")?;
        file.flush()?;

        let configs = parse_ssh_config(file.path().to_string_lossy())?;

        assert_eq!(configs.len(), 1);

        assert_eq!(configs[0].host, "myserver");
        assert_eq!(configs[0].name, "myserver");
        assert_eq!(configs[0].user, "root");
        assert_eq!(configs[0].port, 22);
        assert_eq!(configs[0].identity_file, "~/.ssh/id_rsa".to_string().into());

        Ok(())
    }

    #[test]
    fn test_multiple_hosts() -> anyhow::Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Host server1 server2 server3")?;
        writeln!(file, "    User shared")?;
        writeln!(file, "    Port 2222")?;
        file.flush()?;

        let configs = parse_ssh_config(file.path().to_string_lossy())?;

        assert_eq!(configs.len(), 1);

        assert_eq!(configs[0].host, "server1 server2 server3");
        assert_eq!(configs[0].user, "shared");
        assert_eq!(configs[0].port, 2222);

        Ok(())
    }
}
