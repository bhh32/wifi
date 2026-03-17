use crate::messages::{Message, SecurityChoice, Tab};
use cmd_lib::types::{
    ConnectionInfo, EthernetConfig, InterfaceInfo, Ipv4Config, WifiConfig,
    WifiNetwork,
};
use cmd_lib::{connection, ethernet, wifi};
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::prelude::*;
use cosmic::widget;
use cosmic::Task;

const APP_ID: &str = "com.bhh32.wifi-manager";

pub struct AppState {
    core: cosmic::Core,
    tab: Tab,

    // WiFi form
    con_name: String,
    ssid: String,
    psk: String,
    is_hidden: bool,
    auto_con: bool,
    use_dhcp: bool,
    ipv4: String,
    netmask: String,
    gateway: String,
    dns: String,
    security: SecurityChoice,
    priority: String,

    // Ethernet form
    eth_con_name: String,
    eth_auto_con: bool,
    eth_use_dhcp: bool,
    eth_ipv4: String,
    eth_netmask: String,
    eth_gateway: String,
    eth_dns: String,

    // Data
    wifi_interfaces: Vec<InterfaceInfo>,
    eth_interfaces: Vec<InterfaceInfo>,
    selected_wifi_iface: usize,
    selected_eth_iface: usize,
    networks: Vec<WifiNetwork>,
    connections: Vec<ConnectionInfo>,
    selected_connection: Option<usize>,

    // Status
    status_message: Option<(bool, String)>, // (is_success, message)
}

impl cosmic::Application for AppState {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let app = Self {
            core,
            tab: Tab::Wifi,
            con_name: String::new(),
            ssid: String::new(),
            psk: String::new(),
            is_hidden: false,
            auto_con: false,
            use_dhcp: true,
            ipv4: String::new(),
            netmask: "24".to_string(),
            gateway: String::new(),
            dns: String::new(),
            security: SecurityChoice::WpaPsk,
            priority: String::new(),
            eth_con_name: String::new(),
            eth_auto_con: false,
            eth_use_dhcp: true,
            eth_ipv4: String::new(),
            eth_netmask: "24".to_string(),
            eth_gateway: String::new(),
            eth_dns: String::new(),
            wifi_interfaces: Vec::new(),
            eth_interfaces: Vec::new(),
            selected_wifi_iface: 0,
            selected_eth_iface: 0,
            networks: Vec::new(),
            connections: Vec::new(),
            selected_connection: None,
            status_message: None,
        };

        // Load interfaces and connections on startup
        let init_task = Task::batch(vec![
            Task::perform(
                async { load_interfaces_wifi().await },
                |result| cosmic::Action::App(match result {
                    Ok(ifaces) => Message::InterfacesLoaded(ifaces),
                    Err(e) => Message::OperationError(e),
                }),
            ),
            Task::perform(
                async { load_connections().await },
                |result| cosmic::Action::App(match result {
                    Ok(cons) => Message::ConnectionsLoaded(cons),
                    Err(e) => Message::OperationError(e),
                }),
            ),
        ]);

        (app, init_task)
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::SetTab(tab) => {
                self.tab = tab;
                if tab == Tab::Connections {
                    return Task::perform(
                        async { load_connections().await },
                        |result| cosmic::Action::App(match result {
                            Ok(cons) => Message::ConnectionsLoaded(cons),
                            Err(e) => Message::OperationError(e),
                        }),
                    );
                }
            }

            // WiFi form updates
            Message::UpdateConName(v) => self.con_name = v,
            Message::UpdateSsid(v) => self.ssid = v,
            Message::UpdatePsk(v) => self.psk = v,
            Message::UpdateIPv4(v) => self.ipv4 = v,
            Message::UpdateNetmask(v) => self.netmask = v,
            Message::UpdateGateway(v) => self.gateway = v,
            Message::UpdateDns(v) => self.dns = v,
            Message::ToggleHidden(v) => self.is_hidden = v,
            Message::ToggleAutoCon(v) => self.auto_con = v,
            Message::ToggleDHCP(v) => self.use_dhcp = v,
            Message::SetSecurity(s) => self.security = s,
            Message::SetPriority(v) => self.priority = v,

            // Ethernet form updates
            Message::UpdateEthConName(v) => self.eth_con_name = v,
            Message::UpdateEthIPv4(v) => self.eth_ipv4 = v,
            Message::UpdateEthNetmask(v) => self.eth_netmask = v,
            Message::UpdateEthGateway(v) => self.eth_gateway = v,
            Message::UpdateEthDns(v) => self.eth_dns = v,
            Message::ToggleEthAutoCon(v) => self.eth_auto_con = v,
            Message::ToggleEthDHCP(v) => self.eth_use_dhcp = v,

