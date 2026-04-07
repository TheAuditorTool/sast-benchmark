<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_custom_filter
function ssti025(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('fn');
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $filter = new Twig\TwigFilter('custom', $input);
    $twig->addFilter($filter); // vuln-code-snippet vuln-line php_ssti_twig_custom_filter
    $html = $twig->render('base.twig', []);
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_twig_custom_filter
