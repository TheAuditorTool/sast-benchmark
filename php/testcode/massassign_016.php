<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_explicit_set
function massassign016(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = (int)$req->param('id');
    $pdo->prepare("UPDATE users SET name = ?, email = ? WHERE id = ?") // vuln-code-snippet safe-line php_massassign_explicit_set
        ->execute([$req->post('name'), $req->post('email'), $id]);
    return BenchmarkResponse::ok('User updated');
}
// vuln-code-snippet end php_massassign_explicit_set
