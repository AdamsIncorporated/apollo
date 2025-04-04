use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;

pub static SECTOR_INDUSTY_MAPPING: Lazy<HashMap<&'static str, HashSet<&'static str>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert(
            "basic-materials",
            HashSet::from([
                "specialty-chemicals",
                "gold",
                "building-materials",
                "copper",
                "steel",
                "agricultural-inputs",
                "chemicals",
                "other-industrial-metals-mining",
                "lumber-wood-production",
                "aluminum",
                "other-precious-metals-mining",
                "coking-coal",
                "paper-paper-products",
                "silver",
            ]),
        );
        map.insert(
            "communication-services",
            HashSet::from([
                "internet-content-information",
                "telecom-services",
                "entertainment",
                "electronic-gaming-multimedia",
                "advertising-agencies",
                "broadcasting",
                "publishing",
            ]),
        );
        map.insert(
            "consumer-cyclical",
            HashSet::from([
                "internet-retail",
                "auto-manufacturers",
                "restaurants",
                "home-improvement-retail",
                "travel-services",
                "specialty-retail",
                "apparel-retail",
                "residential-construction",
                "footwear-accessories",
                "packaging-containers",
                "lodging",
                "auto-parts",
                "auto-truck-dealerships",
                "gambling",
                "resorts-casinos",
                "leisure",
                "apparel-manufacturing",
                "personal-services",
                "furnishings-fixtures-appliances",
                "recreational-vehicles",
                "luxury-goods",
                "department-stores",
                "textile-manufacturing",
            ]),
        );
        map.insert(
            "consumer-defensive",
            HashSet::from([
                "discount-stores",
                "beverages-non-alcoholic",
                "household-personal-products",
                "packaged-foods",
                "tobacco",
                "confectioners",
                "farm-products",
                "food-distribution",
                "grocery-stores",
                "beverages-brewers",
                "education-training-services",
                "beverages-wineries-distilleries",
            ]),
        );
        map.insert(
            "energy",
            HashSet::from([
                "oil-gas-integrated",
                "oil-gas-midstream",
                "oil-gas-e-p",
                "oil-gas-equipment-services",
                "oil-gas-refining-marketing",
                "uranium",
                "oil-gas-drilling",
                "thermal-coal",
            ]),
        );
        map.insert(
            "financial-services",
            HashSet::from([
                "banks-diversified",
                "credit-services",
                "asset-management",
                "insurance-diversified",
                "banks-regional",
                "capital-markets",
                "financial-data-stock-exchanges",
                "insurance-property-casualty",
                "insurance-brokers",
                "insurance-life",
                "insurance-specialty",
                "mortgage-finance",
                "insurance-reinsurance",
                "shell-companies",
                "financial-conglomerates",
            ]),
        );
        map.insert(
            "healthcare",
            HashSet::from([
                "drug-manufacturers-general",
                "healthcare-plans",
                "biotechnology",
                "medical-devices",
                "diagnostics-research",
                "medical-instruments-supplies",
                "medical-care-facilities",
                "drug-manufacturers-specialty-generic",
                "health-information-services",
                "medical-distribution",
                "pharmaceutical-retailers",
            ]),
        );
        map.insert(
            "industrials",
            HashSet::from([
                "aerospace-defense",
                "specialty-industrial-machinery",
                "railroads",
                "building-products-equipment",
                "farm-heavy-construction-machinery",
                "specialty-business-services",
                "integrated-freight-logistics",
                "waste-management",
                "conglomerates",
                "industrial-distribution",
                "engineering-construction",
                "rental-leasing-services",
                "consulting-services",
                "trucking",
                "electrical-equipment-parts",
                "airlines",
                "tools-accessories",
                "pollution-treatment-controls",
                "security-protection-services",
                "marine-shipping",
                "metal-fabrication",
                "infrastructure-operations",
                "staffing-employment-services",
                "airports-air-services",
                "business-equipment-supplies",
            ]),
        );
        map.insert(
            "real-estate",
            HashSet::from([
                "reit-specialty",
                "reit-industrial",
                "reit-retail",
                "reit-residential",
                "reit-healthcare-facilities",
                "real-estate-services",
                "reit-office",
                "reit-diversified",
                "reit-mortgage",
                "reit-hotel-motel",
                "real-estate-development",
                "real-estate-diversified",
            ]),
        );
        map.insert(
            "technology",
            HashSet::from([
                "software-infrastructure",
                "semiconductors",
                "consumer-electronics",
                "software-application",
                "information-technology-services",
                "semiconductor-equipment-materials",
                "communication-equipment",
                "computer-hardware",
                "electronic-components",
                "scientific-technical-instruments",
                "solar",
                "electronics-computer-distribution",
            ]),
        );
        map.insert(
            "utilities",
            HashSet::from([
                "utilities-regulated-electric",
                "utilities-renewable",
                "utilities-diversified",
                "utilities-regulated-gas",
                "utilities-independent-power-producers",
                "utilities-regulated-water",
            ]),
        );
        map
    });