            // Interface selection
            Message::SelectWifiInterface(idx) => self.selected_wifi_iface = idx,
            Message::SelectEthInterface(idx) => self.selected_eth_iface = idx,

            // Network selection from scan
            Message::SelectNetwork(net) => {
                self.ssid = net.ssid.clone();
                self.con_name = net.ssid;
            }

            // Actions
            Message::AddWifi => {
                let iface = self
                    .wifi_interfaces
                    .get(self.selected_wifi_iface)
                    .map(|i| i.name.clone())
                    .unwrap_or_default();

                let ipv4_config = if !self.use_dhcp {
                    let prefix = self.netmask.parse().unwrap_or(24u32);
                    let dns_list: Vec<String> = self
                        .dns
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    Some(Ipv4Config {
                        address: self.ipv4.clone(),
                        prefix,
                        gateway: self.gateway.clone(),
                        dns: dns_list,
                    })
                } else {
                    None
                };

                let priority = self.priority.parse::<i32>().ok();

                let config = WifiConfig {
                    con_name: self.con_name.clone(),
                    iface,
                    ssid: self.ssid.clone(),
                    password: if self.security == SecurityChoice::Open {
                        None
                    } else {
                        Some(self.psk.clone())
                    },
                    security: self.security.into(),
                    is_hidden: self.is_hidden,
                    auto_connect: self.auto_con,
                    ipv4: ipv4_config,
                    priority,
                };

                return Task::perform(
                    async move {
                        wifi::setup(&config)
                            .map(|_| format!("WiFi connection '{}' created", config.con_name))
                            .map_err(|e| e.to_string())
                    },
                    |result| cosmic::Action::App(match result {
                        Ok(msg) => Message::OperationSuccess(msg),
                        Err(e) => Message::OperationError(e),
                    }),
                );
            }

            Message::AddEthernet => {
                let iface = self
                    .eth_interfaces
                    .get(self.selected_eth_iface)
                    .map(|i| i.name.clone())
                    .unwrap_or_default();

                let ipv4_config = if !self.eth_use_dhcp {
                    let prefix = self.eth_netmask.parse().unwrap_or(24u32);
                    let dns_list: Vec<String> = self
                        .eth_dns
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    Some(Ipv4Config {
                        address: self.eth_ipv4.clone(),
                        prefix,
                        gateway: self.eth_gateway.clone(),
                        dns: dns_list,
                    })
                } else {
                    None
                };

                let config = EthernetConfig {
                    con_name: self.eth_con_name.clone(),
                    iface,
                    auto_connect: self.eth_auto_con,
                    ipv4: ipv4_config,
                    priority: None,
                };

                return Task::perform(
                    async move {
                        ethernet::setup(&config)
                            .map(|_| format!("Ethernet connection '{}' created", config.con_name))
                            .map_err(|e| e.to_string())
                    },
                    |result| cosmic::Action::App(match result {
                        Ok(msg) => Message::OperationSuccess(msg),
                        Err(e) => Message::OperationError(e),
                    }),
                );
            }

            Message::RemoveConnection => {
                if let Some(idx) = self.selected_connection {
                    if let Some(conn) = self.connections.get(idx) {
                        let name = conn.name.clone();
                        return Task::perform(
                            async move {
                                connection::remove(&name)
                                    .map(|_| format!("Connection '{name}' removed"))
                                    .map_err(|e| e.to_string())
                            },
                            |result| cosmic::Action::App(match result {
                                Ok(msg) => Message::OperationSuccess(msg),
                                Err(e) => Message::OperationError(e),
                            }),
                        );
                    }
                }
            }

            Message::ScanNetworks => {
                let iface = self
                    .wifi_interfaces
                    .get(self.selected_wifi_iface)
                    .map(|i| i.name.clone());

                return Task::perform(
                    async move {
                        wifi::scan(iface.as_deref())
                            .map_err(|e| e.to_string())
                    },
                    |result| cosmic::Action::App(match result {
                        Ok(nets) => Message::ScanComplete(nets),
                        Err(e) => Message::OperationError(e),
                    }),
                );
            }

            Message::RefreshConnections => {
                return Task::perform(
                    async { load_connections().await },
                    |result| cosmic::Action::App(match result {
                        Ok(cons) => Message::ConnectionsLoaded(cons),
                        Err(e) => Message::OperationError(e),
                    }),
                );
            }

