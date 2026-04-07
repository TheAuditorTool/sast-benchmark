<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_cookie_name_inject
function headerinj028(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('name');
    $value = $req->post('value');
    setcookie($name, $value); // vuln-code-snippet vuln-line php_headerinj_cookie_name_inject
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_headerinj_cookie_name_inject
