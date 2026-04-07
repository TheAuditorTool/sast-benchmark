<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_fixed_template_var_input
function ssti040(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('q');
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render('search.twig', ['q' => $input]); // vuln-code-snippet safe-line php_ssti_fixed_template_var_input
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_fixed_template_var_input
