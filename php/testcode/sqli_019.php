<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_eloquent_whereraw
function sqli019(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $status = $req->param('status');
    $raw = "status = '" . $status . "'";
    $result = $pdo->query("SELECT * FROM orders WHERE " . $raw); // vuln-code-snippet vuln-line php_sqli_eloquent_whereraw
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_eloquent_whereraw
