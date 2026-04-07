require_relative 'shared'
require 'erb'

def render_precompiled(req)
  name = escape_html(req.param('name'))
  template = ERB.new(File.read('templates/greeting.erb'))
  output = template.result_with_hash(name: name)
  BenchmarkResponse.html(output)
end
