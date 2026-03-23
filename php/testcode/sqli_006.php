<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_wpdb_prepare
function sqli006(BenchmarkRequest $req): BenchmarkResponse {
    $wpdb = new FakeWpdb(getDbConnection());
    $id = $req->param('id');
    $sql = $wpdb->prepare("SELECT * FROM wp_users WHERE id = %s", $id); // vuln-code-snippet safe-line php_sqli_wpdb_prepare
    $result = $wpdb->query($sql);
    return BenchmarkResponse::ok("Rows affected: " . ($result ? '1' : '0'));
}
// vuln-code-snippet end php_sqli_wpdb_prepare
