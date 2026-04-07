require_relative 'shared'
require 'erb'

def handler(req)
  output = ERB.new(File.read("templates/#{req.param('name')}.erb")).result
  BenchmarkResponse.html(output)
end
