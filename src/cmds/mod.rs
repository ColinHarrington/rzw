//! ZWave command classes - mid layer
//!
//! The command classes together with the message building up the
//! mid layer of the crate, which can be used easely by a developer.
//!
//! The top layer is only needed, if the handling of the devices
//! and their actual status should be also done by this crate.
//!
//! If the full control over the devices and is required, take this layer.

pub mod basic;
pub mod info;
pub mod meter;
pub mod powerlevel;
pub mod switch_binary;
pub mod switch_multilevel;

use enum_primitive::FromPrimitive;
use error::{Error, ErrorKind};

enum_from_primitive! {
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// List of the ZWave Command Classes
pub enum CommandClass {
    NO_OPERATION = 0x00,
    NODE_INFO = 0x01,
    REQUEST_NODE_INFO = 0x02,
    ASSIGN_IDS = 0x03,
    FIND_NODES_IN_RANGE = 0x04,
    GET_NODES_IN_RANGE = 0x05,
    RANGE_INFO = 0x06,
    CMD_COMPLETE = 0x07,
    TRANSFER_PRESENTATION = 0x08,
    TRANSFER_NODE_INFO = 0x09,
    TRANSFER_RANGE_INFO = 0x0A,
    TRANSFER_END = 0x0B,
    ASSIGN_RETURN_ROUTE = 0x0C,
    NEW_NODE_REGISTERED = 0x0D,
    NEW_RANGE_REGISTERED = 0x0E,
    TRANSFER_NEW_PRIMARY_COMPLETE = 0x0F,
    AUTOMATIC_CONTROLLER_UPDATE_START = 0x10,
    SUC_NODE_ID = 0x11,
    SET_SUC = 0x12,
    SET_SUC_ACK = 0x13,
    ASSIGN_SUC_RETURN_ROUTE = 0x14,
    STATIC_ROUTE_REQUEST = 0x15,
    LOST = 0x16,
    ACCEPT_LOST = 0x17,
    NOP_POWER = 0x18,
    RESERVE_NODE_IDS = 0x19,
    RESERVED_IDS = 0x1A,
    // Unknown
    BASIC = 0x20,
    CONTROLLER_REPLICATION = 0x21,
    APPLICATION_STATUS = 0x22,
    ZIP_SERVICES = 0x23,
    ZIP_SERVER = 0x24,
    SWITCH_BINARY = 0x25,
    SWITCH_MULTILEVEL = 0x26,
    SWITCH_ALL = 0x27,
    SWITCH_TOGGLE_BINARY = 0x28,
    SWITCH_TOGGLE_MULTILEVEL = 0x29,
    CHIMNEY_FAN = 0x2A,
    SCENE_ACTIVATION = 0x2B,
    SCENE_ACTUATOR_CONF = 0x2C,
    SCENE_CONTROLLER_CONF = 0x2D,
    ZIP_CLIENT = 0x2E,
    ZIP_ADV_SERVICES = 0x2F,
    SENSOR_BINARY = 0x30,
    SENSOR_MULTILEVEL = 0x31,
    METER = 0x32,
    ZIP_ADV_SERVER = 0x33,
    ZIP_ADV_CLIENT = 0x34,
    METER_PULSE = 0x35,
    METER_TBL_CONFIG = 0x3C,
    METER_TBL_MONITOR = 0x3D,
    METER_TBL_PUSH = 0x3E,
    THERMOSTAT_HEATING = 0x38,
    THERMOSTAT_MODE = 0x40,
    THERMOSTAT_OPERATING_STATE = 0x42,
    THERMOSTAT_SETPOINT = 0x43,
    THERMOSTAT_FAN_MODE = 0x44,
    THERMOSTAT_FAN_STATE = 0x45,
    CLIMATE_CONTROL_SCHEDULE = 0x46,
    THERMOSTAT_SETBACK = 0x47,
    TARIF_CONFIG = 0x4A,
    TARIF_TABLE_MONITOR = 0x4B,
    COMMAND_CLASS_DOOR_LOCK_LOGGING = 0x4C,
    SCHEDULE_ENTRY_LOCK = 0x4E,
    ZIP_6LOWPAN = 0x4F,
    BASIC_WINDOW_COVERING = 0x50,
    MTP_WINDOW_COVERING = 0x51,
    MULTI_INSTANCE = 0x60,
    DOOR_LOCK = 0x62,
    USER_CODE = 0x63,
    CONFIGURATION = 0x70,
    ALARM = 0x71,
    MANUFACTURER_SPECIFIC = 0x72,
    POWER_LEVEL = 0x73,
    PROTECTION = 0x75,
    LOCK = 0x76,
    NODE_NAMING = 0x77,
    FIRMWARE_UPDATE_MD = 0x7A,
    GROUPING_NAME = 0x7B,
    REMOTE_ASSOCIATION_ACTIVATE = 0x7C,
    REMOTE_ASSOCIATION = 0x7D,
    BATTERY = 0x80,
    CLOCK = 0x81,
    HAIL = 0x82,
    WAKE_UP = 0x84,
    ASSOCIATION = 0x85,
    VERSION = 0x86,
    INDICATOR = 0x87,
    PROPRIETARY = 0x88,
    LANGUAGE = 0x89,
    TIME = 0x8A,
    TIME_PARAMETERS = 0x8B,
    GEOGRAPHIC_LOCATION = 0x8C,
    COMPOSITE = 0x8D,
    MULTI_INSTANCE_ASSOCIATION = 0x8E,
    MULTI_CMD = 0x8F,
    ENERGY_PRODUCTION = 0x90,
    MANUFACTURER_PROPRIETARY = 0x91,
    SCREEN_MD = 0x92,
    SCREEN_ATTRIBUTES = 0x93,
    SIMPLE_AV_CONTROL = 0x94,
    AV_CONTENT_DIRECTORY_MD = 0x95,
    AV_RENDERER_STATUS = 0x96,
    AV_CONTENT_SEARCH_MD = 0x97,
    SECURITY = 0x98,
    AV_TAGGING_MD = 0x99,
    IP_CONFIGURATION = 0x9A,
    ASSOCIATION_COMMAND_CONFIGURATION = 0x9B,
    SENSOR_ALARM = 0x9C,
    SILENCE_ALARM = 0x9D,
    SENSOR_CONFIGURATION = 0x9E,
    MARK = 0xEF,
    NON_INTEROPERABLE = 0xF0,
}
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum MeterData {
    Electric_kWh(f64),
    Electric_kVAh(f64),
    Electric_W(f64),
    Electric_PulseCount(f64),
    Gas_meter2(f64),
    Gas_feet2(f64),
    Gas_PulseCount(f64),
    Water_meter2(f64),
    Water_feet2(f64),
    Water_Gallons(f64),
    Water_PulseCount(f64),
}

impl MeterData {
    pub fn get_scale(&self) -> u8 {
        match *self {
            MeterData::Electric_kWh(_) => 0x00,
            MeterData::Electric_kVAh(_) => 0x01,
            MeterData::Electric_W(_) => 0x02,
            MeterData::Electric_PulseCount(_) => 0x03,
            MeterData::Gas_meter2(_) => 0x00,
            MeterData::Gas_feet2(_) => 0x01,
            MeterData::Gas_PulseCount(_) => 0x03,
            MeterData::Water_meter2(_) => 0x00,
            MeterData::Water_feet2(_) => 0x01,
            MeterData::Water_Gallons(_) => 0x02,
            MeterData::Water_PulseCount(_) => 0x03,
        }
    }
}

/// ZWave message to write and read
///
/// The message represent a ZWave message which can be sent or received.
/// To build up such a message use the following implementation.
///
/// ```rust
/// use rzw::cmds::{Message, CommandClass};
///
/// Message::new(0x02, CommandClass::BASIC, 0x01, vec!(0xFF));
/// ```
///
/// The structure of a ZWave message looks like the following:
///
/// `device, data-length, comand class, command, value`
#[derive(Debug, Clone)]
pub struct Message {
    pub node_id: u8,
    pub cmd_class: CommandClass,
    pub cmd: u8,
    pub data: Vec<u8>,
    pub raw: Vec<u8>,
}

impl Message {
    pub fn new(node_id: u8, cmd_class: CommandClass, cmd: u8, data: Vec<u8>) -> Message {
        Message {
            node_id: node_id,
            cmd_class: cmd_class,
            cmd: cmd,
            data: data,
            raw: Vec::new(),
        }
    }

    /// create a new message
    pub fn new_with_raw(
        node_id: u8,
        cmd_class: CommandClass,
        cmd: u8,
        data: Vec<u8>,
        raw: Vec<u8>,
    ) -> Message {
        Message {
            node_id: node_id,
            cmd_class: cmd_class,
            cmd: cmd,
            data: data,
            raw: raw,
        }
    }

    /// Parse a `&[u8]` slice and try to convert it to a `Message`
    pub fn parse(data: &[u8]) -> Result<Message, Error> {
        let raw = data.to_vec();
        // check if the data is avilable
        if data.len() < 1 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message has no data"));
        }

        // check if the data has enough entries
        if data.len() < 3 {
            return Err(Error::new(ErrorKind::UnknownZWave, "Message is too short"));
        }

        // check if the length flag matches
        //if data.len() - 2 != data[1] as usize {
        //    return Err(Error::new(ErrorKind::UnknownZWave, "The length of the message delivered didn't match with the actual length"));
        //}

        // get the node id
        let node_id = data[1];

        // get the commadn class
        let cmd_class = CommandClass::from_u8(data[3]).unwrap_or(CommandClass::NO_OPERATION);

        // get the command
        let cmd = data[3];

        // create the message data array
        let msg_data: &[u8];

        // when there is data extract it
        if data.len() > 4 {
            msg_data = &data[4..(data.len())];
        }
        // if not create a empty array
        else {
            msg_data = &[0; 0];
        }

        // create a new Message and return it
        Ok(Message::new_with_raw(
            node_id,
            cmd_class,
            cmd,
            msg_data.to_vec(),
            raw,
        ))
    }

    /// Return the message as Vec<u8>
    pub fn to_vec(&self) -> Vec<u8> {
        // todo check if there a better way
        let mut v: Vec<u8> = Vec::new();
        v.push(self.node_id);
        v.push((self.data.len() + 2) as u8);
        v.push(self.cmd_class as u8);
        v.push(self.cmd);
        v.append(&mut self.data.clone());
        v
    }
}

impl From<Message> for Vec<u8> {
    /// Convert the message to a u8 vector
    fn from(message: Message) -> Self {
        message.to_vec()
    }
}

impl From<Message> for String {
    /// Convert the message to a hex converted string
    fn from(message: Message) -> Self {
        let data = message.to_vec();
        let mut out = String::new();

        for i in 0..data.len() {
            out.push_str(&*format!("{:#X} ", data[i]));
        }

        out
    }
}
