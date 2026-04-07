<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00926(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('code');
    eval('?>' . preg_replace('/@php (.+?) @endphp/s', '<?php $1 ?>', $input) . '<?php');
    return BenchmarkResponse::ok('rendered');
}
