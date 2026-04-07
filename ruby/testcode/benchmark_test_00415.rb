require_relative 'shared'
require 'erb'

def handler(req)
  output = ERB.new(File.read('templates/page.html.erb')).result
  BenchmarkResponse.html(output)
end
