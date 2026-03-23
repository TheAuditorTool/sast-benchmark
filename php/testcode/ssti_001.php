<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_from_string
function ssti001(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\ArrayLoader([]);
    $twig = new \Twig\Environment($loader);
    $template = $req->post('template');
    $output = $twig->createTemplate($template)->render([]); // vuln-code-snippet vuln-line php_ssti_twig_from_string
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_twig_from_string
