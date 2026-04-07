<?php
// laravel_api - Route definitions (Laravel-style REST API simulation)
// No test cases here -- just route-to-controller mapping.

require_once __DIR__ . '/../../testcode/shared.php';
require_once __DIR__ . '/Controllers/TaskController.php';
require_once __DIR__ . '/Controllers/UserController.php';
require_once __DIR__ . '/Controllers/AuthController.php';
require_once __DIR__ . '/Controllers/FileController.php';
require_once __DIR__ . '/Services/SearchService.php';

$routes = [
    'GET /api/tasks'            => ['TaskController', 'index'],
    'GET /api/tasks/search'     => ['TaskController', 'search'],
    'POST /api/tasks'           => ['TaskController', 'store'],
    'PUT /api/tasks/{id}'       => ['TaskController', 'update'],
    'GET /api/users/{id}'       => ['UserController', 'show'],
    'GET /api/redirect'         => ['UserController', 'redirect'],
    'POST /api/auth/login'      => ['AuthController', 'login'],
    'POST /api/auth/register'   => ['AuthController', 'register'],
    'POST /api/auth/change-pw'  => ['AuthController', 'changePassword'],
    'POST /api/files/upload'    => ['FileController', 'upload'],
    'GET /api/files/download'   => ['FileController', 'download'],
    'POST /api/webhooks'        => ['FileController', 'webhook'],
    'GET /api/search'           => ['SearchService', 'search'],
    'GET /api/search/render'    => ['SearchService', 'render'],
];
