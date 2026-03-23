<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_post
function extract001(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    extract($req->postData); // vuln-code-snippet vuln-line php_extract_post
    if ($is_admin) {
        return BenchmarkResponse::ok('admin panel');
    }
    return BenchmarkResponse::ok('user panel');
}
// vuln-code-snippet end php_extract_post
