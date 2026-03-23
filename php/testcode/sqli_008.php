<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_laravel_binding
function sqli008(BenchmarkRequest $req): BenchmarkResponse {
    FakeDB::setPdo(getDbConnection());
    $id = $req->param('id');
    $rows = FakeDB::select("SELECT * FROM users WHERE id = ?", [$id]); // vuln-code-snippet safe-line php_sqli_laravel_binding
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_laravel_binding
