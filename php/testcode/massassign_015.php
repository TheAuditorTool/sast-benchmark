<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_json_allowlist
function massassign015(BenchmarkRequest $req): BenchmarkResponse {
    $data = json_decode($req->bodyStr(), true);
    $allowed = ['name' => true, 'email' => true, 'phone' => true];
    $safe = array_intersect_key($data, $allowed); // vuln-code-snippet safe-line php_massassign_json_allowlist
    $pdo = getDbConnection();
    $pdo->prepare("UPDATE users SET name = ?, email = ?, phone = ? WHERE id = ?")
        ->execute([$safe['name'] ?? '', $safe['email'] ?? '', $safe['phone'] ?? '', $req->param('id')]);
    return BenchmarkResponse::ok('User updated');
}
// vuln-code-snippet end php_massassign_json_allowlist
