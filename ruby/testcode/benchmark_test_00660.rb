require_relative 'shared'

module Mustache
  def self.render(template, data = {}); template; end
end

def render_mustache(req)
  name = req.param('name')
  template = File.read('templates/greeting.mustache')
  output = Mustache.render(template, name: name)
  BenchmarkResponse.html(output)
end
