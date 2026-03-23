<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_wpdb_raw
function sqli005(BenchmarkRequest $req): BenchmarkResponse {
    $wpdb = new FakeWpdb(getDbConnection());
    $id = $req->param('id');
    $result = $wpdb->query("SELECT * FROM wp_users WHERE id=$id"); // vuln-code-snippet vuln-line php_sqli_wpdb_raw
    return BenchmarkResponse::ok("Rows affected: " . ($result ? '1' : '0'));
}
// vuln-code-snippet end php_sqli_wpdb_raw
