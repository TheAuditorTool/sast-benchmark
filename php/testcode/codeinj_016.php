<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_usort_closure
function codeinj016(BenchmarkRequest $req): BenchmarkResponse {
    $items = explode(',', $req->param('items'));
    usort($items, function($a, $b) { // vuln-code-snippet safe-line php_codeinj_usort_closure
        return strcmp($a, $b);
    });
    return BenchmarkResponse::json($items);
}
// vuln-code-snippet end php_codeinj_usort_closure
