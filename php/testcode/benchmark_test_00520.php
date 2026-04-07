<?php
require_once __DIR__ . '/shared.php';

class CreateUserRequest013 {
    public function __construct(
        public string $name,
        public string $email,
    ) {}
}

function benchmarkTest00520(BenchmarkRequest $req): BenchmarkResponse {
    $input = new CreateUserRequest013(
        name: $req->post('name'),
        email: $req->post('email'),
    );
    return BenchmarkResponse::json(['name' => $input->name, 'email' => $input->email]);
}
