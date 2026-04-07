<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_handlebars_user_string
function ssti031(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $engine = new LightnCandy\LightnCandy();
    $fn = $engine::compile($input); // vuln-code-snippet vuln-line php_ssti_handlebars_user_string
    $renderer = $engine::prepare($fn);
    echo $renderer(['name' => 'world']);
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_handlebars_user_string
