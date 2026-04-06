<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_escape
function ssti016(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\FilesystemLoader(__DIR__ . '/templates');
    $twig = new \Twig\Environment($loader, ['autoescape' => 'html']); // vuln-code-snippet safe-line php_ssti_twig_escape
    $output = $twig->render('greeting.html.twig', ['name' => $req->param('name')]);
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_twig_escape
