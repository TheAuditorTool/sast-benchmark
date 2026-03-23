<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_prefix
function extract002(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->postData;
    extract($data, EXTR_PREFIX_ALL, 'post'); // vuln-code-snippet safe-line php_extract_prefix
    $name = $post_name ?? 'anonymous';
    return BenchmarkResponse::ok("hello $name");
}
// vuln-code-snippet end php_extract_prefix
