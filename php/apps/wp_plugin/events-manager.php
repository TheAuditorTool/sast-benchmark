<?php
// WordPress Events Manager Plugin - Main plugin hooks
require_once __DIR__ . '/../../testcode/shared.php';

$wpdb = new FakeWpdb(getDbConnection());

// vuln-code-snippet start wp_sqli_get_events
function wp_sqli_get_events(FakeWpdb $wpdb, BenchmarkRequest $req): BenchmarkResponse {
    $cat_id = $req->param('cat_id');
    $results = $wpdb->get_results("SELECT * FROM wp_events WHERE cat_id=" . $cat_id); // vuln-code-snippet vuln-line wp_sqli_get_events
    return BenchmarkResponse::json($results);
}
// vuln-code-snippet end wp_sqli_get_events

// vuln-code-snippet start wp_sqli_prepared
function wp_sqli_prepared(FakeWpdb $wpdb, BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('cat_id');
    $sql = $wpdb->prepare("SELECT * FROM wp_events WHERE cat_id=%s", $id); // vuln-code-snippet safe-line wp_sqli_prepared
    $results = $wpdb->get_results($sql);
    return BenchmarkResponse::json($results);
}
// vuln-code-snippet end wp_sqli_prepared

// vuln-code-snippet start wp_xss_shortcode
function wp_xss_shortcode(BenchmarkRequest $req): BenchmarkResponse {
    $atts = ['title' => $req->param('title')];
    $html = '<div>' . $atts['title'] . '</div>'; // vuln-code-snippet vuln-line wp_xss_shortcode
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end wp_xss_shortcode

// vuln-code-snippet start wp_xss_escaped
function wp_xss_escaped(BenchmarkRequest $req): BenchmarkResponse {
    $atts = ['title' => $req->param('title')];
    $html = '<div>' . esc_html($atts['title']) . '</div>'; // vuln-code-snippet safe-line wp_xss_escaped
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end wp_xss_escaped

// vuln-code-snippet start wp_csrf_no_nonce
function wp_csrf_no_nonce(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('event_name');
    $date = $req->post('event_date');
    $result = "INSERT INTO wp_events (name, date) VALUES ('$name', '$date')"; // vuln-code-snippet vuln-line wp_csrf_no_nonce
    return BenchmarkResponse::ok('Event saved');
}
// vuln-code-snippet end wp_csrf_no_nonce

// vuln-code-snippet start wp_csrf_nonce
function wp_csrf_nonce(BenchmarkRequest $req): BenchmarkResponse {
    $nonce = $req->post('_wpnonce');
    if (!wp_verify_nonce($nonce, 'save_event')) { // vuln-code-snippet safe-line wp_csrf_nonce
        return BenchmarkResponse::error('Invalid nonce', 403);
    }
    $name = $req->post('event_name');
    return BenchmarkResponse::ok('Event saved');
}
// vuln-code-snippet end wp_csrf_nonce
