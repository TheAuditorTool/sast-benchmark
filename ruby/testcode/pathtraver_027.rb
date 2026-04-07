require_relative 'shared'

module Rack
  class Static
    def initialize(app, opts = {})
      @app = app
      @root = opts[:root]
      @urls = opts[:urls]
    end
    def call(env)
      [200, {}, []]
    end
  end
end

# vuln-code-snippet start ruby_pt_rack_static
def rack_static_dynamic_root(req)
  app = proc { |env| [200, {}, ['ok']] }
  Rack::Static.new(app, root: req.param('dir'), urls: ['/']) # vuln-code-snippet vuln-line ruby_pt_rack_static
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_pt_rack_static
