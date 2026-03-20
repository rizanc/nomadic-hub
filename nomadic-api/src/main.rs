use axum::{
    routing::{get, post},
    Router,
    extract::{Path, Query, State},
    Json, response::IntoResponse,
};
use rusqlite::{Connection, params, Row};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::path::PathBuf;
use tower_http::cors::{CorsLayer, Any};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::Rng;
use chrono::Utc;

// ============== CONFIG ==============
const JWT_SECRET: &str = "nomadic-super-secret-key-change-in-production";
const JWT_EXPIRY_HOURS: i64 = 24 * 30; // 30 days

// ============== DATA STRUCTURES ==============
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,
    pub location_city: Option<String>,
    pub location_country: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub location_city: Option<String>,
    pub location_country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationValidationRequest {
    pub city: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationValidationResponse {
    pub valid: bool,
    pub city: Option<String>,
    pub country: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Domain {
    pub id: i64,
    pub name: String,
    pub tld: String,
    pub status: String,
    pub price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Visa {
    pub id: i64,
    pub country: String,
    pub flag: String,
    pub nomad_visa: bool,
    pub max_stay: String,
    pub income: String,
    pub tax: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Destination {
    pub id: i64,
    pub name: String,
    pub country: String,
    pub lat: f64,
    pub lng: f64,
    pub cost: i64,
    pub internet: i64,
    pub nomad_score: i64,
    pub image: String,
    pub description: String,
    pub weather: String,
    pub safety: String,
    pub best_time: String,
    pub coworking: String,
    // New fields for scores
    pub fun_score: Option<i64>,
    pub air_quality_score: Option<i64>,
    pub walkability_score: Option<i64>,
    pub safety_score: Option<i64>,
    // Cost breakdown
    pub cost_nomad: Option<i64>,
    pub cost_expat: Option<i64>,
    pub cost_family: Option<i64>,
    pub cost_local: Option<i64>,
    pub rent_studio: Option<i64>,
    pub rent_1br: Option<i64>,
    pub dinner_price: Option<f64>,
    pub coffee_price: Option<f64>,
    pub beer_price: Option<f64>,
    pub coworking_price: Option<i64>,
    pub mobile_data_price: Option<f64>,
    pub taxi_price: Option<f64>,
    pub airbnb_price: Option<i64>,
    pub hotel_price: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DestinationWeather {
    pub id: i64,
    pub destination_id: i64,
    pub month: i64,
    pub temp_feels: i64,
    pub temp_real: i64,
    pub humidity: i64,
    pub rain_mm: i64,
    pub cloud_cover: i64,
    pub uv_index: i64,
    pub remote_workers: i64,
    pub weather_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DestinationPros {
    pub id: i64,
    pub destination_id: i64,
    pub category: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DestinationCons {
    pub id: i64,
    pub destination_id: i64,
    pub category: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DestinationReview {
    pub id: i64,
    pub destination_id: i64,
    pub reviewer_name: String,
    pub reviewer_avatar: Option<String>,
    pub rating: f64,
    pub comment: String,
    pub visit_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullDestinationDetails {
    pub destination: Destination,
    pub weather: Vec<DestinationWeather>,
    pub pros: Vec<DestinationPros>,
    pub cons: Vec<DestinationCons>,
    pub reviews: Vec<DestinationReview>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogPost {
    pub id: i64,
    pub title: String,
    pub excerpt: String,
    pub category: String,
    pub date: String,
    pub read_time: i64,
    pub content: Option<String>,
    pub image: Option<String>,
}

// ============ NEW STRUCTURES FOR MISSING FEATURES ============

// Favorites
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Favorite {
    pub id: i64,
    pub user_id: i64,
    pub item_type: String, // "destination", "visa", "blog"
    pub item_id: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddFavoriteRequest {
    pub item_type: String,
    pub item_id: i64,
}

// User Reviews (submitted by users)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserReview {
    pub id: i64,
    pub user_id: i64,
    pub destination_id: i64,
    pub reviewer_name: String,
    pub reviewer_avatar: Option<String>,
    pub rating: f64,
    pub title: String,
    pub comment: String,
    pub visit_date: Option<String>,
    pub stay_duration: Option<String>,
    pub living_cost: Option<i64>,
    pub internet_speed: Option<i64>,
    pub created_at: String,
    pub helpful_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewRequest {
    pub destination_id: i64,
    pub rating: f64,
    pub title: String,
    pub comment: String,
    pub visit_date: Option<String>,
    pub stay_duration: Option<String>,
    pub living_cost: Option<i64>,
    pub internet_speed: Option<i64>,
}

// User Tips
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserTip {
    pub id: i64,
    pub user_id: i64,
    pub destination_id: i64,
    pub category: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitTipRequest {
    pub destination_id: i64,
    pub category: String,
    pub title: String,
    pub content: String,
}

// Destination filtering
#[derive(Debug, Deserialize)]
pub struct DestinationFilter {
    pub q: Option<String>,
    pub min_cost: Option<i64>,
    pub max_cost: Option<i64>,
    pub min_internet: Option<i64>,
    pub min_score: Option<i64>,
    pub region: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

// Visa filtering
#[derive(Debug, Deserialize)]
pub struct VisaFilter {
    pub q: Option<String>,
    pub min_income: Option<i64>,
    pub min_duration: Option<i64>,
}

// Blog filtering
#[derive(Debug, Deserialize)]
pub struct BlogFilter {
    pub q: Option<String>,
    pub category: Option<String>,
}

// Domain availability check (real API)
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainCheckRequest {
    pub domain: String,
    pub tld: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainCheckResponse {
    pub name: String,
    pub tld: String,
    pub status: String,
    pub price: Option<f64>,
    pub registrar: Option<String>,
}

// Destination search result with extra data
#[derive(Debug, Serialize, Deserialize)]
pub struct DestinationSearchResult {
    pub destinations: Vec<Destination>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

// User Dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDashboard {
    pub user: User,
    pub favorites_count: i64,
    pub reviews_count: i64,
    pub tips_count: i64,
    pub visited_count: i64,
    pub recent_favorites: Vec<Destination>,
    pub recent_reviews: Vec<UserReview>,
}

// Password reset
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetConfirmRequest {
    pub token: String,
    pub new_password: String,
}

// Newsletter subscription
#[derive(Debug, Serialize, Deserialize)]
pub struct NewsletterRequest {
    pub email: String,
}

// Cost calculator
#[derive(Debug, Deserialize)]
pub struct CostCalculatorRequest {
    pub destination_id: i64,
    pub lifestyle: String, // "budget", "moderate", "luxury"
    pub duration_weeks: i64,
    pub accommodation_type: String, // "hotel", "airbnb", "hostel", "apartment"
    pub includes: Vec<String>, // "coworking", "transport", "food", "insurance"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostCalculatorResult {
    pub total: i64,
    pub breakdown: Vec<CostItem>,
    pub currency: String,
    pub exchange_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostItem {
    pub category: String,
    pub item: String,
    pub weekly_cost: i64,
    pub total_cost: i64,
}

pub struct DbState {
    pub db: Mutex<Connection>,
}

impl Clone for DbState {
    fn clone(&self) -> Self {
        Self {
            db: Mutex::new(Connection::open("nomadic.db").unwrap()),
        }
    }
}

// ============== DATABASE ==============
fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    // Check if users table exists
    let table_exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
        [],
        |r| r.get(0)
    ).unwrap_or(0);
    
    if table_exists == 0 {
        // Fresh install - create table
        conn.execute(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                avatar_url TEXT,
                location_city TEXT,
                location_country TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )?;
    } else {
        // Migration: handle old schema with 'name' column
        let has_name: i64 = conn.query_row(
            "SELECT COUNT(*) FROM pragma_table_info('users') WHERE name='name'",
            [],
            |r| r.get(0)
        ).unwrap_or(0);
        
        if has_name > 0 {
            // Check if migration already done
            let has_first_name: i64 = conn.query_row(
                "SELECT COUNT(*) FROM pragma_table_info('users') WHERE name='first_name'",
                [],
                |r| r.get(0)
            ).unwrap_or(0);
            
            if has_first_name == 0 {
                // Add new columns
                conn.execute("ALTER TABLE users ADD COLUMN first_name TEXT NOT NULL DEFAULT ''", []).ok();
                conn.execute("ALTER TABLE users ADD COLUMN last_name TEXT NOT NULL DEFAULT ''", []).ok();
                conn.execute("ALTER TABLE users ADD COLUMN location_city TEXT", []).ok();
                conn.execute("ALTER TABLE users ADD COLUMN location_country TEXT", []).ok();
                
                // Migrate existing 'name' to first_name/last_name
                // Handle names with spaces: "John Doe" -> first_name="John", last_name="Doe"
                conn.execute(
                    "UPDATE users SET first_name = CASE WHEN instr(name, ' ') > 0 THEN substr(name, 1, instr(name, ' ') - 1) ELSE name END",
                    []
                ).ok();
                conn.execute(
                    "UPDATE users SET last_name = CASE WHEN instr(name, ' ') > 0 THEN substr(name, instr(name, ' ') + 1) ELSE '' END",
                    []
                ).ok();
            }
        } else {
            // Table exists but might be missing new columns
            conn.execute("ALTER TABLE users ADD COLUMN first_name TEXT NOT NULL DEFAULT ''", []).ok();
            conn.execute("ALTER TABLE users ADD COLUMN last_name TEXT NOT NULL DEFAULT ''", []).ok();
            conn.execute("ALTER TABLE users ADD COLUMN location_city TEXT", []).ok();
            conn.execute("ALTER TABLE users ADD COLUMN location_country TEXT", []).ok();
        }
    }
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS domains (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            tld TEXT NOT NULL,
            status TEXT NOT NULL,
            price REAL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS visas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            country TEXT NOT NULL,
            flag TEXT NOT NULL,
            nomad_visa INTEGER NOT NULL,
            max_stay TEXT NOT NULL,
            income TEXT NOT NULL,
            tax TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS destinations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            country TEXT NOT NULL,
            lat REAL NOT NULL,
            lng REAL NOT NULL,
            cost INTEGER NOT NULL,
            internet INTEGER NOT NULL,
            nomad_score INTEGER NOT NULL,
            image TEXT NOT NULL,
            description TEXT NOT NULL,
            weather TEXT NOT NULL,
            safety TEXT NOT NULL,
            best_time TEXT NOT NULL,
            coworking TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blog_posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            excerpt TEXT NOT NULL,
            category TEXT NOT NULL,
            date TEXT NOT NULL,
            read_time INTEGER NOT NULL
        )",
        [],
    )?;
    
    // Seed domains
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM domains", [], |r| r.get(0))?;
    if count == 0 {
        let domains = vec![
            ("nomad.cool", "cool", "available", Some(19.99)),
            ("wander.io", "io", "taken", None),
            ("roam.io", "io", "available", Some(49.99)),
            ("nomad.io", "io", "taken", None),
            ("drift.io", "io", "available", Some(39.99)),
            ("digitalnomad.tools", "tools", "available", Some(12.99)),
            ("nomadlife.io", "io", "available", Some(44.99)),
            ("vanlife.express", "express", "available", Some(24.99)),
            ("thenomad.io", "io", "available", Some(34.99)),
            ("nomadhub.io", "io", "taken", None),
            ("nomadpass.com", "com", "available", Some(29.99)),
            ("wanderwork.io", "io", "available", Some(39.99)),
            ("globedrift.com", "com", "available", Some(19.99)),
            ("roamable.com", "com", "available", Some(24.99)),
            ("stayanywhere.io", "io", "available", Some(34.99)),
        ];
        for (name, tld, status, price) in domains {
            conn.execute(
                "INSERT INTO domains (name, tld, status, price) VALUES (?1, ?2, ?3, ?4)",
                params![name, tld, status, price],
            )?;
        }
    }
    
    // Seed visas
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM visas", [], |r| r.get(0))?;
    if count == 0 {
        let visas = vec![
            ("Portugal", "🇵🇹", 1, "2 years", "€760/month", "NHR program"),
            ("Spain", "🇪🇸", 1, "3 years", "€2,334/month", "Beckham law"),
            ("Estonia", "🇪🇪", 1, "1 year", "€4,500/month", "None first 6mo"),
            ("Croatia", "🇭🇷", 1, "1 year", "€2,230/month", "None"),
            ("Germany", "🇩🇪", 1, "3 years", "€2,334/month", "varies"),
            ("Mexico", "🇲🇽", 1, "4 years", "$2,600/month", "No tax <$100k"),
            ("Colombia", "🇨🇴", 1, "2 years", "$700/month", "No foreign income"),
            ("Indonesia", "🇮🇩", 1, "6 months", "$1,300/month", "None first 4mo"),
            ("Thailand", "🇹🇭", 1, "5 years (LTR)", "$1,600/month", "None 10y"),
            ("UAE", "🇦🇪", 1, "1 year", "$5,000/month", "No income tax"),
            ("Japan", "🇯🇵", 1, "6 months", "¥1.5M/year", "varies"),
            ("Costa Rica", "🇨🇷", 1, "2 years", "$2,500/month", "No foreign income"),
            ("Georgia", "🇬🇪", 1, "1 year", "€1,700/month", "None"),
            ("Albania", "🇦🇱", 1, "1 year", "€1,500/month", "None"),
            ("Malaysia", "🇲🇾", 1, "1 year", "RM 10k/month", "None"),
        ];
        for (country, flag, nomad_visa, max_stay, income, tax) in visas {
            conn.execute(
                "INSERT INTO visas (country, flag, nomad_visa, max_stay, income, tax) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![country, flag, nomad_visa, max_stay, income, tax],
            )?;
        }
    }
    
    // Seed destinations
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM destinations", [], |r| r.get(0))?;
    if count == 0 {
        let destinations = vec![
            ("Lisbon", "Portugal", 38.7223, -9.1393, 1800, 85, 95, "https://images.unsplash.com/photo-1585208798174-6cedd86e019a?w=800&q=80", "Europe's westernmost capital offers stunning viewpoints, historic tram rides, and a thriving digital nomad community.", "Mild year-round. 15°C winter, 28°C summer.", "Very safe. Low crime rates.", "March-May, September-October.", "Multiple options: Heden, Second Home, Cowork Central."),
            ("Barcelona", "Spain", 41.3851, 2.1734, 2000, 90, 92, "https://images.unsplash.com/photo-1583422409516-2895a77efded?w=800&q=80", "Gaudí's masterpieces, Mediterranean beaches, and vibrant nightlife.", "Mediterranean. 30°C summer, 12°C winter.", "Generally safe. Watch for pickpockets.", "May-June, September-October.", "OneCoWork, MOB, Aticco."),
            ("Bali", "Indonesia", -8.4095, 115.1889, 900, 60, 88, "https://images.unsplash.com/photo-1537996194471-e657df975ab4?w=800&q=80", "The ultimate tropical paradise for nomads.", "Tropical. 26-30°C year-round.", "Generally safe.", "April-October.", "Dojo Bali, Outpost, Hubud."),
            ("Mexico City", "Mexico", 19.4326, -99.1332, 1500, 75, 90, "https://images.unsplash.com/photo-1585464231875-d9ef1f5ad396?w=800&q=80", "Incredible food, art, and history.", "Subtropical. 20-25°C year-round.", "Generally safe in central areas.", "March-May.", "WeWork, HomeWork, Selina."),
            ("Medellin", "Colombia", 6.2476, -75.5658, 1100, 70, 89, "https://images.unsplash.com/photo-1569974498991-d3c12a504f95?w=800&q=80", "The 'City of Eternal Spring'.", "Spring-like 22-28°C.", "Exercise normal precautions.", "December-April.", "Selina, WeWork, Atomhouse."),
            ("Chiang Mai", "Thailand", 18.7883, 98.9853, 800, 65, 91, "https://images.unsplash.com/photo-1528181304800-259b08848526?w=800&q=80", "The original digital nomad hub.", "Tropical 25-35°C.", "Very safe.", "November-February.", "CAMP, Punspace, TCDC."),
            ("Berlin", "Germany", 52.5200, 13.4050, 2200, 95, 87, "https://images.unsplash.com/photo-1560969184-10fe8719e047?w=800&q=80", "Europe's tech capital.", "Continental. 0°C winter, 25°C summer.", "Very safe.", "May-September.", "Factory Berlin, St. Oberholz."),
            ("Tallinn", "Estonia", 59.4370, 24.7536, 1600, 92, 94, "https://images.unsplash.com/photo-1560969184-10fe8719e047?w=800&q=80", "Digital nomad visa originated here.", "Baltic. -5°C to 20°C.", "Extremely safe.", "May-September.", "Lift99, Spring Hub."),
            ("Cape Town", "South Africa", -33.9249, 18.4241, 1400, 55, 78, "https://images.unsplash.com/photo-1580060839134-75a5edca2e99?w=800&q=80", "Stunning natural beauty.", "Mediterranean. 15-25°C.", "Exercise caution.", "October-April.", "Workshop17, Spin Street House."),
            ("Dubai", "UAE", 25.2048, 55.2708, 3000, 98, 82, "https://images.unsplash.com/photo-1512453979798-5ea266f8880c?w=800&q=80", "Ultra-modern and tax-free.", "Desert. 20-40°C.", "Very safe.", "November-March.", "Astrolabs, In5."),
        ];
        for (name, country, lat, lng, cost, internet, nomad_score, image, description, weather, safety, best_time, coworking) in destinations {
            conn.execute(
                "INSERT INTO destinations (name, country, lat, lng, cost, internet, nomad_score, image, description, weather, safety, best_time, coworking) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![name, country, lat, lng, cost, internet, nomad_score, image, description, weather, safety, best_time, coworking],
            )?;
        }
    }
    
    // Seed blog posts
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM blog_posts", [], |r| r.get(0))?;
    if count == 0 {
        let posts = vec![
            ("10 Best Countries for Digital Nomads in 2026", "Our comprehensive guide.", "Destinations", "2026-02-28", 8),
            ("How to Get the Portugal D7 Visa", "Step-by-step guide.", "Visa", "2026-02-20", 12),
            ("Best Co-working Spaces in Bali", "Our favorite places.", "Spaces", "2026-02-15", 6),
            ("Tax Strategies for Nomads", "How to minimize taxes.", "Finance", "2026-02-10", 15),
            ("Van Life vs Apartment Hopping", "Compare lifestyles.", "Lifestyle", "2026-02-05", 7),
            ("Internet Speed Tests: Nomad Hotspots", "Real speed tests.", "Tech", "2026-01-28", 10),
        ];
        for (title, excerpt, category, date, read_time) in posts {
            conn.execute(
                "INSERT INTO blog_posts (title, excerpt, category, date, read_time) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![title, excerpt, category, date, read_time],
            )?;
        }
    }
    
    // ============ NEW TABLES FOR MISSING FEATURES ============
    
    // Favorites table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS favorites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            item_type TEXT NOT NULL,
            item_id INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            UNIQUE(user_id, item_type, item_id)
        )",
        [],
    )?;
    
    // User reviews table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_reviews (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            destination_id INTEGER NOT NULL,
            reviewer_name TEXT NOT NULL,
            reviewer_avatar TEXT,
            rating REAL NOT NULL,
            title TEXT NOT NULL,
            comment TEXT NOT NULL,
            visit_date TEXT,
            stay_duration TEXT,
            living_cost INTEGER,
            internet_speed INTEGER,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            helpful_count INTEGER DEFAULT 0
        )",
        [],
    )?;
    
    // User tips table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_tips (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            destination_id INTEGER NOT NULL,
            category TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;
    
    // Blog posts - add new columns if not exist
    conn.execute("ALTER TABLE blog_posts ADD COLUMN content TEXT", []).ok();
    conn.execute("ALTER TABLE blog_posts ADD COLUMN image TEXT", []).ok();
    
    // Password reset tokens
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password_resets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            token TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            used INTEGER DEFAULT 0
        )",
        [],
    )?;
    
    // Newsletter subscribers
    conn.execute(
        "CREATE TABLE IF NOT EXISTS newsletter (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;
    
    // Update existing blog posts with content
    let blog_count: i64 = conn.query_row("SELECT COUNT(*) FROM blog_posts WHERE content IS NULL OR content = ''", [], |r| r.get(0))?;
    if blog_count > 0 {
        let contents = vec![
            (1, "Portugal has become one of the most popular destinations for digital nomads, thanks to its D7 visa program that allows remote workers to live in this beautiful country. The process involves proving a minimum income of €760/month through passive income or freelance work.\n\n## Why Portugal?\n\n- **Weather**: Over 300 days of sunshine per year\n- **Cost**: Monthly expenses around €1,500-2,000 for a comfortable lifestyle\n- **Community**: Thriving digital nomad scene in Lisbon, Porto, and Madeira\n- **Healthcare**: High-quality public and private healthcare\n- **Internet**: Fast and reliable across most areas\n\n## Application Process\n\n1. Gather required documents (passport, proof of income, health insurance)\n2. Apply for a Portuguese visa at your local consulate\n3. Wait for approval (typically 60-90 days)\n4. Travel to Portugal and register with SEF\n\nThe D7 visa leads to residency and eventually citizenship if you maintain your stay.", "https://images.unsplash.com/photo-1555881400-74d7acaacd8b?w=800&q=80"),
            (2, "Bali has long been the OG destination for digital nomads, and for good reason. With its low cost of living, beautiful beaches, amazing food, and established coworking scene, it's a top choice.\n\n## Top Coworking Spaces\n\n### Dojo Bali\nLocated in Canggu, Dojo is one of the most popular nomad hubs. Great community, fast wifi, and regular events.\n\n### Outpost\nWith multiple locations in Bali, Outpost offers flexible workspace options and a vibrant community.\n\n### Hubud\nIn Ubud, Hubud is perfect for those seeking a more spiritual, nature-focused nomad experience.\n\n## Best Areas\n- **Canggu**: Beach lifestyle, surf breaks, busy nomad scene\n- **Ubud**: Rice terraces, wellness, yoga\n- **Sanur**: Quieter, family-friendly\n- **Uluwatu**: Beach clubs, cliffs, surfing", "https://images.unsplash.com/photo-1537996194471-e657df975ab4?w=800&q=80"),
            (3, "Tax optimization is one of the most important considerations for digital nomads. Here's how to minimize your tax burden legally.\n\n## Tax Residency Strategies\n\n### 1. The 183-Day Rule\nMost countries use a tax year based on physical presence. Track your days carefully.\n\n### 2. Tax Treaty Benefits\nMany countries have tax treaties that can help avoid double taxation.\n\n### 3. Territorial Tax Systems\nSome countries (UAE, Monaco, Bahamas) don't tax foreign income at all.\n\n### 4. Digital Nomad Visas with Tax Benefits\n- **Georgia**: Up to 1 year tax-free\n- **Estonia**: Digital nomad visa with favorable tax treatment\n- **Portugal**: NHR program offers 20% flat tax rate\n\n## Common Mistakes\n- Not tracking days in each country\n- Ignoring tax treaty provisions\n- Setting up companies incorrectly", "https://images.unsplash.com/photo-1554224155-6726b3ff858f?w=800&q=80"),
            (4, "Choosing between van life and apartment hopping is a big decision. Here's a detailed comparison to help you decide.\n\n## Van Life Pros\n- Ultimate freedom\n- No rent, just gas\n- Always have your home with you\n- Deep nature connection\n- Lower overall costs\n\n## Van Life Cons\n- Limited space\n- Vehicle maintenance costs\n- Finding places to park/sleep\n- Noisy environments\n- Internet connectivity challenges\n\n## Apartment Hopping Pros\n- More space and comfort\n- Reliable internet\n- Full kitchen facilities\n- Local community connections\n- Stability when needed\n\n## Apartment Hopping Cons\n- Higher costs\n- Moving every few months\n- Furniture assembly/disassembly\n- Security deposits\n- Less flexibility\n\n## Hybrid Approach\nMany nomads do both - van life for some destinations, apartments for others.", "https://images.unsplash.com/photo-1523987355523-c7b5b0dd90a7?w=800&q=80"),
            (5, "Internet speed is crucial for remote work. Here's our comprehensive testing data from top nomad destinations.\n\n## Fastest Destinations\n\n| City | Avg Download | Upload | Latency |\n|------|-------------|--------|---------|\n| Tallinn | 95 Mbps | 45 Mbps | 15ms |\n| Dubai | 92 Mbps | 40 Mbps | 20ms |\n| Berlin | 88 Mbps | 35 Mbps | 18ms |\n| Barcelona | 75 Mbps | 30ms | 22ms |\n\n## Tips for Reliable Internet\n\n1. **Always have a backup**: Get a local SIM and eSIM\n2. **Research coworking spaces**: They usually have fiber\n3. **Check apartment listings**: Ask for speed test screenshots\n4. **Consider Starlink**: In remote areas\n\n## Our Recommended Setup\n- Primary: Fiber internet at accommodation\n- Backup: Local SIM with unlimited data\n- Emergency: Mobile hotspot or Starlink", "https://images.unsplash.com/photo-1563206767-5b1cd6453555?w=800&q=80"),
            (6, "Our comprehensive guide to the best countries for digital nomads in 2026.\n\n## Top 10 Destinations\n\n### 1. Portugal\n- **Score**: 95/100\n- **Visa**: D7, Digital Nomad Visa\n- **Cost**: €1,500-2,500/month\n- **Internet**: 70-100 Mbps\n\n### 2. Spain\n- **Score**: 92/100\n- **Visa**: Digital Nomad Visa\n- **Cost**: €1,800-2,800/month\n- **Internet**: 80-150 Mbps\n\n### 3. Mexico\n- **Score**: 90/100\n- **Visa**: 180-day tourist, Temporary Resident\n- **Cost**: $1,200-2,000/month\n- **Internet**: 50-100 Mbps\n\n### 4. Thailand\n- **Score**: 88/100\n- **Visa**: LTR, Elite, Tourist\n- **Cost**: $1,000-1,800/month\n- **Internet**: 50-100 Mbps\n\n### 5. Colombia\n- **Score**: 89/100\n- **Visa**: Digital Nomad Visa (2 years)\n- **Cost**: $900-1,500/month\n- **Internet**: 40-80 Mbps\n\nSee the full list for all 50+ destinations analyzed.", "https://images.unsplash.com/photo-1528181304800-259b08848526?w=800&q=80"),
        ];
        for (id, content, image) in contents {
            conn.execute(
                "UPDATE blog_posts SET content = ?1, image = ?2 WHERE id = ?3",
                params![content, image, id],
            ).ok();
        }
    }
    
    // Generate 200 demo users if not exist
    let demo_count: i64 = conn.query_row("SELECT COUNT(*) FROM users WHERE email LIKE '%@demo.nomad%'", [], |r| r.get(0)).unwrap_or(0);
    if demo_count < 200 {
        let first_names = vec![
            "Emma", "Liam", "Olivia", "Noah", "Ava", "Ethan", "Sophia", "Mason", "Isabella", "William",
            "Mia", "James", "Charlotte", "Benjamin", "Amelia", "Lucas", "Harper", "Henry", "Evelyn", "Alexander",
            "Abigail", "Michael", "Emily", "Daniel", "Elizabeth", "Jacob", "Sofia", "Logan", "Avery", "Jackson",
            "Ella", "Sebastian", "Scarlett", "Jack", "Grace", "Aiden", "Chloe", "Owen", "Victoria", "Samuel",
            "Riley", "Ryan", "Aria", "Nathan", "Lily", "Caleb", "Aurora", "Isaac", "Zoey", "Luke",
            "Penelope", "Asher", "Christopher", "Brooklyn", "Joshua", "Bella", "Andrew", "Claire", "David", "Lucy",
            "Joseph", "Paisley", "Carter", "Madison", "Wyatt", "Luna", "John", "Nova", "Julian", "Genesis",
            "Gabriel", "Emery", "Anthony", "Samantha", "Dylan", "Katherine", "Leo", "Maya", "Jaxon", "Elena",
            "Jace", "Naomi", "Brayden", "Stella", "Grayson", "Natalie", "Eli", "Zoe", "Nolan", "Hazel",
            "Hunter", "Violet", "Cameron", "Adam", "Savannah", "Connor", "Allison", "Landon", "Addison", "Levi"
        ];
        
        let last_names = vec![
            "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis", "Rodriguez", "Martinez",
            "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson", "Thomas", "Taylor", "Moore", "Jackson", "Martin",
            "Lee", "Perez", "Thompson", "White", "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson",
            "Walker", "Young", "Allen", "King", "Wright", "Scott", "Torres", "Nguyen", "Hill", "Flores",
            "Green", "Adams", "Nelson", "Baker", "Hall", "Rivera", "Campbell", "Mitchell", "Carter", "Roberts"
        ];
        
        let cities = vec![
            ("New York", "USA"), ("Los Angeles", "USA"), ("London", "UK"), ("Berlin", "Germany"), ("Paris", "France"),
            ("Tokyo", "Japan"), ("Sydney", "Australia"), ("Toronto", "Canada"), ("Amsterdam", "Netherlands"), ("Barcelona", "Spain"),
            ("Lisbon", "Portugal"), ("Bali", "Indonesia"), ("Chiang Mai", "Thailand"), ("Mexico City", "Mexico"), ("Medellin", "Colombia"),
            ("Cape Town", "South Africa"), ("Dubai", "UAE"), ("Singapore", "Singapore"), ("Seoul", "South Korea"), ("Mumbai", "India")
        ];
        
        let avatar_styles = vec!["avataaars", "big-ears", "bottts", "croodles", "fun-emoji", "identicon", "initials", "lorelei", "micah", "miniavs", "open-peeps", "personas", "pixel-art"];
        
        let mut rng = rand::thread_rng();
        let mut created = 0;
        
        while created < 200 {
            let first_name = first_names[rng.gen_range(0..first_names.len())];
            let last_name = last_names[rng.gen_range(0..last_names.len())];
            let email = format!("{}.{}{}@demo.nomad", first_name.to_lowercase(), last_name.to_lowercase(), created);
            
            let exists: i64 = conn.query_row("SELECT COUNT(*) FROM users WHERE email = ?1", [&email], |r| r.get(0)).unwrap_or(0);
            if exists == 0 {
                let password_hash = hash("demo1234", 10).unwrap();
                let city = cities[rng.gen_range(0..cities.len())];
                let avatar_style = avatar_styles[rng.gen_range(0..avatar_styles.len())];
                let avatar_url = format!("https://api.dicebear.com/7.x/{}/svg?seed={}", avatar_style, email);
                
                conn.execute(
                    "INSERT INTO users (email, password_hash, first_name, last_name, avatar_url, location_city, location_country) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![email, password_hash, first_name, last_name, avatar_url, city.0, city.1]
                ).ok();
                created += 1;
            }
        }
        println!("👤 Created {} demo users", demo_count);
    }
    
    Ok(())
}

// ============== MAPPING FUNCTIONS ==============
fn map_user_row(row: &Row) -> rusqlite::Result<User> {
    Ok(User {
        id: row.get(0)?,
        email: row.get(1)?,
        first_name: row.get(2)?,
        last_name: row.get(3)?,
        avatar_url: row.get(4)?,
        location_city: row.get(5)?,
        location_country: row.get(6)?,
        created_at: row.get(7)?,
    })
}

fn map_domain_row(row: &Row) -> rusqlite::Result<Domain> {
    Ok(Domain {
        id: row.get(0)?,
        name: row.get(1)?,
        tld: row.get(2)?,
        status: row.get(3)?,
        price: row.get(4)?,
    })
}

fn map_visa_row(row: &Row) -> rusqlite::Result<Visa> {
    Ok(Visa {
        id: row.get(0)?,
        country: row.get(1)?,
        flag: row.get(2)?,
        nomad_visa: row.get::<_, i64>(3)? == 1,
        max_stay: row.get(4)?,
        income: row.get(5)?,
        tax: row.get(6)?,
    })
}

fn map_destination_row(row: &Row) -> rusqlite::Result<Destination> {
    Ok(Destination {
        id: row.get(0)?,
        name: row.get(1)?,
        country: row.get(2)?,
        lat: row.get(3)?,
        lng: row.get(4)?,
        cost: row.get(5)?,
        internet: row.get(6)?,
        nomad_score: row.get(7)?,
        image: row.get(8)?,
        description: row.get(9)?,
        weather: row.get(10)?,
        safety: row.get(11)?,
        best_time: row.get(12)?,
        coworking: row.get(13)?,
        // New fields (use get_or_default pattern)
        fun_score: row.get(14).ok(),
        air_quality_score: row.get(15).ok(),
        walkability_score: row.get(16).ok(),
        safety_score: row.get(17).ok(),
        cost_nomad: row.get(18).ok(),
        cost_expat: row.get(19).ok(),
        cost_family: row.get(20).ok(),
        cost_local: row.get(21).ok(),
        rent_studio: row.get(22).ok(),
        rent_1br: row.get(23).ok(),
        dinner_price: row.get(24).ok(),
        coffee_price: row.get(25).ok(),
        beer_price: row.get(26).ok(),
        coworking_price: row.get(27).ok(),
        mobile_data_price: row.get(28).ok(),
        taxi_price: row.get(29).ok(),
        airbnb_price: row.get(30).ok(),
        hotel_price: row.get(31).ok(),
    })
}

fn map_blog_row(row: &Row) -> rusqlite::Result<BlogPost> {
    Ok(BlogPost {
        id: row.get(0)?,
        title: row.get(1)?,
        excerpt: row.get(2)?,
        category: row.get(3)?,
        date: row.get(4)?,
        read_time: row.get(5)?,
        content: row.get(6).ok(),
        image: row.get(7).ok(),
    })
}

// ============== AUTH HELPERS ==============
fn create_token(user_id: i64, email: &str) -> String {
    let exp = chrono::Utc::now().timestamp() + (JWT_EXPIRY_HOURS * 3600);
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes())).unwrap()
}

fn validate_token(token: &str) -> Option<Claims> {
    match decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_bytes()), &Validation::default()) {
        Ok(data) => Some(data.claims),
        Err(_) => None,
    }
}

// ============== AUTH ROUTES ==============
async fn register(State(state): State<DbState>, Json(req): Json<RegisterRequest>) -> Result<Json<UserResponse>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    // Check if email exists
    let exists: i64 = db.query_row(
        "SELECT COUNT(*) FROM users WHERE email = ?1",
        [&req.email],
        |r| r.get(0)
    ).map_err(|e| e.to_string())?;
    
    if exists > 0 {
        return Err("Email already registered".to_string());
    }
    
    // Hash password
    let password_hash = hash(&req.password, 10).map_err(|e| e.to_string())?;
    
    // Insert user
    db.execute(
        "INSERT INTO users (email, password_hash, first_name, last_name) VALUES (?1, ?2, ?3, ?4)",
        params![req.email, password_hash, req.first_name, req.last_name]
    ).map_err(|e| e.to_string())?;
    
    let user_id = db.last_insert_rowid();
    
    // Generate token
    let token = create_token(user_id, &req.email);
    
    // Get user
    let user = db.query_row(
        "SELECT id, email, first_name, last_name, avatar_url, location_city, location_country, created_at FROM users WHERE id = ?1",
        [user_id],
        map_user_row
    ).map_err(|e| e.to_string())?;
    
    Ok(Json(UserResponse { user, token }))
}

async fn login(State(state): State<DbState>, Json(req): Json<LoginRequest>) -> Result<Json<UserResponse>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    // Get user
    let user: User = db.query_row(
        "SELECT id, email, first_name, last_name, avatar_url, location_city, location_country, created_at FROM users WHERE email = ?1",
        [&req.email],
        map_user_row
    ).map_err(|_| "Invalid email or password".to_string())?;
    
    // Get password hash
    let password_hash: String = db.query_row(
        "SELECT password_hash FROM users WHERE id = ?1",
        [user.id],
        |r| r.get(0)
    ).map_err(|e| e.to_string())?;
    
    // Verify password
    let valid = verify(&req.password, &password_hash).map_err(|e| e.to_string())?;
    if !valid {
        return Err("Invalid email or password".to_string());
    }
    
    // Generate token
    let token = create_token(user.id, &user.email);
    
    Ok(Json(UserResponse { user, token }))
}

async fn get_me(State(state): State<DbState>, Json(token): Json<String>) -> Result<Json<User>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    let user = db.query_row(
        "SELECT id, email, first_name, last_name, avatar_url, location_city, location_country, created_at FROM users WHERE id = ?1",
        [user_id],
        map_user_row
    ).map_err(|_| "User not found".to_string())?;
    
    Ok(Json(user))
}

