<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_controller_inject
function fileinclusion028(BenchmarkRequest $req): BenchmarkResponse {
    $ctrl = $req->post('ctrl');
    include 'controllers/' . $ctrl . 'Controller.php'; // vuln-code-snippet vuln-line php_fi_controller_inject
    return BenchmarkResponse::ok('Controller loaded');
}
// vuln-code-snippet end php_fi_controller_inject
