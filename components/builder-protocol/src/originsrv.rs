// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt;
use std::result;
use std::str::FromStr;

use hab_core;
use hab_core::package::{self, Identifiable, FromArchive, PackageArchive};

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub use message::originsrv::*;
use message::{Persistable, Routable};
use sharding::InstaId;

impl Persistable for Origin {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Routable for OriginGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: This won't accurately find the origin without it. We can switch to using the ID
        // of the origin or perform a reverse lookup by storing the name->ID map on a particular
        // originsrv server.
        Some(self.get_name().to_string())
    }
}

impl Routable for OriginCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_name().to_string())
    }
}

impl Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin", 4));
        try!(strukt.serialize_field("id", &self.get_id().to_string()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id().to_string()));
        try!(strukt.serialize_field("private_key_name", self.get_private_key_name()));
        strukt.end()
    }
}

impl Routable for OriginMemberRemove {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginMemberListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Persistable for OriginSecretKey {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for OriginSecretKey {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_secret_key", 6));
        try!(strukt.serialize_field("id", &self.get_id().to_string()));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id().to_string()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("revision", self.get_revision()));
        try!(strukt.serialize_field("body", self.get_body()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id().to_string()));
        strukt.end()
    }
}

impl Routable for OriginSecretKeyCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginSecretKeyGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_origin()))
    }
}

impl Persistable for OriginPublicKey {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for OriginPublicKey {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_public_key", 6));
        try!(strukt.serialize_field("id", &self.get_id().to_string()));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id().to_string()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("revision", self.get_revision()));
        try!(strukt.serialize_field("body", self.get_body()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id().to_string()));
        strukt.end()
    }
}

impl Routable for OriginPublicKeyCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginPublicKeyGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin().to_string())
    }
}

impl Routable for OriginPublicKeyLatestGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin().to_string())
    }
}

impl Routable for OriginPublicKeyListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginPublicKeyListResponse {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Serialize for OriginPublicKeyListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_public_key_list_response", 2));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id().to_string()));
        try!(strukt.serialize_field("keys", self.get_keys()));
        strukt.end()
    }
}

impl Routable for OriginInvitationCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Persistable for OriginInvitation {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for OriginInvitation {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_invitation", 6));
        try!(strukt.serialize_field("id", &self.get_id().to_string()));
        try!(strukt.serialize_field("account_id", &self.get_account_id().to_string()));
        try!(strukt.serialize_field("account_name", self.get_account_name()));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id().to_string()));
        try!(strukt.serialize_field("origin_name", self.get_origin_name()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id().to_string()));
        strukt.end()
    }
}

impl Routable for AccountInvitationListRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_id())
    }
}

impl Routable for AccountInvitationListResponse {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_id())
    }
}

impl Serialize for AccountInvitationListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("account_invitation_list_response", 2));
        try!(strukt.serialize_field("account_id", &self.get_account_id().to_string()));
        try!(strukt.serialize_field("invitations", self.get_invitations()));
        strukt.end()
    }
}

impl Routable for OriginInvitationListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginInvitationListResponse {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Serialize for OriginInvitationListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_invitation_list_response", 2));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id().to_string()));
        try!(strukt.serialize_field("invitations", self.get_invitations()));
        strukt.end()
    }
}

impl Routable for OriginInvitationAcceptRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_invite_id()))
    }
}

impl Serialize for OriginMemberListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_member_list_response", 2));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id().to_string()));
        try!(strukt.serialize_field("members", self.get_members()));
        strukt.end()
    }
}

impl Routable for CheckOriginAccessRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin_name().to_string())
    }
}

impl Routable for OriginProjectGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        let name = self.get_name();
        let origin_name = match name.split('/').nth(0) {
            Some(origin_name) => origin_name,
            None => {
                println!("Cannot route origin project get; malformed project name - routing on \
                        screwedup to not kill the service");
                "screwedup"
            }
        };
        Some(String::from(origin_name))
    }
}

impl Routable for OriginProjectCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_project().get_origin_id()))
    }
}

