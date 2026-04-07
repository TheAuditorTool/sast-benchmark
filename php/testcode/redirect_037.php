<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_numeric_id_map
function redirect037(BenchmarkRequest $req): BenchmarkResponse {
    $id = intval($req->param('id'));
    $urls = [1 => '/home', 2 => '/profile', 3 => '/settings'];
    $url = $urls[$id] ?? '/'; // vuln-code-snippet safe-line php_redirect_numeric_id_map
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_numeric_id_map
