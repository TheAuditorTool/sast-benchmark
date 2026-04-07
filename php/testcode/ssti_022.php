<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_path_traversal
function ssti022(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render('../' . $userPath); // vuln-code-snippet vuln-line php_ssti_twig_path_traversal
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_twig_path_traversal
