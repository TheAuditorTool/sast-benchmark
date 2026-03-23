<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_stored_escaped
function xss_stored_escaped(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $pdo = getDbConnection();

    $stmt = $pdo->prepare('SELECT bio FROM users WHERE id = ?');
    $stmt->execute([$id]);
    $row = $stmt->fetch();
    $bio = $row['bio'] ?? '';

    $safe = htmlspecialchars($bio, ENT_QUOTES, 'UTF-8'); // vuln-code-snippet safe-line php_xss_stored_escaped
    $html = '<div class="bio">' . $safe . '</div>';

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_stored_escaped
