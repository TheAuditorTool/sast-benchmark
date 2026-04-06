<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_json_decode
function massassign009(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = json_decode($req->bodyStr(), true);
    $sets = [];
    $values = [];
    foreach ($data as $col => $val) { // vuln-code-snippet vuln-line php_massassign_json_decode
        $sets[] = "$col = ?";
        $values[] = $val;
    }
    $values[] = $req->param('id');
    $pdo->prepare("UPDATE users SET " . implode(', ', $sets) . " WHERE id = ?")->execute($values);
    return BenchmarkResponse::ok('User updated');
}
// vuln-code-snippet end php_massassign_json_decode
