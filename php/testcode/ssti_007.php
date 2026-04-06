<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_mustache_lambda
function ssti007(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $engine = new Mustache_Engine();
    $output = $engine->render($template, ['name' => 'World']); // vuln-code-snippet vuln-line php_ssti_mustache_lambda
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_mustache_lambda
