<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_create_template
function ssti017(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('template');
    $twig = new Twig\Environment(new Twig\Loader\ArrayLoader());
    $tmpl = $twig->createTemplate($input); // vuln-code-snippet vuln-line php_ssti_twig_create_template
    $html = $tmpl->render([]);
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_twig_create_template
