<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_stored_db
function redirect009(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = (int)$req->param('link_id');
    $stmt = $pdo->prepare("SELECT url FROM short_links WHERE id = ?");
    $stmt->execute([$id]);
    $row = $stmt->fetch(\PDO::FETCH_ASSOC);
    header("Location: " . $row['url']); // vuln-code-snippet vuln-line php_redirect_stored_db
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_stored_db
