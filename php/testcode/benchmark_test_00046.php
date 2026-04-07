<?php
require_once __DIR__ . '/shared.php';

class Settings009 {
    public bool $debug = false;
    public string $mode = 'production';

    public function applyOverrides(array $data): void {
        extract($data);
        $this->debug = $debug ?? $this->debug;
        $this->mode = $mode ?? $this->mode;
    }
}

function benchmarkTest00046(BenchmarkRequest $req): BenchmarkResponse {
    $settings = new Settings009();
    $settings->applyOverrides($req->postData);
    return BenchmarkResponse::json(['debug' => $settings->debug, 'mode' => $settings->mode]);
}