            Message::RefreshInterfaces => {
                return Task::perform(
                    async { load_interfaces_wifi().await },
                    |result| cosmic::Action::App(match result {
                        Ok(ifaces) => Message::InterfacesLoaded(ifaces),
                        Err(e) => Message::OperationError(e),
                    }),
                );
            }

            Message::ActivateConnection(name) => {
                return Task::perform(
                    async move {
                        connection::activate(&name)
                            .map(|_| format!("Connection '{name}' activated"))
                            .map_err(|e| e.to_string())
                    },
                    |result| cosmic::Action::App(match result {
                        Ok(msg) => Message::OperationSuccess(msg),
                        Err(e) => Message::OperationError(e),
                    }),
                );
            }

            Message::DeactivateConnection(name) => {
                return Task::perform(
                    async move {
                        connection::deactivate(&name)
                            .map(|_| format!("Connection '{name}' deactivated"))
                            .map_err(|e| e.to_string())
                    },
                    |result| cosmic::Action::App(match result {
                        Ok(msg) => Message::OperationSuccess(msg),
                        Err(e) => Message::OperationError(e),
                    }),
                );
            }

            Message::SelectConnection(idx) => {
                self.selected_connection = Some(idx);
            }

            // Async results
            Message::ScanComplete(nets) => {
                self.networks = nets;
            }

            Message::ConnectionsLoaded(cons) => {
                self.connections = cons;
                self.selected_connection = None;
            }

            Message::InterfacesLoaded(ifaces) => {
                let wifi_ifaces: Vec<InterfaceInfo> =
                    ifaces.iter().filter(|i| i.iface_type == "wifi").cloned().collect();
                let eth_ifaces: Vec<InterfaceInfo> =
                    ifaces.iter().filter(|i| i.iface_type == "ethernet").cloned().collect();
                self.wifi_interfaces = wifi_ifaces;
                self.eth_interfaces = eth_ifaces;
            }

            Message::OperationSuccess(msg) => {
                self.status_message = Some((true, msg));
                // Refresh connections after successful operation
                return Task::perform(
                    async { load_connections().await },
                    |result| cosmic::Action::App(match result {
                        Ok(cons) => Message::ConnectionsLoaded(cons),
                        Err(e) => Message::OperationError(e),
                    }),
                );
            }

            Message::OperationError(msg) => {
                self.status_message = Some((false, msg));
            }

            Message::DismissStatus => {
                self.status_message = None;
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let spacing = cosmic::theme::active().cosmic().spacing;

        // Tab bar
        let tab_bar = widget::row::with_capacity(3)
            .push(
                widget::button::text("WiFi")
                    .on_press(Message::SetTab(Tab::Wifi)),
            )
            .push(
                widget::button::text("Ethernet")
                    .on_press(Message::SetTab(Tab::Ethernet)),
            )
            .push(
                widget::button::text("Connections")
                    .on_press(Message::SetTab(Tab::Connections)),
            )
            .spacing(spacing.space_xs);

        // Status bar
        let status_bar: Option<Element<Self::Message>> = self.status_message.as_ref().map(|(is_success, msg)| {
            let label = if *is_success {
                format!("[OK] {msg}")
            } else {
                format!("[Error] {msg}")
            };
            widget::row::with_capacity(2)
                .push(widget::text::body(label))
                .push(
                    widget::button::text("Dismiss")
                        .on_press(Message::DismissStatus),
                )
                .spacing(spacing.space_s)
                .into()
        });

        // Content based on active tab
        let content: Element<Self::Message> = match self.tab {
            Tab::Wifi => self.view_wifi(spacing),
            Tab::Ethernet => self.view_ethernet(spacing),
            Tab::Connections => self.view_connections(spacing),
        };

        let mut main_col = widget::column::with_capacity(4)
            .push(tab_bar)
            .spacing(spacing.space_m);

        if let Some(status) = status_bar {
            main_col = main_col.push(status);
        }

        main_col = main_col.push(content);

        widget::container(main_col)
            .padding(spacing.space_m)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
}

impl AppState {
    fn view_wifi(&self, spacing: cosmic::cosmic_theme::Spacing) -> Element<'_, Message> {
        let mut col = widget::column::with_capacity(12).spacing(spacing.space_s);

        col = col.push(widget::text::title3("Add WiFi Connection"));

