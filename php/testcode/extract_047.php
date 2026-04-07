<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_explicit_key_access
function extract047(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int) $req->param('id');
    $stmt = $req->db()->prepare('SELECT title, body FROM posts WHERE id = ?');
    $stmt->execute([$id]);
    $row   = $stmt->fetch(PDO::FETCH_ASSOC);
    $title = $row['title']; // vuln-code-snippet safe-line php_extract_explicit_key_access
    $body  = $row['body'];
    return BenchmarkResponse::json(['title' => $title, 'body' => $body]);
}
// vuln-code-snippet end php_extract_explicit_key_access
