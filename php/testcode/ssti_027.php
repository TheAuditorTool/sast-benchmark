<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_compile_config_inject
function ssti027(BenchmarkRequest $req): BenchmarkResponse {
    $tplStr = file_get_contents(getenv('TEMPLATE_PATH'));
    $tplStr .= $req->param('extra');
    $twig = new Twig\Environment(new Twig\Loader\ArrayLoader(['main' => $tplStr]));
    echo $twig->render('main', []); // vuln-code-snippet vuln-line php_ssti_compile_config_inject
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_compile_config_inject
