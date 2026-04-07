<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_twig_vars_only
function extract048(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('prefs');
    parse_str($input, $parsed);
    $allowed = array_intersect_key($parsed, array_flip(['lang', 'theme']));
    $twig = $req->twig();
    $html = $twig->render('page.twig', $allowed); // vuln-code-snippet safe-line php_extract_twig_vars_only
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_extract_twig_vars_only
