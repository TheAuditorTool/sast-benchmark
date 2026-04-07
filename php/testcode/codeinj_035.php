<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_hardcoded_comparator
function codeinj035(BenchmarkRequest $req): BenchmarkResponse {
    $items = explode(',', $req->param('items'));
    usort($items, fn($a, $b) => strcmp($a, $b)); // vuln-code-snippet safe-line php_codeinj_hardcoded_comparator
    return BenchmarkResponse::ok(implode(',', $items));
}
// vuln-code-snippet end php_codeinj_hardcoded_comparator
