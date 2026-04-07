<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_form_request_validated
function massassign033(BenchmarkRequest $req): BenchmarkResponse {
    $rules = ['name' => 'required|string', 'email' => 'required|email'];
    $validated = array_intersect_key($_POST, $rules); // vuln-code-snippet safe-line php_massassign_form_request_validated
    $model = new stdClass();
    foreach ($validated as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('validated');
}
// vuln-code-snippet end php_massassign_form_request_validated
