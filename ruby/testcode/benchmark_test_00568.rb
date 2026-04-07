require_relative 'shared'

module Rack; module Session
  class Cookie
    def initialize(app, opts = {}); @app = app; @opts = opts; end
  end
end; end

def create_session_store(req)
  app = -> (env) { [200, {}, ['ok']] }
  store = Rack::Session::Cookie.new(app, secret: ENV['SESSION_SECRET'])
  BenchmarkResponse.ok('session store created')
end
