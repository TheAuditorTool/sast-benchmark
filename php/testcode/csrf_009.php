<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_ajax_no_origin
function csrf009(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = json_decode($req->bodyStr(), true);
    $pdo->prepare("DELETE FROM posts WHERE id = ?")->execute([$data['post_id']]); // vuln-code-snippet vuln-line php_csrf_ajax_no_origin
    return BenchmarkResponse::json(['deleted' => true]);
}
// vuln-code-snippet end php_csrf_ajax_no_origin
