use chrono::prelude::*;
use codec::{Decode, Encode};
pub use sp_application_crypto::ed25519;
pub use sp_core::crypto::AccountId32;
pub use sp_core::H256 as Hash;
use std::fmt::{self, Display};
pub use substrate_api_client::{AccountData, AccountInfo};
pub use support::traits::BalanceStatus;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum NodeCertification {
    Diy,
    Certified,
}

impl Default for NodeCertification {
    fn default() -> Self {
        NodeCertification::Diy
    }
}

impl From<tfchain_support::types::NodeCertification> for NodeCertification {
    fn from(ct: tfchain_support::types::NodeCertification) -> Self {
        match ct {
            tfchain_support::types::NodeCertification::Diy => NodeCertification::Diy,
            tfchain_support::types::NodeCertification::Certified => NodeCertification::Certified,
        }
    }
}

impl From<pallet_tfgrid_legacy::types::CertificationType> for NodeCertification {
    fn from(ct: pallet_tfgrid_legacy::types::CertificationType) -> Self {
        match ct {
            pallet_tfgrid_legacy::types::CertificationType::Diy => NodeCertification::Diy,
            pallet_tfgrid_legacy::types::CertificationType::Certified => {
                NodeCertification::Certified
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct FarmingPolicy<BlockNumber> {
    pub version: u32,
    pub id: u32,
    pub name: String,
    pub cu: u32,
    pub su: u32,
    pub nu: u32,
    pub ipv4: u32,
    pub minimal_uptime: u16,
    pub policy_created: BlockNumber,
    pub policy_end: BlockNumber,
    pub immutable: bool,
    pub default: bool,
    pub node_certification: NodeCertification,
    pub farm_certification: FarmCertification,
}

impl<BlockNumber> From<pallet_tfgrid::types::FarmingPolicy<BlockNumber>>
    for FarmingPolicy<BlockNumber>
{
    fn from(fp: pallet_tfgrid::types::FarmingPolicy<BlockNumber>) -> Self {
        let pallet_tfgrid::types::FarmingPolicy {
            // Minimal uptime in order to benefit from this uptime.
            version,
            id,
            name,
            cu,
            su,
            nu,
            ipv4,
            minimal_uptime,
            policy_created,
            policy_end,
            immutable,
            default,
            node_certification,
            farm_certification,
        } = fp;
        Self {
            version,
            id,
            name: String::from_utf8_lossy(&name).into(),
            cu,
            su,
            nu,
            ipv4,
            minimal_uptime,
            policy_created,
            policy_end,
            immutable,
            default,
            node_certification: node_certification.into(),
            farm_certification: farm_certification.into(),
        }
    }
}

impl From<pallet_tfgrid_legacy::types::FarmingPolicy> for FarmingPolicy<BlockNumber> {
    fn from(fp: pallet_tfgrid_legacy::types::FarmingPolicy) -> Self {
        let pallet_tfgrid_legacy::types::FarmingPolicy {
            version,
            id,
            name,
            cu,
            su,
            nu,
            ipv4,
            timestamp,
            certification_type,
        } = fp;
        Self {
            version,
            id,
            name: String::from_utf8_lossy(&name).into(),
            cu,
            su,
            nu,
            ipv4,
            minimal_uptime: 0,
            policy_created: 0,
            policy_end: 0,
            immutable: false,
            default: false,
            node_certification: NodeCertification::Diy,
            farm_certification: FarmCertification::NotCertified,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Policy {
    pub value: u32,
    pub unit: Unit,
}

impl From<pallet_tfgrid::types::Policy> for Policy {
    fn from(p: pallet_tfgrid::types::Policy) -> Self {
        let pallet_tfgrid::types::Policy { value, unit } = p;
        Self {
            value,
            unit: unit.into(),
        }
    }
}

impl From<pallet_tfgrid_legacy::types::Policy> for Policy {
    fn from(p: pallet_tfgrid_legacy::types::Policy) -> Self {
        let pallet_tfgrid_legacy::types::Policy { value, unit } = p;
        Self {
            value,
            unit: unit.into(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum Unit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terrabytes,
}

impl Default for Unit {
    fn default() -> Unit {
        Unit::Gigabytes
    }
}

impl From<pallet_tfgrid::types::Unit> for Unit {
    fn from(u: pallet_tfgrid::types::Unit) -> Self {
        match u {
            pallet_tfgrid::types::Unit::Bytes => Unit::Bytes,
            pallet_tfgrid::types::Unit::Kilobytes => Unit::Kilobytes,
            pallet_tfgrid::types::Unit::Megabytes => Unit::Megabytes,
            pallet_tfgrid::types::Unit::Gigabytes => Unit::Gigabytes,
            pallet_tfgrid::types::Unit::Terrabytes => Unit::Terrabytes,
        }
    }
}

impl From<pallet_tfgrid_legacy::types::Unit> for Unit {
    fn from(u: pallet_tfgrid_legacy::types::Unit) -> Self {
        match u {
            pallet_tfgrid_legacy::types::Unit::Bytes => Unit::Bytes,
            pallet_tfgrid_legacy::types::Unit::Kilobytes => Unit::Kilobytes,
            pallet_tfgrid_legacy::types::Unit::Megabytes => Unit::Megabytes,
            pallet_tfgrid_legacy::types::Unit::Gigabytes => Unit::Gigabytes,
            pallet_tfgrid_legacy::types::Unit::Terrabytes => Unit::Terrabytes,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Resources {
    pub hru: u64,
    pub sru: u64,
    pub cru: u64,
    pub mru: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct ContractResources {
    pub contract_id: u64,
    pub used: Resources,
}

impl From<tfchain_support::types::Resources> for Resources {
    fn from(r: tfchain_support::types::Resources) -> Self {
        let tfchain_support::types::Resources { hru, sru, cru, mru } = r;
        Self { hru, sru, cru, mru }
    }
}

impl From<pallet_tfgrid_legacy::types::Resources> for Resources {
    fn from(r: pallet_tfgrid_legacy::types::Resources) -> Self {
        let pallet_tfgrid_legacy::types::Resources { hru, sru, cru, mru } = r;
        Self { hru, sru, cru, mru }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct PricingPolicy {
    pub version: u32,
    pub id: u32,
    pub name: Vec<u8>,
    pub su: Policy,
    pub cu: Policy,
    pub nu: Policy,
    pub ipu: Policy,
    pub unique_name: Policy,
    pub domain_name: Policy,
    pub foundation_account: AccountId32,
    pub certified_sales_account: AccountId32,
    pub discount_for_dedication_nodes: u8,
}

impl From<pallet_tfgrid::types::PricingPolicy<AccountId32>> for PricingPolicy {
    fn from(pp: pallet_tfgrid::types::PricingPolicy<AccountId32>) -> Self {
        let pallet_tfgrid::types::PricingPolicy::<AccountId32> {
            version,
            id,
            name,
            su,
            cu,
            nu,
            ipu,
            unique_name,
            domain_name,
            foundation_account,
            certified_sales_account,
            discount_for_dedication_nodes,
        } = pp;
        Self {
            version,
            id,
            name,
            su: su.into(),
            cu: cu.into(),
            nu: nu.into(),
            ipu: ipu.into(),
            unique_name: unique_name.into(),
            domain_name: domain_name.into(),
            foundation_account,
            certified_sales_account,
            discount_for_dedication_nodes,
        }
    }
}

impl From<pallet_tfgrid_legacy::types::PricingPolicy<AccountId32>> for PricingPolicy {
    fn from(pp: pallet_tfgrid_legacy::types::PricingPolicy<AccountId32>) -> Self {
        let pallet_tfgrid_legacy::types::PricingPolicy::<AccountId32> {
            version,
            id,
            name,
            su,
            cu,
            nu,
            ipu,
            unique_name,
            domain_name,
            foundation_account,
            certified_sales_account,
            discount_for_dedication_nodes,
        } = pp;
        Self {
            version,
            id,
            name,
            su: su.into(),
            cu: cu.into(),
            nu: nu.into(),
            ipu: ipu.into(),
            unique_name: unique_name.into(),
            domain_name: domain_name.into(),
            foundation_account,
            certified_sales_account,
            discount_for_dedication_nodes,
        }
    }
}

/// A list of Grandpa authorities with associated weights.
pub type AuthorityList = Vec<(AuthorityId, AuthorityWeight)>;

/// The grandpa authority ID type.
pub type AuthorityId = ed25519::Public;

/// The weight of an authority.
pub type AuthorityWeight = u64;

#[derive(Debug)]
pub struct Balance(u64);

impl From<u64> for Balance {
    fn from(amount: u64) -> Self {
        Balance(amount)
    }
}

impl Balance {
    /// Get the balance as the amount of units expressed as a u64.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

pub type BlockNumber = u32;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Twin {
    pub version: u32,
    pub id: u32,
    pub account_id: AccountId32,
    pub ip: String,
    pub entities: Vec<EntityProof>,
}

impl From<pallet_tfgrid::types::Twin<AccountId32>> for Twin {
    fn from(t: pallet_tfgrid::types::Twin<AccountId32>) -> Self {
        let pallet_tfgrid::types::Twin::<AccountId32> {
            version,
            id,
            account_id,
            ip,
            entities,
        } = t;
        Self {
            version,
            id,
            account_id,
            ip: String::from_utf8_lossy(&ip).into(),
            entities: entities.into_iter().map(EntityProof::from).collect(),
        }
    }
}

impl From<pallet_tfgrid_legacy::types::Twin<AccountId32>> for Twin {
    fn from(t: pallet_tfgrid_legacy::types::Twin<AccountId32>) -> Self {
        let pallet_tfgrid_legacy::types::Twin::<AccountId32> {
            version,
            id,
            account_id,
            ip,
            entities,
        } = t;
        Self {
            version,
            id,
            account_id,
            ip: String::from_utf8_lossy(&ip).into(),
            entities: entities.into_iter().map(EntityProof::from).collect(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode, Default)]
pub struct Entity {
    pub version: u32,
    pub id: u32,
    pub name: Vec<u8>,
    pub account_id: AccountId32,
    pub country: Vec<u8>,
    pub city: Vec<u8>,
}

impl From<pallet_tfgrid::types::Entity<AccountId32>> for Entity {
    fn from(e: pallet_tfgrid::types::Entity<AccountId32>) -> Self {
        let pallet_tfgrid::types::Entity::<AccountId32> {
            version,
            id,
            name,
            account_id,
            country,
            city,
        } = e;
        Self {
            version,
            id,
            name,
            account_id,
            country,
            city,
        }
    }
}

impl From<pallet_tfgrid_legacy::types::Entity<AccountId32>> for Entity {
    fn from(e: pallet_tfgrid_legacy::types::Entity<AccountId32>) -> Self {
        let pallet_tfgrid_legacy::types::Entity::<AccountId32> {
            version,
            id,
            name,
            account_id,
            country,
            city,
        } = e;
        Self {
            version,
            id,
            name,
            account_id,
            country,
            city,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct EntityProof {
    pub entity_id: u32,
    pub signature: Vec<u8>,
}

impl From<pallet_tfgrid::types::EntityProof> for EntityProof {
    fn from(ep: pallet_tfgrid::types::EntityProof) -> Self {
        let pallet_tfgrid::types::EntityProof {
            entity_id,
            signature,
        } = ep;
        Self {
            entity_id,
            signature,
        }
    }
}

impl From<pallet_tfgrid_legacy::types::EntityProof> for EntityProof {
    fn from(ep: pallet_tfgrid_legacy::types::EntityProof) -> Self {
        let pallet_tfgrid_legacy::types::EntityProof {
            entity_id,
            signature,
        } = ep;
        Self {
            entity_id,
            signature,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Farm {
    pub version: u32,
    pub id: u32,
    pub name: String,
    pub twin_id: u32,
    pub pricing_policy_id: u32,
    pub certification: FarmCertification,
    pub public_ips: Vec<PublicIP>,
    pub dedicated_farm: bool,
    pub farming_policy_limits: Option<FarmingPolicyLimit>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum FarmCertification {
    NotCertified,
    Gold,
}

impl Default for FarmCertification {
    fn default() -> Self {
        FarmCertification::NotCertified
    }
}

impl From<tfchain_support::types::FarmCertification> for FarmCertification {
    fn from(fc: tfchain_support::types::FarmCertification) -> Self {
        match fc {
            tfchain_support::types::FarmCertification::NotCertified => {
                FarmCertification::NotCertified
            }
            tfchain_support::types::FarmCertification::Gold => FarmCertification::Gold,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct FarmingPolicyLimit {
    pub farming_policy_id: u32,
    pub cu: Option<u64>,
    pub su: Option<u64>,
    pub end: Option<u64>,
    pub node_count: Option<u32>,
    pub node_certification: bool,
}

impl From<tfchain_support::types::FarmingPolicyLimit> for FarmingPolicyLimit {
    fn from(fpl: tfchain_support::types::FarmingPolicyLimit) -> Self {
        let tfchain_support::types::FarmingPolicyLimit {
            farming_policy_id,
            cu,
            su,
            end,
            node_count,
            node_certification,
        } = fpl;
        Self {
            farming_policy_id,
            cu,
            su,
            end,
            node_count,
            node_certification,
        }
    }
}

impl From<tfchain_support::types::Farm> for Farm {
    fn from(f: tfchain_support::types::Farm) -> Self {
        let tfchain_support::types::Farm {
            version,
            id,
            name,
            twin_id,
            pricing_policy_id,
            public_ips,
            dedicated_farm,
            certification,
            farming_policy_limits,
        } = f;
        Self {
            version,
            id,
            name: String::from_utf8_lossy(&name).into(),
            twin_id,
            pricing_policy_id,
            public_ips: public_ips.into_iter().map(PublicIP::from).collect(),
            dedicated_farm,
            certification: certification.into(),
            farming_policy_limits: farming_policy_limits.map(FarmingPolicyLimit::from),
        }
    }
}

impl From<pallet_tfgrid_legacy::types::Farm> for Farm {
    fn from(f: pallet_tfgrid_legacy::types::Farm) -> Self {
        let pallet_tfgrid_legacy::types::Farm {
            version,
            id,
            name,
            twin_id,
            pricing_policy_id,
            certification_type,
            public_ips,
            dedicated_farm,
        } = f;
        Self {
            version,
            id,
            name: String::from_utf8_lossy(&name).into(),
            twin_id,
            pricing_policy_id,
            certification: match certification_type {
                pallet_tfgrid_legacy::types::CertificationType::Diy => {
                    FarmCertification::NotCertified
                }
                pallet_tfgrid_legacy::types::CertificationType::Certified => {
                    FarmCertification::Gold
                }
            },
            public_ips: public_ips.into_iter().map(PublicIP::from).collect(),
            dedicated_farm,
            farming_policy_limits: None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct PublicIP {
    pub ip: String,
    pub gateway: String,
    pub contract_id: u64,
}

impl From<tfchain_support::types::PublicIP> for PublicIP {
    fn from(pip: tfchain_support::types::PublicIP) -> Self {
        let tfchain_support::types::PublicIP {
            ip,
            gateway,
            contract_id,
        } = pip;
        Self {
            ip: String::from_utf8_lossy(&ip).into(),
            gateway: String::from_utf8_lossy(&gateway).into(),
            contract_id,
        }
    }
}

impl From<pallet_tfgrid_legacy::types::PublicIP> for PublicIP {
    fn from(pip: pallet_tfgrid_legacy::types::PublicIP) -> Self {
        let pallet_tfgrid_legacy::types::PublicIP {
            ip,
            gateway,
            contract_id,
        } = pip;
        Self {
            ip: String::from_utf8_lossy(&ip).into(),
            gateway: String::from_utf8_lossy(&gateway).into(),
            contract_id,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Node {
    pub version: u32,
    pub id: u32,
    pub farm_id: u32,
    pub twin_id: u32,
    pub resources: Resources,
    pub location: Location,
    pub country: String,
    pub city: String,
    // optional public config
    pub public_config: Option<PublicConfig>,
    pub created: u64,
    pub farming_policy_id: u32,
    pub interfaces: Vec<Interface>,
    pub certification: NodeCertification,
    pub secure_boot: bool,
    pub virtualized: bool,
    pub serial_number: String,
    pub connection_price: u32,
}

impl From<tfchain_support::types::Node> for Node {
    fn from(n: tfchain_support::types::Node) -> Self {
        let tfchain_support::types::Node {
            version,
            id,
            farm_id,
            twin_id,
            resources,
            location,
            country,
            city,
            public_config,
            created,
            farming_policy_id,
            interfaces,
            certification,
            secure_boot,
            virtualized,
            serial_number,
            connection_price,
        } = n;
        Self {
            version,
            id,
            farm_id,
            twin_id,
            resources: resources.into(),
            location: location.into(),
            country: String::from_utf8_lossy(&country).into(),
            city: String::from_utf8_lossy(&city).into(),
            public_config: public_config.map(PublicConfig::from),
            created,
            farming_policy_id,
            interfaces: interfaces.into_iter().map(Interface::from).collect(),
            certification: certification.into(),
            secure_boot,
            virtualized,
            serial_number: String::from_utf8_lossy(&serial_number).into(),
            connection_price,
        }
    }
}

impl From<pallet_tfgrid_legacy::types::Node> for Node {
    fn from(n: pallet_tfgrid_legacy::types::Node) -> Self {
        let pallet_tfgrid_legacy::types::Node {
            version,
            id,
            farm_id,
            twin_id,
            resources,
            location,
            country,
            city,
            public_config,
            created,
            farming_policy_id,
            interfaces,
            certification_type,
            secure_boot,
            virtualized,
            serial_number,
        } = n;
        Self {
            version,
            id,
            farm_id,
            twin_id,
            resources: resources.into(),
            location: location.into(),
            country: String::from_utf8_lossy(&country).into(),
            city: String::from_utf8_lossy(&city).into(),
            public_config: public_config.map(PublicConfig::from),
            created,
            farming_policy_id,
            interfaces: interfaces.into_iter().map(Interface::from).collect(),
            certification: certification_type.into(),
            secure_boot,
            virtualized,
            serial_number: String::from_utf8_lossy(&serial_number).to_string(),
            connection_price: 0,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Location {
    pub longitude: String,
    pub latitude: String,
}

impl From<tfchain_support::types::Location> for Location {
    fn from(l: tfchain_support::types::Location) -> Self {
        let tfchain_support::types::Location {
            longitude,
            latitude,
        } = l;
        Self {
            longitude: String::from_utf8_lossy(&longitude).into(),
            latitude: String::from_utf8_lossy(&latitude).into(),
        }
    }
}

impl From<pallet_tfgrid_legacy::types::Location> for Location {
    fn from(l: pallet_tfgrid_legacy::types::Location) -> Self {
        let pallet_tfgrid_legacy::types::Location {
            longitude,
            latitude,
        } = l;
        Self {
            longitude: String::from_utf8_lossy(&longitude).into(),
            latitude: String::from_utf8_lossy(&latitude).into(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Interface {
    pub name: String,
    pub mac: String,
    pub ips: Vec<IP>,
}

impl From<tfchain_support::types::Interface> for Interface {
    fn from(iface: tfchain_support::types::Interface) -> Self {
        let tfchain_support::types::Interface { name, mac, ips } = iface;
        Self {
            name: String::from_utf8_lossy(&name).into(),
            mac: String::from_utf8_lossy(&mac).into(),
            ips: ips
                .into_iter()
                .map(|ip| String::from_utf8_lossy(&ip).into())
                .collect(),
        }
    }
}

impl From<pallet_tfgrid_legacy::types::Interface> for Interface {
    fn from(iface: pallet_tfgrid_legacy::types::Interface) -> Self {
        let pallet_tfgrid_legacy::types::Interface { name, mac, ips } = iface;
        Self {
            name: String::from_utf8_lossy(&name).into(),
            mac: String::from_utf8_lossy(&mac).into(),
            ips: ips
                .into_iter()
                .map(|ip| String::from_utf8_lossy(&ip).into())
                .collect(),
        }
    }
}

pub type IP = String;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct PublicConfig {
    pub ipv4: String,
    pub ipv6: String,
    pub gw4: String,
    pub gw6: String,
    pub domain: String,
}

impl From<tfchain_support::types::PublicConfig> for PublicConfig {
    fn from(pc: tfchain_support::types::PublicConfig) -> Self {
        let tfchain_support::types::PublicConfig {
            ipv4,
            ipv6,
            gw4,
            gw6,
            domain,
        } = pc;
        Self {
            ipv4: String::from_utf8_lossy(&ipv4).into(),
            ipv6: String::from_utf8_lossy(&ipv6).into(),
            gw4: String::from_utf8_lossy(&gw4).into(),
            gw6: String::from_utf8_lossy(&gw6).into(),
            domain: String::from_utf8_lossy(&domain).into(),
        }
    }
}

impl From<pallet_tfgrid_legacy::types::PublicConfig> for PublicConfig {
    fn from(pc: pallet_tfgrid_legacy::types::PublicConfig) -> Self {
        let pallet_tfgrid_legacy::types::PublicConfig {
            ipv4,
            ipv6,
            gw4,
            gw6,
            domain,
        } = pc;
        Self {
            ipv4: String::from_utf8_lossy(&ipv4).into(),
            ipv6: String::from_utf8_lossy(&ipv6).into(),
            gw4: String::from_utf8_lossy(&gw4).into(),
            gw6: String::from_utf8_lossy(&gw6).into(),
            domain: String::from_utf8_lossy(&domain).into(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum DiscountLevel {
    None,
    Default,
    Bronze,
    Silver,
    Gold,
}

impl Default for DiscountLevel {
    fn default() -> DiscountLevel {
        DiscountLevel::None
    }
}

// TODO: do we need this, and if so, provide f64 impl or even better integer impl
//impl DiscountLevel {
//    pub fn price_multiplier(&self) -> U64F64 {
//        match self {
//            DiscountLevel::None => U64F64::from_num(1),
//            DiscountLevel::Default => U64F64::from_num(0.8),
//            DiscountLevel::Bronze => U64F64::from_num(0.7),
//            DiscountLevel::Silver => U64F64::from_num(0.6),
//            DiscountLevel::Gold => U64F64::from_num(0.4),
//        }
//    }
//}

impl From<pallet_smart_contract::types::DiscountLevel> for DiscountLevel {
    fn from(dl: pallet_smart_contract::types::DiscountLevel) -> Self {
        match dl {
            pallet_smart_contract::types::DiscountLevel::None => DiscountLevel::None,
            pallet_smart_contract::types::DiscountLevel::Default => DiscountLevel::Default,
            pallet_smart_contract::types::DiscountLevel::Bronze => DiscountLevel::Bronze,
            pallet_smart_contract::types::DiscountLevel::Silver => DiscountLevel::Silver,
            pallet_smart_contract::types::DiscountLevel::Gold => DiscountLevel::Gold,
        }
    }
}

impl From<pallet_smart_contract_legacy::types::DiscountLevel> for DiscountLevel {
    fn from(dl: pallet_smart_contract_legacy::types::DiscountLevel) -> Self {
        match dl {
            pallet_smart_contract_legacy::types::DiscountLevel::None => DiscountLevel::None,
            pallet_smart_contract_legacy::types::DiscountLevel::Default => DiscountLevel::Default,
            pallet_smart_contract_legacy::types::DiscountLevel::Bronze => DiscountLevel::Bronze,
            pallet_smart_contract_legacy::types::DiscountLevel::Silver => DiscountLevel::Silver,
            pallet_smart_contract_legacy::types::DiscountLevel::Gold => DiscountLevel::Gold,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Consumption {
    pub contract_id: u64,
    pub timestamp: u64,
    pub cru: u64,
    pub sru: u64,
    pub hru: u64,
    pub mru: u64,
    pub nru: u64,
}

impl From<pallet_smart_contract::types::Consumption> for Consumption {
    fn from(c: pallet_smart_contract::types::Consumption) -> Self {
        let pallet_smart_contract::types::Consumption {
            contract_id,
            timestamp,
            cru,
            sru,
            hru,
            mru,
            nru,
        } = c;
        Self {
            contract_id,
            timestamp,
            cru,
            sru,
            hru,
            mru,
            nru,
        }
    }
}

impl From<pallet_smart_contract_legacy::types::Consumption> for Consumption {
    fn from(c: pallet_smart_contract_legacy::types::Consumption) -> Self {
        let pallet_smart_contract_legacy::types::Consumption {
            contract_id,
            timestamp,
            cru,
            sru,
            hru,
            mru,
            nru,
        } = c;
        Self {
            contract_id,
            timestamp,
            cru,
            sru,
            hru,
            mru,
            nru,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct ContractBill {
    pub contract_id: u64,
    pub timestamp: u64,
    pub discount_level: DiscountLevel,
    pub amount_billed: u128,
}

impl From<pallet_smart_contract::types::ContractBill> for ContractBill {
    fn from(cb: pallet_smart_contract::types::ContractBill) -> Self {
        let pallet_smart_contract::types::ContractBill {
            contract_id,
            timestamp,
            discount_level,
            amount_billed,
        } = cb;
        Self {
            contract_id,
            timestamp,
            discount_level: discount_level.into(),
            amount_billed,
        }
    }
}

impl From<pallet_smart_contract_legacy::types::ContractBill> for ContractBill {
    fn from(cb: pallet_smart_contract_legacy::types::ContractBill) -> Self {
        let pallet_smart_contract_legacy::types::ContractBill {
            contract_id,
            timestamp,
            discount_level,
            amount_billed,
        } = cb;
        Self {
            contract_id,
            timestamp,
            discount_level: discount_level.into(),
            amount_billed,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct Contract {
    pub version: u32,
    pub state: ContractState,
    pub contract_id: u64,
    pub twin_id: u32,
    pub contract_type: ContractData,
}

impl From<pallet_smart_contract::types::Contract> for Contract {
    fn from(c: pallet_smart_contract::types::Contract) -> Self {
        let pallet_smart_contract::types::Contract {
            version,
            state,
            contract_id,
            twin_id,
            contract_type,
        } = c;
        Self {
            version,
            state: state.into(),
            contract_id,
            twin_id,
            contract_type: contract_type.into(),
        }
    }
}

impl From<pallet_smart_contract_legacy::types::Contract> for Contract {
    fn from(c: pallet_smart_contract_legacy::types::Contract) -> Self {
        let pallet_smart_contract_legacy::types::Contract {
            version,
            state,
            contract_id,
            twin_id,
            contract_type,
        } = c;
        Self {
            version,
            state: state.into(),
            contract_id,
            twin_id,
            contract_type: contract_type.into(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct NodeContract {
    pub node_id: u32,
    // deployment_data is the encrypted deployment body. This encrypted the deployment with the **USER** public key.
    // So only the user can read this data later on (or any other key that he keeps safe).
    // this data part is read only by the user and can actually hold any information to help him reconstruct his deployment or can be left empty.
    pub deployment_data: Vec<u8>,
    // Hash of the deployment, set by the user
    pub deployment_hash: Vec<u8>,
    pub public_ips: u32,
    pub public_ips_list: Vec<PublicIP>,
}

impl From<pallet_smart_contract::types::NodeContract> for NodeContract {
    fn from(nc: pallet_smart_contract::types::NodeContract) -> Self {
        let pallet_smart_contract::types::NodeContract {
            node_id,
            deployment_data,
            deployment_hash,
            public_ips,
            public_ips_list,
        } = nc;
        Self {
            node_id,
            deployment_data,
            deployment_hash,
            public_ips,
            public_ips_list: public_ips_list.into_iter().map(PublicIP::from).collect(),
        }
    }
}

impl From<pallet_smart_contract_legacy::types::NodeContract> for NodeContract {
    fn from(nc: pallet_smart_contract_legacy::types::NodeContract) -> Self {
        let pallet_smart_contract_legacy::types::NodeContract {
            node_id,
            deployment_data,
            deployment_hash,
            public_ips,
            public_ips_list,
        } = nc;
        Self {
            node_id,
            deployment_data,
            deployment_hash,
            public_ips,
            public_ips_list: public_ips_list.into_iter().map(PublicIP::from).collect(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct NameContract {
    pub name: String,
}

impl From<pallet_smart_contract::types::NameContract> for NameContract {
    fn from(nc: pallet_smart_contract::types::NameContract) -> Self {
        let pallet_smart_contract::types::NameContract { name } = nc;
        Self {
            name: String::from_utf8_lossy(&name).into(),
        }
    }
}

impl From<pallet_smart_contract_legacy::types::NameContract> for NameContract {
    fn from(nc: pallet_smart_contract_legacy::types::NameContract) -> Self {
        let pallet_smart_contract_legacy::types::NameContract { name } = nc;
        Self {
            name: String::from_utf8_lossy(&name).into(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct RentContract {
    pub node_id: u32,
}

impl From<pallet_smart_contract::types::RentContract> for RentContract {
    fn from(nc: pallet_smart_contract::types::RentContract) -> Self {
        let pallet_smart_contract::types::RentContract { node_id } = nc;
        Self { node_id }
    }
}

impl From<pallet_smart_contract_legacy::types::RentContract> for RentContract {
    fn from(nc: pallet_smart_contract_legacy::types::RentContract) -> Self {
        let pallet_smart_contract_legacy::types::RentContract { node_id } = nc;
        Self { node_id }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum ContractData {
    NodeContract(NodeContract),
    NameContract(NameContract),
    RentContract(RentContract),
}

impl Default for ContractData {
    fn default() -> ContractData {
        ContractData::NodeContract(NodeContract::default())
    }
}

impl From<pallet_smart_contract::types::ContractData> for ContractData {
    fn from(cd: pallet_smart_contract::types::ContractData) -> Self {
        match cd {
            pallet_smart_contract::types::ContractData::NodeContract(node_contract) => {
                ContractData::NodeContract(node_contract.into())
            }
            pallet_smart_contract::types::ContractData::NameContract(name_contract) => {
                ContractData::NameContract(name_contract.into())
            }
            pallet_smart_contract::types::ContractData::RentContract(rent_contract) => {
                ContractData::RentContract(rent_contract.into())
            }
        }
    }
}

impl From<pallet_smart_contract_legacy::types::ContractData> for ContractData {
    fn from(cd: pallet_smart_contract_legacy::types::ContractData) -> Self {
        match cd {
            pallet_smart_contract_legacy::types::ContractData::NodeContract(node_contract) => {
                ContractData::NodeContract(node_contract.into())
            }
            pallet_smart_contract_legacy::types::ContractData::NameContract(name_contract) => {
                ContractData::NameContract(name_contract.into())
            }
            pallet_smart_contract_legacy::types::ContractData::RentContract(rent_contract) => {
                ContractData::RentContract(rent_contract.into())
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum ContractState {
    Created,
    Deleted(Cause),
    GracePeriod(BlockNumber),
}

impl From<pallet_smart_contract::types::ContractState> for ContractState {
    fn from(cs: pallet_smart_contract::types::ContractState) -> Self {
        match cs {
            pallet_smart_contract::types::ContractState::Created => ContractState::Created,
            pallet_smart_contract::types::ContractState::Deleted(cause) => {
                ContractState::Deleted(cause.into())
            }
            pallet_smart_contract::types::ContractState::GracePeriod(bn) => {
                ContractState::GracePeriod(bn as BlockNumber)
            }
        }
    }
}

impl From<pallet_smart_contract_legacy::types::ContractState> for ContractState {
    fn from(cs: pallet_smart_contract_legacy::types::ContractState) -> Self {
        match cs {
            pallet_smart_contract_legacy::types::ContractState::Created => ContractState::Created,
            pallet_smart_contract_legacy::types::ContractState::Deleted(cause) => {
                ContractState::Deleted(cause.into())
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum Cause {
    CanceledByUser,
    OutOfFunds,
}

impl From<pallet_smart_contract::types::Cause> for Cause {
    fn from(c: pallet_smart_contract::types::Cause) -> Self {
        match c {
            pallet_smart_contract::types::Cause::CanceledByUser => Cause::CanceledByUser,
            pallet_smart_contract::types::Cause::OutOfFunds => Cause::OutOfFunds,
        }
    }
}

impl From<pallet_smart_contract_legacy::types::Cause> for Cause {
    fn from(c: pallet_smart_contract_legacy::types::Cause) -> Self {
        match c {
            pallet_smart_contract_legacy::types::Cause::CanceledByUser => Cause::CanceledByUser,
            pallet_smart_contract_legacy::types::Cause::OutOfFunds => Cause::OutOfFunds,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct CertificationCodes {
    pub version: u32,
    pub id: u32,
    pub name: Vec<u8>,
    pub description: Vec<u8>,
    pub certification_code_type: CertificationCodeType,
}

// impl From<tfchain_support::types::CertificationCodes> for CertificationCodes {
//     fn from(cc: tfchain_support::types::CertificationCodes) -> Self {
//         let tfchain_support::types::CertificationCodes {
//             version,
//             id,
//             name,
//             description,
//             certification_code_type,
//         } = cc;
//         Self {
//             version,
//             id,
//             name,
//             description,
//             certification_code_type: certification_code_type.into(),
//         }
//     }
// }

impl From<pallet_tfgrid_legacy::types::CertificationCodes> for CertificationCodes {
    fn from(cc: pallet_tfgrid_legacy::types::CertificationCodes) -> Self {
        let pallet_tfgrid_legacy::types::CertificationCodes {
            version,
            id,
            name,
            description,
            certification_code_type,
        } = cc;
        Self {
            version,
            id,
            name,
            description,
            certification_code_type: certification_code_type.into(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Debug)]
pub enum CertificationCodeType {
    Farm,
    Entity,
}

impl Default for CertificationCodeType {
    fn default() -> Self {
        CertificationCodeType::Farm
    }
}

// impl From<pallet_tfgrid::types::CertificationCodeType> for CertificationCodeType {
//     fn from(cct: pallet_tfgrid::types::CertificationCodeType) -> Self {
//         match cct {
//             pallet_tfgrid::types::CertificationCodeType::Farm => CertificationCodeType::Farm,
//             pallet_tfgrid::types::CertificationCodeType::Entity => CertificationCodeType::Entity,
//         }
//     }
// }

impl From<pallet_tfgrid_legacy::types::CertificationCodeType> for CertificationCodeType {
    fn from(cct: pallet_tfgrid_legacy::types::CertificationCodeType) -> Self {
        match cct {
            pallet_tfgrid_legacy::types::CertificationCodeType::Farm => CertificationCodeType::Farm,
            pallet_tfgrid_legacy::types::CertificationCodeType::Entity => {
                CertificationCodeType::Entity
            }
        }
    }
}

/// MintTransaction contains all the information about
/// Stellar -> TF Chain minting transaction.
/// if the votes field is larger then (number of validators / 2) + 1 , the transaction will be minted
#[derive(Debug)]
pub struct MintTransaction {
    pub amount: Balance,
    pub target: AccountId32,
    pub block: BlockNumber,
    pub votes: u32,
}

impl From<pallet_tft_bridge::MintTransaction<AccountId32, u32>> for MintTransaction {
    fn from(mtx: pallet_tft_bridge::MintTransaction<AccountId32, u32>) -> Self {
        let pallet_tft_bridge::MintTransaction {
            amount,
            target,
            block,
            votes,
        } = mtx;
        Self {
            amount: amount.into(),
            target,
            block,
            votes,
        }
    }
}

/// BurnTransaction contains all the information about
/// TF Chain -> Stellar burn transaction
/// Transaction is ready when (number of validators / 2) + 1 signatures are present
#[derive(Debug)]
pub struct BurnTransaction {
    pub block: BlockNumber,
    pub amount: Balance,
    pub target: Vec<u8>,
    pub signatures: Vec<StellarSignature>,
    pub sequence_number: u64,
}

impl From<pallet_tft_bridge::BurnTransaction<u32>> for BurnTransaction {
    fn from(btx: pallet_tft_bridge::BurnTransaction<u32>) -> Self {
        let pallet_tft_bridge::BurnTransaction {
            block,
            amount,
            target,
            signatures,
            sequence_number,
        } = btx;
        Self {
            block,
            amount: amount.into(),
            target,
            signatures: signatures.into_iter().map(StellarSignature::from).collect(),
            sequence_number,
        }
    }
}

#[derive(Debug)]
pub struct RefundTransaction {
    pub block: BlockNumber,
    pub amount: Balance,
    pub target: Vec<u8>,
    pub tx_hash: Vec<u8>,
    pub signatures: Vec<StellarSignature>,
    pub sequence_number: u64,
}

impl From<pallet_tft_bridge::RefundTransaction<u32>> for RefundTransaction {
    fn from(rtx: pallet_tft_bridge::RefundTransaction<u32>) -> Self {
        let pallet_tft_bridge::RefundTransaction {
            block,
            amount,
            target,
            tx_hash,
            signatures,
            sequence_number,
        } = rtx;
        Self {
            block,
            amount: amount.into(),
            target,
            tx_hash,
            signatures: signatures.into_iter().map(StellarSignature::from).collect(),
            sequence_number,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, Debug)]
pub struct StellarSignature {
    pub signature: Vec<u8>,
    pub stellar_pub_key: Vec<u8>,
}

impl From<pallet_tft_bridge::StellarSignature> for StellarSignature {
    fn from(ss: pallet_tft_bridge::StellarSignature) -> Self {
        let pallet_tft_bridge::StellarSignature {
            signature,
            stellar_pub_key,
        } = ss;
        Self {
            signature,
            stellar_pub_key,
        }
    }
}

impl Default for ContractState {
    fn default() -> ContractState {
        ContractState::Created
    }
}

impl Display for Twin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Twin details for twin {}", self.id)?;
        writeln!(f, "Account ID {}", self.account_id)?;
        writeln!(f, "IP: {}", self.ip)
    }
}

impl Display for ContractData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractData::NameContract(name_contract) => {
                writeln!(f, "Name: {}", name_contract.name)
            }
            ContractData::NodeContract(node_contract) => {
                writeln!(f, "Node id: {}", node_contract.node_id)?;
                writeln!(
                    f,
                    "Deployment data: {}",
                    String::from_utf8_lossy(&node_contract.deployment_data)
                )?;
                writeln!(
                    f,
                    "Deployment hash: {}",
                    String::from_utf8_lossy(&node_contract.deployment_hash)
                )?;
                for ip in &node_contract.public_ips_list {
                    writeln!(f, "IP: {}", ip.ip)?;
                    writeln!(f, "Gateway: {}", ip.gateway)?;
                }
                write!(f, "Number of public ips: {}", node_contract.public_ips)
            }
            ContractData::RentContract(rent_contract) => {
                writeln!(f, "Rented node id: {}", rent_contract.node_id)
            }
        }
    }
}

impl Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Contract details for contract {}", self.contract_id)?;
        writeln!(f, "State: {}", self.state)?;
        writeln!(f, "Twin id: {}", self.twin_id)?;
        writeln!(f, "{}", self.contract_type)
    }
}

impl Display for ContractState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractState::Created => {
                write!(f, "Created")
            }
            ContractState::Deleted(Cause::CanceledByUser) => {
                write!(f, "Canceled by user")
            }
            ContractState::Deleted(Cause::OutOfFunds) => {
                write!(f, "Out of funds")
            }
            ContractState::GracePeriod(bn) => {
                write!(f, "In grace period until block {}", bn)
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Node details for node {}", self.id)?;
        writeln!(f, "Farm ID: {}", self.farm_id)?;
        writeln!(f, "Twin ID: {}", self.twin_id)?;
        writeln!(f, "Node resources:")?;
        writeln!(f, "\tCRU: {} (logical cores)", self.resources.cru)?;
        writeln!(
            f,
            "\tMRU: {} ({:.02} GB | {:.02} GiB)",
            self.resources.mru,
            self.resources.mru as f64 / 1_000_000_000f64,
            self.resources.mru as f64 / (1 << 30) as f64
        )?;
        writeln!(
            f,
            "\tSRU: {} ({:.02} TB | {:.02} TiB)",
            self.resources.sru,
            self.resources.sru as f64 / 1_000_000_000_000f64,
            self.resources.sru as f64 / (1u64 << 40) as f64
        )?;
        writeln!(
            f,
            "\tHRU: {} ({:.02} TB | {:.02} TiB)",
            self.resources.hru,
            self.resources.hru as f64 / 1_000_000_000_000f64,
            self.resources.hru as f64 / (1u64 << 40) as f64
        )?;
        writeln!(
            f,
            "Location: {}, {} ({} lat {} long)",
            self.city, self.country, self.location.latitude, self.location.longitude
        )?;
        writeln!(
            f,
            "Created at: {}",
            Utc.timestamp(self.created as i64, 0)
                .with_timezone(&Local)
                .to_rfc2822()
        )?;
        writeln!(f, "Farming policy: {}", self.farming_policy_id)?;
        if self.interfaces.is_empty() {
            writeln!(f, "No known interfaces")?;
        } else {
            // Add some nice formatting, make sure all MAC address and IP addresses start in the
            // same column.
            // unwrap here is safe as None is only returned in case of an empty iterator, and we
            // specifically check for that case.
            let iface_name_space = self
                .interfaces
                .iter()
                .map(|iface| iface.name.len())
                .max()
                .unwrap();
            writeln!(f, "Interfaces:")?;
            for iface in &self.interfaces {
                writeln!(
                    f,
                    "\t{}: {:>width$}",
                    iface.name,
                    iface.mac,
                    width = iface_name_space - iface.name.len() + iface.mac.len()
                )?;
                for ip in &iface.ips {
                    writeln!(
                        f,
                        "\t{:>width$}",
                        ip,
                        width = ip.len() + iface_name_space + 2
                    )?;
                }
            }
        }
        if let Some(ref pub_config) = self.public_config {
            f.pad("Public config:\n")?;
            f.pad(&format!(
                "\tIPv4: {} (gw: {})\n",
                pub_config.ipv4, pub_config.gw4
            ))?;
            f.pad(&format!(
                "\tIPv6: {} (gw: {})\n",
                pub_config.ipv6, pub_config.gw6
            ))?;
            f.pad(&format!("\tDomain: {}\n", pub_config.domain))?;
        }
        f.pad(&format!(
            "Certification type {}\n",
            match self.certification {
                NodeCertification::Diy => "DIY",
                NodeCertification::Certified => "Certified",
            }
        ))?;
        f.pad(&format!("Secure boot enabled: {}\n", self.secure_boot))?;
        f.pad(&format!("Virtualized: {}\n", self.virtualized))?;
        f.pad(&format!("MOBO serial number: {}\n", self.serial_number))
    }
}

impl Display for Farm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Farm details for farm {} (ID: {})", self.name, self.id)?;
        writeln!(f, "Twin ID: {}", self.twin_id)?;
        writeln!(
            f,
            "Certification type {}",
            match self.certification {
                FarmCertification::NotCertified => "Not Certified",
                FarmCertification::Gold => "Gold",
            }
        )?;
        if !self.public_ips.is_empty() {
            writeln!(f, "Public IPs:")?;
            for ip in &self.public_ips {
                writeln!(f, "\tIPv4: {} (gw: {})", ip.ip, ip.gateway)?;
                writeln!(f, "\tContract id: {}", ip.contract_id)?;
            }
        }
        writeln!(f, "version: {}", self.version)
    }
}
