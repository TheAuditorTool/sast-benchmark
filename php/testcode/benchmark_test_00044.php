<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00044(BenchmarkRequest $req): BenchmarkResponse {
    $content = $req->post('content');
    $compiled = preg_replace('/\{\{(.+?)\}\}/', '<?php echo $1; ?>', $content);
    ob_start();
    eval('?>' . $compiled);
    $output = ob_get_clean();
    return BenchmarkResponse::html($output);
}
