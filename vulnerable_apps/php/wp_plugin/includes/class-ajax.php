<?php
// WordPress Events Manager Plugin - AJAX handlers
require_once __DIR__ . '/../../../testcode/shared.php';

$wpdb = new FakeWpdb(getDbConnection());

// vuln-code-snippet start wp_sqli_ajax_search
function wp_sqli_ajax_search(FakeWpdb $wpdb, BenchmarkRequest $req): BenchmarkResponse {
    $term = $req->param('term');
    $results = $wpdb->get_results("SELECT * FROM wp_events WHERE name LIKE '%" . $term . "%'"); // vuln-code-snippet vuln-line wp_sqli_ajax_search
    return BenchmarkResponse::json($results);
}
// vuln-code-snippet end wp_sqli_ajax_search

// vuln-code-snippet start wp_sqli_ajax_safe
function wp_sqli_ajax_safe(FakeWpdb $wpdb, BenchmarkRequest $req): BenchmarkResponse {
    $term = $req->param('term');
    $sql = $wpdb->prepare("SELECT * FROM wp_events WHERE name LIKE %s", '%' . $term . '%'); // vuln-code-snippet safe-line wp_sqli_ajax_safe
    $results = $wpdb->get_results($sql);
    return BenchmarkResponse::json($results);
}
// vuln-code-snippet end wp_sqli_ajax_safe

// vuln-code-snippet start wp_fi_template
function wp_fi_template(BenchmarkRequest $req): BenchmarkResponse {
    $view = $req->param('view');
    $path = dirname(__DIR__) . '/templates/' . $view . '.php';
    include($path); // vuln-code-snippet vuln-line wp_fi_template
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end wp_fi_template

// vuln-code-snippet start wp_fi_safe
function wp_fi_safe(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['list', 'detail', 'calendar', 'map'];
    $view = $req->param('view');
    if (!in_array($view, $allowed, true)) { // vuln-code-snippet safe-line wp_fi_safe
        return BenchmarkResponse::badRequest('Invalid template');
    }
    $path = dirname(__DIR__) . '/templates/' . $view . '.php';
    include($path);
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end wp_fi_safe
