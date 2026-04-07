<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_eval_php_template
function ssti021(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    ob_start();
    eval('?>' . $input . '<?php '); // vuln-code-snippet vuln-line php_ssti_eval_php_template
    $html = ob_get_clean();
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_eval_php_template
