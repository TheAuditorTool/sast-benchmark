<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_trim_only_multihop
function fileinclusion031(BenchmarkRequest $req): BenchmarkResponse {
    $page = trim($req->param('page'));
    include $page; // vuln-code-snippet vuln-line php_fi_trim_only_multihop
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_fi_trim_only_multihop
