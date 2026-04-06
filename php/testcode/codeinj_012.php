<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_usort_callback
function codeinj012(BenchmarkRequest $req): BenchmarkResponse {
    $comparator = $req->param('sort_func');
    $items = explode(',', $req->param('items'));
    usort($items, $comparator); // vuln-code-snippet vuln-line php_codeinj_usort_callback
    return BenchmarkResponse::json($items);
}
// vuln-code-snippet end php_codeinj_usort_callback