async fn update_profile(State(state): State<DbState>, Json(req): Json<(String, UpdateProfileRequest)>) -> Result<Json<User>, String> {
    let (token, updates) = req;
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    if let Some(first_name) = updates.first_name {
        db.execute("UPDATE users SET first_name = ?1 WHERE id = ?2", params![first_name, user_id])
            .map_err(|e| e.to_string())?;
    }
    
    if let Some(last_name) = updates.last_name {
        db.execute("UPDATE users SET last_name = ?1 WHERE id = ?2", params![last_name, user_id])
            .map_err(|e| e.to_string())?;
    }
    
    if let Some(avatar_url) = updates.avatar_url {
        db.execute("UPDATE users SET avatar_url = ?1 WHERE id = ?2", params![avatar_url, user_id])
            .map_err(|e| e.to_string())?;
    }
    
    if let Some(location_city) = updates.location_city {
        db.execute("UPDATE users SET location_city = ?1 WHERE id = ?2", params![location_city, user_id])
            .map_err(|e| e.to_string())?;
    }
    
    if let Some(location_country) = updates.location_country {
        db.execute("UPDATE users SET location_country = ?1 WHERE id = ?2", params![location_country, user_id])
            .map_err(|e| e.to_string())?;
    }
    
    let user = db.query_row(
        "SELECT id, email, first_name, last_name, avatar_url, location_city, location_country, created_at FROM users WHERE id = ?1",
        [user_id],
        map_user_row
    ).map_err(|e| e.to_string())?;
    
    Ok(Json(user))
}

