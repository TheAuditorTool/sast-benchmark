require_relative 'shared'

module NewRelicAgent
  def self.add_custom_attributes(attrs)
    # stub: would send to New Relic
    $stdout.puts("newrelic_attrs: #{attrs.inspect}")
  end
end

# vuln-code-snippet start ruby_loginj_newrelic_custom
def log_newrelic_custom(req)
  NewRelicAgent.add_custom_attributes({ event: req.param('event') }) # vuln-code-snippet vuln-line ruby_loginj_newrelic_custom
  BenchmarkResponse.json({ ok: true })
end
# vuln-code-snippet end ruby_loginj_newrelic_custom
