//! Payments Service (Rust/Actix-web)
//!
//! Handles payment processing, integrates with Stripe.
//! Called by Gateway service.
//!
//! TAINT FLOWS:
//! - /intent: amount, currency, orderId → Stripe API
//! - /process: paymentIntentId, paymentMethodId → Stripe API
//! - /{id}/refund: paymentId, reason → Stripe API + logs

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// Config
// vuln-code-snippet start infodisclosurePaymentsHardcodedKey
const STRIPE_SECRET_KEY: &str = "sk_test_fake_key_for_testing";  //Hardcoded key // vuln-code-snippet target-line infodisclosurePaymentsHardcodedKey
// vuln-code-snippet end infodisclosurePaymentsHardcodedKey
const PORT: u16 = 4002;

// State
struct AppState {
    payments: Mutex<HashMap<String, Payment>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Payment {
    id: String,
    amount: i64,
    currency: String,
    status: String,
    order_id: String,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct CreateIntentRequest {
    amount: i64,
    currency: String,
    #[serde(rename = "orderId")]
    order_id: String,
}

#[derive(Debug, Deserialize)]
struct ProcessPaymentRequest {
    #[serde(rename = "paymentIntentId")]
    payment_intent_id: String,
    #[serde(rename = "paymentMethodId")]
    payment_method_id: String,
}

#[derive(Debug, Deserialize)]
struct RefundRequest {
    reason: String,
}

/// Create a payment intent
/// TAINT: amount, currency, order_id flow from frontend
// vuln-code-snippet start inputvalPaymentsMissingValidation
async fn create_intent(
    data: web::Data<AppState>,
    req: web::Json<CreateIntentRequest>,
) -> impl Responder {
    let payment_id = format!("pi_{}", uuid::Uuid::new_v4());

    //No validation on amount (could be negative)
    //No validation on currency (could be invalid)
    let payment = Payment {
        id: payment_id.clone(),
        amount: req.amount, // vuln-code-snippet target-line inputvalPaymentsMissingValidation
        currency: req.currency.clone(),
        status: "pending".to_string(),
        order_id: req.order_id.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    // Store payment
    data.payments.lock().unwrap().insert(payment_id.clone(), payment.clone());

    // In real code, this would call Stripe API
    // TAINT: Data would flow to external API
    tracing::info!(
        "Created payment intent: {} for order {} amount {}",
        payment_id, req.order_id, req.amount
    );

    HttpResponse::Ok().json(payment)
}
// vuln-code-snippet end inputvalPaymentsMissingValidation

/// Process a payment
/// TAINT: payment_intent_id, payment_method_id flow from frontend
async fn process_payment(
    data: web::Data<AppState>,
    req: web::Json<ProcessPaymentRequest>,
) -> impl Responder {
    let mut payments = data.payments.lock().unwrap();

    //No verification that this user owns this payment intent
    if let Some(payment) = payments.get_mut(&req.payment_intent_id) {
        payment.status = "succeeded".to_string();

        // In real code: call Stripe to confirm payment
        // stripe.paymentIntents.confirm(payment_intent_id, { payment_method: payment_method_id })

        tracing::info!(
            "Processed payment: {} with method {}",
            req.payment_intent_id, req.payment_method_id
        );

        HttpResponse::Ok().json(payment.clone())
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": "Payment intent not found"
        }))
    }
}

/// Refund a payment
/// TAINT: payment_id from URL, reason from body
async fn refund_payment(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: web::Json<RefundRequest>,
) -> impl Responder {
    let payment_id = path.into_inner();
    let mut payments = data.payments.lock().unwrap();

    if let Some(payment) = payments.get_mut(&payment_id) {
        payment.status = "refunded".to_string();

        //Reason logged without sanitization
        tracing::info!(
            "Refunded payment {}: reason = {}",
            payment_id, req.reason
        );

        HttpResponse::Ok().json(payment.clone())
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": "Payment not found"
        }))
    }
}

/// Health check
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "payments"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let state = web::Data::new(AppState {
        payments: Mutex::new(HashMap::new()),
    });

    tracing::info!("Payments service starting on port {}", PORT);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/intent", web::post().to(create_intent))
            .route("/process", web::post().to(process_payment))
            .route("/{payment_id}/refund", web::post().to(refund_payment))
            .route("/health", web::get().to(health))
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