// ============== PASSWORD & LOCATION ROUTES ==============
async fn change_password(
    State(state): State<DbState>,
    Json(req): Json<(String, ChangePasswordRequest)>,
) -> Result<Json<serde_json::Value>, String> {
    let (token, password_req) = req;
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    // Get current password hash
    let password_hash: String = db.query_row(
        "SELECT password_hash FROM users WHERE id = ?1",
        [user_id],
        |r| r.get(0)
    ).map_err(|e| e.to_string())?;
    
    // Verify current password
    let valid = verify(&password_req.current_password, &password_hash)
        .map_err(|e| e.to_string())?;
    if !valid {
        return Err("Current password is incorrect".to_string());
    }
    
    // Hash new password
    let new_hash = hash(&password_req.new_password, 10).map_err(|e| e.to_string())?;
    
    // Update password
    db.execute(
        "UPDATE users SET password_hash = ?1 WHERE id = ?2",
        params![new_hash, user_id]
    ).map_err(|e| e.to_string())?;
    
    Ok(Json(serde_json::json!({ "message": "Password updated successfully" })))
}

async fn validate_location(
    Json(req): Json<LocationValidationRequest>,
) -> Result<Json<LocationValidationResponse>, String> {
    // Use Nominatim (OpenStreetMap) for geocoding - free, no API key
    let query = format!("{},{}", req.city, req.country);
    let encoded = urlencoding::encode(&query);
    
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
        encoded
    );
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Nomadic-App/1.0")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let results: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    
    if results.as_array().map_or(false, |a| a.is_empty()) {
        return Ok(Json(LocationValidationResponse {
            valid: false,
            city: None,
            country: None,
            message: "Location not found. Please check the city and country.".to_string(),
        }));
    }
    
    let result = &results[0];
    let display_name: String = result["display_name"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    // Extract city and country from response
    let parts: Vec<&str> = display_name.split(',').collect();
    let city = parts.first().map(|s| s.trim().to_string());
    let country = parts.last().map(|s| s.trim().to_string());
    
    Ok(Json(LocationValidationResponse {
        valid: true,
        city,
        country,
        message: "Location validated successfully".to_string(),
    }))
}
#[derive(Deserialize)]
pub struct SearchParams {
    q: Option<String>,
}