impl Routable for OriginProjectUpdate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_project().get_origin_id()))
    }
}

impl Routable for OriginProjectDelete {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        let name = self.get_name();
        let origin_name = match name.split('/').nth(0) {
            Some(origin_name) => origin_name,
            None => {
                println!("Cannot route origin project get; malformed project name - routing on \
                        screwedup to not kill the service");
                "screwedup"
            }
        };
        Some(String::from(origin_name))
    }
}

impl Serialize for OriginProject {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("project", 2));
        try!(state.serialize_field("id", &self.get_id().to_string()));
        try!(state.serialize_field("origin_id", &self.get_origin_id().to_string()));
        try!(state.serialize_field("origin_name", self.get_origin_name()));
        try!(state.serialize_field("package_name", self.get_package_name()));
        try!(state.serialize_field("name", self.get_name()));
        try!(state.serialize_field("plan_path", self.get_plan_path()));
        try!(state.serialize_field("owner_id", &self.get_owner_id().to_string()));
        try!(state.serialize_field("vcs_type", self.get_vcs_type()));
        try!(state.serialize_field("vcs_data", self.get_vcs_data()));
        state.end()
    }
}

impl Identifiable for OriginPackageIdent {
    fn origin(&self) -> &str {
        self.get_origin()
    }

    fn name(&self) -> &str {
        self.get_name()
    }

    fn version(&self) -> Option<&str> {
        let ver = self.get_version();
        if ver.is_empty() { None } else { Some(ver) }
    }

    fn release(&self) -> Option<&str> {
        let rel = self.get_release();
        if rel.is_empty() { None } else { Some(rel) }
    }
}

impl fmt::Display for OriginPackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.get_version().is_empty() && !self.get_release().is_empty() {
            write!(f,
                   "{}/{}/{}/{}",
                   self.get_origin(),
                   self.get_name(),
                   self.get_version(),
                   self.get_release())
        } else if !self.get_version().is_empty() {
            write!(f,
                   "{}/{}/{}",
                   self.get_origin(),
                   self.get_name(),
                   self.get_version())
        } else {
            write!(f, "{}/{}", self.get_origin(), self.get_name())
        }
    }
}

impl Routable for OriginPackageGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_ident().get_origin()))
    }
}

impl Routable for OriginPackageLatestGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_ident().get_origin()))
    }
}

impl Routable for OriginPackageCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginPackageUniqueListRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_origin()))
    }
}

impl Routable for OriginPackageListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginPackageSearchRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_origin()))
    }
}

impl Serialize for OriginPackageIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_package_ident", 4));
        try!(strukt.serialize_field("origin", self.get_origin()));
        try!(strukt.serialize_field("name", self.get_name()));
        if !self.get_version().is_empty() {
            try!(strukt.serialize_field("version", self.get_version()));
        }
        if !self.get_release().is_empty() {
            try!(strukt.serialize_field("release", self.get_release()));
        }
        strukt.end()
    }
}

impl Serialize for OriginPackage {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_package", 8));
        try!(strukt.serialize_field("ident", self.get_ident()));
        try!(strukt.serialize_field("checksum", self.get_checksum()));
        try!(strukt.serialize_field("manifest", self.get_manifest()));
        try!(strukt.serialize_field("target", self.get_target()));
        try!(strukt.serialize_field("deps", self.get_deps()));
        try!(strukt.serialize_field("tdeps", self.get_tdeps()));
        try!(strukt.serialize_field("exposes", self.get_exposes()));
        try!(strukt.serialize_field("config", self.get_config()));
        strukt.end()
    }
}

impl From<hab_core::package::PackageIdent> for OriginPackageIdent {
    fn from(value: hab_core::package::PackageIdent) -> OriginPackageIdent {
        let mut ident = OriginPackageIdent::new();
        ident.set_origin(value.origin);
        ident.set_name(value.name);
        if let Some(ver) = value.version {
            ident.set_version(ver);
        }
        if let Some(rel) = value.release {
            ident.set_release(rel);
        }
        ident
    }
}

impl FromArchive for OriginPackageCreate {
    type Error = hab_core::Error;

