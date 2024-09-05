// All imports are for GUI implementation only.
use crate::messages::Message;
use crate::{setup, remove};
use iced::widget::{button, checkbox, column, text_input, text_input::TextInput, row, Theme, Renderer};
use iced::Element;
use iced::{Alignment::Center, Sandbox};

/// AppState is the struct that holds the data for the
/// application to use to create or remove a wifi network.
pub struct AppState {
    pub con_name: String,
    pub iface: String,
    pub ssid: String,
    pub psk: String,
    pub is_hidden: bool,
    pub auto_con: bool,
    pub is_cli: bool,
    pub use_dhcp: bool,
    pub ipv4: String,
    pub netmask: u32,
    pub gateway: String,
    pub dns_server: String,
}

/// Used for GUI Implementation
impl Sandbox for AppState {
    // Set the GUI Message type to Message enum
    type Message = Message;

    // Create a new default AppState
    fn new() -> Self {
        Self::default()
    }

    // Create the title for the application
    fn title(&self) -> String {
        String::from("Basic Wi-Fi Setup Utility")
    }

    // Update logic for the application based on the Message
    fn update(&mut self, message: Message) {
        match message {
            // Add the network connection
            Message::Add => {
                setup(&self);
            },
            // Remove the network connection
            Message::Remove => {
                remove(&self);
            },
            // Update the connection name
            Message::UpdateConName(name) => self.con_name = name,
            // Update the interface name
            Message::UpdateIface(iface) => self.iface = iface,
            // Update the SSID
            Message::UpdateSsid(ssid) => self.ssid = ssid,
            // Update the wifi password
            Message::UpdatePsk(psk) => self.psk = psk,
            // Update the IPV4 address
            Message::UpdateIPv4(ipv4) => self.ipv4 = ipv4,
            // Update the Netmask in CIDR notation
            Message::UpdateNetmask(netmask) => self.netmask = netmask.parse().unwrap_or(24),
            // Update the gateway IPV4 address
            Message::UpdateGateway(gateway) => self.gateway = gateway,
            // Update the DNS IPv4 address
            Message::UpdateDns(dns_server) => self.dns_server = dns_server,
            // Update if this is a hidden network
            Message::UpdateHidden(is_hidden) => self.is_hidden = is_hidden,
            // Update if this network should auto connect after creation
            Message::UpdateAutoCon(auto_con) => self.auto_con = auto_con,
            // Udpate if this network uses DHCP to get its IPV4 address
            Message::UpdateDHCP(dhcp) => self.use_dhcp = dhcp,
            // Used as an empty message type
            Message::EnterEmptyInput(_value) => {},
        }
    }

    fn view(&self) -> Element<Message> {   
        // Create the connection name text box     
        let con_name_text: TextInput<Message, Theme, Renderer> = text_input("SSID", &self.con_name)
            .on_input(Message::UpdateConName)
            .padding(10)
            .size(20);
        
        // Create the interface name text box
        let iface_text: TextInput<Message, Theme, Renderer> = text_input("Wi-Fi Interface Name", &self.iface)
            .on_input(Message::UpdateIface)
            .padding(10)
            .size(20);

        // Create the ssid name text box
        let ssid_text: TextInput<Message, Theme, Renderer> = text_input("SSID", &self.ssid)
            .on_input(Message::UpdateSsid)
            .padding(10)
            .size(20);

        // Create the password text box
        // TODO: Figure out how to mask the input
        let psk_text: TextInput<Message, Theme, Renderer> = text_input("Wi-Fi Password", &self.psk)
            .on_input(Message::UpdatePsk)
            .padding(10)
            .size(20);

        // Create the hidden network checkbox
        let is_hidden_check = checkbox("Hidden Network", self.is_hidden)
            .on_toggle(Message::UpdateHidden);
        // Create the auto connection checkbox
        let auto_con_check = checkbox("Auto-connect", self.auto_con)
            .on_toggle(Message::UpdateAutoCon);
        // Create the DHCP checkbox
        let use_dhcp_check = checkbox("Auto-IPv4 (DHCP)", self.use_dhcp)
            .on_toggle(Message::UpdateDHCP);
        
        // Create the manual text boxes as disabled
        let mut ipv4_text: TextInput<Message, Theme, Renderer> = text_input("IPv4 Address", &self.ipv4);
        let mut netmask_text: TextInput<Message, Theme, Renderer> = text_input("Netmask (CIDR Notation)", &self.netmask.to_string());
        let mut gateway_text: TextInput<Message, Theme, Renderer> = text_input("Gateway IPv4 Address", &self.gateway);
        let mut dns_text: TextInput<Message, Theme, Renderer> = text_input("DNS Server", &self.dns_server);

        // Enable the manual text boxes
        if !self.use_dhcp {
            ipv4_text = text_input("IPv4 Address", &self.ipv4)
                .on_input(Message::UpdateIPv4);
            netmask_text = text_input("Netmask (CIDR Notation)", &self.netmask.to_string())
                .on_input(Message::UpdateNetmask);
            gateway_text = text_input("Gateway IPv4 Address", &self.gateway)
                .on_input(Message::UpdateGateway);
            dns_text = text_input("DNS Server", &self.dns_server)
                .on_input(Message::UpdateDns);
        }

        // Create the add button
        let add_con = button("Add Wi-Fi Connection")
            .padding(10)
            .on_press(Message::Add);

        // Create the remove button
        let remove_con = button("Remove Connection")
            .padding(10)
            .on_press(Message::Remove);

        // Set a column layout for the elements
        column![
            con_name_text,
            iface_text,
            ssid_text,
            psk_text,
            row![is_hidden_check,
            auto_con_check,
            use_dhcp_check],            
            row![ipv4_text,
            netmask_text,
            gateway_text],
            dns_text,
            row![
                add_con,
                remove_con,
            ]
            .spacing(10)
            .height(100)
            .align_items(Center),
        ]
        .padding(20)
        .align_items(Center)
        .into()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self { 
            con_name: String::new(), 
            iface: String::new(), 
            ssid: String::new(), 
            psk: String::new(), 
            is_hidden: false, 
            auto_con: false, 
            is_cli: false, 
            use_dhcp: true, 
            ipv4: String::new(), 
            netmask: 24, 
            gateway: String::new(), 
            dns_server: String::new(),
        }
    }
}