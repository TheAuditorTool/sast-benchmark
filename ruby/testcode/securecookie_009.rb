require_relative 'shared'

module Rack; module Session
  class Cookie
    def initialize(app, opts = {}); @app = app; @opts = opts; end
  end
end; end

# vuln-code-snippet start ruby_securecookie_session_no_secure
def create_session_store(req)
  app = -> (env) { [200, {}, ['ok']] }
  store = Rack::Session::Cookie.new(app, secret: ENV['SESSION_SECRET']) # vuln-code-snippet vuln-line ruby_securecookie_session_no_secure
  BenchmarkResponse.ok('session store created')
end
# vuln-code-snippet end ruby_securecookie_session_no_secure
