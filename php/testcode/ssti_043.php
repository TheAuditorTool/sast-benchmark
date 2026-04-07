<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_strict_no_raw
function ssti043(BenchmarkRequest $req): BenchmarkResponse {
    $twig = new Twig\Environment(
        new Twig\Loader\FilesystemLoader('/views'),
        ['autoescape' => 'html', 'strict_variables' => true]
    );
    $html = $twig->render('template.twig', ['q' => $req->param('q')]); // vuln-code-snippet safe-line php_ssti_twig_strict_no_raw
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_twig_strict_no_raw
