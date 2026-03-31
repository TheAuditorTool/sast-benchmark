# Rails API routes - maps paths to controller methods.
# This file has no test case annotations.
require_relative 'Controllers/PostsController'
require_relative 'Controllers/UsersController'
require_relative 'Controllers/AdminController'
require_relative 'Controllers/SearchController'

ROUTES = {
  'GET /api/posts'            => ['PostsController', 'index'],
  'GET /api/posts/search'     => ['PostsController', 'search'],
  'POST /api/posts'           => ['PostsController', 'create'],
  'PUT /api/posts/:id'        => ['PostsController', 'update'],
  'POST /api/posts/upload'    => ['PostsController', 'upload'],
  'GET /api/users/:id'        => ['UsersController', 'show'],
  'GET /api/users/redirect'   => ['UsersController', 'redirect'],
  'POST /api/users/update'    => ['UsersController', 'update'],
  'GET /api/users/profile'    => ['UsersController', 'profile'],
  'GET /api/admin/run'        => ['AdminController', 'run_command'],
  'GET /api/admin/download'   => ['AdminController', 'download_file'],
  'GET /api/admin/config'     => ['AdminController', 'load_config'],
  'GET /api/search'           => ['SearchController', 'search'],
  'GET /api/search/render'    => ['SearchController', 'render_template'],
}.freeze
