<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_latte_from_string
function ssti008(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $latte = new Latte\Engine();
    $latte->setTempDirectory('/tmp/latte');
    $output = $latte->renderToString($template, ['name' => 'World']); // vuln-code-snippet vuln-line php_ssti_latte_from_string
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_latte_from_string
