<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_closure_isolation
function extract043(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('prefs');
    $result = (function () use ($input) {
        parse_str($input, $local); // vuln-code-snippet safe-line php_extract_closure_isolation
        return $local['key'] ?? null;
    })();
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_extract_closure_isolation
