<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_latte_string_render
function ssti020(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $latte = new Latte\Engine();
    $html = $latte->renderToString($input, []); // vuln-code-snippet vuln-line php_ssti_latte_string_render
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_latte_string_render