    fn from_archive(archive: &mut PackageArchive) -> hab_core::Result<Self> {
        let ident = match archive.ident() {
            Ok(value) => OriginPackageIdent::from(value),
            Err(e) => return Err(hab_core::Error::from(e)),
        };
        let manifest = try!(archive.manifest());
        let deps = try!(archive.deps())
            .into_iter()
            .map(|d| d.into())
            .collect();
        let tdeps = try!(archive.tdeps())
            .into_iter()
            .map(|d| d.into())
            .collect();
        let exposes = try!(archive.exposes())
            .into_iter()
            .map(|d| d as u32)
            .collect();
        let config = try!(archive.config());
        let checksum = try!(archive.checksum());
        let target = try!(archive.target());

        let mut package = OriginPackageCreate::new();
        package.set_ident(ident);
        package.set_manifest(manifest);
        package.set_target(target.to_string());
        package.set_deps(deps);
        package.set_tdeps(tdeps);
        package.set_exposes(exposes);
        if let Some(cfg) = config {
            package.set_config(cfg);
        }
        package.set_checksum(checksum);
        Ok(package)
    }
}

impl FromArchive for OriginPackage {
    type Error = hab_core::Error;

    fn from_archive(archive: &mut PackageArchive) -> hab_core::Result<Self> {
        let ident = match archive.ident() {
            Ok(value) => OriginPackageIdent::from(value),
            Err(e) => return Err(hab_core::Error::from(e)),
        };
        let manifest = try!(archive.manifest());
        let deps = try!(archive.deps())
            .into_iter()
            .map(|d| d.into())
            .collect();
        let tdeps = try!(archive.tdeps())
            .into_iter()
            .map(|d| d.into())
            .collect();
        let exposes = try!(archive.exposes())
            .into_iter()
            .map(|d| d as u32)
            .collect();
        let config = try!(archive.config());
        let checksum = try!(archive.checksum());
        let target = try!(archive.target());

        let mut package = OriginPackage::new();
        package.set_ident(ident);
        package.set_manifest(manifest);
        package.set_target(target.to_string());
        package.set_deps(deps);
        package.set_tdeps(tdeps);
        package.set_exposes(exposes);
        if let Some(cfg) = config {
            package.set_config(cfg);
        }
        package.set_checksum(checksum);
        Ok(package)
    }
}

impl fmt::Display for OriginPackage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get_ident().fmt(f)
    }
}

impl Serialize for OriginKeyIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_key", 3));
        try!(strukt.serialize_field("origin", self.get_origin()));
        try!(strukt.serialize_field("revision", self.get_revision()));
        try!(strukt.serialize_field("location", self.get_location()));
        strukt.end()
    }
}

impl Into<package::PackageIdent> for OriginPackageIdent {
    fn into(self) -> package::PackageIdent {
        package::PackageIdent::new(self.get_origin(),
                                   self.get_name(),
                                   Some(self.get_version()),
                                   Some(self.get_release()))
    }
}

impl Into<package::PackageIdent> for OriginPackage {
    fn into(self) -> package::PackageIdent {
        self.get_ident().clone().into()
    }
}

impl FromStr for OriginPackageIdent {
    type Err = hab_core::Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let items: Vec<&str> = value.split("/").collect();
        let mut ident = OriginPackageIdent::new();
        ident.set_origin(items[0].to_string());
        ident.set_name(items[1].to_string());
        ident.set_version(items[2].to_string());
        ident.set_release(items[3].to_string());
        Ok(ident)
    }
}

impl Persistable for OriginChannel {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}


impl Serialize for OriginChannel {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin", 4));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id()));
        strukt.end()
    }
}

impl Serialize for OriginChannelIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_key", 3));
        try!(strukt.serialize_field("name", self.get_name()));
        strukt.end()
    }
}

impl Routable for OriginChannelCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginChannelListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginChannelListResponse {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Serialize for OriginChannelListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_channel_list_response", 2));
        try!(strukt.serialize_field("channels", self.get_channels()));
        strukt.end()
    }
}
