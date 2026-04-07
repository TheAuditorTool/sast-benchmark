<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_variable_safe
function ssti032(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('name');
    $twig = new Twig\Environment(
        new Twig\Loader\FilesystemLoader('/views'),
        ['autoescape' => 'html']
    );
    $html = $twig->render('page.html.twig', ['name' => $input]); // vuln-code-snippet safe-line php_ssti_twig_variable_safe
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_twig_variable_safe
