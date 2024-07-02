#[derive(Debug)]
pub struct MOTD {
    motd: String,
    server_name: String,
    protocol_version: u32,
    version_name: String,
    current_players: u32,
    max_players: u32,
    unique_id: u64,
    level_name: String,
    game_mode: String,
    game_mode_numeric: u32,
    port_ipv4: u16,
    port_ipv6: u16,
}

impl MOTD {
    pub fn builder() -> MOTDBuilder {
        MOTDBuilder::default()
    }
}

#[derive(Default)]
pub struct MOTDBuilder {
    motd: Option<String>,
    server_name: Option<String>,
    protocol_version: Option<u32>,
    version_name: Option<String>,
    current_players: Option<u32>,
    max_players: Option<u32>,
    unique_id: Option<u64>,
    level_name: Option<String>,
    game_mode: Option<String>,
    game_mode_numeric: Option<u32>,
    port_ipv4: Option<u16>,
    port_ipv6: Option<u16>,
}

impl MOTDBuilder {
    pub fn motd(mut self, motd: String) -> Self {
        self.motd = Some(motd);
        self
    }

    pub fn server_name(mut self, server_name: String) -> Self {
        self.server_name = Some(server_name);
        self
    }

    pub fn protocol_version(mut self, protocol_version: u32) -> Self {
        self.protocol_version = Some(protocol_version);
        self
    }

    pub fn version_name(mut self, version_name: String) -> Self {
        self.version_name = Some(version_name);
        self
    }

    pub fn current_players(mut self, current_players: u32) -> Self {
        self.current_players = Some(current_players);
        self
    }

    pub fn max_players(mut self, max_players: u32) -> Self {
        self.max_players = Some(max_players);
        self
    }

    pub fn unique_id(mut self, unique_id: u64) -> Self {
        self.unique_id = Some(unique_id);
        self
    }

    pub fn level_name(mut self, level_name: String) -> Self {
        self.level_name = Some(level_name);
        self
    }

    pub fn game_mode(mut self, game_mode: String) -> Self {
        self.game_mode = Some(game_mode);
        self
    }

    pub fn game_mode_numeric(mut self, game_mode_numeric: u32) -> Self {
        self.game_mode_numeric = Some(game_mode_numeric);
        self
    }

    pub fn port_ipv4(mut self, port_ipv4: u16) -> Self {
        self.port_ipv4 = Some(port_ipv4);
        self
    }

    pub fn port_ipv6(mut self, port_ipv6: u16) -> Self {
        self.port_ipv6 = Some(port_ipv6);
        self
    }

    pub fn build(self) -> Result<MOTD, &'static str> {
        Ok(MOTD {
            motd: self.motd.ok_or("Missing motd")?,
            server_name: self.server_name.ok_or("Missing server_name")?,
            protocol_version: self.protocol_version.ok_or("Missing protocol_version")?,
            version_name: self.version_name.ok_or("Missing version_name")?,
            current_players: self.current_players.ok_or("Missing current_players")?,
            max_players: self.max_players.ok_or("Missing max_players")?,
            unique_id: self.unique_id.ok_or("Missing unique_id")?,
            level_name: self.level_name.ok_or("Missing level_name")?,
            game_mode: self.game_mode.ok_or("Missing game_mode")?,
            game_mode_numeric: self.game_mode_numeric.ok_or("Missing game_mode_numeric")?,
            port_ipv4: self.port_ipv4.ok_or("Missing port_ipv4")?,
            port_ipv6: self.port_ipv6.ok_or("Missing port_ipv6")?,
        })
    }
}
#[derive(Clone)]
pub struct UnconnectedPong {
    pub time: i64,
    pub guid: i64,
    pub magic: bool,
    pub motd: String
}