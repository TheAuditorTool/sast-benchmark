<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_array_search_loose
function typejuggling007(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $acl = ['admin' => 'secret123', 'editor' => 'edit456'];
    $found = array_search($key, $acl); // vuln-code-snippet vuln-line php_tj_array_search_loose
    if ($found !== false) {
        return BenchmarkResponse::ok("role: $found");
    }
    return BenchmarkResponse::error('denied', 403);
}
// vuln-code-snippet end php_tj_array_search_loose
