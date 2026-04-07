<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_mustache_string_render
function ssti023(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $m = new Mustache_Engine();
    $html = $m->render($input, ['user' => 'world']); // vuln-code-snippet vuln-line php_ssti_mustache_string_render
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_mustache_string_render
