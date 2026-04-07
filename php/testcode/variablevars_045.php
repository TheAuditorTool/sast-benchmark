<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_template_context_only
function variablevars045(BenchmarkRequest $req): BenchmarkResponse {
    $vars = ['user' => $req->param('user'), 'title' => $req->param('title')];
    extract($vars); // vuln-code-snippet safe-line php_vv_template_context_only
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_vv_template_context_only