        // Interface selector
        if !self.wifi_interfaces.is_empty() {
            let iface_names: Vec<String> = self.wifi_interfaces.iter().map(|i| i.name.clone()).collect();
            let mut iface_row = widget::row::with_capacity(3).spacing(spacing.space_s);
            iface_row = iface_row.push(widget::text::body("Interface:"));
            for (idx, name) in iface_names.iter().enumerate() {
                let is_selected = idx == self.selected_wifi_iface;
                let label = if is_selected {
                    format!("[{name}]")
                } else {
                    name.clone()
                };
                iface_row = iface_row.push(
                    widget::button::text(label).on_press(Message::SelectWifiInterface(idx)),
                );
            }
            col = col.push(iface_row);
        }

        // Form fields
        col = col
            .push(
                widget::text_input("Connection Name", &self.con_name)
                    .on_input(Message::UpdateConName),
            )
            .push(
                widget::text_input("SSID", &self.ssid)
                    .on_input(Message::UpdateSsid),
            );

        // Security selector
        let security_row = widget::row::with_capacity(5)
            .spacing(spacing.space_xs)
            .push(widget::text::body("Security:"))
            .push(
                widget::button::text("Open")
                    .on_press(Message::SetSecurity(SecurityChoice::Open)),
            )
            .push(
                widget::button::text("WPA-PSK")
                    .on_press(Message::SetSecurity(SecurityChoice::WpaPsk)),
            )
            .push(
                widget::button::text("WPA3")
                    .on_press(Message::SetSecurity(SecurityChoice::Wpa3Sae)),
            )
            .push(
                widget::button::text("Enterprise")
                    .on_press(Message::SetSecurity(SecurityChoice::WpaEnterprise)),
            );
        col = col.push(security_row);

        // Password field (hidden for open networks)
        if self.security != SecurityChoice::Open {
            col = col.push(
                widget::secure_input("Password", &self.psk, None, true)
                    .on_input(Message::UpdatePsk),
            );
        }

        // Checkboxes
        let checks_row = widget::row::with_capacity(3)
            .spacing(spacing.space_m)
            .push(widget::checkbox(self.is_hidden).label("Hidden Network").on_toggle(Message::ToggleHidden))
            .push(widget::checkbox(self.auto_con).label("Auto-Connect").on_toggle(Message::ToggleAutoCon))
            .push(widget::checkbox(self.use_dhcp).label("DHCP").on_toggle(Message::ToggleDHCP));
        col = col.push(checks_row);

        // Manual IP config (shown when DHCP is off)
        if !self.use_dhcp {
            col = col
                .push(
                    widget::text_input("IPv4 Address", &self.ipv4)
                        .on_input(Message::UpdateIPv4),
                )
                .push(
                    widget::row::with_capacity(2)
                        .spacing(spacing.space_s)
                        .push(
                            widget::text_input("Netmask (CIDR)", &self.netmask)
                                .on_input(Message::UpdateNetmask),
                        )
                        .push(
                            widget::text_input("Gateway", &self.gateway)
                                .on_input(Message::UpdateGateway),
                        ),
                )
                .push(
                    widget::text_input("DNS Servers (comma-separated)", &self.dns)
                        .on_input(Message::UpdateDns),
                );
        }

        // Priority
        col = col.push(
            widget::text_input("Priority (optional, -999 to 999)", &self.priority)
                .on_input(Message::SetPriority),
        );

        // Action buttons
        let actions = widget::row::with_capacity(2)
            .spacing(spacing.space_s)
            .push(
                widget::button::suggested("Add WiFi Connection")
                    .on_press(Message::AddWifi),
            )
            .push(
                widget::button::text("Scan Networks")
                    .on_press(Message::ScanNetworks),
            );
        col = col.push(actions);

        // Scanned networks list
        if !self.networks.is_empty() {
            col = col.push(widget::text::title4("Available Networks"));

            for net in &self.networks {
                let in_use_icon = if net.in_use { "* " } else { "  " };
                let label = format!(
                    "{}{} | Signal: {}% | {}",
                    in_use_icon, net.ssid, net.signal, net.security
                );
                let net_clone = net.clone();
                col = col.push(
                    widget::button::text(label)
                        .on_press(Message::SelectNetwork(net_clone))
                        .width(Length::Fill),
                );
            }
        }

        col.into()
    }

    fn view_ethernet(&self, spacing: cosmic::cosmic_theme::Spacing) -> Element<'_, Message> {
        let mut col = widget::column::with_capacity(10).spacing(spacing.space_s);

        col = col.push(widget::text::title3("Add Ethernet Connection"));

