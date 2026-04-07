<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_latte_file_not_string
function ssti045(BenchmarkRequest $req): BenchmarkResponse {
    $latte = new Latte\Engine();
    $latte->render('/views/template.latte', ['user' => $req->param('name')]); // vuln-code-snippet safe-line php_ssti_latte_file_not_string
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_latte_file_not_string
