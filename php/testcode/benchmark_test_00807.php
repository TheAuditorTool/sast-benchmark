<?php
require_once __DIR__ . '/shared.php';

class UserModel042 {
    private string $name = '';
    private string $email = '';
    public function setName(string $name): void { $this->name = $name; }
    public function setEmail(string $email): void { $this->email = $email; }
}

function benchmarkTest00807(BenchmarkRequest $req): BenchmarkResponse {
    $model = new UserModel042();
    $model->setName($_POST['name'] ?? '');
    $model->setEmail($_POST['email'] ?? '');
    return BenchmarkResponse::ok('set');
}
