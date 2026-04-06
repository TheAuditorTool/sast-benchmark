<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_loader_string
function ssti009(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $loader = new \Twig\Loader\ArrayLoader(['dynamic' => $template]); // vuln-code-snippet vuln-line php_ssti_twig_loader_string
    $twig = new \Twig\Environment($loader);
    $output = $twig->render('dynamic', ['name' => 'World']);
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_twig_loader_string
