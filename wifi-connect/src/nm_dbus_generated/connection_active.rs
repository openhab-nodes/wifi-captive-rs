// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use dbus;
use dbus::arg;
use dbus::nonblock;

pub trait Properties {
    fn get(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> nonblock::MethodReply<arg::Variant<Box<dyn arg::RefArg + 'static>>>;
    fn get_all(
        &self,
        interface_name: &str,
    ) -> nonblock::MethodReply<
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    >;
    fn set(
        &self,
        interface_name: &str,
        property_name: &str,
        value: arg::Variant<Box<dyn arg::RefArg>>,
    ) -> nonblock::MethodReply<()>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> Properties
    for nonblock::Proxy<'a, C>
{
    fn get(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> nonblock::MethodReply<arg::Variant<Box<dyn arg::RefArg + 'static>>> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "Get",
            (interface_name, property_name),
        )
        .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>,)| Ok(r.0))
    }

    fn get_all(
        &self,
        interface_name: &str,
    ) -> nonblock::MethodReply<
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    > {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "GetAll",
            (interface_name,),
        )
        .and_then(
            |r: (
                ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
            )| Ok(r.0),
        )
    }

    fn set(
        &self,
        interface_name: &str,
        property_name: &str,
        value: arg::Variant<Box<dyn arg::RefArg>>,
    ) -> nonblock::MethodReply<()> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "Set",
            (interface_name, property_name, value),
        )
    }
}

#[derive(Debug)]
pub struct PropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for PropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for PropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(PropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for PropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait Introspectable {
    fn introspect(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> Introspectable
    for nonblock::Proxy<'a, C>
{
    fn introspect(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait Peer {
    fn ping(&self) -> nonblock::MethodReply<()>;
    fn get_machine_id(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> Peer
    for nonblock::Proxy<'a, C>
{
    fn ping(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait OrgFreedesktopNetworkManagerConnectionActive {
    fn connection(&self) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn specific_object(&self) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn id(&self) -> nonblock::MethodReply<String>;
    fn uuid(&self) -> nonblock::MethodReply<String>;
    fn type_(&self) -> nonblock::MethodReply<String>;
    fn devices(&self) -> nonblock::MethodReply<Vec<dbus::Path<'static>>>;
    fn state(&self) -> nonblock::MethodReply<u32>;
    fn state_flags(&self) -> nonblock::MethodReply<u32>;
    fn default(&self) -> nonblock::MethodReply<bool>;
    fn ip4_config(&self) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn dhcp4_config(&self) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn default6(&self) -> nonblock::MethodReply<bool>;
    fn ip6_config(&self) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn dhcp6_config(&self) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn vpn(&self) -> nonblock::MethodReply<bool>;
    fn master(&self) -> nonblock::MethodReply<dbus::Path<'static>>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopNetworkManagerConnectionActive for nonblock::Proxy<'a, C>
{
    fn connection(&self) -> nonblock::MethodReply<dbus::Path<'static>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Connection",
        )
    }

    fn specific_object(&self) -> nonblock::MethodReply<dbus::Path<'static>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "SpecificObject",
        )
    }

    fn id(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Id",
        )
    }

    fn uuid(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Uuid",
        )
    }

    fn type_(&self) -> nonblock::MethodReply<String> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Type",
        )
    }

    fn devices(&self) -> nonblock::MethodReply<Vec<dbus::Path<'static>>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Devices",
        )
    }

    fn state(&self) -> nonblock::MethodReply<u32> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "State",
        )
    }

    fn state_flags(&self) -> nonblock::MethodReply<u32> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "StateFlags",
        )
    }

    fn default(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Default",
        )
    }

    fn ip4_config(&self) -> nonblock::MethodReply<dbus::Path<'static>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Ip4Config",
        )
    }

    fn dhcp4_config(&self) -> nonblock::MethodReply<dbus::Path<'static>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Dhcp4Config",
        )
    }

    fn default6(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Default6",
        )
    }

    fn ip6_config(&self) -> nonblock::MethodReply<dbus::Path<'static>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Ip6Config",
        )
    }

    fn dhcp6_config(&self) -> nonblock::MethodReply<dbus::Path<'static>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Dhcp6Config",
        )
    }

    fn vpn(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Vpn",
        )
    }

    fn master(&self) -> nonblock::MethodReply<dbus::Path<'static>> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Connection.Active",
            "Master",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerConnectionActivePropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerConnectionActivePropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerConnectionActivePropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(
            OrgFreedesktopNetworkManagerConnectionActivePropertiesChanged {
                properties: i.read()?,
            },
        )
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerConnectionActivePropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Connection.Active";
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
    pub state: u32,
    pub reason: u32,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.state, i);
        arg::RefArg::append(&self.reason, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
            state: i.read()?,
            reason: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerConnectionActiveStateChanged {
    const NAME: &'static str = "StateChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Connection.Active";
}
