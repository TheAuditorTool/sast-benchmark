<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_stored_db_value
function redirect024(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $userId = (int) $req->param('user_id');
    $stmt = $db->prepare("SELECT redirect_url FROM user_prefs WHERE user_id = ?");
    $stmt->execute([$userId]);
    $url = $stmt->fetchColumn();
    header('Location: ' . $url); // vuln-code-snippet vuln-line php_redirect_stored_db_value
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_stored_db_value
