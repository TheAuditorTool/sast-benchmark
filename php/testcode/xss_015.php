<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_stored_db
function xss_stored_db(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $pdo = getDbConnection();

    $stmt = $pdo->prepare('SELECT bio FROM users WHERE id = ?');
    $stmt->execute([$id]);
    $row = $stmt->fetch();
    $bio = $row['bio'] ?? '';

    $html = '<div class="bio">' . $bio . '</div>'; // vuln-code-snippet vuln-line php_xss_stored_db

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_stored_db
