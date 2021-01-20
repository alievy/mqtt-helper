use mosquitto_mqtt as mqtt;
use mqtt::Mosquitto;
use std::vec::Vec;

use crate::Data;
use crate::{Error, Result};
use log::debug;
use std::collections::HashMap;
use std::ffi::CString;
use std::path::PathBuf;

pub struct MqttHandle {
    pub mosquitto: Mosquitto,
    pub client_id: String,
    pub devices: HashMap<String, Data>,
}

pub struct MqttHelper {
    mqtt_handle_list: Vec<MqttHandle>,
}

impl MqttHelper {
    /// Setup user & password.
    fn setup_userpass(&mut self, client_id: &str, username: &str, password: &str) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| mosq.mosquitto.set_username_password(username, password))
    }

    /// Setup TLS.
    fn setup_tls(&mut self, client_id: &str, ca_cert: Option<&PathBuf>) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| {
                if let Some(cert) = ca_cert {
                    mosq.mosquitto.tls_set_using_ca_file(cert)?;
                }
                Ok(())
            })
    }

    /// Setup callback.
    fn setup_callback(&mut self, client_id: &str) -> Result<()> {
        self.find_handler_mut(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| {
                mosq.mosquitto.callback_init();
                Ok(())
            })
    }
}

impl MqttHelper {
    /// Find and fetch a mutable MQTT handler.
    fn find_handler_mut(&mut self, client_id: &str) -> Option<&mut MqttHandle> {
        self.mqtt_handle_list
            .iter_mut()
            .find(|handler| handler.client_id == client_id)
    }

    ///  Find and fetch MQTT handler.
    fn find_handler(&self, client_id: &str) -> Option<&MqttHandle> {
        self.mqtt_handle_list
            .iter()
            .find(|handler| handler.client_id == client_id)
    }
}

impl MqttHelper {
    pub fn new(client_id: &str) -> Result<Self> {
        debug!("Initiate Mosquitto");
        // Initiate Mosquitto
        let mosquitto_mqtt = Mosquitto::new(client_id)?;
        // Mqtt Handle list
        let mut mqtt_handle_list: Vec<MqttHandle> = Vec::new();
        // Devices map
        let mut devices_map = HashMap::new();
        // Data info (e.g: Topic and local_id)
        let data = Data::new(None, Some(client_id.to_string()));
        devices_map.insert(client_id.to_string(), data);

        let mqtt_handle = MqttHandle {
            mosquitto: mosquitto_mqtt,
            client_id: client_id.into(),
            devices: devices_map,
        };

        mqtt_handle_list.push(mqtt_handle);

        Ok(MqttHelper { mqtt_handle_list })
    }

    /// Setup user/pass, callback and TLS.
    pub fn setup(
        &mut self,
        client_id: &str,
        username: &str,
        password: &str,
        ca_cert: Option<&PathBuf>,
    ) -> Result<()> {
        self.setup_userpass(client_id, username, password)?;
        self.setup_callback(client_id)?;
        if ca_cert.is_some() {
            self.setup_tls(client_id, ca_cert)?;
        }
        Ok(())
    }

    /// Connect to broker.
    pub fn connect(
        &mut self,
        client_id: &str,
        hostname: &str,
        port: i32,
        keep_alive: i32,
    ) -> Result<()> {
        self.find_handler_mut(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| mosq.mosquitto.connect(hostname, port, keep_alive))
    }

    /// Disconnect from broker.
    pub fn disconnect(&mut self, client_id: &str) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| mosq.mosquitto.disconnect())
    }

    /// Reconnect to broker.
    pub fn reconnect(&mut self, client_id: &str) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| mosq.mosquitto.reconnect())
    }

    /// Get mqtt socket
    pub fn socket(&self, client_id: &str) -> Result<i32> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| mosq.mosquitto.socket())
    }

    /// Run mqtt_loop
    pub fn run_loop(&self, client_id: &str, timeout: i32, maxpackets: i32) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| mosq.mosquitto.mqtt_loop(timeout, maxpackets))
    }

    /// Run mqtt_loop_start
    pub fn run_loop_start(&self, client_id: &str) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| mosq.mosquitto.mqtt_loop_start())
    }

    pub fn alternative_loop(&self, client_id: &str) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| self.alternative_loop_ex(mosq))
    }

    pub fn alternative_loop_ex(&self, handle: &MqttHandle) -> Result<()> {
        while handle.mosquitto.loop_want_write() {
            handle.mosquitto.loop_write(10)?;
        }
        handle.mosquitto.loop_misc()?;
        Ok(())
    }

    /// Set connect_callback.
    pub fn set_connect_callback<C>(&mut self, client_id: &str, callback: C) -> Result<()>
    where
        C: Fn(i32),
        C: 'static,
    {
        self.find_handler_mut(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| {
                mosq.mosquitto.set_connect_callback(callback);
                Ok(())
            })
    }

    /// Set disconnect_callback.
    pub fn set_disconnect_callback<C>(&mut self, client_id: &str, callback: C) -> Result<()>
    where
        C: Fn(i32),
        C: 'static,
    {
        self.find_handler_mut(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| {
                mosq.mosquitto.set_disconnect_callback(callback);
                Ok(())
            })
    }

    /// Publish to broker.
    pub fn publish(&self, client_id: &str, topic: &str, payload: &str) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| {
                let data = CString::new(payload)?;
                let data_ptr = data.as_bytes_with_nul();

                mosq.mosquitto.publish(topic, data_ptr)
            })
    }

    /// Subscribe to topic.
    pub fn subscribe(&self, client_id: &str, topic: &str) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| {
                for data in mosq.devices.values() {
                    if data.topic_is_found(topic) {
                        return Err(Error::TopicNotFound.into());
                    }
                }

                mosq.mosquitto.subscribe(topic)
            })
    }

    /// Unsubcribe topic.
    pub fn unsubscribe(&self, client_id: &str, topic: &str) -> Result<()> {
        self.find_handler(client_id)
            .ok_or_else(|| Error::ClientId.into())
            .and_then(|mosq| {
                for data in mosq.devices.values() {
                    if !data.topic_is_found(topic) {
                        return Err(Error::TopicNotFound.into());
                    }
                }

                mosq.mosquitto.unsubscribe(topic)
            })
    }
}
