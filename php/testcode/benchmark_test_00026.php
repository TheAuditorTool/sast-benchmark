<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00026(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    ob_start();
    eval('?>' . $input . '<?php ');
    $html = ob_get_clean();
    return BenchmarkResponse::ok($html);
}