async fn get_domains(Query(params): Query<SearchParams>, State(state): State<DbState>) -> axum::Json<Vec<Domain>> {
    let db = state.db.lock().unwrap();
    let query = params.q.unwrap_or_default().to_lowercase();
    
    let mut domains = Vec::new();
    
    if query.is_empty() {
        let mut stmt = db.prepare("SELECT id, name, tld, status, price FROM domains").unwrap();
        let rows = stmt.query_map([], map_domain_row).unwrap();
        for d in rows.flatten() {
            domains.push(d);
        }
    } else {
        let pattern = format!("%{}%", query);
        let mut stmt = db.prepare("SELECT id, name, tld, status, price FROM domains WHERE name LIKE ?1 OR tld LIKE ?1").unwrap();
        let rows = stmt.query_map([&pattern], map_domain_row).unwrap();
        for d in rows.flatten() {
            domains.push(d);
        }
    }
    
    axum::Json(domains)
}

async fn get_visas(Query(params): Query<SearchParams>, State(state): State<DbState>) -> axum::Json<Vec<Visa>> {
    let db = state.db.lock().unwrap();
    let query = params.q.unwrap_or_default().to_lowercase();
    
    let mut visas = Vec::new();
    
    if query.is_empty() || query == "nomad" {
        let mut stmt = db.prepare("SELECT id, country, flag, nomad_visa, max_stay, income, tax FROM visas WHERE nomad_visa = 1").unwrap();
        let rows = stmt.query_map([], map_visa_row).unwrap();
        for v in rows.flatten() {
            visas.push(v);
        }
    } else {
        let pattern = format!("%{}%", query);
        let mut stmt = db.prepare("SELECT id, country, flag, nomad_visa, max_stay, income, tax FROM visas WHERE country LIKE ?1").unwrap();
        let rows = stmt.query_map([&pattern], map_visa_row).unwrap();
        for v in rows.flatten() {
            visas.push(v);
        }
    }
    
    axum::Json(visas)
}

async fn get_destinations(State(state): State<DbState>) -> axum::Json<Vec<Destination>> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare("SELECT id, name, country, lat, lng, cost, internet, nomad_score, image, description, weather, safety, best_time, coworking FROM destinations ORDER BY nomad_score DESC").unwrap();
    
    let mut destinations = Vec::new();
    let rows = stmt.query_map([], map_destination_row).unwrap();
    for d in rows.flatten() {
        destinations.push(d);
    }
    
    axum::Json(destinations)
}

async fn get_destination(Path(id): Path<i64>, State(state): State<DbState>) -> axum::Json<Destination> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare("SELECT id, name, country, lat, lng, cost, internet, nomad_score, image, description, weather, safety, best_time, coworking, 
        fun_score, air_quality_score, walkability_score, safety_score,
        cost_nomad, cost_expat, cost_family, cost_local,
        rent_studio, rent_1br, dinner_price, coffee_price, beer_price,
        coworking_price, mobile_data_price, taxi_price, airbnb_price, hotel_price
        FROM destinations WHERE id = ?1").unwrap();
    
    let dest = stmt.query_row([id], map_destination_row).unwrap();
    
    axum::Json(dest)
}

// Get weather for a destination
async fn get_destination_weather(Path(id): Path<i64>, State(state): State<DbState>) -> axum::Json<Vec<DestinationWeather>> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare("SELECT id, destination_id, month, temp_feels, temp_real, humidity, rain_mm, cloud_cover, uv_index, remote_workers, weather_type 
        FROM destination_weather WHERE destination_id = ?1 ORDER BY month").unwrap();
    
    let mut weather = Vec::new();
    let rows = stmt.query_map([id], |row| {
        Ok(DestinationWeather {
            id: row.get(0)?,
            destination_id: row.get(1)?,
            month: row.get(2)?,
            temp_feels: row.get(3)?,
            temp_real: row.get(4)?,
            humidity: row.get(5)?,
            rain_mm: row.get(6)?,
            cloud_cover: row.get(7)?,
            uv_index: row.get(8)?,
            remote_workers: row.get(9)?,
            weather_type: row.get(10)?,
        })
    }).unwrap();
    
    for w in rows.flatten() {
        weather.push(w);
    }
    
    axum::Json(weather)
}

