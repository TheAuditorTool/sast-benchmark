<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_explicit
function massassign006(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $name = $req->post('name');
    $email = $req->post('email'); // vuln-code-snippet safe-line php_massassign_explicit
    $stmt = $pdo->prepare("UPDATE users SET name = ?, email = ? WHERE id = ?");
    $stmt->execute([$name, $email, $req->param('id')]);
    return BenchmarkResponse::json(['updated' => true]);
}
// vuln-code-snippet end php_massassign_explicit
