use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct Ec2Pricing {
    pub location_type: LocationType,
    pub region: Region,
    pub os: Os,
    pub tenancy: Tenancy,
    pub offering_class: OfferingClass,
    pub instance_type_pricing: Vec<InstanceTypePricing>,
}

#[derive(Deserialize, Debug)]
pub enum LocationType {
    Aws,
}

#[derive(Deserialize, Debug)]
pub enum Region {
    Tokyo,
    NVirginia,
}

#[derive(Deserialize, Debug)]
pub enum Os {
    Linux,
    Windows,
}

#[derive(Deserialize, Debug)]
pub enum Tenancy {
    Shared,
    Dedicated,
}

#[derive(Deserialize, Debug)]
pub enum OfferingClass {
    Standard,
    Convertible,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstanceTypePricing {
    pub instance_type: InstanceType,
    pub instance_pricing: InstancePricing,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstancePricing {
    pub on_demand: OnDemandPricing,
    pub reserved: Vec<ReservedPricing>,
}

impl InstancePricing {
    pub fn get(&self) -> String {
        let mut s = self.on_demand.get();
        for x in &self.reserved {
            let r = x.get();
            s.push_str(&r);
        }
        s
    }

    pub fn json(&self, count: u32) -> String {
        let mut s = String::from("{ \"on_demand\":");
        s.push_str(&self.on_demand.json(count));
        s.push_str(", \"reserved\": [");
        // s.push_str(format!(", \"reserved\": [").as_str());

        for x in &self.reserved {
            let r = x.json(count);
            s.push_str(&r);
            s.push(',');
        }
        s.pop();
        s.push_str("]}");
        s
    }
}

pub trait CostPerDuration {
    fn per_day(&self) -> Decimal;
    fn per_year(&self) -> Decimal;
    fn per_week(&self) -> Decimal {
        (self.per_day() * dec!(7.0)).round_dp(2)
    }
    fn per_month(&self) -> Decimal {
        (self.per_year() / dec!(12.0)).round_dp(2)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OnDemandPricing {
    pub hourly: Decimal,
}

impl OnDemandPricing {
    pub fn get(&self) -> String {
        format!("type: on_demand, hourly: {}\n", self.hourly)
    }
    pub fn json(&self, count: u32) -> String {
        let quantity = Decimal::from_u32(count).unwrap();
        let j = json!([{
            "quantity": quantity,
            "currency": "USD",
            "hourly": self.hourly * quantity,
            "per_day": self.per_day() * quantity,
            "per_week": self.per_week() * quantity,
            "per_month": self.per_month() * quantity,
            "per_year": self.per_year() * quantity
        },{
            "quantity": quantity,
            "currency": "JPY",
            "hourly": self.hourly.to_yen() * quantity,
            "per_day": self.per_day().to_yen() * quantity,
            "per_week": self.per_week().to_yen() * quantity,
            "per_month": self.per_month().to_yen() * quantity,
            "per_year": self.per_year().to_yen() * quantity
        }]);
        j.to_string()
    }
}

#[allow(dead_code)]
impl CostPerDuration for OnDemandPricing {
    fn per_day(&self) -> Decimal {
        (self.hourly * dec!(24.0)).round_dp(2)
    }
    fn per_year(&self) -> Decimal {
        (self.per_day() * dec!(365.0)).round_dp(2)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReservedPricing {
    pub term: Term,
    pub upfront: Upfront,
    pub initial: Decimal,
    pub monthly: Decimal,
}

impl ReservedPricing {
    pub fn get(&self) -> String {
        format!("type: reserved, term: {:>8}, upfront: {:>10}, initial: {:>8}, monthly: {:>8}, per_day: {:>8}\n", self.term, self.upfront, self.initial, self.monthly, self.per_day())
    }
    pub fn json(&self, count: u32) -> String {
        let quantity = Decimal::from_u32(count).unwrap();
        let j = json!([{
            "quantity": quantity,
            "currency": "USD",
            "term": self.term,
            "upfront": self.upfront,
            "initial": self.initial * quantity,
            "monthly": self.monthly * quantity,
            "per_day": self.per_day() * quantity,
            "per_week": self.per_week() * quantity,
            "per_month": self.per_month() * quantity,
            "per_year": self.per_year() * quantity
        }, {
            "quantity": quantity,
            "currency": "JPY",
            "term": self.term,
            "upfront": self.upfront,
            "initial": self.initial.to_yen() * quantity,
            "monthly": self.monthly.to_yen() * quantity,
            "per_day": self.per_day().to_yen() * quantity,
            "per_week": self.per_week().to_yen() * quantity,
            "per_month": self.per_month().to_yen() * quantity,
            "per_year": self.per_year().to_yen() * quantity
        }]);
        j.to_string()
    }
}

#[allow(dead_code)]
impl CostPerDuration for ReservedPricing {
    fn per_day(&self) -> Decimal {
        match &self.term {
            Term::OneYear => match &self.upfront {
                Upfront::No => (self.monthly * dec!(12.0) / dec!(365.0)).round_dp(2),
                Upfront::Partial => {
                    ((self.initial + (self.monthly * dec!(12.0))) / dec!(365.0)).round_dp(2)
                }
                Upfront::All => (self.initial / dec!(365.0)).round_dp(2),
            },
            Term::ThreeYears => match &self.upfront {
                Upfront::No => (self.monthly * (dec!(12.0) * dec!(3.0))
                    / (dec!(365.0) * dec!(3.0)))
                .round_dp(2),
                Upfront::Partial => ((self.initial + (self.monthly * dec!(12.0) * dec!(3.0)))
                    / (dec!(365.0) * dec!(3.0)))
                .round_dp(2),
                Upfront::All => (self.initial / (dec!(365.0) * dec!(3.0))).round_dp(2),
            },
        }
    }
    fn per_year(&self) -> Decimal {
        (self.per_day() * dec!(365.0)).round_dp(2)
    }
}

pub trait UsdToJpy {
    fn to_yen(&self) -> Self;
}

const DOLLAR_TO_YEN: Decimal = dec!(136.0);
impl UsdToJpy for Decimal {
    fn to_yen(&self) -> Self {
        *self * DOLLAR_TO_YEN
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Copy, Clone)]
pub enum InstanceType {
    T4gSmall,
    T4gLarge,
    M6gMedium,
    M6gLarge,
    C5Large,
    C5Xlarge,
    C6iXlarge,
    Na,
}

impl FromStr for InstanceType {
    type Err = ();
    fn from_str(instance_type: &str) -> Result<InstanceType, Self::Err> {
        match instance_type {
            "t4g.small" => Ok(InstanceType::T4gSmall),
            "t4g.large" => Ok(InstanceType::T4gLarge),
            "m6g.medium" => Ok(InstanceType::M6gMedium),
            "m6g.large" => Ok(InstanceType::M6gLarge),
            "c5.large" => Ok(InstanceType::C5Large),
            "c5.xlarge" => Ok(InstanceType::C5Xlarge),
            "c6i.xlarge" => Ok(InstanceType::C6iXlarge),
            _ => Ok(InstanceType::Na),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Term {
    OneYear,
    ThreeYears,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Term::OneYear => String::from(" 1year"),
            Term::ThreeYears => String::from("3years"),
        };
        write!(f, "{}", printable)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Upfront {
    No,
    Partial,
    All,
}

impl fmt::Display for Upfront {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Upfront::No => String::from("     no"),
            Upfront::Partial => String::from("partial"),
            Upfront::All => String::from("    all"),
        };
        write!(f, "{}", printable)
    }
}

pub fn get_configuration() -> Result<Ec2Pricing, config::ConfigError> {
    let ec2pricing = config::Config::builder()
        .add_source(config::File::new(
            "ec2pricing.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    ec2pricing.try_deserialize::<Ec2Pricing>()
}
