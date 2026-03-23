<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_pdo_exec
function sqli021(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $table = $req->param('table');
    $pdo->exec("DROP TABLE IF EXISTS " . $table); // vuln-code-snippet vuln-line php_sqli_pdo_exec
    return BenchmarkResponse::ok("Table dropped");
}
// vuln-code-snippet end php_sqli_pdo_exec