        // Interface selector
        if !self.eth_interfaces.is_empty() {
            let mut iface_row = widget::row::with_capacity(3).spacing(spacing.space_s);
            iface_row = iface_row.push(widget::text::body("Interface:"));
            for (idx, iface) in self.eth_interfaces.iter().enumerate() {
                let is_selected = idx == self.selected_eth_iface;
                let label = if is_selected {
                    format!("[{}]", iface.name)
                } else {
                    iface.name.clone()
                };
                iface_row = iface_row.push(
                    widget::button::text(label).on_press(Message::SelectEthInterface(idx)),
                );
            }
            col = col.push(iface_row);
        }

        col = col.push(
            widget::text_input("Connection Name", &self.eth_con_name)
                .on_input(Message::UpdateEthConName),
        );

        let checks_row = widget::row::with_capacity(2)
            .spacing(spacing.space_m)
            .push(widget::checkbox(self.eth_auto_con).label("Auto-Connect").on_toggle(Message::ToggleEthAutoCon))
            .push(widget::checkbox(self.eth_use_dhcp).label("DHCP").on_toggle(Message::ToggleEthDHCP));
        col = col.push(checks_row);

        if !self.eth_use_dhcp {
            col = col
                .push(
                    widget::text_input("IPv4 Address", &self.eth_ipv4)
                        .on_input(Message::UpdateEthIPv4),
                )
                .push(
                    widget::row::with_capacity(2)
                        .spacing(spacing.space_s)
                        .push(
                            widget::text_input("Netmask (CIDR)", &self.eth_netmask)
                                .on_input(Message::UpdateEthNetmask),
                        )
                        .push(
                            widget::text_input("Gateway", &self.eth_gateway)
                                .on_input(Message::UpdateEthGateway),
                        ),
                )
                .push(
                    widget::text_input("DNS Servers (comma-separated)", &self.eth_dns)
                        .on_input(Message::UpdateEthDns),
                );
        }

        col = col.push(
            widget::button::suggested("Add Ethernet Connection")
                .on_press(Message::AddEthernet),
        );

        col.into()
    }

    fn view_connections(&self, spacing: cosmic::cosmic_theme::Spacing) -> Element<'_, Message> {
        let mut col = widget::column::with_capacity(20).spacing(spacing.space_s);

        col = col.push(widget::text::title3("Saved Connections"));

        let actions = widget::row::with_capacity(2)
            .spacing(spacing.space_s)
            .push(
                widget::button::text("Refresh")
                    .on_press(Message::RefreshConnections),
            )
            .push(
                widget::button::destructive("Remove Selected")
                    .on_press(Message::RemoveConnection),
            );
        col = col.push(actions);

        if self.connections.is_empty() {
            col = col.push(widget::text::body("No saved connections found."));
        } else {
            // Header
            let header = widget::row::with_capacity(4)
                .spacing(spacing.space_m)
                .push(widget::text::body("Name").width(Length::FillPortion(3)))
                .push(widget::text::body("Type").width(Length::FillPortion(2)))
                .push(widget::text::body("Device").width(Length::FillPortion(2)))
                .push(widget::text::body("Actions").width(Length::FillPortion(2)));
            col = col.push(header);

            for (idx, conn) in self.connections.iter().enumerate() {
                let is_selected = self.selected_connection == Some(idx);
                let device = if conn.device.is_empty() {
                    "--".to_string()
                } else {
                    conn.device.clone()
                };

                let name_label = if is_selected {
                    format!("> {}", conn.name)
                } else {
                    conn.name.clone()
                };

                let mut row = widget::row::with_capacity(4)
                    .spacing(spacing.space_m)
                    .align_y(Alignment::Center);

                row = row
                    .push(
                        widget::button::text(name_label)
                            .on_press(Message::SelectConnection(idx))
                            .width(Length::FillPortion(3)),
                    )
                    .push(widget::text::body(&conn.conn_type).width(Length::FillPortion(2)))
                    .push(widget::text::body(device).width(Length::FillPortion(2)));

                let action_row = if conn.active {
                    widget::button::text("Disconnect")
                        .on_press(Message::DeactivateConnection(conn.name.clone()))
                } else {
                    widget::button::text("Connect")
                        .on_press(Message::ActivateConnection(conn.name.clone()))
                };

                row = row.push(
                    widget::container(action_row).width(Length::FillPortion(2)),
                );

                col = col.push(row);
            }
        }

        col.into()
    }
}

// Async helper functions
async fn load_interfaces_wifi() -> Result<Vec<InterfaceInfo>, String> {
    connection::list_interfaces().map_err(|e| e.to_string())
}

async fn load_connections() -> Result<Vec<ConnectionInfo>, String> {
    connection::list().map_err(|e| e.to_string())
}
