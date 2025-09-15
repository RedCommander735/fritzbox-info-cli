use serde_aux::prelude::deserialize_number_from_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

macro_rules! pub_struct {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        pub struct $name {
            $(
                $(#[$field_meta])*
                pub $field : $ty,
            )*
        }
    };
}

pub_struct!(
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FritzHomeNetResponse {
    pid: String,
    hide: Hide,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    time_till_logout: u64,
    time: Vec<Option<serde_json::Value>>,
    data: Data,
    sid: String,
}
);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hide {
    pub share_usb: bool,
    pub live_tv: bool,
    pub fax_set: bool,
    pub prov_serv: bool,
    pub dect_moni_ex: bool,
    pub rss: bool,
    pub mobile: bool,
    pub dect_rdio: bool,
    pub dect_moni: bool,
    pub dect_mail: bool,
    pub sso_set: bool,
    pub rrd: bool,
    pub live_img: bool
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub searching: bool,
    pub ipclient: bool,
    pub fwcheck: Fwcheck,
    pub updating: String,
    pub topology: Topology,
    pub nexusclient: bool,
    pub devices: Vec<Device>,
}

#[derive(Serialize, Deserialize)]
pub struct Connection {
    pub dsl_diagnosis: bool,
    pub medium_upstream: u64,
    pub downstream: u64,
    pub role: String,
    pub provider: String,
    pub ipv4: Ipv4,
    pub connected: bool,
    pub shapedrate: bool,
    pub direct_connection: bool,
    pub ready_for_fallback: bool,
    pub medium_downstream: u64,
    pub state: String,
    pub upstream: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub active: bool,
    pub ipv6: Ipv6,
    pub speed_manual: bool,
    pub medium: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ipv4 {
    pub connected: bool,
    pub dns: Vec<Dns>,
    pub dslite: bool,
    pub ip: String,
    pub since: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Dns {
    #[serde(rename = "type")]
    pub dns_type: String,
    pub ip: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ipv6 {
    pub ip_lifetime: Lifetime,
    pub connected: bool,
    pub dns: Vec<Dns>,
    pub ip: String,
    pub prefix: String,
    pub prefix_lifetime: Lifetime,
    pub since: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Lifetime {
    pub valid: u64,
    pub preferred: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ConninfoConninfo {
    pub kind: String,
    pub speed: Option<String>,
    pub bandinfo: Option<Vec<BandInfo>>,
    pub usedbands: Option<i64>,
    pub desc: String,
}

#[derive(Serialize, Deserialize)]
pub struct BandInfo {
    pub band: u64,
    pub speed_tx: u64,
    pub speed_rx: u64,
    pub speed: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize)]
pub struct DetailInfo {
    pub edit: Edit,
    pub portrelease: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Edit {
    pub pid: EditPid,
    pub params: Params,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub dev: String,
    pub back_to_page: BackToPageEnum,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackToPageEnum {
    #[serde(rename = "homeNet")]
    HomeNet,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditPid {
    #[serde(rename = "edit_device")]
    EditDevice,
}

#[derive(Serialize, Deserialize)]
pub struct NameInfo {
    pub name: String,
    pub product: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    pub number_count: u64,
    pub active_count: u64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateInfo {
    pub state: State,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Current,
    None,
}

#[derive(Serialize, Deserialize)]
pub struct Wlaninfo {
    pub text: String,
    pub title: String,
    pub shorttitle: String,
}

#[derive(Serialize, Deserialize)]
pub struct Fwcheck {
    pub notdone: bool,
    pub nocheck: bool,
    pub auto: bool,
    pub started: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Topology {
    pub rootuid: String,
    pub devices: HashMap<String, Device>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Device {
    RouterDevice(RouterDevice),
    PLCDevice(PLCDevice),
    DefaultDevice(DefaultDevice),
}

#[derive(Serialize, Deserialize)]
pub struct DefaultDevice {
    pub own_client_device: bool,
    pub dist: u64,
    pub parent: String,
    #[serde(rename="UID")]
    pub uid: String,
    pub category: Category,
    pub switch: bool,
    pub children: Vec<String>,
    pub devtype: String,
    pub ownentry: bool,
    pub stateinfo: StateInfo,
    pub conn: Conn,
    pub master: bool,
    pub ipinfo: String,
    pub updateinfo: UpdateInfo,
    pub gateway: bool,
    pub nameinfo: NameInfo,
    pub detailinfo: DetailInfo,
    pub conninfo: ConnInfo
}

#[derive(Serialize, Deserialize)]
pub struct RouterDevice {
    own_client_device: bool,
    dist: u64,
    parent: String,
    versioninfo: RouterVersionInfo,
    #[serde(rename="UID")]
    uid: String,
    category: Category,
    switch: bool,
    children: Vec<String>,
    devtype: DevType,
    ownentry: bool,
    stateinfo: RouterStateInfo,
    conn: Conn,
    master: bool,
    ipinfo: String,
    updateinfo: UpdateInfo,
    gateway: bool,
    nameinfo: NameInfo,
    detailinfo: RouterDetailInfo,
    connections: Vec<Connection>,
    phone: Phone,
    #[serde(rename="boxType")]
    box_type: String,
    wlaninfo: Vec<Wlaninfo>
}

#[derive(Serialize, Deserialize)]
pub struct PLCDevice {
    own_client_device: bool,
    dist: u64,
    parent: String,
    versioninfo: VersionInfo,
    #[serde(rename="UID")]
    uid: String,
    category: Category,
    switch: bool,
    children: Vec<String>,
    devtype: String,
    conninfo: ConnInfo,
    ownentry: bool,
    stateinfo: StateInfo,
    conn: Conn,
    master: bool,
    ipinfo: String,
    updateinfo: UpdateInfo,
    gateway: bool,
    nameinfo: NameInfo,
    detailinfo: DetailInfo,
    isplc: bool
}

#[derive(Serialize, Deserialize)]
pub struct RouterDetailInfo {
    wlan24: bool,
    wlan5: bool,
    guestaccess: bool
}

#[derive(Serialize, Deserialize)]
pub enum DevType {
    #[serde(rename="fritzbox")]
    FritzBox
}

#[derive(Serialize, Deserialize)]
pub struct RouterStateInfo {
    nexustrust: bool,
    active: bool
}

#[derive(Serialize, Deserialize)]
pub struct StateInfo {
    guest_owe: bool,
    active: bool,
    meshable: bool,
    guest: bool,
    online: bool,
    blocked: bool,
    realtime: bool,
    notallowed: bool,
    #[serde(rename = "internetBlocked")]
    internet_blocked: bool
}

#[derive(Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "lan")]
    Lan,
    #[serde(rename = "wlan")]
    Wlan,
    #[serde(rename = "plc")]
    Plc,
    #[serde(rename = "ownentry")]
    OwnEntry
}

#[derive(Serialize, Deserialize)]
pub enum Conn {
    #[serde(rename = "lan")]
    Lan,
    #[serde(rename = "wlan")]
    Wlan,
    #[serde(rename = "plc")]
    Plc,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnInfo {
    WiredConnInfo(WiredConnInfo),
    WirelessConnInfo(WirelessConnInfo)
}

#[derive(Serialize, Deserialize)]
pub struct WiredConnInfo {
    speed: Option<String>,
    kind: WiredConnKind,
    desc: String
}

#[derive(Serialize, Deserialize)]
pub struct WirelessConnInfo {
    speed: String,
    kind: WirelessConnKind,
    bandinfo: Vec<BandInfo>,
    usedbands: u64,
    desc: String
}

#[derive(Serialize, Deserialize)]
pub enum WiredConnKind {
    #[serde(rename = "lan")]
    Lan,
    #[serde(rename = "plc")]
    Plc
}

#[derive(Serialize, Deserialize)]
pub enum WirelessConnKind {
    #[serde(rename = "wlan")]
    Wlan
}

#[derive(Serialize, Deserialize)]
pub struct VersionInfo {
    version: String
}

#[derive(Serialize, Deserialize)]
pub struct RouterVersionInfo {
    version: String,
    fos: bool
}
