<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_config_defined_path
function ssti042(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = constant('DEFAULT_TEMPLATE');
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render($tpl, ['data' => $req->param('q')]); // vuln-code-snippet safe-line php_ssti_config_defined_path
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_config_defined_path
