<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_db_multihop
function headerinj032(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = (int)$req->param('id');
    $stmt = $pdo->prepare('SELECT redirect_url FROM redirects WHERE id = ?');
    $stmt->execute([$id]);
    $row = $stmt->fetch();
    header('Location: ' . $row['redirect_url']); // vuln-code-snippet vuln-line php_headerinj_db_multihop
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_headerinj_db_multihop
