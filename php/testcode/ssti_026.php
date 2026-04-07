<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_blade_eval_directive
function ssti026(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('code');
    eval('?>' . preg_replace('/@php (.+?) @endphp/s', '<?php $1 ?>', $input) . '<?php'); // vuln-code-snippet vuln-line php_ssti_blade_eval_directive
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_ssti_blade_eval_directive
