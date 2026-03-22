//! Search Service (Rust/Actix-web)
//!
//! Handles product search, integrates with Elasticsearch.
//! Called by Gateway service.
//!
//! TAINT FLOWS:
//! - /products: q, category, sortBy → Elasticsearch query
//! - /suggestions: q → autocomplete index

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

const PORT: u16 = 4003;

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
    category: Option<String>,
    #[serde(rename = "minPrice")]
    min_price: Option<f64>,
    #[serde(rename = "maxPrice")]
    max_price: Option<f64>,
    #[serde(rename = "sortBy")]
    sort_by: Option<String>,
}

#[derive(Debug, Serialize)]
struct Product {
    id: String,
    name: String,
    description: String,
    price: f64,
    category: String,
    #[serde(rename = "imageUrl")]
    image_url: String,
}

#[derive(Debug, Serialize)]
struct SearchResult {
    items: Vec<Product>,
    total: usize,
    page: usize,
    #[serde(rename = "pageSize")]
    page_size: usize,
}

/// Search products
/// TAINT: query params flow to Elasticsearch
// vuln-code-snippet start sqliSearchElasticsearchInjection
async fn search_products(query: web::Query<SearchQuery>) -> impl Responder {
    // Simulated product data
    let products = vec![
        Product {
            id: "prod_1".to_string(),
            name: "Wireless Headphones".to_string(),
            description: "High-quality wireless headphones".to_string(),
            price: 99.99,
            category: "electronics".to_string(),
            image_url: "/images/headphones.jpg".to_string(),
        },
        Product {
            id: "prod_2".to_string(),
            name: "Running Shoes".to_string(),
            description: "Comfortable running shoes".to_string(),
            price: 79.99,
            category: "clothing".to_string(),
            image_url: "/images/shoes.jpg".to_string(),
        },
    ];

    // TAINT: User query used in search
    // In real code, this would build an Elasticsearch query
    let search_term = query.q.as_deref().unwrap_or("");

    //Building query string without proper escaping
    // This would be vulnerable to Elasticsearch injection
    let es_query = format!(
        r#"{{
            "query": {{
                "bool": {{
                    "must": [
                        {{ "match": {{ "name": "{}" }} }}
                    ]
                }}
            }},
            "sort": [{{ "{}": "asc" }}]
        }}"#,
        search_term,  // TAINT: User input in query // vuln-code-snippet vuln-line sqliSearchElasticsearchInjection
        query.sort_by.as_deref().unwrap_or("relevance")  // TAINT: User input in sort
    );

    tracing::info!("Search query: {}", es_query);

    // Filter products (simulated)
    let filtered: Vec<Product> = products
        .into_iter()
        .filter(|p| {
            if search_term.is_empty() {
                return true;
            }
            p.name.to_lowercase().contains(&search_term.to_lowercase())
                || p.description.to_lowercase().contains(&search_term.to_lowercase())
        })
        .filter(|p| {
            if let Some(cat) = &query.category {
                return p.category == *cat;
            }
            true
        })
        .filter(|p| {
            if let Some(min) = query.min_price {
                if p.price < min {
                    return false;
                }
            }
            if let Some(max) = query.max_price {
                if p.price > max {
                    return false;
                }
            }
            true
        })
        .collect();

    let total = filtered.len();

    HttpResponse::Ok().json(SearchResult {
        items: filtered,
        total,
        page: 1,
        page_size: 20,
    })
}
// vuln-code-snippet end sqliSearchElasticsearchInjection

/// Get search suggestions (autocomplete)
/// TAINT: q flows to autocomplete index
async fn search_suggestions(query: web::Query<SearchQuery>) -> impl Responder {
    let search_term = query.q.as_deref().unwrap_or("");

    // Simulated suggestions
    let all_suggestions = vec![
        "wireless headphones",
        "wireless earbuds",
        "wireless charger",
        "running shoes",
        "running shorts",
        "gaming mouse",
        "gaming keyboard",
    ];

    // TAINT: User input used to filter suggestions
    //Could be used for ReDoS if regex is built from user input
    let suggestions: Vec<String> = all_suggestions
        .into_iter()
        .filter(|s| s.contains(search_term))
        .map(|s| s.to_string())
        .take(5)
        .collect();

    HttpResponse::Ok().json(suggestions)
}

/// Health check
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "search"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Search service starting on port {}", PORT);

    HttpServer::new(|| {
        App::new()
            .route("/products", web::get().to(search_products))
            .route("/suggestions", web::get().to(search_suggestions))
            .route("/health", web::get().to(health))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
