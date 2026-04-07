require_relative 'shared'
require 'erb'

def handler(req)
  output = ERB.new('Hello <%= name %>').result_with_hash({ name: ERB::Util.html_escape(req.param('name')) })
  BenchmarkResponse.html(output)
end