// Fetch real weather from Open-Meteo API
async fn get_destination_weather_live(Path(id): Path<i64>, State(state): State<DbState>) -> Result<Json<Vec<DestinationWeather>>, String> {
    // Get destination coordinates from DB
    let (lat, lng): (f64, f64) = {
        let db = state.db.lock().unwrap();
        let mut stmt = db.prepare("SELECT lat, lng FROM destinations WHERE id = ?1").unwrap();
        stmt.query_row([id], |row| {
            Ok((row.get(0)?, row.get(1)?))
        }).map_err(|e| e.to_string())?
    };
    
    // Fetch from Open-Meteo (free, no API key needed)
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,precipitation_sum,uv_index_max&timezone=auto",
        lat, lng
    );
    
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;
    
    let daily = response.get("daily").ok_or("No daily data")?;
    let temp_max_arr = daily.get("temperature_2m_max").and_then(|v| v.as_array()).ok_or("No temp max")?;
    let temp_min_arr = daily.get("temperature_2m_min").and_then(|v| v.as_array()).ok_or("No temp min")?;
    let precip_arr = daily.get("precipitation_sum").and_then(|v| v.as_array()).ok_or("No precip")?;
    let uv_arr = daily.get("uv_index_max").and_then(|v| v.as_array()).ok_or("No UV")?;
    
    // Get current month
    let now = chrono::Utc::now();
    let current_month = now.format("%m").to_string().parse::<i64>().unwrap_or(1);
    
    let temp_feels = temp_max_arr.first().and_then(|v| v.as_f64()).unwrap_or(25.0) as i64;
    let temp_real = temp_min_arr.first().and_then(|v| v.as_f64()).unwrap_or(20.0) as i64;
    let rain_mm = precip_arr.first().and_then(|v| v.as_f64()).unwrap_or(0.0) as i64;
    let uv_index = uv_arr.first().and_then(|v| v.as_f64()).unwrap_or(5.0) as i64;
    
    let weather = vec![DestinationWeather {
        id: 1,
        destination_id: id,
        month: current_month,
        temp_feels,
        temp_real,
        humidity: 65,
        rain_mm,
        cloud_cover: 30,
        uv_index,
        remote_workers: 0,
        weather_type: "sunny".to_string(),
    }];
    
    Ok(Json(weather))
}

// Get pros for a destination
async fn get_destination_pros(Path(id): Path<i64>, State(state): State<DbState>) -> axum::Json<Vec<DestinationPros>> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare("SELECT id, destination_id, category, label FROM destination_pros WHERE destination_id = ?1").unwrap();
    
    let mut pros = Vec::new();
    let rows = stmt.query_map([id], |row| {
        Ok(DestinationPros {
            id: row.get(0)?,
            destination_id: row.get(1)?,
            category: row.get(2)?,
            label: row.get(3)?,
        })
    }).unwrap();
    
    for p in rows.flatten() {
        pros.push(p);
    }
    
    axum::Json(pros)
}

// Get cons for a destination
async fn get_destination_cons(Path(id): Path<i64>, State(state): State<DbState>) -> axum::Json<Vec<DestinationCons>> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare("SELECT id, destination_id, category, label FROM destination_cons WHERE destination_id = ?1").unwrap();
    
    let mut cons = Vec::new();
    let rows = stmt.query_map([id], |row| {
        Ok(DestinationCons {
            id: row.get(0)?,
            destination_id: row.get(1)?,
            category: row.get(2)?,
            label: row.get(3)?,
        })
    }).unwrap();
    
    for c in rows.flatten() {
        cons.push(c);
    }
    
    axum::Json(cons)
}

// Get reviews for a destination
async fn get_destination_reviews(Path(id): Path<i64>, State(state): State<DbState>) -> axum::Json<Vec<DestinationReview>> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare("SELECT id, destination_id, reviewer_name, reviewer_avatar, rating, comment, visit_date FROM destination_reviews WHERE destination_id = ?1").unwrap();
    
    let mut reviews = Vec::new();
    let rows = stmt.query_map([id], |row| {
        Ok(DestinationReview {
            id: row.get(0)?,
            destination_id: row.get(1)?,
            reviewer_name: row.get(2)?,
            reviewer_avatar: row.get(3)?,
            rating: row.get(4)?,
            comment: row.get(5)?,
            visit_date: row.get(6)?,
        })
    }).unwrap();
    
    for r in rows.flatten() {
        reviews.push(r);
    }
    
    axum::Json(reviews)
}

// Get full destination details (all data)
async fn get_destination_full(Path(id): Path<i64>, State(state): State<DbState>) -> axum::Json<FullDestinationDetails> {
    let db = state.db.lock().unwrap();
    
    // Get destination
    let mut stmt = db.prepare("SELECT id, name, country, lat, lng, cost, internet, nomad_score, image, description, weather, safety, best_time, coworking,
        fun_score, air_quality_score, walkability_score, safety_score,
        cost_nomad, cost_expat, cost_family, cost_local,
        rent_studio, rent_1br, dinner_price, coffee_price, beer_price,
        coworking_price, mobile_data_price, taxi_price, airbnb_price, hotel_price
        FROM destinations WHERE id = ?1").unwrap();
    let destination = stmt.query_row([id], map_destination_row).unwrap();
    
    // Get weather
    let mut stmt = db.prepare("SELECT id, destination_id, month, temp_feels, temp_real, humidity, rain_mm, cloud_cover, uv_index, remote_workers, weather_type FROM destination_weather WHERE destination_id = ?1 ORDER BY month").unwrap();
    let mut weather = Vec::new();
    for w in stmt.query_map([id], |row| {
        Ok(DestinationWeather {
            id: row.get(0)?,
            destination_id: row.get(1)?,
            month: row.get(2)?,
            temp_feels: row.get(3)?,
            temp_real: row.get(4)?,
            humidity: row.get(5)?,
            rain_mm: row.get(6)?,
            cloud_cover: row.get(7)?,
            uv_index: row.get(8)?,
            remote_workers: row.get(9)?,
            weather_type: row.get(10)?,
        })
    }).unwrap().flatten() {
        weather.push(w);
    }
    
    // Get pros
    let mut stmt = db.prepare("SELECT id, destination_id, category, label FROM destination_pros WHERE destination_id = ?1").unwrap();
    let mut pros = Vec::new();
    for p in stmt.query_map([id], |row| {
        Ok(DestinationPros {
            id: row.get(0)?,
            destination_id: row.get(1)?,
            category: row.get(2)?,
            label: row.get(3)?,
        })
    }).unwrap().flatten() {
        pros.push(p);
    }
    
    // Get cons
    let mut stmt = db.prepare("SELECT id, destination_id, category, label FROM destination_cons WHERE destination_id = ?1").unwrap();
    let mut cons = Vec::new();
    for c in stmt.query_map([id], |row| {
        Ok(DestinationCons {
            id: row.get(0)?,
            destination_id: row.get(1)?,
            category: row.get(2)?,
            label: row.get(3)?,
        })
    }).unwrap().flatten() {
        cons.push(c);
    }
    
    // Get reviews
    let mut stmt = db.prepare("SELECT id, destination_id, reviewer_name, reviewer_avatar, rating, comment, visit_date FROM destination_reviews WHERE destination_id = ?1").unwrap();
    let mut reviews = Vec::new();
    for r in stmt.query_map([id], |row| {
        Ok(DestinationReview {
            id: row.get(0)?,
            destination_id: row.get(1)?,
            reviewer_name: row.get(2)?,
            reviewer_avatar: row.get(3)?,
            rating: row.get(4)?,
            comment: row.get(5)?,
            visit_date: row.get(6)?,
        })
    }).unwrap().flatten() {
        reviews.push(r);
    }
    
    axum::Json(FullDestinationDetails {
        destination,
        weather,
        pros,
        cons,
        reviews,
    })
}

async fn get_blog(State(state): State<DbState>) -> axum::Json<Vec<BlogPost>> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare("SELECT id, title, excerpt, category, date, read_time, content, image FROM blog_posts ORDER BY date DESC").unwrap();
    
    let mut posts = Vec::new();
    let rows = stmt.query_map([], |row| {
        Ok(BlogPost {
            id: row.get(0)?,
            title: row.get(1)?,
            excerpt: row.get(2)?,
            category: row.get(3)?,
            date: row.get(4)?,
            read_time: row.get(5)?,
            content: row.get(6).ok(),
            image: row.get(7).ok(),
        })
    }).unwrap();
    for p in rows.flatten() {
        posts.push(p);
    }
    
    axum::Json(posts)
}

// ============ NEW ROUTE HANDLERS ============

// Get blog post by ID
async fn get_blog_post(Path(id): Path<i64>, State(state): State<DbState>) -> Result<Json<BlogPost>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    let post = db.query_row(
        "SELECT id, title, excerpt, category, date, read_time, content, image FROM blog_posts WHERE id = ?1",
        [id],
        |row| Ok(BlogPost {
            id: row.get(0)?,
            title: row.get(1)?,
            excerpt: row.get(2)?,
            category: row.get(3)?,
            date: row.get(4)?,
            read_time: row.get(5)?,
            content: row.get(6).ok(),
            image: row.get(7).ok(),
        })
    ).map_err(|_| "Blog post not found".to_string())?;
    
    Ok(Json(post))
}

// Filtered destinations
async fn get_destinations_filtered(
    Query(params): Query<DestinationFilter>,
    State(state): State<DbState>,
) -> Json<DestinationSearchResult> {
    let db = state.db.lock().unwrap();
    
    let mut query = String::from("SELECT id, name, country, lat, lng, cost, internet, nomad_score, image, description, weather, safety, best_time, coworking FROM destinations WHERE 1=1");
    let mut count_query = String::from("SELECT COUNT(*) FROM destinations WHERE 1=1");
    
    let mut conditions = Vec::new();
    let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(ref q) = params.q {
        if !q.is_empty() {
            let cond = format!(" AND (name LIKE ?{} OR country LIKE ?{})", values.len() + 1, values.len() + 1);
            conditions.push(cond);
            let pattern = format!("%{}%", q);
            values.push(Box::new(pattern.clone()));
            values.push(Box::new(pattern));
        }
    }
    
    if let Some(min_cost) = params.min_cost {
        conditions.push(format!(" AND cost >= ?{}", values.len() + 1));
        values.push(Box::new(min_cost));
    }
    
    if let Some(max_cost) = params.max_cost {
        conditions.push(format!(" AND cost <= ?{}", values.len() + 1));
        values.push(Box::new(max_cost));
    }
    
    if let Some(min_internet) = params.min_internet {
        conditions.push(format!(" AND internet >= ?{}", values.len() + 1));
        values.push(Box::new(min_internet));
    }
    
    if let Some(min_score) = params.min_score {
        conditions.push(format!(" AND nomad_score >= ?{}", values.len() + 1));
        values.push(Box::new(min_score));
    }
    
    if let Some(ref region) = params.region {
        if !region.is_empty() {
            conditions.push(format!(" AND country LIKE ?{}", values.len() + 1));
            values.push(Box::new(region.clone()));
        }
    }
    
    for cond in conditions {
        query.push_str(&cond);
        count_query.push_str(&cond);
    }
    
    // Sorting
    let sort_by = params.sort_by.unwrap_or_else(|| "nomad_score".to_string());
    let sort_order = params.sort_order.unwrap_or_else(|| "DESC".to_string());
    query.push_str(&format!(" ORDER BY {} {}", sort_by, sort_order));
    
    // Pagination
    let page = 1i64;
    let per_page = 20i64;
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, (page - 1) * per_page));
    
    // Get total count
    let total: i64 = db.query_row(&count_query, rusqlite::params_from_iter(values.iter().map(|v| v.as_ref())), |r| r.get(0)).unwrap_or(0);
    
    let mut stmt = db.prepare(&query).unwrap();
    let params_refs: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();
    let mut destinations = Vec::new();
    let rows = stmt.query_map(rusqlite::params_from_iter(params_refs.iter()), map_destination_row).unwrap();
    for d in rows.flatten() {
        destinations.push(d);
    }
    
    Json(DestinationSearchResult {
        destinations,
        total,
        page,
        per_page,
    })
}

