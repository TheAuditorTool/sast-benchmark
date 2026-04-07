<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_service_layer_translate
function createUser045(string $name, string $email): stdClass {
    $m = new stdClass();
    $m->name = $name;
    $m->email = $email;
    return $m;
}

function massassign045(BenchmarkRequest $req): BenchmarkResponse {
    $model = createUser045($_POST['name'] ?? '', $_POST['email'] ?? ''); // vuln-code-snippet safe-line php_massassign_service_layer_translate
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_massassign_service_layer_translate
