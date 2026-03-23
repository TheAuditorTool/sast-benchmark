<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_file
function ssti002(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\FilesystemLoader('/templates');
    $twig = new \Twig\Environment($loader);
    $name = $req->param('name');
    $output = $twig->render('contact.html.twig', ['name' => $name]); // vuln-code-snippet safe-line php_ssti_twig_file
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_twig_file