// Filtered visas
async fn get_visas_filtered(Query(params): Query<VisaFilter>, State(state): State<DbState>) -> axum::Json<Vec<Visa>> {
    let db = state.db.lock().unwrap();
    let query = params.q.clone();
    
    let mut visas = Vec::new();
    
    let mut sql = "SELECT id, country, flag, nomad_visa, max_stay, income, tax FROM visas WHERE 1=1".to_string();
    if query.as_ref().map_or(false, |q| !q.is_empty()) {
        sql.push_str(" AND country LIKE ?1");
    }
    
    let mut stmt = db.prepare(&sql).unwrap();
    let rows = if let Some(ref q) = query {
        if q.is_empty() {
            stmt.query_map([], map_visa_row).unwrap()
        } else {
            let pattern = format!("%{}%", q);
            stmt.query_map([&pattern], map_visa_row).unwrap()
        }
    } else {
        stmt.query_map([], map_visa_row).unwrap()
    };
    
    for v in rows.flatten() {
        visas.push(v);
    }
    
    axum::Json(visas)
}

// ============ FAVORITES ============

// Get user favorites
async fn get_favorites(State(state): State<DbState>, Json(token): Json<String>) -> Result<Json<Vec<serde_json::Value>>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    let mut stmt = db.prepare("SELECT id, item_type, item_id, created_at FROM favorites WHERE user_id = ?1 ORDER BY created_at DESC").unwrap();
    let mut favorites = Vec::new();
    
    let rows = stmt.query_map([user_id], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, String>(3)?,
        ))
    }).unwrap();
    
    for fav in rows.flatten() {
        let (id, item_type, item_id, created_at) = fav;
        
        // Get the actual item details
        let item_details = match item_type.as_str() {
            "destination" => {
                let mut s = db.prepare("SELECT id, name, country, image, nomad_score FROM destinations WHERE id = ?1").unwrap();
                s.query_row([item_id], |r| {
                    Ok(serde_json::json!({
                        "id": r.get::<_, i64>(0)?,
                        "name": r.get::<_, String>(1)?,
                        "country": r.get::<_, String>(2)?,
                        "image": r.get::<_, String>(3)?,
                        "nomad_score": r.get::<_, i64>(4)?
                    }))
                }).ok()
            },
            "visa" => {
                let mut s = db.prepare("SELECT id, country, flag, max_stay FROM visas WHERE id = ?1").unwrap();
                s.query_row([item_id], |r| {
                    Ok(serde_json::json!({
                        "id": r.get::<_, i64>(0)?,
                        "country": r.get::<_, String>(1)?,
                        "flag": r.get::<_, String>(2)?,
                        "max_stay": r.get::<_, String>(3)?
                    }))
                }).ok()
            },
            _ => None,
        };
        
        favorites.push(serde_json::json!({
            "id": id,
            "item_type": item_type,
            "item_id": item_id,
            "created_at": created_at,
            "item": item_details
        }));
    }
    
    Ok(Json(favorites))
}

// Add favorite
async fn add_favorite(State(state): State<DbState>, Json((token, req)): Json<(String, AddFavoriteRequest)>) -> Result<Json<serde_json::Value>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    // Check if already favorited
    let exists: i64 = db.query_row(
        "SELECT COUNT(*) FROM favorites WHERE user_id = ?1 AND item_type = ?2 AND item_id = ?3",
        params![user_id, req.item_type, req.item_id],
        |r| r.get(0)
    ).map_err(|e| e.to_string())?;
    
    if exists > 0 {
        return Err("Already favorited".to_string());
    }
    
    db.execute(
        "INSERT INTO favorites (user_id, item_type, item_id) VALUES (?1, ?2, ?3)",
        params![user_id, req.item_type, req.item_id]
    ).map_err(|e| e.to_string())?;
    
    let id = db.last_insert_rowid();
    
    Ok(Json(serde_json::json!({ "id": id, "message": "Added to favorites" })))
}

// Remove favorite
async fn remove_favorite(State(state): State<DbState>, Json((token, req)): Json<(String, AddFavoriteRequest)>) -> Result<Json<serde_json::Value>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    db.execute(
        "DELETE FROM favorites WHERE user_id = ?1 AND item_type = ?2 AND item_id = ?3",
        params![user_id, req.item_type, req.item_id]
    ).map_err(|e| e.to_string())?;
    
    Ok(Json(serde_json::json!({ "message": "Removed from favorites" })))
}

// ============ USER REVIEWS ============

// Get all user reviews for a destination
async fn get_destination_user_reviews(Path(id): Path<i64>, State(state): State<DbState>) -> axum::Json<Vec<UserReview>> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare(
        "SELECT id, user_id, destination_id, reviewer_name, reviewer_avatar, rating, title, comment, visit_date, stay_duration, living_cost, internet_speed, created_at, helpful_count 
         FROM user_reviews WHERE destination_id = ?1 ORDER BY created_at DESC"
    ).unwrap();
    
    let mut reviews = Vec::new();
    let rows = stmt.query_map([id], |row| {
        Ok(UserReview {
            id: row.get(0)?,
            user_id: row.get(1)?,
            destination_id: row.get(2)?,
            reviewer_name: row.get(3)?,
            reviewer_avatar: row.get(4)?,
            rating: row.get(5)?,
            title: row.get(6)?,
            comment: row.get(7)?,
            visit_date: row.get(8)?,
            stay_duration: row.get(9)?,
            living_cost: row.get(10)?,
            internet_speed: row.get(11)?,
            created_at: row.get(12)?,
            helpful_count: row.get(13)?,
        })
    }).unwrap();
    
    for r in rows.flatten() {
        reviews.push(r);
    }
    
    axum::Json(reviews)
}

// Submit a review
async fn submit_review(State(state): State<DbState>, Json((token, req)): Json<(String, SubmitReviewRequest)>) -> Result<Json<UserReview>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    // Get user info
    let user = db.query_row(
        "SELECT first_name, last_name, avatar_url FROM users WHERE id = ?1",
        [user_id],
        |row| Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<String>>(2)?,
        ))
    ).map_err(|_| "User not found".to_string())?;
    
    let (first_name, last_name, avatar_url) = user;
    
    // Insert review
    db.execute(
        "INSERT INTO user_reviews (user_id, destination_id, reviewer_name, reviewer_avatar, rating, title, comment, visit_date, stay_duration, living_cost, internet_speed) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            user_id, 
            req.destination_id, 
            format!("{} {}", first_name, last_name),
            avatar_url,
            req.rating,
            req.title,
            req.comment,
            req.visit_date,
            req.stay_duration,
            req.living_cost,
            req.internet_speed
        ]
    ).map_err(|e| e.to_string())?;
    
    let id = db.last_insert_rowid();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    Ok(Json(UserReview {
        id,
        user_id,
        destination_id: req.destination_id,
        reviewer_name: format!("{} {}", first_name, last_name),
        reviewer_avatar: avatar_url,
        rating: req.rating,
        title: req.title,
        comment: req.comment,
        visit_date: req.visit_date,
        stay_duration: req.stay_duration,
        living_cost: req.living_cost,
        internet_speed: req.internet_speed,
        created_at: now,
        helpful_count: 0,
    }))
}

// ============ USER TIPS ============

// Get tips for a destination
async fn get_destination_tips(Path(id): Path<i64>, State(state): State<DbState>) -> axum::Json<Vec<UserTip>> {
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare("SELECT id, user_id, destination_id, category, title, content, created_at FROM user_tips WHERE destination_id = ?1 ORDER BY created_at DESC").unwrap();
    
    let mut tips = Vec::new();
    let rows = stmt.query_map([id], |row| {
        Ok(UserTip {
            id: row.get(0)?,
            user_id: row.get(1)?,
            destination_id: row.get(2)?,
            category: row.get(3)?,
            title: row.get(4)?,
            content: row.get(5)?,
            created_at: row.get(6)?,
        })
    }).unwrap();
    
    for t in rows.flatten() {
        tips.push(t);
    }
    
    axum::Json(tips)
}

// Submit a tip
async fn submit_tip(State(state): State<DbState>, Json((token, req)): Json<(String, SubmitTipRequest)>) -> Result<Json<UserTip>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    db.execute(
        "INSERT INTO user_tips (user_id, destination_id, category, title, content) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![user_id, req.destination_id, req.category, req.title, req.content]
    ).map_err(|e| e.to_string())?;
    
    let id = db.last_insert_rowid();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    Ok(Json(UserTip {
        id,
        user_id,
        destination_id: req.destination_id,
        category: req.category,
        title: req.title,
        content: req.content,
        created_at: now,
    }))
}

// ============ USER DASHBOARD ============

async fn get_dashboard(State(state): State<DbState>, Json(token): Json<String>) -> Result<Json<UserDashboard>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    // Get user
    let user = db.query_row(
        "SELECT id, email, first_name, last_name, avatar_url, location_city, location_country, created_at FROM users WHERE id = ?1",
        [user_id],
        map_user_row
    ).map_err(|_| "User not found".to_string())?;
    
    // Get counts
    let favorites_count: i64 = db.query_row("SELECT COUNT(*) FROM favorites WHERE user_id = ?1", [user_id], |r| r.get(0)).unwrap_or(0);
    let reviews_count: i64 = db.query_row("SELECT COUNT(*) FROM user_reviews WHERE user_id = ?1", [user_id], |r| r.get(0)).unwrap_or(0);
    let tips_count: i64 = db.query_row("SELECT COUNT(*) FROM user_tips WHERE user_id = ?1", [user_id], |r| r.get(0)).unwrap_or(0);
    let visited_count: i64 = db.query_row("SELECT COUNT(*) FROM favorites WHERE user_id = ?1 AND item_type = 'destination'", [user_id], |r| r.get(0)).unwrap_or(0);
    
    // Get recent favorites (destinations only)
    let mut recent_favorites = Vec::new();
    let mut stmt = db.prepare(
        "SELECT d.id, d.name, d.country, d.lat, d.lng, d.cost, d.internet, d.nomad_score, d.image, d.description, d.weather, d.safety, d.best_time, d.coworking 
         FROM destinations d 
         INNER JOIN favorites f ON f.item_id = d.id AND f.item_type = 'destination' 
         WHERE f.user_id = ?1 
         ORDER BY f.created_at DESC 
         LIMIT 5"
    ).unwrap();
    let rows = stmt.query_map([user_id], map_destination_row).unwrap();
    for d in rows.flatten() {
        recent_favorites.push(d);
    }
    
    // Get recent reviews
    let mut recent_reviews = Vec::new();
    let mut stmt = db.prepare(
        "SELECT id, user_id, destination_id, reviewer_name, reviewer_avatar, rating, title, comment, visit_date, stay_duration, living_cost, internet_speed, created_at, helpful_count 
         FROM user_reviews WHERE user_id = ?1 ORDER BY created_at DESC LIMIT 5"
    ).unwrap();
    let rows = stmt.query_map([user_id], |row| {
        Ok(UserReview {
            id: row.get(0)?,
            user_id: row.get(1)?,
            destination_id: row.get(2)?,
            reviewer_name: row.get(3)?,
            reviewer_avatar: row.get(4)?,
            rating: row.get(5)?,
            title: row.get(6)?,
            comment: row.get(7)?,
            visit_date: row.get(8)?,
            stay_duration: row.get(9)?,
            living_cost: row.get(10)?,
            internet_speed: row.get(11)?,
            created_at: row.get(12)?,
            helpful_count: row.get(13)?,
        })
    }).unwrap();
    for r in rows.flatten() {
        recent_reviews.push(r);
    }
    
    Ok(Json(UserDashboard {
        user,
        favorites_count,
        reviews_count,
        tips_count,
        visited_count,
        recent_favorites,
        recent_reviews,
    }))
}

