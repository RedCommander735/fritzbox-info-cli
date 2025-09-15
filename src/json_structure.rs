use serde_aux::prelude::deserialize_number_from_string;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::Error;

macro_rules! pub_struct {
    (
        $(#[$meta:meta])*
        struct $name:ident {
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

// TODO Rename most fields to make more sense or look better

pub_struct!(
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FritzResponse {
    pid: String,
    hide: Hide,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    time_till_logout: u64,
    data: Data,
    sid: String,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Hide {
    share_usb: bool,
    live_tv: bool,
    fax_set: bool,
    prov_serv: bool,
    dect_moni_ex: bool,
    rss: bool,
    mobile: bool,
    dect_rdio: bool,
    dect_moni: bool,
    dect_mail: bool,
    sso_set: bool,
    rrd: bool,
    live_img: bool
}
);

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    HomeNetData(HomeNetData),
    EditDeviceData(EditDeviceData),
}

impl Data {
    pub fn to_home_net_data(self) -> Result<HomeNetData, Error> {
        match self {
            Data::HomeNetData(d) => Ok(d),
            Data::EditDeviceData(_) => Err(Error::ConversionError)
        }
    }
    pub fn to_edit_device_data(self) -> Result<EditDeviceData, Error> {
        match self {
            Data::EditDeviceData(d) => Ok(d),
            Data::HomeNetData(_) => Err(Error::ConversionError)
        }
    }
}

pub_struct!(
#[derive(Serialize, Deserialize)]
struct EditDeviceData {
    vars: EditDeviceVars
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditDeviceVars {
    ipv6_enabled: bool,
    ip_mask: String,
    plc: Plc,
    is_mac_filter_active: bool,
    dev: EditDeviceDevice,
    back_to_page: String,
    dev_node: String
}
);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditDeviceDevice {
    // dev_details: DevDetails,
    #[serde(rename="UID")]
    uid: String,
    // TODO a.json line 37
}

pub_struct!(
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Plc {
    emv_norm_en50561: EmvNormEN50561
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmvNormEN50561 {
    can_optimize: bool,
    optimize: bool
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct HomeNetData {
    searching: bool,
    ipclient: bool,
    fwcheck: Fwcheck,
    updating: String,
    topology: Topology,
    nexusclient: bool,
    devices: Vec<Device>,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Connection {
    dsl_diagnosis: bool,
    medium_upstream: u64,
    downstream: u64,
    role: String,
    provider: String,
    ipv4: Ipv4,
    connected: bool,
    shapedrate: bool,
    direct_connection: bool,
    ready_for_fallback: bool,
    medium_downstream: u64,
    state: String,
    upstream: u64,
    name: String,
    #[serde(rename = "type")]
    connection_type: String,
    active: bool,
    ipv6: Ipv6,
    speed_manual: bool,
    medium: String,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Ipv4 {
    connected: bool,
    dns: Vec<Dns>,
    dslite: bool,
    ip: String,
    since: u64,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Dns {
    #[serde(rename = "type")]
    dns_type: String,
    ip: String,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Ipv6 {
    ip_lifetime: Lifetime,
    connected: bool,
    dns: Vec<Dns>,
    ip: String,
    prefix: String,
    prefix_lifetime: Lifetime,
    since: u64,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Lifetime {
    valid: u64,
    preferred: u64,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct ConninfoConninfo {
    kind: String,
    speed: Option<String>,
    bandinfo: Option<Vec<BandInfo>>,
    usedbands: Option<i64>,
    desc: String,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct BandInfo {
    band: u64,
    speed_tx: u64,
    speed_rx: u64,
    speed: String,
    desc: String,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct DetailInfo {
    edit: Edit,
    portrelease: bool,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Edit {
    pid: EditPid,
    params: Params,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Params {
    dev: String,
    back_to_page: BackToPageEnum,
}
);

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

pub_struct!(
#[derive(Serialize, Deserialize)]
struct NameInfo {
    name: String,
    product: Option<String>,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Phone {
    number_count: u64,
    active_count: u64,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct UpdateInfo {
    state: State,
}
);

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Current,
    None,
}

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Wlaninfo {
    text: String,
    title: String,
    shorttitle: String,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Fwcheck {
    notdone: bool,
    nocheck: bool,
    auto: bool,
    started: bool,
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct Topology {
    rootuid: String,
    devices: HashMap<String, Device>,
}
);

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Device {
    RouterDevice(RouterDevice),
    PLCDevice(PLCDevice),
    DefaultDevice(DefaultDevice),
}

pub_struct!(
#[derive(Serialize, Deserialize)]
struct DefaultDevice {
    own_client_device: bool,
    dist: u64,
    parent: String,
    #[serde(rename="UID")]
    uid: String,
    category: Category,
    switch: bool,
    children: Vec<String>,
    devtype: String,
    ownentry: bool,
    stateinfo: StateInfo,
    conn: Conn,
    master: bool,
    ipinfo: String,
    updateinfo: UpdateInfo,
    gateway: bool,
    nameinfo: NameInfo,
    detailinfo: DetailInfo,
    conninfo: ConnInfo
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct RouterDevice {
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
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct PLCDevice {
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
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct RouterDetailInfo {
    wlan24: bool,
    wlan5: bool,
    guestaccess: bool
}
);

#[derive(Serialize, Deserialize)]
pub enum DevType {
    #[serde(rename="fritzbox")]
    FritzBox
}

pub_struct!(
#[derive(Serialize, Deserialize)]
struct RouterStateInfo {
    nexustrust: bool,
    active: bool
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct StateInfo {
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
);


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

pub_struct!(
#[derive(Serialize, Deserialize)]
struct WiredConnInfo {
    speed: Option<String>,
    kind: WiredConnKind,
    desc: String
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct WirelessConnInfo {
    speed: String,
    kind: WirelessConnKind,
    bandinfo: Vec<BandInfo>,
    usedbands: u64,
    desc: String
}
);

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

pub_struct!(
#[derive(Serialize, Deserialize)]
struct VersionInfo {
    version: String
}
);

pub_struct!(
#[derive(Serialize, Deserialize)]
struct RouterVersionInfo {
    version: String,
    fos: bool
}
);