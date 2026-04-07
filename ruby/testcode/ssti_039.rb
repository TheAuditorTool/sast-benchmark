require_relative 'shared'

begin
  require 'redcarpet'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_redcarpet_markdown
def handler(req)
  renderer = Redcarpet::Render::HTML.new
  output = Redcarpet::Markdown.new(renderer).render(req.param('content')) # vuln-code-snippet safe-line ruby_ssti_redcarpet_markdown
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_redcarpet_markdown