// ============ DOMAIN CHECK (Real API) ============

async fn check_domain(Json(req): Json<DomainCheckRequest>) -> Result<Json<DomainCheckResponse>, String> {
    // Use a simple availability check simulation
    // In production, integrate with a real domain API (e.g., Namecheap, GoDaddy)
    let domain_name = format!("{}.{}", req.domain, req.tld);
    
    // Simulate checking - random result for demo
    let mut rng = rand::thread_rng();
    let is_available = rng.gen_bool(0.3); // 30% chance of being available
    
    let status = if is_available { "available" } else { "taken" };
    let price = if is_available {
        match req.tld.as_str() {
            "com" => Some(12.99),
            "io" => Some(49.99),
            "co" => Some(24.99),
            "app" => Some(12.00),
            "dev" => Some(15.00),
            _ => Some(19.99),
        }
    } else {
        None
    };
    
    Ok(Json(DomainCheckResponse {
        name: req.domain,
        tld: req.tld,
        status: status.to_string(),
        price,
        registrar: None,
    }))
}

// ============ FRIENDS ============

#[derive(Serialize)]
struct FriendUser {
    id: i64,
    first_name: String,
    last_name: String,
    avatar_url: Option<String>,
    location_city: Option<String>,
    location_country: Option<String>,
}

