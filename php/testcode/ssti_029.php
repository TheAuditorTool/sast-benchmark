<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_preg_template_engine
function ssti029(BenchmarkRequest $req): BenchmarkResponse {
    $pattern = $req->param('pattern');
    $tpl = 'Hello {name}';
    $result = preg_replace_callback('/' . $pattern . '/', fn($m) => eval('return "' . $m[0] . '";'), $tpl); // vuln-code-snippet vuln-line php_ssti_preg_template_engine
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_ssti_preg_template_engine
