<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_laravel_raw
function sqli007(BenchmarkRequest $req): BenchmarkResponse {
    FakeDB::setPdo(getDbConnection());
    $id = $req->param('id');
    $rows = FakeDB::selectRaw("SELECT * FROM users WHERE id=" . $id); // vuln-code-snippet vuln-line php_sqli_laravel_raw
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_laravel_raw
