require_relative 'shared'

begin
  require 'kramdown'
rescue LoadError
end

# vuln-code-snippet start ruby_ssti_kramdown_markdown
def handler(req)
  output = Kramdown::Document.new(req.param('md')).to_html # vuln-code-snippet safe-line ruby_ssti_kramdown_markdown
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_kramdown_markdown
