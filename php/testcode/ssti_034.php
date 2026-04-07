<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_allowlisted_file
function ssti034(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('page');
    $allowed = ['home', 'about', 'contact'];
    if (!in_array($name, $allowed, true)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render($name . '.twig', []); // vuln-code-snippet safe-line php_ssti_twig_allowlisted_file
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_twig_allowlisted_file
