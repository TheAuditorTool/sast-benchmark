<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_numeric_id_map
function ssti044(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $map = [1 => 'home.twig', 2 => 'about.twig'];
    $tpl = $map[$id] ?? 'default.twig'; // vuln-code-snippet safe-line php_ssti_numeric_id_map
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render($tpl, ['name' => $req->param('name')]);
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_numeric_id_map
