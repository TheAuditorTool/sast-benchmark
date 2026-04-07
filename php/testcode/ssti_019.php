<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_env_create
function ssti019(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $loader = new Twig\Loader\ArrayLoader(['user' => $input]);
    $env = new Twig\Environment($loader);
    $html = $env->render('user', []); // vuln-code-snippet vuln-line php_ssti_twig_env_create
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_twig_env_create
