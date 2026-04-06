<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_schema_validate
function massassign014(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true);
    $schema = ['name' => 'string', 'email' => 'string'];
    foreach ($data as $key => $val) {
        if (!isset($schema[$key])) { // vuln-code-snippet safe-line php_massassign_schema_validate
            return BenchmarkResponse::badRequest("Unknown field: $key");
        }
    }
    $pdo = getDbConnection();
    $pdo->prepare("INSERT INTO users (name, email) VALUES (?, ?)")
        ->execute([$data['name'] ?? '', $data['email'] ?? '']);
    return BenchmarkResponse::ok('User created');
}
// vuln-code-snippet end php_massassign_schema_validate
