require_relative 'shared'

MUSTACHE_TEMPLATE = 'Hello, {{name}}! Your score is {{score}}.'.freeze

# vuln-code-snippet start ruby_ssti_mustache
def ssti_mustache(req)
  vars = { 'name' => escape_html(req.param('name')), 'score' => escape_html(req.param('score')) }
  output = MUSTACHE_TEMPLATE.gsub(/\{\{(\w+)\}\}/) { vars[$1] || '' } # vuln-code-snippet safe-line ruby_ssti_mustache
  BenchmarkResponse.html(output)
end
# vuln-code-snippet end ruby_ssti_mustache
