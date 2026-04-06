require_relative 'shared'

module Mustache
  def self.render(template, data = {}); template; end
end

# vuln-code-snippet start ruby_ssti_mustache_logicless
def render_mustache(req)
  name = req.param('name')
  template = File.read('templates/greeting.mustache')
  output = Mustache.render(template, name: name) # vuln-code-snippet safe-line ruby_ssti_mustache_logicless
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_mustache_logicless