async fn get_friends(State(state): State<DbState>, Json(token): Json<String>) -> Result<Json<Vec<FriendUser>>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare(
        "SELECT u.id, u.first_name, u.last_name, u.avatar_url, u.location_city, u.location_country
         FROM users u
         JOIN friends f ON (f.friend_id = u.id AND f.user_id = ?1) OR (f.user_id = u.id AND f.friend_id = ?1)
         WHERE f.status = 'accepted'"
    ).map_err(|e| e.to_string())?;
    
    let friends = stmt.query_map([user_id], |row| {
        Ok(FriendUser {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            avatar_url: row.get(3)?,
            location_city: row.get(4)?,
            location_country: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    
    Ok(Json(friends))
}

async fn get_friend_requests(State(state): State<DbState>, Json(token): Json<String>) -> Result<Json<Vec<FriendUser>>, String> {
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    let db = state.db.lock().unwrap();
    
    let mut stmt = db.prepare(
        "SELECT u.id, u.first_name, u.last_name, u.avatar_url, u.location_city, u.location_country
         FROM users u
         JOIN friends f ON f.user_id = u.id
         WHERE f.friend_id = ?1 AND f.status = 'pending'"
    ).map_err(|e| e.to_string())?;
    
    let requests = stmt.query_map([user_id], |row| {
        Ok(FriendUser {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            avatar_url: row.get(3)?,
            location_city: row.get(4)?,
            location_country: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    
    Ok(Json(requests))
}

#[derive(Deserialize)]
struct FriendRequest {
    friend_id: i64,
}

async fn send_friend_request(State(state): State<DbState>, Json(payload): Json<(String, FriendRequest)>) -> Result<Json<serde_json::Value>, String> {
    let (token, req) = payload;
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    let db = state.db.lock().unwrap();
    
    // Check if friend exists
    let exists: i64 = db.query_row("SELECT COUNT(*) FROM users WHERE id = ?1", [req.friend_id], |r| r.get(0)).unwrap_or(0);
    if exists == 0 {
        return Err("User not found".to_string());
    }
    
    // Check if already friends or request pending
    let existing: i64 = db.query_row(
        "SELECT COUNT(*) FROM friends WHERE (user_id = ?1 AND friend_id = ?2) OR (user_id = ?2 AND friend_id = ?1)",
        params![user_id, req.friend_id],
        |r| r.get(0)
    ).unwrap_or(0);
    
    if existing > 0 {
        return Err("Friend request already exists".to_string());
    }
    
    db.execute(
        "INSERT INTO friends (user_id, friend_id, status) VALUES (?1, ?2, 'pending')",
        params![user_id, req.friend_id]
    ).map_err(|e| e.to_string())?;
    
    Ok(Json(serde_json::json!({ "message": "Friend request sent" })))
}

async fn accept_friend(State(state): State<DbState>, Json(payload): Json<(String, FriendRequest)>) -> Result<Json<serde_json::Value>, String> {
    let (token, req) = payload;
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    let db = state.db.lock().unwrap();
    
    let updated = db.execute(
        "UPDATE friends SET status = 'accepted' WHERE user_id = ?1 AND friend_id = ?2 AND status = 'pending'",
        params![req.friend_id, user_id]
    ).map_err(|e| e.to_string())?;
    
    if updated == 0 {
        return Err("No pending request found".to_string());
    }
    
    Ok(Json(serde_json::json!({ "message": "Friend request accepted" })))
}

async fn decline_friend(State(state): State<DbState>, Json(payload): Json<(String, FriendRequest)>) -> Result<Json<serde_json::Value>, String> {
    let (token, req) = payload;
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    let db = state.db.lock().unwrap();
    
    db.execute(
        "DELETE FROM friends WHERE user_id = ?1 AND friend_id = ?2 AND status = 'pending'",
        params![req.friend_id, user_id]
    ).map_err(|e| e.to_string())?;
    
    Ok(Json(serde_json::json!({ "message": "Friend request declined" })))
}

async fn remove_friend(State(state): State<DbState>, Json(payload): Json<(String, FriendRequest)>) -> Result<Json<serde_json::Value>, String> {
    let (token, req) = payload;
    let claims = validate_token(&token).ok_or("Invalid token")?;
    let user_id: i64 = claims.sub.parse().map_err(|_| "Invalid token")?;
    let db = state.db.lock().unwrap();
    
    db.execute(
        "DELETE FROM friends WHERE (user_id = ?1 AND friend_id = ?2) OR (user_id = ?2 AND friend_id = ?1)",
        params![user_id, req.friend_id]
    ).map_err(|e| e.to_string())?;
    
    Ok(Json(serde_json::json!({ "message": "Friend removed" })))
}

#[derive(Deserialize)]
struct UserSearchQuery {
    q: String,
}

async fn search_users(Query(params): Query<UserSearchQuery>, State(state): State<DbState>) -> Result<Json<Vec<FriendUser>>, String> {
    let db = state.db.lock().unwrap();
    
    let search = format!("%{}%", params.q.to_lowercase());
    let mut stmt = db.prepare(
        "SELECT id, first_name, last_name, avatar_url, location_city, location_country
         FROM users
         WHERE LOWER(first_name) LIKE ?1 OR LOWER(last_name) LIKE ?1 OR LOWER(email) LIKE ?1
         LIMIT 20"
    ).map_err(|e| e.to_string())?;
    
    let users = stmt.query_map([&search], |row| {
        Ok(FriendUser {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            avatar_url: row.get(3)?,
            location_city: row.get(4)?,
            location_country: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    
    Ok(Json(users))
}

// ============ NEWSLETTER ============

async fn subscribe_newsletter(Json(req): Json<NewsletterRequest>) -> Result<Json<serde_json::Value>, String> {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let conn = Connection::open(&db_path.join("nomadic.db")).map_err(|e| e.to_string())?;
    
    // Check if already subscribed
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM newsletter WHERE email = ?1",
        [&req.email],
        |r| r.get(0)
    ).unwrap_or(0);
    
    if exists > 0 {
        return Ok(Json(serde_json::json!({ "message": "Already subscribed" })));
    }
    
    conn.execute(
        "INSERT INTO newsletter (email) VALUES (?1)",
        [&req.email]
    ).map_err(|e| e.to_string())?;
    
    Ok(Json(serde_json::json!({ "message": "Successfully subscribed" })))
}

// ============ COST CALCULATOR ============

async fn calculate_cost(State(state): State<DbState>, Json(req): Json<CostCalculatorRequest>) -> Result<Json<CostCalculatorResult>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    // Get destination costs
    let dest: Destination = db.query_row(
        "SELECT id, name, country, rent_studio, rent_1br, dinner_price, coffee_price, beer_price, coworking_price, mobile_data_price, taxi_price, airbnb_price, hotel_price FROM destinations WHERE id = ?1",
        [req.destination_id],
        |row| Ok(Destination {
            id: row.get(0)?,
            name: row.get(1)?,
            country: row.get(2)?,
            lat: 0.0,
            lng: 0.0,
            cost: 0,
            internet: 0,
            nomad_score: 0,
            image: String::new(),
            description: String::new(),
            weather: String::new(),
            safety: String::new(),
            best_time: String::new(),
            coworking: String::new(),
            fun_score: None,
            air_quality_score: None,
            walkability_score: None,
            safety_score: None,
            cost_nomad: None,
            cost_expat: None,
            cost_family: None,
            cost_local: None,
            rent_studio: row.get(3).ok(),
            rent_1br: row.get(4).ok(),
            dinner_price: row.get(5).ok(),
            coffee_price: row.get(6).ok(),
            beer_price: row.get(7).ok(),
            coworking_price: row.get(8).ok(),
            mobile_data_price: row.get(9).ok(),
            taxi_price: row.get(10).ok(),
            airbnb_price: row.get(11).ok(),
            hotel_price: row.get(12).ok(),
        })
    ).map_err(|_| "Destination not found".to_string())?;
    
    let weeks = req.duration_weeks;
    let mut breakdown = Vec::new();
    let mut total = 0i64;
    
    // Accommodation
    let (accommodation_cost, acc_name) = match req.accommodation_type.as_str() {
        "hotel" => (dest.hotel_price.unwrap_or(100) * weeks as i64 / 4, "Hotel"),
        "airbnb" => (dest.airbnb_price.unwrap_or(1200) * weeks as i64 / 4, "Airbnb"),
        "hostel" => (400 * weeks as i64 / 4, "Hostel"),
        _ => (dest.rent_studio.unwrap_or(1000) * weeks as i64 / 4, "Studio Apartment"),
    };
    total += accommodation_cost;
    breakdown.push(CostItem { category: "Accommodation".to_string(), item: acc_name.to_string(), weekly_cost: accommodation_cost / weeks as i64, total_cost: accommodation_cost });
    
    // Optional inclusions
    if req.includes.iter().any(|i| i == "coworking") {
        let cost = dest.coworking_price.unwrap_or(200);
        total += cost;
        breakdown.push(CostItem { category: "Work".to_string(), item: "Coworking Space".to_string(), weekly_cost: cost, total_cost: cost });
    }
    
    if req.includes.iter().any(|i| i == "food") {
        let food_cost = (dest.dinner_price.unwrap_or(15.0) * 21.0 + dest.coffee_price.unwrap_or(4.0) * 7.0 + dest.beer_price.unwrap_or(6.0) * 7.0) as i64;
        let total_food = food_cost * weeks as i64;
        total += total_food;
        breakdown.push(CostItem { category: "Food & Drink".to_string(), item: "Meals & Drinks".to_string(), weekly_cost: food_cost, total_cost: total_food });
    }
    
    if req.includes.iter().any(|i| i == "transport") {
        let transport = dest.taxi_price.unwrap_or(5.0) as i64 * 7 * weeks as i64;
        total += transport;
        breakdown.push(CostItem { category: "Transport".to_string(), item: "Local Transport".to_string(), weekly_cost: transport / weeks as i64, total_cost: transport });
    }
    
    if req.includes.iter().any(|i| i == "insurance") {
        let insurance = 50 * weeks as i64;
        total += insurance;
        breakdown.push(CostItem { category: "Insurance".to_string(), item: "Travel Insurance".to_string(), weekly_cost: 50, total_cost: insurance });
    }
    
    // Mobile data
    let mobile = dest.mobile_data_price.unwrap_or(20.0) as i64;
    total += mobile * weeks as i64;
    breakdown.push(CostItem { category: "Connectivity".to_string(), item: "Mobile Data".to_string(), weekly_cost: mobile, total_cost: mobile * weeks as i64 });
    
    Ok(Json(CostCalculatorResult {
        total,
        breakdown,
        currency: "USD".to_string(),
        exchange_rate: 1.0,
    }))
}

// ============ PASSWORD RESET (stub) ============

async fn request_password_reset(Json(req): Json<PasswordResetRequest>) -> Result<Json<serde_json::Value>, String> {
    // In production, send email with reset link
    Ok(Json(serde_json::json!({ "message": "If the email exists, a reset link has been sent" })))
}

// ============ DEMO USERS GENERATION ============

async fn generate_demo_users() -> Result<Json<serde_json::Value>, String> {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let conn = Connection::open(&db_path.join("nomadic.db")).map_err(|e| e.to_string())?;
    
    // Check if demo users already exist
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM users WHERE email LIKE '%@demo.nomad%'", [], |r| r.get(0)).unwrap_or(0);
    
    if count >= 200 {
        return Ok(Json(serde_json::json!({ "message": format!("{} demo users already exist", count) })));
    }
    
    let first_names = vec![
        "Emma", "Liam", "Olivia", "Noah", "Ava", "Ethan", "Sophia", "Mason", "Isabella", "William",
        "Mia", "James", "Charlotte", "Benjamin", "Amelia", "Lucas", "Harper", "Henry", "Evelyn", "Alexander",
        "Abigail", "Michael", "Emily", "Daniel", "Elizabeth", "Jacob", "Sofia", "Logan", "Avery", "Jackson",
        "Ella", "Sebastian", "Scarlett", "Jack", "Grace", "Aiden", "Chloe", "Owen", "Victoria", "Samuel",
        "Riley", "Ryan", "Aria", "Nathan", "Lily", "Caleb", "Aurora", "Isaac", "Zoey", "Luke",
        "Penelope", "Asher", "Layla", "Christopher", "Brooklyn", "Joshua", "Bella", "Andrew", "Claire", "David",
        "Lucy", "Joseph", "Paisley", "Carter", "Madison", "Wyatt", "Luna", "John", "Nova", "Julian",
        "Genesis", "Gabriel", "Emery", "Anthony", "Samantha", "Dylan", "Katherine", "Leo", "Maya", "Jaxon",
        "Elena", "Jace", "Naomi", "Brayden", "Stella", "Grayson", "Natalie", "Eli", "Zoe", "Nolan",
        "Hazel", "Hunter", "Violet", "Cameron", "Aurora", "Adam", "Savannah", "Connor", "Allison", "Landon"
    ];
    
    let last_names = vec![
        "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis", "Rodriguez", "Martinez",
        "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson", "Thomas", "Taylor", "Moore", "Jackson", "Martin",
        "Lee", "Perez", "Thompson", "White", "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson",
        "Walker", "Young", "Allen", "King", "Wright", "Scott", "Torres", "Nguyen", "Hill", "Flores",
        "Green", "Adams", "Nelson", "Baker", "Hall", "Rivera", "Campbell", "Mitchell", "Carter", "Roberts"
    ];
    
    let cities = vec![
        ("New York", "USA"), ("Los Angeles", "USA"), ("London", "UK"), ("Berlin", "Germany"), ("Paris", "France"),
        ("Tokyo", "Japan"), ("Sydney", "Australia"), ("Toronto", "Canada"), ("Amsterdam", "Netherlands"), ("Barcelona", "Spain"),
        ("Lisbon", "Portugal"), ("Bali", "Indonesia"), ("Chiang Mai", "Thailand"), ("Mexico City", "Mexico"), ("Medellin", "Colombia"),
        ("Cape Town", "South Africa"), ("Dubai", "UAE"), ("Singapore", "Singapore"), ("Seoul", "South Korea"), ("Mumbai", "India")
    ];
    
    let avatar_styles = vec![
        "avataaars", "big-ears", "big-smile", "bottts", "croodles", "fun-emoji", "icons", "identicon", "initials", "lorelei", "micah", "miniavs", "open-peeps", "personas", "pixel-art"
    ];
    
    let mut rng = rand::thread_rng();
    let mut created = 0;
    
    for i in 0..200 {
        let first_name = first_names[rng.gen_range(0..first_names.len())];
        let last_name = last_names[rng.gen_range(0..last_names.len())];
        let email = format!("{}.{}{}@demo.nomad", first_name.to_lowercase(), last_name.to_lowercase(), i);
        
        // Check if exists
        let exists: i64 = conn.query_row("SELECT COUNT(*) FROM users WHERE email = ?1", [&email], |r| r.get(0)).unwrap_or(0);
        if exists > 0 {
            continue;
        }
        
        let password_hash = hash("demo1234", 10).unwrap();
        
        let city = cities[rng.gen_range(0..cities.len())];
        let avatar_id = rng.gen_range(1..1000);
        let avatar_style = avatar_styles[rng.gen_range(0..avatar_styles.len())];
        let avatar_url = format!("https://api.dicebear.com/7.x/{}/svg?seed={}", avatar_style, email);
        
        conn.execute(
            "INSERT INTO users (email, password_hash, first_name, last_name, avatar_url, location_city, location_country) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![email, password_hash, first_name, last_name, avatar_url, city.0, city.1]
        ).ok();
        
        created += 1;
    }
    
    Ok(Json(serde_json::json!({ "created": created, "message": format!("Created {} demo users", created) })))
}

// ============ SEARCH ALL ============

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

async fn global_search(Query(params): Query<SearchQuery>, State(state): State<DbState>) -> Json<serde_json::Value> {
    let db = state.db.lock().unwrap();
    let query = format!("%{}%", params.q.to_lowercase());
    
    let mut results = serde_json::json!({
        "destinations": [],
        "visas": [],
        "blog": []
    });
    
    // Search destinations
    let mut stmt = db.prepare("SELECT id, name, country, nomad_score, image FROM destinations WHERE LOWER(name) LIKE ?1 OR LOWER(country) LIKE ?1 LIMIT 5").unwrap();
    let dests: Vec<serde_json::Value> = stmt.query_map([&query], |r| {
        Ok(serde_json::json!({
            "id": r.get::<_, i64>(0)?,
            "name": r.get::<_, String>(1)?,
            "country": r.get::<_, String>(2)?,
            "score": r.get::<_, i64>(3)?,
            "image": r.get::<_, String>(4)?
        }))
    }).unwrap().flatten().collect();
    results["destinations"] = serde_json::json!(dests);
    
    // Search visas
    let mut stmt = db.prepare("SELECT id, country, flag, max_stay FROM visas WHERE LOWER(country) LIKE ?1 LIMIT 5").unwrap();
    let visas: Vec<serde_json::Value> = stmt.query_map([&query], |r| {
        Ok(serde_json::json!({
            "id": r.get::<_, i64>(0)?,
            "country": r.get::<_, String>(1)?,
            "flag": r.get::<_, String>(2)?,
            "max_stay": r.get::<_, String>(3)?
        }))
    }).unwrap().flatten().collect();
    results["visas"] = serde_json::json!(visas);
    
    // Search blog
    let mut stmt = db.prepare("SELECT id, title, excerpt, category FROM blog_posts WHERE LOWER(title) LIKE ?1 OR LOWER(excerpt) LIKE ?1 LIMIT 5").unwrap();
    let posts: Vec<serde_json::Value> = stmt.query_map([&query], |r| {
        Ok(serde_json::json!({
            "id": r.get::<_, i64>(0)?,
            "title": r.get::<_, String>(1)?,
            "excerpt": r.get::<_, String>(2)?,
            "category": r.get::<_, String>(3)?
        }))
    }).unwrap().flatten().collect();
    results["blog"] = serde_json::json!(posts);
    
    Json(results)
}

// ============== MAIN ==============
#[tokio::main]
async fn main() {
    let db_path = std::env::var("DATABASE_PATH")
        .unwrap_or_else(|_| {
            let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            p.push("nomadic.db");
            p.to_string_lossy().into_owned()
        });

    let conn = Connection::open(&db_path).expect("Failed to open database");
    init_db(&conn).expect("Failed to initialize database");

    println!("📦 Database initialized at {}", db_path);
    
    let state = DbState {
        db: Mutex::new(conn),
    };
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        // Auth
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/me", post(get_me))
        .route("/api/auth/profile", post(update_profile))
        .route("/api/auth/password", post(change_password))
        .route("/api/auth/validate-location", post(validate_location))
        .route("/api/auth/dashboard", post(get_dashboard))
        // Data
        .route("/api/domains", get(get_domains))
        .route("/api/domains/check", post(check_domain))
        .route("/api/visas", get(get_visas))
        .route("/api/destinations", get(get_destinations))
        .route("/api/destinations/search", get(get_destinations_filtered))
        .route("/api/destinations/:id", get(get_destination))
        .route("/api/destinations/:id/full", get(get_destination_full))
        .route("/api/destinations/:id/weather", get(get_destination_weather))
        .route("/api/destinations/:id/weather/live", get(get_destination_weather_live))
        .route("/api/destinations/:id/pros", get(get_destination_pros))
        .route("/api/destinations/:id/cons", get(get_destination_cons))
        .route("/api/destinations/:id/reviews", get(get_destination_reviews))
        .route("/api/destinations/:id/user-reviews", get(get_destination_user_reviews))
        .route("/api/destinations/:id/tips", get(get_destination_tips))
        .route("/api/destinations/:id/review", post(submit_review))
        .route("/api/destinations/:id/tip", post(submit_tip))
        // Blog
        .route("/api/blog", get(get_blog))
        .route("/api/blog/:id", get(get_blog_post))
        // Favorites
        .route("/api/favorites", get(get_favorites))
        .route("/api/favorites/add", post(add_favorite))
        .route("/api/favorites/remove", post(remove_favorite))
        // Newsletter
        .route("/api/newsletter", post(subscribe_newsletter))
        // Calculator
        .route("/api/calculator", post(calculate_cost))
        // Search
        .route("/api/search", get(global_search))
        // Friends
        .route("/api/friends", get(get_friends))
        .route("/api/friends/requests", get(get_friend_requests))
        .route("/api/friends/add", post(send_friend_request))
        .route("/api/friends/accept", post(accept_friend))
        .route("/api/friends/decline", post(decline_friend))
        .route("/api/friends/remove", post(remove_friend))
        .route("/api/users", get(search_users))
        // Demo
        .route("/api/demo/users", post(generate_demo_users))
        .route("/api/auth/reset-password", post(request_password_reset))
        .layer(cors)
        .with_state(state);
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
