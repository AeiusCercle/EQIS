// Dual-protocol FTP/SFTP client for AI-to-AI communication
// Supports both plain FTP (port 21) and SFTP (port 22)

use anyhow::{Result, Context};
use std::io::{Read, Write, Cursor};
use std::path::Path;
use serde::Serialize;

use crate::config::Config;

// Enum to handle both FTP and SFTP connections
pub enum FtpClient {
    PlainFtp(suppaftp::FtpStream),
    SecureFtp(ssh2::Session),
}

impl FtpClient {
    pub async fn connect(config: &Config) -> Result<Self> {
        // Auto-detect protocol based on port
        if config.ftp_port == 21 {
            Self::connect_plain_ftp(config)
        } else if config.ftp_port == 22 || config.ftp_port == 2222 {
            Self::connect_sftp(config)
        } else {
            // Try plain FTP first, fall back to SFTP
            Self::connect_plain_ftp(config)
                .or_else(|_| Self::connect_sftp(config))
        }
    }

    fn connect_plain_ftp(config: &Config) -> Result<Self> {
        let address = format!("{}:{}", config.ftp_host, config.ftp_port);

        let mut stream = suppaftp::FtpStream::connect(&address)
            .context(format!("Failed to connect to FTP server at {}", address))?;

        stream.login(&config.ftp_user, &config.ftp_pass)
            .context("FTP authentication failed")?;

        Ok(FtpClient::PlainFtp(stream))
    }

    fn connect_sftp(config: &Config) -> Result<Self> {
        use std::net::TcpStream;

        let address = format!("{}:{}", config.ftp_host, config.ftp_port);

        let tcp = TcpStream::connect(&address)
            .context(format!("Failed to connect to SFTP server at {}", address))?;

        let mut session = ssh2::Session::new()
            .context("Failed to create SSH session")?;

        session.set_tcp_stream(tcp);
        session.handshake()
            .context("SSH handshake failed")?;

        session.userauth_password(&config.ftp_user, &config.ftp_pass)
            .context("SFTP authentication failed")?;

        if !session.authenticated() {
            anyhow::bail!("SFTP authentication failed");
        }

        Ok(FtpClient::SecureFtp(session))
    }

    pub async fn pwd(&mut self) -> Result<String> {
        match self {
            FtpClient::PlainFtp(stream) => {
                stream.pwd().context("Failed to get current directory")
            },
            FtpClient::SecureFtp(session) => {
                let sftp = session.sftp().context("Failed to initialize SFTP")?;
                let path = sftp.realpath(Path::new("."))
                    .context("Failed to get current directory")?;
                Ok(path.to_str().unwrap_or("/").to_string())
            }
        }
    }

    pub async fn mkdir(&mut self, path: &str) -> Result<()> {
        // Create all parent directories recursively
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current_path = String::new();

        for part in parts {
            current_path.push('/');
            current_path.push_str(part);

            match self {
                FtpClient::PlainFtp(stream) => {
                    // Try to create, ignore error if already exists
                    let _ = stream.mkdir(&current_path);
                },
                FtpClient::SecureFtp(session) => {
                    let sftp = session.sftp().context("Failed to initialize SFTP")?;
                    let _ = sftp.mkdir(Path::new(&current_path), 0o755);
                }
            }
        }

        Ok(())
    }

    pub async fn list_files(&mut self, path: &str) -> Result<Vec<String>> {
        match self {
            FtpClient::PlainFtp(stream) => {
                let files = stream.nlst(Some(path))
                    .context(format!("Failed to list files in {}", path))?;
                Ok(files.into_iter()
                    .filter(|f| !f.is_empty())
                    .map(|f| {
                        // Extract just filename from full path
                        f.split('/').last().unwrap_or(&f).to_string()
                    })
                    .collect())
            },
            FtpClient::SecureFtp(session) => {
                let sftp = session.sftp().context("Failed to initialize SFTP")?;
                let entries = sftp.readdir(Path::new(path))
                    .context(format!("Failed to list directory: {}", path))?;
                let files: Vec<String> = entries.iter()
                    .filter_map(|(path, _)| {
                        path.file_name()
                            .and_then(|name| name.to_str())
                            .map(|s| s.to_string())
                    })
                    .collect();
                Ok(files)
            }
        }
    }

    pub async fn upload_json<T: Serialize>(&mut self, remote_path: &str, data: &T) -> Result<()> {
        let json = serde_json::to_vec_pretty(data)?;

        // Ensure parent directory exists
        if let Some(parent) = Path::new(remote_path).parent() {
            if let Some(parent_str) = parent.to_str() {
                self.mkdir(parent_str).await?;
            }
        }

        match self {
            FtpClient::PlainFtp(stream) => {
                let mut reader = Cursor::new(json);
                stream.put_file(remote_path, &mut reader)
                    .context(format!("Failed to upload file to {}", remote_path))?;
                Ok(())
            },
            FtpClient::SecureFtp(session) => {
                let sftp = session.sftp().context("Failed to initialize SFTP")?;

                // Ensure parent directory exists
                if let Some(parent) = Path::new(remote_path).parent() {
                    let _ = sftp.mkdir(parent, 0o755);
                }

                let mut remote_file = sftp.create(Path::new(remote_path))
                    .context(format!("Failed to create remote file: {}", remote_path))?;

                remote_file.write_all(&json)
                    .context(format!("Failed to write to remote file: {}", remote_path))?;

                Ok(())
            }
        }
    }

    pub async fn download_string(&mut self, remote_path: &str) -> Result<String> {
        match self {
            FtpClient::PlainFtp(stream) => {
                let cursor = stream.retr_as_buffer(remote_path)
                    .context(format!("Failed to download file from {}", remote_path))?;
                Ok(String::from_utf8(cursor.into_inner())?)
            },
            FtpClient::SecureFtp(session) => {
                let sftp = session.sftp().context("Failed to initialize SFTP")?;
                let mut remote_file = sftp.open(Path::new(remote_path))
                    .context(format!("Failed to open remote file: {}", remote_path))?;
                let mut contents = String::new();
                remote_file.read_to_string(&mut contents)
                    .context(format!("Failed to read remote file: {}", remote_path))?;
                Ok(contents)
            }
        }
    }

    pub async fn rename(&mut self, from: &str, to: &str) -> Result<()> {
        match self {
            FtpClient::PlainFtp(stream) => {
                stream.rename(from, to)
                    .context(format!("Failed to rename {} to {}", from, to))?;
                Ok(())
            },
            FtpClient::SecureFtp(session) => {
                let sftp = session.sftp().context("Failed to initialize SFTP")?;

                // Ensure parent directory of destination exists
                if let Some(parent) = Path::new(to).parent() {
                    let _ = sftp.mkdir(parent, 0o755);
                }

                sftp.rename(Path::new(from), Path::new(to), None)
                    .context(format!("Failed to rename {} to {}", from, to))?;

                Ok(())
            }
        }
    }

    pub async fn disconnect(self) -> Result<()> {
        match self {
            FtpClient::PlainFtp(mut stream) => {
                stream.quit().context("Failed to disconnect from FTP server")?;
            },
            FtpClient::SecureFtp(_session) => {
                // SSH session automatically closes when dropped
            }
        }
        Ok(())
    }
}
