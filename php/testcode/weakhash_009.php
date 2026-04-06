<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha1_file
function weakhash009(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('file');
    $hash = sha1_file($path); // vuln-code-snippet vuln-line php_weakhash_sha1_file
    return BenchmarkResponse::json(['integrity' => $hash]);
}
// vuln-code-snippet end php_weakhash_sha1_file
