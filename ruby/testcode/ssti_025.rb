require_relative 'shared'
require 'erb'

class UserTemplate
  def compile(source)
    ERB.new(source)
  end

  def render(source, context = binding)
    compile(source).result(context)
  end
end

# vuln-code-snippet start ruby_ssti_curly_user
def handler(req)
  engine = UserTemplate.new
  output = engine.render(req.param('template'))  # vuln-code-snippet vuln-line ruby_ssti_curly_user
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_curly_user
